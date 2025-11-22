pub fn sub_826503A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826503A8 size=112
    let mut pc: u32 = 0x826503A8;
    'dispatch: loop {
        match pc {
            0x826503A8 => {
    //   block [0x826503A8..0x82650418)
	// 826503A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826503AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826503B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826503B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826503B8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826503BC: 38AA4150  addi r5, r10, 0x4150
	ctx.r[5].s64 = ctx.r[10].s64 + 16720;
	// 826503C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826503C4: 390B5940  addi r8, r11, 0x5940
	ctx.r[8].s64 = ctx.r[11].s64 + 22848;
	// 826503C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826503CC: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 826503D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826503D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826503D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826503DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826503E0: 386A4180  addi r3, r10, 0x4180
	ctx.r[3].s64 = ctx.r[10].s64 + 16768;
	// 826503E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826503E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826503EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826503F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826503F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826503F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826503FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650404: 4BE16A1D  bl 0x82466e20
	ctx.lr = 0x82650408;
	sub_82466E20(ctx, base);
	// 82650408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265040C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650418 size=112
    let mut pc: u32 = 0x82650418;
    'dispatch: loop {
        match pc {
            0x82650418 => {
    //   block [0x82650418..0x82650488)
	// 82650418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265041C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650424: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650428: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265042C: 38AA4150  addi r5, r10, 0x4150
	ctx.r[5].s64 = ctx.r[10].s64 + 16720;
	// 82650430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650434: 390B59A0  addi r8, r11, 0x59a0
	ctx.r[8].s64 = ctx.r[11].s64 + 22944;
	// 82650438: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265043C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82650440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650444: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265044C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650450: 386A41B0  addi r3, r10, 0x41b0
	ctx.r[3].s64 = ctx.r[10].s64 + 16816;
	// 82650454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265045C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265046C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650474: 4BE169AD  bl 0x82466e20
	ctx.lr = 0x82650478;
	sub_82466E20(ctx, base);
	// 82650478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265047C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650488 size=112
    let mut pc: u32 = 0x82650488;
    'dispatch: loop {
        match pc {
            0x82650488 => {
    //   block [0x82650488..0x826504F8)
	// 82650488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650494: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650498: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265049C: 38AA4150  addi r5, r10, 0x4150
	ctx.r[5].s64 = ctx.r[10].s64 + 16720;
	// 826504A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826504A4: 390B59D0  addi r8, r11, 0x59d0
	ctx.r[8].s64 = ctx.r[11].s64 + 22992;
	// 826504A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826504AC: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826504B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826504B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826504B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826504BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826504C0: 386A41E0  addi r3, r10, 0x41e0
	ctx.r[3].s64 = ctx.r[10].s64 + 16864;
	// 826504C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826504C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826504CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826504D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826504D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826504D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826504DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826504E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826504E4: 4BE1693D  bl 0x82466e20
	ctx.lr = 0x826504E8;
	sub_82466E20(ctx, base);
	// 826504E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826504EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826504F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826504F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826504F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826504F8 size=108
    let mut pc: u32 = 0x826504F8;
    'dispatch: loop {
        match pc {
            0x826504F8 => {
    //   block [0x826504F8..0x82650564)
	// 826504F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826504FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650504: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265050C: 38EB5A18  addi r7, r11, 0x5a18
	ctx.r[7].s64 = ctx.r[11].s64 + 23064;
	// 82650510: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82650514: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 82650518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265051C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650520: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82650524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650528: 386A4210  addi r3, r10, 0x4210
	ctx.r[3].s64 = ctx.r[10].s64 + 16912;
	// 8265052C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82650530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265053C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265054C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82650550: 4BE168D1  bl 0x82466e20
	ctx.lr = 0x82650554;
	sub_82466E20(ctx, base);
	// 82650554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265055C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650568 size=112
    let mut pc: u32 = 0x82650568;
    'dispatch: loop {
        match pc {
            0x82650568 => {
    //   block [0x82650568..0x826505D8)
	// 82650568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265056C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650574: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650578: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265057C: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 82650580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650584: 390B5A48  addi r8, r11, 0x5a48
	ctx.r[8].s64 = ctx.r[11].s64 + 23112;
	// 82650588: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265058C: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 82650590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650594: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265059C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826505A0: 386A4240  addi r3, r10, 0x4240
	ctx.r[3].s64 = ctx.r[10].s64 + 16960;
	// 826505A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826505A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826505AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826505B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826505B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826505B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826505BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826505C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826505C4: 4BE1685D  bl 0x82466e20
	ctx.lr = 0x826505C8;
	sub_82466E20(ctx, base);
	// 826505C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826505CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826505D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826505D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826505D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826505D8 size=116
    let mut pc: u32 = 0x826505D8;
    'dispatch: loop {
        match pc {
            0x826505D8 => {
    //   block [0x826505D8..0x8265064C)
	// 826505D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826505DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826505E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826505E4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 826505E8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 826505EC: 390A5A60  addi r8, r10, 0x5a60
	ctx.r[8].s64 = ctx.r[10].s64 + 23136;
	// 826505F0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826505F4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826505F8: 38AA46C0  addi r5, r10, 0x46c0
	ctx.r[5].s64 = ctx.r[10].s64 + 18112;
	// 826505FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650600: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82650604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265060C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 82650610: 396BA4A0  addi r11, r11, -0x5b60
	ctx.r[11].s64 = ctx.r[11].s64 + -23392;
	// 82650614: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650618: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265061C: 386A4270  addi r3, r10, 0x4270
	ctx.r[3].s64 = ctx.r[10].s64 + 17008;
	// 82650620: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82650624: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650628: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265062C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650638: 4BE167E9  bl 0x82466e20
	ctx.lr = 0x8265063C;
	sub_82466E20(ctx, base);
	// 8265063C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650650 size=100
    let mut pc: u32 = 0x82650650;
    'dispatch: loop {
        match pc {
            0x82650650 => {
    //   block [0x82650650..0x826506B4)
	// 82650650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265065C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650664: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82650668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265066C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650670: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 82650674: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265067C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650684: 386A42A0  addi r3, r10, 0x42a0
	ctx.r[3].s64 = ctx.r[10].s64 + 17056;
	// 82650688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265068C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650690: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650698: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265069C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826506A0: 4BE16781  bl 0x82466e20
	ctx.lr = 0x826506A4;
	sub_82466E20(ctx, base);
	// 826506A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826506A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826506AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826506B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826506B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826506B8 size=100
    let mut pc: u32 = 0x826506B8;
    'dispatch: loop {
        match pc {
            0x826506B8 => {
    //   block [0x826506B8..0x8265071C)
	// 826506B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826506BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826506C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826506C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826506C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826506CC: 38AA4330  addi r5, r10, 0x4330
	ctx.r[5].s64 = ctx.r[10].s64 + 17200;
	// 826506D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826506D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826506D8: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 826506DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826506E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826506E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826506E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826506EC: 386A42D0  addi r3, r10, 0x42d0
	ctx.r[3].s64 = ctx.r[10].s64 + 17104;
	// 826506F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826506F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826506F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826506FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650700: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650708: 4BE16719  bl 0x82466e20
	ctx.lr = 0x8265070C;
	sub_82466E20(ctx, base);
	// 8265070C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650720 size=100
    let mut pc: u32 = 0x82650720;
    'dispatch: loop {
        match pc {
            0x82650720 => {
    //   block [0x82650720..0x82650784)
	// 82650720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265072C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650734: 38AA4270  addi r5, r10, 0x4270
	ctx.r[5].s64 = ctx.r[10].s64 + 17008;
	// 82650738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265073C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650740: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 82650744: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265074C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650754: 386A4300  addi r3, r10, 0x4300
	ctx.r[3].s64 = ctx.r[10].s64 + 17152;
	// 82650758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265075C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650760: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650768: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265076C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650770: 4BE166B1  bl 0x82466e20
	ctx.lr = 0x82650774;
	sub_82466E20(ctx, base);
	// 82650774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265077C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650788 size=104
    let mut pc: u32 = 0x82650788;
    'dispatch: loop {
        match pc {
            0x82650788 => {
    //   block [0x82650788..0x826507F0)
	// 82650788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265078C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650794: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82650798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265079C: 392AA504  addi r9, r10, -0x5afc
	ctx.r[9].s64 = ctx.r[10].s64 + -23292;
	// 826507A0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826507A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826507A8: 38AA42A0  addi r5, r10, 0x42a0
	ctx.r[5].s64 = ctx.r[10].s64 + 17056;
	// 826507AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826507B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826507B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826507B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826507BC: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826507C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826507C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826507C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826507CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826507D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826507D4: 386A4330  addi r3, r10, 0x4330
	ctx.r[3].s64 = ctx.r[10].s64 + 17200;
	// 826507D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826507DC: 4BE16645  bl 0x82466e20
	ctx.lr = 0x826507E0;
	sub_82466E20(ctx, base);
	// 826507E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826507E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826507E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826507EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826507F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826507F0 size=108
    let mut pc: u32 = 0x826507F0;
    'dispatch: loop {
        match pc {
            0x826507F0 => {
    //   block [0x826507F0..0x8265085C)
	// 826507F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826507F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826507F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826507FC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650804: 38EB5BE0  addi r7, r11, 0x5be0
	ctx.r[7].s64 = ctx.r[11].s64 + 23520;
	// 82650808: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265080C: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82650810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650814: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650818: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265081C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650820: 386A4360  addi r3, r10, 0x4360
	ctx.r[3].s64 = ctx.r[10].s64 + 17248;
	// 82650824: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82650828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265082C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265083C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82650848: 4BE165D9  bl 0x82466e20
	ctx.lr = 0x8265084C;
	sub_82466E20(ctx, base);
	// 8265084C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650860 size=112
    let mut pc: u32 = 0x82650860;
    'dispatch: loop {
        match pc {
            0x82650860 => {
    //   block [0x82650860..0x826508D0)
	// 82650860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265086C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650870: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650874: 38AA4330  addi r5, r10, 0x4330
	ctx.r[5].s64 = ctx.r[10].s64 + 17200;
	// 82650878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265087C: 390B5C10  addi r8, r11, 0x5c10
	ctx.r[8].s64 = ctx.r[11].s64 + 23568;
	// 82650880: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82650884: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82650888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265088C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650898: 386A4390  addi r3, r10, 0x4390
	ctx.r[3].s64 = ctx.r[10].s64 + 17296;
	// 8265089C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826508A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826508A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826508A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826508AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826508B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826508B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826508B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826508BC: 4BE16565  bl 0x82466e20
	ctx.lr = 0x826508C0;
	sub_82466E20(ctx, base);
	// 826508C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826508C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826508C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826508CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826508D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826508D0 size=24
    let mut pc: u32 = 0x826508D0;
    'dispatch: loop {
        match pc {
            0x826508D0 => {
    //   block [0x826508D0..0x826508E8)
	// 826508D0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826508D4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826508D8: 394A84B8  addi r10, r10, -0x7b48
	ctx.r[10].s64 = ctx.r[10].s64 + -31560;
	// 826508DC: 816B5CB8  lwz r11, 0x5cb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23736 as u32) ) } as u64;
	// 826508E0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826508E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826508E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826508E8 size=116
    let mut pc: u32 = 0x826508E8;
    'dispatch: loop {
        match pc {
            0x826508E8 => {
    //   block [0x826508E8..0x8265095C)
	// 826508E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826508EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826508F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826508F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826508F8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826508FC: 390B84B8  addi r8, r11, -0x7b48
	ctx.r[8].s64 = ctx.r[11].s64 + -31560;
	// 82650900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650904: 392AA568  addi r9, r10, -0x5a98
	ctx.r[9].s64 = ctx.r[10].s64 + -23192;
	// 82650908: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265090C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82650910: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82650914: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265091C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265092C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82650930: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 82650934: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82650938: 386B43C0  addi r3, r11, 0x43c0
	ctx.r[3].s64 = ctx.r[11].s64 + 17344;
	// 8265093C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82650940: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650948: 4BE164D9  bl 0x82466e20
	ctx.lr = 0x8265094C;
	sub_82466E20(ctx, base);
	// 8265094C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650960 size=100
    let mut pc: u32 = 0x82650960;
    'dispatch: loop {
        match pc {
            0x82650960 => {
    //   block [0x82650960..0x826509C4)
	// 82650960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265096C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650974: 38AA43C0  addi r5, r10, 0x43c0
	ctx.r[5].s64 = ctx.r[10].s64 + 17344;
	// 82650978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265097C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650980: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 82650984: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265098C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650994: 386A43F0  addi r3, r10, 0x43f0
	ctx.r[3].s64 = ctx.r[10].s64 + 17392;
	// 82650998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265099C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826509A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826509A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826509A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826509AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826509B0: 4BE16471  bl 0x82466e20
	ctx.lr = 0x826509B4;
	sub_82466E20(ctx, base);
	// 826509B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826509B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826509BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826509C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826509C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826509C8 size=100
    let mut pc: u32 = 0x826509C8;
    'dispatch: loop {
        match pc {
            0x826509C8 => {
    //   block [0x826509C8..0x82650A2C)
	// 826509C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826509CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826509D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826509D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826509D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826509DC: 38AA4450  addi r5, r10, 0x4450
	ctx.r[5].s64 = ctx.r[10].s64 + 17488;
	// 826509E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826509E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826509E8: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826509EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826509F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826509F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826509F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826509FC: 386A4420  addi r3, r10, 0x4420
	ctx.r[3].s64 = ctx.r[10].s64 + 17440;
	// 82650A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650A04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650A08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650A10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650A18: 4BE16409  bl 0x82466e20
	ctx.lr = 0x82650A1C;
	sub_82466E20(ctx, base);
	// 82650A1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650A30 size=112
    let mut pc: u32 = 0x82650A30;
    'dispatch: loop {
        match pc {
            0x82650A30 => {
    //   block [0x82650A30..0x82650AA0)
	// 82650A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650A3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650A40: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650A44: 38AA43C0  addi r5, r10, 0x43c0
	ctx.r[5].s64 = ctx.r[10].s64 + 17344;
	// 82650A48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650A4C: 390B5CBC  addi r8, r11, 0x5cbc
	ctx.r[8].s64 = ctx.r[11].s64 + 23740;
	// 82650A50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82650A54: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82650A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650A5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650A60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650A68: 386A4450  addi r3, r10, 0x4450
	ctx.r[3].s64 = ctx.r[10].s64 + 17488;
	// 82650A6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650A70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650A7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650A8C: 4BE16395  bl 0x82466e20
	ctx.lr = 0x82650A90;
	sub_82466E20(ctx, base);
	// 82650A90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650AA0 size=100
    let mut pc: u32 = 0x82650AA0;
    'dispatch: loop {
        match pc {
            0x82650AA0 => {
    //   block [0x82650AA0..0x82650B04)
	// 82650AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650AAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650AB4: 38AA4450  addi r5, r10, 0x4450
	ctx.r[5].s64 = ctx.r[10].s64 + 17488;
	// 82650AB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650AC0: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 82650AC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650AD4: 386A4480  addi r3, r10, 0x4480
	ctx.r[3].s64 = ctx.r[10].s64 + 17536;
	// 82650AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650ADC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650AE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650AE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650AF0: 4BE16331  bl 0x82466e20
	ctx.lr = 0x82650AF4;
	sub_82466E20(ctx, base);
	// 82650AF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650AF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650AFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650B00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650B08 size=100
    let mut pc: u32 = 0x82650B08;
    'dispatch: loop {
        match pc {
            0x82650B08 => {
    //   block [0x82650B08..0x82650B6C)
	// 82650B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650B14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650B1C: 38AA43C0  addi r5, r10, 0x43c0
	ctx.r[5].s64 = ctx.r[10].s64 + 17344;
	// 82650B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650B28: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 82650B2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650B3C: 386A44B0  addi r3, r10, 0x44b0
	ctx.r[3].s64 = ctx.r[10].s64 + 17584;
	// 82650B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650B44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650B48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650B50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650B58: 4BE162C9  bl 0x82466e20
	ctx.lr = 0x82650B5C;
	sub_82466E20(ctx, base);
	// 82650B5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650B60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650B64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650B70 size=100
    let mut pc: u32 = 0x82650B70;
    'dispatch: loop {
        match pc {
            0x82650B70 => {
    //   block [0x82650B70..0x82650BD4)
	// 82650B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650B7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650B84: 38AA43F0  addi r5, r10, 0x43f0
	ctx.r[5].s64 = ctx.r[10].s64 + 17392;
	// 82650B88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650B90: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 82650B94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650BA4: 386A44E0  addi r3, r10, 0x44e0
	ctx.r[3].s64 = ctx.r[10].s64 + 17632;
	// 82650BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650BAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650BB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650BB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650BC0: 4BE16261  bl 0x82466e20
	ctx.lr = 0x82650BC4;
	sub_82466E20(ctx, base);
	// 82650BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650BD8 size=100
    let mut pc: u32 = 0x82650BD8;
    'dispatch: loop {
        match pc {
            0x82650BD8 => {
    //   block [0x82650BD8..0x82650C3C)
	// 82650BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650BE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650BEC: 38AA44B0  addi r5, r10, 0x44b0
	ctx.r[5].s64 = ctx.r[10].s64 + 17584;
	// 82650BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650BF8: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82650BFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650C0C: 386A4510  addi r3, r10, 0x4510
	ctx.r[3].s64 = ctx.r[10].s64 + 17680;
	// 82650C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650C14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650C18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650C20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650C28: 4BE161F9  bl 0x82466e20
	ctx.lr = 0x82650C2C;
	sub_82466E20(ctx, base);
	// 82650C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650C40 size=100
    let mut pc: u32 = 0x82650C40;
    'dispatch: loop {
        match pc {
            0x82650C40 => {
    //   block [0x82650C40..0x82650CA4)
	// 82650C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650C4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650C54: 38AA43F0  addi r5, r10, 0x43f0
	ctx.r[5].s64 = ctx.r[10].s64 + 17392;
	// 82650C58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650C60: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 82650C64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650C68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650C6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650C70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650C74: 386A4540  addi r3, r10, 0x4540
	ctx.r[3].s64 = ctx.r[10].s64 + 17728;
	// 82650C78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650C7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650C80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650C88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650C90: 4BE16191  bl 0x82466e20
	ctx.lr = 0x82650C94;
	sub_82466E20(ctx, base);
	// 82650C94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650CA8 size=112
    let mut pc: u32 = 0x82650CA8;
    'dispatch: loop {
        match pc {
            0x82650CA8 => {
    //   block [0x82650CA8..0x82650D18)
	// 82650CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650CB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650CB8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650CBC: 38AA45D0  addi r5, r10, 0x45d0
	ctx.r[5].s64 = ctx.r[10].s64 + 17872;
	// 82650CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650CC4: 390B5CEC  addi r8, r11, 0x5cec
	ctx.r[8].s64 = ctx.r[11].s64 + 23788;
	// 82650CC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82650CCC: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82650CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650CD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650CD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650CE0: 386A4570  addi r3, r10, 0x4570
	ctx.r[3].s64 = ctx.r[10].s64 + 17776;
	// 82650CE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650D04: 4BE1611D  bl 0x82466e20
	ctx.lr = 0x82650D08;
	sub_82466E20(ctx, base);
	// 82650D08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650D18 size=112
    let mut pc: u32 = 0x82650D18;
    'dispatch: loop {
        match pc {
            0x82650D18 => {
    //   block [0x82650D18..0x82650D88)
	// 82650D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650D24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650D28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650D2C: 38AA4600  addi r5, r10, 0x4600
	ctx.r[5].s64 = ctx.r[10].s64 + 17920;
	// 82650D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650D34: 390B5D1C  addi r8, r11, 0x5d1c
	ctx.r[8].s64 = ctx.r[11].s64 + 23836;
	// 82650D38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82650D3C: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 82650D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650D44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650D50: 386A45A0  addi r3, r10, 0x45a0
	ctx.r[3].s64 = ctx.r[10].s64 + 17824;
	// 82650D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650D74: 4BE160AD  bl 0x82466e20
	ctx.lr = 0x82650D78;
	sub_82466E20(ctx, base);
	// 82650D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650D88 size=112
    let mut pc: u32 = 0x82650D88;
    'dispatch: loop {
        match pc {
            0x82650D88 => {
    //   block [0x82650D88..0x82650DF8)
	// 82650D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650D94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650D98: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650D9C: 38AA46C0  addi r5, r10, 0x46c0
	ctx.r[5].s64 = ctx.r[10].s64 + 18112;
	// 82650DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650DA4: 390B5D34  addi r8, r11, 0x5d34
	ctx.r[8].s64 = ctx.r[11].s64 + 23860;
	// 82650DA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82650DAC: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82650DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650DB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650DC0: 386A45D0  addi r3, r10, 0x45d0
	ctx.r[3].s64 = ctx.r[10].s64 + 17872;
	// 82650DC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650DE4: 4BE1603D  bl 0x82466e20
	ctx.lr = 0x82650DE8;
	sub_82466E20(ctx, base);
	// 82650DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650DF8 size=112
    let mut pc: u32 = 0x82650DF8;
    'dispatch: loop {
        match pc {
            0x82650DF8 => {
    //   block [0x82650DF8..0x82650E68)
	// 82650DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650E04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650E08: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650E0C: 38AA45D0  addi r5, r10, 0x45d0
	ctx.r[5].s64 = ctx.r[10].s64 + 17872;
	// 82650E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650E14: 390B5D64  addi r8, r11, 0x5d64
	ctx.r[8].s64 = ctx.r[11].s64 + 23908;
	// 82650E18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82650E1C: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 82650E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650E24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650E28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650E2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650E30: 386A4600  addi r3, r10, 0x4600
	ctx.r[3].s64 = ctx.r[10].s64 + 17920;
	// 82650E34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650E44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650E54: 4BE15FCD  bl 0x82466e20
	ctx.lr = 0x82650E58;
	sub_82466E20(ctx, base);
	// 82650E58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650E68 size=112
    let mut pc: u32 = 0x82650E68;
    'dispatch: loop {
        match pc {
            0x82650E68 => {
    //   block [0x82650E68..0x82650ED8)
	// 82650E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650E74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650E78: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650E7C: 38AA4600  addi r5, r10, 0x4600
	ctx.r[5].s64 = ctx.r[10].s64 + 17920;
	// 82650E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650E84: 390B5D7C  addi r8, r11, 0x5d7c
	ctx.r[8].s64 = ctx.r[11].s64 + 23932;
	// 82650E88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82650E8C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 82650E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650E94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650E98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650EA0: 386A4630  addi r3, r10, 0x4630
	ctx.r[3].s64 = ctx.r[10].s64 + 17968;
	// 82650EA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650EC4: 4BE15F5D  bl 0x82466e20
	ctx.lr = 0x82650EC8;
	sub_82466E20(ctx, base);
	// 82650EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650ED8 size=116
    let mut pc: u32 = 0x82650ED8;
    'dispatch: loop {
        match pc {
            0x82650ED8 => {
    //   block [0x82650ED8..0x82650F4C)
	// 82650ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650EE4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82650EE8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82650EEC: 390A5D98  addi r8, r10, 0x5d98
	ctx.r[8].s64 = ctx.r[10].s64 + 23960;
	// 82650EF0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650EF4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82650EF8: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82650EFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650F00: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82650F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650F0C: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82650F10: 396BA57C  addi r11, r11, -0x5a84
	ctx.r[11].s64 = ctx.r[11].s64 + -23172;
	// 82650F14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650F18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650F1C: 386A4660  addi r3, r10, 0x4660
	ctx.r[3].s64 = ctx.r[10].s64 + 18016;
	// 82650F20: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82650F24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650F28: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82650F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650F38: 4BE15EE9  bl 0x82466e20
	ctx.lr = 0x82650F3C;
	sub_82466E20(ctx, base);
	// 82650F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82650F50 size=48
    let mut pc: u32 = 0x82650F50;
    'dispatch: loop {
        match pc {
            0x82650F50 => {
    //   block [0x82650F50..0x82650F80)
	// 82650F50: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650F54: 814B5E48  lwz r10, 0x5e48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24136 as u32) ) } as u64;
	// 82650F58: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82650F5C: 396B8578  addi r11, r11, -0x7a88
	ctx.r[11].s64 = ctx.r[11].s64 + -31368;
	// 82650F60: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82650F64: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82650F68: 814A5E44  lwz r10, 0x5e44(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24132 as u32) ) } as u64;
	// 82650F6C: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 82650F70: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82650F74: 814A5E40  lwz r10, 0x5e40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24128 as u32) ) } as u64;
	// 82650F78: 914B0278  stw r10, 0x278(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(632 as u32), ctx.r[10].u32 ) };
	// 82650F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650F80 size=116
    let mut pc: u32 = 0x82650F80;
    'dispatch: loop {
        match pc {
            0x82650F80 => {
    //   block [0x82650F80..0x82650FF4)
	// 82650F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650F8C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82650F90: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650F94: 392BA650  addi r9, r11, -0x59b0
	ctx.r[9].s64 = ctx.r[11].s64 + -22960;
	// 82650F98: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82650F9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650FA0: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 82650FA4: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82650FA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82650FAC: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 82650FB0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650FB4: 396B8578  addi r11, r11, -0x7a88
	ctx.r[11].s64 = ctx.r[11].s64 + -31368;
	// 82650FB8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82650FBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650FC0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82650FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650FC8: 386A4690  addi r3, r10, 0x4690
	ctx.r[3].s64 = ctx.r[10].s64 + 18064;
	// 82650FCC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82650FD0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82650FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650FD8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82650FDC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650FE0: 4BE15E41  bl 0x82466e20
	ctx.lr = 0x82650FE4;
	sub_82466E20(ctx, base);
	// 82650FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650FF8 size=116
    let mut pc: u32 = 0x82650FF8;
    'dispatch: loop {
        match pc {
            0x82650FF8 => {
    //   block [0x82650FF8..0x8265106C)
	// 82650FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651004: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651008: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265100C: 390B5E50  addi r8, r11, 0x5e50
	ctx.r[8].s64 = ctx.r[11].s64 + 24144;
	// 82651010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651014: 392AA79C  addi r9, r10, -0x5864
	ctx.r[9].s64 = ctx.r[10].s64 + -22628;
	// 82651018: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265101C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82651020: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651024: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265102C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265103C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82651040: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 82651044: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82651048: 386B46C0  addi r3, r11, 0x46c0
	ctx.r[3].s64 = ctx.r[11].s64 + 18112;
	// 8265104C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82651050: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651058: 4BE15DC9  bl 0x82466e20
	ctx.lr = 0x8265105C;
	sub_82466E20(ctx, base);
	// 8265105C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651070 size=112
    let mut pc: u32 = 0x82651070;
    'dispatch: loop {
        match pc {
            0x82651070 => {
    //   block [0x82651070..0x826510E0)
	// 82651070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265107C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651080: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651084: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265108C: 390B5EE0  addi r8, r11, 0x5ee0
	ctx.r[8].s64 = ctx.r[11].s64 + 24288;
	// 82651090: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82651094: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82651098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265109C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826510A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826510A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826510A8: 386A46F0  addi r3, r10, 0x46f0
	ctx.r[3].s64 = ctx.r[10].s64 + 18160;
	// 826510AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826510B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826510B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826510B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826510BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826510C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826510C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826510C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826510CC: 4BE15D55  bl 0x82466e20
	ctx.lr = 0x826510D0;
	sub_82466E20(ctx, base);
	// 826510D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826510D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826510D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826510DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826510E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826510E0 size=112
    let mut pc: u32 = 0x826510E0;
    'dispatch: loop {
        match pc {
            0x826510E0 => {
    //   block [0x826510E0..0x82651150)
	// 826510E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826510E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826510E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826510EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826510F0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826510F4: 38AA2920  addi r5, r10, 0x2920
	ctx.r[5].s64 = ctx.r[10].s64 + 10528;
	// 826510F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826510FC: 390B5EF8  addi r8, r11, 0x5ef8
	ctx.r[8].s64 = ctx.r[11].s64 + 24312;
	// 82651100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82651104: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82651108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265110C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651118: 386A4720  addi r3, r10, 0x4720
	ctx.r[3].s64 = ctx.r[10].s64 + 18208;
	// 8265111C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265112C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265113C: 4BE15CE5  bl 0x82466e20
	ctx.lr = 0x82651140;
	sub_82466E20(ctx, base);
	// 82651140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265114C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651150 size=108
    let mut pc: u32 = 0x82651150;
    'dispatch: loop {
        match pc {
            0x82651150 => {
    //   block [0x82651150..0x826511BC)
	// 82651150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265115C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651164: 38EB5F10  addi r7, r11, 0x5f10
	ctx.r[7].s64 = ctx.r[11].s64 + 24336;
	// 82651168: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265116C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82651170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651174: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651178: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265117C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651180: 386A4750  addi r3, r10, 0x4750
	ctx.r[3].s64 = ctx.r[10].s64 + 18256;
	// 82651184: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265118C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265119C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826511A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826511A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826511A8: 4BE15C79  bl 0x82466e20
	ctx.lr = 0x826511AC;
	sub_82466E20(ctx, base);
	// 826511AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826511B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826511B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826511B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826511C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826511C0 size=112
    let mut pc: u32 = 0x826511C0;
    'dispatch: loop {
        match pc {
            0x826511C0 => {
    //   block [0x826511C0..0x82651230)
	// 826511C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826511C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826511C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826511CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826511D0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826511D4: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 826511D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826511DC: 390B5F28  addi r8, r11, 0x5f28
	ctx.r[8].s64 = ctx.r[11].s64 + 24360;
	// 826511E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826511E4: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 826511E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826511EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826511F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826511F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826511F8: 386A4780  addi r3, r10, 0x4780
	ctx.r[3].s64 = ctx.r[10].s64 + 18304;
	// 826511FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265120C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265121C: 4BE15C05  bl 0x82466e20
	ctx.lr = 0x82651220;
	sub_82466E20(ctx, base);
	// 82651220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265122C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651230 size=108
    let mut pc: u32 = 0x82651230;
    'dispatch: loop {
        match pc {
            0x82651230 => {
    //   block [0x82651230..0x8265129C)
	// 82651230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265123C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651244: 38EB5F70  addi r7, r11, 0x5f70
	ctx.r[7].s64 = ctx.r[11].s64 + 24432;
	// 82651248: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265124C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82651250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651254: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651258: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265125C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651260: 386A47B0  addi r3, r10, 0x47b0
	ctx.r[3].s64 = ctx.r[10].s64 + 18352;
	// 82651264: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265126C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265127C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651288: 4BE15B99  bl 0x82466e20
	ctx.lr = 0x8265128C;
	sub_82466E20(ctx, base);
	// 8265128C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826512A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826512A0 size=112
    let mut pc: u32 = 0x826512A0;
    'dispatch: loop {
        match pc {
            0x826512A0 => {
    //   block [0x826512A0..0x82651310)
	// 826512A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826512A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826512A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826512AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826512B0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826512B4: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 826512B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826512BC: 390B5F88  addi r8, r11, 0x5f88
	ctx.r[8].s64 = ctx.r[11].s64 + 24456;
	// 826512C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826512C4: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826512C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826512CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826512D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826512D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826512D8: 386A47E0  addi r3, r10, 0x47e0
	ctx.r[3].s64 = ctx.r[10].s64 + 18400;
	// 826512DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826512E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826512E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826512E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826512EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826512F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826512F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826512F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826512FC: 4BE15B25  bl 0x82466e20
	ctx.lr = 0x82651300;
	sub_82466E20(ctx, base);
	// 82651300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265130C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651310 size=112
    let mut pc: u32 = 0x82651310;
    'dispatch: loop {
        match pc {
            0x82651310 => {
    //   block [0x82651310..0x82651380)
	// 82651310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265131C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82651320: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651324: 392AA7F4  addi r9, r10, -0x580c
	ctx.r[9].s64 = ctx.r[10].s64 + -22540;
	// 82651328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265132C: 390B5FC0  addi r8, r11, 0x5fc0
	ctx.r[8].s64 = ctx.r[11].s64 + 24512;
	// 82651330: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82651334: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 82651338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265133C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651348: 386A4810  addi r3, r10, 0x4810
	ctx.r[3].s64 = ctx.r[10].s64 + 18448;
	// 8265134C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82651350: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82651354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265135C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265136C: 4BE15AB5  bl 0x82466e20
	ctx.lr = 0x82651370;
	sub_82466E20(ctx, base);
	// 82651370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265137C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651380 size=116
    let mut pc: u32 = 0x82651380;
    'dispatch: loop {
        match pc {
            0x82651380 => {
    //   block [0x82651380..0x826513F4)
	// 82651380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265138C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651390: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82651394: 390B6068  addi r8, r11, 0x6068
	ctx.r[8].s64 = ctx.r[11].s64 + 24680;
	// 82651398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265139C: 392AA7C8  addi r9, r10, -0x5838
	ctx.r[9].s64 = ctx.r[10].s64 + -22584;
	// 826513A0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826513A4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826513A8: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 826513AC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826513B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826513B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826513B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826513BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826513C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826513C4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 826513C8: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826513CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826513D0: 386B4840  addi r3, r11, 0x4840
	ctx.r[3].s64 = ctx.r[11].s64 + 18496;
	// 826513D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826513D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826513DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826513E0: 4BE15A41  bl 0x82466e20
	ctx.lr = 0x826513E4;
	sub_82466E20(ctx, base);
	// 826513E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826513E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826513EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826513F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826513F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826513F8 size=112
    let mut pc: u32 = 0x826513F8;
    'dispatch: loop {
        match pc {
            0x826513F8 => {
    //   block [0x826513F8..0x82651468)
	// 826513F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826513FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651404: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82651408: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265140C: 392AA820  addi r9, r10, -0x57e0
	ctx.r[9].s64 = ctx.r[10].s64 + -22496;
	// 82651410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651414: 390B6088  addi r8, r11, 0x6088
	ctx.r[8].s64 = ctx.r[11].s64 + 24712;
	// 82651418: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8265141C: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 82651420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651424: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651428: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265142C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651430: 386A4870  addi r3, r10, 0x4870
	ctx.r[3].s64 = ctx.r[10].s64 + 18544;
	// 82651434: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82651438: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265143C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265144C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651454: 4BE159CD  bl 0x82466e20
	ctx.lr = 0x82651458;
	sub_82466E20(ctx, base);
	// 82651458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265145C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651468 size=112
    let mut pc: u32 = 0x82651468;
    'dispatch: loop {
        match pc {
            0x82651468 => {
    //   block [0x82651468..0x826514D8)
	// 82651468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265146C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651474: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651478: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265147C: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 82651480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651484: 390B60E8  addi r8, r11, 0x60e8
	ctx.r[8].s64 = ctx.r[11].s64 + 24808;
	// 82651488: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265148C: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 82651490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651494: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265149C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826514A0: 386A48A0  addi r3, r10, 0x48a0
	ctx.r[3].s64 = ctx.r[10].s64 + 18592;
	// 826514A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826514A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826514AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826514B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826514B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826514B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826514BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826514C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826514C4: 4BE1595D  bl 0x82466e20
	ctx.lr = 0x826514C8;
	sub_82466E20(ctx, base);
	// 826514C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826514CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826514D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826514D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826514D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826514D8 size=112
    let mut pc: u32 = 0x826514D8;
    'dispatch: loop {
        match pc {
            0x826514D8 => {
    //   block [0x826514D8..0x82651548)
	// 826514D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826514DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826514E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826514E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826514E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826514EC: 38AA39A0  addi r5, r10, 0x39a0
	ctx.r[5].s64 = ctx.r[10].s64 + 14752;
	// 826514F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826514F4: 390B6100  addi r8, r11, 0x6100
	ctx.r[8].s64 = ctx.r[11].s64 + 24832;
	// 826514F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826514FC: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82651500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651504: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265150C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651510: 386A48D0  addi r3, r10, 0x48d0
	ctx.r[3].s64 = ctx.r[10].s64 + 18640;
	// 82651514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265151C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265152C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651534: 4BE158ED  bl 0x82466e20
	ctx.lr = 0x82651538;
	sub_82466E20(ctx, base);
	// 82651538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265153C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651548 size=112
    let mut pc: u32 = 0x82651548;
    'dispatch: loop {
        match pc {
            0x82651548 => {
    //   block [0x82651548..0x826515B8)
	// 82651548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265154C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651554: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651558: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265155C: 38AA39A0  addi r5, r10, 0x39a0
	ctx.r[5].s64 = ctx.r[10].s64 + 14752;
	// 82651560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651564: 390B6148  addi r8, r11, 0x6148
	ctx.r[8].s64 = ctx.r[11].s64 + 24904;
	// 82651568: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265156C: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 82651570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651574: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651578: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265157C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651580: 386A4900  addi r3, r10, 0x4900
	ctx.r[3].s64 = ctx.r[10].s64 + 18688;
	// 82651584: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265158C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265159C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826515A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826515A4: 4BE1587D  bl 0x82466e20
	ctx.lr = 0x826515A8;
	sub_82466E20(ctx, base);
	// 826515A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826515AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826515B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826515B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826515B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826515B8 size=112
    let mut pc: u32 = 0x826515B8;
    'dispatch: loop {
        match pc {
            0x826515B8 => {
    //   block [0x826515B8..0x82651628)
	// 826515B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826515BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826515C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826515C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826515C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826515CC: 38AA39D0  addi r5, r10, 0x39d0
	ctx.r[5].s64 = ctx.r[10].s64 + 14800;
	// 826515D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826515D4: 390B61A8  addi r8, r11, 0x61a8
	ctx.r[8].s64 = ctx.r[11].s64 + 25000;
	// 826515D8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826515DC: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826515E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826515E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826515E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826515EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826515F0: 386A4930  addi r3, r10, 0x4930
	ctx.r[3].s64 = ctx.r[10].s64 + 18736;
	// 826515F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826515F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826515FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265160C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651614: 4BE1580D  bl 0x82466e20
	ctx.lr = 0x82651618;
	sub_82466E20(ctx, base);
	// 82651618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265161C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651628 size=112
    let mut pc: u32 = 0x82651628;
    'dispatch: loop {
        match pc {
            0x82651628 => {
    //   block [0x82651628..0x82651698)
	// 82651628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265162C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651634: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651638: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265163C: 38AA39D0  addi r5, r10, 0x39d0
	ctx.r[5].s64 = ctx.r[10].s64 + 14800;
	// 82651640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651644: 390B6208  addi r8, r11, 0x6208
	ctx.r[8].s64 = ctx.r[11].s64 + 25096;
	// 82651648: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265164C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82651650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651654: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265165C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651660: 386A4960  addi r3, r10, 0x4960
	ctx.r[3].s64 = ctx.r[10].s64 + 18784;
	// 82651664: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265166C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265167C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651684: 4BE1579D  bl 0x82466e20
	ctx.lr = 0x82651688;
	sub_82466E20(ctx, base);
	// 82651688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265168C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651698 size=112
    let mut pc: u32 = 0x82651698;
    'dispatch: loop {
        match pc {
            0x82651698 => {
    //   block [0x82651698..0x82651708)
	// 82651698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265169C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826516A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826516A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826516A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826516AC: 38AA39A0  addi r5, r10, 0x39a0
	ctx.r[5].s64 = ctx.r[10].s64 + 14752;
	// 826516B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826516B4: 390B6268  addi r8, r11, 0x6268
	ctx.r[8].s64 = ctx.r[11].s64 + 25192;
	// 826516B8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826516BC: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826516C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826516C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826516C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826516CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826516D0: 386A4990  addi r3, r10, 0x4990
	ctx.r[3].s64 = ctx.r[10].s64 + 18832;
	// 826516D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826516D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826516DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826516E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826516E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826516E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826516EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826516F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826516F4: 4BE1572D  bl 0x82466e20
	ctx.lr = 0x826516F8;
	sub_82466E20(ctx, base);
	// 826516F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826516FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651708 size=112
    let mut pc: u32 = 0x82651708;
    'dispatch: loop {
        match pc {
            0x82651708 => {
    //   block [0x82651708..0x82651778)
	// 82651708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265170C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651714: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82651718: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8265171C: 38EA6328  addi r7, r10, 0x6328
	ctx.r[7].s64 = ctx.r[10].s64 + 25384;
	// 82651720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651724: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82651728: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 8265172C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651730: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651734: 396BA838  addi r11, r11, -0x57c8
	ctx.r[11].s64 = ctx.r[11].s64 + -22472;
	// 82651738: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265173C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651740: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651744: 386A49C0  addi r3, r10, 0x49c0
	ctx.r[3].s64 = ctx.r[10].s64 + 18880;
	// 82651748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265174C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82651750: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651754: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82651758: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265175C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651760: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651764: 4BE156BD  bl 0x82466e20
	ctx.lr = 0x82651768;
	sub_82466E20(ctx, base);
	// 82651768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265176C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651778 size=112
    let mut pc: u32 = 0x82651778;
    'dispatch: loop {
        match pc {
            0x82651778 => {
    //   block [0x82651778..0x826517E8)
	// 82651778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265177C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651784: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651788: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265178C: 38AA29E0  addi r5, r10, 0x29e0
	ctx.r[5].s64 = ctx.r[10].s64 + 10720;
	// 82651790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651794: 390B64F0  addi r8, r11, 0x64f0
	ctx.r[8].s64 = ctx.r[11].s64 + 25840;
	// 82651798: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265179C: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826517A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826517A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826517A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826517AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826517B0: 386A49F0  addi r3, r10, 0x49f0
	ctx.r[3].s64 = ctx.r[10].s64 + 18928;
	// 826517B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826517B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826517BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826517C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826517C4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826517C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826517CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826517D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826517D4: 4BE1564D  bl 0x82466e20
	ctx.lr = 0x826517D8;
	sub_82466E20(ctx, base);
	// 826517D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826517DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826517E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826517E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826517E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826517E8 size=112
    let mut pc: u32 = 0x826517E8;
    'dispatch: loop {
        match pc {
            0x826517E8 => {
    //   block [0x826517E8..0x82651858)
	// 826517E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826517EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826517F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826517F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826517F8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826517FC: 38AA29E0  addi r5, r10, 0x29e0
	ctx.r[5].s64 = ctx.r[10].s64 + 10720;
	// 82651800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651804: 390B6508  addi r8, r11, 0x6508
	ctx.r[8].s64 = ctx.r[11].s64 + 25864;
	// 82651808: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265180C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82651810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651814: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265181C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651820: 386A4A20  addi r3, r10, 0x4a20
	ctx.r[3].s64 = ctx.r[10].s64 + 18976;
	// 82651824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265182C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651834: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82651838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265183C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651844: 4BE155DD  bl 0x82466e20
	ctx.lr = 0x82651848;
	sub_82466E20(ctx, base);
	// 82651848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265184C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651858 size=112
    let mut pc: u32 = 0x82651858;
    'dispatch: loop {
        match pc {
            0x82651858 => {
    //   block [0x82651858..0x826518C8)
	// 82651858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265185C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651868: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265186C: 38AA29E0  addi r5, r10, 0x29e0
	ctx.r[5].s64 = ctx.r[10].s64 + 10720;
	// 82651870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651874: 390B6520  addi r8, r11, 0x6520
	ctx.r[8].s64 = ctx.r[11].s64 + 25888;
	// 82651878: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265187C: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 82651880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651884: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265188C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651890: 386A4A50  addi r3, r10, 0x4a50
	ctx.r[3].s64 = ctx.r[10].s64 + 19024;
	// 82651894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265189C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826518A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826518A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826518A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826518AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826518B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826518B4: 4BE1556D  bl 0x82466e20
	ctx.lr = 0x826518B8;
	sub_82466E20(ctx, base);
	// 826518B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826518BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826518C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826518C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826518C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826518C8 size=108
    let mut pc: u32 = 0x826518C8;
    'dispatch: loop {
        match pc {
            0x826518C8 => {
    //   block [0x826518C8..0x82651934)
	// 826518C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826518CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826518D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826518D4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826518D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826518DC: 38EB6550  addi r7, r11, 0x6550
	ctx.r[7].s64 = ctx.r[11].s64 + 25936;
	// 826518E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826518E4: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 826518E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826518EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826518F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826518F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826518F8: 386A4A80  addi r3, r10, 0x4a80
	ctx.r[3].s64 = ctx.r[10].s64 + 19072;
	// 826518FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265190C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265191C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651920: 4BE15501  bl 0x82466e20
	ctx.lr = 0x82651924;
	sub_82466E20(ctx, base);
	// 82651924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265192C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651938 size=112
    let mut pc: u32 = 0x82651938;
    'dispatch: loop {
        match pc {
            0x82651938 => {
    //   block [0x82651938..0x826519A8)
	// 82651938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265193C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651944: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651948: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265194C: 38AA29E0  addi r5, r10, 0x29e0
	ctx.r[5].s64 = ctx.r[10].s64 + 10720;
	// 82651950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651954: 390B6580  addi r8, r11, 0x6580
	ctx.r[8].s64 = ctx.r[11].s64 + 25984;
	// 82651958: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265195C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 82651960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651964: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265196C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651970: 386A4AB0  addi r3, r10, 0x4ab0
	ctx.r[3].s64 = ctx.r[10].s64 + 19120;
	// 82651974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265197C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651984: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82651988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265198C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651994: 4BE1548D  bl 0x82466e20
	ctx.lr = 0x82651998;
	sub_82466E20(ctx, base);
	// 82651998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265199C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826519A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826519A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826519A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826519A8 size=108
    let mut pc: u32 = 0x826519A8;
    'dispatch: loop {
        match pc {
            0x826519A8 => {
    //   block [0x826519A8..0x82651A14)
	// 826519A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826519AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826519B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826519B4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826519B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826519BC: 38EB6598  addi r7, r11, 0x6598
	ctx.r[7].s64 = ctx.r[11].s64 + 26008;
	// 826519C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826519C4: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 826519C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826519CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826519D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826519D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826519D8: 386A4AE0  addi r3, r10, 0x4ae0
	ctx.r[3].s64 = ctx.r[10].s64 + 19168;
	// 826519DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826519E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826519E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826519E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826519EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826519F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826519F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826519F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826519FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651A00: 4BE15421  bl 0x82466e20
	ctx.lr = 0x82651A04;
	sub_82466E20(ctx, base);
	// 82651A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651A18 size=108
    let mut pc: u32 = 0x82651A18;
    'dispatch: loop {
        match pc {
            0x82651A18 => {
    //   block [0x82651A18..0x82651A84)
	// 82651A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651A24: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651A2C: 38EB65C8  addi r7, r11, 0x65c8
	ctx.r[7].s64 = ctx.r[11].s64 + 26056;
	// 82651A30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82651A34: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 82651A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651A3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651A40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82651A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651A48: 386A4B10  addi r3, r10, 0x4b10
	ctx.r[3].s64 = ctx.r[10].s64 + 19216;
	// 82651A4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651A6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651A70: 4BE153B1  bl 0x82466e20
	ctx.lr = 0x82651A74;
	sub_82466E20(ctx, base);
	// 82651A74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651A88 size=112
    let mut pc: u32 = 0x82651A88;
    'dispatch: loop {
        match pc {
            0x82651A88 => {
    //   block [0x82651A88..0x82651AF8)
	// 82651A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651A94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651A98: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651A9C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651AA4: 390B6610  addi r8, r11, 0x6610
	ctx.r[8].s64 = ctx.r[11].s64 + 26128;
	// 82651AA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82651AAC: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 82651AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651AB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651AB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651AC0: 386A4B40  addi r3, r10, 0x4b40
	ctx.r[3].s64 = ctx.r[10].s64 + 19264;
	// 82651AC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651AE4: 4BE1533D  bl 0x82466e20
	ctx.lr = 0x82651AE8;
	sub_82466E20(ctx, base);
	// 82651AE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651AF8 size=112
    let mut pc: u32 = 0x82651AF8;
    'dispatch: loop {
        match pc {
            0x82651AF8 => {
    //   block [0x82651AF8..0x82651B68)
	// 82651AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651B04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651B08: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651B0C: 38AA39D0  addi r5, r10, 0x39d0
	ctx.r[5].s64 = ctx.r[10].s64 + 14800;
	// 82651B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651B14: 390B6658  addi r8, r11, 0x6658
	ctx.r[8].s64 = ctx.r[11].s64 + 26200;
	// 82651B18: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82651B1C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82651B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651B24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651B28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651B30: 386A4B70  addi r3, r10, 0x4b70
	ctx.r[3].s64 = ctx.r[10].s64 + 19312;
	// 82651B34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651B54: 4BE152CD  bl 0x82466e20
	ctx.lr = 0x82651B58;
	sub_82466E20(ctx, base);
	// 82651B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651B68 size=108
    let mut pc: u32 = 0x82651B68;
    'dispatch: loop {
        match pc {
            0x82651B68 => {
    //   block [0x82651B68..0x82651BD4)
	// 82651B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651B74: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651B7C: 38EB66E8  addi r7, r11, 0x66e8
	ctx.r[7].s64 = ctx.r[11].s64 + 26344;
	// 82651B80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82651B84: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 82651B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651B8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651B90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82651B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651B98: 386A4BA0  addi r3, r10, 0x4ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 19360;
	// 82651B9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651BBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651BC0: 4BE15261  bl 0x82466e20
	ctx.lr = 0x82651BC4;
	sub_82466E20(ctx, base);
	// 82651BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651BD8 size=108
    let mut pc: u32 = 0x82651BD8;
    'dispatch: loop {
        match pc {
            0x82651BD8 => {
    //   block [0x82651BD8..0x82651C44)
	// 82651BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651BE4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651BE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651BEC: 38EB6730  addi r7, r11, 0x6730
	ctx.r[7].s64 = ctx.r[11].s64 + 26416;
	// 82651BF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82651BF4: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 82651BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651BFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651C00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82651C04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651C08: 386A4BD0  addi r3, r10, 0x4bd0
	ctx.r[3].s64 = ctx.r[10].s64 + 19408;
	// 82651C0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651C10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651C2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651C30: 4BE151F1  bl 0x82466e20
	ctx.lr = 0x82651C34;
	sub_82466E20(ctx, base);
	// 82651C34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651C48 size=108
    let mut pc: u32 = 0x82651C48;
    'dispatch: loop {
        match pc {
            0x82651C48 => {
    //   block [0x82651C48..0x82651CB4)
	// 82651C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651C54: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651C58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651C5C: 38EB6760  addi r7, r11, 0x6760
	ctx.r[7].s64 = ctx.r[11].s64 + 26464;
	// 82651C60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82651C64: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 82651C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651C6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651C70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82651C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651C78: 386A4C00  addi r3, r10, 0x4c00
	ctx.r[3].s64 = ctx.r[10].s64 + 19456;
	// 82651C7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651CA0: 4BE15181  bl 0x82466e20
	ctx.lr = 0x82651CA4;
	sub_82466E20(ctx, base);
	// 82651CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651CB8 size=112
    let mut pc: u32 = 0x82651CB8;
    'dispatch: loop {
        match pc {
            0x82651CB8 => {
    //   block [0x82651CB8..0x82651D28)
	// 82651CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651CC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651CC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651CCC: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651CD4: 390B6790  addi r8, r11, 0x6790
	ctx.r[8].s64 = ctx.r[11].s64 + 26512;
	// 82651CD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82651CDC: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 82651CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651CE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651CE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651CF0: 386A4C30  addi r3, r10, 0x4c30
	ctx.r[3].s64 = ctx.r[10].s64 + 19504;
	// 82651CF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651D14: 4BE1510D  bl 0x82466e20
	ctx.lr = 0x82651D18;
	sub_82466E20(ctx, base);
	// 82651D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651D28 size=112
    let mut pc: u32 = 0x82651D28;
    'dispatch: loop {
        match pc {
            0x82651D28 => {
    //   block [0x82651D28..0x82651D98)
	// 82651D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651D34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651D38: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651D3C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651D44: 390B67C0  addi r8, r11, 0x67c0
	ctx.r[8].s64 = ctx.r[11].s64 + 26560;
	// 82651D48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82651D4C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82651D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651D54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651D58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651D60: 386A4C60  addi r3, r10, 0x4c60
	ctx.r[3].s64 = ctx.r[10].s64 + 19552;
	// 82651D64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651D84: 4BE1509D  bl 0x82466e20
	ctx.lr = 0x82651D88;
	sub_82466E20(ctx, base);
	// 82651D88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651D98 size=112
    let mut pc: u32 = 0x82651D98;
    'dispatch: loop {
        match pc {
            0x82651D98 => {
    //   block [0x82651D98..0x82651E08)
	// 82651D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651DA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651DA8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651DAC: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651DB4: 390B67D8  addi r8, r11, 0x67d8
	ctx.r[8].s64 = ctx.r[11].s64 + 26584;
	// 82651DB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82651DBC: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 82651DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651DC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651DC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651DD0: 386A4C90  addi r3, r10, 0x4c90
	ctx.r[3].s64 = ctx.r[10].s64 + 19600;
	// 82651DD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651DDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651DF4: 4BE1502D  bl 0x82466e20
	ctx.lr = 0x82651DF8;
	sub_82466E20(ctx, base);
	// 82651DF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651E08 size=108
    let mut pc: u32 = 0x82651E08;
    'dispatch: loop {
        match pc {
            0x82651E08 => {
    //   block [0x82651E08..0x82651E74)
	// 82651E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651E14: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651E18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651E1C: 38EB67F0  addi r7, r11, 0x67f0
	ctx.r[7].s64 = ctx.r[11].s64 + 26608;
	// 82651E20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82651E24: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82651E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651E2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651E30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82651E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651E38: 386A4CC0  addi r3, r10, 0x4cc0
	ctx.r[3].s64 = ctx.r[10].s64 + 19648;
	// 82651E3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651E40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651E44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651E48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651E50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651E54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651E58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651E5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651E60: 4BE14FC1  bl 0x82466e20
	ctx.lr = 0x82651E64;
	sub_82466E20(ctx, base);
	// 82651E64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651E78 size=112
    let mut pc: u32 = 0x82651E78;
    'dispatch: loop {
        match pc {
            0x82651E78 => {
    //   block [0x82651E78..0x82651EE8)
	// 82651E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651E84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651E88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651E8C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651E94: 390B6820  addi r8, r11, 0x6820
	ctx.r[8].s64 = ctx.r[11].s64 + 26656;
	// 82651E98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82651E9C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 82651EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651EA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651EB0: 386A4CF0  addi r3, r10, 0x4cf0
	ctx.r[3].s64 = ctx.r[10].s64 + 19696;
	// 82651EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651ED4: 4BE14F4D  bl 0x82466e20
	ctx.lr = 0x82651ED8;
	sub_82466E20(ctx, base);
	// 82651ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651EE8 size=108
    let mut pc: u32 = 0x82651EE8;
    'dispatch: loop {
        match pc {
            0x82651EE8 => {
    //   block [0x82651EE8..0x82651F54)
	// 82651EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651EF4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651EF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651EFC: 38EB6838  addi r7, r11, 0x6838
	ctx.r[7].s64 = ctx.r[11].s64 + 26680;
	// 82651F00: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82651F04: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 82651F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651F0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651F10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82651F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651F18: 386A4D20  addi r3, r10, 0x4d20
	ctx.r[3].s64 = ctx.r[10].s64 + 19744;
	// 82651F1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651F20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651F3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651F40: 4BE14EE1  bl 0x82466e20
	ctx.lr = 0x82651F44;
	sub_82466E20(ctx, base);
	// 82651F44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651F58 size=112
    let mut pc: u32 = 0x82651F58;
    'dispatch: loop {
        match pc {
            0x82651F58 => {
    //   block [0x82651F58..0x82651FC8)
	// 82651F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651F64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651F68: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651F6C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651F74: 390B6928  addi r8, r11, 0x6928
	ctx.r[8].s64 = ctx.r[11].s64 + 26920;
	// 82651F78: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 82651F7C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 82651F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651F84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651F88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651F90: 386A4D50  addi r3, r10, 0x4d50
	ctx.r[3].s64 = ctx.r[10].s64 + 19792;
	// 82651F94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651FB4: 4BE14E6D  bl 0x82466e20
	ctx.lr = 0x82651FB8;
	sub_82466E20(ctx, base);
	// 82651FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651FC8 size=108
    let mut pc: u32 = 0x82651FC8;
    'dispatch: loop {
        match pc {
            0x82651FC8 => {
    //   block [0x82651FC8..0x82652034)
	// 82651FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651FD4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651FD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651FDC: 38EB6AD8  addi r7, r11, 0x6ad8
	ctx.r[7].s64 = ctx.r[11].s64 + 27352;
	// 82651FE0: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82651FE4: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 82651FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651FEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651FF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82651FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651FF8: 386A4D80  addi r3, r10, 0x4d80
	ctx.r[3].s64 = ctx.r[10].s64 + 19840;
	// 82651FFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265200C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265201C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652020: 4BE14E01  bl 0x82466e20
	ctx.lr = 0x82652024;
	sub_82466E20(ctx, base);
	// 82652024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265202C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652038 size=112
    let mut pc: u32 = 0x82652038;
    'dispatch: loop {
        match pc {
            0x82652038 => {
    //   block [0x82652038..0x826520A8)
	// 82652038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265203C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652044: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652048: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265204C: 38AA39D0  addi r5, r10, 0x39d0
	ctx.r[5].s64 = ctx.r[10].s64 + 14800;
	// 82652050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652054: 390B6C70  addi r8, r11, 0x6c70
	ctx.r[8].s64 = ctx.r[11].s64 + 27760;
	// 82652058: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8265205C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 82652060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652064: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265206C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652070: 386A4DB0  addi r3, r10, 0x4db0
	ctx.r[3].s64 = ctx.r[10].s64 + 19888;
	// 82652074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265207C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265208C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652094: 4BE14D8D  bl 0x82466e20
	ctx.lr = 0x82652098;
	sub_82466E20(ctx, base);
	// 82652098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265209C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826520A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826520A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826520A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826520A8 size=100
    let mut pc: u32 = 0x826520A8;
    'dispatch: loop {
        match pc {
            0x826520A8 => {
    //   block [0x826520A8..0x8265210C)
	// 826520A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826520AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826520B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826520B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826520B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826520BC: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 826520C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826520C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826520C8: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826520CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826520D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826520D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826520D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826520DC: 386A4DE0  addi r3, r10, 0x4de0
	ctx.r[3].s64 = ctx.r[10].s64 + 19936;
	// 826520E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826520E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826520E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826520EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826520F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826520F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826520F8: 4BE14D29  bl 0x82466e20
	ctx.lr = 0x826520FC;
	sub_82466E20(ctx, base);
	// 826520FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652110 size=112
    let mut pc: u32 = 0x82652110;
    'dispatch: loop {
        match pc {
            0x82652110 => {
    //   block [0x82652110..0x82652180)
	// 82652110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265211C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652120: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652124: 38AA4DE0  addi r5, r10, 0x4de0
	ctx.r[5].s64 = ctx.r[10].s64 + 19936;
	// 82652128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265212C: 390B6EC8  addi r8, r11, 0x6ec8
	ctx.r[8].s64 = ctx.r[11].s64 + 28360;
	// 82652130: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82652134: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82652138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265213C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652148: 386A4E10  addi r3, r10, 0x4e10
	ctx.r[3].s64 = ctx.r[10].s64 + 19984;
	// 8265214C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265215C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265216C: 4BE14CB5  bl 0x82466e20
	ctx.lr = 0x82652170;
	sub_82466E20(ctx, base);
	// 82652170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265217C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652180 size=100
    let mut pc: u32 = 0x82652180;
    'dispatch: loop {
        match pc {
            0x82652180 => {
    //   block [0x82652180..0x826521E4)
	// 82652180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265218C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652194: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265219C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826521A0: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 826521A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826521A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826521AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826521B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826521B4: 386A4E40  addi r3, r10, 0x4e40
	ctx.r[3].s64 = ctx.r[10].s64 + 20032;
	// 826521B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826521BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826521C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826521C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826521C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826521CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826521D0: 4BE14C51  bl 0x82466e20
	ctx.lr = 0x826521D4;
	sub_82466E20(ctx, base);
	// 826521D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826521D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826521DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826521E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826521E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826521E8 size=108
    let mut pc: u32 = 0x826521E8;
    'dispatch: loop {
        match pc {
            0x826521E8 => {
    //   block [0x826521E8..0x82652254)
	// 826521E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826521EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826521F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826521F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826521F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826521FC: 38EB6F40  addi r7, r11, 0x6f40
	ctx.r[7].s64 = ctx.r[11].s64 + 28480;
	// 82652200: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82652204: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 82652208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265220C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652210: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82652214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652218: 386A4E70  addi r3, r10, 0x4e70
	ctx.r[3].s64 = ctx.r[10].s64 + 20080;
	// 8265221C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265222C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265223C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652240: 4BE14BE1  bl 0x82466e20
	ctx.lr = 0x82652244;
	sub_82466E20(ctx, base);
	// 82652244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265224C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652258 size=112
    let mut pc: u32 = 0x82652258;
    'dispatch: loop {
        match pc {
            0x82652258 => {
    //   block [0x82652258..0x826522C8)
	// 82652258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265225C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652264: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652268: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265226C: 38AA4E40  addi r5, r10, 0x4e40
	ctx.r[5].s64 = ctx.r[10].s64 + 20032;
	// 82652270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652274: 390B6F88  addi r8, r11, 0x6f88
	ctx.r[8].s64 = ctx.r[11].s64 + 28552;
	// 82652278: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265227C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 82652280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652284: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265228C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652290: 386A4EA0  addi r3, r10, 0x4ea0
	ctx.r[3].s64 = ctx.r[10].s64 + 20128;
	// 82652294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265229C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826522A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826522A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826522A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826522AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826522B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826522B4: 4BE14B6D  bl 0x82466e20
	ctx.lr = 0x826522B8;
	sub_82466E20(ctx, base);
	// 826522B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826522BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826522C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826522C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826522C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826522C8 size=100
    let mut pc: u32 = 0x826522C8;
    'dispatch: loop {
        match pc {
            0x826522C8 => {
    //   block [0x826522C8..0x8265232C)
	// 826522C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826522CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826522D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826522D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826522D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826522DC: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 826522E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826522E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826522E8: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826522EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826522F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826522F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826522F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826522FC: 386A4ED0  addi r3, r10, 0x4ed0
	ctx.r[3].s64 = ctx.r[10].s64 + 20176;
	// 82652300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652304: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652308: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265230C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652310: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82652314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652318: 4BE14B09  bl 0x82466e20
	ctx.lr = 0x8265231C;
	sub_82466E20(ctx, base);
	// 8265231C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652330 size=100
    let mut pc: u32 = 0x82652330;
    'dispatch: loop {
        match pc {
            0x82652330 => {
    //   block [0x82652330..0x82652394)
	// 82652330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265233C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652344: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265234C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652350: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 82652354: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265235C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652364: 386A4F00  addi r3, r10, 0x4f00
	ctx.r[3].s64 = ctx.r[10].s64 + 20224;
	// 82652368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265236C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652370: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82652374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652378: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265237C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652380: 4BE14AA1  bl 0x82466e20
	ctx.lr = 0x82652384;
	sub_82466E20(ctx, base);
	// 82652384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265238C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652398 size=112
    let mut pc: u32 = 0x82652398;
    'dispatch: loop {
        match pc {
            0x82652398 => {
    //   block [0x82652398..0x82652408)
	// 82652398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265239C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826523A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826523A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826523A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826523AC: 38AA4ED0  addi r5, r10, 0x4ed0
	ctx.r[5].s64 = ctx.r[10].s64 + 20176;
	// 826523B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826523B4: 390B6FB8  addi r8, r11, 0x6fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 28600;
	// 826523B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826523BC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826523C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826523C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826523C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826523CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826523D0: 386A4F30  addi r3, r10, 0x4f30
	ctx.r[3].s64 = ctx.r[10].s64 + 20272;
	// 826523D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826523D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826523DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826523E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826523E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826523E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826523EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826523F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826523F4: 4BE14A2D  bl 0x82466e20
	ctx.lr = 0x826523F8;
	sub_82466E20(ctx, base);
	// 826523F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826523FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652408 size=112
    let mut pc: u32 = 0x82652408;
    'dispatch: loop {
        match pc {
            0x82652408 => {
    //   block [0x82652408..0x82652478)
	// 82652408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265240C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652414: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652418: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265241C: 38AA4F00  addi r5, r10, 0x4f00
	ctx.r[5].s64 = ctx.r[10].s64 + 20224;
	// 82652420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652424: 390B7018  addi r8, r11, 0x7018
	ctx.r[8].s64 = ctx.r[11].s64 + 28696;
	// 82652428: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265242C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82652430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652434: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265243C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652440: 386A4F60  addi r3, r10, 0x4f60
	ctx.r[3].s64 = ctx.r[10].s64 + 20320;
	// 82652444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265244C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265245C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652464: 4BE149BD  bl 0x82466e20
	ctx.lr = 0x82652468;
	sub_82466E20(ctx, base);
	// 82652468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265246C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652478 size=100
    let mut pc: u32 = 0x82652478;
    'dispatch: loop {
        match pc {
            0x82652478 => {
    //   block [0x82652478..0x826524DC)
	// 82652478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265247C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652484: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265248C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652498: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8265249C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826524A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826524A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826524A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826524AC: 386A4F90  addi r3, r10, 0x4f90
	ctx.r[3].s64 = ctx.r[10].s64 + 20368;
	// 826524B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826524B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826524B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826524BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826524C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826524C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826524C8: 4BE14959  bl 0x82466e20
	ctx.lr = 0x826524CC;
	sub_82466E20(ctx, base);
	// 826524CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826524D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826524D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826524D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826524E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826524E0 size=112
    let mut pc: u32 = 0x826524E0;
    'dispatch: loop {
        match pc {
            0x826524E0 => {
    //   block [0x826524E0..0x82652550)
	// 826524E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826524E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826524E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826524EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826524F0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826524F4: 38AA4F90  addi r5, r10, 0x4f90
	ctx.r[5].s64 = ctx.r[10].s64 + 20368;
	// 826524F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826524FC: 390B7078  addi r8, r11, 0x7078
	ctx.r[8].s64 = ctx.r[11].s64 + 28792;
	// 82652500: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82652504: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 82652508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265250C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652518: 386A4FC0  addi r3, r10, 0x4fc0
	ctx.r[3].s64 = ctx.r[10].s64 + 20416;
	// 8265251C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265252C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265253C: 4BE148E5  bl 0x82466e20
	ctx.lr = 0x82652540;
	sub_82466E20(ctx, base);
	// 82652540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265254C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652550 size=108
    let mut pc: u32 = 0x82652550;
    'dispatch: loop {
        match pc {
            0x82652550 => {
    //   block [0x82652550..0x826525BC)
	// 82652550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265255C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652564: 38EB7168  addi r7, r11, 0x7168
	ctx.r[7].s64 = ctx.r[11].s64 + 29032;
	// 82652568: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8265256C: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 82652570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652574: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652578: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265257C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652580: 386A4FF0  addi r3, r10, 0x4ff0
	ctx.r[3].s64 = ctx.r[10].s64 + 20464;
	// 82652584: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265258C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265259C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826525A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826525A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826525A8: 4BE14879  bl 0x82466e20
	ctx.lr = 0x826525AC;
	sub_82466E20(ctx, base);
	// 826525AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826525B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826525B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826525B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826525C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826525C0 size=108
    let mut pc: u32 = 0x826525C0;
    'dispatch: loop {
        match pc {
            0x826525C0 => {
    //   block [0x826525C0..0x8265262C)
	// 826525C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826525C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826525C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826525CC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826525D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826525D4: 38EB7258  addi r7, r11, 0x7258
	ctx.r[7].s64 = ctx.r[11].s64 + 29272;
	// 826525D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826525DC: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826525E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826525E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826525E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826525EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826525F0: 386A5020  addi r3, r10, 0x5020
	ctx.r[3].s64 = ctx.r[10].s64 + 20512;
	// 826525F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826525F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826525FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265260C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652618: 4BE14809  bl 0x82466e20
	ctx.lr = 0x8265261C;
	sub_82466E20(ctx, base);
	// 8265261C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652630 size=108
    let mut pc: u32 = 0x82652630;
    'dispatch: loop {
        match pc {
            0x82652630 => {
    //   block [0x82652630..0x8265269C)
	// 82652630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265263C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652644: 38EB72A0  addi r7, r11, 0x72a0
	ctx.r[7].s64 = ctx.r[11].s64 + 29344;
	// 82652648: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8265264C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82652650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652654: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265265C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652660: 386A5050  addi r3, r10, 0x5050
	ctx.r[3].s64 = ctx.r[10].s64 + 20560;
	// 82652664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265266C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265267C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652688: 4BE14799  bl 0x82466e20
	ctx.lr = 0x8265268C;
	sub_82466E20(ctx, base);
	// 8265268C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826526A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826526A0 size=108
    let mut pc: u32 = 0x826526A0;
    'dispatch: loop {
        match pc {
            0x826526A0 => {
    //   block [0x826526A0..0x8265270C)
	// 826526A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826526A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826526A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826526AC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826526B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826526B4: 38EB7378  addi r7, r11, 0x7378
	ctx.r[7].s64 = ctx.r[11].s64 + 29560;
	// 826526B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826526BC: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826526C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826526C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826526C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826526CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826526D0: 386A5080  addi r3, r10, 0x5080
	ctx.r[3].s64 = ctx.r[10].s64 + 20608;
	// 826526D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826526D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826526DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826526E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826526E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826526E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826526EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826526F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826526F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826526F8: 4BE14729  bl 0x82466e20
	ctx.lr = 0x826526FC;
	sub_82466E20(ctx, base);
	// 826526FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652710 size=100
    let mut pc: u32 = 0x82652710;
    'dispatch: loop {
        match pc {
            0x82652710 => {
    //   block [0x82652710..0x82652774)
	// 82652710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265271C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652724: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652728: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265272C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652730: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 82652734: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265273C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652744: 386A50B0  addi r3, r10, 0x50b0
	ctx.r[3].s64 = ctx.r[10].s64 + 20656;
	// 82652748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265274C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652750: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82652754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652758: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265275C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652760: 4BE146C1  bl 0x82466e20
	ctx.lr = 0x82652764;
	sub_82466E20(ctx, base);
	// 82652764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265276C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652778 size=112
    let mut pc: u32 = 0x82652778;
    'dispatch: loop {
        match pc {
            0x82652778 => {
    //   block [0x82652778..0x826527E8)
	// 82652778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265277C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652784: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652788: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265278C: 38AA50B0  addi r5, r10, 0x50b0
	ctx.r[5].s64 = ctx.r[10].s64 + 20656;
	// 82652790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652794: 390B7390  addi r8, r11, 0x7390
	ctx.r[8].s64 = ctx.r[11].s64 + 29584;
	// 82652798: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265279C: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826527A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826527A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826527A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826527AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826527B0: 386A50E0  addi r3, r10, 0x50e0
	ctx.r[3].s64 = ctx.r[10].s64 + 20704;
	// 826527B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826527B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826527BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826527C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826527C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826527C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826527CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826527D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826527D4: 4BE1464D  bl 0x82466e20
	ctx.lr = 0x826527D8;
	sub_82466E20(ctx, base);
	// 826527D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826527DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826527E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826527E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826527E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826527E8 size=108
    let mut pc: u32 = 0x826527E8;
    'dispatch: loop {
        match pc {
            0x826527E8 => {
    //   block [0x826527E8..0x82652854)
	// 826527E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826527EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826527F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826527F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826527F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826527FC: 38EB73D8  addi r7, r11, 0x73d8
	ctx.r[7].s64 = ctx.r[11].s64 + 29656;
	// 82652800: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82652804: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82652808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265280C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82652814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652818: 386A5110  addi r3, r10, 0x5110
	ctx.r[3].s64 = ctx.r[10].s64 + 20752;
	// 8265281C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265282C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265283C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652840: 4BE145E1  bl 0x82466e20
	ctx.lr = 0x82652844;
	sub_82466E20(ctx, base);
	// 82652844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265284C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652858 size=112
    let mut pc: u32 = 0x82652858;
    'dispatch: loop {
        match pc {
            0x82652858 => {
    //   block [0x82652858..0x826528C8)
	// 82652858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265285C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652868: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265286C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652874: 390B7420  addi r8, r11, 0x7420
	ctx.r[8].s64 = ctx.r[11].s64 + 29728;
	// 82652878: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265287C: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 82652880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652884: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265288C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652890: 386A5140  addi r3, r10, 0x5140
	ctx.r[3].s64 = ctx.r[10].s64 + 20800;
	// 82652894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265289C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826528A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826528A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826528A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826528AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826528B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826528B4: 4BE1456D  bl 0x82466e20
	ctx.lr = 0x826528B8;
	sub_82466E20(ctx, base);
	// 826528B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826528BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826528C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826528C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826528C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826528C8 size=108
    let mut pc: u32 = 0x826528C8;
    'dispatch: loop {
        match pc {
            0x826528C8 => {
    //   block [0x826528C8..0x82652934)
	// 826528C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826528CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826528D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826528D4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826528D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826528DC: 38EB7438  addi r7, r11, 0x7438
	ctx.r[7].s64 = ctx.r[11].s64 + 29752;
	// 826528E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826528E4: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 826528E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826528EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826528F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826528F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826528F8: 386A5170  addi r3, r10, 0x5170
	ctx.r[3].s64 = ctx.r[10].s64 + 20848;
	// 826528FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265290C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265291C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652920: 4BE14501  bl 0x82466e20
	ctx.lr = 0x82652924;
	sub_82466E20(ctx, base);
	// 82652924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265292C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652938 size=112
    let mut pc: u32 = 0x82652938;
    'dispatch: loop {
        match pc {
            0x82652938 => {
    //   block [0x82652938..0x826529A8)
	// 82652938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265293C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652944: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652948: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265294C: 38AA5140  addi r5, r10, 0x5140
	ctx.r[5].s64 = ctx.r[10].s64 + 20800;
	// 82652950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652954: 390B7480  addi r8, r11, 0x7480
	ctx.r[8].s64 = ctx.r[11].s64 + 29824;
	// 82652958: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265295C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82652960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652964: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265296C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652970: 386A51A0  addi r3, r10, 0x51a0
	ctx.r[3].s64 = ctx.r[10].s64 + 20896;
	// 82652974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265297C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265298C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652994: 4BE1448D  bl 0x82466e20
	ctx.lr = 0x82652998;
	sub_82466E20(ctx, base);
	// 82652998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265299C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826529A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826529A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826529A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826529A8 size=100
    let mut pc: u32 = 0x826529A8;
    'dispatch: loop {
        match pc {
            0x826529A8 => {
    //   block [0x826529A8..0x82652A0C)
	// 826529A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826529AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826529B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826529B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826529B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826529BC: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 826529C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826529C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826529C8: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 826529CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826529D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826529D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826529D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826529DC: 386A51D0  addi r3, r10, 0x51d0
	ctx.r[3].s64 = ctx.r[10].s64 + 20944;
	// 826529E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826529E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826529E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826529EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826529F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826529F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826529F8: 4BE14429  bl 0x82466e20
	ctx.lr = 0x826529FC;
	sub_82466E20(ctx, base);
	// 826529FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652A10 size=112
    let mut pc: u32 = 0x82652A10;
    'dispatch: loop {
        match pc {
            0x82652A10 => {
    //   block [0x82652A10..0x82652A80)
	// 82652A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652A1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652A20: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652A24: 38AA51D0  addi r5, r10, 0x51d0
	ctx.r[5].s64 = ctx.r[10].s64 + 20944;
	// 82652A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652A2C: 390B7498  addi r8, r11, 0x7498
	ctx.r[8].s64 = ctx.r[11].s64 + 29848;
	// 82652A30: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82652A34: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82652A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652A3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652A40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652A48: 386A5200  addi r3, r10, 0x5200
	ctx.r[3].s64 = ctx.r[10].s64 + 20992;
	// 82652A4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652A6C: 4BE143B5  bl 0x82466e20
	ctx.lr = 0x82652A70;
	sub_82466E20(ctx, base);
	// 82652A70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652A80 size=108
    let mut pc: u32 = 0x82652A80;
    'dispatch: loop {
        match pc {
            0x82652A80 => {
    //   block [0x82652A80..0x82652AEC)
	// 82652A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652A8C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652A94: 38EB7540  addi r7, r11, 0x7540
	ctx.r[7].s64 = ctx.r[11].s64 + 30016;
	// 82652A98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82652A9C: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 82652AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652AA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82652AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652AB0: 386A5230  addi r3, r10, 0x5230
	ctx.r[3].s64 = ctx.r[10].s64 + 21040;
	// 82652AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652AD8: 4BE14349  bl 0x82466e20
	ctx.lr = 0x82652ADC;
	sub_82466E20(ctx, base);
	// 82652ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652AF0 size=112
    let mut pc: u32 = 0x82652AF0;
    'dispatch: loop {
        match pc {
            0x82652AF0 => {
    //   block [0x82652AF0..0x82652B60)
	// 82652AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652AFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652B00: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652B04: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652B0C: 390B7570  addi r8, r11, 0x7570
	ctx.r[8].s64 = ctx.r[11].s64 + 30064;
	// 82652B10: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82652B14: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82652B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652B1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652B20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652B28: 386A5260  addi r3, r10, 0x5260
	ctx.r[3].s64 = ctx.r[10].s64 + 21088;
	// 82652B2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652B34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652B4C: 4BE142D5  bl 0x82466e20
	ctx.lr = 0x82652B50;
	sub_82466E20(ctx, base);
	// 82652B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652B60 size=112
    let mut pc: u32 = 0x82652B60;
    'dispatch: loop {
        match pc {
            0x82652B60 => {
    //   block [0x82652B60..0x82652BD0)
	// 82652B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652B6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652B70: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652B74: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652B7C: 390B75B8  addi r8, r11, 0x75b8
	ctx.r[8].s64 = ctx.r[11].s64 + 30136;
	// 82652B80: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82652B84: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 82652B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652B8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652B98: 386A5290  addi r3, r10, 0x5290
	ctx.r[3].s64 = ctx.r[10].s64 + 21136;
	// 82652B9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652BBC: 4BE14265  bl 0x82466e20
	ctx.lr = 0x82652BC0;
	sub_82466E20(ctx, base);
	// 82652BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652BD0 size=100
    let mut pc: u32 = 0x82652BD0;
    'dispatch: loop {
        match pc {
            0x82652BD0 => {
    //   block [0x82652BD0..0x82652C34)
	// 82652BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652BDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652BE4: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652BE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652BF0: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82652BF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652BF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652C00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652C04: 386A52C0  addi r3, r10, 0x52c0
	ctx.r[3].s64 = ctx.r[10].s64 + 21184;
	// 82652C08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652C0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652C10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82652C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652C18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82652C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652C20: 4BE14201  bl 0x82466e20
	ctx.lr = 0x82652C24;
	sub_82466E20(ctx, base);
	// 82652C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652C38 size=112
    let mut pc: u32 = 0x82652C38;
    'dispatch: loop {
        match pc {
            0x82652C38 => {
    //   block [0x82652C38..0x82652CA8)
	// 82652C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652C44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652C48: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652C4C: 38AA52C0  addi r5, r10, 0x52c0
	ctx.r[5].s64 = ctx.r[10].s64 + 21184;
	// 82652C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652C54: 390B7600  addi r8, r11, 0x7600
	ctx.r[8].s64 = ctx.r[11].s64 + 30208;
	// 82652C58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82652C5C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 82652C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652C64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652C70: 386A52F0  addi r3, r10, 0x52f0
	ctx.r[3].s64 = ctx.r[10].s64 + 21232;
	// 82652C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652C94: 4BE1418D  bl 0x82466e20
	ctx.lr = 0x82652C98;
	sub_82466E20(ctx, base);
	// 82652C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652CA8 size=112
    let mut pc: u32 = 0x82652CA8;
    'dispatch: loop {
        match pc {
            0x82652CA8 => {
    //   block [0x82652CA8..0x82652D18)
	// 82652CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652CB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652CB8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652CBC: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652CC4: 390B7648  addi r8, r11, 0x7648
	ctx.r[8].s64 = ctx.r[11].s64 + 30280;
	// 82652CC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82652CCC: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82652CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652CD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652CD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652CE0: 386A5320  addi r3, r10, 0x5320
	ctx.r[3].s64 = ctx.r[10].s64 + 21280;
	// 82652CE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652D04: 4BE1411D  bl 0x82466e20
	ctx.lr = 0x82652D08;
	sub_82466E20(ctx, base);
	// 82652D08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652D18 size=112
    let mut pc: u32 = 0x82652D18;
    'dispatch: loop {
        match pc {
            0x82652D18 => {
    //   block [0x82652D18..0x82652D88)
	// 82652D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652D24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652D28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652D2C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652D34: 390B7660  addi r8, r11, 0x7660
	ctx.r[8].s64 = ctx.r[11].s64 + 30304;
	// 82652D38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82652D3C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82652D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652D44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652D50: 386A5350  addi r3, r10, 0x5350
	ctx.r[3].s64 = ctx.r[10].s64 + 21328;
	// 82652D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652D64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82652D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652D74: 4BE140AD  bl 0x82466e20
	ctx.lr = 0x82652D78;
	sub_82466E20(ctx, base);
	// 82652D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652D88 size=112
    let mut pc: u32 = 0x82652D88;
    'dispatch: loop {
        match pc {
            0x82652D88 => {
    //   block [0x82652D88..0x82652DF8)
	// 82652D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652D94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652D98: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652D9C: 38AA5320  addi r5, r10, 0x5320
	ctx.r[5].s64 = ctx.r[10].s64 + 21280;
	// 82652DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652DA4: 390B7678  addi r8, r11, 0x7678
	ctx.r[8].s64 = ctx.r[11].s64 + 30328;
	// 82652DA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82652DAC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 82652DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652DB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652DC0: 386A5380  addi r3, r10, 0x5380
	ctx.r[3].s64 = ctx.r[10].s64 + 21376;
	// 82652DC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652DE4: 4BE1403D  bl 0x82466e20
	ctx.lr = 0x82652DE8;
	sub_82466E20(ctx, base);
	// 82652DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652DF8 size=72
    let mut pc: u32 = 0x82652DF8;
    'dispatch: loop {
        match pc {
            0x82652DF8 => {
    //   block [0x82652DF8..0x82652E40)
	// 82652DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652E04: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82652E08: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82652E0C: 38CB94E0  addi r6, r11, -0x6b20
	ctx.r[6].s64 = ctx.r[11].s64 + -27424;
	// 82652E10: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82652E14: 388BA890  addi r4, r11, -0x5770
	ctx.r[4].s64 = ctx.r[11].s64 + -22384;
	// 82652E18: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82652E1C: 386B53B0  addi r3, r11, 0x53b0
	ctx.r[3].s64 = ctx.r[11].s64 + 21424;
	// 82652E20: 4BE28C69  bl 0x8247ba88
	ctx.lr = 0x82652E24;
	sub_8247BA88(ctx, base);
	// 82652E24: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82652E28: 386BCDC8  addi r3, r11, -0x3238
	ctx.r[3].s64 = ctx.r[11].s64 + -12856;
	// 82652E2C: 4BEDFD0D  bl 0x82532b38
	ctx.lr = 0x82652E30;
	sub_82532B38(ctx, base);
	// 82652E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82652E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652E40 size=108
    let mut pc: u32 = 0x82652E40;
    'dispatch: loop {
        match pc {
            0x82652E40 => {
    //   block [0x82652E40..0x82652EAC)
	// 82652E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652E4C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82652E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652E54: 38EB88C0  addi r7, r11, -0x7740
	ctx.r[7].s64 = ctx.r[11].s64 + -30528;
	// 82652E58: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82652E5C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 82652E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652E64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652E68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82652E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652E70: 386A53C8  addi r3, r10, 0x53c8
	ctx.r[3].s64 = ctx.r[10].s64 + 21448;
	// 82652E74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652E94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652E98: 4BE13F89  bl 0x82466e20
	ctx.lr = 0x82652E9C;
	sub_82466E20(ctx, base);
	// 82652E9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652EA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652EA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82652EB0 size=24
    let mut pc: u32 = 0x82652EB0;
    'dispatch: loop {
        match pc {
            0x82652EB0 => {
    //   block [0x82652EB0..0x82652EC8)
	// 82652EB0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82652EB4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82652EB8: 394A0EE8  addi r10, r10, 0xee8
	ctx.r[10].s64 = ctx.r[10].s64 + 3816;
	// 82652EBC: 816B8938  lwz r11, -0x76c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30408 as u32) ) } as u64;
	// 82652EC0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82652EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652EC8 size=112
    let mut pc: u32 = 0x82652EC8;
    'dispatch: loop {
        match pc {
            0x82652EC8 => {
    //   block [0x82652EC8..0x82652F38)
	// 82652EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652ED4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82652ED8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82652EDC: 392AC82C  addi r9, r10, -0x37d4
	ctx.r[9].s64 = ctx.r[10].s64 + -14292;
	// 82652EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652EE4: 390B0EE8  addi r8, r11, 0xee8
	ctx.r[8].s64 = ctx.r[11].s64 + 3816;
	// 82652EE8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82652EEC: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 82652EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652EF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652EFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652F00: 386A53F8  addi r3, r10, 0x53f8
	ctx.r[3].s64 = ctx.r[10].s64 + 21496;
	// 82652F04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82652F08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82652F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652F14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652F1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652F24: 4BE13EFD  bl 0x82466e20
	ctx.lr = 0x82652F28;
	sub_82466E20(ctx, base);
	// 82652F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652F38 size=108
    let mut pc: u32 = 0x82652F38;
    'dispatch: loop {
        match pc {
            0x82652F38 => {
    //   block [0x82652F38..0x82652FA4)
	// 82652F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652F44: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82652F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652F4C: 38EB893C  addi r7, r11, -0x76c4
	ctx.r[7].s64 = ctx.r[11].s64 + -30404;
	// 82652F50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82652F54: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 82652F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652F5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652F60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82652F64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652F68: 386A5428  addi r3, r10, 0x5428
	ctx.r[3].s64 = ctx.r[10].s64 + 21544;
	// 82652F6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652F74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652F8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652F90: 4BE13E91  bl 0x82466e20
	ctx.lr = 0x82652F94;
	sub_82466E20(ctx, base);
	// 82652F94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652F98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652F9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652FA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652FA8 size=108
    let mut pc: u32 = 0x82652FA8;
    'dispatch: loop {
        match pc {
            0x82652FA8 => {
    //   block [0x82652FA8..0x82653014)
	// 82652FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652FB4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82652FB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652FBC: 38EB896C  addi r7, r11, -0x7694
	ctx.r[7].s64 = ctx.r[11].s64 + -30356;
	// 82652FC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82652FC4: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 82652FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652FCC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652FD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82652FD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652FD8: 386A5458  addi r3, r10, 0x5458
	ctx.r[3].s64 = ctx.r[10].s64 + 21592;
	// 82652FDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652FE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652FE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652FEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652FF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652FF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652FFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82653000: 4BE13E21  bl 0x82466e20
	ctx.lr = 0x82653004;
	sub_82466E20(ctx, base);
	// 82653004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265300C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82653018 size=24
    let mut pc: u32 = 0x82653018;
    'dispatch: loop {
        match pc {
            0x82653018 => {
    //   block [0x82653018..0x82653030)
	// 82653018: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265301C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82653020: 394A0F30  addi r10, r10, 0xf30
	ctx.r[10].s64 = ctx.r[10].s64 + 3888;
	// 82653024: 816B899C  lwz r11, -0x7664(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30308 as u32) ) } as u64;
	// 82653028: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265302C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653030 size=116
    let mut pc: u32 = 0x82653030;
    'dispatch: loop {
        match pc {
            0x82653030 => {
    //   block [0x82653030..0x826530A4)
	// 82653030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265303C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653040: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82653044: 390B0F30  addi r8, r11, 0xf30
	ctx.r[8].s64 = ctx.r[11].s64 + 3888;
	// 82653048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265304C: 392AC860  addi r9, r10, -0x37a0
	ctx.r[9].s64 = ctx.r[10].s64 + -14240;
	// 82653050: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653054: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82653058: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265305C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82653060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653064: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265306C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653074: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82653078: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 8265307C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82653080: 386B5488  addi r3, r11, 0x5488
	ctx.r[3].s64 = ctx.r[11].s64 + 21640;
	// 82653084: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82653088: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265308C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653090: 4BE13D91  bl 0x82466e20
	ctx.lr = 0x82653094;
	sub_82466E20(ctx, base);
	// 82653094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265309C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826530A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826530A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826530A8 size=108
    let mut pc: u32 = 0x826530A8;
    'dispatch: loop {
        match pc {
            0x826530A8 => {
    //   block [0x826530A8..0x82653114)
	// 826530A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826530AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826530B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826530B4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826530B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826530BC: 38EB89A0  addi r7, r11, -0x7660
	ctx.r[7].s64 = ctx.r[11].s64 + -30304;
	// 826530C0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826530C4: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 826530C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826530CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826530D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826530D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826530D8: 386A54B8  addi r3, r10, 0x54b8
	ctx.r[3].s64 = ctx.r[10].s64 + 21688;
	// 826530DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826530E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826530E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826530E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826530EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826530F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826530F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826530F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826530FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82653100: 4BE13D21  bl 0x82466e20
	ctx.lr = 0x82653104;
	sub_82466E20(ctx, base);
	// 82653104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265310C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653118 size=112
    let mut pc: u32 = 0x82653118;
    'dispatch: loop {
        match pc {
            0x82653118 => {
    //   block [0x82653118..0x82653188)
	// 82653118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265311C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653124: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653128: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265312C: 38AA5488  addi r5, r10, 0x5488
	ctx.r[5].s64 = ctx.r[10].s64 + 21640;
	// 82653130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653134: 390B8A30  addi r8, r11, -0x75d0
	ctx.r[8].s64 = ctx.r[11].s64 + -30160;
	// 82653138: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8265313C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 82653140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653144: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265314C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653150: 386A54E8  addi r3, r10, 0x54e8
	ctx.r[3].s64 = ctx.r[10].s64 + 21736;
	// 82653154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82653158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265315C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265316C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653174: 4BE13CAD  bl 0x82466e20
	ctx.lr = 0x82653178;
	sub_82466E20(ctx, base);
	// 82653178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265317C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653188 size=112
    let mut pc: u32 = 0x82653188;
    'dispatch: loop {
        match pc {
            0x82653188 => {
    //   block [0x82653188..0x826531F8)
	// 82653188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265318C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653194: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653198: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265319C: 38AA5488  addi r5, r10, 0x5488
	ctx.r[5].s64 = ctx.r[10].s64 + 21640;
	// 826531A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826531A4: 390B8B50  addi r8, r11, -0x74b0
	ctx.r[8].s64 = ctx.r[11].s64 + -29872;
	// 826531A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826531AC: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826531B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826531B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826531B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826531BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826531C0: 386A5518  addi r3, r10, 0x5518
	ctx.r[3].s64 = ctx.r[10].s64 + 21784;
	// 826531C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826531C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826531CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826531D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826531D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826531D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826531DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826531E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826531E4: 4BE13C3D  bl 0x82466e20
	ctx.lr = 0x826531E8;
	sub_82466E20(ctx, base);
	// 826531E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826531EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826531F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826531F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826531F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826531F8 size=108
    let mut pc: u32 = 0x826531F8;
    'dispatch: loop {
        match pc {
            0x826531F8 => {
    //   block [0x826531F8..0x82653264)
	// 826531F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826531FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653204: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265320C: 38EB8B68  addi r7, r11, -0x7498
	ctx.r[7].s64 = ctx.r[11].s64 + -29848;
	// 82653210: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82653214: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82653218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265321C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653220: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82653224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653228: 386A5548  addi r3, r10, 0x5548
	ctx.r[3].s64 = ctx.r[10].s64 + 21832;
	// 8265322C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82653230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265323C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265324C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82653250: 4BE13BD1  bl 0x82466e20
	ctx.lr = 0x82653254;
	sub_82466E20(ctx, base);
	// 82653254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265325C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653268 size=112
    let mut pc: u32 = 0x82653268;
    'dispatch: loop {
        match pc {
            0x82653268 => {
    //   block [0x82653268..0x826532D8)
	// 82653268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265326C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653274: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653278: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265327C: 38AA5488  addi r5, r10, 0x5488
	ctx.r[5].s64 = ctx.r[10].s64 + 21640;
	// 82653280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653284: 390B8BF8  addi r8, r11, -0x7408
	ctx.r[8].s64 = ctx.r[11].s64 + -29704;
	// 82653288: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8265328C: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 82653290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653294: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265329C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826532A0: 386A5578  addi r3, r10, 0x5578
	ctx.r[3].s64 = ctx.r[10].s64 + 21880;
	// 826532A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826532A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826532AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826532B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826532B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826532B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826532BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826532C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826532C4: 4BE13B5D  bl 0x82466e20
	ctx.lr = 0x826532C8;
	sub_82466E20(ctx, base);
	// 826532C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826532CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826532D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826532D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826532D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826532D8 size=108
    let mut pc: u32 = 0x826532D8;
    'dispatch: loop {
        match pc {
            0x826532D8 => {
    //   block [0x826532D8..0x82653344)
	// 826532D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826532DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826532E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826532E4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826532E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826532EC: 38EB8CE8  addi r7, r11, -0x7318
	ctx.r[7].s64 = ctx.r[11].s64 + -29464;
	// 826532F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826532F4: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 826532F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826532FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82653304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653308: 386A55A8  addi r3, r10, 0x55a8
	ctx.r[3].s64 = ctx.r[10].s64 + 21928;
	// 8265330C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82653310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265331C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265332C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82653330: 4BE13AF1  bl 0x82466e20
	ctx.lr = 0x82653334;
	sub_82466E20(ctx, base);
	// 82653334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265333C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653348 size=108
    let mut pc: u32 = 0x82653348;
    'dispatch: loop {
        match pc {
            0x82653348 => {
    //   block [0x82653348..0x826533B4)
	// 82653348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265334C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653354: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265335C: 38EB8D00  addi r7, r11, -0x7300
	ctx.r[7].s64 = ctx.r[11].s64 + -29440;
	// 82653360: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82653364: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 82653368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265336C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653370: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82653374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653378: 386A55D8  addi r3, r10, 0x55d8
	ctx.r[3].s64 = ctx.r[10].s64 + 21976;
	// 8265337C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82653380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265338C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265339C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826533A0: 4BE13A81  bl 0x82466e20
	ctx.lr = 0x826533A4;
	sub_82466E20(ctx, base);
	// 826533A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826533A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826533AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826533B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826533B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826533B8 size=116
    let mut pc: u32 = 0x826533B8;
    'dispatch: loop {
        match pc {
            0x826533B8 => {
    //   block [0x826533B8..0x8265342C)
	// 826533B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826533BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826533C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826533C4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826533C8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826533CC: 390B8D64  addi r8, r11, -0x729c
	ctx.r[8].s64 = ctx.r[11].s64 + -29340;
	// 826533D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826533D4: 392AC88C  addi r9, r10, -0x3774
	ctx.r[9].s64 = ctx.r[10].s64 + -14196;
	// 826533D8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826533DC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826533E0: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 826533E4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826533E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826533EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826533F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826533F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826533F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826533FC: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82653400: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 82653404: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82653408: 386B5608  addi r3, r11, 0x5608
	ctx.r[3].s64 = ctx.r[11].s64 + 22024;
	// 8265340C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82653410: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653418: 4BE13A09  bl 0x82466e20
	ctx.lr = 0x8265341C;
	sub_82466E20(ctx, base);
	// 8265341C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653430 size=108
    let mut pc: u32 = 0x82653430;
    'dispatch: loop {
        match pc {
            0x82653430 => {
    //   block [0x82653430..0x8265349C)
	// 82653430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265343C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653444: 38EB8D80  addi r7, r11, -0x7280
	ctx.r[7].s64 = ctx.r[11].s64 + -29312;
	// 82653448: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265344C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 82653450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653454: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653458: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265345C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653460: 386A5638  addi r3, r10, 0x5638
	ctx.r[3].s64 = ctx.r[10].s64 + 22072;
	// 82653464: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82653468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265346C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265347C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82653488: 4BE13999  bl 0x82466e20
	ctx.lr = 0x8265348C;
	sub_82466E20(ctx, base);
	// 8265348C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826534A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826534A0 size=108
    let mut pc: u32 = 0x826534A0;
    'dispatch: loop {
        match pc {
            0x826534A0 => {
    //   block [0x826534A0..0x8265350C)
	// 826534A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826534A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826534A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826534AC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826534B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826534B4: 38EB8DC8  addi r7, r11, -0x7238
	ctx.r[7].s64 = ctx.r[11].s64 + -29240;
	// 826534B8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826534BC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826534C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826534C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826534C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826534CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826534D0: 386A5668  addi r3, r10, 0x5668
	ctx.r[3].s64 = ctx.r[10].s64 + 22120;
	// 826534D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826534D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826534DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826534E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826534E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826534E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826534EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826534F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826534F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826534F8: 4BE13929  bl 0x82466e20
	ctx.lr = 0x826534FC;
	sub_82466E20(ctx, base);
	// 826534FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653510 size=108
    let mut pc: u32 = 0x82653510;
    'dispatch: loop {
        match pc {
            0x82653510 => {
    //   block [0x82653510..0x8265357C)
	// 82653510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265351C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653524: 38EB8E58  addi r7, r11, -0x71a8
	ctx.r[7].s64 = ctx.r[11].s64 + -29096;
	// 82653528: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8265352C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82653530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653534: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265353C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653540: 386A5698  addi r3, r10, 0x5698
	ctx.r[3].s64 = ctx.r[10].s64 + 22168;
	// 82653544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82653548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265354C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265355C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82653568: 4BE138B9  bl 0x82466e20
	ctx.lr = 0x8265356C;
	sub_82466E20(ctx, base);
	// 8265356C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653580 size=100
    let mut pc: u32 = 0x82653580;
    'dispatch: loop {
        match pc {
            0x82653580 => {
    //   block [0x82653580..0x826535E4)
	// 82653580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265358C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653594: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82653598: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265359C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826535A0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826535A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826535A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826535AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826535B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826535B4: 386A56C8  addi r3, r10, 0x56c8
	ctx.r[3].s64 = ctx.r[10].s64 + 22216;
	// 826535B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826535BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826535C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826535C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826535C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826535CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826535D0: 4BE13851  bl 0x82466e20
	ctx.lr = 0x826535D4;
	sub_82466E20(ctx, base);
	// 826535D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826535D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826535DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826535E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826535E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826535E8 size=112
    let mut pc: u32 = 0x826535E8;
    'dispatch: loop {
        match pc {
            0x826535E8 => {
    //   block [0x826535E8..0x82653658)
	// 826535E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826535EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826535F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826535F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826535F8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826535FC: 38AA56C8  addi r5, r10, 0x56c8
	ctx.r[5].s64 = ctx.r[10].s64 + 22216;
	// 82653600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653604: 390B8EE8  addi r8, r11, -0x7118
	ctx.r[8].s64 = ctx.r[11].s64 + -28952;
	// 82653608: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265360C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82653610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653614: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265361C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653620: 386A56F8  addi r3, r10, 0x56f8
	ctx.r[3].s64 = ctx.r[10].s64 + 22264;
	// 82653624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82653628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265362C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265363C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653644: 4BE137DD  bl 0x82466e20
	ctx.lr = 0x82653648;
	sub_82466E20(ctx, base);
	// 82653648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265364C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653658 size=108
    let mut pc: u32 = 0x82653658;
    'dispatch: loop {
        match pc {
            0x82653658 => {
    //   block [0x82653658..0x826536C4)
	// 82653658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265365C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653664: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265366C: 38EB8F48  addi r7, r11, -0x70b8
	ctx.r[7].s64 = ctx.r[11].s64 + -28856;
	// 82653670: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82653674: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 82653678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265367C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82653684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653688: 386A5728  addi r3, r10, 0x5728
	ctx.r[3].s64 = ctx.r[10].s64 + 22312;
	// 8265368C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82653690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265369C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826536A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826536A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826536A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826536AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826536B0: 4BE13771  bl 0x82466e20
	ctx.lr = 0x826536B4;
	sub_82466E20(ctx, base);
	// 826536B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826536B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826536BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826536C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826536C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826536C8 size=108
    let mut pc: u32 = 0x826536C8;
    'dispatch: loop {
        match pc {
            0x826536C8 => {
    //   block [0x826536C8..0x82653734)
	// 826536C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826536CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826536D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826536D4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826536D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826536DC: 38EB8F78  addi r7, r11, -0x7088
	ctx.r[7].s64 = ctx.r[11].s64 + -28808;
	// 826536E0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826536E4: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 826536E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826536EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826536F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826536F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826536F8: 386A5758  addi r3, r10, 0x5758
	ctx.r[3].s64 = ctx.r[10].s64 + 22360;
	// 826536FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82653700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265370C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265371C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82653720: 4BE13701  bl 0x82466e20
	ctx.lr = 0x82653724;
	sub_82466E20(ctx, base);
	// 82653724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265372C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653738 size=108
    let mut pc: u32 = 0x82653738;
    'dispatch: loop {
        match pc {
            0x82653738 => {
    //   block [0x82653738..0x826537A4)
	// 82653738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265373C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653744: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265374C: 38EB8FD8  addi r7, r11, -0x7028
	ctx.r[7].s64 = ctx.r[11].s64 + -28712;
	// 82653750: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82653754: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 82653758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265375C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653760: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82653764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653768: 386A5788  addi r3, r10, 0x5788
	ctx.r[3].s64 = ctx.r[10].s64 + 22408;
	// 8265376C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82653770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265377C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265378C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82653790: 4BE13691  bl 0x82466e20
	ctx.lr = 0x82653794;
	sub_82466E20(ctx, base);
	// 82653794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265379C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826537A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826537A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826537A8 size=24
    let mut pc: u32 = 0x826537A8;
    'dispatch: loop {
        match pc {
            0x826537A8 => {
    //   block [0x826537A8..0x826537C0)
	// 826537A8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826537AC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826537B0: 394A0FA8  addi r10, r10, 0xfa8
	ctx.r[10].s64 = ctx.r[10].s64 + 4008;
	// 826537B4: 816B8D7C  lwz r11, -0x7284(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29316 as u32) ) } as u64;
	// 826537B8: 916A0128  stw r11, 0x128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 826537BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826537C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826537C0 size=116
    let mut pc: u32 = 0x826537C0;
    'dispatch: loop {
        match pc {
            0x826537C0 => {
    //   block [0x826537C0..0x82653834)
	// 826537C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826537C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826537C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826537CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826537D0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826537D4: 390B0FA8  addi r8, r11, 0xfa8
	ctx.r[8].s64 = ctx.r[11].s64 + 4008;
	// 826537D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826537DC: 392AC8C0  addi r9, r10, -0x3740
	ctx.r[9].s64 = ctx.r[10].s64 + -14144;
	// 826537E0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826537E4: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826537E8: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 826537EC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826537F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826537F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826537F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826537FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653804: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82653808: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 8265380C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82653810: 386B57B8  addi r3, r11, 0x57b8
	ctx.r[3].s64 = ctx.r[11].s64 + 22456;
	// 82653814: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82653818: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265381C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653820: 4BE13601  bl 0x82466e20
	ctx.lr = 0x82653824;
	sub_82466E20(ctx, base);
	// 82653824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265382C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653838 size=112
    let mut pc: u32 = 0x82653838;
    'dispatch: loop {
        match pc {
            0x82653838 => {
    //   block [0x82653838..0x826538A8)
	// 82653838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265383C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653844: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653848: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265384C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82653850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653854: 390B9038  addi r8, r11, -0x6fc8
	ctx.r[8].s64 = ctx.r[11].s64 + -28616;
	// 82653858: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265385C: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 82653860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265386C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653870: 386A57E8  addi r3, r10, 0x57e8
	ctx.r[3].s64 = ctx.r[10].s64 + 22504;
	// 82653874: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82653878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265387C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265388C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653894: 4BE1358D  bl 0x82466e20
	ctx.lr = 0x82653898;
	sub_82466E20(ctx, base);
	// 82653898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265389C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826538A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826538A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826538A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826538A8 size=112
    let mut pc: u32 = 0x826538A8;
    'dispatch: loop {
        match pc {
            0x826538A8 => {
    //   block [0x826538A8..0x82653918)
	// 826538A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826538AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826538B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826538B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826538B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826538BC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 826538C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826538C4: 390B9080  addi r8, r11, -0x6f80
	ctx.r[8].s64 = ctx.r[11].s64 + -28544;
	// 826538C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826538CC: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 826538D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826538D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826538D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826538DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826538E0: 386A5818  addi r3, r10, 0x5818
	ctx.r[3].s64 = ctx.r[10].s64 + 22552;
	// 826538E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826538E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826538EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826538F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826538F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826538F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826538FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653904: 4BE1351D  bl 0x82466e20
	ctx.lr = 0x82653908;
	sub_82466E20(ctx, base);
	// 82653908: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265390C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653918 size=112
    let mut pc: u32 = 0x82653918;
    'dispatch: loop {
        match pc {
            0x82653918 => {
    //   block [0x82653918..0x82653988)
	// 82653918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265391C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653924: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653928: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265392C: 38AA59F8  addi r5, r10, 0x59f8
	ctx.r[5].s64 = ctx.r[10].s64 + 23032;
	// 82653930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653934: 390B90C8  addi r8, r11, -0x6f38
	ctx.r[8].s64 = ctx.r[11].s64 + -28472;
	// 82653938: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8265393C: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 82653940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653944: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265394C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653950: 386A5848  addi r3, r10, 0x5848
	ctx.r[3].s64 = ctx.r[10].s64 + 22600;
	// 82653954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82653958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265395C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265396C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653974: 4BE134AD  bl 0x82466e20
	ctx.lr = 0x82653978;
	sub_82466E20(ctx, base);
	// 82653978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265397C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653988 size=108
    let mut pc: u32 = 0x82653988;
    'dispatch: loop {
        match pc {
            0x82653988 => {
    //   block [0x82653988..0x826539F4)
	// 82653988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265398C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653994: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265399C: 38EB91B8  addi r7, r11, -0x6e48
	ctx.r[7].s64 = ctx.r[11].s64 + -28232;
	// 826539A0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826539A4: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 826539A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826539AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826539B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826539B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826539B8: 386A5878  addi r3, r10, 0x5878
	ctx.r[3].s64 = ctx.r[10].s64 + 22648;
	// 826539BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826539C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826539C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826539C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826539CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826539D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826539D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826539D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826539DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826539E0: 4BE13441  bl 0x82466e20
	ctx.lr = 0x826539E4;
	sub_82466E20(ctx, base);
	// 826539E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826539E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826539EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826539F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826539F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826539F8 size=108
    let mut pc: u32 = 0x826539F8;
    'dispatch: loop {
        match pc {
            0x826539F8 => {
    //   block [0x826539F8..0x82653A64)
	// 826539F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826539FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653A04: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653A0C: 38EB9218  addi r7, r11, -0x6de8
	ctx.r[7].s64 = ctx.r[11].s64 + -28136;
	// 82653A10: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82653A14: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 82653A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653A1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653A20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82653A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653A28: 386A58A8  addi r3, r10, 0x58a8
	ctx.r[3].s64 = ctx.r[10].s64 + 22696;
	// 82653A2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82653A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82653A50: 4BE133D1  bl 0x82466e20
	ctx.lr = 0x82653A54;
	sub_82466E20(ctx, base);
	// 82653A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653A68 size=112
    let mut pc: u32 = 0x82653A68;
    'dispatch: loop {
        match pc {
            0x82653A68 => {
    //   block [0x82653A68..0x82653AD8)
	// 82653A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653A74: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82653A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653A7C: 392BC8F4  addi r9, r11, -0x370c
	ctx.r[9].s64 = ctx.r[11].s64 + -14092;
	// 82653A80: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 82653A84: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82653A88: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653A8C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 82653A90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653A94: 396B92C8  addi r11, r11, -0x6d38
	ctx.r[11].s64 = ctx.r[11].s64 + -27960;
	// 82653A98: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82653A9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653AA0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82653AA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653AA8: 386A58D8  addi r3, r10, 0x58d8
	ctx.r[3].s64 = ctx.r[10].s64 + 22744;
	// 82653AAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82653AB0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82653AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653AB8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82653ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82653AC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82653AC4: 4BE1335D  bl 0x82466e20
	ctx.lr = 0x82653AC8;
	sub_82466E20(ctx, base);
	// 82653AC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653AD8 size=112
    let mut pc: u32 = 0x82653AD8;
    'dispatch: loop {
        match pc {
            0x82653AD8 => {
    //   block [0x82653AD8..0x82653B48)
	// 82653AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653AE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653AE8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653AEC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82653AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653AF4: 390B9418  addi r8, r11, -0x6be8
	ctx.r[8].s64 = ctx.r[11].s64 + -27624;
	// 82653AF8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82653AFC: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 82653B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653B04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653B08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82653B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653B10: 386A5908  addi r3, r10, 0x5908
	ctx.r[3].s64 = ctx.r[10].s64 + 22792;
	// 82653B14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82653B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653B34: 4BE132ED  bl 0x82466e20
	ctx.lr = 0x82653B38;
	sub_82466E20(ctx, base);
	// 82653B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653B48 size=112
    let mut pc: u32 = 0x82653B48;
    'dispatch: loop {
        match pc {
            0x82653B48 => {
    //   block [0x82653B48..0x82653BB8)
	// 82653B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653B54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653B58: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653B5C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82653B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653B64: 390B94C0  addi r8, r11, -0x6b40
	ctx.r[8].s64 = ctx.r[11].s64 + -27456;
	// 82653B68: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82653B6C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 82653B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653B74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82653B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653B80: 386A5938  addi r3, r10, 0x5938
	ctx.r[3].s64 = ctx.r[10].s64 + 22840;
	// 82653B84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82653B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653BA4: 4BE1327D  bl 0x82466e20
	ctx.lr = 0x82653BA8;
	sub_82466E20(ctx, base);
	// 82653BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653BB8 size=112
    let mut pc: u32 = 0x82653BB8;
    'dispatch: loop {
        match pc {
            0x82653BB8 => {
    //   block [0x82653BB8..0x82653C28)
	// 82653BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653BC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653BC8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653BCC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82653BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653BD4: 390B9550  addi r8, r11, -0x6ab0
	ctx.r[8].s64 = ctx.r[11].s64 + -27312;
	// 82653BD8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82653BDC: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 82653BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653BE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653BE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82653BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653BF0: 386A5968  addi r3, r10, 0x5968
	ctx.r[3].s64 = ctx.r[10].s64 + 22888;
	// 82653BF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82653BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653C14: 4BE1320D  bl 0x82466e20
	ctx.lr = 0x82653C18;
	sub_82466E20(ctx, base);
	// 82653C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653C28 size=108
    let mut pc: u32 = 0x82653C28;
    'dispatch: loop {
        match pc {
            0x82653C28 => {
    //   block [0x82653C28..0x82653C94)
	// 82653C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653C34: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653C38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653C3C: 38EB95C8  addi r7, r11, -0x6a38
	ctx.r[7].s64 = ctx.r[11].s64 + -27192;
	// 82653C40: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82653C44: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 82653C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653C4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653C50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82653C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653C58: 386A5998  addi r3, r10, 0x5998
	ctx.r[3].s64 = ctx.r[10].s64 + 22936;
	// 82653C5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82653C60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653C68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653C70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653C78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653C7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82653C80: 4BE131A1  bl 0x82466e20
	ctx.lr = 0x82653C84;
	sub_82466E20(ctx, base);
	// 82653C84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653C98 size=112
    let mut pc: u32 = 0x82653C98;
    'dispatch: loop {
        match pc {
            0x82653C98 => {
    //   block [0x82653C98..0x82653D08)
	// 82653C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653CA4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82653CA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653CAC: 392AC950  addi r9, r10, -0x36b0
	ctx.r[9].s64 = ctx.r[10].s64 + -14000;
	// 82653CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653CB4: 390B9670  addi r8, r11, -0x6990
	ctx.r[8].s64 = ctx.r[11].s64 + -27024;
	// 82653CB8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82653CBC: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 82653CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653CC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82653CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653CD0: 386A59C8  addi r3, r10, 0x59c8
	ctx.r[3].s64 = ctx.r[10].s64 + 22984;
	// 82653CD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82653CD8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82653CDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653CE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82653CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653CF4: 4BE1312D  bl 0x82466e20
	ctx.lr = 0x82653CF8;
	sub_82466E20(ctx, base);
	// 82653CF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653D08 size=100
    let mut pc: u32 = 0x82653D08;
    'dispatch: loop {
        match pc {
            0x82653D08 => {
    //   block [0x82653D08..0x82653D6C)
	// 82653D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653D14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653D1C: 38AA61A8  addi r5, r10, 0x61a8
	ctx.r[5].s64 = ctx.r[10].s64 + 25000;
	// 82653D20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653D28: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 82653D2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653D3C: 386A59F8  addi r3, r10, 0x59f8
	ctx.r[3].s64 = ctx.r[10].s64 + 23032;
	// 82653D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653D44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653D48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82653D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653D50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82653D54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653D58: 4BE130C9  bl 0x82466e20
	ctx.lr = 0x82653D5C;
	sub_82466E20(ctx, base);
	// 82653D5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653D70 size=108
    let mut pc: u32 = 0x82653D70;
    'dispatch: loop {
        match pc {
            0x82653D70 => {
    //   block [0x82653D70..0x82653DDC)
	// 82653D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653D7C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653D84: 38EB96A4  addi r7, r11, -0x695c
	ctx.r[7].s64 = ctx.r[11].s64 + -26972;
	// 82653D88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82653D8C: 388A2E80  addi r4, r10, 0x2e80
	ctx.r[4].s64 = ctx.r[10].s64 + 11904;
	// 82653D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653D94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653D98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82653D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653DA0: 386A5A28  addi r3, r10, 0x5a28
	ctx.r[3].s64 = ctx.r[10].s64 + 23080;
	// 82653DA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82653DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653DC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82653DC8: 4BE13059  bl 0x82466e20
	ctx.lr = 0x82653DCC;
	sub_82466E20(ctx, base);
	// 82653DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653DE0 size=112
    let mut pc: u32 = 0x82653DE0;
    'dispatch: loop {
        match pc {
            0x82653DE0 => {
    //   block [0x82653DE0..0x82653E50)
	// 82653DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653DEC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82653DF0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653DF4: 392AC9B0  addi r9, r10, -0x3650
	ctx.r[9].s64 = ctx.r[10].s64 + -13904;
	// 82653DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653DFC: 390B96D8  addi r8, r11, -0x6928
	ctx.r[8].s64 = ctx.r[11].s64 + -26920;
	// 82653E00: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82653E04: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 82653E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653E0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653E10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82653E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653E18: 386A5A58  addi r3, r10, 0x5a58
	ctx.r[3].s64 = ctx.r[10].s64 + 23128;
	// 82653E1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82653E20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82653E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653E2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653E34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82653E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653E3C: 4BE12FE5  bl 0x82466e20
	ctx.lr = 0x82653E40;
	sub_82466E20(ctx, base);
	// 82653E40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653E50 size=112
    let mut pc: u32 = 0x82653E50;
    'dispatch: loop {
        match pc {
            0x82653E50 => {
    //   block [0x82653E50..0x82653EC0)
	// 82653E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653E5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653E60: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653E64: 38AA59F8  addi r5, r10, 0x59f8
	ctx.r[5].s64 = ctx.r[10].s64 + 23032;
	// 82653E68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653E6C: 390B9750  addi r8, r11, -0x68b0
	ctx.r[8].s64 = ctx.r[11].s64 + -26800;
	// 82653E70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82653E74: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 82653E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653E7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653E80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82653E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653E88: 386A5A88  addi r3, r10, 0x5a88
	ctx.r[3].s64 = ctx.r[10].s64 + 23176;
	// 82653E8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82653E90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653E94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653E98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653EA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653EAC: 4BE12F75  bl 0x82466e20
	ctx.lr = 0x82653EB0;
	sub_82466E20(ctx, base);
	// 82653EB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653EC0 size=116
    let mut pc: u32 = 0x82653EC0;
    'dispatch: loop {
        match pc {
            0x82653EC0 => {
    //   block [0x82653EC0..0x82653F34)
	// 82653EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653ECC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82653ED0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82653ED4: 390A9780  addi r8, r10, -0x6880
	ctx.r[8].s64 = ctx.r[10].s64 + -26752;
	// 82653ED8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653EDC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82653EE0: 38AA59F8  addi r5, r10, 0x59f8
	ctx.r[5].s64 = ctx.r[10].s64 + 23032;
	// 82653EE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653EE8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82653EEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653EF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82653EF4: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 82653EF8: 396BC9C4  addi r11, r11, -0x363c
	ctx.r[11].s64 = ctx.r[11].s64 + -13884;
	// 82653EFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653F00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653F04: 386A5AB8  addi r3, r10, 0x5ab8
	ctx.r[3].s64 = ctx.r[10].s64 + 23224;
	// 82653F08: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82653F0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653F10: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82653F14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653F20: 4BE12F01  bl 0x82466e20
	ctx.lr = 0x82653F24;
	sub_82466E20(ctx, base);
	// 82653F24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653F38 size=100
    let mut pc: u32 = 0x82653F38;
    'dispatch: loop {
        match pc {
            0x82653F38 => {
    //   block [0x82653F38..0x82653F9C)
	// 82653F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653F44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82653F4C: 38AA5AB8  addi r5, r10, 0x5ab8
	ctx.r[5].s64 = ctx.r[10].s64 + 23224;
	// 82653F50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82653F58: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 82653F5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82653F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82653F6C: 386A5AE8  addi r3, r10, 0x5ae8
	ctx.r[3].s64 = ctx.r[10].s64 + 23272;
	// 82653F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82653F74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82653F78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82653F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82653F80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82653F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82653F88: 4BE12E99  bl 0x82466e20
	ctx.lr = 0x82653F8C;
	sub_82466E20(ctx, base);
	// 82653F8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82653F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82653F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82653F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82653FA0 size=24
    let mut pc: u32 = 0x82653FA0;
    'dispatch: loop {
        match pc {
            0x82653FA0 => {
    //   block [0x82653FA0..0x82653FB8)
	// 82653FA0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653FA4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82653FA8: 394A10E0  addi r10, r10, 0x10e0
	ctx.r[10].s64 = ctx.r[10].s64 + 4320;
	// 82653FAC: 816B96D4  lwz r11, -0x692c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26924 as u32) ) } as u64;
	// 82653FB0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82653FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82653FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82653FB8 size=116
    let mut pc: u32 = 0x82653FB8;
    'dispatch: loop {
        match pc {
            0x82653FB8 => {
    //   block [0x82653FB8..0x8265402C)
	// 82653FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82653FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82653FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82653FC4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82653FC8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653FCC: 392BCA00  addi r9, r11, -0x3600
	ctx.r[9].s64 = ctx.r[11].s64 + -13824;
	// 82653FD0: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82653FD4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82653FD8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82653FDC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82653FE0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82653FE4: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 82653FE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82653FEC: 396B10E0  addi r11, r11, 0x10e0
	ctx.r[11].s64 = ctx.r[11].s64 + 4320;
	// 82653FF0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82653FF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82653FF8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82653FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654000: 386A5B18  addi r3, r10, 0x5b18
	ctx.r[3].s64 = ctx.r[10].s64 + 23320;
	// 82654004: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82654008: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8265400C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654010: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82654014: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82654018: 4BE12E09  bl 0x82466e20
	ctx.lr = 0x8265401C;
	sub_82466E20(ctx, base);
	// 8265401C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654030 size=116
    let mut pc: u32 = 0x82654030;
    'dispatch: loop {
        match pc {
            0x82654030 => {
    //   block [0x82654030..0x826540A4)
	// 82654030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265403C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82654040: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654044: 392BCA54  addi r9, r11, -0x35ac
	ctx.r[9].s64 = ctx.r[11].s64 + -13740;
	// 82654048: 38AA59F8  addi r5, r10, 0x59f8
	ctx.r[5].s64 = ctx.r[10].s64 + 23032;
	// 8265404C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654050: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82654054: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82654058: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265405C: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 82654060: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654064: 396B9830  addi r11, r11, -0x67d0
	ctx.r[11].s64 = ctx.r[11].s64 + -26576;
	// 82654068: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8265406C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654070: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82654074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654078: 386A5B48  addi r3, r10, 0x5b48
	ctx.r[3].s64 = ctx.r[10].s64 + 23368;
	// 8265407C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82654080: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82654084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654088: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8265408C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82654090: 4BE12D91  bl 0x82466e20
	ctx.lr = 0x82654094;
	sub_82466E20(ctx, base);
	// 82654094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265409C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826540A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826540A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826540A8 size=108
    let mut pc: u32 = 0x826540A8;
    'dispatch: loop {
        match pc {
            0x826540A8 => {
    //   block [0x826540A8..0x82654114)
	// 826540A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826540AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826540B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826540B4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826540B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826540BC: 38EB9908  addi r7, r11, -0x66f8
	ctx.r[7].s64 = ctx.r[11].s64 + -26360;
	// 826540C0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826540C4: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826540C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826540CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826540D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826540D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826540D8: 386A5B78  addi r3, r10, 0x5b78
	ctx.r[3].s64 = ctx.r[10].s64 + 23416;
	// 826540DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826540E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826540E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826540E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826540EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826540F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826540F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826540F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826540FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82654100: 4BE12D21  bl 0x82466e20
	ctx.lr = 0x82654104;
	sub_82466E20(ctx, base);
	// 82654104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265410C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82654118 size=24
    let mut pc: u32 = 0x82654118;
    'dispatch: loop {
        match pc {
            0x82654118 => {
    //   block [0x82654118..0x82654130)
	// 82654118: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265411C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82654120: 394A1188  addi r10, r10, 0x1188
	ctx.r[10].s64 = ctx.r[10].s64 + 4488;
	// 82654124: 816B982C  lwz r11, -0x67d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26580 as u32) ) } as u64;
	// 82654128: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265412C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654130 size=116
    let mut pc: u32 = 0x82654130;
    'dispatch: loop {
        match pc {
            0x82654130 => {
    //   block [0x82654130..0x826541A4)
	// 82654130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265413C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654140: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82654144: 390B1188  addi r8, r11, 0x1188
	ctx.r[8].s64 = ctx.r[11].s64 + 4488;
	// 82654148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265414C: 392ACAC8  addi r9, r10, -0x3538
	ctx.r[9].s64 = ctx.r[10].s64 + -13624;
	// 82654150: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654154: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 82654158: 38AA59F8  addi r5, r10, 0x59f8
	ctx.r[5].s64 = ctx.r[10].s64 + 23032;
	// 8265415C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654164: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265416C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654174: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82654178: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 8265417C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82654180: 386B5BA8  addi r3, r11, 0x5ba8
	ctx.r[3].s64 = ctx.r[11].s64 + 23464;
	// 82654184: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82654188: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265418C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654190: 4BE12C91  bl 0x82466e20
	ctx.lr = 0x82654194;
	sub_82466E20(ctx, base);
	// 82654194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265419C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826541A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826541A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826541A8 size=112
    let mut pc: u32 = 0x826541A8;
    'dispatch: loop {
        match pc {
            0x826541A8 => {
    //   block [0x826541A8..0x82654218)
	// 826541A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826541AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826541B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826541B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826541B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826541BC: 38AA59F8  addi r5, r10, 0x59f8
	ctx.r[5].s64 = ctx.r[10].s64 + 23032;
	// 826541C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826541C4: 390B996C  addi r8, r11, -0x6694
	ctx.r[8].s64 = ctx.r[11].s64 + -26260;
	// 826541C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826541CC: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 826541D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826541D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826541D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826541DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826541E0: 386A5BD8  addi r3, r10, 0x5bd8
	ctx.r[3].s64 = ctx.r[10].s64 + 23512;
	// 826541E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826541E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826541EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826541F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826541F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826541F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826541FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654204: 4BE12C1D  bl 0x82466e20
	ctx.lr = 0x82654208;
	sub_82466E20(ctx, base);
	// 82654208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265420C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82654218 size=24
    let mut pc: u32 = 0x82654218;
    'dispatch: loop {
        match pc {
            0x82654218 => {
    //   block [0x82654218..0x82654230)
	// 82654218: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265421C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82654220: 394A1320  addi r10, r10, 0x1320
	ctx.r[10].s64 = ctx.r[10].s64 + 4896;
	// 82654224: 816B999C  lwz r11, -0x6664(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26212 as u32) ) } as u64;
	// 82654228: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8265422C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654230 size=116
    let mut pc: u32 = 0x82654230;
    'dispatch: loop {
        match pc {
            0x82654230 => {
    //   block [0x82654230..0x826542A4)
	// 82654230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265423C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654240: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82654244: 390B1320  addi r8, r11, 0x1320
	ctx.r[8].s64 = ctx.r[11].s64 + 4896;
	// 82654248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265424C: 392ACB00  addi r9, r10, -0x3500
	ctx.r[9].s64 = ctx.r[10].s64 + -13568;
	// 82654250: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654254: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 82654258: 38AA5B48  addi r5, r10, 0x5b48
	ctx.r[5].s64 = ctx.r[10].s64 + 23368;
	// 8265425C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654264: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265426C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654274: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82654278: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 8265427C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82654280: 386B5C08  addi r3, r11, 0x5c08
	ctx.r[3].s64 = ctx.r[11].s64 + 23560;
	// 82654284: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82654288: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265428C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654290: 4BE12B91  bl 0x82466e20
	ctx.lr = 0x82654294;
	sub_82466E20(ctx, base);
	// 82654294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265429C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826542A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826542A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826542A8 size=112
    let mut pc: u32 = 0x826542A8;
    'dispatch: loop {
        match pc {
            0x826542A8 => {
    //   block [0x826542A8..0x82654318)
	// 826542A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826542AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826542B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826542B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826542B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826542BC: 38AA59F8  addi r5, r10, 0x59f8
	ctx.r[5].s64 = ctx.r[10].s64 + 23032;
	// 826542C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826542C4: 390B99A0  addi r8, r11, -0x6660
	ctx.r[8].s64 = ctx.r[11].s64 + -26208;
	// 826542C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826542CC: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826542D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826542D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826542D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826542DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826542E0: 386A5C38  addi r3, r10, 0x5c38
	ctx.r[3].s64 = ctx.r[10].s64 + 23608;
	// 826542E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826542E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826542EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826542F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826542F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826542F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826542FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654304: 4BE12B1D  bl 0x82466e20
	ctx.lr = 0x82654308;
	sub_82466E20(ctx, base);
	// 82654308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265430C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654318 size=100
    let mut pc: u32 = 0x82654318;
    'dispatch: loop {
        match pc {
            0x82654318 => {
    //   block [0x82654318..0x8265437C)
	// 82654318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265431C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654324: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265432C: 38AA61A8  addi r5, r10, 0x61a8
	ctx.r[5].s64 = ctx.r[10].s64 + 25000;
	// 82654330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654338: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 8265433C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265434C: 386A5C68  addi r3, r10, 0x5c68
	ctx.r[3].s64 = ctx.r[10].s64 + 23656;
	// 82654350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654354: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654358: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265435C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654360: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82654364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654368: 4BE12AB9  bl 0x82466e20
	ctx.lr = 0x8265436C;
	sub_82466E20(ctx, base);
	// 8265436C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654380 size=108
    let mut pc: u32 = 0x82654380;
    'dispatch: loop {
        match pc {
            0x82654380 => {
    //   block [0x82654380..0x826543EC)
	// 82654380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265438C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654394: 38EB99B8  addi r7, r11, -0x6648
	ctx.r[7].s64 = ctx.r[11].s64 + -26184;
	// 82654398: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8265439C: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 826543A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826543A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826543A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826543AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826543B0: 386A5C98  addi r3, r10, 0x5c98
	ctx.r[3].s64 = ctx.r[10].s64 + 23704;
	// 826543B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826543B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826543BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826543C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826543C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826543C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826543CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826543D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826543D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826543D8: 4BE12A49  bl 0x82466e20
	ctx.lr = 0x826543DC;
	sub_82466E20(ctx, base);
	// 826543DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826543E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826543E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826543E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826543F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826543F0 size=112
    let mut pc: u32 = 0x826543F0;
    'dispatch: loop {
        match pc {
            0x826543F0 => {
    //   block [0x826543F0..0x82654460)
	// 826543F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826543F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826543F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826543FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654400: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654404: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 82654408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265440C: 390B9A90  addi r8, r11, -0x6570
	ctx.r[8].s64 = ctx.r[11].s64 + -25968;
	// 82654410: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82654414: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 82654418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265441C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654420: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654428: 386A5CC8  addi r3, r10, 0x5cc8
	ctx.r[3].s64 = ctx.r[10].s64 + 23752;
	// 8265442C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82654430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265443C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82654444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265444C: 4BE129D5  bl 0x82466e20
	ctx.lr = 0x82654450;
	sub_82466E20(ctx, base);
	// 82654450: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265445C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654460 size=108
    let mut pc: u32 = 0x82654460;
    'dispatch: loop {
        match pc {
            0x82654460 => {
    //   block [0x82654460..0x826544CC)
	// 82654460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265446C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654474: 38EB9AC0  addi r7, r11, -0x6540
	ctx.r[7].s64 = ctx.r[11].s64 + -25920;
	// 82654478: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265447C: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 82654480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82654484: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654488: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265448C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654490: 386A5CF8  addi r3, r10, 0x5cf8
	ctx.r[3].s64 = ctx.r[10].s64 + 23800;
	// 82654494: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82654498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265449C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826544A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826544A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826544A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826544AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826544B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826544B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826544B8: 4BE12969  bl 0x82466e20
	ctx.lr = 0x826544BC;
	sub_82466E20(ctx, base);
	// 826544BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826544C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826544C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826544C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826544D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826544D0 size=112
    let mut pc: u32 = 0x826544D0;
    'dispatch: loop {
        match pc {
            0x826544D0 => {
    //   block [0x826544D0..0x82654540)
	// 826544D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826544D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826544D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826544DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826544E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826544E4: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 826544E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826544EC: 390B9AF0  addi r8, r11, -0x6510
	ctx.r[8].s64 = ctx.r[11].s64 + -25872;
	// 826544F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826544F4: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826544F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826544FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654508: 386A5D28  addi r3, r10, 0x5d28
	ctx.r[3].s64 = ctx.r[10].s64 + 23848;
	// 8265450C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82654510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265451C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82654524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265452C: 4BE128F5  bl 0x82466e20
	ctx.lr = 0x82654530;
	sub_82466E20(ctx, base);
	// 82654530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265453C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654540 size=112
    let mut pc: u32 = 0x82654540;
    'dispatch: loop {
        match pc {
            0x82654540 => {
    //   block [0x82654540..0x826545B0)
	// 82654540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265454C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82654550: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82654554: 38EA9B08  addi r7, r10, -0x64f8
	ctx.r[7].s64 = ctx.r[10].s64 + -25848;
	// 82654558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265455C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82654560: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 82654564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654568: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265456C: 396BCB14  addi r11, r11, -0x34ec
	ctx.r[11].s64 = ctx.r[11].s64 + -13548;
	// 82654570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82654574: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654578: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265457C: 386A5D58  addi r3, r10, 0x5d58
	ctx.r[3].s64 = ctx.r[10].s64 + 23896;
	// 82654580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654584: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82654588: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265458C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82654590: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654594: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654598: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265459C: 4BE12885  bl 0x82466e20
	ctx.lr = 0x826545A0;
	sub_82466E20(ctx, base);
	// 826545A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826545A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826545A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826545AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826545B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826545B0 size=108
    let mut pc: u32 = 0x826545B0;
    'dispatch: loop {
        match pc {
            0x826545B0 => {
    //   block [0x826545B0..0x8265461C)
	// 826545B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826545B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826545B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826545BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826545C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826545C4: 38EB9BE0  addi r7, r11, -0x6420
	ctx.r[7].s64 = ctx.r[11].s64 + -25632;
	// 826545C8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826545CC: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826545D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826545D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826545D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826545DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826545E0: 386A5D88  addi r3, r10, 0x5d88
	ctx.r[3].s64 = ctx.r[10].s64 + 23944;
	// 826545E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826545E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826545EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826545F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826545F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826545F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826545FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82654608: 4BE12819  bl 0x82466e20
	ctx.lr = 0x8265460C;
	sub_82466E20(ctx, base);
	// 8265460C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654620 size=108
    let mut pc: u32 = 0x82654620;
    'dispatch: loop {
        match pc {
            0x82654620 => {
    //   block [0x82654620..0x8265468C)
	// 82654620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265462C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654634: 38EB9BF8  addi r7, r11, -0x6408
	ctx.r[7].s64 = ctx.r[11].s64 + -25608;
	// 82654638: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8265463C: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 82654640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82654644: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265464C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654650: 386A5DB8  addi r3, r10, 0x5db8
	ctx.r[3].s64 = ctx.r[10].s64 + 23992;
	// 82654654: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82654658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265465C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265466C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654674: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82654678: 4BE127A9  bl 0x82466e20
	ctx.lr = 0x8265467C;
	sub_82466E20(ctx, base);
	// 8265467C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654690 size=108
    let mut pc: u32 = 0x82654690;
    'dispatch: loop {
        match pc {
            0x82654690 => {
    //   block [0x82654690..0x826546FC)
	// 82654690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265469C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826546A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826546A4: 38EB9D00  addi r7, r11, -0x6300
	ctx.r[7].s64 = ctx.r[11].s64 + -25344;
	// 826546A8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826546AC: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 826546B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826546B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826546B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826546BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826546C0: 386A5DE8  addi r3, r10, 0x5de8
	ctx.r[3].s64 = ctx.r[10].s64 + 24040;
	// 826546C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826546C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826546CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826546D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826546D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826546D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826546DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826546E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826546E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826546E8: 4BE12739  bl 0x82466e20
	ctx.lr = 0x826546EC;
	sub_82466E20(ctx, base);
	// 826546EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826546F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826546F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826546F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654700 size=112
    let mut pc: u32 = 0x82654700;
    'dispatch: loop {
        match pc {
            0x82654700 => {
    //   block [0x82654700..0x82654770)
	// 82654700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265470C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654710: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654714: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 82654718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265471C: 390B9D60  addi r8, r11, -0x62a0
	ctx.r[8].s64 = ctx.r[11].s64 + -25248;
	// 82654720: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82654724: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 82654728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265472C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654730: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654738: 386A5E18  addi r3, r10, 0x5e18
	ctx.r[3].s64 = ctx.r[10].s64 + 24088;
	// 8265473C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82654740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265474C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82654754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265475C: 4BE126C5  bl 0x82466e20
	ctx.lr = 0x82654760;
	sub_82466E20(ctx, base);
	// 82654760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265476C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654770 size=112
    let mut pc: u32 = 0x82654770;
    'dispatch: loop {
        match pc {
            0x82654770 => {
    //   block [0x82654770..0x826547E0)
	// 82654770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265477C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654780: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654784: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 82654788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265478C: 390B9E80  addi r8, r11, -0x6180
	ctx.r[8].s64 = ctx.r[11].s64 + -24960;
	// 82654790: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82654794: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 82654798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265479C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826547A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826547A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826547A8: 386A5E48  addi r3, r10, 0x5e48
	ctx.r[3].s64 = ctx.r[10].s64 + 24136;
	// 826547AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826547B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826547B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826547B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826547BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826547C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826547C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826547C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826547CC: 4BE12655  bl 0x82466e20
	ctx.lr = 0x826547D0;
	sub_82466E20(ctx, base);
	// 826547D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826547D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826547D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826547DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826547E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826547E0 size=116
    let mut pc: u32 = 0x826547E0;
    'dispatch: loop {
        match pc {
            0x826547E0 => {
    //   block [0x826547E0..0x82654854)
	// 826547E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826547E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826547E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826547EC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826547F0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826547F4: 390A9E98  addi r8, r10, -0x6168
	ctx.r[8].s64 = ctx.r[10].s64 + -24936;
	// 826547F8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826547FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82654800: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 82654804: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654808: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265480C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654810: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654814: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 82654818: 396BCB44  addi r11, r11, -0x34bc
	ctx.r[11].s64 = ctx.r[11].s64 + -13500;
	// 8265481C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654820: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654824: 386A5E78  addi r3, r10, 0x5e78
	ctx.r[3].s64 = ctx.r[10].s64 + 24184;
	// 82654828: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265482C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654830: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82654834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265483C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654840: 4BE125E1  bl 0x82466e20
	ctx.lr = 0x82654844;
	sub_82466E20(ctx, base);
	// 82654844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265484C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654858 size=108
    let mut pc: u32 = 0x82654858;
    'dispatch: loop {
        match pc {
            0x82654858 => {
    //   block [0x82654858..0x826548C4)
	// 82654858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265485C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654864: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654868: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265486C: 38EB9EF8  addi r7, r11, -0x6108
	ctx.r[7].s64 = ctx.r[11].s64 + -24840;
	// 82654870: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82654874: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 82654878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265487C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654880: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82654884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654888: 386A5EA8  addi r3, r10, 0x5ea8
	ctx.r[3].s64 = ctx.r[10].s64 + 24232;
	// 8265488C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82654890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265489C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826548A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826548A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826548A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826548AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826548B0: 4BE12571  bl 0x82466e20
	ctx.lr = 0x826548B4;
	sub_82466E20(ctx, base);
	// 826548B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826548B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826548BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826548C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826548C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826548C8 size=112
    let mut pc: u32 = 0x826548C8;
    'dispatch: loop {
        match pc {
            0x826548C8 => {
    //   block [0x826548C8..0x82654938)
	// 826548C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826548CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826548D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826548D4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826548D8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826548DC: 38EA9FA0  addi r7, r10, -0x6060
	ctx.r[7].s64 = ctx.r[10].s64 + -24672;
	// 826548E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826548E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826548E8: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 826548EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826548F0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826548F4: 396BCB58  addi r11, r11, -0x34a8
	ctx.r[11].s64 = ctx.r[11].s64 + -13480;
	// 826548F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826548FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654900: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654904: 386A5ED8  addi r3, r10, 0x5ed8
	ctx.r[3].s64 = ctx.r[10].s64 + 24280;
	// 82654908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265490C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82654910: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654914: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82654918: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265491C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654920: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82654924: 4BE124FD  bl 0x82466e20
	ctx.lr = 0x82654928;
	sub_82466E20(ctx, base);
	// 82654928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265492C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654938 size=112
    let mut pc: u32 = 0x82654938;
    'dispatch: loop {
        match pc {
            0x82654938 => {
    //   block [0x82654938..0x826549A8)
	// 82654938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265493C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654944: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654948: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265494C: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 82654950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654954: 390BA018  addi r8, r11, -0x5fe8
	ctx.r[8].s64 = ctx.r[11].s64 + -24552;
	// 82654958: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265495C: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 82654960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82654964: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265496C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654970: 386A5F08  addi r3, r10, 0x5f08
	ctx.r[3].s64 = ctx.r[10].s64 + 24328;
	// 82654974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82654978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265497C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265498C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654994: 4BE1248D  bl 0x82466e20
	ctx.lr = 0x82654998;
	sub_82466E20(ctx, base);
	// 82654998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265499C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826549A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826549A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826549A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826549A8 size=112
    let mut pc: u32 = 0x826549A8;
    'dispatch: loop {
        match pc {
            0x826549A8 => {
    //   block [0x826549A8..0x82654A18)
	// 826549A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826549AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826549B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826549B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826549B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826549BC: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 826549C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826549C4: 390BA060  addi r8, r11, -0x5fa0
	ctx.r[8].s64 = ctx.r[11].s64 + -24480;
	// 826549C8: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826549CC: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826549D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826549D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826549D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826549DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826549E0: 386A5F38  addi r3, r10, 0x5f38
	ctx.r[3].s64 = ctx.r[10].s64 + 24376;
	// 826549E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826549E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826549EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826549F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826549F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826549F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826549FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654A04: 4BE1241D  bl 0x82466e20
	ctx.lr = 0x82654A08;
	sub_82466E20(ctx, base);
	// 82654A08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654A18 size=112
    let mut pc: u32 = 0x82654A18;
    'dispatch: loop {
        match pc {
            0x82654A18 => {
    //   block [0x82654A18..0x82654A88)
	// 82654A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654A24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654A28: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654A2C: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 82654A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654A34: 390BA168  addi r8, r11, -0x5e98
	ctx.r[8].s64 = ctx.r[11].s64 + -24216;
	// 82654A38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82654A3C: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 82654A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82654A44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654A48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654A50: 386A5F68  addi r3, r10, 0x5f68
	ctx.r[3].s64 = ctx.r[10].s64 + 24424;
	// 82654A54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82654A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82654A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654A74: 4BE123AD  bl 0x82466e20
	ctx.lr = 0x82654A78;
	sub_82466E20(ctx, base);
	// 82654A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654A88 size=112
    let mut pc: u32 = 0x82654A88;
    'dispatch: loop {
        match pc {
            0x82654A88 => {
    //   block [0x82654A88..0x82654AF8)
	// 82654A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654A94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654A98: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654A9C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82654AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654AA4: 390BA180  addi r8, r11, -0x5e80
	ctx.r[8].s64 = ctx.r[11].s64 + -24192;
	// 82654AA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82654AAC: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 82654AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82654AB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654AB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654AC0: 386A5F98  addi r3, r10, 0x5f98
	ctx.r[3].s64 = ctx.r[10].s64 + 24472;
	// 82654AC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82654AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82654ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654AE4: 4BE1233D  bl 0x82466e20
	ctx.lr = 0x82654AE8;
	sub_82466E20(ctx, base);
	// 82654AE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654AF8 size=108
    let mut pc: u32 = 0x82654AF8;
    'dispatch: loop {
        match pc {
            0x82654AF8 => {
    //   block [0x82654AF8..0x82654B64)
	// 82654AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654B04: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654B0C: 38EBA1B0  addi r7, r11, -0x5e50
	ctx.r[7].s64 = ctx.r[11].s64 + -24144;
	// 82654B10: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82654B14: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 82654B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82654B1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654B20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82654B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654B28: 386A5FC8  addi r3, r10, 0x5fc8
	ctx.r[3].s64 = ctx.r[10].s64 + 24520;
	// 82654B2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82654B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82654B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654B4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82654B50: 4BE122D1  bl 0x82466e20
	ctx.lr = 0x82654B54;
	sub_82466E20(ctx, base);
	// 82654B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82654B68 size=24
    let mut pc: u32 = 0x82654B68;
    'dispatch: loop {
        match pc {
            0x82654B68 => {
    //   block [0x82654B68..0x82654B80)
	// 82654B68: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654B6C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82654B70: 394A1410  addi r10, r10, 0x1410
	ctx.r[10].s64 = ctx.r[10].s64 + 5136;
	// 82654B74: 816BA228  lwz r11, -0x5dd8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24024 as u32) ) } as u64;
	// 82654B78: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82654B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654B80 size=116
    let mut pc: u32 = 0x82654B80;
    'dispatch: loop {
        match pc {
            0x82654B80 => {
    //   block [0x82654B80..0x82654BF4)
	// 82654B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654B8C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654B90: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82654B94: 390B1410  addi r8, r11, 0x1410
	ctx.r[8].s64 = ctx.r[11].s64 + 5136;
	// 82654B98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82654B9C: 392ACB90  addi r9, r10, -0x3470
	ctx.r[9].s64 = ctx.r[10].s64 + -13424;
	// 82654BA0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654BA4: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82654BA8: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 82654BAC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654BB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82654BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654BC4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82654BC8: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 82654BCC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82654BD0: 386B5FF8  addi r3, r11, 0x5ff8
	ctx.r[3].s64 = ctx.r[11].s64 + 24568;
	// 82654BD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82654BD8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654BE0: 4BE12241  bl 0x82466e20
	ctx.lr = 0x82654BE4;
	sub_82466E20(ctx, base);
	// 82654BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654BF8 size=112
    let mut pc: u32 = 0x82654BF8;
    'dispatch: loop {
        match pc {
            0x82654BF8 => {
    //   block [0x82654BF8..0x82654C68)
	// 82654BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654C04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654C08: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654C0C: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 82654C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654C14: 390BA22C  addi r8, r11, -0x5dd4
	ctx.r[8].s64 = ctx.r[11].s64 + -24020;
	// 82654C18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82654C1C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 82654C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82654C24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654C30: 386A6028  addi r3, r10, 0x6028
	ctx.r[3].s64 = ctx.r[10].s64 + 24616;
	// 82654C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82654C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82654C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654C54: 4BE121CD  bl 0x82466e20
	ctx.lr = 0x82654C58;
	sub_82466E20(ctx, base);
	// 82654C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654C68 size=116
    let mut pc: u32 = 0x82654C68;
    'dispatch: loop {
        match pc {
            0x82654C68 => {
    //   block [0x82654C68..0x82654CDC)
	// 82654C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654C74: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82654C78: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82654C7C: 390AA260  addi r8, r10, -0x5da0
	ctx.r[8].s64 = ctx.r[10].s64 + -23968;
	// 82654C80: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654C84: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82654C88: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 82654C8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654C90: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82654C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654C9C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 82654CA0: 396BCBA4  addi r11, r11, -0x345c
	ctx.r[11].s64 = ctx.r[11].s64 + -13404;
	// 82654CA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654CA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654CAC: 386A6058  addi r3, r10, 0x6058
	ctx.r[3].s64 = ctx.r[10].s64 + 24664;
	// 82654CB0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82654CB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654CB8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82654CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654CC8: 4BE12159  bl 0x82466e20
	ctx.lr = 0x82654CCC;
	sub_82466E20(ctx, base);
	// 82654CCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654CE0 size=112
    let mut pc: u32 = 0x82654CE0;
    'dispatch: loop {
        match pc {
            0x82654CE0 => {
    //   block [0x82654CE0..0x82654D50)
	// 82654CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654CEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654CF0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654CF4: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 82654CF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654CFC: 390BA320  addi r8, r11, -0x5ce0
	ctx.r[8].s64 = ctx.r[11].s64 + -23776;
	// 82654D00: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82654D04: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 82654D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82654D0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654D10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654D18: 386A6088  addi r3, r10, 0x6088
	ctx.r[3].s64 = ctx.r[10].s64 + 24712;
	// 82654D1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82654D20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82654D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654D3C: 4BE120E5  bl 0x82466e20
	ctx.lr = 0x82654D40;
	sub_82466E20(ctx, base);
	// 82654D40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654D50 size=108
    let mut pc: u32 = 0x82654D50;
    'dispatch: loop {
        match pc {
            0x82654D50 => {
    //   block [0x82654D50..0x82654DBC)
	// 82654D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654D5C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654D64: 38EBA338  addi r7, r11, -0x5cc8
	ctx.r[7].s64 = ctx.r[11].s64 + -23752;
	// 82654D68: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 82654D6C: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 82654D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82654D74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654D78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82654D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654D80: 386A60B8  addi r3, r10, 0x60b8
	ctx.r[3].s64 = ctx.r[10].s64 + 24760;
	// 82654D84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82654D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654D8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82654D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654DA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82654DA8: 4BE12079  bl 0x82466e20
	ctx.lr = 0x82654DAC;
	sub_82466E20(ctx, base);
	// 82654DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654DC0 size=116
    let mut pc: u32 = 0x82654DC0;
    'dispatch: loop {
        match pc {
            0x82654DC0 => {
    //   block [0x82654DC0..0x82654E34)
	// 82654DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654DCC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82654DD0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82654DD4: 390AA470  addi r8, r10, -0x5b90
	ctx.r[8].s64 = ctx.r[10].s64 + -23440;
	// 82654DD8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654DDC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82654DE0: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 82654DE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654DE8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82654DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654DF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654DF4: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 82654DF8: 396BCBC8  addi r11, r11, -0x3438
	ctx.r[11].s64 = ctx.r[11].s64 + -13368;
	// 82654DFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654E00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654E04: 386A60E8  addi r3, r10, 0x60e8
	ctx.r[3].s64 = ctx.r[10].s64 + 24808;
	// 82654E08: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82654E0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654E10: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82654E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654E20: 4BE12001  bl 0x82466e20
	ctx.lr = 0x82654E24;
	sub_82466E20(ctx, base);
	// 82654E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654E38 size=112
    let mut pc: u32 = 0x82654E38;
    'dispatch: loop {
        match pc {
            0x82654E38 => {
    //   block [0x82654E38..0x82654EA8)
	// 82654E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654E44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654E48: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654E4C: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 82654E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654E54: 390BA518  addi r8, r11, -0x5ae8
	ctx.r[8].s64 = ctx.r[11].s64 + -23272;
	// 82654E58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82654E5C: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 82654E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82654E64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654E68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654E70: 386A6118  addi r3, r10, 0x6118
	ctx.r[3].s64 = ctx.r[10].s64 + 24856;
	// 82654E74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82654E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82654E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654E94: 4BE11F8D  bl 0x82466e20
	ctx.lr = 0x82654E98;
	sub_82466E20(ctx, base);
	// 82654E98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654EA8 size=112
    let mut pc: u32 = 0x82654EA8;
    'dispatch: loop {
        match pc {
            0x82654EA8 => {
    //   block [0x82654EA8..0x82654F18)
	// 82654EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654EB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654EB8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654EBC: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 82654EC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654EC4: 390BA530  addi r8, r11, -0x5ad0
	ctx.r[8].s64 = ctx.r[11].s64 + -23248;
	// 82654EC8: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 82654ECC: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 82654ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82654ED4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654ED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654EE0: 386A6148  addi r3, r10, 0x6148
	ctx.r[3].s64 = ctx.r[10].s64 + 24904;
	// 82654EE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82654EE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654EEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654EF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654EF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82654EFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654F00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654F04: 4BE11F1D  bl 0x82466e20
	ctx.lr = 0x82654F08;
	sub_82466E20(ctx, base);
	// 82654F08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654F18 size=112
    let mut pc: u32 = 0x82654F18;
    'dispatch: loop {
        match pc {
            0x82654F18 => {
    //   block [0x82654F18..0x82654F88)
	// 82654F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654F24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654F28: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654F2C: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 82654F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654F34: 390BA668  addi r8, r11, -0x5998
	ctx.r[8].s64 = ctx.r[11].s64 + -22936;
	// 82654F38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82654F3C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 82654F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82654F44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654F48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654F50: 386A6178  addi r3, r10, 0x6178
	ctx.r[3].s64 = ctx.r[10].s64 + 24952;
	// 82654F54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82654F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82654F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82654F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82654F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654F74: 4BE11EAD  bl 0x82466e20
	ctx.lr = 0x82654F78;
	sub_82466E20(ctx, base);
	// 82654F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82654F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82654F88 size=116
    let mut pc: u32 = 0x82654F88;
    'dispatch: loop {
        match pc {
            0x82654F88 => {
    //   block [0x82654F88..0x82654FFC)
	// 82654F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82654F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82654F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82654F94: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82654F98: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82654F9C: 390BA680  addi r8, r11, -0x5980
	ctx.r[8].s64 = ctx.r[11].s64 + -22912;
	// 82654FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82654FA4: 392ACC00  addi r9, r10, -0x3400
	ctx.r[9].s64 = ctx.r[10].s64 + -13312;
	// 82654FA8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82654FAC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82654FB0: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82654FB4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82654FB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82654FBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82654FC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82654FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82654FC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82654FCC: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82654FD0: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 82654FD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82654FD8: 386B61A8  addi r3, r11, 0x61a8
	ctx.r[3].s64 = ctx.r[11].s64 + 25000;
	// 82654FDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82654FE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82654FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82654FE8: 4BE11E39  bl 0x82466e20
	ctx.lr = 0x82654FEC;
	sub_82466E20(ctx, base);
	// 82654FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82654FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82654FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82654FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655000 size=100
    let mut pc: u32 = 0x82655000;
    'dispatch: loop {
        match pc {
            0x82655000 => {
    //   block [0x82655000..0x82655064)
	// 82655000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265500C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655014: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82655018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265501C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655020: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 82655024: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265502C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655034: 386A61D8  addi r3, r10, 0x61d8
	ctx.r[3].s64 = ctx.r[10].s64 + 25048;
	// 82655038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265503C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655040: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82655044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655048: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265504C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655050: 4BE11DD1  bl 0x82466e20
	ctx.lr = 0x82655054;
	sub_82466E20(ctx, base);
	// 82655054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265505C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655068 size=112
    let mut pc: u32 = 0x82655068;
    'dispatch: loop {
        match pc {
            0x82655068 => {
    //   block [0x82655068..0x826550D8)
	// 82655068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265506C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655074: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655078: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265507C: 38AA61D8  addi r5, r10, 0x61d8
	ctx.r[5].s64 = ctx.r[10].s64 + 25048;
	// 82655080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655084: 390BA6B0  addi r8, r11, -0x5950
	ctx.r[8].s64 = ctx.r[11].s64 + -22864;
	// 82655088: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265508C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 82655090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655094: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265509C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826550A0: 386A6208  addi r3, r10, 0x6208
	ctx.r[3].s64 = ctx.r[10].s64 + 25096;
	// 826550A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826550A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826550AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826550B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826550B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826550B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826550BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826550C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826550C4: 4BE11D5D  bl 0x82466e20
	ctx.lr = 0x826550C8;
	sub_82466E20(ctx, base);
	// 826550C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826550CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826550D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826550D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826550D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826550D8 size=112
    let mut pc: u32 = 0x826550D8;
    'dispatch: loop {
        match pc {
            0x826550D8 => {
    //   block [0x826550D8..0x82655148)
	// 826550D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826550DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826550E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826550E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826550E8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826550EC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 826550F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826550F4: 390BA6C8  addi r8, r11, -0x5938
	ctx.r[8].s64 = ctx.r[11].s64 + -22840;
	// 826550F8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826550FC: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 82655100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655104: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265510C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655110: 386A6238  addi r3, r10, 0x6238
	ctx.r[3].s64 = ctx.r[10].s64 + 25144;
	// 82655114: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82655118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265511C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265512C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655134: 4BE11CED  bl 0x82466e20
	ctx.lr = 0x82655138;
	sub_82466E20(ctx, base);
	// 82655138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265513C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655148 size=112
    let mut pc: u32 = 0x82655148;
    'dispatch: loop {
        match pc {
            0x82655148 => {
    //   block [0x82655148..0x826551B8)
	// 82655148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265514C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655154: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655158: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265515C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82655160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655164: 390BA770  addi r8, r11, -0x5890
	ctx.r[8].s64 = ctx.r[11].s64 + -22672;
	// 82655168: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265516C: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 82655170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655174: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655178: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265517C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655180: 386A6268  addi r3, r10, 0x6268
	ctx.r[3].s64 = ctx.r[10].s64 + 25192;
	// 82655184: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82655188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265518C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265519C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826551A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826551A4: 4BE11C7D  bl 0x82466e20
	ctx.lr = 0x826551A8;
	sub_82466E20(ctx, base);
	// 826551A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826551AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826551B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826551B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826551B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826551B8 size=112
    let mut pc: u32 = 0x826551B8;
    'dispatch: loop {
        match pc {
            0x826551B8 => {
    //   block [0x826551B8..0x82655228)
	// 826551B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826551BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826551C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826551C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826551C8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826551CC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 826551D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826551D4: 390BA7B8  addi r8, r11, -0x5848
	ctx.r[8].s64 = ctx.r[11].s64 + -22600;
	// 826551D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826551DC: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 826551E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826551E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826551E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826551EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826551F0: 386A6298  addi r3, r10, 0x6298
	ctx.r[3].s64 = ctx.r[10].s64 + 25240;
	// 826551F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826551F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826551FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265520C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655214: 4BE11C0D  bl 0x82466e20
	ctx.lr = 0x82655218;
	sub_82466E20(ctx, base);
	// 82655218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265521C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655228 size=116
    let mut pc: u32 = 0x82655228;
    'dispatch: loop {
        match pc {
            0x82655228 => {
    //   block [0x82655228..0x8265529C)
	// 82655228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265522C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655234: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82655238: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8265523C: 390AA7E8  addi r8, r10, -0x5818
	ctx.r[8].s64 = ctx.r[10].s64 + -22552;
	// 82655240: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655244: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82655248: 38AA5C68  addi r5, r10, 0x5c68
	ctx.r[5].s64 = ctx.r[10].s64 + 23656;
	// 8265524C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655250: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82655254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265525C: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 82655260: 396BCC14  addi r11, r11, -0x33ec
	ctx.r[11].s64 = ctx.r[11].s64 + -13292;
	// 82655264: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655268: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265526C: 386A62C8  addi r3, r10, 0x62c8
	ctx.r[3].s64 = ctx.r[10].s64 + 25288;
	// 82655270: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82655274: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655278: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265527C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655288: 4BE11B99  bl 0x82466e20
	ctx.lr = 0x8265528C;
	sub_82466E20(ctx, base);
	// 8265528C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826552A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826552A0 size=100
    let mut pc: u32 = 0x826552A0;
    'dispatch: loop {
        match pc {
            0x826552A0 => {
    //   block [0x826552A0..0x82655304)
	// 826552A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826552A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826552A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826552AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826552B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826552B4: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 826552B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826552BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826552C0: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 826552C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826552C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826552CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826552D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826552D4: 386A62F8  addi r3, r10, 0x62f8
	ctx.r[3].s64 = ctx.r[10].s64 + 25336;
	// 826552D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826552DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826552E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826552E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826552E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826552EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826552F0: 4BE11B31  bl 0x82466e20
	ctx.lr = 0x826552F4;
	sub_82466E20(ctx, base);
	// 826552F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826552F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826552FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655308 size=108
    let mut pc: u32 = 0x82655308;
    'dispatch: loop {
        match pc {
            0x82655308 => {
    //   block [0x82655308..0x82655374)
	// 82655308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265530C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655314: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265531C: 38EBA8A8  addi r7, r11, -0x5758
	ctx.r[7].s64 = ctx.r[11].s64 + -22360;
	// 82655320: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82655324: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 82655328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265532C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82655334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655338: 386A6328  addi r3, r10, 0x6328
	ctx.r[3].s64 = ctx.r[10].s64 + 25384;
	// 8265533C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82655340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265534C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265535C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82655360: 4BE11AC1  bl 0x82466e20
	ctx.lr = 0x82655364;
	sub_82466E20(ctx, base);
	// 82655364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265536C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655378 size=112
    let mut pc: u32 = 0x82655378;
    'dispatch: loop {
        match pc {
            0x82655378 => {
    //   block [0x82655378..0x826553E8)
	// 82655378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265537C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655384: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655388: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265538C: 38AA62F8  addi r5, r10, 0x62f8
	ctx.r[5].s64 = ctx.r[10].s64 + 25336;
	// 82655390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655394: 390BA8D8  addi r8, r11, -0x5728
	ctx.r[8].s64 = ctx.r[11].s64 + -22312;
	// 82655398: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265539C: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 826553A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826553A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826553A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826553AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826553B0: 386A6358  addi r3, r10, 0x6358
	ctx.r[3].s64 = ctx.r[10].s64 + 25432;
	// 826553B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826553B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826553BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826553C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826553C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826553C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826553CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826553D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826553D4: 4BE11A4D  bl 0x82466e20
	ctx.lr = 0x826553D8;
	sub_82466E20(ctx, base);
	// 826553D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826553DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826553E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826553E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826553E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826553E8 size=108
    let mut pc: u32 = 0x826553E8;
    'dispatch: loop {
        match pc {
            0x826553E8 => {
    //   block [0x826553E8..0x82655454)
	// 826553E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826553EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826553F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826553F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826553F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826553FC: 38EBA908  addi r7, r11, -0x56f8
	ctx.r[7].s64 = ctx.r[11].s64 + -22264;
	// 82655400: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82655404: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 82655408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265540C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655410: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82655414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655418: 386A6388  addi r3, r10, 0x6388
	ctx.r[3].s64 = ctx.r[10].s64 + 25480;
	// 8265541C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82655420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265542C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265543C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82655440: 4BE119E1  bl 0x82466e20
	ctx.lr = 0x82655444;
	sub_82466E20(ctx, base);
	// 82655444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265544C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655458 size=112
    let mut pc: u32 = 0x82655458;
    'dispatch: loop {
        match pc {
            0x82655458 => {
    //   block [0x82655458..0x826554C8)
	// 82655458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265545C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655464: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655468: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265546C: 38AA62F8  addi r5, r10, 0x62f8
	ctx.r[5].s64 = ctx.r[10].s64 + 25336;
	// 82655470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655474: 390BA938  addi r8, r11, -0x56c8
	ctx.r[8].s64 = ctx.r[11].s64 + -22216;
	// 82655478: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265547C: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 82655480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655484: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265548C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655490: 386A63B8  addi r3, r10, 0x63b8
	ctx.r[3].s64 = ctx.r[10].s64 + 25528;
	// 82655494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82655498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265549C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826554A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826554A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826554A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826554AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826554B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826554B4: 4BE1196D  bl 0x82466e20
	ctx.lr = 0x826554B8;
	sub_82466E20(ctx, base);
	// 826554B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826554BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826554C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826554C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826554C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826554C8 size=108
    let mut pc: u32 = 0x826554C8;
    'dispatch: loop {
        match pc {
            0x826554C8 => {
    //   block [0x826554C8..0x82655534)
	// 826554C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826554CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826554D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826554D4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826554D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826554DC: 38EBA980  addi r7, r11, -0x5680
	ctx.r[7].s64 = ctx.r[11].s64 + -22144;
	// 826554E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826554E4: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 826554E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826554EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826554F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826554F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826554F8: 386A63E8  addi r3, r10, 0x63e8
	ctx.r[3].s64 = ctx.r[10].s64 + 25576;
	// 826554FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82655500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265550C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265551C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82655520: 4BE11901  bl 0x82466e20
	ctx.lr = 0x82655524;
	sub_82466E20(ctx, base);
	// 82655524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265552C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655538 size=112
    let mut pc: u32 = 0x82655538;
    'dispatch: loop {
        match pc {
            0x82655538 => {
    //   block [0x82655538..0x826555A8)
	// 82655538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265553C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655544: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655548: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265554C: 38AA62F8  addi r5, r10, 0x62f8
	ctx.r[5].s64 = ctx.r[10].s64 + 25336;
	// 82655550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655554: 390BA9B0  addi r8, r11, -0x5650
	ctx.r[8].s64 = ctx.r[11].s64 + -22096;
	// 82655558: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265555C: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 82655560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655564: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265556C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655570: 386A6418  addi r3, r10, 0x6418
	ctx.r[3].s64 = ctx.r[10].s64 + 25624;
	// 82655574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82655578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265557C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265558C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655594: 4BE1188D  bl 0x82466e20
	ctx.lr = 0x82655598;
	sub_82466E20(ctx, base);
	// 82655598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265559C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826555A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826555A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826555A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826555A8 size=108
    let mut pc: u32 = 0x826555A8;
    'dispatch: loop {
        match pc {
            0x826555A8 => {
    //   block [0x826555A8..0x82655614)
	// 826555A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826555AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826555B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826555B4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826555B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826555BC: 38EBA9F8  addi r7, r11, -0x5608
	ctx.r[7].s64 = ctx.r[11].s64 + -22024;
	// 826555C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826555C4: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 826555C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826555CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826555D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826555D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826555D8: 386A6448  addi r3, r10, 0x6448
	ctx.r[3].s64 = ctx.r[10].s64 + 25672;
	// 826555DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826555E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826555E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826555E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826555EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826555F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826555F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826555F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826555FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82655600: 4BE11821  bl 0x82466e20
	ctx.lr = 0x82655604;
	sub_82466E20(ctx, base);
	// 82655604: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265560C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655618 size=112
    let mut pc: u32 = 0x82655618;
    'dispatch: loop {
        match pc {
            0x82655618 => {
    //   block [0x82655618..0x82655688)
	// 82655618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265561C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655624: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655628: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265562C: 38AA62F8  addi r5, r10, 0x62f8
	ctx.r[5].s64 = ctx.r[10].s64 + 25336;
	// 82655630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655634: 390BAA28  addi r8, r11, -0x55d8
	ctx.r[8].s64 = ctx.r[11].s64 + -21976;
	// 82655638: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265563C: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 82655640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655644: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265564C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655650: 386A6478  addi r3, r10, 0x6478
	ctx.r[3].s64 = ctx.r[10].s64 + 25720;
	// 82655654: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82655658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265565C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265566C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655674: 4BE117AD  bl 0x82466e20
	ctx.lr = 0x82655678;
	sub_82466E20(ctx, base);
	// 82655678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265567C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655688 size=108
    let mut pc: u32 = 0x82655688;
    'dispatch: loop {
        match pc {
            0x82655688 => {
    //   block [0x82655688..0x826556F4)
	// 82655688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265568C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655694: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655698: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265569C: 38EBAA78  addi r7, r11, -0x5588
	ctx.r[7].s64 = ctx.r[11].s64 + -21896;
	// 826556A0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826556A4: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 826556A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826556AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826556B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826556B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826556B8: 386A64A8  addi r3, r10, 0x64a8
	ctx.r[3].s64 = ctx.r[10].s64 + 25768;
	// 826556BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826556C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826556C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826556C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826556CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826556D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826556D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826556D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826556DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826556E0: 4BE11741  bl 0x82466e20
	ctx.lr = 0x826556E4;
	sub_82466E20(ctx, base);
	// 826556E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826556E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826556EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826556F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826556F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826556F8 size=112
    let mut pc: u32 = 0x826556F8;
    'dispatch: loop {
        match pc {
            0x826556F8 => {
    //   block [0x826556F8..0x82655768)
	// 826556F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826556FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655704: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82655708: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265570C: 392ACCD0  addi r9, r10, -0x3330
	ctx.r[9].s64 = ctx.r[10].s64 + -13104;
	// 82655710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655714: 390BAAD8  addi r8, r11, -0x5528
	ctx.r[8].s64 = ctx.r[11].s64 + -21800;
	// 82655718: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8265571C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 82655720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655724: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265572C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655730: 386A64D8  addi r3, r10, 0x64d8
	ctx.r[3].s64 = ctx.r[10].s64 + 25816;
	// 82655734: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82655738: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265573C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265574C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82655750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655754: 4BE116CD  bl 0x82466e20
	ctx.lr = 0x82655758;
	sub_82466E20(ctx, base);
	// 82655758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265575C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655768 size=108
    let mut pc: u32 = 0x82655768;
    'dispatch: loop {
        match pc {
            0x82655768 => {
    //   block [0x82655768..0x826557D4)
	// 82655768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265576C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655774: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265577C: 38EBAB98  addi r7, r11, -0x5468
	ctx.r[7].s64 = ctx.r[11].s64 + -21608;
	// 82655780: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82655784: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 82655788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265578C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655790: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82655794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655798: 386A6508  addi r3, r10, 0x6508
	ctx.r[3].s64 = ctx.r[10].s64 + 25864;
	// 8265579C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826557A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826557A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826557A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826557AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826557B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826557B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826557B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826557BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826557C0: 4BE11661  bl 0x82466e20
	ctx.lr = 0x826557C4;
	sub_82466E20(ctx, base);
	// 826557C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826557C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826557CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826557D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826557D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826557D8 size=108
    let mut pc: u32 = 0x826557D8;
    'dispatch: loop {
        match pc {
            0x826557D8 => {
    //   block [0x826557D8..0x82655844)
	// 826557D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826557DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826557E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826557E4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826557E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826557EC: 38EBABE0  addi r7, r11, -0x5420
	ctx.r[7].s64 = ctx.r[11].s64 + -21536;
	// 826557F0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826557F4: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826557F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826557FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82655804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655808: 386A6538  addi r3, r10, 0x6538
	ctx.r[3].s64 = ctx.r[10].s64 + 25912;
	// 8265580C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82655810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265581C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265582C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82655830: 4BE115F1  bl 0x82466e20
	ctx.lr = 0x82655834;
	sub_82466E20(ctx, base);
	// 82655834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265583C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82655848 size=24
    let mut pc: u32 = 0x82655848;
    'dispatch: loop {
        match pc {
            0x82655848 => {
    //   block [0x82655848..0x82655860)
	// 82655848: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265584C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82655850: 394A1518  addi r10, r10, 0x1518
	ctx.r[10].s64 = ctx.r[10].s64 + 5400;
	// 82655854: 816BAA70  lwz r11, -0x5590(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21904 as u32) ) } as u64;
	// 82655858: 916A0098  stw r11, 0x98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 8265585C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655860 size=116
    let mut pc: u32 = 0x82655860;
    'dispatch: loop {
        match pc {
            0x82655860 => {
    //   block [0x82655860..0x826558D4)
	// 82655860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265586C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82655870: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655874: 392BCC5C  addi r9, r11, -0x33a4
	ctx.r[9].s64 = ctx.r[11].s64 + -13220;
	// 82655878: 38AA59F8  addi r5, r10, 0x59f8
	ctx.r[5].s64 = ctx.r[10].s64 + 23032;
	// 8265587C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655880: 38E9008C  addi r7, r9, 0x8c
	ctx.r[7].s64 = ctx.r[9].s64 + 140;
	// 82655884: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 82655888: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265588C: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 82655890: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655894: 396B1518  addi r11, r11, 0x1518
	ctx.r[11].s64 = ctx.r[11].s64 + 5400;
	// 82655898: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8265589C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826558A0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826558A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826558A8: 386A6568  addi r3, r10, 0x6568
	ctx.r[3].s64 = ctx.r[10].s64 + 25960;
	// 826558AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826558B0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826558B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826558B8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826558BC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826558C0: 4BE11561  bl 0x82466e20
	ctx.lr = 0x826558C4;
	sub_82466E20(ctx, base);
	// 826558C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826558C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826558CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826558D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826558D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826558D8 size=100
    let mut pc: u32 = 0x826558D8;
    'dispatch: loop {
        match pc {
            0x826558D8 => {
    //   block [0x826558D8..0x8265593C)
	// 826558D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826558DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826558E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826558E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826558E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826558EC: 38AA59F8  addi r5, r10, 0x59f8
	ctx.r[5].s64 = ctx.r[10].s64 + 23032;
	// 826558F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826558F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826558F8: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826558FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265590C: 386A6598  addi r3, r10, 0x6598
	ctx.r[3].s64 = ctx.r[10].s64 + 26008;
	// 82655910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655918: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265591C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655920: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82655924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655928: 4BE114F9  bl 0x82466e20
	ctx.lr = 0x8265592C;
	sub_82466E20(ctx, base);
	// 8265592C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82655940 size=24
    let mut pc: u32 = 0x82655940;
    'dispatch: loop {
        match pc {
            0x82655940 => {
    //   block [0x82655940..0x82655958)
	// 82655940: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655944: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82655948: 394A16C8  addi r10, r10, 0x16c8
	ctx.r[10].s64 = ctx.r[10].s64 + 5832;
	// 8265594C: 816BAC74  lwz r11, -0x538c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21388 as u32) ) } as u64;
	// 82655950: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82655954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655958 size=116
    let mut pc: u32 = 0x82655958;
    'dispatch: loop {
        match pc {
            0x82655958 => {
    //   block [0x82655958..0x826559CC)
	// 82655958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265595C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655964: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655968: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265596C: 390B16C8  addi r8, r11, 0x16c8
	ctx.r[8].s64 = ctx.r[11].s64 + 5832;
	// 82655970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655974: 392ACD68  addi r9, r10, -0x3298
	ctx.r[9].s64 = ctx.r[10].s64 + -12952;
	// 82655978: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265597C: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82655980: 38AA6598  addi r5, r10, 0x6598
	ctx.r[5].s64 = ctx.r[10].s64 + 26008;
	// 82655984: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82655988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265598C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265599C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 826559A0: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826559A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826559A8: 386B65C8  addi r3, r11, 0x65c8
	ctx.r[3].s64 = ctx.r[11].s64 + 26056;
	// 826559AC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826559B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826559B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826559B8: 4BE11469  bl 0x82466e20
	ctx.lr = 0x826559BC;
	sub_82466E20(ctx, base);
	// 826559BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826559C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826559C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826559C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826559D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826559D0 size=112
    let mut pc: u32 = 0x826559D0;
    'dispatch: loop {
        match pc {
            0x826559D0 => {
    //   block [0x826559D0..0x82655A40)
	// 826559D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826559D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826559D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826559DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826559E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826559E4: 38AA6598  addi r5, r10, 0x6598
	ctx.r[5].s64 = ctx.r[10].s64 + 26008;
	// 826559E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826559EC: 390BAC78  addi r8, r11, -0x5388
	ctx.r[8].s64 = ctx.r[11].s64 + -21384;
	// 826559F0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826559F4: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 826559F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826559FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655A00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82655A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655A08: 386A65F8  addi r3, r10, 0x65f8
	ctx.r[3].s64 = ctx.r[10].s64 + 26104;
	// 82655A0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82655A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655A2C: 4BE113F5  bl 0x82466e20
	ctx.lr = 0x82655A30;
	sub_82466E20(ctx, base);
	// 82655A30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655A40 size=112
    let mut pc: u32 = 0x82655A40;
    'dispatch: loop {
        match pc {
            0x82655A40 => {
    //   block [0x82655A40..0x82655AB0)
	// 82655A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655A4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655A50: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655A54: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82655A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655A5C: 390BAD50  addi r8, r11, -0x52b0
	ctx.r[8].s64 = ctx.r[11].s64 + -21168;
	// 82655A60: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82655A64: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 82655A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655A6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655A70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82655A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655A78: 386A6628  addi r3, r10, 0x6628
	ctx.r[3].s64 = ctx.r[10].s64 + 26152;
	// 82655A7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82655A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655A9C: 4BE11385  bl 0x82466e20
	ctx.lr = 0x82655AA0;
	sub_82466E20(ctx, base);
	// 82655AA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655AB0 size=108
    let mut pc: u32 = 0x82655AB0;
    'dispatch: loop {
        match pc {
            0x82655AB0 => {
    //   block [0x82655AB0..0x82655B1C)
	// 82655AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655ABC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655AC4: 38EBAE28  addi r7, r11, -0x51d8
	ctx.r[7].s64 = ctx.r[11].s64 + -20952;
	// 82655AC8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82655ACC: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 82655AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655AD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655AD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82655ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655AE0: 386A6658  addi r3, r10, 0x6658
	ctx.r[3].s64 = ctx.r[10].s64 + 26200;
	// 82655AE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82655AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655B04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82655B08: 4BE11319  bl 0x82466e20
	ctx.lr = 0x82655B0C;
	sub_82466E20(ctx, base);
	// 82655B0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655B20 size=108
    let mut pc: u32 = 0x82655B20;
    'dispatch: loop {
        match pc {
            0x82655B20 => {
    //   block [0x82655B20..0x82655B8C)
	// 82655B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655B2C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655B34: 38EBAEA0  addi r7, r11, -0x5160
	ctx.r[7].s64 = ctx.r[11].s64 + -20832;
	// 82655B38: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82655B3C: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 82655B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655B44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655B48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82655B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655B50: 386A6688  addi r3, r10, 0x6688
	ctx.r[3].s64 = ctx.r[10].s64 + 26248;
	// 82655B54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82655B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655B74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82655B78: 4BE112A9  bl 0x82466e20
	ctx.lr = 0x82655B7C;
	sub_82466E20(ctx, base);
	// 82655B7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655B90 size=112
    let mut pc: u32 = 0x82655B90;
    'dispatch: loop {
        match pc {
            0x82655B90 => {
    //   block [0x82655B90..0x82655C00)
	// 82655B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655B9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655BA0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655BA4: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82655BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655BAC: 390BAEE8  addi r8, r11, -0x5118
	ctx.r[8].s64 = ctx.r[11].s64 + -20760;
	// 82655BB0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82655BB4: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 82655BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655BBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655BC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82655BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655BC8: 386A66B8  addi r3, r10, 0x66b8
	ctx.r[3].s64 = ctx.r[10].s64 + 26296;
	// 82655BCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82655BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655BEC: 4BE11235  bl 0x82466e20
	ctx.lr = 0x82655BF0;
	sub_82466E20(ctx, base);
	// 82655BF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655C00 size=108
    let mut pc: u32 = 0x82655C00;
    'dispatch: loop {
        match pc {
            0x82655C00 => {
    //   block [0x82655C00..0x82655C6C)
	// 82655C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655C0C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655C14: 38EBB0C8  addi r7, r11, -0x4f38
	ctx.r[7].s64 = ctx.r[11].s64 + -20280;
	// 82655C18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82655C1C: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 82655C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655C24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655C28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82655C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655C30: 386A66E8  addi r3, r10, 0x66e8
	ctx.r[3].s64 = ctx.r[10].s64 + 26344;
	// 82655C34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82655C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82655C58: 4BE111C9  bl 0x82466e20
	ctx.lr = 0x82655C5C;
	sub_82466E20(ctx, base);
	// 82655C5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82655C70 size=24
    let mut pc: u32 = 0x82655C70;
    'dispatch: loop {
        match pc {
            0x82655C70 => {
    //   block [0x82655C70..0x82655C88)
	// 82655C70: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655C74: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82655C78: 394A17E8  addi r10, r10, 0x17e8
	ctx.r[10].s64 = ctx.r[10].s64 + 6120;
	// 82655C7C: 816BB0E0  lwz r11, -0x4f20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20256 as u32) ) } as u64;
	// 82655C80: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82655C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655C88 size=112
    let mut pc: u32 = 0x82655C88;
    'dispatch: loop {
        match pc {
            0x82655C88 => {
    //   block [0x82655C88..0x82655CF8)
	// 82655C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655C94: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82655C98: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655C9C: 392ACDC0  addi r9, r10, -0x3240
	ctx.r[9].s64 = ctx.r[10].s64 + -12864;
	// 82655CA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655CA4: 390B17E8  addi r8, r11, 0x17e8
	ctx.r[8].s64 = ctx.r[11].s64 + 6120;
	// 82655CA8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82655CAC: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 82655CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655CB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655CB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82655CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655CC0: 386A6718  addi r3, r10, 0x6718
	ctx.r[3].s64 = ctx.r[10].s64 + 26392;
	// 82655CC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82655CC8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82655CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655CD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655CD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655CD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655CDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82655CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655CE4: 4BE1113D  bl 0x82466e20
	ctx.lr = 0x82655CE8;
	sub_82466E20(ctx, base);
	// 82655CE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655CF8 size=112
    let mut pc: u32 = 0x82655CF8;
    'dispatch: loop {
        match pc {
            0x82655CF8 => {
    //   block [0x82655CF8..0x82655D68)
	// 82655CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655D04: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82655D08: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82655D0C: 38EAB0E8  addi r7, r10, -0x4f18
	ctx.r[7].s64 = ctx.r[10].s64 + -20248;
	// 82655D10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655D14: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82655D18: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 82655D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655D20: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82655D24: 396BCDD4  addi r11, r11, -0x322c
	ctx.r[11].s64 = ctx.r[11].s64 + -12844;
	// 82655D28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82655D2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655D30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655D34: 386A6748  addi r3, r10, 0x6748
	ctx.r[3].s64 = ctx.r[10].s64 + 26440;
	// 82655D38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655D3C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82655D40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655D44: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82655D48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655D4C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655D50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82655D54: 4BE110CD  bl 0x82466e20
	ctx.lr = 0x82655D58;
	sub_82466E20(ctx, base);
	// 82655D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655D68 size=112
    let mut pc: u32 = 0x82655D68;
    'dispatch: loop {
        match pc {
            0x82655D68 => {
    //   block [0x82655D68..0x82655DD8)
	// 82655D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655D74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655D78: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655D7C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82655D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655D84: 390BB178  addi r8, r11, -0x4e88
	ctx.r[8].s64 = ctx.r[11].s64 + -20104;
	// 82655D88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82655D8C: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 82655D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655D94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655D98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82655D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655DA0: 386A6778  addi r3, r10, 0x6778
	ctx.r[3].s64 = ctx.r[10].s64 + 26488;
	// 82655DA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82655DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655DC4: 4BE1105D  bl 0x82466e20
	ctx.lr = 0x82655DC8;
	sub_82466E20(ctx, base);
	// 82655DC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655DD8 size=108
    let mut pc: u32 = 0x82655DD8;
    'dispatch: loop {
        match pc {
            0x82655DD8 => {
    //   block [0x82655DD8..0x82655E44)
	// 82655DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655DE4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655DEC: 38EBB190  addi r7, r11, -0x4e70
	ctx.r[7].s64 = ctx.r[11].s64 + -20080;
	// 82655DF0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82655DF4: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 82655DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655DFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655E00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82655E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655E08: 386A67A8  addi r3, r10, 0x67a8
	ctx.r[3].s64 = ctx.r[10].s64 + 26536;
	// 82655E0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82655E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655E2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82655E30: 4BE10FF1  bl 0x82466e20
	ctx.lr = 0x82655E34;
	sub_82466E20(ctx, base);
	// 82655E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655E48 size=108
    let mut pc: u32 = 0x82655E48;
    'dispatch: loop {
        match pc {
            0x82655E48 => {
    //   block [0x82655E48..0x82655EB4)
	// 82655E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655E54: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655E58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655E5C: 38EBB1F0  addi r7, r11, -0x4e10
	ctx.r[7].s64 = ctx.r[11].s64 + -19984;
	// 82655E60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82655E64: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 82655E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655E6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655E70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82655E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655E78: 386A67D8  addi r3, r10, 0x67d8
	ctx.r[3].s64 = ctx.r[10].s64 + 26584;
	// 82655E7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82655E80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655E94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655E9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82655EA0: 4BE10F81  bl 0x82466e20
	ctx.lr = 0x82655EA4;
	sub_82466E20(ctx, base);
	// 82655EA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655EB8 size=116
    let mut pc: u32 = 0x82655EB8;
    'dispatch: loop {
        match pc {
            0x82655EB8 => {
    //   block [0x82655EB8..0x82655F2C)
	// 82655EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655EC4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655EC8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82655ECC: 390BB220  addi r8, r11, -0x4de0
	ctx.r[8].s64 = ctx.r[11].s64 + -19936;
	// 82655ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655ED4: 392ACE08  addi r9, r10, -0x31f8
	ctx.r[9].s64 = ctx.r[10].s64 + -12792;
	// 82655ED8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655EDC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82655EE0: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82655EE4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82655EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655EEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82655EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655EFC: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82655F00: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 82655F04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82655F08: 386B6808  addi r3, r11, 0x6808
	ctx.r[3].s64 = ctx.r[11].s64 + 26632;
	// 82655F0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82655F10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655F18: 4BE10F09  bl 0x82466e20
	ctx.lr = 0x82655F1C;
	sub_82466E20(ctx, base);
	// 82655F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655F30 size=96
    let mut pc: u32 = 0x82655F30;
    'dispatch: loop {
        match pc {
            0x82655F30 => {
    //   block [0x82655F30..0x82655F90)
	// 82655F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655F3C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82655F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655F44: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 82655F48: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655F50: 386A6838  addi r3, r10, 0x6838
	ctx.r[3].s64 = ctx.r[10].s64 + 26680;
	// 82655F54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655F5C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82655F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655F70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82655F74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82655F78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82655F7C: 4BE10EA5  bl 0x82466e20
	ctx.lr = 0x82655F80;
	sub_82466E20(ctx, base);
	// 82655F80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655F84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655F88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82655F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82655F90 size=112
    let mut pc: u32 = 0x82655F90;
    'dispatch: loop {
        match pc {
            0x82655F90 => {
    //   block [0x82655F90..0x82656000)
	// 82655F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82655F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82655F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82655F9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655FA0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82655FA4: 38AA6838  addi r5, r10, 0x6838
	ctx.r[5].s64 = ctx.r[10].s64 + 26680;
	// 82655FA8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82655FAC: 390BB238  addi r8, r11, -0x4dc8
	ctx.r[8].s64 = ctx.r[11].s64 + -19912;
	// 82655FB0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82655FB4: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82655FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82655FBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82655FC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82655FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82655FC8: 386A6868  addi r3, r10, 0x6868
	ctx.r[3].s64 = ctx.r[10].s64 + 26728;
	// 82655FCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82655FD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82655FD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82655FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82655FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82655FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82655FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82655FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82655FEC: 4BE10E35  bl 0x82466e20
	ctx.lr = 0x82655FF0;
	sub_82466E20(ctx, base);
	// 82655FF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82655FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82655FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82655FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656000 size=112
    let mut pc: u32 = 0x82656000;
    'dispatch: loop {
        match pc {
            0x82656000 => {
    //   block [0x82656000..0x82656070)
	// 82656000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265600C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82656010: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656014: 392ACE24  addi r9, r10, -0x31dc
	ctx.r[9].s64 = ctx.r[10].s64 + -12764;
	// 82656018: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8265601C: 390BB270  addi r8, r11, -0x4d90
	ctx.r[8].s64 = ctx.r[11].s64 + -19856;
	// 82656020: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82656024: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 82656028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265602C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82656034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656038: 386A6898  addi r3, r10, 0x6898
	ctx.r[3].s64 = ctx.r[10].s64 + 26776;
	// 8265603C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82656040: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82656044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265604C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265605C: 4BE10DC5  bl 0x82466e20
	ctx.lr = 0x82656060;
	sub_82466E20(ctx, base);
	// 82656060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265606C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656070 size=108
    let mut pc: u32 = 0x82656070;
    'dispatch: loop {
        match pc {
            0x82656070 => {
    //   block [0x82656070..0x826560DC)
	// 82656070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265607C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656080: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82656084: 38EBB318  addi r7, r11, -0x4ce8
	ctx.r[7].s64 = ctx.r[11].s64 + -19688;
	// 82656088: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265608C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 82656090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656094: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265609C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826560A0: 386A68C8  addi r3, r10, 0x68c8
	ctx.r[3].s64 = ctx.r[10].s64 + 26824;
	// 826560A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826560A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826560AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826560B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826560B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826560B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826560BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826560C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826560C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826560C8: 4BE10D59  bl 0x82466e20
	ctx.lr = 0x826560CC;
	sub_82466E20(ctx, base);
	// 826560CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826560D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826560D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826560D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826560E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826560E0 size=108
    let mut pc: u32 = 0x826560E0;
    'dispatch: loop {
        match pc {
            0x826560E0 => {
    //   block [0x826560E0..0x8265614C)
	// 826560E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826560E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826560E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826560EC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826560F0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826560F4: 38EBB348  addi r7, r11, -0x4cb8
	ctx.r[7].s64 = ctx.r[11].s64 + -19640;
	// 826560F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826560FC: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82656100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656104: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656108: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265610C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656110: 386A68F8  addi r3, r10, 0x68f8
	ctx.r[3].s64 = ctx.r[10].s64 + 26872;
	// 82656114: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265611C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265612C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656134: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656138: 4BE10CE9  bl 0x82466e20
	ctx.lr = 0x8265613C;
	sub_82466E20(ctx, base);
	// 8265613C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82656150 size=28
    let mut pc: u32 = 0x82656150;
    'dispatch: loop {
        match pc {
            0x82656150 => {
    //   block [0x82656150..0x8265616C)
	// 82656150: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656154: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82656158: 394A1818  addi r10, r10, 0x1818
	ctx.r[10].s64 = ctx.r[10].s64 + 6168;
	// 8265615C: 816BB26C  lwz r11, -0x4d94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19860 as u32) ) } as u64;
	// 82656160: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82656164: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82656168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656170 size=112
    let mut pc: u32 = 0x82656170;
    'dispatch: loop {
        match pc {
            0x82656170 => {
    //   block [0x82656170..0x826561E0)
	// 82656170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265617C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82656180: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656184: 392ACFD0  addi r9, r10, -0x3030
	ctx.r[9].s64 = ctx.r[10].s64 + -12336;
	// 82656188: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8265618C: 390B1818  addi r8, r11, 0x1818
	ctx.r[8].s64 = ctx.r[11].s64 + 6168;
	// 82656190: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82656194: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 82656198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265619C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826561A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826561A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826561A8: 386A6928  addi r3, r10, 0x6928
	ctx.r[3].s64 = ctx.r[10].s64 + 26920;
	// 826561AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826561B0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826561B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826561B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826561BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826561C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826561C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826561C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826561CC: 4BE10C55  bl 0x82466e20
	ctx.lr = 0x826561D0;
	sub_82466E20(ctx, base);
	// 826561D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826561D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826561D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826561DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826561E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826561E0 size=108
    let mut pc: u32 = 0x826561E0;
    'dispatch: loop {
        match pc {
            0x826561E0 => {
    //   block [0x826561E0..0x8265624C)
	// 826561E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826561E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826561E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826561EC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826561F0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826561F4: 38EBB380  addi r7, r11, -0x4c80
	ctx.r[7].s64 = ctx.r[11].s64 + -19584;
	// 826561F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826561FC: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 82656200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656204: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656208: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265620C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656210: 386A6958  addi r3, r10, 0x6958
	ctx.r[3].s64 = ctx.r[10].s64 + 26968;
	// 82656214: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265621C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265622C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656238: 4BE10BE9  bl 0x82466e20
	ctx.lr = 0x8265623C;
	sub_82466E20(ctx, base);
	// 8265623C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656250 size=108
    let mut pc: u32 = 0x82656250;
    'dispatch: loop {
        match pc {
            0x82656250 => {
    //   block [0x82656250..0x826562BC)
	// 82656250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265625C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656260: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82656264: 38EBB3B0  addi r7, r11, -0x4c50
	ctx.r[7].s64 = ctx.r[11].s64 + -19536;
	// 82656268: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265626C: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 82656270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656274: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265627C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656280: 386A6988  addi r3, r10, 0x6988
	ctx.r[3].s64 = ctx.r[10].s64 + 27016;
	// 82656284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265628C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265629C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826562A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826562A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826562A8: 4BE10B79  bl 0x82466e20
	ctx.lr = 0x826562AC;
	sub_82466E20(ctx, base);
	// 826562AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826562B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826562B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826562B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826562C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826562C0 size=108
    let mut pc: u32 = 0x826562C0;
    'dispatch: loop {
        match pc {
            0x826562C0 => {
    //   block [0x826562C0..0x8265632C)
	// 826562C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826562C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826562C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826562CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826562D0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826562D4: 38EBB3E0  addi r7, r11, -0x4c20
	ctx.r[7].s64 = ctx.r[11].s64 + -19488;
	// 826562D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826562DC: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826562E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826562E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826562E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826562EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826562F0: 386A69B8  addi r3, r10, 0x69b8
	ctx.r[3].s64 = ctx.r[10].s64 + 27064;
	// 826562F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826562F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826562FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265630C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656318: 4BE10B09  bl 0x82466e20
	ctx.lr = 0x8265631C;
	sub_82466E20(ctx, base);
	// 8265631C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82656330 size=24
    let mut pc: u32 = 0x82656330;
    'dispatch: loop {
        match pc {
            0x82656330 => {
    //   block [0x82656330..0x82656348)
	// 82656330: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656334: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82656338: 394A18D8  addi r10, r10, 0x18d8
	ctx.r[10].s64 = ctx.r[10].s64 + 6360;
	// 8265633C: 816BB3F8  lwz r11, -0x4c08(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19464 as u32) ) } as u64;
	// 82656340: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82656344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656348 size=112
    let mut pc: u32 = 0x82656348;
    'dispatch: loop {
        match pc {
            0x82656348 => {
    //   block [0x82656348..0x826563B8)
	// 82656348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265634C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656354: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82656358: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265635C: 392AD024  addi r9, r10, -0x2fdc
	ctx.r[9].s64 = ctx.r[10].s64 + -12252;
	// 82656360: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82656364: 390B18D8  addi r8, r11, 0x18d8
	ctx.r[8].s64 = ctx.r[11].s64 + 6360;
	// 82656368: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8265636C: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 82656370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656374: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265637C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656380: 386A69E8  addi r3, r10, 0x69e8
	ctx.r[3].s64 = ctx.r[10].s64 + 27112;
	// 82656384: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82656388: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265638C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265639C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826563A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826563A4: 4BE10A7D  bl 0x82466e20
	ctx.lr = 0x826563A8;
	sub_82466E20(ctx, base);
	// 826563A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826563AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826563B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826563B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826563B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826563B8 size=112
    let mut pc: u32 = 0x826563B8;
    'dispatch: loop {
        match pc {
            0x826563B8 => {
    //   block [0x826563B8..0x82656428)
	// 826563B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826563BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826563C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826563C4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826563C8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826563CC: 392AD060  addi r9, r10, -0x2fa0
	ctx.r[9].s64 = ctx.r[10].s64 + -12192;
	// 826563D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826563D4: 390BB408  addi r8, r11, -0x4bf8
	ctx.r[8].s64 = ctx.r[11].s64 + -19448;
	// 826563D8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826563DC: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 826563E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826563E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826563E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826563EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826563F0: 386A6A18  addi r3, r10, 0x6a18
	ctx.r[3].s64 = ctx.r[10].s64 + 27160;
	// 826563F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826563F8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826563FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265640C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656414: 4BE10A0D  bl 0x82466e20
	ctx.lr = 0x82656418;
	sub_82466E20(ctx, base);
	// 82656418: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265641C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656428 size=108
    let mut pc: u32 = 0x82656428;
    'dispatch: loop {
        match pc {
            0x82656428 => {
    //   block [0x82656428..0x82656494)
	// 82656428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265642C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656434: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656438: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8265643C: 38EBB450  addi r7, r11, -0x4bb0
	ctx.r[7].s64 = ctx.r[11].s64 + -19376;
	// 82656440: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82656444: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 82656448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265644C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656450: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656458: 386A6A48  addi r3, r10, 0x6a48
	ctx.r[3].s64 = ctx.r[10].s64 + 27208;
	// 8265645C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265646C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265647C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656480: 4BE109A1  bl 0x82466e20
	ctx.lr = 0x82656484;
	sub_82466E20(ctx, base);
	// 82656484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265648C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656498 size=108
    let mut pc: u32 = 0x82656498;
    'dispatch: loop {
        match pc {
            0x82656498 => {
    //   block [0x82656498..0x82656504)
	// 82656498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265649C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826564A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826564A4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826564A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826564AC: 38EBB480  addi r7, r11, -0x4b80
	ctx.r[7].s64 = ctx.r[11].s64 + -19328;
	// 826564B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826564B4: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826564B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826564BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826564C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826564C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826564C8: 386A6A78  addi r3, r10, 0x6a78
	ctx.r[3].s64 = ctx.r[10].s64 + 27256;
	// 826564CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826564D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826564D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826564D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826564DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826564E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826564E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826564E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826564EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826564F0: 4BE10931  bl 0x82466e20
	ctx.lr = 0x826564F4;
	sub_82466E20(ctx, base);
	// 826564F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826564F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826564FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656508 size=112
    let mut pc: u32 = 0x82656508;
    'dispatch: loop {
        match pc {
            0x82656508 => {
    //   block [0x82656508..0x82656578)
	// 82656508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265650C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656514: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82656518: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265651C: 392AD0A0  addi r9, r10, -0x2f60
	ctx.r[9].s64 = ctx.r[10].s64 + -12128;
	// 82656520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656524: 390BB4B0  addi r8, r11, -0x4b50
	ctx.r[8].s64 = ctx.r[11].s64 + -19280;
	// 82656528: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8265652C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 82656530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656534: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656538: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265653C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656540: 386A6AA8  addi r3, r10, 0x6aa8
	ctx.r[3].s64 = ctx.r[10].s64 + 27304;
	// 82656544: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82656548: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265654C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265655C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656564: 4BE108BD  bl 0x82466e20
	ctx.lr = 0x82656568;
	sub_82466E20(ctx, base);
	// 82656568: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265656C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656578 size=108
    let mut pc: u32 = 0x82656578;
    'dispatch: loop {
        match pc {
            0x82656578 => {
    //   block [0x82656578..0x826565E4)
	// 82656578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265657C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656584: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656588: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8265658C: 38EBB528  addi r7, r11, -0x4ad8
	ctx.r[7].s64 = ctx.r[11].s64 + -19160;
	// 82656590: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82656594: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 82656598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265659C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826565A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826565A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826565A8: 386A6AD8  addi r3, r10, 0x6ad8
	ctx.r[3].s64 = ctx.r[10].s64 + 27352;
	// 826565AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826565B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826565B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826565B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826565BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826565C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826565C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826565C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826565CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826565D0: 4BE10851  bl 0x82466e20
	ctx.lr = 0x826565D4;
	sub_82466E20(ctx, base);
	// 826565D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826565D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826565DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826565E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826565E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826565E8 size=108
    let mut pc: u32 = 0x826565E8;
    'dispatch: loop {
        match pc {
            0x826565E8 => {
    //   block [0x826565E8..0x82656654)
	// 826565E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826565EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826565F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826565F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826565F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826565FC: 38EBB630  addi r7, r11, -0x49d0
	ctx.r[7].s64 = ctx.r[11].s64 + -18896;
	// 82656600: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82656604: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 82656608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265660C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656618: 386A6B08  addi r3, r10, 0x6b08
	ctx.r[3].s64 = ctx.r[10].s64 + 27400;
	// 8265661C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265662C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265663C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656640: 4BE107E1  bl 0x82466e20
	ctx.lr = 0x82656644;
	sub_82466E20(ctx, base);
	// 82656644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265664C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656658 size=108
    let mut pc: u32 = 0x82656658;
    'dispatch: loop {
        match pc {
            0x82656658 => {
    //   block [0x82656658..0x826566C4)
	// 82656658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265665C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656664: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265666C: 38EBB648  addi r7, r11, -0x49b8
	ctx.r[7].s64 = ctx.r[11].s64 + -18872;
	// 82656670: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82656674: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82656678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265667C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656688: 386A6B38  addi r3, r10, 0x6b38
	ctx.r[3].s64 = ctx.r[10].s64 + 27448;
	// 8265668C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265669C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826566A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826566A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826566A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826566AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826566B0: 4BE10771  bl 0x82466e20
	ctx.lr = 0x826566B4;
	sub_82466E20(ctx, base);
	// 826566B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826566B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826566BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826566C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826566C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826566C8 size=24
    let mut pc: u32 = 0x826566C8;
    'dispatch: loop {
        match pc {
            0x826566C8 => {
    //   block [0x826566C8..0x826566E0)
	// 826566C8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826566CC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826566D0: 394A19B0  addi r10, r10, 0x19b0
	ctx.r[10].s64 = ctx.r[10].s64 + 6576;
	// 826566D4: 816BB6D8  lwz r11, -0x4928(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18728 as u32) ) } as u64;
	// 826566D8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826566DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826566E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826566E0 size=108
    let mut pc: u32 = 0x826566E0;
    'dispatch: loop {
        match pc {
            0x826566E0 => {
    //   block [0x826566E0..0x8265674C)
	// 826566E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826566E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826566E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826566EC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826566F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826566F4: 38EB19B0  addi r7, r11, 0x19b0
	ctx.r[7].s64 = ctx.r[11].s64 + 6576;
	// 826566F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826566FC: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 82656700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656704: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656708: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265670C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656710: 386A6B68  addi r3, r10, 0x6b68
	ctx.r[3].s64 = ctx.r[10].s64 + 27496;
	// 82656714: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265671C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265672C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656734: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656738: 4BE106E9  bl 0x82466e20
	ctx.lr = 0x8265673C;
	sub_82466E20(ctx, base);
	// 8265673C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82656750 size=24
    let mut pc: u32 = 0x82656750;
    'dispatch: loop {
        match pc {
            0x82656750 => {
    //   block [0x82656750..0x82656768)
	// 82656750: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656754: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82656758: 394A19E0  addi r10, r10, 0x19e0
	ctx.r[10].s64 = ctx.r[10].s64 + 6624;
	// 8265675C: 816BB6D8  lwz r11, -0x4928(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18728 as u32) ) } as u64;
	// 82656760: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82656764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656768 size=108
    let mut pc: u32 = 0x82656768;
    'dispatch: loop {
        match pc {
            0x82656768 => {
    //   block [0x82656768..0x826567D4)
	// 82656768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265676C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656774: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265677C: 38EB19E0  addi r7, r11, 0x19e0
	ctx.r[7].s64 = ctx.r[11].s64 + 6624;
	// 82656780: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82656784: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 82656788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265678C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656790: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656798: 386A6B98  addi r3, r10, 0x6b98
	ctx.r[3].s64 = ctx.r[10].s64 + 27544;
	// 8265679C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826567A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826567A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826567A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826567AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826567B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826567B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826567B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826567BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826567C0: 4BE10661  bl 0x82466e20
	ctx.lr = 0x826567C4;
	sub_82466E20(ctx, base);
	// 826567C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826567C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826567CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826567D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826567D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826567D8 size=108
    let mut pc: u32 = 0x826567D8;
    'dispatch: loop {
        match pc {
            0x826567D8 => {
    //   block [0x826567D8..0x82656844)
	// 826567D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826567DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826567E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826567E4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826567E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826567EC: 38EBB6C0  addi r7, r11, -0x4940
	ctx.r[7].s64 = ctx.r[11].s64 + -18752;
	// 826567F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826567F4: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826567F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826567FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656808: 386A6BC8  addi r3, r10, 0x6bc8
	ctx.r[3].s64 = ctx.r[10].s64 + 27592;
	// 8265680C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265681C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265682C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656830: 4BE105F1  bl 0x82466e20
	ctx.lr = 0x82656834;
	sub_82466E20(ctx, base);
	// 82656834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265683C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82656848 size=24
    let mut pc: u32 = 0x82656848;
    'dispatch: loop {
        match pc {
            0x82656848 => {
    //   block [0x82656848..0x82656860)
	// 82656848: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265684C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82656850: 394A1A10  addi r10, r10, 0x1a10
	ctx.r[10].s64 = ctx.r[10].s64 + 6672;
	// 82656854: 816BB6D8  lwz r11, -0x4928(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18728 as u32) ) } as u64;
	// 82656858: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8265685C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656860 size=108
    let mut pc: u32 = 0x82656860;
    'dispatch: loop {
        match pc {
            0x82656860 => {
    //   block [0x82656860..0x826568CC)
	// 82656860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265686C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656874: 38EB1A10  addi r7, r11, 0x1a10
	ctx.r[7].s64 = ctx.r[11].s64 + 6672;
	// 82656878: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265687C: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 82656880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656884: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656888: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265688C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656890: 386A6BF8  addi r3, r10, 0x6bf8
	ctx.r[3].s64 = ctx.r[10].s64 + 27640;
	// 82656894: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265689C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826568A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826568A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826568A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826568AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826568B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826568B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826568B8: 4BE10569  bl 0x82466e20
	ctx.lr = 0x826568BC;
	sub_82466E20(ctx, base);
	// 826568BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826568C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826568C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826568C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826568D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826568D0 size=112
    let mut pc: u32 = 0x826568D0;
    'dispatch: loop {
        match pc {
            0x826568D0 => {
    //   block [0x826568D0..0x82656940)
	// 826568D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826568D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826568D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826568DC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826568E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826568E4: 392AD0E4  addi r9, r10, -0x2f1c
	ctx.r[9].s64 = ctx.r[10].s64 + -12060;
	// 826568E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826568EC: 390BB6DC  addi r8, r11, -0x4924
	ctx.r[8].s64 = ctx.r[11].s64 + -18724;
	// 826568F0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826568F4: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826568F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826568FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656900: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82656904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656908: 386A6C28  addi r3, r10, 0x6c28
	ctx.r[3].s64 = ctx.r[10].s64 + 27688;
	// 8265690C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82656910: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82656914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265691C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265692C: 4BE104F5  bl 0x82466e20
	ctx.lr = 0x82656930;
	sub_82466E20(ctx, base);
	// 82656930: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265693C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656940 size=108
    let mut pc: u32 = 0x82656940;
    'dispatch: loop {
        match pc {
            0x82656940 => {
    //   block [0x82656940..0x826569AC)
	// 82656940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265694C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656954: 38EBB70C  addi r7, r11, -0x48f4
	ctx.r[7].s64 = ctx.r[11].s64 + -18676;
	// 82656958: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265695C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 82656960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656964: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656968: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265696C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656970: 386A6C58  addi r3, r10, 0x6c58
	ctx.r[3].s64 = ctx.r[10].s64 + 27736;
	// 82656974: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265697C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265698C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656994: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656998: 4BE10489  bl 0x82466e20
	ctx.lr = 0x8265699C;
	sub_82466E20(ctx, base);
	// 8265699C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826569A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826569A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826569A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826569B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826569B0 size=108
    let mut pc: u32 = 0x826569B0;
    'dispatch: loop {
        match pc {
            0x826569B0 => {
    //   block [0x826569B0..0x82656A1C)
	// 826569B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826569B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826569B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826569BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826569C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826569C4: 38EBB73C  addi r7, r11, -0x48c4
	ctx.r[7].s64 = ctx.r[11].s64 + -18628;
	// 826569C8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826569CC: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 826569D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826569D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826569D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826569DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826569E0: 386A6C88  addi r3, r10, 0x6c88
	ctx.r[3].s64 = ctx.r[10].s64 + 27784;
	// 826569E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826569E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826569EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826569F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826569F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826569F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826569FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656A04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656A08: 4BE10419  bl 0x82466e20
	ctx.lr = 0x82656A0C;
	sub_82466E20(ctx, base);
	// 82656A0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656A20 size=108
    let mut pc: u32 = 0x82656A20;
    'dispatch: loop {
        match pc {
            0x82656A20 => {
    //   block [0x82656A20..0x82656A8C)
	// 82656A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656A2C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656A34: 38EBB754  addi r7, r11, -0x48ac
	ctx.r[7].s64 = ctx.r[11].s64 + -18604;
	// 82656A38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82656A3C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 82656A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656A44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656A48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656A50: 386A6CB8  addi r3, r10, 0x6cb8
	ctx.r[3].s64 = ctx.r[10].s64 + 27832;
	// 82656A54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656A74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656A78: 4BE103A9  bl 0x82466e20
	ctx.lr = 0x82656A7C;
	sub_82466E20(ctx, base);
	// 82656A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656A90 size=112
    let mut pc: u32 = 0x82656A90;
    'dispatch: loop {
        match pc {
            0x82656A90 => {
    //   block [0x82656A90..0x82656B00)
	// 82656A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656A9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656AA0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656AA4: 38AA6D18  addi r5, r10, 0x6d18
	ctx.r[5].s64 = ctx.r[10].s64 + 27928;
	// 82656AA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656AAC: 390BB784  addi r8, r11, -0x487c
	ctx.r[8].s64 = ctx.r[11].s64 + -18556;
	// 82656AB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82656AB4: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82656AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656ABC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656AC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82656AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656AC8: 386A6CE8  addi r3, r10, 0x6ce8
	ctx.r[3].s64 = ctx.r[10].s64 + 27880;
	// 82656ACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82656AD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656AD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656AE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656AE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656AE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656AEC: 4BE10335  bl 0x82466e20
	ctx.lr = 0x82656AF0;
	sub_82466E20(ctx, base);
	// 82656AF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


