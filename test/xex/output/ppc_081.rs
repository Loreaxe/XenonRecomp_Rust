pub fn sub_82612258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612258 size=112
    let mut pc: u32 = 0x82612258;
    'dispatch: loop {
        match pc {
            0x82612258 => {
    //   block [0x82612258..0x826122C8)
	// 82612258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261225C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612264: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612268: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261226C: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82612270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612274: 390B5DD0  addi r8, r11, 0x5dd0
	ctx.r[8].s64 = ctx.r[11].s64 + 24016;
	// 82612278: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261227C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 82612280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612284: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261228C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612290: 386A9D4C  addi r3, r10, -0x62b4
	ctx.r[3].s64 = ctx.r[10].s64 + -25268;
	// 82612294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261229C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826122A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826122A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826122A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826122AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826122B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826122B4: 4BE54B6D  bl 0x82466e20
	ctx.lr = 0x826122B8;
	sub_82466E20(ctx, base);
	// 826122B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826122BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826122C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826122C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826122C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826122C8 size=116
    let mut pc: u32 = 0x826122C8;
    'dispatch: loop {
        match pc {
            0x826122C8 => {
    //   block [0x826122C8..0x8261233C)
	// 826122C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826122CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826122D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826122D4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 826122D8: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826122DC: 390A5E00  addi r8, r10, 0x5e00
	ctx.r[8].s64 = ctx.r[10].s64 + 24064;
	// 826122E0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826122E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826122E8: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 826122EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826122F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826122F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826122F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826122FC: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 82612300: 396BF144  addi r11, r11, -0xebc
	ctx.r[11].s64 = ctx.r[11].s64 + -3772;
	// 82612304: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612308: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261230C: 386A9D7C  addi r3, r10, -0x6284
	ctx.r[3].s64 = ctx.r[10].s64 + -25220;
	// 82612310: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82612314: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612318: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261231C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612328: 4BE54AF9  bl 0x82466e20
	ctx.lr = 0x8261232C;
	sub_82466E20(ctx, base);
	// 8261232C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612340 size=112
    let mut pc: u32 = 0x82612340;
    'dispatch: loop {
        match pc {
            0x82612340 => {
    //   block [0x82612340..0x826123B0)
	// 82612340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261234C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612350: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612354: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82612358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261235C: 390B5EC0  addi r8, r11, 0x5ec0
	ctx.r[8].s64 = ctx.r[11].s64 + 24256;
	// 82612360: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82612364: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 82612368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261236C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612378: 386A9DAC  addi r3, r10, -0x6254
	ctx.r[3].s64 = ctx.r[10].s64 + -25172;
	// 8261237C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261238C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261239C: 4BE54A85  bl 0x82466e20
	ctx.lr = 0x826123A0;
	sub_82466E20(ctx, base);
	// 826123A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826123A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826123A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826123AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826123B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826123B0 size=108
    let mut pc: u32 = 0x826123B0;
    'dispatch: loop {
        match pc {
            0x826123B0 => {
    //   block [0x826123B0..0x8261241C)
	// 826123B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826123B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826123B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826123BC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826123C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826123C4: 38EB5ED8  addi r7, r11, 0x5ed8
	ctx.r[7].s64 = ctx.r[11].s64 + 24280;
	// 826123C8: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826123CC: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826123D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826123D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826123D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826123DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826123E0: 386A9DDC  addi r3, r10, -0x6224
	ctx.r[3].s64 = ctx.r[10].s64 + -25124;
	// 826123E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826123E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826123EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826123F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826123F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826123F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826123FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612404: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612408: 4BE54A19  bl 0x82466e20
	ctx.lr = 0x8261240C;
	sub_82466E20(ctx, base);
	// 8261240C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612420 size=116
    let mut pc: u32 = 0x82612420;
    'dispatch: loop {
        match pc {
            0x82612420 => {
    //   block [0x82612420..0x82612494)
	// 82612420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261242C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82612430: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82612434: 390A6010  addi r8, r10, 0x6010
	ctx.r[8].s64 = ctx.r[10].s64 + 24592;
	// 82612438: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261243C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82612440: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82612444: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612448: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261244C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612454: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 82612458: 396BF168  addi r11, r11, -0xe98
	ctx.r[11].s64 = ctx.r[11].s64 + -3736;
	// 8261245C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612460: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612464: 386A9E0C  addi r3, r10, -0x61f4
	ctx.r[3].s64 = ctx.r[10].s64 + -25076;
	// 82612468: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8261246C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612470: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82612474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261247C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612480: 4BE549A1  bl 0x82466e20
	ctx.lr = 0x82612484;
	sub_82466E20(ctx, base);
	// 82612484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261248C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612498 size=112
    let mut pc: u32 = 0x82612498;
    'dispatch: loop {
        match pc {
            0x82612498 => {
    //   block [0x82612498..0x82612508)
	// 82612498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261249C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826124A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826124A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826124A8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826124AC: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 826124B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826124B4: 390B60B8  addi r8, r11, 0x60b8
	ctx.r[8].s64 = ctx.r[11].s64 + 24760;
	// 826124B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826124BC: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826124C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826124C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826124C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826124CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826124D0: 386A9E3C  addi r3, r10, -0x61c4
	ctx.r[3].s64 = ctx.r[10].s64 + -25028;
	// 826124D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826124D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826124DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826124E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826124E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826124E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826124EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826124F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826124F4: 4BE5492D  bl 0x82466e20
	ctx.lr = 0x826124F8;
	sub_82466E20(ctx, base);
	// 826124F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826124FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612508 size=112
    let mut pc: u32 = 0x82612508;
    'dispatch: loop {
        match pc {
            0x82612508 => {
    //   block [0x82612508..0x82612578)
	// 82612508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261250C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612514: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612518: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261251C: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82612520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612524: 390B60D0  addi r8, r11, 0x60d0
	ctx.r[8].s64 = ctx.r[11].s64 + 24784;
	// 82612528: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 8261252C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 82612530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612534: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612538: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261253C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612540: 386A9E6C  addi r3, r10, -0x6194
	ctx.r[3].s64 = ctx.r[10].s64 + -24980;
	// 82612544: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261254C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261255C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612564: 4BE548BD  bl 0x82466e20
	ctx.lr = 0x82612568;
	sub_82466E20(ctx, base);
	// 82612568: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261256C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612578 size=112
    let mut pc: u32 = 0x82612578;
    'dispatch: loop {
        match pc {
            0x82612578 => {
    //   block [0x82612578..0x826125E8)
	// 82612578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261257C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612584: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612588: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261258C: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82612590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612594: 390B6208  addi r8, r11, 0x6208
	ctx.r[8].s64 = ctx.r[11].s64 + 25096;
	// 82612598: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261259C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826125A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826125A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826125A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826125AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826125B0: 386A9E9C  addi r3, r10, -0x6164
	ctx.r[3].s64 = ctx.r[10].s64 + -24932;
	// 826125B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826125B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826125BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826125C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826125C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826125C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826125CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826125D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826125D4: 4BE5484D  bl 0x82466e20
	ctx.lr = 0x826125D8;
	sub_82466E20(ctx, base);
	// 826125D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826125DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826125E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826125E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826125E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826125E8 size=116
    let mut pc: u32 = 0x826125E8;
    'dispatch: loop {
        match pc {
            0x826125E8 => {
    //   block [0x826125E8..0x8261265C)
	// 826125E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826125EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826125F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826125F4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826125F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826125FC: 390B6224  addi r8, r11, 0x6224
	ctx.r[8].s64 = ctx.r[11].s64 + 25124;
	// 82612600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612604: 392AF1A0  addi r9, r10, -0xe60
	ctx.r[9].s64 = ctx.r[10].s64 + -3680;
	// 82612608: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261260C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82612610: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82612614: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261261C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261262C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82612630: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 82612634: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82612638: 386B9ECC  addi r3, r11, -0x6134
	ctx.r[3].s64 = ctx.r[11].s64 + -24884;
	// 8261263C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82612640: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612648: 4BE547D9  bl 0x82466e20
	ctx.lr = 0x8261264C;
	sub_82466E20(ctx, base);
	// 8261264C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612660 size=100
    let mut pc: u32 = 0x82612660;
    'dispatch: loop {
        match pc {
            0x82612660 => {
    //   block [0x82612660..0x826126C4)
	// 82612660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261266C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612674: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82612678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261267C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612680: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 82612684: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261268C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612694: 386A9EFC  addi r3, r10, -0x6104
	ctx.r[3].s64 = ctx.r[10].s64 + -24836;
	// 82612698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261269C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826126A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826126A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826126A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826126AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826126B0: 4BE54771  bl 0x82466e20
	ctx.lr = 0x826126B4;
	sub_82466E20(ctx, base);
	// 826126B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826126B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826126BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826126C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826126C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826126C8 size=112
    let mut pc: u32 = 0x826126C8;
    'dispatch: loop {
        match pc {
            0x826126C8 => {
    //   block [0x826126C8..0x82612738)
	// 826126C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826126CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826126D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826126D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826126D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826126DC: 38AA9EFC  addi r5, r10, -0x6104
	ctx.r[5].s64 = ctx.r[10].s64 + -24836;
	// 826126E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826126E4: 390B6254  addi r8, r11, 0x6254
	ctx.r[8].s64 = ctx.r[11].s64 + 25172;
	// 826126E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826126EC: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826126F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826126F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826126F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826126FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612700: 386A9F2C  addi r3, r10, -0x60d4
	ctx.r[3].s64 = ctx.r[10].s64 + -24788;
	// 82612704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261270C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261271C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612724: 4BE546FD  bl 0x82466e20
	ctx.lr = 0x82612728;
	sub_82466E20(ctx, base);
	// 82612728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261272C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612738 size=112
    let mut pc: u32 = 0x82612738;
    'dispatch: loop {
        match pc {
            0x82612738 => {
    //   block [0x82612738..0x826127A8)
	// 82612738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261273C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612744: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612748: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261274C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82612750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612754: 390B6270  addi r8, r11, 0x6270
	ctx.r[8].s64 = ctx.r[11].s64 + 25200;
	// 82612758: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261275C: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 82612760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612764: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261276C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612770: 386A9F5C  addi r3, r10, -0x60a4
	ctx.r[3].s64 = ctx.r[10].s64 + -24740;
	// 82612774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261277C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261278C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612794: 4BE5468D  bl 0x82466e20
	ctx.lr = 0x82612798;
	sub_82466E20(ctx, base);
	// 82612798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261279C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826127A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826127A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826127A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826127A8 size=112
    let mut pc: u32 = 0x826127A8;
    'dispatch: loop {
        match pc {
            0x826127A8 => {
    //   block [0x826127A8..0x82612818)
	// 826127A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826127AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826127B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826127B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826127B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826127BC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826127C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826127C4: 390B6318  addi r8, r11, 0x6318
	ctx.r[8].s64 = ctx.r[11].s64 + 25368;
	// 826127C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826127CC: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826127D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826127D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826127D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826127DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826127E0: 386A9F8C  addi r3, r10, -0x6074
	ctx.r[3].s64 = ctx.r[10].s64 + -24692;
	// 826127E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826127E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826127EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826127F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826127F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826127F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826127FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612804: 4BE5461D  bl 0x82466e20
	ctx.lr = 0x82612808;
	sub_82466E20(ctx, base);
	// 82612808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261280C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612818 size=112
    let mut pc: u32 = 0x82612818;
    'dispatch: loop {
        match pc {
            0x82612818 => {
    //   block [0x82612818..0x82612888)
	// 82612818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261281C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612824: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612828: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261282C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82612830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612834: 390B6360  addi r8, r11, 0x6360
	ctx.r[8].s64 = ctx.r[11].s64 + 25440;
	// 82612838: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261283C: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 82612840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612844: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612848: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261284C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612850: 386A9FBC  addi r3, r10, -0x6044
	ctx.r[3].s64 = ctx.r[10].s64 + -24644;
	// 82612854: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261285C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261286C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612874: 4BE545AD  bl 0x82466e20
	ctx.lr = 0x82612878;
	sub_82466E20(ctx, base);
	// 82612878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261287C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612888 size=116
    let mut pc: u32 = 0x82612888;
    'dispatch: loop {
        match pc {
            0x82612888 => {
    //   block [0x82612888..0x826128FC)
	// 82612888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261288C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612894: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82612898: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8261289C: 390A6390  addi r8, r10, 0x6390
	ctx.r[8].s64 = ctx.r[10].s64 + 25488;
	// 826128A0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826128A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826128A8: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 826128AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826128B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826128B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826128B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826128BC: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826128C0: 396BF1B4  addi r11, r11, -0xe4c
	ctx.r[11].s64 = ctx.r[11].s64 + -3660;
	// 826128C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826128C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826128CC: 386A9FEC  addi r3, r10, -0x6014
	ctx.r[3].s64 = ctx.r[10].s64 + -24596;
	// 826128D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826128D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826128D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826128DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826128E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826128E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826128E8: 4BE54539  bl 0x82466e20
	ctx.lr = 0x826128EC;
	sub_82466E20(ctx, base);
	// 826128EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826128F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826128F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826128F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612900 size=100
    let mut pc: u32 = 0x82612900;
    'dispatch: loop {
        match pc {
            0x82612900 => {
    //   block [0x82612900..0x82612964)
	// 82612900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261290C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612914: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82612918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261291C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612920: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 82612924: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261292C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612934: 386AA01C  addi r3, r10, -0x5fe4
	ctx.r[3].s64 = ctx.r[10].s64 + -24548;
	// 82612938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261293C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612940: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82612944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612948: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261294C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612950: 4BE544D1  bl 0x82466e20
	ctx.lr = 0x82612954;
	sub_82466E20(ctx, base);
	// 82612954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261295C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612968 size=108
    let mut pc: u32 = 0x82612968;
    'dispatch: loop {
        match pc {
            0x82612968 => {
    //   block [0x82612968..0x826129D4)
	// 82612968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261296C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612974: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261297C: 38EB6450  addi r7, r11, 0x6450
	ctx.r[7].s64 = ctx.r[11].s64 + 25680;
	// 82612980: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82612984: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 82612988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261298C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612998: 386AA04C  addi r3, r10, -0x5fb4
	ctx.r[3].s64 = ctx.r[10].s64 + -24500;
	// 8261299C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826129A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826129A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826129A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826129AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826129B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826129B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826129B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826129BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826129C0: 4BE54461  bl 0x82466e20
	ctx.lr = 0x826129C4;
	sub_82466E20(ctx, base);
	// 826129C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826129C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826129CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826129D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826129D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826129D8 size=112
    let mut pc: u32 = 0x826129D8;
    'dispatch: loop {
        match pc {
            0x826129D8 => {
    //   block [0x826129D8..0x82612A48)
	// 826129D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826129DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826129E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826129E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826129E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826129EC: 38AAA01C  addi r5, r10, -0x5fe4
	ctx.r[5].s64 = ctx.r[10].s64 + -24548;
	// 826129F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826129F4: 390B6480  addi r8, r11, 0x6480
	ctx.r[8].s64 = ctx.r[11].s64 + 25728;
	// 826129F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826129FC: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 82612A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612A04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612A08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612A10: 386AA07C  addi r3, r10, -0x5f84
	ctx.r[3].s64 = ctx.r[10].s64 + -24452;
	// 82612A14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612A34: 4BE543ED  bl 0x82466e20
	ctx.lr = 0x82612A38;
	sub_82466E20(ctx, base);
	// 82612A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612A48 size=108
    let mut pc: u32 = 0x82612A48;
    'dispatch: loop {
        match pc {
            0x82612A48 => {
    //   block [0x82612A48..0x82612AB4)
	// 82612A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612A54: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612A5C: 38EB64B0  addi r7, r11, 0x64b0
	ctx.r[7].s64 = ctx.r[11].s64 + 25776;
	// 82612A60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82612A64: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 82612A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612A6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612A70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612A78: 386AA0AC  addi r3, r10, -0x5f54
	ctx.r[3].s64 = ctx.r[10].s64 + -24404;
	// 82612A7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82612A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612A9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612AA0: 4BE54381  bl 0x82466e20
	ctx.lr = 0x82612AA4;
	sub_82466E20(ctx, base);
	// 82612AA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612AB8 size=112
    let mut pc: u32 = 0x82612AB8;
    'dispatch: loop {
        match pc {
            0x82612AB8 => {
    //   block [0x82612AB8..0x82612B28)
	// 82612AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612AC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612AC8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612ACC: 38AAA01C  addi r5, r10, -0x5fe4
	ctx.r[5].s64 = ctx.r[10].s64 + -24548;
	// 82612AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612AD4: 390B64E0  addi r8, r11, 0x64e0
	ctx.r[8].s64 = ctx.r[11].s64 + 25824;
	// 82612AD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82612ADC: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 82612AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612AE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612AE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612AF0: 386AA0DC  addi r3, r10, -0x5f24
	ctx.r[3].s64 = ctx.r[10].s64 + -24356;
	// 82612AF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612B14: 4BE5430D  bl 0x82466e20
	ctx.lr = 0x82612B18;
	sub_82466E20(ctx, base);
	// 82612B18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612B28 size=108
    let mut pc: u32 = 0x82612B28;
    'dispatch: loop {
        match pc {
            0x82612B28 => {
    //   block [0x82612B28..0x82612B94)
	// 82612B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612B34: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612B38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612B3C: 38EB6528  addi r7, r11, 0x6528
	ctx.r[7].s64 = ctx.r[11].s64 + 25896;
	// 82612B40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82612B44: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 82612B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612B4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612B50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612B58: 386AA10C  addi r3, r10, -0x5ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -24308;
	// 82612B5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82612B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612B7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612B80: 4BE542A1  bl 0x82466e20
	ctx.lr = 0x82612B84;
	sub_82466E20(ctx, base);
	// 82612B84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612B98 size=112
    let mut pc: u32 = 0x82612B98;
    'dispatch: loop {
        match pc {
            0x82612B98 => {
    //   block [0x82612B98..0x82612C08)
	// 82612B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612BA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612BA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612BAC: 38AAA01C  addi r5, r10, -0x5fe4
	ctx.r[5].s64 = ctx.r[10].s64 + -24548;
	// 82612BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612BB4: 390B6558  addi r8, r11, 0x6558
	ctx.r[8].s64 = ctx.r[11].s64 + 25944;
	// 82612BB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82612BBC: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 82612BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612BC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612BC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612BD0: 386AA13C  addi r3, r10, -0x5ec4
	ctx.r[3].s64 = ctx.r[10].s64 + -24260;
	// 82612BD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612BF4: 4BE5422D  bl 0x82466e20
	ctx.lr = 0x82612BF8;
	sub_82466E20(ctx, base);
	// 82612BF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612C08 size=108
    let mut pc: u32 = 0x82612C08;
    'dispatch: loop {
        match pc {
            0x82612C08 => {
    //   block [0x82612C08..0x82612C74)
	// 82612C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612C14: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612C1C: 38EB65A0  addi r7, r11, 0x65a0
	ctx.r[7].s64 = ctx.r[11].s64 + 26016;
	// 82612C20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82612C24: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 82612C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612C2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612C30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612C38: 386AA16C  addi r3, r10, -0x5e94
	ctx.r[3].s64 = ctx.r[10].s64 + -24212;
	// 82612C3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82612C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612C5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612C60: 4BE541C1  bl 0x82466e20
	ctx.lr = 0x82612C64;
	sub_82466E20(ctx, base);
	// 82612C64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612C78 size=112
    let mut pc: u32 = 0x82612C78;
    'dispatch: loop {
        match pc {
            0x82612C78 => {
    //   block [0x82612C78..0x82612CE8)
	// 82612C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612C84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612C88: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612C8C: 38AAA01C  addi r5, r10, -0x5fe4
	ctx.r[5].s64 = ctx.r[10].s64 + -24548;
	// 82612C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612C94: 390B65D0  addi r8, r11, 0x65d0
	ctx.r[8].s64 = ctx.r[11].s64 + 26064;
	// 82612C98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82612C9C: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 82612CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612CA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612CB0: 386AA19C  addi r3, r10, -0x5e64
	ctx.r[3].s64 = ctx.r[10].s64 + -24164;
	// 82612CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612CD4: 4BE5414D  bl 0x82466e20
	ctx.lr = 0x82612CD8;
	sub_82466E20(ctx, base);
	// 82612CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612CE8 size=108
    let mut pc: u32 = 0x82612CE8;
    'dispatch: loop {
        match pc {
            0x82612CE8 => {
    //   block [0x82612CE8..0x82612D54)
	// 82612CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612CF4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612CF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612CFC: 38EB6618  addi r7, r11, 0x6618
	ctx.r[7].s64 = ctx.r[11].s64 + 26136;
	// 82612D00: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82612D04: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 82612D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612D0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612D10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612D18: 386AA1CC  addi r3, r10, -0x5e34
	ctx.r[3].s64 = ctx.r[10].s64 + -24116;
	// 82612D1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82612D20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612D3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612D40: 4BE540E1  bl 0x82466e20
	ctx.lr = 0x82612D44;
	sub_82466E20(ctx, base);
	// 82612D44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612D58 size=112
    let mut pc: u32 = 0x82612D58;
    'dispatch: loop {
        match pc {
            0x82612D58 => {
    //   block [0x82612D58..0x82612DC8)
	// 82612D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612D64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82612D68: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612D6C: 392AF290  addi r9, r10, -0xd70
	ctx.r[9].s64 = ctx.r[10].s64 + -3440;
	// 82612D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612D74: 390B6680  addi r8, r11, 0x6680
	ctx.r[8].s64 = ctx.r[11].s64 + 26240;
	// 82612D78: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82612D7C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 82612D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612D84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612D88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612D90: 386AA1FC  addi r3, r10, -0x5e04
	ctx.r[3].s64 = ctx.r[10].s64 + -24068;
	// 82612D94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82612D98: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82612D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612DB4: 4BE5406D  bl 0x82466e20
	ctx.lr = 0x82612DB8;
	sub_82466E20(ctx, base);
	// 82612DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612DC8 size=108
    let mut pc: u32 = 0x82612DC8;
    'dispatch: loop {
        match pc {
            0x82612DC8 => {
    //   block [0x82612DC8..0x82612E34)
	// 82612DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612DD4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612DDC: 38EB67A0  addi r7, r11, 0x67a0
	ctx.r[7].s64 = ctx.r[11].s64 + 26528;
	// 82612DE0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82612DE4: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 82612DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612DEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612DF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612DF8: 386AA22C  addi r3, r10, -0x5dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -24020;
	// 82612DFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82612E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612E20: 4BE54001  bl 0x82466e20
	ctx.lr = 0x82612E24;
	sub_82466E20(ctx, base);
	// 82612E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612E38 size=108
    let mut pc: u32 = 0x82612E38;
    'dispatch: loop {
        match pc {
            0x82612E38 => {
    //   block [0x82612E38..0x82612EA4)
	// 82612E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612E44: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612E4C: 38EB6800  addi r7, r11, 0x6800
	ctx.r[7].s64 = ctx.r[11].s64 + 26624;
	// 82612E50: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82612E54: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 82612E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612E5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612E60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612E68: 386AA25C  addi r3, r10, -0x5da4
	ctx.r[3].s64 = ctx.r[10].s64 + -23972;
	// 82612E6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82612E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612E90: 4BE53F91  bl 0x82466e20
	ctx.lr = 0x82612E94;
	sub_82466E20(ctx, base);
	// 82612E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82612EA8 size=24
    let mut pc: u32 = 0x82612EA8;
    'dispatch: loop {
        match pc {
            0x82612EA8 => {
    //   block [0x82612EA8..0x82612EC0)
	// 82612EA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612EAC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82612EB0: 394AD2B8  addi r10, r10, -0x2d48
	ctx.r[10].s64 = ctx.r[10].s64 + -11592;
	// 82612EB4: 816B626C  lwz r11, 0x626c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25196 as u32) ) } as u64;
	// 82612EB8: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 82612EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612EC0 size=116
    let mut pc: u32 = 0x82612EC0;
    'dispatch: loop {
        match pc {
            0x82612EC0 => {
    //   block [0x82612EC0..0x82612F34)
	// 82612EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612ECC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82612ED0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612ED4: 392BF1FC  addi r9, r11, -0xe04
	ctx.r[9].s64 = ctx.r[11].s64 + -3588;
	// 82612ED8: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 82612EDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612EE0: 38E900AC  addi r7, r9, 0xac
	ctx.r[7].s64 = ctx.r[9].s64 + 172;
	// 82612EE4: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82612EE8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82612EEC: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 82612EF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612EF4: 396BD2B8  addi r11, r11, -0x2d48
	ctx.r[11].s64 = ctx.r[11].s64 + -11592;
	// 82612EF8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82612EFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612F00: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82612F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612F08: 386AA28C  addi r3, r10, -0x5d74
	ctx.r[3].s64 = ctx.r[10].s64 + -23924;
	// 82612F0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82612F10: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82612F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612F18: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82612F1C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82612F20: 4BE53F01  bl 0x82466e20
	ctx.lr = 0x82612F24;
	sub_82466E20(ctx, base);
	// 82612F24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612F38 size=100
    let mut pc: u32 = 0x82612F38;
    'dispatch: loop {
        match pc {
            0x82612F38 => {
    //   block [0x82612F38..0x82612F9C)
	// 82612F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612F44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612F4C: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 82612F50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612F58: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 82612F5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612F6C: 386AA2BC  addi r3, r10, -0x5d44
	ctx.r[3].s64 = ctx.r[10].s64 + -23876;
	// 82612F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612F74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612F78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82612F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612F80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82612F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612F88: 4BE53E99  bl 0x82466e20
	ctx.lr = 0x82612F8C;
	sub_82466E20(ctx, base);
	// 82612F8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82612FA0 size=24
    let mut pc: u32 = 0x82612FA0;
    'dispatch: loop {
        match pc {
            0x82612FA0 => {
    //   block [0x82612FA0..0x82612FB8)
	// 82612FA0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612FA4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82612FA8: 394AD528  addi r10, r10, -0x2ad8
	ctx.r[10].s64 = ctx.r[10].s64 + -10968;
	// 82612FAC: 816B68A8  lwz r11, 0x68a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26792 as u32) ) } as u64;
	// 82612FB0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82612FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612FB8 size=116
    let mut pc: u32 = 0x82612FB8;
    'dispatch: loop {
        match pc {
            0x82612FB8 => {
    //   block [0x82612FB8..0x8261302C)
	// 82612FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612FC4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82612FC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82612FCC: 390BD528  addi r8, r11, -0x2ad8
	ctx.r[8].s64 = ctx.r[11].s64 + -10968;
	// 82612FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612FD4: 392AF364  addi r9, r10, -0xc9c
	ctx.r[9].s64 = ctx.r[10].s64 + -3228;
	// 82612FD8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612FDC: 38E0000F  li r7, 0xf
	ctx.r[7].s64 = 15;
	// 82612FE0: 38AAA2BC  addi r5, r10, -0x5d44
	ctx.r[5].s64 = ctx.r[10].s64 + -23876;
	// 82612FE4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612FE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612FEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612FF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612FF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612FFC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82613000: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 82613004: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82613008: 386BA2EC  addi r3, r11, -0x5d14
	ctx.r[3].s64 = ctx.r[11].s64 + -23828;
	// 8261300C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82613010: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613018: 4BE53E09  bl 0x82466e20
	ctx.lr = 0x8261301C;
	sub_82466E20(ctx, base);
	// 8261301C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613030 size=112
    let mut pc: u32 = 0x82613030;
    'dispatch: loop {
        match pc {
            0x82613030 => {
    //   block [0x82613030..0x826130A0)
	// 82613030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261303C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613040: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613044: 38AAA2BC  addi r5, r10, -0x5d44
	ctx.r[5].s64 = ctx.r[10].s64 + -23876;
	// 82613048: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261304C: 390B68B0  addi r8, r11, 0x68b0
	ctx.r[8].s64 = ctx.r[11].s64 + 26800;
	// 82613050: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82613054: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 82613058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261305C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613068: 386AA31C  addi r3, r10, -0x5ce4
	ctx.r[3].s64 = ctx.r[10].s64 + -23780;
	// 8261306C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82613070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261307C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261308C: 4BE53D95  bl 0x82466e20
	ctx.lr = 0x82613090;
	sub_82466E20(ctx, base);
	// 82613090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261309C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826130A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826130A0 size=112
    let mut pc: u32 = 0x826130A0;
    'dispatch: loop {
        match pc {
            0x826130A0 => {
    //   block [0x826130A0..0x82613110)
	// 826130A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826130A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826130A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826130AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826130B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826130B4: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826130B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826130BC: 390B6988  addi r8, r11, 0x6988
	ctx.r[8].s64 = ctx.r[11].s64 + 27016;
	// 826130C0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826130C4: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 826130C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826130CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826130D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826130D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826130D8: 386AA34C  addi r3, r10, -0x5cb4
	ctx.r[3].s64 = ctx.r[10].s64 + -23732;
	// 826130DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826130E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826130E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826130E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826130EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826130F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826130F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826130F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826130FC: 4BE53D25  bl 0x82466e20
	ctx.lr = 0x82613100;
	sub_82466E20(ctx, base);
	// 82613100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261310C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613110 size=108
    let mut pc: u32 = 0x82613110;
    'dispatch: loop {
        match pc {
            0x82613110 => {
    //   block [0x82613110..0x8261317C)
	// 82613110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261311C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613124: 38EB6A60  addi r7, r11, 0x6a60
	ctx.r[7].s64 = ctx.r[11].s64 + 27232;
	// 82613128: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8261312C: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 82613130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613134: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261313C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613140: 386AA37C  addi r3, r10, -0x5c84
	ctx.r[3].s64 = ctx.r[10].s64 + -23684;
	// 82613144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261314C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261315C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613168: 4BE53CB9  bl 0x82466e20
	ctx.lr = 0x8261316C;
	sub_82466E20(ctx, base);
	// 8261316C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613180 size=108
    let mut pc: u32 = 0x82613180;
    'dispatch: loop {
        match pc {
            0x82613180 => {
    //   block [0x82613180..0x826131EC)
	// 82613180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261318C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613194: 38EB6AD8  addi r7, r11, 0x6ad8
	ctx.r[7].s64 = ctx.r[11].s64 + 27352;
	// 82613198: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261319C: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 826131A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826131A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826131A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826131AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826131B0: 386AA3AC  addi r3, r10, -0x5c54
	ctx.r[3].s64 = ctx.r[10].s64 + -23636;
	// 826131B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826131B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826131BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826131C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826131C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826131C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826131CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826131D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826131D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826131D8: 4BE53C49  bl 0x82466e20
	ctx.lr = 0x826131DC;
	sub_82466E20(ctx, base);
	// 826131DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826131E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826131E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826131E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826131F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826131F0 size=112
    let mut pc: u32 = 0x826131F0;
    'dispatch: loop {
        match pc {
            0x826131F0 => {
    //   block [0x826131F0..0x82613260)
	// 826131F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826131F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826131F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826131FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613200: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613204: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82613208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261320C: 390B6B20  addi r8, r11, 0x6b20
	ctx.r[8].s64 = ctx.r[11].s64 + 27424;
	// 82613210: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82613214: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 82613218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261321C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613228: 386AA3DC  addi r3, r10, -0x5c24
	ctx.r[3].s64 = ctx.r[10].s64 + -23588;
	// 8261322C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82613230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261323C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261324C: 4BE53BD5  bl 0x82466e20
	ctx.lr = 0x82613250;
	sub_82466E20(ctx, base);
	// 82613250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261325C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613260 size=108
    let mut pc: u32 = 0x82613260;
    'dispatch: loop {
        match pc {
            0x82613260 => {
    //   block [0x82613260..0x826132CC)
	// 82613260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261326C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613274: 38EB6D00  addi r7, r11, 0x6d00
	ctx.r[7].s64 = ctx.r[11].s64 + 27904;
	// 82613278: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261327C: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 82613280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613284: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261328C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613290: 386AA40C  addi r3, r10, -0x5bf4
	ctx.r[3].s64 = ctx.r[10].s64 + -23540;
	// 82613294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261329C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826132A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826132A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826132A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826132AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826132B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826132B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826132B8: 4BE53B69  bl 0x82466e20
	ctx.lr = 0x826132BC;
	sub_82466E20(ctx, base);
	// 826132BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826132C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826132C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826132C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826132D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826132D0 size=24
    let mut pc: u32 = 0x826132D0;
    'dispatch: loop {
        match pc {
            0x826132D0 => {
    //   block [0x826132D0..0x826132E8)
	// 826132D0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826132D4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826132D8: 394AD690  addi r10, r10, -0x2970
	ctx.r[10].s64 = ctx.r[10].s64 + -10608;
	// 826132DC: 816B68AC  lwz r11, 0x68ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26796 as u32) ) } as u64;
	// 826132E0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826132E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826132E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826132E8 size=112
    let mut pc: u32 = 0x826132E8;
    'dispatch: loop {
        match pc {
            0x826132E8 => {
    //   block [0x826132E8..0x82613358)
	// 826132E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826132EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826132F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826132F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826132F8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826132FC: 392AF3BC  addi r9, r10, -0xc44
	ctx.r[9].s64 = ctx.r[10].s64 + -3140;
	// 82613300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613304: 390BD690  addi r8, r11, -0x2970
	ctx.r[8].s64 = ctx.r[11].s64 + -10608;
	// 82613308: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8261330C: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 82613310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613314: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261331C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613320: 386AA43C  addi r3, r10, -0x5bc4
	ctx.r[3].s64 = ctx.r[10].s64 + -23492;
	// 82613324: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82613328: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261332C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261333C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613344: 4BE53ADD  bl 0x82466e20
	ctx.lr = 0x82613348;
	sub_82466E20(ctx, base);
	// 82613348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261334C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613358 size=112
    let mut pc: u32 = 0x82613358;
    'dispatch: loop {
        match pc {
            0x82613358 => {
    //   block [0x82613358..0x826133C8)
	// 82613358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261335C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613364: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82613368: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8261336C: 38EA6D18  addi r7, r10, 0x6d18
	ctx.r[7].s64 = ctx.r[10].s64 + 27928;
	// 82613370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613374: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82613378: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 8261337C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613380: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613384: 396BF3D0  addi r11, r11, -0xc30
	ctx.r[11].s64 = ctx.r[11].s64 + -3120;
	// 82613388: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261338C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613390: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613394: 386AA46C  addi r3, r10, -0x5b94
	ctx.r[3].s64 = ctx.r[10].s64 + -23444;
	// 82613398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261339C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826133A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826133A4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826133A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826133AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826133B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826133B4: 4BE53A6D  bl 0x82466e20
	ctx.lr = 0x826133B8;
	sub_82466E20(ctx, base);
	// 826133B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826133BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826133C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826133C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826133C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826133C8 size=112
    let mut pc: u32 = 0x826133C8;
    'dispatch: loop {
        match pc {
            0x826133C8 => {
    //   block [0x826133C8..0x82613438)
	// 826133C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826133CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826133D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826133D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826133D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826133DC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826133E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826133E4: 390B6DA8  addi r8, r11, 0x6da8
	ctx.r[8].s64 = ctx.r[11].s64 + 28072;
	// 826133E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826133EC: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 826133F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826133F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826133F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826133FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613400: 386AA49C  addi r3, r10, -0x5b64
	ctx.r[3].s64 = ctx.r[10].s64 + -23396;
	// 82613404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82613408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261340C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261341C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613424: 4BE539FD  bl 0x82466e20
	ctx.lr = 0x82613428;
	sub_82466E20(ctx, base);
	// 82613428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261342C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613438 size=108
    let mut pc: u32 = 0x82613438;
    'dispatch: loop {
        match pc {
            0x82613438 => {
    //   block [0x82613438..0x826134A4)
	// 82613438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261343C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613444: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261344C: 38EB6DC8  addi r7, r11, 0x6dc8
	ctx.r[7].s64 = ctx.r[11].s64 + 28104;
	// 82613450: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82613454: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 82613458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261345C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613460: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613468: 386AA4CC  addi r3, r10, -0x5b34
	ctx.r[3].s64 = ctx.r[10].s64 + -23348;
	// 8261346C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261347C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261348C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613490: 4BE53991  bl 0x82466e20
	ctx.lr = 0x82613494;
	sub_82466E20(ctx, base);
	// 82613494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261349C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826134A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826134A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826134A8 size=108
    let mut pc: u32 = 0x826134A8;
    'dispatch: loop {
        match pc {
            0x826134A8 => {
    //   block [0x826134A8..0x82613514)
	// 826134A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826134AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826134B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826134B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826134B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826134BC: 38EB6E28  addi r7, r11, 0x6e28
	ctx.r[7].s64 = ctx.r[11].s64 + 28200;
	// 826134C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826134C4: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826134C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826134CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826134D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826134D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826134D8: 386AA4FC  addi r3, r10, -0x5b04
	ctx.r[3].s64 = ctx.r[10].s64 + -23300;
	// 826134DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826134E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826134E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826134E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826134EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826134F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826134F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826134F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826134FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613500: 4BE53921  bl 0x82466e20
	ctx.lr = 0x82613504;
	sub_82466E20(ctx, base);
	// 82613504: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261350C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613518 size=116
    let mut pc: u32 = 0x82613518;
    'dispatch: loop {
        match pc {
            0x82613518 => {
    //   block [0x82613518..0x8261358C)
	// 82613518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261351C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613524: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613528: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261352C: 390B6E58  addi r8, r11, 0x6e58
	ctx.r[8].s64 = ctx.r[11].s64 + 28248;
	// 82613530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613534: 392AF404  addi r9, r10, -0xbfc
	ctx.r[9].s64 = ctx.r[10].s64 + -3068;
	// 82613538: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261353C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82613540: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82613544: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261354C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261355C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82613560: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 82613564: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82613568: 386BA52C  addi r3, r11, -0x5ad4
	ctx.r[3].s64 = ctx.r[11].s64 + -23252;
	// 8261356C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82613570: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613578: 4BE538A9  bl 0x82466e20
	ctx.lr = 0x8261357C;
	sub_82466E20(ctx, base);
	// 8261357C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613590 size=96
    let mut pc: u32 = 0x82613590;
    'dispatch: loop {
        match pc {
            0x82613590 => {
    //   block [0x82613590..0x826135F0)
	// 82613590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261359C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826135A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826135A4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826135A8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826135AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826135B0: 386AA55C  addi r3, r10, -0x5aa4
	ctx.r[3].s64 = ctx.r[10].s64 + -23204;
	// 826135B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826135B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826135BC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826135C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826135C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826135C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826135CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826135D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826135D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826135D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826135DC: 4BE53845  bl 0x82466e20
	ctx.lr = 0x826135E0;
	sub_82466E20(ctx, base);
	// 826135E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826135E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826135E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826135EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826135F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826135F0 size=112
    let mut pc: u32 = 0x826135F0;
    'dispatch: loop {
        match pc {
            0x826135F0 => {
    //   block [0x826135F0..0x82613660)
	// 826135F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826135F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826135F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826135FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613600: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613604: 38AAA55C  addi r5, r10, -0x5aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -23204;
	// 82613608: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261360C: 390B6E70  addi r8, r11, 0x6e70
	ctx.r[8].s64 = ctx.r[11].s64 + 28272;
	// 82613610: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82613614: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82613618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261361C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613620: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613628: 386AA58C  addi r3, r10, -0x5a74
	ctx.r[3].s64 = ctx.r[10].s64 + -23156;
	// 8261362C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82613630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261363C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261364C: 4BE537D5  bl 0x82466e20
	ctx.lr = 0x82613650;
	sub_82466E20(ctx, base);
	// 82613650: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261365C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613660 size=112
    let mut pc: u32 = 0x82613660;
    'dispatch: loop {
        match pc {
            0x82613660 => {
    //   block [0x82613660..0x826136D0)
	// 82613660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261366C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82613670: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613674: 392AF420  addi r9, r10, -0xbe0
	ctx.r[9].s64 = ctx.r[10].s64 + -3040;
	// 82613678: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261367C: 390B6EA0  addi r8, r11, 0x6ea0
	ctx.r[8].s64 = ctx.r[11].s64 + 28320;
	// 82613680: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82613684: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 82613688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261368C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613690: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613698: 386AA5BC  addi r3, r10, -0x5a44
	ctx.r[3].s64 = ctx.r[10].s64 + -23108;
	// 8261369C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826136A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826136A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826136A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826136AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826136B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826136B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826136B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826136BC: 4BE53765  bl 0x82466e20
	ctx.lr = 0x826136C0;
	sub_82466E20(ctx, base);
	// 826136C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826136C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826136C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826136CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826136D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826136D0 size=108
    let mut pc: u32 = 0x826136D0;
    'dispatch: loop {
        match pc {
            0x826136D0 => {
    //   block [0x826136D0..0x8261373C)
	// 826136D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826136D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826136D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826136DC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826136E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826136E4: 38EB6F48  addi r7, r11, 0x6f48
	ctx.r[7].s64 = ctx.r[11].s64 + 28488;
	// 826136E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826136EC: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826136F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826136F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826136F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826136FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613700: 386AA5EC  addi r3, r10, -0x5a14
	ctx.r[3].s64 = ctx.r[10].s64 + -23060;
	// 82613704: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261370C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261371C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613728: 4BE536F9  bl 0x82466e20
	ctx.lr = 0x8261372C;
	sub_82466E20(ctx, base);
	// 8261372C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613740 size=108
    let mut pc: u32 = 0x82613740;
    'dispatch: loop {
        match pc {
            0x82613740 => {
    //   block [0x82613740..0x826137AC)
	// 82613740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261374C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613750: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82613754: 38EB6F78  addi r7, r11, 0x6f78
	ctx.r[7].s64 = ctx.r[11].s64 + 28536;
	// 82613758: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261375C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82613760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613764: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261376C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613770: 386AA61C  addi r3, r10, -0x59e4
	ctx.r[3].s64 = ctx.r[10].s64 + -23012;
	// 82613774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261377C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261378C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613798: 4BE53689  bl 0x82466e20
	ctx.lr = 0x8261379C;
	sub_82466E20(ctx, base);
	// 8261379C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826137A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826137A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826137A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826137B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826137B0 size=28
    let mut pc: u32 = 0x826137B0;
    'dispatch: loop {
        match pc {
            0x826137B0 => {
    //   block [0x826137B0..0x826137CC)
	// 826137B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826137B4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826137B8: 394AD6C0  addi r10, r10, -0x2940
	ctx.r[10].s64 = ctx.r[10].s64 + -10560;
	// 826137BC: 816B6FA8  lwz r11, 0x6fa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28584 as u32) ) } as u64;
	// 826137C0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826137C4: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826137C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826137D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826137D0 size=112
    let mut pc: u32 = 0x826137D0;
    'dispatch: loop {
        match pc {
            0x826137D0 => {
    //   block [0x826137D0..0x82613840)
	// 826137D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826137D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826137D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826137DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826137E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826137E4: 392AF5D0  addi r9, r10, -0xa30
	ctx.r[9].s64 = ctx.r[10].s64 + -2608;
	// 826137E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826137EC: 390BD6C0  addi r8, r11, -0x2940
	ctx.r[8].s64 = ctx.r[11].s64 + -10560;
	// 826137F0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826137F4: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826137F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826137FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613808: 386AA64C  addi r3, r10, -0x59b4
	ctx.r[3].s64 = ctx.r[10].s64 + -22964;
	// 8261380C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82613810: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82613814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261381C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261382C: 4BE535F5  bl 0x82466e20
	ctx.lr = 0x82613830;
	sub_82466E20(ctx, base);
	// 82613830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261383C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613840 size=108
    let mut pc: u32 = 0x82613840;
    'dispatch: loop {
        match pc {
            0x82613840 => {
    //   block [0x82613840..0x826138AC)
	// 82613840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261384C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613850: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82613854: 38EB6FB4  addi r7, r11, 0x6fb4
	ctx.r[7].s64 = ctx.r[11].s64 + 28596;
	// 82613858: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261385C: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 82613860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613864: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261386C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613870: 386AA67C  addi r3, r10, -0x5984
	ctx.r[3].s64 = ctx.r[10].s64 + -22916;
	// 82613874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261387C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261388C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613898: 4BE53589  bl 0x82466e20
	ctx.lr = 0x8261389C;
	sub_82466E20(ctx, base);
	// 8261389C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826138A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826138A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826138A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826138B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826138B0 size=108
    let mut pc: u32 = 0x826138B0;
    'dispatch: loop {
        match pc {
            0x826138B0 => {
    //   block [0x826138B0..0x8261391C)
	// 826138B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826138B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826138B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826138BC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826138C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826138C4: 38EB6FE4  addi r7, r11, 0x6fe4
	ctx.r[7].s64 = ctx.r[11].s64 + 28644;
	// 826138C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826138CC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826138D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826138D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826138D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826138DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826138E0: 386AA6AC  addi r3, r10, -0x5954
	ctx.r[3].s64 = ctx.r[10].s64 + -22868;
	// 826138E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826138E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826138EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826138F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826138F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826138F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826138FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613908: 4BE53519  bl 0x82466e20
	ctx.lr = 0x8261390C;
	sub_82466E20(ctx, base);
	// 8261390C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613920 size=108
    let mut pc: u32 = 0x82613920;
    'dispatch: loop {
        match pc {
            0x82613920 => {
    //   block [0x82613920..0x8261398C)
	// 82613920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261392C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613930: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82613934: 38EB7014  addi r7, r11, 0x7014
	ctx.r[7].s64 = ctx.r[11].s64 + 28692;
	// 82613938: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261393C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 82613940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613944: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613948: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261394C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613950: 386AA6DC  addi r3, r10, -0x5924
	ctx.r[3].s64 = ctx.r[10].s64 + -22820;
	// 82613954: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261395C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261396C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613978: 4BE534A9  bl 0x82466e20
	ctx.lr = 0x8261397C;
	sub_82466E20(ctx, base);
	// 8261397C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82613990 size=24
    let mut pc: u32 = 0x82613990;
    'dispatch: loop {
        match pc {
            0x82613990 => {
    //   block [0x82613990..0x826139A8)
	// 82613990: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613994: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82613998: 394AD780  addi r10, r10, -0x2880
	ctx.r[10].s64 = ctx.r[10].s64 + -10368;
	// 8261399C: 816B702C  lwz r11, 0x702c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28716 as u32) ) } as u64;
	// 826139A0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826139A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826139A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826139A8 size=112
    let mut pc: u32 = 0x826139A8;
    'dispatch: loop {
        match pc {
            0x826139A8 => {
    //   block [0x826139A8..0x82613A18)
	// 826139A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826139AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826139B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826139B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826139B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826139BC: 392AF624  addi r9, r10, -0x9dc
	ctx.r[9].s64 = ctx.r[10].s64 + -2524;
	// 826139C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826139C4: 390BD780  addi r8, r11, -0x2880
	ctx.r[8].s64 = ctx.r[11].s64 + -10368;
	// 826139C8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826139CC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826139D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826139D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826139D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826139DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826139E0: 386AA70C  addi r3, r10, -0x58f4
	ctx.r[3].s64 = ctx.r[10].s64 + -22772;
	// 826139E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826139E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826139EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826139F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826139F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826139F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826139FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613A04: 4BE5341D  bl 0x82466e20
	ctx.lr = 0x82613A08;
	sub_82466E20(ctx, base);
	// 82613A08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613A18 size=112
    let mut pc: u32 = 0x82613A18;
    'dispatch: loop {
        match pc {
            0x82613A18 => {
    //   block [0x82613A18..0x82613A88)
	// 82613A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613A24: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82613A28: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613A2C: 392AF660  addi r9, r10, -0x9a0
	ctx.r[9].s64 = ctx.r[10].s64 + -2464;
	// 82613A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613A34: 390B7038  addi r8, r11, 0x7038
	ctx.r[8].s64 = ctx.r[11].s64 + 28728;
	// 82613A38: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82613A3C: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 82613A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613A44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613A48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613A50: 386AA73C  addi r3, r10, -0x58c4
	ctx.r[3].s64 = ctx.r[10].s64 + -22724;
	// 82613A54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82613A58: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82613A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613A6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613A74: 4BE533AD  bl 0x82466e20
	ctx.lr = 0x82613A78;
	sub_82466E20(ctx, base);
	// 82613A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613A88 size=108
    let mut pc: u32 = 0x82613A88;
    'dispatch: loop {
        match pc {
            0x82613A88 => {
    //   block [0x82613A88..0x82613AF4)
	// 82613A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613A94: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613A98: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82613A9C: 38EB7080  addi r7, r11, 0x7080
	ctx.r[7].s64 = ctx.r[11].s64 + 28800;
	// 82613AA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82613AA4: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 82613AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613AAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613AB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613AB8: 386AA76C  addi r3, r10, -0x5894
	ctx.r[3].s64 = ctx.r[10].s64 + -22676;
	// 82613ABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613AE0: 4BE53341  bl 0x82466e20
	ctx.lr = 0x82613AE4;
	sub_82466E20(ctx, base);
	// 82613AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613AF8 size=108
    let mut pc: u32 = 0x82613AF8;
    'dispatch: loop {
        match pc {
            0x82613AF8 => {
    //   block [0x82613AF8..0x82613B64)
	// 82613AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613B04: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613B08: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82613B0C: 38EB70B0  addi r7, r11, 0x70b0
	ctx.r[7].s64 = ctx.r[11].s64 + 28848;
	// 82613B10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82613B14: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 82613B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613B1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613B20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613B28: 386AA79C  addi r3, r10, -0x5864
	ctx.r[3].s64 = ctx.r[10].s64 + -22628;
	// 82613B2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613B4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613B50: 4BE532D1  bl 0x82466e20
	ctx.lr = 0x82613B54;
	sub_82466E20(ctx, base);
	// 82613B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613B68 size=112
    let mut pc: u32 = 0x82613B68;
    'dispatch: loop {
        match pc {
            0x82613B68 => {
    //   block [0x82613B68..0x82613BD8)
	// 82613B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613B74: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82613B78: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613B7C: 392AF6A0  addi r9, r10, -0x960
	ctx.r[9].s64 = ctx.r[10].s64 + -2400;
	// 82613B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613B84: 390B70E8  addi r8, r11, 0x70e8
	ctx.r[8].s64 = ctx.r[11].s64 + 28904;
	// 82613B88: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82613B8C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 82613B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613B94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613B98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613BA0: 386AA7CC  addi r3, r10, -0x5834
	ctx.r[3].s64 = ctx.r[10].s64 + -22580;
	// 82613BA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82613BA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82613BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613BBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613BC4: 4BE5325D  bl 0x82466e20
	ctx.lr = 0x82613BC8;
	sub_82466E20(ctx, base);
	// 82613BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613BD8 size=108
    let mut pc: u32 = 0x82613BD8;
    'dispatch: loop {
        match pc {
            0x82613BD8 => {
    //   block [0x82613BD8..0x82613C44)
	// 82613BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613BE4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613BE8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82613BEC: 38EB7160  addi r7, r11, 0x7160
	ctx.r[7].s64 = ctx.r[11].s64 + 29024;
	// 82613BF0: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82613BF4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 82613BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613BFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613C00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613C04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613C08: 386AA7FC  addi r3, r10, -0x5804
	ctx.r[3].s64 = ctx.r[10].s64 + -22532;
	// 82613C0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613C10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613C2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613C30: 4BE531F1  bl 0x82466e20
	ctx.lr = 0x82613C34;
	sub_82466E20(ctx, base);
	// 82613C34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613C48 size=108
    let mut pc: u32 = 0x82613C48;
    'dispatch: loop {
        match pc {
            0x82613C48 => {
    //   block [0x82613C48..0x82613CB4)
	// 82613C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613C54: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613C58: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82613C5C: 38EB7268  addi r7, r11, 0x7268
	ctx.r[7].s64 = ctx.r[11].s64 + 29288;
	// 82613C60: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82613C64: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 82613C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613C6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613C70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613C78: 386AA82C  addi r3, r10, -0x57d4
	ctx.r[3].s64 = ctx.r[10].s64 + -22484;
	// 82613C7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613CA0: 4BE53181  bl 0x82466e20
	ctx.lr = 0x82613CA4;
	sub_82466E20(ctx, base);
	// 82613CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613CB8 size=108
    let mut pc: u32 = 0x82613CB8;
    'dispatch: loop {
        match pc {
            0x82613CB8 => {
    //   block [0x82613CB8..0x82613D24)
	// 82613CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613CC4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613CC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613CCC: 38EB7280  addi r7, r11, 0x7280
	ctx.r[7].s64 = ctx.r[11].s64 + 29312;
	// 82613CD0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82613CD4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82613CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613CDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613CE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613CE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613CE8: 386AA85C  addi r3, r10, -0x57a4
	ctx.r[3].s64 = ctx.r[10].s64 + -22436;
	// 82613CEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613CF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613CF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613D00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613D04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613D08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613D0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613D10: 4BE53111  bl 0x82466e20
	ctx.lr = 0x82613D14;
	sub_82466E20(ctx, base);
	// 82613D14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613D18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613D1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82613D28 size=24
    let mut pc: u32 = 0x82613D28;
    'dispatch: loop {
        match pc {
            0x82613D28 => {
    //   block [0x82613D28..0x82613D40)
	// 82613D28: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613D2C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82613D30: 394AD858  addi r10, r10, -0x27a8
	ctx.r[10].s64 = ctx.r[10].s64 + -10152;
	// 82613D34: 816B70E4  lwz r11, 0x70e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28900 as u32) ) } as u64;
	// 82613D38: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82613D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613D40 size=108
    let mut pc: u32 = 0x82613D40;
    'dispatch: loop {
        match pc {
            0x82613D40 => {
    //   block [0x82613D40..0x82613DAC)
	// 82613D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613D4C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82613D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613D54: 38EBD858  addi r7, r11, -0x27a8
	ctx.r[7].s64 = ctx.r[11].s64 + -10152;
	// 82613D58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82613D5C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 82613D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613D64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613D68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613D70: 386AA88C  addi r3, r10, -0x5774
	ctx.r[3].s64 = ctx.r[10].s64 + -22388;
	// 82613D74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613D94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613D98: 4BE53089  bl 0x82466e20
	ctx.lr = 0x82613D9C;
	sub_82466E20(ctx, base);
	// 82613D9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82613DB0 size=24
    let mut pc: u32 = 0x82613DB0;
    'dispatch: loop {
        match pc {
            0x82613DB0 => {
    //   block [0x82613DB0..0x82613DC8)
	// 82613DB0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613DB4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82613DB8: 394AD888  addi r10, r10, -0x2778
	ctx.r[10].s64 = ctx.r[10].s64 + -10104;
	// 82613DBC: 816B70E4  lwz r11, 0x70e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28900 as u32) ) } as u64;
	// 82613DC0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82613DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613DC8 size=108
    let mut pc: u32 = 0x82613DC8;
    'dispatch: loop {
        match pc {
            0x82613DC8 => {
    //   block [0x82613DC8..0x82613E34)
	// 82613DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613DD4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82613DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613DDC: 38EBD888  addi r7, r11, -0x2778
	ctx.r[7].s64 = ctx.r[11].s64 + -10104;
	// 82613DE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82613DE4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 82613DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613DEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613DF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613DF8: 386AA8BC  addi r3, r10, -0x5744
	ctx.r[3].s64 = ctx.r[10].s64 + -22340;
	// 82613DFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613E20: 4BE53001  bl 0x82466e20
	ctx.lr = 0x82613E24;
	sub_82466E20(ctx, base);
	// 82613E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613E38 size=108
    let mut pc: u32 = 0x82613E38;
    'dispatch: loop {
        match pc {
            0x82613E38 => {
    //   block [0x82613E38..0x82613EA4)
	// 82613E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613E44: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613E4C: 38EB72F8  addi r7, r11, 0x72f8
	ctx.r[7].s64 = ctx.r[11].s64 + 29432;
	// 82613E50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82613E54: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 82613E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613E5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613E60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613E68: 386AA8EC  addi r3, r10, -0x5714
	ctx.r[3].s64 = ctx.r[10].s64 + -22292;
	// 82613E6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613E90: 4BE52F91  bl 0x82466e20
	ctx.lr = 0x82613E94;
	sub_82466E20(ctx, base);
	// 82613E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82613EA8 size=24
    let mut pc: u32 = 0x82613EA8;
    'dispatch: loop {
        match pc {
            0x82613EA8 => {
    //   block [0x82613EA8..0x82613EC0)
	// 82613EA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613EAC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82613EB0: 394AD8B8  addi r10, r10, -0x2748
	ctx.r[10].s64 = ctx.r[10].s64 + -10056;
	// 82613EB4: 816B70E4  lwz r11, 0x70e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28900 as u32) ) } as u64;
	// 82613EB8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82613EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613EC0 size=108
    let mut pc: u32 = 0x82613EC0;
    'dispatch: loop {
        match pc {
            0x82613EC0 => {
    //   block [0x82613EC0..0x82613F2C)
	// 82613EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613ECC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82613ED0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613ED4: 38EBD8B8  addi r7, r11, -0x2748
	ctx.r[7].s64 = ctx.r[11].s64 + -10056;
	// 82613ED8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82613EDC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 82613EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613EE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613EE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613EEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613EF0: 386AA91C  addi r3, r10, -0x56e4
	ctx.r[3].s64 = ctx.r[10].s64 + -22244;
	// 82613EF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613EFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613F00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613F08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613F10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613F14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613F18: 4BE52F09  bl 0x82466e20
	ctx.lr = 0x82613F1C;
	sub_82466E20(ctx, base);
	// 82613F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613F30 size=112
    let mut pc: u32 = 0x82613F30;
    'dispatch: loop {
        match pc {
            0x82613F30 => {
    //   block [0x82613F30..0x82613FA0)
	// 82613F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613F3C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82613F40: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613F44: 392AF6E4  addi r9, r10, -0x91c
	ctx.r[9].s64 = ctx.r[10].s64 + -2332;
	// 82613F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613F4C: 390B7310  addi r8, r11, 0x7310
	ctx.r[8].s64 = ctx.r[11].s64 + 29456;
	// 82613F50: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82613F54: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 82613F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613F5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613F68: 386AA94C  addi r3, r10, -0x56b4
	ctx.r[3].s64 = ctx.r[10].s64 + -22196;
	// 82613F6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82613F70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82613F74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613F84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613F8C: 4BE52E95  bl 0x82466e20
	ctx.lr = 0x82613F90;
	sub_82466E20(ctx, base);
	// 82613F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613FA0 size=108
    let mut pc: u32 = 0x82613FA0;
    'dispatch: loop {
        match pc {
            0x82613FA0 => {
    //   block [0x82613FA0..0x8261400C)
	// 82613FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613FAC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613FB4: 38EB7340  addi r7, r11, 0x7340
	ctx.r[7].s64 = ctx.r[11].s64 + 29504;
	// 82613FB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82613FBC: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 82613FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613FC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613FC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613FD0: 386AA97C  addi r3, r10, -0x5684
	ctx.r[3].s64 = ctx.r[10].s64 + -22148;
	// 82613FD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613FF8: 4BE52E29  bl 0x82466e20
	ctx.lr = 0x82613FFC;
	sub_82466E20(ctx, base);
	// 82613FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614010 size=108
    let mut pc: u32 = 0x82614010;
    'dispatch: loop {
        match pc {
            0x82614010 => {
    //   block [0x82614010..0x8261407C)
	// 82614010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261401C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614024: 38EB7370  addi r7, r11, 0x7370
	ctx.r[7].s64 = ctx.r[11].s64 + 29552;
	// 82614028: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261402C: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 82614030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614034: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261403C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614040: 386AA9AC  addi r3, r10, -0x5654
	ctx.r[3].s64 = ctx.r[10].s64 + -22100;
	// 82614044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261404C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261405C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614068: 4BE52DB9  bl 0x82466e20
	ctx.lr = 0x8261406C;
	sub_82466E20(ctx, base);
	// 8261406C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614080 size=108
    let mut pc: u32 = 0x82614080;
    'dispatch: loop {
        match pc {
            0x82614080 => {
    //   block [0x82614080..0x826140EC)
	// 82614080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261408C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614094: 38EB7388  addi r7, r11, 0x7388
	ctx.r[7].s64 = ctx.r[11].s64 + 29576;
	// 82614098: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261409C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826140A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826140A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826140A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826140AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826140B0: 386AA9DC  addi r3, r10, -0x5624
	ctx.r[3].s64 = ctx.r[10].s64 + -22052;
	// 826140B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826140B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826140BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826140C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826140C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826140C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826140CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826140D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826140D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826140D8: 4BE52D49  bl 0x82466e20
	ctx.lr = 0x826140DC;
	sub_82466E20(ctx, base);
	// 826140DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826140E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826140E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826140E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826140F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826140F0 size=112
    let mut pc: u32 = 0x826140F0;
    'dispatch: loop {
        match pc {
            0x826140F0 => {
    //   block [0x826140F0..0x82614160)
	// 826140F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826140F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826140F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826140FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614100: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614104: 38AAAA3C  addi r5, r10, -0x55c4
	ctx.r[5].s64 = ctx.r[10].s64 + -21956;
	// 82614108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261410C: 390B73B8  addi r8, r11, 0x73b8
	ctx.r[8].s64 = ctx.r[11].s64 + 29624;
	// 82614110: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82614114: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82614118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261411C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82614124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614128: 386AAA0C  addi r3, r10, -0x55f4
	ctx.r[3].s64 = ctx.r[10].s64 + -22004;
	// 8261412C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82614130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261413C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261414C: 4BE52CD5  bl 0x82466e20
	ctx.lr = 0x82614150;
	sub_82466E20(ctx, base);
	// 82614150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261415C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614160 size=108
    let mut pc: u32 = 0x82614160;
    'dispatch: loop {
        match pc {
            0x82614160 => {
    //   block [0x82614160..0x826141CC)
	// 82614160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261416C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614174: 38EB73D0  addi r7, r11, 0x73d0
	ctx.r[7].s64 = ctx.r[11].s64 + 29648;
	// 82614178: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261417C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 82614180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614184: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261418C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614190: 386AAA3C  addi r3, r10, -0x55c4
	ctx.r[3].s64 = ctx.r[10].s64 + -21956;
	// 82614194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261419C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826141A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826141A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826141A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826141AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826141B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826141B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826141B8: 4BE52C69  bl 0x82466e20
	ctx.lr = 0x826141BC;
	sub_82466E20(ctx, base);
	// 826141BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826141C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826141C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826141C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826141D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826141D0 size=108
    let mut pc: u32 = 0x826141D0;
    'dispatch: loop {
        match pc {
            0x826141D0 => {
    //   block [0x826141D0..0x8261423C)
	// 826141D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826141D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826141D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826141DC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826141E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826141E4: 38EB7400  addi r7, r11, 0x7400
	ctx.r[7].s64 = ctx.r[11].s64 + 29696;
	// 826141E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826141EC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826141F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826141F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826141F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826141FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614200: 386AAA6C  addi r3, r10, -0x5594
	ctx.r[3].s64 = ctx.r[10].s64 + -21908;
	// 82614204: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261420C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261421C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614224: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614228: 4BE52BF9  bl 0x82466e20
	ctx.lr = 0x8261422C;
	sub_82466E20(ctx, base);
	// 8261422C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614240 size=108
    let mut pc: u32 = 0x82614240;
    'dispatch: loop {
        match pc {
            0x82614240 => {
    //   block [0x82614240..0x826142AC)
	// 82614240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261424C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614254: 38EB7418  addi r7, r11, 0x7418
	ctx.r[7].s64 = ctx.r[11].s64 + 29720;
	// 82614258: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261425C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82614260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614264: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614268: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261426C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614270: 386AAA9C  addi r3, r10, -0x5564
	ctx.r[3].s64 = ctx.r[10].s64 + -21860;
	// 82614274: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261427C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261428C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614294: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614298: 4BE52B89  bl 0x82466e20
	ctx.lr = 0x8261429C;
	sub_82466E20(ctx, base);
	// 8261429C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826142A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826142A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826142A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826142B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826142B0 size=108
    let mut pc: u32 = 0x826142B0;
    'dispatch: loop {
        match pc {
            0x826142B0 => {
    //   block [0x826142B0..0x8261431C)
	// 826142B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826142B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826142B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826142BC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826142C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826142C4: 38EB7448  addi r7, r11, 0x7448
	ctx.r[7].s64 = ctx.r[11].s64 + 29768;
	// 826142C8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826142CC: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826142D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826142D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826142D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826142DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826142E0: 386AAACC  addi r3, r10, -0x5534
	ctx.r[3].s64 = ctx.r[10].s64 + -21812;
	// 826142E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826142E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826142EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826142F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826142F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826142F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826142FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614308: 4BE52B19  bl 0x82466e20
	ctx.lr = 0x8261430C;
	sub_82466E20(ctx, base);
	// 8261430C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614320 size=108
    let mut pc: u32 = 0x82614320;
    'dispatch: loop {
        match pc {
            0x82614320 => {
    //   block [0x82614320..0x8261438C)
	// 82614320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261432C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614334: 38EB74F0  addi r7, r11, 0x74f0
	ctx.r[7].s64 = ctx.r[11].s64 + 29936;
	// 82614338: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261433C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 82614340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614344: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614348: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261434C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614350: 386AAAFC  addi r3, r10, -0x5504
	ctx.r[3].s64 = ctx.r[10].s64 + -21764;
	// 82614354: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261435C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261436C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614378: 4BE52AA9  bl 0x82466e20
	ctx.lr = 0x8261437C;
	sub_82466E20(ctx, base);
	// 8261437C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614390 size=108
    let mut pc: u32 = 0x82614390;
    'dispatch: loop {
        match pc {
            0x82614390 => {
    //   block [0x82614390..0x826143FC)
	// 82614390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261439C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826143A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826143A4: 38EB7520  addi r7, r11, 0x7520
	ctx.r[7].s64 = ctx.r[11].s64 + 29984;
	// 826143A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826143AC: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826143B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826143B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826143B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826143BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826143C0: 386AAB2C  addi r3, r10, -0x54d4
	ctx.r[3].s64 = ctx.r[10].s64 + -21716;
	// 826143C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826143C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826143CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826143D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826143D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826143D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826143DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826143E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826143E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826143E8: 4BE52A39  bl 0x82466e20
	ctx.lr = 0x826143EC;
	sub_82466E20(ctx, base);
	// 826143EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826143F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826143F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826143F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614400 size=108
    let mut pc: u32 = 0x82614400;
    'dispatch: loop {
        match pc {
            0x82614400 => {
    //   block [0x82614400..0x8261446C)
	// 82614400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261440C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614414: 38EB7538  addi r7, r11, 0x7538
	ctx.r[7].s64 = ctx.r[11].s64 + 30008;
	// 82614418: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261441C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82614420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614428: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261442C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614430: 386AAB5C  addi r3, r10, -0x54a4
	ctx.r[3].s64 = ctx.r[10].s64 + -21668;
	// 82614434: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261443C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261444C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614458: 4BE529C9  bl 0x82466e20
	ctx.lr = 0x8261445C;
	sub_82466E20(ctx, base);
	// 8261445C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614470 size=112
    let mut pc: u32 = 0x82614470;
    'dispatch: loop {
        match pc {
            0x82614470 => {
    //   block [0x82614470..0x826144E0)
	// 82614470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261447C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614480: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614484: 38AAA9AC  addi r5, r10, -0x5654
	ctx.r[5].s64 = ctx.r[10].s64 + -22100;
	// 82614488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261448C: 390B7568  addi r8, r11, 0x7568
	ctx.r[8].s64 = ctx.r[11].s64 + 30056;
	// 82614490: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82614494: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82614498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261449C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826144A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826144A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826144A8: 386AAB8C  addi r3, r10, -0x5474
	ctx.r[3].s64 = ctx.r[10].s64 + -21620;
	// 826144AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826144B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826144B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826144B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826144BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826144C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826144C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826144C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826144CC: 4BE52955  bl 0x82466e20
	ctx.lr = 0x826144D0;
	sub_82466E20(ctx, base);
	// 826144D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826144D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826144D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826144DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826144E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826144E0 size=24
    let mut pc: u32 = 0x826144E0;
    'dispatch: loop {
        match pc {
            0x826144E0 => {
    //   block [0x826144E0..0x826144F8)
	// 826144E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826144E4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826144E8: 394AD8E8  addi r10, r10, -0x2718
	ctx.r[10].s64 = ctx.r[10].s64 + -10008;
	// 826144EC: 816B7610  lwz r11, 0x7610(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30224 as u32) ) } as u64;
	// 826144F0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826144F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826144F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826144F8 size=112
    let mut pc: u32 = 0x826144F8;
    'dispatch: loop {
        match pc {
            0x826144F8 => {
    //   block [0x826144F8..0x82614568)
	// 826144F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826144FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614504: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82614508: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261450C: 392AF710  addi r9, r10, -0x8f0
	ctx.r[9].s64 = ctx.r[10].s64 + -2288;
	// 82614510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614514: 390BD8E8  addi r8, r11, -0x2718
	ctx.r[8].s64 = ctx.r[11].s64 + -10008;
	// 82614518: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8261451C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82614520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614524: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261452C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614530: 386AABBC  addi r3, r10, -0x5444
	ctx.r[3].s64 = ctx.r[10].s64 + -21572;
	// 82614534: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82614538: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261453C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261454C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614554: 4BE528CD  bl 0x82466e20
	ctx.lr = 0x82614558;
	sub_82466E20(ctx, base);
	// 82614558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261455C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614568 size=108
    let mut pc: u32 = 0x82614568;
    'dispatch: loop {
        match pc {
            0x82614568 => {
    //   block [0x82614568..0x826145D4)
	// 82614568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261456C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614574: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261457C: 38EB7618  addi r7, r11, 0x7618
	ctx.r[7].s64 = ctx.r[11].s64 + 30232;
	// 82614580: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82614584: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82614588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261458C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614598: 386AABEC  addi r3, r10, -0x5414
	ctx.r[3].s64 = ctx.r[10].s64 + -21524;
	// 8261459C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826145A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826145A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826145A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826145AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826145B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826145B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826145B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826145BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826145C0: 4BE52861  bl 0x82466e20
	ctx.lr = 0x826145C4;
	sub_82466E20(ctx, base);
	// 826145C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826145C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826145CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826145D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826145D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826145D8 size=116
    let mut pc: u32 = 0x826145D8;
    'dispatch: loop {
        match pc {
            0x826145D8 => {
    //   block [0x826145D8..0x8261464C)
	// 826145D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826145DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826145E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826145E4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826145E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826145EC: 390B7648  addi r8, r11, 0x7648
	ctx.r[8].s64 = ctx.r[11].s64 + 30280;
	// 826145F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826145F4: 392AF754  addi r9, r10, -0x8ac
	ctx.r[9].s64 = ctx.r[10].s64 + -2220;
	// 826145F8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826145FC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82614600: 38AAA9AC  addi r5, r10, -0x5654
	ctx.r[5].s64 = ctx.r[10].s64 + -22100;
	// 82614604: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82614608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261460C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261461C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82614620: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82614624: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82614628: 386BAC1C  addi r3, r11, -0x53e4
	ctx.r[3].s64 = ctx.r[11].s64 + -21476;
	// 8261462C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82614630: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614638: 4BE527E9  bl 0x82466e20
	ctx.lr = 0x8261463C;
	sub_82466E20(ctx, base);
	// 8261463C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82614650 size=24
    let mut pc: u32 = 0x82614650;
    'dispatch: loop {
        match pc {
            0x82614650 => {
    //   block [0x82614650..0x82614668)
	// 82614650: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614654: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82614658: 394AD960  addi r10, r10, -0x26a0
	ctx.r[10].s64 = ctx.r[10].s64 + -9888;
	// 8261465C: 816B7708  lwz r11, 0x7708(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30472 as u32) ) } as u64;
	// 82614660: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82614664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614668 size=112
    let mut pc: u32 = 0x82614668;
    'dispatch: loop {
        match pc {
            0x82614668 => {
    //   block [0x82614668..0x826146D8)
	// 82614668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261466C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614674: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82614678: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261467C: 392AF790  addi r9, r10, -0x870
	ctx.r[9].s64 = ctx.r[10].s64 + -2160;
	// 82614680: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614684: 390BD960  addi r8, r11, -0x26a0
	ctx.r[8].s64 = ctx.r[11].s64 + -9888;
	// 82614688: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8261468C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 82614690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614694: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614698: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261469C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826146A0: 386AAC4C  addi r3, r10, -0x53b4
	ctx.r[3].s64 = ctx.r[10].s64 + -21428;
	// 826146A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826146A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826146AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826146B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826146B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826146B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826146BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826146C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826146C4: 4BE5275D  bl 0x82466e20
	ctx.lr = 0x826146C8;
	sub_82466E20(ctx, base);
	// 826146C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826146CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826146D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826146D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826146D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826146D8 size=108
    let mut pc: u32 = 0x826146D8;
    'dispatch: loop {
        match pc {
            0x826146D8 => {
    //   block [0x826146D8..0x82614744)
	// 826146D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826146DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826146E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826146E4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826146E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826146EC: 38EB770C  addi r7, r11, 0x770c
	ctx.r[7].s64 = ctx.r[11].s64 + 30476;
	// 826146F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826146F4: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826146F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826146FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614700: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614708: 386AAC7C  addi r3, r10, -0x5384
	ctx.r[3].s64 = ctx.r[10].s64 + -21380;
	// 8261470C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261471C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261472C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614730: 4BE526F1  bl 0x82466e20
	ctx.lr = 0x82614734;
	sub_82466E20(ctx, base);
	// 82614734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261473C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614748 size=108
    let mut pc: u32 = 0x82614748;
    'dispatch: loop {
        match pc {
            0x82614748 => {
    //   block [0x82614748..0x826147B4)
	// 82614748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261474C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614754: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261475C: 38EB7724  addi r7, r11, 0x7724
	ctx.r[7].s64 = ctx.r[11].s64 + 30500;
	// 82614760: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82614764: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82614768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261476C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614778: 386AACAC  addi r3, r10, -0x5354
	ctx.r[3].s64 = ctx.r[10].s64 + -21332;
	// 8261477C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261478C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261479C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826147A0: 4BE52681  bl 0x82466e20
	ctx.lr = 0x826147A4;
	sub_82466E20(ctx, base);
	// 826147A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826147A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826147AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826147B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826147B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826147B8 size=24
    let mut pc: u32 = 0x826147B8;
    'dispatch: loop {
        match pc {
            0x826147B8 => {
    //   block [0x826147B8..0x826147D0)
	// 826147B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826147BC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826147C0: 394AD9A8  addi r10, r10, -0x2658
	ctx.r[10].s64 = ctx.r[10].s64 + -9816;
	// 826147C4: 816B7754  lwz r11, 0x7754(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30548 as u32) ) } as u64;
	// 826147C8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826147CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826147D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826147D0 size=112
    let mut pc: u32 = 0x826147D0;
    'dispatch: loop {
        match pc {
            0x826147D0 => {
    //   block [0x826147D0..0x82614840)
	// 826147D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826147D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826147D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826147DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826147E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826147E4: 392AF7CC  addi r9, r10, -0x834
	ctx.r[9].s64 = ctx.r[10].s64 + -2100;
	// 826147E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826147EC: 390BD9A8  addi r8, r11, -0x2658
	ctx.r[8].s64 = ctx.r[11].s64 + -9816;
	// 826147F0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826147F4: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826147F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826147FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82614804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614808: 386AACDC  addi r3, r10, -0x5324
	ctx.r[3].s64 = ctx.r[10].s64 + -21284;
	// 8261480C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82614810: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82614814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261481C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261482C: 4BE525F5  bl 0x82466e20
	ctx.lr = 0x82614830;
	sub_82466E20(ctx, base);
	// 82614830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261483C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614840 size=112
    let mut pc: u32 = 0x82614840;
    'dispatch: loop {
        match pc {
            0x82614840 => {
    //   block [0x82614840..0x826148B0)
	// 82614840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261484C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614850: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614854: 38AAA9AC  addi r5, r10, -0x5654
	ctx.r[5].s64 = ctx.r[10].s64 + -22100;
	// 82614858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261485C: 390B7758  addi r8, r11, 0x7758
	ctx.r[8].s64 = ctx.r[11].s64 + 30552;
	// 82614860: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82614864: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 82614868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261486C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82614874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614878: 386AAD0C  addi r3, r10, -0x52f4
	ctx.r[3].s64 = ctx.r[10].s64 + -21236;
	// 8261487C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82614880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261488C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261489C: 4BE52585  bl 0x82466e20
	ctx.lr = 0x826148A0;
	sub_82466E20(ctx, base);
	// 826148A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826148A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826148A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826148AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826148B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826148B0 size=108
    let mut pc: u32 = 0x826148B0;
    'dispatch: loop {
        match pc {
            0x826148B0 => {
    //   block [0x826148B0..0x8261491C)
	// 826148B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826148B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826148B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826148BC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826148C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826148C4: 38EB7788  addi r7, r11, 0x7788
	ctx.r[7].s64 = ctx.r[11].s64 + 30600;
	// 826148C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826148CC: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826148D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826148D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826148D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826148DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826148E0: 386AAD3C  addi r3, r10, -0x52c4
	ctx.r[3].s64 = ctx.r[10].s64 + -21188;
	// 826148E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826148E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826148EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826148F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826148F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826148F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826148FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614908: 4BE52519  bl 0x82466e20
	ctx.lr = 0x8261490C;
	sub_82466E20(ctx, base);
	// 8261490C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614920 size=108
    let mut pc: u32 = 0x82614920;
    'dispatch: loop {
        match pc {
            0x82614920 => {
    //   block [0x82614920..0x8261498C)
	// 82614920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261492C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614934: 38EB77B8  addi r7, r11, 0x77b8
	ctx.r[7].s64 = ctx.r[11].s64 + 30648;
	// 82614938: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8261493C: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82614940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614944: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614948: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261494C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614950: 386AAD6C  addi r3, r10, -0x5294
	ctx.r[3].s64 = ctx.r[10].s64 + -21140;
	// 82614954: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261495C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261496C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614978: 4BE524A9  bl 0x82466e20
	ctx.lr = 0x8261497C;
	sub_82466E20(ctx, base);
	// 8261497C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614990 size=108
    let mut pc: u32 = 0x82614990;
    'dispatch: loop {
        match pc {
            0x82614990 => {
    //   block [0x82614990..0x826149FC)
	// 82614990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261499C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826149A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826149A4: 38EB7818  addi r7, r11, 0x7818
	ctx.r[7].s64 = ctx.r[11].s64 + 30744;
	// 826149A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826149AC: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826149B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826149B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826149B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826149BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826149C0: 386AAD9C  addi r3, r10, -0x5264
	ctx.r[3].s64 = ctx.r[10].s64 + -21092;
	// 826149C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826149C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826149CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826149D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826149D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826149D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826149DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826149E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826149E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826149E8: 4BE52439  bl 0x82466e20
	ctx.lr = 0x826149EC;
	sub_82466E20(ctx, base);
	// 826149EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826149F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826149F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826149F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614A00 size=108
    let mut pc: u32 = 0x82614A00;
    'dispatch: loop {
        match pc {
            0x82614A00 => {
    //   block [0x82614A00..0x82614A6C)
	// 82614A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614A0C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614A14: 38EB7848  addi r7, r11, 0x7848
	ctx.r[7].s64 = ctx.r[11].s64 + 30792;
	// 82614A18: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82614A1C: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82614A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614A24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614A28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614A30: 386AADCC  addi r3, r10, -0x5234
	ctx.r[3].s64 = ctx.r[10].s64 + -21044;
	// 82614A34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614A54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614A58: 4BE523C9  bl 0x82466e20
	ctx.lr = 0x82614A5C;
	sub_82466E20(ctx, base);
	// 82614A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614A70 size=108
    let mut pc: u32 = 0x82614A70;
    'dispatch: loop {
        match pc {
            0x82614A70 => {
    //   block [0x82614A70..0x82614ADC)
	// 82614A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614A7C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614A84: 38EB7968  addi r7, r11, 0x7968
	ctx.r[7].s64 = ctx.r[11].s64 + 31080;
	// 82614A88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82614A8C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 82614A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614A94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614A98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614AA0: 386AADFC  addi r3, r10, -0x5204
	ctx.r[3].s64 = ctx.r[10].s64 + -20996;
	// 82614AA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614AC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614AC8: 4BE52359  bl 0x82466e20
	ctx.lr = 0x82614ACC;
	sub_82466E20(ctx, base);
	// 82614ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614AE0 size=108
    let mut pc: u32 = 0x82614AE0;
    'dispatch: loop {
        match pc {
            0x82614AE0 => {
    //   block [0x82614AE0..0x82614B4C)
	// 82614AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614AEC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614AF4: 38EB7980  addi r7, r11, 0x7980
	ctx.r[7].s64 = ctx.r[11].s64 + 31104;
	// 82614AF8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82614AFC: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 82614B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614B04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614B08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614B10: 386AAE2C  addi r3, r10, -0x51d4
	ctx.r[3].s64 = ctx.r[10].s64 + -20948;
	// 82614B14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614B34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614B38: 4BE522E9  bl 0x82466e20
	ctx.lr = 0x82614B3C;
	sub_82466E20(ctx, base);
	// 82614B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614B50 size=108
    let mut pc: u32 = 0x82614B50;
    'dispatch: loop {
        match pc {
            0x82614B50 => {
    //   block [0x82614B50..0x82614BBC)
	// 82614B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614B5C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614B64: 38EB7998  addi r7, r11, 0x7998
	ctx.r[7].s64 = ctx.r[11].s64 + 31128;
	// 82614B68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82614B6C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 82614B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614B74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614B78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614B80: 386AAE5C  addi r3, r10, -0x51a4
	ctx.r[3].s64 = ctx.r[10].s64 + -20900;
	// 82614B84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614BA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614BA8: 4BE52279  bl 0x82466e20
	ctx.lr = 0x82614BAC;
	sub_82466E20(ctx, base);
	// 82614BAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614BC0 size=108
    let mut pc: u32 = 0x82614BC0;
    'dispatch: loop {
        match pc {
            0x82614BC0 => {
    //   block [0x82614BC0..0x82614C2C)
	// 82614BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614BCC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614BD4: 38EB79B0  addi r7, r11, 0x79b0
	ctx.r[7].s64 = ctx.r[11].s64 + 31152;
	// 82614BD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82614BDC: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 82614BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614BE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614BE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614BF0: 386AAE8C  addi r3, r10, -0x5174
	ctx.r[3].s64 = ctx.r[10].s64 + -20852;
	// 82614BF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614C14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614C18: 4BE52209  bl 0x82466e20
	ctx.lr = 0x82614C1C;
	sub_82466E20(ctx, base);
	// 82614C1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614C30 size=108
    let mut pc: u32 = 0x82614C30;
    'dispatch: loop {
        match pc {
            0x82614C30 => {
    //   block [0x82614C30..0x82614C9C)
	// 82614C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614C38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614C3C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614C44: 38EB79C8  addi r7, r11, 0x79c8
	ctx.r[7].s64 = ctx.r[11].s64 + 31176;
	// 82614C48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82614C4C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 82614C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614C54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614C58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614C60: 386AAEBC  addi r3, r10, -0x5144
	ctx.r[3].s64 = ctx.r[10].s64 + -20804;
	// 82614C64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614C6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614C74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614C84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614C88: 4BE52199  bl 0x82466e20
	ctx.lr = 0x82614C8C;
	sub_82466E20(ctx, base);
	// 82614C8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614CA0 size=108
    let mut pc: u32 = 0x82614CA0;
    'dispatch: loop {
        match pc {
            0x82614CA0 => {
    //   block [0x82614CA0..0x82614D0C)
	// 82614CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614CAC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614CB4: 38EB79E0  addi r7, r11, 0x79e0
	ctx.r[7].s64 = ctx.r[11].s64 + 31200;
	// 82614CB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82614CBC: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 82614CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614CC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614CC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614CCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614CD0: 386AAEEC  addi r3, r10, -0x5114
	ctx.r[3].s64 = ctx.r[10].s64 + -20756;
	// 82614CD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614CDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614CE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614CEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614CF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614CF8: 4BE52129  bl 0x82466e20
	ctx.lr = 0x82614CFC;
	sub_82466E20(ctx, base);
	// 82614CFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614D10 size=108
    let mut pc: u32 = 0x82614D10;
    'dispatch: loop {
        match pc {
            0x82614D10 => {
    //   block [0x82614D10..0x82614D7C)
	// 82614D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614D1C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614D20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614D24: 38EB79F8  addi r7, r11, 0x79f8
	ctx.r[7].s64 = ctx.r[11].s64 + 31224;
	// 82614D28: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82614D2C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 82614D30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614D34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614D38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614D3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614D40: 386AAF1C  addi r3, r10, -0x50e4
	ctx.r[3].s64 = ctx.r[10].s64 + -20708;
	// 82614D44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614D48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614D4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614D50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614D54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614D58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614D64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614D68: 4BE520B9  bl 0x82466e20
	ctx.lr = 0x82614D6C;
	sub_82466E20(ctx, base);
	// 82614D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614D80 size=108
    let mut pc: u32 = 0x82614D80;
    'dispatch: loop {
        match pc {
            0x82614D80 => {
    //   block [0x82614D80..0x82614DEC)
	// 82614D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614D8C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614D94: 38EB7A88  addi r7, r11, 0x7a88
	ctx.r[7].s64 = ctx.r[11].s64 + 31368;
	// 82614D98: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82614D9C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 82614DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614DA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614DA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614DB0: 386AAF4C  addi r3, r10, -0x50b4
	ctx.r[3].s64 = ctx.r[10].s64 + -20660;
	// 82614DB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614DD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614DD8: 4BE52049  bl 0x82466e20
	ctx.lr = 0x82614DDC;
	sub_82466E20(ctx, base);
	// 82614DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614DF0 size=108
    let mut pc: u32 = 0x82614DF0;
    'dispatch: loop {
        match pc {
            0x82614DF0 => {
    //   block [0x82614DF0..0x82614E5C)
	// 82614DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614DFC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614E00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614E04: 38EB7B48  addi r7, r11, 0x7b48
	ctx.r[7].s64 = ctx.r[11].s64 + 31560;
	// 82614E08: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82614E0C: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 82614E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614E14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614E18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614E20: 386AAF7C  addi r3, r10, -0x5084
	ctx.r[3].s64 = ctx.r[10].s64 + -20612;
	// 82614E24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614E28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614E34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614E44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614E48: 4BE51FD9  bl 0x82466e20
	ctx.lr = 0x82614E4C;
	sub_82466E20(ctx, base);
	// 82614E4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614E60 size=108
    let mut pc: u32 = 0x82614E60;
    'dispatch: loop {
        match pc {
            0x82614E60 => {
    //   block [0x82614E60..0x82614ECC)
	// 82614E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614E6C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614E70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614E74: 38EB7C20  addi r7, r11, 0x7c20
	ctx.r[7].s64 = ctx.r[11].s64 + 31776;
	// 82614E78: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82614E7C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 82614E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614E84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614E88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614E90: 386AAFAC  addi r3, r10, -0x5054
	ctx.r[3].s64 = ctx.r[10].s64 + -20564;
	// 82614E94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614EB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614EB8: 4BE51F69  bl 0x82466e20
	ctx.lr = 0x82614EBC;
	sub_82466E20(ctx, base);
	// 82614EBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614ED0 size=108
    let mut pc: u32 = 0x82614ED0;
    'dispatch: loop {
        match pc {
            0x82614ED0 => {
    //   block [0x82614ED0..0x82614F3C)
	// 82614ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614EDC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614EE4: 38EB7CE0  addi r7, r11, 0x7ce0
	ctx.r[7].s64 = ctx.r[11].s64 + 31968;
	// 82614EE8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82614EEC: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 82614EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614EF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614EF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614F00: 386AAFDC  addi r3, r10, -0x5024
	ctx.r[3].s64 = ctx.r[10].s64 + -20516;
	// 82614F04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614F14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614F24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614F28: 4BE51EF9  bl 0x82466e20
	ctx.lr = 0x82614F2C;
	sub_82466E20(ctx, base);
	// 82614F2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614F30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614F34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614F40 size=112
    let mut pc: u32 = 0x82614F40;
    'dispatch: loop {
        match pc {
            0x82614F40 => {
    //   block [0x82614F40..0x82614FB0)
	// 82614F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614F4C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82614F50: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82614F54: 38EA7D88  addi r7, r10, 0x7d88
	ctx.r[7].s64 = ctx.r[10].s64 + 32136;
	// 82614F58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614F5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82614F60: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 82614F64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614F68: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614F6C: 396BF7E0  addi r11, r11, -0x820
	ctx.r[11].s64 = ctx.r[11].s64 + -2080;
	// 82614F70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614F74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614F78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614F7C: 386AB00C  addi r3, r10, -0x4ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -20468;
	// 82614F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614F84: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82614F88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614F8C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82614F90: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614F94: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614F98: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614F9C: 4BE51E85  bl 0x82466e20
	ctx.lr = 0x82614FA0;
	sub_82466E20(ctx, base);
	// 82614FA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614FB0 size=108
    let mut pc: u32 = 0x82614FB0;
    'dispatch: loop {
        match pc {
            0x82614FB0 => {
    //   block [0x82614FB0..0x8261501C)
	// 82614FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614FBC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614FC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614FC4: 38EB7EA8  addi r7, r11, 0x7ea8
	ctx.r[7].s64 = ctx.r[11].s64 + 32424;
	// 82614FC8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82614FCC: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 82614FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614FD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614FD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614FE0: 386AB03C  addi r3, r10, -0x4fc4
	ctx.r[3].s64 = ctx.r[10].s64 + -20420;
	// 82614FE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614FE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614FEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615008: 4BE51E19  bl 0x82466e20
	ctx.lr = 0x8261500C;
	sub_82466E20(ctx, base);
	// 8261500C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615020 size=108
    let mut pc: u32 = 0x82615020;
    'dispatch: loop {
        match pc {
            0x82615020 => {
    //   block [0x82615020..0x8261508C)
	// 82615020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261502C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82615030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615034: 38EB7F08  addi r7, r11, 0x7f08
	ctx.r[7].s64 = ctx.r[11].s64 + 32520;
	// 82615038: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8261503C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 82615040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615044: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261504C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615050: 386AB06C  addi r3, r10, -0x4f94
	ctx.r[3].s64 = ctx.r[10].s64 + -20372;
	// 82615054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261505C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261506C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615078: 4BE51DA9  bl 0x82466e20
	ctx.lr = 0x8261507C;
	sub_82466E20(ctx, base);
	// 8261507C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615090 size=108
    let mut pc: u32 = 0x82615090;
    'dispatch: loop {
        match pc {
            0x82615090 => {
    //   block [0x82615090..0x826150FC)
	// 82615090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261509C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826150A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826150A4: 38EB8010  addi r7, r11, -0x7ff0
	ctx.r[7].s64 = ctx.r[11].s64 + -32752;
	// 826150A8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826150AC: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826150B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826150B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826150B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826150BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826150C0: 386AB09C  addi r3, r10, -0x4f64
	ctx.r[3].s64 = ctx.r[10].s64 + -20324;
	// 826150C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826150C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826150CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826150D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826150D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826150D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826150DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826150E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826150E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826150E8: 4BE51D39  bl 0x82466e20
	ctx.lr = 0x826150EC;
	sub_82466E20(ctx, base);
	// 826150EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826150F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826150F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826150F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615100 size=108
    let mut pc: u32 = 0x82615100;
    'dispatch: loop {
        match pc {
            0x82615100 => {
    //   block [0x82615100..0x8261516C)
	// 82615100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261510C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615114: 38EB80E8  addi r7, r11, -0x7f18
	ctx.r[7].s64 = ctx.r[11].s64 + -32536;
	// 82615118: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261511C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 82615120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615124: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615128: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261512C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615130: 386AB0CC  addi r3, r10, -0x4f34
	ctx.r[3].s64 = ctx.r[10].s64 + -20276;
	// 82615134: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261513C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261514C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615154: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615158: 4BE51CC9  bl 0x82466e20
	ctx.lr = 0x8261515C;
	sub_82466E20(ctx, base);
	// 8261515C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615170 size=108
    let mut pc: u32 = 0x82615170;
    'dispatch: loop {
        match pc {
            0x82615170 => {
    //   block [0x82615170..0x826151DC)
	// 82615170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261517C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615184: 38EB8130  addi r7, r11, -0x7ed0
	ctx.r[7].s64 = ctx.r[11].s64 + -32464;
	// 82615188: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261518C: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 82615190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615194: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615198: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261519C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826151A0: 386AB0FC  addi r3, r10, -0x4f04
	ctx.r[3].s64 = ctx.r[10].s64 + -20228;
	// 826151A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826151A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826151AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826151B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826151B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826151B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826151BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826151C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826151C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826151C8: 4BE51C59  bl 0x82466e20
	ctx.lr = 0x826151CC;
	sub_82466E20(ctx, base);
	// 826151CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826151D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826151D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826151D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826151E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826151E0 size=112
    let mut pc: u32 = 0x826151E0;
    'dispatch: loop {
        match pc {
            0x826151E0 => {
    //   block [0x826151E0..0x82615250)
	// 826151E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826151E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826151E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826151EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826151F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826151F4: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826151F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826151FC: 390B8148  addi r8, r11, -0x7eb8
	ctx.r[8].s64 = ctx.r[11].s64 + -32440;
	// 82615200: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82615204: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 82615208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261520C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615210: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82615214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615218: 386AB12C  addi r3, r10, -0x4ed4
	ctx.r[3].s64 = ctx.r[10].s64 + -20180;
	// 8261521C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261522C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261523C: 4BE51BE5  bl 0x82466e20
	ctx.lr = 0x82615240;
	sub_82466E20(ctx, base);
	// 82615240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261524C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615250 size=112
    let mut pc: u32 = 0x82615250;
    'dispatch: loop {
        match pc {
            0x82615250 => {
    //   block [0x82615250..0x826152C0)
	// 82615250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261525C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615260: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615264: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82615268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261526C: 390B81A8  addi r8, r11, -0x7e58
	ctx.r[8].s64 = ctx.r[11].s64 + -32344;
	// 82615270: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82615274: 388A3824  addi r4, r10, 0x3824
	ctx.r[4].s64 = ctx.r[10].s64 + 14372;
	// 82615278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261527C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615280: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82615284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615288: 386AB15C  addi r3, r10, -0x4ea4
	ctx.r[3].s64 = ctx.r[10].s64 + -20132;
	// 8261528C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261529C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826152A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826152A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826152A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826152AC: 4BE51B75  bl 0x82466e20
	ctx.lr = 0x826152B0;
	sub_82466E20(ctx, base);
	// 826152B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826152B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826152B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826152BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826152C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826152C0 size=108
    let mut pc: u32 = 0x826152C0;
    'dispatch: loop {
        match pc {
            0x826152C0 => {
    //   block [0x826152C0..0x8261532C)
	// 826152C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826152C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826152C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826152CC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826152D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826152D4: 38EB81F0  addi r7, r11, -0x7e10
	ctx.r[7].s64 = ctx.r[11].s64 + -32272;
	// 826152D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826152DC: 388A3838  addi r4, r10, 0x3838
	ctx.r[4].s64 = ctx.r[10].s64 + 14392;
	// 826152E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826152E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826152E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826152EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826152F0: 386AB18C  addi r3, r10, -0x4e74
	ctx.r[3].s64 = ctx.r[10].s64 + -20084;
	// 826152F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826152F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826152FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261530C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615318: 4BE51B09  bl 0x82466e20
	ctx.lr = 0x8261531C;
	sub_82466E20(ctx, base);
	// 8261531C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82615330 size=24
    let mut pc: u32 = 0x82615330;
    'dispatch: loop {
        match pc {
            0x82615330 => {
    //   block [0x82615330..0x82615348)
	// 82615330: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615334: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82615338: 394ADA20  addi r10, r10, -0x25e0
	ctx.r[10].s64 = ctx.r[10].s64 + -9696;
	// 8261533C: 816B8730  lwz r11, -0x78d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30928 as u32) ) } as u64;
	// 82615340: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82615344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615348 size=112
    let mut pc: u32 = 0x82615348;
    'dispatch: loop {
        match pc {
            0x82615348 => {
    //   block [0x82615348..0x826153B8)
	// 82615348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261534C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615354: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615358: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261535C: 38AAB39C  addi r5, r10, -0x4c64
	ctx.r[5].s64 = ctx.r[10].s64 + -19556;
	// 82615360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615364: 390BDA20  addi r8, r11, -0x25e0
	ctx.r[8].s64 = ctx.r[11].s64 + -9696;
	// 82615368: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8261536C: 388A3888  addi r4, r10, 0x3888
	ctx.r[4].s64 = ctx.r[10].s64 + 14472;
	// 82615370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615374: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261537C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615380: 386AB1BC  addi r3, r10, -0x4e44
	ctx.r[3].s64 = ctx.r[10].s64 + -20036;
	// 82615384: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261538C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261539C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826153A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826153A4: 4BE51A7D  bl 0x82466e20
	ctx.lr = 0x826153A8;
	sub_82466E20(ctx, base);
	// 826153A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826153AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826153B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826153B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826153B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826153B8 size=108
    let mut pc: u32 = 0x826153B8;
    'dispatch: loop {
        match pc {
            0x826153B8 => {
    //   block [0x826153B8..0x82615424)
	// 826153B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826153BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826153C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826153C4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826153C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826153CC: 38EB8208  addi r7, r11, -0x7df8
	ctx.r[7].s64 = ctx.r[11].s64 + -32248;
	// 826153D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826153D4: 388A38A0  addi r4, r10, 0x38a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14496;
	// 826153D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826153DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826153E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826153E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826153E8: 386AB1EC  addi r3, r10, -0x4e14
	ctx.r[3].s64 = ctx.r[10].s64 + -19988;
	// 826153EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826153F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826153F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826153F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826153FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261540C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615410: 4BE51A11  bl 0x82466e20
	ctx.lr = 0x82615414;
	sub_82466E20(ctx, base);
	// 82615414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261541C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615428 size=112
    let mut pc: u32 = 0x82615428;
    'dispatch: loop {
        match pc {
            0x82615428 => {
    //   block [0x82615428..0x82615498)
	// 82615428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261542C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615434: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615438: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261543C: 38AAB39C  addi r5, r10, -0x4c64
	ctx.r[5].s64 = ctx.r[10].s64 + -19556;
	// 82615440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615444: 390B8268  addi r8, r11, -0x7d98
	ctx.r[8].s64 = ctx.r[11].s64 + -32152;
	// 82615448: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8261544C: 388A38BC  addi r4, r10, 0x38bc
	ctx.r[4].s64 = ctx.r[10].s64 + 14524;
	// 82615450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615454: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615458: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261545C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615460: 386AB21C  addi r3, r10, -0x4de4
	ctx.r[3].s64 = ctx.r[10].s64 + -19940;
	// 82615464: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261546C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261547C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615484: 4BE5199D  bl 0x82466e20
	ctx.lr = 0x82615488;
	sub_82466E20(ctx, base);
	// 82615488: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261548C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615498 size=108
    let mut pc: u32 = 0x82615498;
    'dispatch: loop {
        match pc {
            0x82615498 => {
    //   block [0x82615498..0x82615504)
	// 82615498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261549C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826154A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826154A4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826154A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826154AC: 38EB8328  addi r7, r11, -0x7cd8
	ctx.r[7].s64 = ctx.r[11].s64 + -31960;
	// 826154B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826154B4: 388A38D0  addi r4, r10, 0x38d0
	ctx.r[4].s64 = ctx.r[10].s64 + 14544;
	// 826154B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826154BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826154C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826154C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826154C8: 386AB24C  addi r3, r10, -0x4db4
	ctx.r[3].s64 = ctx.r[10].s64 + -19892;
	// 826154CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826154D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826154D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826154D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826154DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826154E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826154E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826154E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826154EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826154F0: 4BE51931  bl 0x82466e20
	ctx.lr = 0x826154F4;
	sub_82466E20(ctx, base);
	// 826154F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826154F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826154FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615508 size=108
    let mut pc: u32 = 0x82615508;
    'dispatch: loop {
        match pc {
            0x82615508 => {
    //   block [0x82615508..0x82615574)
	// 82615508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261550C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615514: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261551C: 38EB83A0  addi r7, r11, -0x7c60
	ctx.r[7].s64 = ctx.r[11].s64 + -31840;
	// 82615520: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82615524: 388A38E0  addi r4, r10, 0x38e0
	ctx.r[4].s64 = ctx.r[10].s64 + 14560;
	// 82615528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261552C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615530: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615538: 386AB27C  addi r3, r10, -0x4d84
	ctx.r[3].s64 = ctx.r[10].s64 + -19844;
	// 8261553C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261554C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261555C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615560: 4BE518C1  bl 0x82466e20
	ctx.lr = 0x82615564;
	sub_82466E20(ctx, base);
	// 82615564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261556C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615578 size=108
    let mut pc: u32 = 0x82615578;
    'dispatch: loop {
        match pc {
            0x82615578 => {
    //   block [0x82615578..0x826155E4)
	// 82615578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261557C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615584: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261558C: 38EB83E8  addi r7, r11, -0x7c18
	ctx.r[7].s64 = ctx.r[11].s64 + -31768;
	// 82615590: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82615594: 388A3904  addi r4, r10, 0x3904
	ctx.r[4].s64 = ctx.r[10].s64 + 14596;
	// 82615598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261559C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826155A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826155A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826155A8: 386AB2AC  addi r3, r10, -0x4d54
	ctx.r[3].s64 = ctx.r[10].s64 + -19796;
	// 826155AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826155B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826155B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826155B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826155BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826155C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826155C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826155C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826155CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826155D0: 4BE51851  bl 0x82466e20
	ctx.lr = 0x826155D4;
	sub_82466E20(ctx, base);
	// 826155D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826155D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826155DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826155E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826155E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826155E8 size=112
    let mut pc: u32 = 0x826155E8;
    'dispatch: loop {
        match pc {
            0x826155E8 => {
    //   block [0x826155E8..0x82615658)
	// 826155E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826155EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826155F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826155F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826155F8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826155FC: 38AAB2AC  addi r5, r10, -0x4d54
	ctx.r[5].s64 = ctx.r[10].s64 + -19796;
	// 82615600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615604: 390B8430  addi r8, r11, -0x7bd0
	ctx.r[8].s64 = ctx.r[11].s64 + -31696;
	// 82615608: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8261560C: 388A3928  addi r4, r10, 0x3928
	ctx.r[4].s64 = ctx.r[10].s64 + 14632;
	// 82615610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615614: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261561C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615620: 386AB2DC  addi r3, r10, -0x4d24
	ctx.r[3].s64 = ctx.r[10].s64 + -19748;
	// 82615624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261562C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261563C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615644: 4BE517DD  bl 0x82466e20
	ctx.lr = 0x82615648;
	sub_82466E20(ctx, base);
	// 82615648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261564C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615658 size=108
    let mut pc: u32 = 0x82615658;
    'dispatch: loop {
        match pc {
            0x82615658 => {
    //   block [0x82615658..0x826156C4)
	// 82615658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261565C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615664: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261566C: 38EB84A8  addi r7, r11, -0x7b58
	ctx.r[7].s64 = ctx.r[11].s64 + -31576;
	// 82615670: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82615674: 388A394C  addi r4, r10, 0x394c
	ctx.r[4].s64 = ctx.r[10].s64 + 14668;
	// 82615678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261567C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615688: 386AB30C  addi r3, r10, -0x4cf4
	ctx.r[3].s64 = ctx.r[10].s64 + -19700;
	// 8261568C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261569C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826156A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826156A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826156A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826156AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826156B0: 4BE51771  bl 0x82466e20
	ctx.lr = 0x826156B4;
	sub_82466E20(ctx, base);
	// 826156B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826156B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826156BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826156C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826156C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826156C8 size=108
    let mut pc: u32 = 0x826156C8;
    'dispatch: loop {
        match pc {
            0x826156C8 => {
    //   block [0x826156C8..0x82615734)
	// 826156C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826156CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826156D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826156D4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826156D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826156DC: 38EB84F0  addi r7, r11, -0x7b10
	ctx.r[7].s64 = ctx.r[11].s64 + -31504;
	// 826156E0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826156E4: 388A3968  addi r4, r10, 0x3968
	ctx.r[4].s64 = ctx.r[10].s64 + 14696;
	// 826156E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826156EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826156F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826156F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826156F8: 386AB33C  addi r3, r10, -0x4cc4
	ctx.r[3].s64 = ctx.r[10].s64 + -19652;
	// 826156FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261570C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261571C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615720: 4BE51701  bl 0x82466e20
	ctx.lr = 0x82615724;
	sub_82466E20(ctx, base);
	// 82615724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261572C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615738 size=108
    let mut pc: u32 = 0x82615738;
    'dispatch: loop {
        match pc {
            0x82615738 => {
    //   block [0x82615738..0x826157A4)
	// 82615738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261573C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615744: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261574C: 38EB85B0  addi r7, r11, -0x7a50
	ctx.r[7].s64 = ctx.r[11].s64 + -31312;
	// 82615750: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82615754: 388A3980  addi r4, r10, 0x3980
	ctx.r[4].s64 = ctx.r[10].s64 + 14720;
	// 82615758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261575C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615760: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615768: 386AB36C  addi r3, r10, -0x4c94
	ctx.r[3].s64 = ctx.r[10].s64 + -19604;
	// 8261576C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261577C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261578C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615790: 4BE51691  bl 0x82466e20
	ctx.lr = 0x82615794;
	sub_82466E20(ctx, base);
	// 82615794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261579C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826157A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826157A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826157A8 size=112
    let mut pc: u32 = 0x826157A8;
    'dispatch: loop {
        match pc {
            0x826157A8 => {
    //   block [0x826157A8..0x82615818)
	// 826157A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826157AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826157B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826157B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826157B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826157BC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826157C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826157C4: 390B8738  addi r8, r11, -0x78c8
	ctx.r[8].s64 = ctx.r[11].s64 + -30920;
	// 826157C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826157CC: 388A3990  addi r4, r10, 0x3990
	ctx.r[4].s64 = ctx.r[10].s64 + 14736;
	// 826157D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826157D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826157D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826157DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826157E0: 386AB39C  addi r3, r10, -0x4c64
	ctx.r[3].s64 = ctx.r[10].s64 + -19556;
	// 826157E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826157E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826157EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826157F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826157F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826157F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826157FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615804: 4BE5161D  bl 0x82466e20
	ctx.lr = 0x82615808;
	sub_82466E20(ctx, base);
	// 82615808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261580C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615818 size=108
    let mut pc: u32 = 0x82615818;
    'dispatch: loop {
        match pc {
            0x82615818 => {
    //   block [0x82615818..0x82615884)
	// 82615818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261581C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615824: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261582C: 38EB8798  addi r7, r11, -0x7868
	ctx.r[7].s64 = ctx.r[11].s64 + -30824;
	// 82615830: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82615834: 388A39A0  addi r4, r10, 0x39a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14752;
	// 82615838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261583C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615840: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615848: 386AB3CC  addi r3, r10, -0x4c34
	ctx.r[3].s64 = ctx.r[10].s64 + -19508;
	// 8261584C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261585C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261586C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615870: 4BE515B1  bl 0x82466e20
	ctx.lr = 0x82615874;
	sub_82466E20(ctx, base);
	// 82615874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261587C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615888 size=112
    let mut pc: u32 = 0x82615888;
    'dispatch: loop {
        match pc {
            0x82615888 => {
    //   block [0x82615888..0x826158F8)
	// 82615888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261588C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615894: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615898: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261589C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826158A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826158A4: 390B8810  addi r8, r11, -0x77f0
	ctx.r[8].s64 = ctx.r[11].s64 + -30704;
	// 826158A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826158AC: 388A39B4  addi r4, r10, 0x39b4
	ctx.r[4].s64 = ctx.r[10].s64 + 14772;
	// 826158B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826158B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826158B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826158BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826158C0: 386AB3FC  addi r3, r10, -0x4c04
	ctx.r[3].s64 = ctx.r[10].s64 + -19460;
	// 826158C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826158C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826158CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826158D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826158D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826158D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826158DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826158E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826158E4: 4BE5153D  bl 0x82466e20
	ctx.lr = 0x826158E8;
	sub_82466E20(ctx, base);
	// 826158E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826158EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826158F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826158F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826158F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826158F8 size=108
    let mut pc: u32 = 0x826158F8;
    'dispatch: loop {
        match pc {
            0x826158F8 => {
    //   block [0x826158F8..0x82615964)
	// 826158F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826158FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615904: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261590C: 38EB8858  addi r7, r11, -0x77a8
	ctx.r[7].s64 = ctx.r[11].s64 + -30632;
	// 82615910: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82615914: 388A39C0  addi r4, r10, 0x39c0
	ctx.r[4].s64 = ctx.r[10].s64 + 14784;
	// 82615918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261591C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615928: 386AB42C  addi r3, r10, -0x4bd4
	ctx.r[3].s64 = ctx.r[10].s64 + -19412;
	// 8261592C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261593C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261594C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615950: 4BE514D1  bl 0x82466e20
	ctx.lr = 0x82615954;
	sub_82466E20(ctx, base);
	// 82615954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261595C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615968 size=108
    let mut pc: u32 = 0x82615968;
    'dispatch: loop {
        match pc {
            0x82615968 => {
    //   block [0x82615968..0x826159D4)
	// 82615968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261596C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615974: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261597C: 38EB88B8  addi r7, r11, -0x7748
	ctx.r[7].s64 = ctx.r[11].s64 + -30536;
	// 82615980: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82615984: 388A0DBC  addi r4, r10, 0xdbc
	ctx.r[4].s64 = ctx.r[10].s64 + 3516;
	// 82615988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261598C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615998: 386AB45C  addi r3, r10, -0x4ba4
	ctx.r[3].s64 = ctx.r[10].s64 + -19364;
	// 8261599C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826159A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826159A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826159A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826159AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826159B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826159B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826159B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826159BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826159C0: 4BE51461  bl 0x82466e20
	ctx.lr = 0x826159C4;
	sub_82466E20(ctx, base);
	// 826159C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826159C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826159CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826159D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826159D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826159D8 size=108
    let mut pc: u32 = 0x826159D8;
    'dispatch: loop {
        match pc {
            0x826159D8 => {
    //   block [0x826159D8..0x82615A44)
	// 826159D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826159DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826159E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826159E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826159E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826159EC: 38EB8900  addi r7, r11, -0x7700
	ctx.r[7].s64 = ctx.r[11].s64 + -30464;
	// 826159F0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826159F4: 388A39D8  addi r4, r10, 0x39d8
	ctx.r[4].s64 = ctx.r[10].s64 + 14808;
	// 826159F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826159FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615A00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615A08: 386AB48C  addi r3, r10, -0x4b74
	ctx.r[3].s64 = ctx.r[10].s64 + -19316;
	// 82615A0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615A2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615A30: 4BE513F1  bl 0x82466e20
	ctx.lr = 0x82615A34;
	sub_82466E20(ctx, base);
	// 82615A34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615A48 size=108
    let mut pc: u32 = 0x82615A48;
    'dispatch: loop {
        match pc {
            0x82615A48 => {
    //   block [0x82615A48..0x82615AB4)
	// 82615A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615A54: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615A5C: 38EB89C0  addi r7, r11, -0x7640
	ctx.r[7].s64 = ctx.r[11].s64 + -30272;
	// 82615A60: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82615A64: 388A39F4  addi r4, r10, 0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14836;
	// 82615A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615A6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615A70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615A78: 386AB4BC  addi r3, r10, -0x4b44
	ctx.r[3].s64 = ctx.r[10].s64 + -19268;
	// 82615A7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615A9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615AA0: 4BE51381  bl 0x82466e20
	ctx.lr = 0x82615AA4;
	sub_82466E20(ctx, base);
	// 82615AA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615AB8 size=108
    let mut pc: u32 = 0x82615AB8;
    'dispatch: loop {
        match pc {
            0x82615AB8 => {
    //   block [0x82615AB8..0x82615B24)
	// 82615AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615AC4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615ACC: 38EB8A50  addi r7, r11, -0x75b0
	ctx.r[7].s64 = ctx.r[11].s64 + -30128;
	// 82615AD0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 82615AD4: 388A3A14  addi r4, r10, 0x3a14
	ctx.r[4].s64 = ctx.r[10].s64 + 14868;
	// 82615AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615ADC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615AE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615AE8: 386AB4EC  addi r3, r10, -0x4b14
	ctx.r[3].s64 = ctx.r[10].s64 + -19220;
	// 82615AEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615B0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615B10: 4BE51311  bl 0x82466e20
	ctx.lr = 0x82615B14;
	sub_82466E20(ctx, base);
	// 82615B14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615B18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615B1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615B28 size=108
    let mut pc: u32 = 0x82615B28;
    'dispatch: loop {
        match pc {
            0x82615B28 => {
    //   block [0x82615B28..0x82615B94)
	// 82615B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615B34: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615B38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615B3C: 38EB8B88  addi r7, r11, -0x7478
	ctx.r[7].s64 = ctx.r[11].s64 + -29816;
	// 82615B40: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82615B44: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 82615B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615B4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615B50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615B58: 386AB51C  addi r3, r10, -0x4ae4
	ctx.r[3].s64 = ctx.r[10].s64 + -19172;
	// 82615B5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615B7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615B80: 4BE512A1  bl 0x82466e20
	ctx.lr = 0x82615B84;
	sub_82466E20(ctx, base);
	// 82615B84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615B98 size=116
    let mut pc: u32 = 0x82615B98;
    'dispatch: loop {
        match pc {
            0x82615B98 => {
    //   block [0x82615B98..0x82615C0C)
	// 82615B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615BA4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615BA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82615BAC: 390B8BE8  addi r8, r11, -0x7418
	ctx.r[8].s64 = ctx.r[11].s64 + -29720;
	// 82615BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615BB4: 392AF894  addi r9, r10, -0x76c
	ctx.r[9].s64 = ctx.r[10].s64 + -1900;
	// 82615BB8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615BBC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82615BC0: 38AAB51C  addi r5, r10, -0x4ae4
	ctx.r[5].s64 = ctx.r[10].s64 + -19172;
	// 82615BC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82615BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615BCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615BDC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82615BE0: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 82615BE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82615BE8: 386BB54C  addi r3, r11, -0x4ab4
	ctx.r[3].s64 = ctx.r[11].s64 + -19124;
	// 82615BEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82615BF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615BF8: 4BE51229  bl 0x82466e20
	ctx.lr = 0x82615BFC;
	sub_82466E20(ctx, base);
	// 82615BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615C10 size=96
    let mut pc: u32 = 0x82615C10;
    'dispatch: loop {
        match pc {
            0x82615C10 => {
    //   block [0x82615C10..0x82615C70)
	// 82615C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615C1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615C24: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 82615C28: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615C30: 386AB57C  addi r3, r10, -0x4a84
	ctx.r[3].s64 = ctx.r[10].s64 + -19076;
	// 82615C34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615C3C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82615C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615C50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82615C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615C58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82615C5C: 4BE511C5  bl 0x82466e20
	ctx.lr = 0x82615C60;
	sub_82466E20(ctx, base);
	// 82615C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615C70 size=112
    let mut pc: u32 = 0x82615C70;
    'dispatch: loop {
        match pc {
            0x82615C70 => {
    //   block [0x82615C70..0x82615CE0)
	// 82615C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615C7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615C80: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615C84: 38AAD70C  addi r5, r10, -0x28f4
	ctx.r[5].s64 = ctx.r[10].s64 + -10484;
	// 82615C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615C8C: 390B8C60  addi r8, r11, -0x73a0
	ctx.r[8].s64 = ctx.r[11].s64 + -29600;
	// 82615C90: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82615C94: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 82615C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615C9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82615CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615CA8: 386AB5AC  addi r3, r10, -0x4a54
	ctx.r[3].s64 = ctx.r[10].s64 + -19028;
	// 82615CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615CCC: 4BE51155  bl 0x82466e20
	ctx.lr = 0x82615CD0;
	sub_82466E20(ctx, base);
	// 82615CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615CE0 size=96
    let mut pc: u32 = 0x82615CE0;
    'dispatch: loop {
        match pc {
            0x82615CE0 => {
    //   block [0x82615CE0..0x82615D40)
	// 82615CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615CEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615CF4: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82615CF8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615D00: 386AB5DC  addi r3, r10, -0x4a24
	ctx.r[3].s64 = ctx.r[10].s64 + -18980;
	// 82615D04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615D0C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82615D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615D20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82615D24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615D28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82615D2C: 4BE510F5  bl 0x82466e20
	ctx.lr = 0x82615D30;
	sub_82466E20(ctx, base);
	// 82615D30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82615D40 size=24
    let mut pc: u32 = 0x82615D40;
    'dispatch: loop {
        match pc {
            0x82615D40 => {
    //   block [0x82615D40..0x82615D58)
	// 82615D40: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615D44: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82615D48: 394ADAE0  addi r10, r10, -0x2520
	ctx.r[10].s64 = ctx.r[10].s64 + -9504;
	// 82615D4C: 816B8CC0  lwz r11, -0x7340(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29504 as u32) ) } as u64;
	// 82615D50: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82615D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615D58 size=116
    let mut pc: u32 = 0x82615D58;
    'dispatch: loop {
        match pc {
            0x82615D58 => {
    //   block [0x82615D58..0x82615DCC)
	// 82615D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615D64: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615D68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82615D6C: 390BDAE0  addi r8, r11, -0x2520
	ctx.r[8].s64 = ctx.r[11].s64 + -9504;
	// 82615D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615D74: 392AF8D0  addi r9, r10, -0x730
	ctx.r[9].s64 = ctx.r[10].s64 + -1840;
	// 82615D78: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615D7C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82615D80: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82615D84: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82615D88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615D8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615D90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615D94: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82615D98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615D9C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82615DA0: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82615DA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82615DA8: 386BB60C  addi r3, r11, -0x49f4
	ctx.r[3].s64 = ctx.r[11].s64 + -18932;
	// 82615DAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82615DB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615DB8: 4BE51069  bl 0x82466e20
	ctx.lr = 0x82615DBC;
	sub_82466E20(ctx, base);
	// 82615DBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615DD0 size=104
    let mut pc: u32 = 0x82615DD0;
    'dispatch: loop {
        match pc {
            0x82615DD0 => {
    //   block [0x82615DD0..0x82615E38)
	// 82615DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615DDC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82615DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615DE4: 392AF8FC  addi r9, r10, -0x704
	ctx.r[9].s64 = ctx.r[10].s64 + -1796;
	// 82615DE8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615DF0: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82615DF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615E04: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 82615E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615E0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615E10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82615E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615E18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82615E1C: 386AB63C  addi r3, r10, -0x49c4
	ctx.r[3].s64 = ctx.r[10].s64 + -18884;
	// 82615E20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82615E24: 4BE50FFD  bl 0x82466e20
	ctx.lr = 0x82615E28;
	sub_82466E20(ctx, base);
	// 82615E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615E38 size=96
    let mut pc: u32 = 0x82615E38;
    'dispatch: loop {
        match pc {
            0x82615E38 => {
    //   block [0x82615E38..0x82615E98)
	// 82615E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615E44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615E4C: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 82615E50: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615E58: 386AB66C  addi r3, r10, -0x4994
	ctx.r[3].s64 = ctx.r[10].s64 + -18836;
	// 82615E5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615E64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82615E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615E78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82615E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615E80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82615E84: 4BE50F9D  bl 0x82466e20
	ctx.lr = 0x82615E88;
	sub_82466E20(ctx, base);
	// 82615E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615E98 size=100
    let mut pc: u32 = 0x82615E98;
    'dispatch: loop {
        match pc {
            0x82615E98 => {
    //   block [0x82615E98..0x82615EFC)
	// 82615E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615EA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615EAC: 38AAB63C  addi r5, r10, -0x49c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18884;
	// 82615EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615EB8: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 82615EBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615ECC: 386AB69C  addi r3, r10, -0x4964
	ctx.r[3].s64 = ctx.r[10].s64 + -18788;
	// 82615ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615ED4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615ED8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82615EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615EE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82615EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615EE8: 4BE50F39  bl 0x82466e20
	ctx.lr = 0x82615EEC;
	sub_82466E20(ctx, base);
	// 82615EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615F00 size=112
    let mut pc: u32 = 0x82615F00;
    'dispatch: loop {
        match pc {
            0x82615F00 => {
    //   block [0x82615F00..0x82615F70)
	// 82615F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615F0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615F10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615F14: 38AAB60C  addi r5, r10, -0x49f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18932;
	// 82615F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615F1C: 390B8CC8  addi r8, r11, -0x7338
	ctx.r[8].s64 = ctx.r[11].s64 + -29496;
	// 82615F20: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82615F24: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 82615F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615F2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615F30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82615F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615F38: 386AB6CC  addi r3, r10, -0x4934
	ctx.r[3].s64 = ctx.r[10].s64 + -18740;
	// 82615F3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615F5C: 4BE50EC5  bl 0x82466e20
	ctx.lr = 0x82615F60;
	sub_82466E20(ctx, base);
	// 82615F60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615F70 size=112
    let mut pc: u32 = 0x82615F70;
    'dispatch: loop {
        match pc {
            0x82615F70 => {
    //   block [0x82615F70..0x82615FE0)
	// 82615F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615F7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615F80: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615F84: 38AAB60C  addi r5, r10, -0x49f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18932;
	// 82615F88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615F8C: 390B8D10  addi r8, r11, -0x72f0
	ctx.r[8].s64 = ctx.r[11].s64 + -29424;
	// 82615F90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82615F94: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 82615F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615F9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615FA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82615FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615FA8: 386AB6FC  addi r3, r10, -0x4904
	ctx.r[3].s64 = ctx.r[10].s64 + -18692;
	// 82615FAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615FB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615FB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615FC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615FC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615FC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615FCC: 4BE50E55  bl 0x82466e20
	ctx.lr = 0x82615FD0;
	sub_82466E20(ctx, base);
	// 82615FD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615FE0 size=100
    let mut pc: u32 = 0x82615FE0;
    'dispatch: loop {
        match pc {
            0x82615FE0 => {
    //   block [0x82615FE0..0x82616044)
	// 82615FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615FEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615FF4: 38AAB60C  addi r5, r10, -0x49f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18932;
	// 82615FF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616000: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 82616004: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261600C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616014: 386AB72C  addi r3, r10, -0x48d4
	ctx.r[3].s64 = ctx.r[10].s64 + -18644;
	// 82616018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261601C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616020: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82616024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616028: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261602C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616030: 4BE50DF1  bl 0x82466e20
	ctx.lr = 0x82616034;
	sub_82466E20(ctx, base);
	// 82616034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261603C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616048 size=96
    let mut pc: u32 = 0x82616048;
    'dispatch: loop {
        match pc {
            0x82616048 => {
    //   block [0x82616048..0x826160A8)
	// 82616048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261604C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616054: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261605C: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82616060: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616068: 386AB75C  addi r3, r10, -0x48a4
	ctx.r[3].s64 = ctx.r[10].s64 + -18596;
	// 8261606C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616074: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82616078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261607C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616088: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261608C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616090: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616094: 4BE50D8D  bl 0x82466e20
	ctx.lr = 0x82616098;
	sub_82466E20(ctx, base);
	// 82616098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261609C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826160A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826160A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826160A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826160A8 size=112
    let mut pc: u32 = 0x826160A8;
    'dispatch: loop {
        match pc {
            0x826160A8 => {
    //   block [0x826160A8..0x82616118)
	// 826160A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826160AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826160B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826160B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826160B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826160BC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826160C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826160C4: 390B8D28  addi r8, r11, -0x72d8
	ctx.r[8].s64 = ctx.r[11].s64 + -29400;
	// 826160C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826160CC: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826160D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826160D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826160D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826160DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826160E0: 386AB78C  addi r3, r10, -0x4874
	ctx.r[3].s64 = ctx.r[10].s64 + -18548;
	// 826160E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826160E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826160EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826160F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826160F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826160F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826160FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616104: 4BE50D1D  bl 0x82466e20
	ctx.lr = 0x82616108;
	sub_82466E20(ctx, base);
	// 82616108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261610C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616118 size=96
    let mut pc: u32 = 0x82616118;
    'dispatch: loop {
        match pc {
            0x82616118 => {
    //   block [0x82616118..0x82616178)
	// 82616118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261611C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616124: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261612C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 82616130: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616138: 386AB7BC  addi r3, r10, -0x4844
	ctx.r[3].s64 = ctx.r[10].s64 + -18500;
	// 8261613C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616144: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82616148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261614C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616158: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261615C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616160: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616164: 4BE50CBD  bl 0x82466e20
	ctx.lr = 0x82616168;
	sub_82466E20(ctx, base);
	// 82616168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261616C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616178 size=112
    let mut pc: u32 = 0x82616178;
    'dispatch: loop {
        match pc {
            0x82616178 => {
    //   block [0x82616178..0x826161E8)
	// 82616178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261617C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616184: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616188: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261618C: 38AAB7BC  addi r5, r10, -0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + -18500;
	// 82616190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616194: 390B8D58  addi r8, r11, -0x72a8
	ctx.r[8].s64 = ctx.r[11].s64 + -29352;
	// 82616198: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261619C: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826161A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826161A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826161A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826161AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826161B0: 386AB7EC  addi r3, r10, -0x4814
	ctx.r[3].s64 = ctx.r[10].s64 + -18452;
	// 826161B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826161B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826161BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826161C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826161C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826161C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826161CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826161D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826161D4: 4BE50C4D  bl 0x82466e20
	ctx.lr = 0x826161D8;
	sub_82466E20(ctx, base);
	// 826161D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826161DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826161E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826161E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826161E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826161E8 size=108
    let mut pc: u32 = 0x826161E8;
    'dispatch: loop {
        match pc {
            0x826161E8 => {
    //   block [0x826161E8..0x82616254)
	// 826161E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826161EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826161F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826161F4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826161F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826161FC: 38EB8D70  addi r7, r11, -0x7290
	ctx.r[7].s64 = ctx.r[11].s64 + -29328;
	// 82616200: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82616204: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 82616208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261620C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616210: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82616214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616218: 386AB81C  addi r3, r10, -0x47e4
	ctx.r[3].s64 = ctx.r[10].s64 + -18404;
	// 8261621C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82616220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261622C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261623C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616240: 4BE50BE1  bl 0x82466e20
	ctx.lr = 0x82616244;
	sub_82466E20(ctx, base);
	// 82616244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261624C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616258 size=112
    let mut pc: u32 = 0x82616258;
    'dispatch: loop {
        match pc {
            0x82616258 => {
    //   block [0x82616258..0x826162C8)
	// 82616258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261625C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616264: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616268: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261626C: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 82616270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616274: 390B8DD0  addi r8, r11, -0x7230
	ctx.r[8].s64 = ctx.r[11].s64 + -29232;
	// 82616278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261627C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 82616280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616284: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261628C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616290: 386AB84C  addi r3, r10, -0x47b4
	ctx.r[3].s64 = ctx.r[10].s64 + -18356;
	// 82616294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261629C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826162A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826162A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826162A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826162AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826162B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826162B4: 4BE50B6D  bl 0x82466e20
	ctx.lr = 0x826162B8;
	sub_82466E20(ctx, base);
	// 826162B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826162BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826162C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826162C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826162C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826162C8 size=112
    let mut pc: u32 = 0x826162C8;
    'dispatch: loop {
        match pc {
            0x826162C8 => {
    //   block [0x826162C8..0x82616338)
	// 826162C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826162CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826162D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826162D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826162D8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826162DC: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 826162E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826162E4: 390B8DE8  addi r8, r11, -0x7218
	ctx.r[8].s64 = ctx.r[11].s64 + -29208;
	// 826162E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826162EC: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 826162F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826162F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826162F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826162FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616300: 386AB87C  addi r3, r10, -0x4784
	ctx.r[3].s64 = ctx.r[10].s64 + -18308;
	// 82616304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261630C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261631C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616324: 4BE50AFD  bl 0x82466e20
	ctx.lr = 0x82616328;
	sub_82466E20(ctx, base);
	// 82616328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261632C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616338 size=100
    let mut pc: u32 = 0x82616338;
    'dispatch: loop {
        match pc {
            0x82616338 => {
    //   block [0x82616338..0x8261639C)
	// 82616338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261633C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616344: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261634C: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 82616350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616358: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8261635C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261636C: 386AB8AC  addi r3, r10, -0x4754
	ctx.r[3].s64 = ctx.r[10].s64 + -18260;
	// 82616370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616374: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616378: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261637C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616380: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616388: 4BE50A99  bl 0x82466e20
	ctx.lr = 0x8261638C;
	sub_82466E20(ctx, base);
	// 8261638C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826163A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826163A0 size=116
    let mut pc: u32 = 0x826163A0;
    'dispatch: loop {
        match pc {
            0x826163A0 => {
    //   block [0x826163A0..0x82616414)
	// 826163A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826163A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826163A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826163AC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826163B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826163B4: 390B8E1C  addi r8, r11, -0x71e4
	ctx.r[8].s64 = ctx.r[11].s64 + -29156;
	// 826163B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826163BC: 392AF928  addi r9, r10, -0x6d8
	ctx.r[9].s64 = ctx.r[10].s64 + -1752;
	// 826163C0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826163C4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826163C8: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 826163CC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826163D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826163D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826163D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826163DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826163E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826163E4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 826163E8: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826163EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826163F0: 386BB8DC  addi r3, r11, -0x4724
	ctx.r[3].s64 = ctx.r[11].s64 + -18212;
	// 826163F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826163F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826163FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616400: 4BE50A21  bl 0x82466e20
	ctx.lr = 0x82616404;
	sub_82466E20(ctx, base);
	// 82616404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261640C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616418 size=112
    let mut pc: u32 = 0x82616418;
    'dispatch: loop {
        match pc {
            0x82616418 => {
    //   block [0x82616418..0x82616488)
	// 82616418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261641C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616428: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261642C: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 82616430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616434: 390B8E4C  addi r8, r11, -0x71b4
	ctx.r[8].s64 = ctx.r[11].s64 + -29108;
	// 82616438: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261643C: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 82616440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616444: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261644C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616450: 386AB90C  addi r3, r10, -0x46f4
	ctx.r[3].s64 = ctx.r[10].s64 + -18164;
	// 82616454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261645C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616464: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82616468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261646C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616474: 4BE509AD  bl 0x82466e20
	ctx.lr = 0x82616478;
	sub_82466E20(ctx, base);
	// 82616478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261647C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616488 size=116
    let mut pc: u32 = 0x82616488;
    'dispatch: loop {
        match pc {
            0x82616488 => {
    //   block [0x82616488..0x826164FC)
	// 82616488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261648C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616494: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616498: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261649C: 390B8E68  addi r8, r11, -0x7198
	ctx.r[8].s64 = ctx.r[11].s64 + -29080;
	// 826164A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826164A4: 392AF954  addi r9, r10, -0x6ac
	ctx.r[9].s64 = ctx.r[10].s64 + -1708;
	// 826164A8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826164AC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826164B0: 38AABF9C  addi r5, r10, -0x4064
	ctx.r[5].s64 = ctx.r[10].s64 + -16484;
	// 826164B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826164B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826164BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826164C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826164C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826164C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826164CC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 826164D0: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826164D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826164D8: 386BB93C  addi r3, r11, -0x46c4
	ctx.r[3].s64 = ctx.r[11].s64 + -18116;
	// 826164DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826164E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826164E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826164E8: 4BE50939  bl 0x82466e20
	ctx.lr = 0x826164EC;
	sub_82466E20(ctx, base);
	// 826164EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826164F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826164F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826164F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616500 size=112
    let mut pc: u32 = 0x82616500;
    'dispatch: loop {
        match pc {
            0x82616500 => {
    //   block [0x82616500..0x82616570)
	// 82616500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261650C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616510: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616514: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 82616518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261651C: 390B8E80  addi r8, r11, -0x7180
	ctx.r[8].s64 = ctx.r[11].s64 + -29056;
	// 82616520: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82616524: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 82616528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261652C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616538: 386AB96C  addi r3, r10, -0x4694
	ctx.r[3].s64 = ctx.r[10].s64 + -18068;
	// 8261653C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261654C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82616550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261655C: 4BE508C5  bl 0x82466e20
	ctx.lr = 0x82616560;
	sub_82466E20(ctx, base);
	// 82616560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261656C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616570 size=112
    let mut pc: u32 = 0x82616570;
    'dispatch: loop {
        match pc {
            0x82616570 => {
    //   block [0x82616570..0x826165E0)
	// 82616570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261657C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616580: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616584: 38AAB90C  addi r5, r10, -0x46f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18164;
	// 82616588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261658C: 390B8EF8  addi r8, r11, -0x7108
	ctx.r[8].s64 = ctx.r[11].s64 + -28936;
	// 82616590: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82616594: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 82616598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261659C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826165A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826165A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826165A8: 386AB99C  addi r3, r10, -0x4664
	ctx.r[3].s64 = ctx.r[10].s64 + -18020;
	// 826165AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826165B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826165B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826165B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826165BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826165C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826165C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826165C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826165CC: 4BE50855  bl 0x82466e20
	ctx.lr = 0x826165D0;
	sub_82466E20(ctx, base);
	// 826165D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826165D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826165D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826165DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826165E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826165E0 size=112
    let mut pc: u32 = 0x826165E0;
    'dispatch: loop {
        match pc {
            0x826165E0 => {
    //   block [0x826165E0..0x82616650)
	// 826165E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826165E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826165E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826165EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826165F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826165F4: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 826165F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826165FC: 390B8F40  addi r8, r11, -0x70c0
	ctx.r[8].s64 = ctx.r[11].s64 + -28864;
	// 82616600: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82616604: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 82616608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261660C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616618: 386AB9CC  addi r3, r10, -0x4634
	ctx.r[3].s64 = ctx.r[10].s64 + -17972;
	// 8261661C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261662C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261663C: 4BE507E5  bl 0x82466e20
	ctx.lr = 0x82616640;
	sub_82466E20(ctx, base);
	// 82616640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261664C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616650 size=112
    let mut pc: u32 = 0x82616650;
    'dispatch: loop {
        match pc {
            0x82616650 => {
    //   block [0x82616650..0x826166C0)
	// 82616650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261665C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616660: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616664: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 82616668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261666C: 390B8F88  addi r8, r11, -0x7078
	ctx.r[8].s64 = ctx.r[11].s64 + -28792;
	// 82616670: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82616674: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 82616678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261667C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616688: 386AB9FC  addi r3, r10, -0x4604
	ctx.r[3].s64 = ctx.r[10].s64 + -17924;
	// 8261668C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261669C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826166A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826166A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826166A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826166AC: 4BE50775  bl 0x82466e20
	ctx.lr = 0x826166B0;
	sub_82466E20(ctx, base);
	// 826166B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826166B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826166B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826166BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826166C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826166C0 size=108
    let mut pc: u32 = 0x826166C0;
    'dispatch: loop {
        match pc {
            0x826166C0 => {
    //   block [0x826166C0..0x8261672C)
	// 826166C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826166C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826166C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826166CC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826166D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826166D4: 38EB8FD0  addi r7, r11, -0x7030
	ctx.r[7].s64 = ctx.r[11].s64 + -28720;
	// 826166D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826166DC: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826166E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826166E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826166E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826166EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826166F0: 386ABA2C  addi r3, r10, -0x45d4
	ctx.r[3].s64 = ctx.r[10].s64 + -17876;
	// 826166F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826166F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826166FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261670C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616718: 4BE50709  bl 0x82466e20
	ctx.lr = 0x8261671C;
	sub_82466E20(ctx, base);
	// 8261671C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616730 size=112
    let mut pc: u32 = 0x82616730;
    'dispatch: loop {
        match pc {
            0x82616730 => {
    //   block [0x82616730..0x826167A0)
	// 82616730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261673C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616740: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616744: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 82616748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261674C: 390B9018  addi r8, r11, -0x6fe8
	ctx.r[8].s64 = ctx.r[11].s64 + -28648;
	// 82616750: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82616754: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 82616758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261675C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616768: 386ABA5C  addi r3, r10, -0x45a4
	ctx.r[3].s64 = ctx.r[10].s64 + -17828;
	// 8261676C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261677C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261678C: 4BE50695  bl 0x82466e20
	ctx.lr = 0x82616790;
	sub_82466E20(ctx, base);
	// 82616790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261679C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826167A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826167A0 size=116
    let mut pc: u32 = 0x826167A0;
    'dispatch: loop {
        match pc {
            0x826167A0 => {
    //   block [0x826167A0..0x82616814)
	// 826167A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826167A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826167A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826167AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826167B0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826167B4: 392BF990  addi r9, r11, -0x670
	ctx.r[9].s64 = ctx.r[11].s64 + -1648;
	// 826167B8: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 826167BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826167C0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826167C4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826167C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826167CC: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826167D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826167D4: 396B9098  addi r11, r11, -0x6f68
	ctx.r[11].s64 = ctx.r[11].s64 + -28520;
	// 826167D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826167DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826167E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826167E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826167E8: 386ABA8C  addi r3, r10, -0x4574
	ctx.r[3].s64 = ctx.r[10].s64 + -17780;
	// 826167EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826167F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826167F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826167F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826167FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616800: 4BE50621  bl 0x82466e20
	ctx.lr = 0x82616804;
	sub_82466E20(ctx, base);
	// 82616804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261680C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82616818 size=36
    let mut pc: u32 = 0x82616818;
    'dispatch: loop {
        match pc {
            0x82616818 => {
    //   block [0x82616818..0x8261683C)
	// 82616818: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261681C: 814B912C  lwz r10, -0x6ed4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28372 as u32) ) } as u64;
	// 82616820: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616824: 396BDB10  addi r11, r11, -0x24f0
	ctx.r[11].s64 = ctx.r[11].s64 + -9456;
	// 82616828: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8261682C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82616830: 814A9094  lwz r10, -0x6f6c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28524 as u32) ) } as u64;
	// 82616834: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82616838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616840 size=108
    let mut pc: u32 = 0x82616840;
    'dispatch: loop {
        match pc {
            0x82616840 => {
    //   block [0x82616840..0x826168AC)
	// 82616840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261684C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616854: 38EBDB10  addi r7, r11, -0x24f0
	ctx.r[7].s64 = ctx.r[11].s64 + -9456;
	// 82616858: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8261685C: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 82616860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616864: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261686C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616870: 386ABABC  addi r3, r10, -0x4544
	ctx.r[3].s64 = ctx.r[10].s64 + -17732;
	// 82616874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82616878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261687C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261688C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616898: 4BE50589  bl 0x82466e20
	ctx.lr = 0x8261689C;
	sub_82466E20(ctx, base);
	// 8261689C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826168A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826168A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826168A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826168B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826168B0 size=24
    let mut pc: u32 = 0x826168B0;
    'dispatch: loop {
        match pc {
            0x826168B0 => {
    //   block [0x826168B0..0x826168C8)
	// 826168B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826168B4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826168B8: 394ADBB8  addi r10, r10, -0x2448
	ctx.r[10].s64 = ctx.r[10].s64 + -9288;
	// 826168BC: 816B9094  lwz r11, -0x6f6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28524 as u32) ) } as u64;
	// 826168C0: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826168C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826168C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826168C8 size=116
    let mut pc: u32 = 0x826168C8;
    'dispatch: loop {
        match pc {
            0x826168C8 => {
    //   block [0x826168C8..0x8261693C)
	// 826168C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826168CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826168D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826168D4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826168D8: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826168DC: 390ADBB8  addi r8, r10, -0x2448
	ctx.r[8].s64 = ctx.r[10].s64 + -9288;
	// 826168E0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826168E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826168E8: 38AABABC  addi r5, r10, -0x4544
	ctx.r[5].s64 = ctx.r[10].s64 + -17732;
	// 826168EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826168F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826168F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826168F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826168FC: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 82616900: 396BFA4C  addi r11, r11, -0x5b4
	ctx.r[11].s64 = ctx.r[11].s64 + -1460;
	// 82616904: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616908: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261690C: 386ABAEC  addi r3, r10, -0x4514
	ctx.r[3].s64 = ctx.r[10].s64 + -17684;
	// 82616910: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82616914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616918: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261691C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616928: 4BE504F9  bl 0x82466e20
	ctx.lr = 0x8261692C;
	sub_82466E20(ctx, base);
	// 8261692C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616940 size=112
    let mut pc: u32 = 0x82616940;
    'dispatch: loop {
        match pc {
            0x82616940 => {
    //   block [0x82616940..0x826169B0)
	// 82616940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261694C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616950: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616954: 38AABABC  addi r5, r10, -0x4544
	ctx.r[5].s64 = ctx.r[10].s64 + -17732;
	// 82616958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261695C: 390B9130  addi r8, r11, -0x6ed0
	ctx.r[8].s64 = ctx.r[11].s64 + -28368;
	// 82616960: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82616964: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 82616968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261696C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616978: 386ABB1C  addi r3, r10, -0x44e4
	ctx.r[3].s64 = ctx.r[10].s64 + -17636;
	// 8261697C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261698C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261699C: 4BE50485  bl 0x82466e20
	ctx.lr = 0x826169A0;
	sub_82466E20(ctx, base);
	// 826169A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826169A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826169A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826169AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826169B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826169B0 size=24
    let mut pc: u32 = 0x826169B0;
    'dispatch: loop {
        match pc {
            0x826169B0 => {
    //   block [0x826169B0..0x826169C8)
	// 826169B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826169B4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826169B8: 394ADCA8  addi r10, r10, -0x2358
	ctx.r[10].s64 = ctx.r[10].s64 + -9048;
	// 826169BC: 816B98E8  lwz r11, -0x6718(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26392 as u32) ) } as u64;
	// 826169C0: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826169C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826169C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826169C8 size=116
    let mut pc: u32 = 0x826169C8;
    'dispatch: loop {
        match pc {
            0x826169C8 => {
    //   block [0x826169C8..0x82616A3C)
	// 826169C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826169CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826169D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826169D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826169D8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826169DC: 392BFA10  addi r9, r11, -0x5f0
	ctx.r[9].s64 = ctx.r[11].s64 + -1520;
	// 826169E0: 38AAB90C  addi r5, r10, -0x46f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18164;
	// 826169E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826169E8: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826169EC: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 826169F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826169F4: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 826169F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826169FC: 396BDCA8  addi r11, r11, -0x2358
	ctx.r[11].s64 = ctx.r[11].s64 + -9048;
	// 82616A00: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82616A04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616A08: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82616A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616A10: 386ABB4C  addi r3, r10, -0x44b4
	ctx.r[3].s64 = ctx.r[10].s64 + -17588;
	// 82616A14: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82616A18: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82616A1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616A20: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82616A24: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616A28: 4BE503F9  bl 0x82466e20
	ctx.lr = 0x82616A2C;
	sub_82466E20(ctx, base);
	// 82616A2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616A40 size=100
    let mut pc: u32 = 0x82616A40;
    'dispatch: loop {
        match pc {
            0x82616A40 => {
    //   block [0x82616A40..0x82616AA4)
	// 82616A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616A4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616A54: 38AABC9C  addi r5, r10, -0x4364
	ctx.r[5].s64 = ctx.r[10].s64 + -17252;
	// 82616A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616A60: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 82616A64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616A74: 386ABB7C  addi r3, r10, -0x4484
	ctx.r[3].s64 = ctx.r[10].s64 + -17540;
	// 82616A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616A7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616A80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82616A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616A88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616A8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616A90: 4BE50391  bl 0x82466e20
	ctx.lr = 0x82616A94;
	sub_82466E20(ctx, base);
	// 82616A94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616AA8 size=100
    let mut pc: u32 = 0x82616AA8;
    'dispatch: loop {
        match pc {
            0x82616AA8 => {
    //   block [0x82616AA8..0x82616B0C)
	// 82616AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616AB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616ABC: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 82616AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616AC8: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 82616ACC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616ADC: 386ABBAC  addi r3, r10, -0x4454
	ctx.r[3].s64 = ctx.r[10].s64 + -17492;
	// 82616AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616AE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616AE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82616AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616AF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616AF8: 4BE50329  bl 0x82466e20
	ctx.lr = 0x82616AFC;
	sub_82466E20(ctx, base);
	// 82616AFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616B10 size=108
    let mut pc: u32 = 0x82616B10;
    'dispatch: loop {
        match pc {
            0x82616B10 => {
    //   block [0x82616B10..0x82616B7C)
	// 82616B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616B1C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616B24: 38EB91A8  addi r7, r11, -0x6e58
	ctx.r[7].s64 = ctx.r[11].s64 + -28248;
	// 82616B28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82616B2C: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 82616B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616B34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616B38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82616B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616B40: 386ABBDC  addi r3, r10, -0x4424
	ctx.r[3].s64 = ctx.r[10].s64 + -17444;
	// 82616B44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82616B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616B68: 4BE502B9  bl 0x82466e20
	ctx.lr = 0x82616B6C;
	sub_82466E20(ctx, base);
	// 82616B6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616B80 size=112
    let mut pc: u32 = 0x82616B80;
    'dispatch: loop {
        match pc {
            0x82616B80 => {
    //   block [0x82616B80..0x82616BF0)
	// 82616B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616B8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616B90: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616B94: 38AAB90C  addi r5, r10, -0x46f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18164;
	// 82616B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616B9C: 390B9208  addi r8, r11, -0x6df8
	ctx.r[8].s64 = ctx.r[11].s64 + -28152;
	// 82616BA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82616BA4: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 82616BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616BAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616BB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616BB8: 386ABC0C  addi r3, r10, -0x43f4
	ctx.r[3].s64 = ctx.r[10].s64 + -17396;
	// 82616BBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616BDC: 4BE50245  bl 0x82466e20
	ctx.lr = 0x82616BE0;
	sub_82466E20(ctx, base);
	// 82616BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616BF0 size=108
    let mut pc: u32 = 0x82616BF0;
    'dispatch: loop {
        match pc {
            0x82616BF0 => {
    //   block [0x82616BF0..0x82616C5C)
	// 82616BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616BFC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616C04: 38EB9268  addi r7, r11, -0x6d98
	ctx.r[7].s64 = ctx.r[11].s64 + -28056;
	// 82616C08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82616C0C: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 82616C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616C14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616C18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82616C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616C20: 386ABC3C  addi r3, r10, -0x43c4
	ctx.r[3].s64 = ctx.r[10].s64 + -17348;
	// 82616C24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82616C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616C44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616C48: 4BE501D9  bl 0x82466e20
	ctx.lr = 0x82616C4C;
	sub_82466E20(ctx, base);
	// 82616C4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82616C60 size=28
    let mut pc: u32 = 0x82616C60;
    'dispatch: loop {
        match pc {
            0x82616C60 => {
    //   block [0x82616C60..0x82616C7C)
	// 82616C60: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616C64: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82616C68: 394ADD98  addi r10, r10, -0x2268
	ctx.r[10].s64 = ctx.r[10].s64 + -8808;
	// 82616C6C: 816B9280  lwz r11, -0x6d80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28032 as u32) ) } as u64;
	// 82616C70: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82616C74: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82616C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616C80 size=112
    let mut pc: u32 = 0x82616C80;
    'dispatch: loop {
        match pc {
            0x82616C80 => {
    //   block [0x82616C80..0x82616CF0)
	// 82616C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616C8C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82616C90: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 82616C94: 38EADD98  addi r7, r10, -0x2268
	ctx.r[7].s64 = ctx.r[10].s64 + -8808;
	// 82616C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616C9C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82616CA0: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 82616CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616CA8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82616CAC: 396BFB10  addi r11, r11, -0x4f0
	ctx.r[11].s64 = ctx.r[11].s64 + -1264;
	// 82616CB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82616CB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616CB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616CBC: 386ABC6C  addi r3, r10, -0x4394
	ctx.r[3].s64 = ctx.r[10].s64 + -17300;
	// 82616CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616CC4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82616CC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616CCC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82616CD0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616CD4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616CD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616CDC: 4BE50145  bl 0x82466e20
	ctx.lr = 0x82616CE0;
	sub_82466E20(ctx, base);
	// 82616CE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82616CF0 size=24
    let mut pc: u32 = 0x82616CF0;
    'dispatch: loop {
        match pc {
            0x82616CF0 => {
    //   block [0x82616CF0..0x82616D08)
	// 82616CF0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616CF4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82616CF8: 394ADF00  addi r10, r10, -0x2100
	ctx.r[10].s64 = ctx.r[10].s64 + -8448;
	// 82616CFC: 816B98E8  lwz r11, -0x6718(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26392 as u32) ) } as u64;
	// 82616D00: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82616D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616D08 size=116
    let mut pc: u32 = 0x82616D08;
    'dispatch: loop {
        match pc {
            0x82616D08 => {
    //   block [0x82616D08..0x82616D7C)
	// 82616D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616D14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82616D18: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616D1C: 392BFAE4  addi r9, r11, -0x51c
	ctx.r[9].s64 = ctx.r[11].s64 + -1308;
	// 82616D20: 38AAB90C  addi r5, r10, -0x46f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18164;
	// 82616D24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616D28: 38E9006C  addi r7, r9, 0x6c
	ctx.r[7].s64 = ctx.r[9].s64 + 108;
	// 82616D2C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82616D30: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616D34: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 82616D38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616D3C: 396BDF00  addi r11, r11, -0x2100
	ctx.r[11].s64 = ctx.r[11].s64 + -8448;
	// 82616D40: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82616D44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616D48: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82616D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616D50: 386ABC9C  addi r3, r10, -0x4364
	ctx.r[3].s64 = ctx.r[10].s64 + -17252;
	// 82616D54: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82616D58: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82616D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616D60: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82616D64: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616D68: 4BE500B9  bl 0x82466e20
	ctx.lr = 0x82616D6C;
	sub_82466E20(ctx, base);
	// 82616D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616D80 size=112
    let mut pc: u32 = 0x82616D80;
    'dispatch: loop {
        match pc {
            0x82616D80 => {
    //   block [0x82616D80..0x82616DF0)
	// 82616D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616D8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616D90: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616D94: 38AAB8AC  addi r5, r10, -0x4754
	ctx.r[5].s64 = ctx.r[10].s64 + -18260;
	// 82616D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616D9C: 390B9288  addi r8, r11, -0x6d78
	ctx.r[8].s64 = ctx.r[11].s64 + -28024;
	// 82616DA0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82616DA4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 82616DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616DAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616DB8: 386ABCCC  addi r3, r10, -0x4334
	ctx.r[3].s64 = ctx.r[10].s64 + -17204;
	// 82616DBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616DDC: 4BE50045  bl 0x82466e20
	ctx.lr = 0x82616DE0;
	sub_82466E20(ctx, base);
	// 82616DE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82616DF0 size=24
    let mut pc: u32 = 0x82616DF0;
    'dispatch: loop {
        match pc {
            0x82616DF0 => {
    //   block [0x82616DF0..0x82616E08)
	// 82616DF0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616DF4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82616DF8: 394ADFA8  addi r10, r10, -0x2058
	ctx.r[10].s64 = ctx.r[10].s64 + -8280;
	// 82616DFC: 816B98E8  lwz r11, -0x6718(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26392 as u32) ) } as u64;
	// 82616E00: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82616E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616E08 size=116
    let mut pc: u32 = 0x82616E08;
    'dispatch: loop {
        match pc {
            0x82616E08 => {
    //   block [0x82616E08..0x82616E7C)
	// 82616E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616E14: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82616E18: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82616E1C: 390ADFA8  addi r8, r10, -0x2058
	ctx.r[8].s64 = ctx.r[10].s64 + -8280;
	// 82616E20: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616E24: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82616E28: 38AAB8AC  addi r5, r10, -0x4754
	ctx.r[5].s64 = ctx.r[10].s64 + -18260;
	// 82616E2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616E30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82616E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616E3C: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 82616E40: 396BFB70  addi r11, r11, -0x490
	ctx.r[11].s64 = ctx.r[11].s64 + -1168;
	// 82616E44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616E48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82616E4C: 386ABCFC  addi r3, r10, -0x4304
	ctx.r[3].s64 = ctx.r[10].s64 + -17156;
	// 82616E50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82616E54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616E58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82616E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616E68: 4BE4FFB9  bl 0x82466e20
	ctx.lr = 0x82616E6C;
	sub_82466E20(ctx, base);
	// 82616E6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616E80 size=112
    let mut pc: u32 = 0x82616E80;
    'dispatch: loop {
        match pc {
            0x82616E80 => {
    //   block [0x82616E80..0x82616EF0)
	// 82616E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616E8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616E90: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616E94: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82616E98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616E9C: 390B9318  addi r8, r11, -0x6ce8
	ctx.r[8].s64 = ctx.r[11].s64 + -27880;
	// 82616EA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82616EA4: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 82616EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616EAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616EB8: 386ABD2C  addi r3, r10, -0x42d4
	ctx.r[3].s64 = ctx.r[10].s64 + -17108;
	// 82616EBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616ECC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82616ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616EDC: 4BE4FF45  bl 0x82466e20
	ctx.lr = 0x82616EE0;
	sub_82466E20(ctx, base);
	// 82616EE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616EF0 size=108
    let mut pc: u32 = 0x82616EF0;
    'dispatch: loop {
        match pc {
            0x82616EF0 => {
    //   block [0x82616EF0..0x82616F5C)
	// 82616EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616EFC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616F04: 38EB9348  addi r7, r11, -0x6cb8
	ctx.r[7].s64 = ctx.r[11].s64 + -27832;
	// 82616F08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82616F0C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 82616F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616F14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616F18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82616F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616F20: 386ABD5C  addi r3, r10, -0x42a4
	ctx.r[3].s64 = ctx.r[10].s64 + -17060;
	// 82616F24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82616F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616F48: 4BE4FED9  bl 0x82466e20
	ctx.lr = 0x82616F4C;
	sub_82466E20(ctx, base);
	// 82616F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616F60 size=112
    let mut pc: u32 = 0x82616F60;
    'dispatch: loop {
        match pc {
            0x82616F60 => {
    //   block [0x82616F60..0x82616FD0)
	// 82616F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616F6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616F70: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616F74: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 82616F78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616F7C: 390B9378  addi r8, r11, -0x6c88
	ctx.r[8].s64 = ctx.r[11].s64 + -27784;
	// 82616F80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82616F84: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 82616F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616F8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616F90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616F98: 386ABD8C  addi r3, r10, -0x4274
	ctx.r[3].s64 = ctx.r[10].s64 + -17012;
	// 82616F9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616FBC: 4BE4FE65  bl 0x82466e20
	ctx.lr = 0x82616FC0;
	sub_82466E20(ctx, base);
	// 82616FC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616FD0 size=112
    let mut pc: u32 = 0x82616FD0;
    'dispatch: loop {
        match pc {
            0x82616FD0 => {
    //   block [0x82616FD0..0x82617040)
	// 82616FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616FDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616FE0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616FE4: 38AABF9C  addi r5, r10, -0x4064
	ctx.r[5].s64 = ctx.r[10].s64 + -16484;
	// 82616FE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616FEC: 390B93A8  addi r8, r11, -0x6c58
	ctx.r[8].s64 = ctx.r[11].s64 + -27736;
	// 82616FF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82616FF4: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 82616FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616FFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617000: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617008: 386ABDBC  addi r3, r10, -0x4244
	ctx.r[3].s64 = ctx.r[10].s64 + -16964;
	// 8261700C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261701C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261702C: 4BE4FDF5  bl 0x82466e20
	ctx.lr = 0x82617030;
	sub_82466E20(ctx, base);
	// 82617030: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261703C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617040 size=108
    let mut pc: u32 = 0x82617040;
    'dispatch: loop {
        match pc {
            0x82617040 => {
    //   block [0x82617040..0x826170AC)
	// 82617040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261704C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617054: 38EB93D8  addi r7, r11, -0x6c28
	ctx.r[7].s64 = ctx.r[11].s64 + -27688;
	// 82617058: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261705C: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 82617060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617064: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617068: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261706C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617070: 386ABDEC  addi r3, r10, -0x4214
	ctx.r[3].s64 = ctx.r[10].s64 + -16916;
	// 82617074: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82617078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261707C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261708C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617094: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82617098: 4BE4FD89  bl 0x82466e20
	ctx.lr = 0x8261709C;
	sub_82466E20(ctx, base);
	// 8261709C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826170A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826170A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826170A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826170B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826170B0 size=108
    let mut pc: u32 = 0x826170B0;
    'dispatch: loop {
        match pc {
            0x826170B0 => {
    //   block [0x826170B0..0x8261711C)
	// 826170B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826170B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826170B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826170BC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826170C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826170C4: 38EB9420  addi r7, r11, -0x6be0
	ctx.r[7].s64 = ctx.r[11].s64 + -27616;
	// 826170C8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826170CC: 388A0DDC  addi r4, r10, 0xddc
	ctx.r[4].s64 = ctx.r[10].s64 + 3548;
	// 826170D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826170D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826170D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826170DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826170E0: 386ABE1C  addi r3, r10, -0x41e4
	ctx.r[3].s64 = ctx.r[10].s64 + -16868;
	// 826170E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826170E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826170EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826170F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826170F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826170F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826170FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617104: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82617108: 4BE4FD19  bl 0x82466e20
	ctx.lr = 0x8261710C;
	sub_82466E20(ctx, base);
	// 8261710C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617120 size=112
    let mut pc: u32 = 0x82617120;
    'dispatch: loop {
        match pc {
            0x82617120 => {
    //   block [0x82617120..0x82617190)
	// 82617120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261712C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617130: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617134: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 82617138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261713C: 390B9480  addi r8, r11, -0x6b80
	ctx.r[8].s64 = ctx.r[11].s64 + -27520;
	// 82617140: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82617144: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 82617148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261714C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617158: 386ABE4C  addi r3, r10, -0x41b4
	ctx.r[3].s64 = ctx.r[10].s64 + -16820;
	// 8261715C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261716C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261717C: 4BE4FCA5  bl 0x82466e20
	ctx.lr = 0x82617180;
	sub_82466E20(ctx, base);
	// 82617180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261718C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617190 size=100
    let mut pc: u32 = 0x82617190;
    'dispatch: loop {
        match pc {
            0x82617190 => {
    //   block [0x82617190..0x826171F4)
	// 82617190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261719C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826171A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826171A4: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 826171A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826171AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826171B0: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826171B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826171B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826171BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826171C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826171C4: 386ABE7C  addi r3, r10, -0x4184
	ctx.r[3].s64 = ctx.r[10].s64 + -16772;
	// 826171C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826171CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826171D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826171D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826171D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826171DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826171E0: 4BE4FC41  bl 0x82466e20
	ctx.lr = 0x826171E4;
	sub_82466E20(ctx, base);
	// 826171E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826171E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826171EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826171F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826171F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826171F8 size=112
    let mut pc: u32 = 0x826171F8;
    'dispatch: loop {
        match pc {
            0x826171F8 => {
    //   block [0x826171F8..0x82617268)
	// 826171F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826171FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617204: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617208: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261720C: 38AABBAC  addi r5, r10, -0x4454
	ctx.r[5].s64 = ctx.r[10].s64 + -17492;
	// 82617210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617214: 390B94E0  addi r8, r11, -0x6b20
	ctx.r[8].s64 = ctx.r[11].s64 + -27424;
	// 82617218: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261721C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 82617220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617224: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261722C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617230: 386ABEAC  addi r3, r10, -0x4154
	ctx.r[3].s64 = ctx.r[10].s64 + -16724;
	// 82617234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261723C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261724C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617254: 4BE4FBCD  bl 0x82466e20
	ctx.lr = 0x82617258;
	sub_82466E20(ctx, base);
	// 82617258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261725C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617268 size=112
    let mut pc: u32 = 0x82617268;
    'dispatch: loop {
        match pc {
            0x82617268 => {
    //   block [0x82617268..0x826172D8)
	// 82617268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261726C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617274: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617278: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261727C: 38AABBAC  addi r5, r10, -0x4454
	ctx.r[5].s64 = ctx.r[10].s64 + -17492;
	// 82617280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617284: 390B9528  addi r8, r11, -0x6ad8
	ctx.r[8].s64 = ctx.r[11].s64 + -27352;
	// 82617288: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261728C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 82617290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617294: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261729C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826172A0: 386ABEDC  addi r3, r10, -0x4124
	ctx.r[3].s64 = ctx.r[10].s64 + -16676;
	// 826172A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826172A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826172AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826172B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826172B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826172B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826172BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826172C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826172C4: 4BE4FB5D  bl 0x82466e20
	ctx.lr = 0x826172C8;
	sub_82466E20(ctx, base);
	// 826172C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826172CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826172D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826172D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826172D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826172D8 size=108
    let mut pc: u32 = 0x826172D8;
    'dispatch: loop {
        match pc {
            0x826172D8 => {
    //   block [0x826172D8..0x82617344)
	// 826172D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826172DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826172E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826172E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826172E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826172EC: 38EB95D0  addi r7, r11, -0x6a30
	ctx.r[7].s64 = ctx.r[11].s64 + -27184;
	// 826172F0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826172F4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826172F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826172FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82617304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617308: 386ABF0C  addi r3, r10, -0x40f4
	ctx.r[3].s64 = ctx.r[10].s64 + -16628;
	// 8261730C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82617310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261731C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261732C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82617330: 4BE4FAF1  bl 0x82466e20
	ctx.lr = 0x82617334;
	sub_82466E20(ctx, base);
	// 82617334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261733C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82617348 size=24
    let mut pc: u32 = 0x82617348;
    'dispatch: loop {
        match pc {
            0x82617348 => {
    //   block [0x82617348..0x82617360)
	// 82617348: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261734C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82617350: 394AE0E0  addi r10, r10, -0x1f20
	ctx.r[10].s64 = ctx.r[10].s64 + -7968;
	// 82617354: 816B98E8  lwz r11, -0x6718(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26392 as u32) ) } as u64;
	// 82617358: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8261735C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617360 size=116
    let mut pc: u32 = 0x82617360;
    'dispatch: loop {
        match pc {
            0x82617360 => {
    //   block [0x82617360..0x826173D4)
	// 82617360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261736C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82617370: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82617374: 390AE0E0  addi r8, r10, -0x1f20
	ctx.r[8].s64 = ctx.r[10].s64 + -7968;
	// 82617378: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261737C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82617380: 38AAB90C  addi r5, r10, -0x46f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18164;
	// 82617384: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617388: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261738C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617390: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617394: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 82617398: 396BFBA8  addi r11, r11, -0x458
	ctx.r[11].s64 = ctx.r[11].s64 + -1112;
	// 8261739C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826173A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826173A4: 386ABF3C  addi r3, r10, -0x40c4
	ctx.r[3].s64 = ctx.r[10].s64 + -16580;
	// 826173A8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826173AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826173B0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826173B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826173B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826173BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826173C0: 4BE4FA61  bl 0x82466e20
	ctx.lr = 0x826173C4;
	sub_82466E20(ctx, base);
	// 826173C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826173C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826173CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826173D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826173D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826173D8 size=112
    let mut pc: u32 = 0x826173D8;
    'dispatch: loop {
        match pc {
            0x826173D8 => {
    //   block [0x826173D8..0x82617448)
	// 826173D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826173DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826173E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826173E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826173E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826173EC: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 826173F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826173F4: 390B9630  addi r8, r11, -0x69d0
	ctx.r[8].s64 = ctx.r[11].s64 + -27088;
	// 826173F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826173FC: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 82617400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617404: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261740C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617410: 386ABF6C  addi r3, r10, -0x4094
	ctx.r[3].s64 = ctx.r[10].s64 + -16532;
	// 82617414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261741C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261742C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617434: 4BE4F9ED  bl 0x82466e20
	ctx.lr = 0x82617438;
	sub_82466E20(ctx, base);
	// 82617438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261743C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617448 size=100
    let mut pc: u32 = 0x82617448;
    'dispatch: loop {
        match pc {
            0x82617448 => {
    //   block [0x82617448..0x826174AC)
	// 82617448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261744C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617454: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261745C: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 82617460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617468: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8261746C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261747C: 386ABF9C  addi r3, r10, -0x4064
	ctx.r[3].s64 = ctx.r[10].s64 + -16484;
	// 82617480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617484: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617488: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261748C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617490: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82617494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617498: 4BE4F989  bl 0x82466e20
	ctx.lr = 0x8261749C;
	sub_82466E20(ctx, base);
	// 8261749C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826174A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826174A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826174A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826174B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826174B0 size=112
    let mut pc: u32 = 0x826174B0;
    'dispatch: loop {
        match pc {
            0x826174B0 => {
    //   block [0x826174B0..0x82617520)
	// 826174B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826174B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826174B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826174BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826174C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826174C4: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826174C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826174CC: 390B9648  addi r8, r11, -0x69b8
	ctx.r[8].s64 = ctx.r[11].s64 + -27064;
	// 826174D0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826174D4: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 826174D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826174DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826174E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826174E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826174E8: 386ABFCC  addi r3, r10, -0x4034
	ctx.r[3].s64 = ctx.r[10].s64 + -16436;
	// 826174EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826174F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826174F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826174F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826174FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261750C: 4BE4F915  bl 0x82466e20
	ctx.lr = 0x82617510;
	sub_82466E20(ctx, base);
	// 82617510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261751C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617520 size=112
    let mut pc: u32 = 0x82617520;
    'dispatch: loop {
        match pc {
            0x82617520 => {
    //   block [0x82617520..0x82617590)
	// 82617520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261752C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617530: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617534: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82617538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261753C: 390B96D8  addi r8, r11, -0x6928
	ctx.r[8].s64 = ctx.r[11].s64 + -26920;
	// 82617540: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82617544: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 82617548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261754C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617550: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617558: 386ABFFC  addi r3, r10, -0x4004
	ctx.r[3].s64 = ctx.r[10].s64 + -16388;
	// 8261755C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261756C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261757C: 4BE4F8A5  bl 0x82466e20
	ctx.lr = 0x82617580;
	sub_82466E20(ctx, base);
	// 82617580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261758C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617590 size=112
    let mut pc: u32 = 0x82617590;
    'dispatch: loop {
        match pc {
            0x82617590 => {
    //   block [0x82617590..0x82617600)
	// 82617590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261759C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826175A0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826175A4: 38AABB4C  addi r5, r10, -0x44b4
	ctx.r[5].s64 = ctx.r[10].s64 + -17588;
	// 826175A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826175AC: 390B9738  addi r8, r11, -0x68c8
	ctx.r[8].s64 = ctx.r[11].s64 + -26824;
	// 826175B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826175B4: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 826175B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826175BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826175C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826175C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826175C8: 386AC02C  addi r3, r10, -0x3fd4
	ctx.r[3].s64 = ctx.r[10].s64 + -16340;
	// 826175CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826175D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826175D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826175D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826175DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826175E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826175E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826175E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826175EC: 4BE4F835  bl 0x82466e20
	ctx.lr = 0x826175F0;
	sub_82466E20(ctx, base);
	// 826175F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826175F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826175F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826175FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617600 size=112
    let mut pc: u32 = 0x82617600;
    'dispatch: loop {
        match pc {
            0x82617600 => {
    //   block [0x82617600..0x82617670)
	// 82617600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261760C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617610: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617614: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82617618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261761C: 390B9768  addi r8, r11, -0x6898
	ctx.r[8].s64 = ctx.r[11].s64 + -26776;
	// 82617620: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82617624: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 82617628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261762C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617630: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617638: 386AC05C  addi r3, r10, -0x3fa4
	ctx.r[3].s64 = ctx.r[10].s64 + -16292;
	// 8261763C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261764C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261765C: 4BE4F7C5  bl 0x82466e20
	ctx.lr = 0x82617660;
	sub_82466E20(ctx, base);
	// 82617660: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261766C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617670 size=112
    let mut pc: u32 = 0x82617670;
    'dispatch: loop {
        match pc {
            0x82617670 => {
    //   block [0x82617670..0x826176E0)
	// 82617670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261767C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617680: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617684: 38AABC9C  addi r5, r10, -0x4364
	ctx.r[5].s64 = ctx.r[10].s64 + -17252;
	// 82617688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261768C: 390B97F8  addi r8, r11, -0x6808
	ctx.r[8].s64 = ctx.r[11].s64 + -26632;
	// 82617690: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82617694: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 82617698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261769C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826176A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826176A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826176A8: 386AC08C  addi r3, r10, -0x3f74
	ctx.r[3].s64 = ctx.r[10].s64 + -16244;
	// 826176AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826176B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826176B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826176B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826176BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826176C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826176C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826176C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826176CC: 4BE4F755  bl 0x82466e20
	ctx.lr = 0x826176D0;
	sub_82466E20(ctx, base);
	// 826176D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826176D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826176D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826176DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826176E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826176E0 size=112
    let mut pc: u32 = 0x826176E0;
    'dispatch: loop {
        match pc {
            0x826176E0 => {
    //   block [0x826176E0..0x82617750)
	// 826176E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826176E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826176E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826176EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826176F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826176F4: 38AABEDC  addi r5, r10, -0x4124
	ctx.r[5].s64 = ctx.r[10].s64 + -16676;
	// 826176F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826176FC: 390B9810  addi r8, r11, -0x67f0
	ctx.r[8].s64 = ctx.r[11].s64 + -26608;
	// 82617700: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82617704: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 82617708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261770C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617718: 386AC0BC  addi r3, r10, -0x3f44
	ctx.r[3].s64 = ctx.r[10].s64 + -16196;
	// 8261771C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261772C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261773C: 4BE4F6E5  bl 0x82466e20
	ctx.lr = 0x82617740;
	sub_82466E20(ctx, base);
	// 82617740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261774C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617750 size=112
    let mut pc: u32 = 0x82617750;
    'dispatch: loop {
        match pc {
            0x82617750 => {
    //   block [0x82617750..0x826177C0)
	// 82617750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261775C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617760: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617764: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 82617768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261776C: 390B9840  addi r8, r11, -0x67c0
	ctx.r[8].s64 = ctx.r[11].s64 + -26560;
	// 82617770: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82617774: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 82617778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261777C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617788: 386AC0EC  addi r3, r10, -0x3f14
	ctx.r[3].s64 = ctx.r[10].s64 + -16148;
	// 8261778C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261779C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826177A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826177A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826177A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826177AC: 4BE4F675  bl 0x82466e20
	ctx.lr = 0x826177B0;
	sub_82466E20(ctx, base);
	// 826177B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826177B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826177B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826177BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826177C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826177C0 size=24
    let mut pc: u32 = 0x826177C0;
    'dispatch: loop {
        match pc {
            0x826177C0 => {
    //   block [0x826177C0..0x826177D8)
	// 826177C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826177C4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826177C8: 394AE158  addi r10, r10, -0x1ea8
	ctx.r[10].s64 = ctx.r[10].s64 + -7848;
	// 826177CC: 816B98E8  lwz r11, -0x6718(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26392 as u32) ) } as u64;
	// 826177D0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826177D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826177D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826177D8 size=116
    let mut pc: u32 = 0x826177D8;
    'dispatch: loop {
        match pc {
            0x826177D8 => {
    //   block [0x826177D8..0x8261784C)
	// 826177D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826177DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826177E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826177E4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826177E8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826177EC: 390AE158  addi r8, r10, -0x1ea8
	ctx.r[8].s64 = ctx.r[10].s64 + -7848;
	// 826177F0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826177F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826177F8: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 826177FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617800: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82617804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261780C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 82617810: 396BFBC0  addi r11, r11, -0x440
	ctx.r[11].s64 = ctx.r[11].s64 + -1088;
	// 82617814: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617818: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261781C: 386AC11C  addi r3, r10, -0x3ee4
	ctx.r[3].s64 = ctx.r[10].s64 + -16100;
	// 82617820: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82617824: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617828: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261782C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617838: 4BE4F5E9  bl 0x82466e20
	ctx.lr = 0x8261783C;
	sub_82466E20(ctx, base);
	// 8261783C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617850 size=112
    let mut pc: u32 = 0x82617850;
    'dispatch: loop {
        match pc {
            0x82617850 => {
    //   block [0x82617850..0x826178C0)
	// 82617850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261785C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617860: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617864: 38AAB8AC  addi r5, r10, -0x4754
	ctx.r[5].s64 = ctx.r[10].s64 + -18260;
	// 82617868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261786C: 390B9888  addi r8, r11, -0x6778
	ctx.r[8].s64 = ctx.r[11].s64 + -26488;
	// 82617870: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82617874: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 82617878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261787C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617888: 386AC14C  addi r3, r10, -0x3eb4
	ctx.r[3].s64 = ctx.r[10].s64 + -16052;
	// 8261788C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261789C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826178A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826178A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826178A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826178AC: 4BE4F575  bl 0x82466e20
	ctx.lr = 0x826178B0;
	sub_82466E20(ctx, base);
	// 826178B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826178B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826178B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826178BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826178C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826178C0 size=112
    let mut pc: u32 = 0x826178C0;
    'dispatch: loop {
        match pc {
            0x826178C0 => {
    //   block [0x826178C0..0x82617930)
	// 826178C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826178C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826178C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826178CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826178D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826178D4: 38AAB90C  addi r5, r10, -0x46f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18164;
	// 826178D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826178DC: 390B98B8  addi r8, r11, -0x6748
	ctx.r[8].s64 = ctx.r[11].s64 + -26440;
	// 826178E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826178E4: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826178E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826178EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826178F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826178F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826178F8: 386AC17C  addi r3, r10, -0x3e84
	ctx.r[3].s64 = ctx.r[10].s64 + -16004;
	// 826178FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261790C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261791C: 4BE4F505  bl 0x82466e20
	ctx.lr = 0x82617920;
	sub_82466E20(ctx, base);
	// 82617920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261792C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617930 size=100
    let mut pc: u32 = 0x82617930;
    'dispatch: loop {
        match pc {
            0x82617930 => {
    //   block [0x82617930..0x82617994)
	// 82617930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261793C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82617940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617944: 392AFC30  addi r9, r10, -0x3d0
	ctx.r[9].s64 = ctx.r[10].s64 + -976;
	// 82617948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261794C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617950: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 82617954: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261795C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617964: 386AC1AC  addi r3, r10, -0x3e54
	ctx.r[3].s64 = ctx.r[10].s64 + -15956;
	// 82617968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261796C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82617970: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82617974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617978: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261797C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82617980: 4BE4F4A1  bl 0x82466e20
	ctx.lr = 0x82617984;
	sub_82466E20(ctx, base);
	// 82617984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261798C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82617998 size=24
    let mut pc: u32 = 0x82617998;
    'dispatch: loop {
        match pc {
            0x82617998 => {
    //   block [0x82617998..0x826179B0)
	// 82617998: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261799C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826179A0: 394AE200  addi r10, r10, -0x1e00
	ctx.r[10].s64 = ctx.r[10].s64 + -7680;
	// 826179A4: 816B98F4  lwz r11, -0x670c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26380 as u32) ) } as u64;
	// 826179A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826179AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826179B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826179B0 size=112
    let mut pc: u32 = 0x826179B0;
    'dispatch: loop {
        match pc {
            0x826179B0 => {
    //   block [0x826179B0..0x82617A20)
	// 826179B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826179B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826179B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826179BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826179C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826179C4: 392AFD70  addi r9, r10, -0x290
	ctx.r[9].s64 = ctx.r[10].s64 + -656;
	// 826179C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826179CC: 390BE200  addi r8, r11, -0x1e00
	ctx.r[8].s64 = ctx.r[11].s64 + -7680;
	// 826179D0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826179D4: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 826179D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826179DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826179E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826179E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826179E8: 386AC1DC  addi r3, r10, -0x3e24
	ctx.r[3].s64 = ctx.r[10].s64 + -15908;
	// 826179EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826179F0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826179F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826179F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826179FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617A04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82617A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617A0C: 4BE4F415  bl 0x82466e20
	ctx.lr = 0x82617A10;
	sub_82466E20(ctx, base);
	// 82617A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617A20 size=112
    let mut pc: u32 = 0x82617A20;
    'dispatch: loop {
        match pc {
            0x82617A20 => {
    //   block [0x82617A20..0x82617A90)
	// 82617A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617A2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617A30: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617A34: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617A3C: 390B98FC  addi r8, r11, -0x6704
	ctx.r[8].s64 = ctx.r[11].s64 + -26372;
	// 82617A40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82617A44: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 82617A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617A4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617A50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617A58: 386AC20C  addi r3, r10, -0x3df4
	ctx.r[3].s64 = ctx.r[10].s64 + -15860;
	// 82617A5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617A7C: 4BE4F3A5  bl 0x82466e20
	ctx.lr = 0x82617A80;
	sub_82466E20(ctx, base);
	// 82617A80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617A90 size=108
    let mut pc: u32 = 0x82617A90;
    'dispatch: loop {
        match pc {
            0x82617A90 => {
    //   block [0x82617A90..0x82617AFC)
	// 82617A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617A9C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617AA4: 38EB992C  addi r7, r11, -0x66d4
	ctx.r[7].s64 = ctx.r[11].s64 + -26324;
	// 82617AA8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82617AAC: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 82617AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617AB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617AB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82617ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617AC0: 386AC23C  addi r3, r10, -0x3dc4
	ctx.r[3].s64 = ctx.r[10].s64 + -15812;
	// 82617AC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82617AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617AE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82617AE8: 4BE4F339  bl 0x82466e20
	ctx.lr = 0x82617AEC;
	sub_82466E20(ctx, base);
	// 82617AEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617B00 size=112
    let mut pc: u32 = 0x82617B00;
    'dispatch: loop {
        match pc {
            0x82617B00 => {
    //   block [0x82617B00..0x82617B70)
	// 82617B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617B0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617B10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617B14: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617B18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82617B1C: 390B9948  addi r8, r11, -0x66b8
	ctx.r[8].s64 = ctx.r[11].s64 + -26296;
	// 82617B20: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82617B24: 388A0E04  addi r4, r10, 0xe04
	ctx.r[4].s64 = ctx.r[10].s64 + 3588;
	// 82617B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617B2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617B30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617B38: 386AC26C  addi r3, r10, -0x3d94
	ctx.r[3].s64 = ctx.r[10].s64 + -15764;
	// 82617B3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617B5C: 4BE4F2C5  bl 0x82466e20
	ctx.lr = 0x82617B60;
	sub_82466E20(ctx, base);
	// 82617B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617B70 size=100
    let mut pc: u32 = 0x82617B70;
    'dispatch: loop {
        match pc {
            0x82617B70 => {
    //   block [0x82617B70..0x82617BD4)
	// 82617B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617B7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617B84: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617B88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617B90: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 82617B94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617BA4: 386AC29C  addi r3, r10, -0x3d64
	ctx.r[3].s64 = ctx.r[10].s64 + -15716;
	// 82617BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617BAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617BB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82617BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617BB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82617BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617BC0: 4BE4F261  bl 0x82466e20
	ctx.lr = 0x82617BC4;
	sub_82466E20(ctx, base);
	// 82617BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617BD8 size=112
    let mut pc: u32 = 0x82617BD8;
    'dispatch: loop {
        match pc {
            0x82617BD8 => {
    //   block [0x82617BD8..0x82617C48)
	// 82617BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617BE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617BE8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617BEC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617BF4: 390B99A8  addi r8, r11, -0x6658
	ctx.r[8].s64 = ctx.r[11].s64 + -26200;
	// 82617BF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82617BFC: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 82617C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617C04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617C08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617C10: 386AC2CC  addi r3, r10, -0x3d34
	ctx.r[3].s64 = ctx.r[10].s64 + -15668;
	// 82617C14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617C34: 4BE4F1ED  bl 0x82466e20
	ctx.lr = 0x82617C38;
	sub_82466E20(ctx, base);
	// 82617C38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617C48 size=112
    let mut pc: u32 = 0x82617C48;
    'dispatch: loop {
        match pc {
            0x82617C48 => {
    //   block [0x82617C48..0x82617CB8)
	// 82617C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617C54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617C58: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617C5C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617C64: 390B99C0  addi r8, r11, -0x6640
	ctx.r[8].s64 = ctx.r[11].s64 + -26176;
	// 82617C68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82617C6C: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 82617C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617C74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617C78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617C80: 386AC2FC  addi r3, r10, -0x3d04
	ctx.r[3].s64 = ctx.r[10].s64 + -15620;
	// 82617C84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617CA4: 4BE4F17D  bl 0x82466e20
	ctx.lr = 0x82617CA8;
	sub_82466E20(ctx, base);
	// 82617CA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617CB8 size=112
    let mut pc: u32 = 0x82617CB8;
    'dispatch: loop {
        match pc {
            0x82617CB8 => {
    //   block [0x82617CB8..0x82617D28)
	// 82617CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617CC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617CC8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617CCC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617CD4: 390B99F0  addi r8, r11, -0x6610
	ctx.r[8].s64 = ctx.r[11].s64 + -26128;
	// 82617CD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82617CDC: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 82617CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617CE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617CE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617CF0: 386AC32C  addi r3, r10, -0x3cd4
	ctx.r[3].s64 = ctx.r[10].s64 + -15572;
	// 82617CF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617D14: 4BE4F10D  bl 0x82466e20
	ctx.lr = 0x82617D18;
	sub_82466E20(ctx, base);
	// 82617D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617D28 size=112
    let mut pc: u32 = 0x82617D28;
    'dispatch: loop {
        match pc {
            0x82617D28 => {
    //   block [0x82617D28..0x82617D98)
	// 82617D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617D34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617D38: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617D3C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617D44: 390B9A20  addi r8, r11, -0x65e0
	ctx.r[8].s64 = ctx.r[11].s64 + -26080;
	// 82617D48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82617D4C: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 82617D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617D54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617D58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617D60: 386AC35C  addi r3, r10, -0x3ca4
	ctx.r[3].s64 = ctx.r[10].s64 + -15524;
	// 82617D64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617D84: 4BE4F09D  bl 0x82466e20
	ctx.lr = 0x82617D88;
	sub_82466E20(ctx, base);
	// 82617D88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617D98 size=112
    let mut pc: u32 = 0x82617D98;
    'dispatch: loop {
        match pc {
            0x82617D98 => {
    //   block [0x82617D98..0x82617E08)
	// 82617D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617DA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617DA8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617DAC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617DB4: 390B9A50  addi r8, r11, -0x65b0
	ctx.r[8].s64 = ctx.r[11].s64 + -26032;
	// 82617DB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82617DBC: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 82617DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617DC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617DC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617DD0: 386AC38C  addi r3, r10, -0x3c74
	ctx.r[3].s64 = ctx.r[10].s64 + -15476;
	// 82617DD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617DDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617DF4: 4BE4F02D  bl 0x82466e20
	ctx.lr = 0x82617DF8;
	sub_82466E20(ctx, base);
	// 82617DF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617E08 size=112
    let mut pc: u32 = 0x82617E08;
    'dispatch: loop {
        match pc {
            0x82617E08 => {
    //   block [0x82617E08..0x82617E78)
	// 82617E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617E14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617E18: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617E1C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617E24: 390B9A68  addi r8, r11, -0x6598
	ctx.r[8].s64 = ctx.r[11].s64 + -26008;
	// 82617E28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82617E2C: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 82617E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617E34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617E40: 386AC3BC  addi r3, r10, -0x3c44
	ctx.r[3].s64 = ctx.r[10].s64 + -15428;
	// 82617E44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617E48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617E64: 4BE4EFBD  bl 0x82466e20
	ctx.lr = 0x82617E68;
	sub_82466E20(ctx, base);
	// 82617E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617E78 size=112
    let mut pc: u32 = 0x82617E78;
    'dispatch: loop {
        match pc {
            0x82617E78 => {
    //   block [0x82617E78..0x82617EE8)
	// 82617E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617E84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617E88: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617E8C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617E94: 390B9A80  addi r8, r11, -0x6580
	ctx.r[8].s64 = ctx.r[11].s64 + -25984;
	// 82617E98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82617E9C: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 82617EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617EA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617EB0: 386AC3EC  addi r3, r10, -0x3c14
	ctx.r[3].s64 = ctx.r[10].s64 + -15380;
	// 82617EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617ED4: 4BE4EF4D  bl 0x82466e20
	ctx.lr = 0x82617ED8;
	sub_82466E20(ctx, base);
	// 82617ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617EE8 size=112
    let mut pc: u32 = 0x82617EE8;
    'dispatch: loop {
        match pc {
            0x82617EE8 => {
    //   block [0x82617EE8..0x82617F58)
	// 82617EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617EF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617EF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617EFC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617F04: 390B9AC8  addi r8, r11, -0x6538
	ctx.r[8].s64 = ctx.r[11].s64 + -25912;
	// 82617F08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82617F0C: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 82617F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617F14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617F18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617F20: 386AC41C  addi r3, r10, -0x3be4
	ctx.r[3].s64 = ctx.r[10].s64 + -15332;
	// 82617F24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617F44: 4BE4EEDD  bl 0x82466e20
	ctx.lr = 0x82617F48;
	sub_82466E20(ctx, base);
	// 82617F48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617F58 size=112
    let mut pc: u32 = 0x82617F58;
    'dispatch: loop {
        match pc {
            0x82617F58 => {
    //   block [0x82617F58..0x82617FC8)
	// 82617F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617F64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617F68: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617F6C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617F74: 390B9B10  addi r8, r11, -0x64f0
	ctx.r[8].s64 = ctx.r[11].s64 + -25840;
	// 82617F78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82617F7C: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 82617F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617F84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617F88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617F90: 386AC44C  addi r3, r10, -0x3bb4
	ctx.r[3].s64 = ctx.r[10].s64 + -15284;
	// 82617F94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617FB4: 4BE4EE6D  bl 0x82466e20
	ctx.lr = 0x82617FB8;
	sub_82466E20(ctx, base);
	// 82617FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617FC8 size=112
    let mut pc: u32 = 0x82617FC8;
    'dispatch: loop {
        match pc {
            0x82617FC8 => {
    //   block [0x82617FC8..0x82618038)
	// 82617FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617FD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617FD8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617FDC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617FE4: 390B9B28  addi r8, r11, -0x64d8
	ctx.r[8].s64 = ctx.r[11].s64 + -25816;
	// 82617FE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82617FEC: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 82617FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617FF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617FF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618000: 386AC47C  addi r3, r10, -0x3b84
	ctx.r[3].s64 = ctx.r[10].s64 + -15236;
	// 82618004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261800C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261801C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618024: 4BE4EDFD  bl 0x82466e20
	ctx.lr = 0x82618028;
	sub_82466E20(ctx, base);
	// 82618028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261802C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618038 size=116
    let mut pc: u32 = 0x82618038;
    'dispatch: loop {
        match pc {
            0x82618038 => {
    //   block [0x82618038..0x826180AC)
	// 82618038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261803C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618044: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82618048: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8261804C: 390A9B58  addi r8, r10, -0x64a8
	ctx.r[8].s64 = ctx.r[10].s64 + -25768;
	// 82618050: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618054: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82618058: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 8261805C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618060: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82618064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261806C: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 82618070: 396BFD98  addi r11, r11, -0x268
	ctx.r[11].s64 = ctx.r[11].s64 + -616;
	// 82618074: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618078: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261807C: 386AC4AC  addi r3, r10, -0x3b54
	ctx.r[3].s64 = ctx.r[10].s64 + -15188;
	// 82618080: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82618084: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618088: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261808C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618098: 4BE4ED89  bl 0x82466e20
	ctx.lr = 0x8261809C;
	sub_82466E20(ctx, base);
	// 8261809C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826180A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826180A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826180A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826180B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826180B0 size=116
    let mut pc: u32 = 0x826180B0;
    'dispatch: loop {
        match pc {
            0x826180B0 => {
    //   block [0x826180B0..0x82618124)
	// 826180B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826180B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826180B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826180BC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826180C0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826180C4: 390A9BD0  addi r8, r10, -0x6430
	ctx.r[8].s64 = ctx.r[10].s64 + -25648;
	// 826180C8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826180CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826180D0: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 826180D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826180D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826180DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826180E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826180E4: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 826180E8: 396BFDB0  addi r11, r11, -0x250
	ctx.r[11].s64 = ctx.r[11].s64 + -592;
	// 826180EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826180F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826180F4: 386AC4DC  addi r3, r10, -0x3b24
	ctx.r[3].s64 = ctx.r[10].s64 + -15140;
	// 826180F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826180FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618100: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82618104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261810C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618110: 4BE4ED11  bl 0x82466e20
	ctx.lr = 0x82618114;
	sub_82466E20(ctx, base);
	// 82618114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261811C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82618128 size=24
    let mut pc: u32 = 0x82618128;
    'dispatch: loop {
        match pc {
            0x82618128 => {
    //   block [0x82618128..0x82618140)
	// 82618128: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261812C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82618130: 394AE218  addi r10, r10, -0x1de8
	ctx.r[10].s64 = ctx.r[10].s64 + -7656;
	// 82618134: 816B9944  lwz r11, -0x66bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26300 as u32) ) } as u64;
	// 82618138: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8261813C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618140 size=116
    let mut pc: u32 = 0x82618140;
    'dispatch: loop {
        match pc {
            0x82618140 => {
    //   block [0x82618140..0x826181B4)
	// 82618140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261814C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82618150: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618154: 392BFDDC  addi r9, r11, -0x224
	ctx.r[9].s64 = ctx.r[11].s64 + -548;
	// 82618158: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 8261815C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618160: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82618164: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82618168: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261816C: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 82618170: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618174: 396BE218  addi r11, r11, -0x1de8
	ctx.r[11].s64 = ctx.r[11].s64 + -7656;
	// 82618178: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8261817C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618180: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82618184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618188: 386AC50C  addi r3, r10, -0x3af4
	ctx.r[3].s64 = ctx.r[10].s64 + -15092;
	// 8261818C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82618190: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82618194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618198: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8261819C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826181A0: 4BE4EC81  bl 0x82466e20
	ctx.lr = 0x826181A4;
	sub_82466E20(ctx, base);
	// 826181A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826181A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826181AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826181B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826181B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826181B8 size=112
    let mut pc: u32 = 0x826181B8;
    'dispatch: loop {
        match pc {
            0x826181B8 => {
    //   block [0x826181B8..0x82618228)
	// 826181B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826181BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826181C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826181C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826181C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826181CC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 826181D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826181D4: 390B9C60  addi r8, r11, -0x63a0
	ctx.r[8].s64 = ctx.r[11].s64 + -25504;
	// 826181D8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826181DC: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 826181E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826181E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826181E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826181EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826181F0: 386AC53C  addi r3, r10, -0x3ac4
	ctx.r[3].s64 = ctx.r[10].s64 + -15044;
	// 826181F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826181F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826181FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261820C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618214: 4BE4EC0D  bl 0x82466e20
	ctx.lr = 0x82618218;
	sub_82466E20(ctx, base);
	// 82618218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261821C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618228 size=112
    let mut pc: u32 = 0x82618228;
    'dispatch: loop {
        match pc {
            0x82618228 => {
    //   block [0x82618228..0x82618298)
	// 82618228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261822C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618234: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618238: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261823C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82618240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618244: 390B9CC0  addi r8, r11, -0x6340
	ctx.r[8].s64 = ctx.r[11].s64 + -25408;
	// 82618248: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261824C: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 82618250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618254: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261825C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618260: 386AC56C  addi r3, r10, -0x3a94
	ctx.r[3].s64 = ctx.r[10].s64 + -14996;
	// 82618264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261826C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261827C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618284: 4BE4EB9D  bl 0x82466e20
	ctx.lr = 0x82618288;
	sub_82466E20(ctx, base);
	// 82618288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261828C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618298 size=112
    let mut pc: u32 = 0x82618298;
    'dispatch: loop {
        match pc {
            0x82618298 => {
    //   block [0x82618298..0x82618308)
	// 82618298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261829C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826182A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826182A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826182A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826182AC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 826182B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826182B4: 390B9D68  addi r8, r11, -0x6298
	ctx.r[8].s64 = ctx.r[11].s64 + -25240;
	// 826182B8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826182BC: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 826182C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826182C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826182C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826182CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826182D0: 386AC59C  addi r3, r10, -0x3a64
	ctx.r[3].s64 = ctx.r[10].s64 + -14948;
	// 826182D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826182D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826182DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826182E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826182E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826182E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826182EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826182F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826182F4: 4BE4EB2D  bl 0x82466e20
	ctx.lr = 0x826182F8;
	sub_82466E20(ctx, base);
	// 826182F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826182FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618308 size=112
    let mut pc: u32 = 0x82618308;
    'dispatch: loop {
        match pc {
            0x82618308 => {
    //   block [0x82618308..0x82618378)
	// 82618308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261830C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618314: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618318: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261831C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82618320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618324: 390B9DE0  addi r8, r11, -0x6220
	ctx.r[8].s64 = ctx.r[11].s64 + -25120;
	// 82618328: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261832C: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 82618330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618334: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618338: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261833C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618340: 386AC5CC  addi r3, r10, -0x3a34
	ctx.r[3].s64 = ctx.r[10].s64 + -14900;
	// 82618344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261834C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261835C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618364: 4BE4EABD  bl 0x82466e20
	ctx.lr = 0x82618368;
	sub_82466E20(ctx, base);
	// 82618368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261836C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618378 size=112
    let mut pc: u32 = 0x82618378;
    'dispatch: loop {
        match pc {
            0x82618378 => {
    //   block [0x82618378..0x826183E8)
	// 82618378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261837C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618384: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618388: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261838C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82618390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618394: 390B9E28  addi r8, r11, -0x61d8
	ctx.r[8].s64 = ctx.r[11].s64 + -25048;
	// 82618398: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8261839C: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 826183A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826183A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826183A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826183AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826183B0: 386AC5FC  addi r3, r10, -0x3a04
	ctx.r[3].s64 = ctx.r[10].s64 + -14852;
	// 826183B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826183B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826183BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826183C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826183C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826183C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826183CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826183D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826183D4: 4BE4EA4D  bl 0x82466e20
	ctx.lr = 0x826183D8;
	sub_82466E20(ctx, base);
	// 826183D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826183DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826183E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826183E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826183E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826183E8 size=112
    let mut pc: u32 = 0x826183E8;
    'dispatch: loop {
        match pc {
            0x826183E8 => {
    //   block [0x826183E8..0x82618458)
	// 826183E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826183EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826183F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826183F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826183F8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826183FC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82618400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618404: 390B9EB8  addi r8, r11, -0x6148
	ctx.r[8].s64 = ctx.r[11].s64 + -24904;
	// 82618408: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261840C: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 82618410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618414: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618418: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261841C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618420: 386AC62C  addi r3, r10, -0x39d4
	ctx.r[3].s64 = ctx.r[10].s64 + -14804;
	// 82618424: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261842C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261843C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618444: 4BE4E9DD  bl 0x82466e20
	ctx.lr = 0x82618448;
	sub_82466E20(ctx, base);
	// 82618448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261844C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618458 size=112
    let mut pc: u32 = 0x82618458;
    'dispatch: loop {
        match pc {
            0x82618458 => {
    //   block [0x82618458..0x826184C8)
	// 82618458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261845C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618464: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618468: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261846C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82618470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618474: 390B9F18  addi r8, r11, -0x60e8
	ctx.r[8].s64 = ctx.r[11].s64 + -24808;
	// 82618478: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261847C: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 82618480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618484: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261848C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618490: 386AC65C  addi r3, r10, -0x39a4
	ctx.r[3].s64 = ctx.r[10].s64 + -14756;
	// 82618494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261849C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826184A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826184A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826184A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826184AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826184B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826184B4: 4BE4E96D  bl 0x82466e20
	ctx.lr = 0x826184B8;
	sub_82466E20(ctx, base);
	// 826184B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826184BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826184C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826184C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826184C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826184C8 size=112
    let mut pc: u32 = 0x826184C8;
    'dispatch: loop {
        match pc {
            0x826184C8 => {
    //   block [0x826184C8..0x82618538)
	// 826184C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826184CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826184D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826184D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826184D8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826184DC: 38AAC65C  addi r5, r10, -0x39a4
	ctx.r[5].s64 = ctx.r[10].s64 + -14756;
	// 826184E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826184E4: 390B9F78  addi r8, r11, -0x6088
	ctx.r[8].s64 = ctx.r[11].s64 + -24712;
	// 826184E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826184EC: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 826184F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826184F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826184F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826184FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618500: 386AC68C  addi r3, r10, -0x3974
	ctx.r[3].s64 = ctx.r[10].s64 + -14708;
	// 82618504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261850C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261851C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618524: 4BE4E8FD  bl 0x82466e20
	ctx.lr = 0x82618528;
	sub_82466E20(ctx, base);
	// 82618528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261852C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618538 size=112
    let mut pc: u32 = 0x82618538;
    'dispatch: loop {
        match pc {
            0x82618538 => {
    //   block [0x82618538..0x826185A8)
	// 82618538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261853C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618544: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618548: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261854C: 38AAC65C  addi r5, r10, -0x39a4
	ctx.r[5].s64 = ctx.r[10].s64 + -14756;
	// 82618550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618554: 390B9FA8  addi r8, r11, -0x6058
	ctx.r[8].s64 = ctx.r[11].s64 + -24664;
	// 82618558: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261855C: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 82618560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618564: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261856C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618570: 386AC6BC  addi r3, r10, -0x3944
	ctx.r[3].s64 = ctx.r[10].s64 + -14660;
	// 82618574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261857C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261858C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618594: 4BE4E88D  bl 0x82466e20
	ctx.lr = 0x82618598;
	sub_82466E20(ctx, base);
	// 82618598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261859C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826185A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826185A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826185A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826185A8 size=100
    let mut pc: u32 = 0x826185A8;
    'dispatch: loop {
        match pc {
            0x826185A8 => {
    //   block [0x826185A8..0x8261860C)
	// 826185A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826185AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826185B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826185B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826185B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826185BC: 38AAC65C  addi r5, r10, -0x39a4
	ctx.r[5].s64 = ctx.r[10].s64 + -14756;
	// 826185C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826185C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826185C8: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 826185CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826185D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826185D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826185D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826185DC: 386AC6EC  addi r3, r10, -0x3914
	ctx.r[3].s64 = ctx.r[10].s64 + -14612;
	// 826185E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826185E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826185E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826185EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826185F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826185F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826185F8: 4BE4E829  bl 0x82466e20
	ctx.lr = 0x826185FC;
	sub_82466E20(ctx, base);
	// 826185FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618610 size=112
    let mut pc: u32 = 0x82618610;
    'dispatch: loop {
        match pc {
            0x82618610 => {
    //   block [0x82618610..0x82618680)
	// 82618610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261861C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618620: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618624: 38AAC65C  addi r5, r10, -0x39a4
	ctx.r[5].s64 = ctx.r[10].s64 + -14756;
	// 82618628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261862C: 390B9FF0  addi r8, r11, -0x6010
	ctx.r[8].s64 = ctx.r[11].s64 + -24592;
	// 82618630: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82618634: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 82618638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261863C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618648: 386AC71C  addi r3, r10, -0x38e4
	ctx.r[3].s64 = ctx.r[10].s64 + -14564;
	// 8261864C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261865C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261866C: 4BE4E7B5  bl 0x82466e20
	ctx.lr = 0x82618670;
	sub_82466E20(ctx, base);
	// 82618670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261867C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618680 size=108
    let mut pc: u32 = 0x82618680;
    'dispatch: loop {
        match pc {
            0x82618680 => {
    //   block [0x82618680..0x826186EC)
	// 82618680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261868C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82618694: 38EBA008  addi r7, r11, -0x5ff8
	ctx.r[7].s64 = ctx.r[11].s64 + -24568;
	// 82618698: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261869C: 388A0E24  addi r4, r10, 0xe24
	ctx.r[4].s64 = ctx.r[10].s64 + 3620;
	// 826186A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826186A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826186A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826186AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826186B0: 386AC74C  addi r3, r10, -0x38b4
	ctx.r[3].s64 = ctx.r[10].s64 + -14516;
	// 826186B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826186B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826186BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826186C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826186C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826186C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826186CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826186D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826186D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826186D8: 4BE4E749  bl 0x82466e20
	ctx.lr = 0x826186DC;
	sub_82466E20(ctx, base);
	// 826186DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826186E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826186E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826186E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826186F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826186F0 size=112
    let mut pc: u32 = 0x826186F0;
    'dispatch: loop {
        match pc {
            0x826186F0 => {
    //   block [0x826186F0..0x82618760)
	// 826186F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826186F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826186F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826186FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618700: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618704: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82618708: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261870C: 390BA050  addi r8, r11, -0x5fb0
	ctx.r[8].s64 = ctx.r[11].s64 + -24496;
	// 82618710: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82618714: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 82618718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261871C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618728: 386AC77C  addi r3, r10, -0x3884
	ctx.r[3].s64 = ctx.r[10].s64 + -14468;
	// 8261872C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261873C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261874C: 4BE4E6D5  bl 0x82466e20
	ctx.lr = 0x82618750;
	sub_82466E20(ctx, base);
	// 82618750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261875C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618760 size=112
    let mut pc: u32 = 0x82618760;
    'dispatch: loop {
        match pc {
            0x82618760 => {
    //   block [0x82618760..0x826187D0)
	// 82618760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261876C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618770: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618774: 38AAC77C  addi r5, r10, -0x3884
	ctx.r[5].s64 = ctx.r[10].s64 + -14468;
	// 82618778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261877C: 390BA0B0  addi r8, r11, -0x5f50
	ctx.r[8].s64 = ctx.r[11].s64 + -24400;
	// 82618780: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82618784: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 82618788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261878C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618798: 386AC7AC  addi r3, r10, -0x3854
	ctx.r[3].s64 = ctx.r[10].s64 + -14420;
	// 8261879C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826187A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826187A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826187A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826187AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826187B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826187B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826187B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826187BC: 4BE4E665  bl 0x82466e20
	ctx.lr = 0x826187C0;
	sub_82466E20(ctx, base);
	// 826187C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826187C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826187C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826187CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


