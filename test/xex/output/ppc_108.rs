pub fn sub_826C02F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C02F8 size=112
    let mut pc: u32 = 0x826C02F8;
    'dispatch: loop {
        match pc {
            0x826C02F8 => {
    //   block [0x826C02F8..0x826C0368)
	// 826C02F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C02FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0304: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0308: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C030C: 38AA341C  addi r5, r10, 0x341c
	ctx.r[5].s64 = ctx.r[10].s64 + 13340;
	// 826C0310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0314: 390B0F60  addi r8, r11, 0xf60
	ctx.r[8].s64 = ctx.r[11].s64 + 3936;
	// 826C0318: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C031C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826C0320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0324: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C032C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0330: 386A344C  addi r3, r10, 0x344c
	ctx.r[3].s64 = ctx.r[10].s64 + 13388;
	// 826C0334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C033C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C034C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0354: 4BDA6ACD  bl 0x82466e20
	ctx.lr = 0x826C0358;
	sub_82466E20(ctx, base);
	// 826C0358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C035C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0368 size=116
    let mut pc: u32 = 0x826C0368;
    'dispatch: loop {
        match pc {
            0x826C0368 => {
    //   block [0x826C0368..0x826C03DC)
	// 826C0368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C036C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0374: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C0378: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C037C: 390A0F78  addi r8, r10, 0xf78
	ctx.r[8].s64 = ctx.r[10].s64 + 3960;
	// 826C0380: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0384: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C0388: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C038C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0390: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C0394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C039C: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 826C03A0: 396B1244  addi r11, r11, 0x1244
	ctx.r[11].s64 = ctx.r[11].s64 + 4676;
	// 826C03A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C03A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C03AC: 386A347C  addi r3, r10, 0x347c
	ctx.r[3].s64 = ctx.r[10].s64 + 13436;
	// 826C03B0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C03B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C03B8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C03BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C03C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C03C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C03C8: 4BDA6A59  bl 0x82466e20
	ctx.lr = 0x826C03CC;
	sub_82466E20(ctx, base);
	// 826C03CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C03D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C03D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C03D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C03E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C03E0 size=48
    let mut pc: u32 = 0x826C03E0;
    'dispatch: loop {
        match pc {
            0x826C03E0 => {
    //   block [0x826C03E0..0x826C0410)
	// 826C03E0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C03E4: 814B102C  lwz r10, 0x102c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4140 as u32) ) } as u64;
	// 826C03E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C03EC: 396B3548  addi r11, r11, 0x3548
	ctx.r[11].s64 = ctx.r[11].s64 + 13640;
	// 826C03F0: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826C03F4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C03F8: 814A1028  lwz r10, 0x1028(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4136 as u32) ) } as u64;
	// 826C03FC: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826C0400: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C0404: 814A1024  lwz r10, 0x1024(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4132 as u32) ) } as u64;
	// 826C0408: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 826C040C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0410 size=116
    let mut pc: u32 = 0x826C0410;
    'dispatch: loop {
        match pc {
            0x826C0410 => {
    //   block [0x826C0410..0x826C0484)
	// 826C0410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C041C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C0420: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0424: 392B1318  addi r9, r11, 0x1318
	ctx.r[9].s64 = ctx.r[11].s64 + 4888;
	// 826C0428: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C042C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0430: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826C0434: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 826C0438: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C043C: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 826C0440: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0444: 396B3548  addi r11, r11, 0x3548
	ctx.r[11].s64 = ctx.r[11].s64 + 13640;
	// 826C0448: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C044C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0450: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C0454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0458: 386A34AC  addi r3, r10, 0x34ac
	ctx.r[3].s64 = ctx.r[10].s64 + 13484;
	// 826C045C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826C0460: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C0464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0468: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C046C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C0470: 4BDA69B1  bl 0x82466e20
	ctx.lr = 0x826C0474;
	sub_82466E20(ctx, base);
	// 826C0474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C047C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0488 size=116
    let mut pc: u32 = 0x826C0488;
    'dispatch: loop {
        match pc {
            0x826C0488 => {
    //   block [0x826C0488..0x826C04FC)
	// 826C0488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0494: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0498: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C049C: 390B1038  addi r8, r11, 0x1038
	ctx.r[8].s64 = ctx.r[11].s64 + 4152;
	// 826C04A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C04A4: 392A1440  addi r9, r10, 0x1440
	ctx.r[9].s64 = ctx.r[10].s64 + 5184;
	// 826C04A8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C04AC: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826C04B0: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C04B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C04B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C04BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C04C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C04C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C04C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C04CC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C04D0: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 826C04D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C04D8: 386B34DC  addi r3, r11, 0x34dc
	ctx.r[3].s64 = ctx.r[11].s64 + 13532;
	// 826C04DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C04E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C04E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C04E8: 4BDA6939  bl 0x82466e20
	ctx.lr = 0x826C04EC;
	sub_82466E20(ctx, base);
	// 826C04EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C04F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C04F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C04F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0500 size=112
    let mut pc: u32 = 0x826C0500;
    'dispatch: loop {
        match pc {
            0x826C0500 => {
    //   block [0x826C0500..0x826C0570)
	// 826C0500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C050C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0510: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0514: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C0518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C051C: 390B10C8  addi r8, r11, 0x10c8
	ctx.r[8].s64 = ctx.r[11].s64 + 4296;
	// 826C0520: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C0524: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 826C0528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C052C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0538: 386A350C  addi r3, r10, 0x350c
	ctx.r[3].s64 = ctx.r[10].s64 + 13580;
	// 826C053C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C054C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C055C: 4BDA68C5  bl 0x82466e20
	ctx.lr = 0x826C0560;
	sub_82466E20(ctx, base);
	// 826C0560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C056C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0570 size=112
    let mut pc: u32 = 0x826C0570;
    'dispatch: loop {
        match pc {
            0x826C0570 => {
    //   block [0x826C0570..0x826C05E0)
	// 826C0570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C057C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0580: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0584: 38AA185C  addi r5, r10, 0x185c
	ctx.r[5].s64 = ctx.r[10].s64 + 6236;
	// 826C0588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C058C: 390B10E0  addi r8, r11, 0x10e0
	ctx.r[8].s64 = ctx.r[11].s64 + 4320;
	// 826C0590: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C0594: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 826C0598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C059C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C05A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C05A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C05A8: 386A353C  addi r3, r10, 0x353c
	ctx.r[3].s64 = ctx.r[10].s64 + 13628;
	// 826C05AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C05B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C05B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C05B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C05BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C05C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C05C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C05C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C05CC: 4BDA6855  bl 0x82466e20
	ctx.lr = 0x826C05D0;
	sub_82466E20(ctx, base);
	// 826C05D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C05D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C05D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C05DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C05E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C05E0 size=108
    let mut pc: u32 = 0x826C05E0;
    'dispatch: loop {
        match pc {
            0x826C05E0 => {
    //   block [0x826C05E0..0x826C064C)
	// 826C05E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C05E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C05E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C05EC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C05F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C05F4: 38EB10F8  addi r7, r11, 0x10f8
	ctx.r[7].s64 = ctx.r[11].s64 + 4344;
	// 826C05F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C05FC: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 826C0600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C060C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0610: 386A356C  addi r3, r10, 0x356c
	ctx.r[3].s64 = ctx.r[10].s64 + 13676;
	// 826C0614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C0618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C061C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C062C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C0638: 4BDA67E9  bl 0x82466e20
	ctx.lr = 0x826C063C;
	sub_82466E20(ctx, base);
	// 826C063C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0650 size=112
    let mut pc: u32 = 0x826C0650;
    'dispatch: loop {
        match pc {
            0x826C0650 => {
    //   block [0x826C0650..0x826C06C0)
	// 826C0650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C065C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0660: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0664: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C0668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C066C: 390B1110  addi r8, r11, 0x1110
	ctx.r[8].s64 = ctx.r[11].s64 + 4368;
	// 826C0670: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C0674: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 826C0678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C067C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0688: 386A359C  addi r3, r10, 0x359c
	ctx.r[3].s64 = ctx.r[10].s64 + 13724;
	// 826C068C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C069C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C06A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C06A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C06A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C06AC: 4BDA6775  bl 0x82466e20
	ctx.lr = 0x826C06B0;
	sub_82466E20(ctx, base);
	// 826C06B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C06B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C06B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C06BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C06C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C06C0 size=108
    let mut pc: u32 = 0x826C06C0;
    'dispatch: loop {
        match pc {
            0x826C06C0 => {
    //   block [0x826C06C0..0x826C072C)
	// 826C06C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C06C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C06C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C06CC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C06D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C06D4: 38EB1158  addi r7, r11, 0x1158
	ctx.r[7].s64 = ctx.r[11].s64 + 4440;
	// 826C06D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C06DC: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 826C06E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C06E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C06E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C06EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C06F0: 386A35CC  addi r3, r10, 0x35cc
	ctx.r[3].s64 = ctx.r[10].s64 + 13772;
	// 826C06F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C06F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C06FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C070C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C0718: 4BDA6709  bl 0x82466e20
	ctx.lr = 0x826C071C;
	sub_82466E20(ctx, base);
	// 826C071C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0730 size=112
    let mut pc: u32 = 0x826C0730;
    'dispatch: loop {
        match pc {
            0x826C0730 => {
    //   block [0x826C0730..0x826C07A0)
	// 826C0730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C073C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0740: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0744: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C0748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C074C: 390B1170  addi r8, r11, 0x1170
	ctx.r[8].s64 = ctx.r[11].s64 + 4464;
	// 826C0750: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C0754: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826C0758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C075C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0768: 386A35FC  addi r3, r10, 0x35fc
	ctx.r[3].s64 = ctx.r[10].s64 + 13820;
	// 826C076C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C077C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C078C: 4BDA6695  bl 0x82466e20
	ctx.lr = 0x826C0790;
	sub_82466E20(ctx, base);
	// 826C0790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C079C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C07A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C07A0 size=112
    let mut pc: u32 = 0x826C07A0;
    'dispatch: loop {
        match pc {
            0x826C07A0 => {
    //   block [0x826C07A0..0x826C0810)
	// 826C07A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C07A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C07A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C07AC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C07B0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C07B4: 392A1498  addi r9, r10, 0x1498
	ctx.r[9].s64 = ctx.r[10].s64 + 5272;
	// 826C07B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C07BC: 390B11A8  addi r8, r11, 0x11a8
	ctx.r[8].s64 = ctx.r[11].s64 + 4520;
	// 826C07C0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C07C4: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 826C07C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C07CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C07D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C07D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C07D8: 386A362C  addi r3, r10, 0x362c
	ctx.r[3].s64 = ctx.r[10].s64 + 13868;
	// 826C07DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C07E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C07E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C07E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C07EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C07F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C07F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C07F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C07FC: 4BDA6625  bl 0x82466e20
	ctx.lr = 0x826C0800;
	sub_82466E20(ctx, base);
	// 826C0800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C080C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0810 size=116
    let mut pc: u32 = 0x826C0810;
    'dispatch: loop {
        match pc {
            0x826C0810 => {
    //   block [0x826C0810..0x826C0884)
	// 826C0810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C081C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0820: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C0824: 390B1250  addi r8, r11, 0x1250
	ctx.r[8].s64 = ctx.r[11].s64 + 4688;
	// 826C0828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C082C: 392A146C  addi r9, r10, 0x146c
	ctx.r[9].s64 = ctx.r[10].s64 + 5228;
	// 826C0830: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0834: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826C0838: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826C083C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0844: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C084C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0854: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C0858: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826C085C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C0860: 386B365C  addi r3, r11, 0x365c
	ctx.r[3].s64 = ctx.r[11].s64 + 13916;
	// 826C0864: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C0868: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C086C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0870: 4BDA65B1  bl 0x82466e20
	ctx.lr = 0x826C0874;
	sub_82466E20(ctx, base);
	// 826C0874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C087C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0888 size=112
    let mut pc: u32 = 0x826C0888;
    'dispatch: loop {
        match pc {
            0x826C0888 => {
    //   block [0x826C0888..0x826C08F8)
	// 826C0888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C088C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0894: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C0898: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C089C: 392A14C4  addi r9, r10, 0x14c4
	ctx.r[9].s64 = ctx.r[10].s64 + 5316;
	// 826C08A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C08A4: 390B1268  addi r8, r11, 0x1268
	ctx.r[8].s64 = ctx.r[11].s64 + 4712;
	// 826C08A8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826C08AC: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 826C08B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C08B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C08B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C08BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C08C0: 386A368C  addi r3, r10, 0x368c
	ctx.r[3].s64 = ctx.r[10].s64 + 13964;
	// 826C08C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C08C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C08CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C08D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C08D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C08D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C08DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C08E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C08E4: 4BDA653D  bl 0x82466e20
	ctx.lr = 0x826C08E8;
	sub_82466E20(ctx, base);
	// 826C08E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C08EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C08F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C08F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C08F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C08F8 size=112
    let mut pc: u32 = 0x826C08F8;
    'dispatch: loop {
        match pc {
            0x826C08F8 => {
    //   block [0x826C08F8..0x826C0968)
	// 826C08F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C08FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0904: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0908: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C090C: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826C0910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0914: 390B12C8  addi r8, r11, 0x12c8
	ctx.r[8].s64 = ctx.r[11].s64 + 4808;
	// 826C0918: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C091C: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826C0920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0924: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0928: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C092C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0930: 386A36BC  addi r3, r10, 0x36bc
	ctx.r[3].s64 = ctx.r[10].s64 + 14012;
	// 826C0934: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C093C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C094C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0954: 4BDA64CD  bl 0x82466e20
	ctx.lr = 0x826C0958;
	sub_82466E20(ctx, base);
	// 826C0958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C095C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0968 size=112
    let mut pc: u32 = 0x826C0968;
    'dispatch: loop {
        match pc {
            0x826C0968 => {
    //   block [0x826C0968..0x826C09D8)
	// 826C0968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C096C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0974: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0978: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C097C: 38AA27BC  addi r5, r10, 0x27bc
	ctx.r[5].s64 = ctx.r[10].s64 + 10172;
	// 826C0980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0984: 390B12E0  addi r8, r11, 0x12e0
	ctx.r[8].s64 = ctx.r[11].s64 + 4832;
	// 826C0988: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C098C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 826C0990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0994: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0998: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C099C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C09A0: 386A36EC  addi r3, r10, 0x36ec
	ctx.r[3].s64 = ctx.r[10].s64 + 14060;
	// 826C09A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C09A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C09AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C09B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C09B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C09B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C09BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C09C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C09C4: 4BDA645D  bl 0x82466e20
	ctx.lr = 0x826C09C8;
	sub_82466E20(ctx, base);
	// 826C09C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C09CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C09D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C09D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C09D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C09D8 size=112
    let mut pc: u32 = 0x826C09D8;
    'dispatch: loop {
        match pc {
            0x826C09D8 => {
    //   block [0x826C09D8..0x826C0A48)
	// 826C09D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C09DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C09E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C09E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C09E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C09EC: 38AA27BC  addi r5, r10, 0x27bc
	ctx.r[5].s64 = ctx.r[10].s64 + 10172;
	// 826C09F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C09F4: 390B1328  addi r8, r11, 0x1328
	ctx.r[8].s64 = ctx.r[11].s64 + 4904;
	// 826C09F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C09FC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826C0A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0A04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0A08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0A10: 386A371C  addi r3, r10, 0x371c
	ctx.r[3].s64 = ctx.r[10].s64 + 14108;
	// 826C0A14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0A34: 4BDA63ED  bl 0x82466e20
	ctx.lr = 0x826C0A38;
	sub_82466E20(ctx, base);
	// 826C0A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0A48 size=112
    let mut pc: u32 = 0x826C0A48;
    'dispatch: loop {
        match pc {
            0x826C0A48 => {
    //   block [0x826C0A48..0x826C0AB8)
	// 826C0A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0A54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0A58: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0A5C: 38AA27EC  addi r5, r10, 0x27ec
	ctx.r[5].s64 = ctx.r[10].s64 + 10220;
	// 826C0A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0A64: 390B1388  addi r8, r11, 0x1388
	ctx.r[8].s64 = ctx.r[11].s64 + 5000;
	// 826C0A68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C0A6C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826C0A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0A74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0A78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0A80: 386A374C  addi r3, r10, 0x374c
	ctx.r[3].s64 = ctx.r[10].s64 + 14156;
	// 826C0A84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0AA4: 4BDA637D  bl 0x82466e20
	ctx.lr = 0x826C0AA8;
	sub_82466E20(ctx, base);
	// 826C0AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0AB8 size=112
    let mut pc: u32 = 0x826C0AB8;
    'dispatch: loop {
        match pc {
            0x826C0AB8 => {
    //   block [0x826C0AB8..0x826C0B28)
	// 826C0AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0AC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0AC8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0ACC: 38AA27EC  addi r5, r10, 0x27ec
	ctx.r[5].s64 = ctx.r[10].s64 + 10220;
	// 826C0AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0AD4: 390B13E8  addi r8, r11, 0x13e8
	ctx.r[8].s64 = ctx.r[11].s64 + 5096;
	// 826C0AD8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C0ADC: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 826C0AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0AE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0AE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0AF0: 386A377C  addi r3, r10, 0x377c
	ctx.r[3].s64 = ctx.r[10].s64 + 14204;
	// 826C0AF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0B14: 4BDA630D  bl 0x82466e20
	ctx.lr = 0x826C0B18;
	sub_82466E20(ctx, base);
	// 826C0B18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0B28 size=112
    let mut pc: u32 = 0x826C0B28;
    'dispatch: loop {
        match pc {
            0x826C0B28 => {
    //   block [0x826C0B28..0x826C0B98)
	// 826C0B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0B34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0B38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0B3C: 38AA27BC  addi r5, r10, 0x27bc
	ctx.r[5].s64 = ctx.r[10].s64 + 10172;
	// 826C0B40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0B44: 390B1448  addi r8, r11, 0x1448
	ctx.r[8].s64 = ctx.r[11].s64 + 5192;
	// 826C0B48: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826C0B4C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826C0B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0B54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0B58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0B5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0B60: 386A37AC  addi r3, r10, 0x37ac
	ctx.r[3].s64 = ctx.r[10].s64 + 14252;
	// 826C0B64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0B6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0B84: 4BDA629D  bl 0x82466e20
	ctx.lr = 0x826C0B88;
	sub_82466E20(ctx, base);
	// 826C0B88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0B98 size=112
    let mut pc: u32 = 0x826C0B98;
    'dispatch: loop {
        match pc {
            0x826C0B98 => {
    //   block [0x826C0B98..0x826C0C08)
	// 826C0B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0BA4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C0BA8: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 826C0BAC: 38EA1508  addi r7, r10, 0x1508
	ctx.r[7].s64 = ctx.r[10].s64 + 5384;
	// 826C0BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0BB4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C0BB8: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 826C0BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0BC0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C0BC4: 396B14D8  addi r11, r11, 0x14d8
	ctx.r[11].s64 = ctx.r[11].s64 + 5336;
	// 826C0BC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C0BCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0BD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0BD4: 386A37DC  addi r3, r10, 0x37dc
	ctx.r[3].s64 = ctx.r[10].s64 + 14300;
	// 826C0BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0BDC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C0BE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0BE4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C0BE8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0BEC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0BF0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C0BF4: 4BDA622D  bl 0x82466e20
	ctx.lr = 0x826C0BF8;
	sub_82466E20(ctx, base);
	// 826C0BF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0C08 size=112
    let mut pc: u32 = 0x826C0C08;
    'dispatch: loop {
        match pc {
            0x826C0C08 => {
    //   block [0x826C0C08..0x826C0C78)
	// 826C0C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0C14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0C18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0C1C: 38AA191C  addi r5, r10, 0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + 6428;
	// 826C0C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0C24: 390B16D0  addi r8, r11, 0x16d0
	ctx.r[8].s64 = ctx.r[11].s64 + 5840;
	// 826C0C28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C0C2C: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826C0C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0C34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0C38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0C40: 386A380C  addi r3, r10, 0x380c
	ctx.r[3].s64 = ctx.r[10].s64 + 14348;
	// 826C0C44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0C4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0C54: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C0C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0C64: 4BDA61BD  bl 0x82466e20
	ctx.lr = 0x826C0C68;
	sub_82466E20(ctx, base);
	// 826C0C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0C78 size=112
    let mut pc: u32 = 0x826C0C78;
    'dispatch: loop {
        match pc {
            0x826C0C78 => {
    //   block [0x826C0C78..0x826C0CE8)
	// 826C0C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0C84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0C88: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0C8C: 38AA191C  addi r5, r10, 0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + 6428;
	// 826C0C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0C94: 390B16E8  addi r8, r11, 0x16e8
	ctx.r[8].s64 = ctx.r[11].s64 + 5864;
	// 826C0C98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C0C9C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 826C0CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0CA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0CB0: 386A383C  addi r3, r10, 0x383c
	ctx.r[3].s64 = ctx.r[10].s64 + 14396;
	// 826C0CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0CC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C0CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0CD4: 4BDA614D  bl 0x82466e20
	ctx.lr = 0x826C0CD8;
	sub_82466E20(ctx, base);
	// 826C0CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0CE8 size=112
    let mut pc: u32 = 0x826C0CE8;
    'dispatch: loop {
        match pc {
            0x826C0CE8 => {
    //   block [0x826C0CE8..0x826C0D58)
	// 826C0CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0CF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0CF8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0CFC: 38AA191C  addi r5, r10, 0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + 6428;
	// 826C0D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0D04: 390B1700  addi r8, r11, 0x1700
	ctx.r[8].s64 = ctx.r[11].s64 + 5888;
	// 826C0D08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C0D0C: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826C0D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0D14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0D20: 386A386C  addi r3, r10, 0x386c
	ctx.r[3].s64 = ctx.r[10].s64 + 14444;
	// 826C0D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0D44: 4BDA60DD  bl 0x82466e20
	ctx.lr = 0x826C0D48;
	sub_82466E20(ctx, base);
	// 826C0D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0D58 size=108
    let mut pc: u32 = 0x826C0D58;
    'dispatch: loop {
        match pc {
            0x826C0D58 => {
    //   block [0x826C0D58..0x826C0DC4)
	// 826C0D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0D64: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0D6C: 38EB1730  addi r7, r11, 0x1730
	ctx.r[7].s64 = ctx.r[11].s64 + 5936;
	// 826C0D70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C0D74: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 826C0D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0D7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C0D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0D88: 386A389C  addi r3, r10, 0x389c
	ctx.r[3].s64 = ctx.r[10].s64 + 14492;
	// 826C0D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C0D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C0DB0: 4BDA6071  bl 0x82466e20
	ctx.lr = 0x826C0DB4;
	sub_82466E20(ctx, base);
	// 826C0DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0DC8 size=112
    let mut pc: u32 = 0x826C0DC8;
    'dispatch: loop {
        match pc {
            0x826C0DC8 => {
    //   block [0x826C0DC8..0x826C0E38)
	// 826C0DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0DD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0DD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0DDC: 38AA191C  addi r5, r10, 0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + 6428;
	// 826C0DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0DE4: 390B1760  addi r8, r11, 0x1760
	ctx.r[8].s64 = ctx.r[11].s64 + 5984;
	// 826C0DE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C0DEC: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826C0DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0DF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0E00: 386A38CC  addi r3, r10, 0x38cc
	ctx.r[3].s64 = ctx.r[10].s64 + 14540;
	// 826C0E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0E14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C0E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0E24: 4BDA5FFD  bl 0x82466e20
	ctx.lr = 0x826C0E28;
	sub_82466E20(ctx, base);
	// 826C0E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0E38 size=108
    let mut pc: u32 = 0x826C0E38;
    'dispatch: loop {
        match pc {
            0x826C0E38 => {
    //   block [0x826C0E38..0x826C0EA4)
	// 826C0E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0E44: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0E4C: 38EB1778  addi r7, r11, 0x1778
	ctx.r[7].s64 = ctx.r[11].s64 + 6008;
	// 826C0E50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C0E54: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 826C0E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0E5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0E60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C0E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0E68: 386A38FC  addi r3, r10, 0x38fc
	ctx.r[3].s64 = ctx.r[10].s64 + 14588;
	// 826C0E6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C0E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C0E90: 4BDA5F91  bl 0x82466e20
	ctx.lr = 0x826C0E94;
	sub_82466E20(ctx, base);
	// 826C0E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0EA8 size=108
    let mut pc: u32 = 0x826C0EA8;
    'dispatch: loop {
        match pc {
            0x826C0EA8 => {
    //   block [0x826C0EA8..0x826C0F14)
	// 826C0EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0EB4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0EBC: 38EB17A8  addi r7, r11, 0x17a8
	ctx.r[7].s64 = ctx.r[11].s64 + 6056;
	// 826C0EC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C0EC4: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 826C0EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0ECC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C0ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0ED8: 386A392C  addi r3, r10, 0x392c
	ctx.r[3].s64 = ctx.r[10].s64 + 14636;
	// 826C0EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C0EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C0F00: 4BDA5F21  bl 0x82466e20
	ctx.lr = 0x826C0F04;
	sub_82466E20(ctx, base);
	// 826C0F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0F18 size=112
    let mut pc: u32 = 0x826C0F18;
    'dispatch: loop {
        match pc {
            0x826C0F18 => {
    //   block [0x826C0F18..0x826C0F88)
	// 826C0F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0F24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0F28: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0F2C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C0F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0F34: 390B17F0  addi r8, r11, 0x17f0
	ctx.r[8].s64 = ctx.r[11].s64 + 6128;
	// 826C0F38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C0F3C: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 826C0F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0F44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0F48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0F50: 386A395C  addi r3, r10, 0x395c
	ctx.r[3].s64 = ctx.r[10].s64 + 14684;
	// 826C0F54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0F74: 4BDA5EAD  bl 0x82466e20
	ctx.lr = 0x826C0F78;
	sub_82466E20(ctx, base);
	// 826C0F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0F88 size=112
    let mut pc: u32 = 0x826C0F88;
    'dispatch: loop {
        match pc {
            0x826C0F88 => {
    //   block [0x826C0F88..0x826C0FF8)
	// 826C0F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0F94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0F98: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0F9C: 38AA27EC  addi r5, r10, 0x27ec
	ctx.r[5].s64 = ctx.r[10].s64 + 10220;
	// 826C0FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0FA4: 390B1838  addi r8, r11, 0x1838
	ctx.r[8].s64 = ctx.r[11].s64 + 6200;
	// 826C0FA8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826C0FAC: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 826C0FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0FB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0FB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0FC0: 386A398C  addi r3, r10, 0x398c
	ctx.r[3].s64 = ctx.r[10].s64 + 14732;
	// 826C0FC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0FE4: 4BDA5E3D  bl 0x82466e20
	ctx.lr = 0x826C0FE8;
	sub_82466E20(ctx, base);
	// 826C0FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0FF8 size=108
    let mut pc: u32 = 0x826C0FF8;
    'dispatch: loop {
        match pc {
            0x826C0FF8 => {
    //   block [0x826C0FF8..0x826C1064)
	// 826C0FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1004: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C100C: 38EB18C8  addi r7, r11, 0x18c8
	ctx.r[7].s64 = ctx.r[11].s64 + 6344;
	// 826C1010: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C1014: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826C1018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C101C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1028: 386A39BC  addi r3, r10, 0x39bc
	ctx.r[3].s64 = ctx.r[10].s64 + 14780;
	// 826C102C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C103C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C104C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1050: 4BDA5DD1  bl 0x82466e20
	ctx.lr = 0x826C1054;
	sub_82466E20(ctx, base);
	// 826C1054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C105C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1068 size=108
    let mut pc: u32 = 0x826C1068;
    'dispatch: loop {
        match pc {
            0x826C1068 => {
    //   block [0x826C1068..0x826C10D4)
	// 826C1068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C106C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1074: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C107C: 38EB1910  addi r7, r11, 0x1910
	ctx.r[7].s64 = ctx.r[11].s64 + 6416;
	// 826C1080: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C1084: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 826C1088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C108C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1098: 386A39EC  addi r3, r10, 0x39ec
	ctx.r[3].s64 = ctx.r[10].s64 + 14828;
	// 826C109C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C10A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C10A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C10A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C10AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C10B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C10B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C10B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C10BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C10C0: 4BDA5D61  bl 0x82466e20
	ctx.lr = 0x826C10C4;
	sub_82466E20(ctx, base);
	// 826C10C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C10C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C10CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C10D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C10D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C10D8 size=108
    let mut pc: u32 = 0x826C10D8;
    'dispatch: loop {
        match pc {
            0x826C10D8 => {
    //   block [0x826C10D8..0x826C1144)
	// 826C10D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C10DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C10E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C10E4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C10E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C10EC: 38EB1940  addi r7, r11, 0x1940
	ctx.r[7].s64 = ctx.r[11].s64 + 6464;
	// 826C10F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C10F4: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 826C10F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C10FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1108: 386A3A1C  addi r3, r10, 0x3a1c
	ctx.r[3].s64 = ctx.r[10].s64 + 14876;
	// 826C110C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C111C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C112C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1130: 4BDA5CF1  bl 0x82466e20
	ctx.lr = 0x826C1134;
	sub_82466E20(ctx, base);
	// 826C1134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C113C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1148 size=112
    let mut pc: u32 = 0x826C1148;
    'dispatch: loop {
        match pc {
            0x826C1148 => {
    //   block [0x826C1148..0x826C11B8)
	// 826C1148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C114C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1154: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1158: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C115C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1164: 390B1970  addi r8, r11, 0x1970
	ctx.r[8].s64 = ctx.r[11].s64 + 6512;
	// 826C1168: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C116C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826C1170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1174: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1178: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C117C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1180: 386A3A4C  addi r3, r10, 0x3a4c
	ctx.r[3].s64 = ctx.r[10].s64 + 14924;
	// 826C1184: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C118C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C119C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C11A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C11A4: 4BDA5C7D  bl 0x82466e20
	ctx.lr = 0x826C11A8;
	sub_82466E20(ctx, base);
	// 826C11A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C11AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C11B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C11B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C11B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C11B8 size=112
    let mut pc: u32 = 0x826C11B8;
    'dispatch: loop {
        match pc {
            0x826C11B8 => {
    //   block [0x826C11B8..0x826C1228)
	// 826C11B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C11BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C11C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C11C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C11C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C11CC: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C11D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C11D4: 390B19A0  addi r8, r11, 0x19a0
	ctx.r[8].s64 = ctx.r[11].s64 + 6560;
	// 826C11D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C11DC: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 826C11E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C11E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C11E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C11EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C11F0: 386A3A7C  addi r3, r10, 0x3a7c
	ctx.r[3].s64 = ctx.r[10].s64 + 14972;
	// 826C11F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C11F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C11FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C120C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1214: 4BDA5C0D  bl 0x82466e20
	ctx.lr = 0x826C1218;
	sub_82466E20(ctx, base);
	// 826C1218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C121C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1228 size=112
    let mut pc: u32 = 0x826C1228;
    'dispatch: loop {
        match pc {
            0x826C1228 => {
    //   block [0x826C1228..0x826C1298)
	// 826C1228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C122C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1234: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1238: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C123C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1244: 390B19B8  addi r8, r11, 0x19b8
	ctx.r[8].s64 = ctx.r[11].s64 + 6584;
	// 826C1248: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C124C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826C1250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1254: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C125C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1260: 386A3AAC  addi r3, r10, 0x3aac
	ctx.r[3].s64 = ctx.r[10].s64 + 15020;
	// 826C1264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C126C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C127C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1284: 4BDA5B9D  bl 0x82466e20
	ctx.lr = 0x826C1288;
	sub_82466E20(ctx, base);
	// 826C1288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C128C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1298 size=108
    let mut pc: u32 = 0x826C1298;
    'dispatch: loop {
        match pc {
            0x826C1298 => {
    //   block [0x826C1298..0x826C1304)
	// 826C1298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C129C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C12A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C12A4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C12A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C12AC: 38EB19D0  addi r7, r11, 0x19d0
	ctx.r[7].s64 = ctx.r[11].s64 + 6608;
	// 826C12B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C12B4: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 826C12B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C12BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C12C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C12C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C12C8: 386A3ADC  addi r3, r10, 0x3adc
	ctx.r[3].s64 = ctx.r[10].s64 + 15068;
	// 826C12CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C12D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C12D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C12D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C12DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C12E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C12E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C12E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C12EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C12F0: 4BDA5B31  bl 0x82466e20
	ctx.lr = 0x826C12F4;
	sub_82466E20(ctx, base);
	// 826C12F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C12F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C12FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1308 size=112
    let mut pc: u32 = 0x826C1308;
    'dispatch: loop {
        match pc {
            0x826C1308 => {
    //   block [0x826C1308..0x826C1378)
	// 826C1308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C130C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1314: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1318: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C131C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1324: 390B1A00  addi r8, r11, 0x1a00
	ctx.r[8].s64 = ctx.r[11].s64 + 6656;
	// 826C1328: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C132C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 826C1330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1334: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1338: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C133C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1340: 386A3B0C  addi r3, r10, 0x3b0c
	ctx.r[3].s64 = ctx.r[10].s64 + 15116;
	// 826C1344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C134C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C135C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1364: 4BDA5ABD  bl 0x82466e20
	ctx.lr = 0x826C1368;
	sub_82466E20(ctx, base);
	// 826C1368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C136C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1378 size=108
    let mut pc: u32 = 0x826C1378;
    'dispatch: loop {
        match pc {
            0x826C1378 => {
    //   block [0x826C1378..0x826C13E4)
	// 826C1378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C137C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1384: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C138C: 38EB1A18  addi r7, r11, 0x1a18
	ctx.r[7].s64 = ctx.r[11].s64 + 6680;
	// 826C1390: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826C1394: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 826C1398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C139C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C13A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C13A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C13A8: 386A3B3C  addi r3, r10, 0x3b3c
	ctx.r[3].s64 = ctx.r[10].s64 + 15164;
	// 826C13AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C13B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C13B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C13B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C13BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C13C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C13C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C13C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C13CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C13D0: 4BDA5A51  bl 0x82466e20
	ctx.lr = 0x826C13D4;
	sub_82466E20(ctx, base);
	// 826C13D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C13D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C13DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C13E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C13E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C13E8 size=112
    let mut pc: u32 = 0x826C13E8;
    'dispatch: loop {
        match pc {
            0x826C13E8 => {
    //   block [0x826C13E8..0x826C1458)
	// 826C13E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C13EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C13F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C13F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C13F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C13FC: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1404: 390B1B08  addi r8, r11, 0x1b08
	ctx.r[8].s64 = ctx.r[11].s64 + 6920;
	// 826C1408: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 826C140C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 826C1410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1414: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1418: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C141C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1420: 386A3B6C  addi r3, r10, 0x3b6c
	ctx.r[3].s64 = ctx.r[10].s64 + 15212;
	// 826C1424: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C142C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C143C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1444: 4BDA59DD  bl 0x82466e20
	ctx.lr = 0x826C1448;
	sub_82466E20(ctx, base);
	// 826C1448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C144C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1458 size=108
    let mut pc: u32 = 0x826C1458;
    'dispatch: loop {
        match pc {
            0x826C1458 => {
    //   block [0x826C1458..0x826C14C4)
	// 826C1458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C145C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1464: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C146C: 38EB1CB8  addi r7, r11, 0x1cb8
	ctx.r[7].s64 = ctx.r[11].s64 + 7352;
	// 826C1470: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826C1474: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 826C1478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C147C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1480: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1488: 386A3B9C  addi r3, r10, 0x3b9c
	ctx.r[3].s64 = ctx.r[10].s64 + 15260;
	// 826C148C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C149C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C14A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C14A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C14A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C14AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C14B0: 4BDA5971  bl 0x82466e20
	ctx.lr = 0x826C14B4;
	sub_82466E20(ctx, base);
	// 826C14B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C14B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C14BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C14C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C14C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C14C8 size=112
    let mut pc: u32 = 0x826C14C8;
    'dispatch: loop {
        match pc {
            0x826C14C8 => {
    //   block [0x826C14C8..0x826C1538)
	// 826C14C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C14CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C14D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C14D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C14D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C14DC: 38AA27EC  addi r5, r10, 0x27ec
	ctx.r[5].s64 = ctx.r[10].s64 + 10220;
	// 826C14E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C14E4: 390B1E50  addi r8, r11, 0x1e50
	ctx.r[8].s64 = ctx.r[11].s64 + 7760;
	// 826C14E8: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826C14EC: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 826C14F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C14F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C14F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C14FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1500: 386A3BCC  addi r3, r10, 0x3bcc
	ctx.r[3].s64 = ctx.r[10].s64 + 15308;
	// 826C1504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C150C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C151C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1524: 4BDA58FD  bl 0x82466e20
	ctx.lr = 0x826C1528;
	sub_82466E20(ctx, base);
	// 826C1528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C152C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1538 size=100
    let mut pc: u32 = 0x826C1538;
    'dispatch: loop {
        match pc {
            0x826C1538 => {
    //   block [0x826C1538..0x826C159C)
	// 826C1538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C153C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1544: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C154C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1558: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826C155C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C156C: 386A3BFC  addi r3, r10, 0x3bfc
	ctx.r[3].s64 = ctx.r[10].s64 + 15356;
	// 826C1570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1574: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1578: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C157C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1580: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C1584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1588: 4BDA5899  bl 0x82466e20
	ctx.lr = 0x826C158C;
	sub_82466E20(ctx, base);
	// 826C158C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C15A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C15A0 size=112
    let mut pc: u32 = 0x826C15A0;
    'dispatch: loop {
        match pc {
            0x826C15A0 => {
    //   block [0x826C15A0..0x826C1610)
	// 826C15A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C15A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C15A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C15AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C15B0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C15B4: 38AA3BFC  addi r5, r10, 0x3bfc
	ctx.r[5].s64 = ctx.r[10].s64 + 15356;
	// 826C15B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C15BC: 390B20A8  addi r8, r11, 0x20a8
	ctx.r[8].s64 = ctx.r[11].s64 + 8360;
	// 826C15C0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826C15C4: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 826C15C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C15CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C15D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C15D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C15D8: 386A3C2C  addi r3, r10, 0x3c2c
	ctx.r[3].s64 = ctx.r[10].s64 + 15404;
	// 826C15DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C15E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C15E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C15E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C15EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C15F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C15F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C15F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C15FC: 4BDA5825  bl 0x82466e20
	ctx.lr = 0x826C1600;
	sub_82466E20(ctx, base);
	// 826C1600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C160C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1610 size=100
    let mut pc: u32 = 0x826C1610;
    'dispatch: loop {
        match pc {
            0x826C1610 => {
    //   block [0x826C1610..0x826C1674)
	// 826C1610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C161C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1624: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C162C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1630: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 826C1634: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C163C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1644: 386A3C5C  addi r3, r10, 0x3c5c
	ctx.r[3].s64 = ctx.r[10].s64 + 15452;
	// 826C1648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C164C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1650: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C1654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1658: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C165C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1660: 4BDA57C1  bl 0x82466e20
	ctx.lr = 0x826C1664;
	sub_82466E20(ctx, base);
	// 826C1664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C166C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1678 size=108
    let mut pc: u32 = 0x826C1678;
    'dispatch: loop {
        match pc {
            0x826C1678 => {
    //   block [0x826C1678..0x826C16E4)
	// 826C1678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C167C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1684: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C168C: 38EB2120  addi r7, r11, 0x2120
	ctx.r[7].s64 = ctx.r[11].s64 + 8480;
	// 826C1690: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C1694: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 826C1698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C169C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C16A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C16A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C16A8: 386A3C8C  addi r3, r10, 0x3c8c
	ctx.r[3].s64 = ctx.r[10].s64 + 15500;
	// 826C16AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C16B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C16B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C16B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C16BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C16C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C16C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C16C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C16CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C16D0: 4BDA5751  bl 0x82466e20
	ctx.lr = 0x826C16D4;
	sub_82466E20(ctx, base);
	// 826C16D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C16D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C16DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C16E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C16E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C16E8 size=112
    let mut pc: u32 = 0x826C16E8;
    'dispatch: loop {
        match pc {
            0x826C16E8 => {
    //   block [0x826C16E8..0x826C1758)
	// 826C16E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C16EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C16F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C16F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C16F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C16FC: 38AA3C5C  addi r5, r10, 0x3c5c
	ctx.r[5].s64 = ctx.r[10].s64 + 15452;
	// 826C1700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1704: 390B2168  addi r8, r11, 0x2168
	ctx.r[8].s64 = ctx.r[11].s64 + 8552;
	// 826C1708: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C170C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 826C1710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1714: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C171C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1720: 386A3CBC  addi r3, r10, 0x3cbc
	ctx.r[3].s64 = ctx.r[10].s64 + 15548;
	// 826C1724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C172C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C173C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1744: 4BDA56DD  bl 0x82466e20
	ctx.lr = 0x826C1748;
	sub_82466E20(ctx, base);
	// 826C1748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C174C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1758 size=100
    let mut pc: u32 = 0x826C1758;
    'dispatch: loop {
        match pc {
            0x826C1758 => {
    //   block [0x826C1758..0x826C17BC)
	// 826C1758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C175C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1764: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C176C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1778: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826C177C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C178C: 386A3CEC  addi r3, r10, 0x3cec
	ctx.r[3].s64 = ctx.r[10].s64 + 15596;
	// 826C1790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1794: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1798: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C179C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C17A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C17A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C17A8: 4BDA5679  bl 0x82466e20
	ctx.lr = 0x826C17AC;
	sub_82466E20(ctx, base);
	// 826C17AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C17B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C17B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C17B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C17C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C17C0 size=100
    let mut pc: u32 = 0x826C17C0;
    'dispatch: loop {
        match pc {
            0x826C17C0 => {
    //   block [0x826C17C0..0x826C1824)
	// 826C17C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C17C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C17C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C17CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C17D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C17D4: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C17D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C17DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C17E0: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 826C17E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C17E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C17EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C17F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C17F4: 386A3D1C  addi r3, r10, 0x3d1c
	ctx.r[3].s64 = ctx.r[10].s64 + 15644;
	// 826C17F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C17FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1800: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C1804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1808: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C180C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1810: 4BDA5611  bl 0x82466e20
	ctx.lr = 0x826C1814;
	sub_82466E20(ctx, base);
	// 826C1814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C181C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1828 size=112
    let mut pc: u32 = 0x826C1828;
    'dispatch: loop {
        match pc {
            0x826C1828 => {
    //   block [0x826C1828..0x826C1898)
	// 826C1828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C182C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1834: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1838: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C183C: 38AA3CEC  addi r5, r10, 0x3cec
	ctx.r[5].s64 = ctx.r[10].s64 + 15596;
	// 826C1840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1844: 390B2198  addi r8, r11, 0x2198
	ctx.r[8].s64 = ctx.r[11].s64 + 8600;
	// 826C1848: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C184C: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826C1850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C185C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1860: 386A3D4C  addi r3, r10, 0x3d4c
	ctx.r[3].s64 = ctx.r[10].s64 + 15692;
	// 826C1864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C186C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C187C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1884: 4BDA559D  bl 0x82466e20
	ctx.lr = 0x826C1888;
	sub_82466E20(ctx, base);
	// 826C1888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C188C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1898 size=112
    let mut pc: u32 = 0x826C1898;
    'dispatch: loop {
        match pc {
            0x826C1898 => {
    //   block [0x826C1898..0x826C1908)
	// 826C1898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C189C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C18A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C18A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C18A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C18AC: 38AA3D1C  addi r5, r10, 0x3d1c
	ctx.r[5].s64 = ctx.r[10].s64 + 15644;
	// 826C18B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C18B4: 390B21F8  addi r8, r11, 0x21f8
	ctx.r[8].s64 = ctx.r[11].s64 + 8696;
	// 826C18B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C18BC: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 826C18C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C18C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C18C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C18CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C18D0: 386A3D7C  addi r3, r10, 0x3d7c
	ctx.r[3].s64 = ctx.r[10].s64 + 15740;
	// 826C18D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C18D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C18DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C18E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C18E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C18E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C18EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C18F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C18F4: 4BDA552D  bl 0x82466e20
	ctx.lr = 0x826C18F8;
	sub_82466E20(ctx, base);
	// 826C18F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C18FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1908 size=100
    let mut pc: u32 = 0x826C1908;
    'dispatch: loop {
        match pc {
            0x826C1908 => {
    //   block [0x826C1908..0x826C196C)
	// 826C1908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C190C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1914: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C191C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1928: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826C192C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C193C: 386A3DAC  addi r3, r10, 0x3dac
	ctx.r[3].s64 = ctx.r[10].s64 + 15788;
	// 826C1940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1944: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1948: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C194C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1950: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C1954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1958: 4BDA54C9  bl 0x82466e20
	ctx.lr = 0x826C195C;
	sub_82466E20(ctx, base);
	// 826C195C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1970 size=112
    let mut pc: u32 = 0x826C1970;
    'dispatch: loop {
        match pc {
            0x826C1970 => {
    //   block [0x826C1970..0x826C19E0)
	// 826C1970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C197C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1980: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1984: 38AA3DAC  addi r5, r10, 0x3dac
	ctx.r[5].s64 = ctx.r[10].s64 + 15788;
	// 826C1988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C198C: 390B2258  addi r8, r11, 0x2258
	ctx.r[8].s64 = ctx.r[11].s64 + 8792;
	// 826C1990: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826C1994: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 826C1998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C199C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C19A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C19A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C19A8: 386A3DDC  addi r3, r10, 0x3ddc
	ctx.r[3].s64 = ctx.r[10].s64 + 15836;
	// 826C19AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C19B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C19B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C19B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C19BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C19C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C19C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C19C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C19CC: 4BDA5455  bl 0x82466e20
	ctx.lr = 0x826C19D0;
	sub_82466E20(ctx, base);
	// 826C19D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C19D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C19D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C19DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C19E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C19E0 size=108
    let mut pc: u32 = 0x826C19E0;
    'dispatch: loop {
        match pc {
            0x826C19E0 => {
    //   block [0x826C19E0..0x826C1A4C)
	// 826C19E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C19E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C19E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C19EC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C19F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C19F4: 38EB2348  addi r7, r11, 0x2348
	ctx.r[7].s64 = ctx.r[11].s64 + 9032;
	// 826C19F8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826C19FC: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 826C1A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1A04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1A08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1A10: 386A3E0C  addi r3, r10, 0x3e0c
	ctx.r[3].s64 = ctx.r[10].s64 + 15884;
	// 826C1A14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1A24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1A34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1A38: 4BDA53E9  bl 0x82466e20
	ctx.lr = 0x826C1A3C;
	sub_82466E20(ctx, base);
	// 826C1A3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1A50 size=108
    let mut pc: u32 = 0x826C1A50;
    'dispatch: loop {
        match pc {
            0x826C1A50 => {
    //   block [0x826C1A50..0x826C1ABC)
	// 826C1A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1A5C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1A64: 38EB2438  addi r7, r11, 0x2438
	ctx.r[7].s64 = ctx.r[11].s64 + 9272;
	// 826C1A68: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C1A6C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826C1A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1A74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1A80: 386A3E3C  addi r3, r10, 0x3e3c
	ctx.r[3].s64 = ctx.r[10].s64 + 15932;
	// 826C1A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1AA8: 4BDA5379  bl 0x82466e20
	ctx.lr = 0x826C1AAC;
	sub_82466E20(ctx, base);
	// 826C1AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1AC0 size=108
    let mut pc: u32 = 0x826C1AC0;
    'dispatch: loop {
        match pc {
            0x826C1AC0 => {
    //   block [0x826C1AC0..0x826C1B2C)
	// 826C1AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1ACC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1AD4: 38EB2480  addi r7, r11, 0x2480
	ctx.r[7].s64 = ctx.r[11].s64 + 9344;
	// 826C1AD8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826C1ADC: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 826C1AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1AE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1AE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1AF0: 386A3E6C  addi r3, r10, 0x3e6c
	ctx.r[3].s64 = ctx.r[10].s64 + 15980;
	// 826C1AF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1B18: 4BDA5309  bl 0x82466e20
	ctx.lr = 0x826C1B1C;
	sub_82466E20(ctx, base);
	// 826C1B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1B30 size=108
    let mut pc: u32 = 0x826C1B30;
    'dispatch: loop {
        match pc {
            0x826C1B30 => {
    //   block [0x826C1B30..0x826C1B9C)
	// 826C1B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1B3C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1B40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1B44: 38EB2558  addi r7, r11, 0x2558
	ctx.r[7].s64 = ctx.r[11].s64 + 9560;
	// 826C1B48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C1B4C: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826C1B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1B54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1B58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1B60: 386A3E9C  addi r3, r10, 0x3e9c
	ctx.r[3].s64 = ctx.r[10].s64 + 16028;
	// 826C1B64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1B84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1B88: 4BDA5299  bl 0x82466e20
	ctx.lr = 0x826C1B8C;
	sub_82466E20(ctx, base);
	// 826C1B8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1BA0 size=100
    let mut pc: u32 = 0x826C1BA0;
    'dispatch: loop {
        match pc {
            0x826C1BA0 => {
    //   block [0x826C1BA0..0x826C1C04)
	// 826C1BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1BAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1BB4: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1BC0: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 826C1BC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1BD4: 386A3ECC  addi r3, r10, 0x3ecc
	ctx.r[3].s64 = ctx.r[10].s64 + 16076;
	// 826C1BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1BDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1BE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C1BE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1BE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C1BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1BF0: 4BDA5231  bl 0x82466e20
	ctx.lr = 0x826C1BF4;
	sub_82466E20(ctx, base);
	// 826C1BF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1C08 size=112
    let mut pc: u32 = 0x826C1C08;
    'dispatch: loop {
        match pc {
            0x826C1C08 => {
    //   block [0x826C1C08..0x826C1C78)
	// 826C1C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1C14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1C18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1C1C: 38AA3ECC  addi r5, r10, 0x3ecc
	ctx.r[5].s64 = ctx.r[10].s64 + 16076;
	// 826C1C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1C24: 390B2570  addi r8, r11, 0x2570
	ctx.r[8].s64 = ctx.r[11].s64 + 9584;
	// 826C1C28: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C1C2C: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826C1C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1C34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1C38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C1C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1C40: 386A3EFC  addi r3, r10, 0x3efc
	ctx.r[3].s64 = ctx.r[10].s64 + 16124;
	// 826C1C44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1C4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1C64: 4BDA51BD  bl 0x82466e20
	ctx.lr = 0x826C1C68;
	sub_82466E20(ctx, base);
	// 826C1C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1C78 size=108
    let mut pc: u32 = 0x826C1C78;
    'dispatch: loop {
        match pc {
            0x826C1C78 => {
    //   block [0x826C1C78..0x826C1CE4)
	// 826C1C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1C84: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1C8C: 38EB25B8  addi r7, r11, 0x25b8
	ctx.r[7].s64 = ctx.r[11].s64 + 9656;
	// 826C1C90: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C1C94: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 826C1C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1C9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1CA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1CA8: 386A3F2C  addi r3, r10, 0x3f2c
	ctx.r[3].s64 = ctx.r[10].s64 + 16172;
	// 826C1CAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1CCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1CD0: 4BDA5151  bl 0x82466e20
	ctx.lr = 0x826C1CD4;
	sub_82466E20(ctx, base);
	// 826C1CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1CE8 size=112
    let mut pc: u32 = 0x826C1CE8;
    'dispatch: loop {
        match pc {
            0x826C1CE8 => {
    //   block [0x826C1CE8..0x826C1D58)
	// 826C1CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1CF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1CF8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1CFC: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1D04: 390B2600  addi r8, r11, 0x2600
	ctx.r[8].s64 = ctx.r[11].s64 + 9728;
	// 826C1D08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C1D0C: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826C1D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1D14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C1D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1D20: 386A3F5C  addi r3, r10, 0x3f5c
	ctx.r[3].s64 = ctx.r[10].s64 + 16220;
	// 826C1D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1D44: 4BDA50DD  bl 0x82466e20
	ctx.lr = 0x826C1D48;
	sub_82466E20(ctx, base);
	// 826C1D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1D58 size=108
    let mut pc: u32 = 0x826C1D58;
    'dispatch: loop {
        match pc {
            0x826C1D58 => {
    //   block [0x826C1D58..0x826C1DC4)
	// 826C1D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1D64: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1D6C: 38EB2618  addi r7, r11, 0x2618
	ctx.r[7].s64 = ctx.r[11].s64 + 9752;
	// 826C1D70: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C1D74: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 826C1D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1D7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1D88: 386A3F8C  addi r3, r10, 0x3f8c
	ctx.r[3].s64 = ctx.r[10].s64 + 16268;
	// 826C1D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1DB0: 4BDA5071  bl 0x82466e20
	ctx.lr = 0x826C1DB4;
	sub_82466E20(ctx, base);
	// 826C1DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1DC8 size=112
    let mut pc: u32 = 0x826C1DC8;
    'dispatch: loop {
        match pc {
            0x826C1DC8 => {
    //   block [0x826C1DC8..0x826C1E38)
	// 826C1DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1DD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1DD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1DDC: 38AA3F5C  addi r5, r10, 0x3f5c
	ctx.r[5].s64 = ctx.r[10].s64 + 16220;
	// 826C1DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1DE4: 390B2660  addi r8, r11, 0x2660
	ctx.r[8].s64 = ctx.r[11].s64 + 9824;
	// 826C1DE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C1DEC: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 826C1DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1DF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C1DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1E00: 386A3FBC  addi r3, r10, 0x3fbc
	ctx.r[3].s64 = ctx.r[10].s64 + 16316;
	// 826C1E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1E24: 4BDA4FFD  bl 0x82466e20
	ctx.lr = 0x826C1E28;
	sub_82466E20(ctx, base);
	// 826C1E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1E38 size=100
    let mut pc: u32 = 0x826C1E38;
    'dispatch: loop {
        match pc {
            0x826C1E38 => {
    //   block [0x826C1E38..0x826C1E9C)
	// 826C1E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1E44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1E4C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1E58: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 826C1E5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1E6C: 386A3FEC  addi r3, r10, 0x3fec
	ctx.r[3].s64 = ctx.r[10].s64 + 16364;
	// 826C1E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1E74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1E78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C1E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1E80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C1E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1E88: 4BDA4F99  bl 0x82466e20
	ctx.lr = 0x826C1E8C;
	sub_82466E20(ctx, base);
	// 826C1E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1EA0 size=112
    let mut pc: u32 = 0x826C1EA0;
    'dispatch: loop {
        match pc {
            0x826C1EA0 => {
    //   block [0x826C1EA0..0x826C1F10)
	// 826C1EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1EAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1EB0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1EB4: 38AA3FEC  addi r5, r10, 0x3fec
	ctx.r[5].s64 = ctx.r[10].s64 + 16364;
	// 826C1EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1EBC: 390B2678  addi r8, r11, 0x2678
	ctx.r[8].s64 = ctx.r[11].s64 + 9848;
	// 826C1EC0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826C1EC4: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 826C1EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1ECC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C1ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1ED8: 386A401C  addi r3, r10, 0x401c
	ctx.r[3].s64 = ctx.r[10].s64 + 16412;
	// 826C1EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1EFC: 4BDA4F25  bl 0x82466e20
	ctx.lr = 0x826C1F00;
	sub_82466E20(ctx, base);
	// 826C1F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1F10 size=108
    let mut pc: u32 = 0x826C1F10;
    'dispatch: loop {
        match pc {
            0x826C1F10 => {
    //   block [0x826C1F10..0x826C1F7C)
	// 826C1F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1F1C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1F24: 38EB2720  addi r7, r11, 0x2720
	ctx.r[7].s64 = ctx.r[11].s64 + 10016;
	// 826C1F28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C1F2C: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826C1F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1F34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1F38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1F40: 386A404C  addi r3, r10, 0x404c
	ctx.r[3].s64 = ctx.r[10].s64 + 16460;
	// 826C1F44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1F64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1F68: 4BDA4EB9  bl 0x82466e20
	ctx.lr = 0x826C1F6C;
	sub_82466E20(ctx, base);
	// 826C1F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1F80 size=112
    let mut pc: u32 = 0x826C1F80;
    'dispatch: loop {
        match pc {
            0x826C1F80 => {
    //   block [0x826C1F80..0x826C1FF0)
	// 826C1F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1F8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1F90: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1F94: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1F9C: 390B2750  addi r8, r11, 0x2750
	ctx.r[8].s64 = ctx.r[11].s64 + 10064;
	// 826C1FA0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C1FA4: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 826C1FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1FAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1FB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C1FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1FB8: 386A407C  addi r3, r10, 0x407c
	ctx.r[3].s64 = ctx.r[10].s64 + 16508;
	// 826C1FBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1FDC: 4BDA4E45  bl 0x82466e20
	ctx.lr = 0x826C1FE0;
	sub_82466E20(ctx, base);
	// 826C1FE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1FF0 size=112
    let mut pc: u32 = 0x826C1FF0;
    'dispatch: loop {
        match pc {
            0x826C1FF0 => {
    //   block [0x826C1FF0..0x826C2060)
	// 826C1FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1FFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2000: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2004: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C2008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C200C: 390B2798  addi r8, r11, 0x2798
	ctx.r[8].s64 = ctx.r[11].s64 + 10136;
	// 826C2010: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C2014: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826C2018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C201C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2028: 386A40AC  addi r3, r10, 0x40ac
	ctx.r[3].s64 = ctx.r[10].s64 + 16556;
	// 826C202C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C203C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C204C: 4BDA4DD5  bl 0x82466e20
	ctx.lr = 0x826C2050;
	sub_82466E20(ctx, base);
	// 826C2050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C205C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2060 size=100
    let mut pc: u32 = 0x826C2060;
    'dispatch: loop {
        match pc {
            0x826C2060 => {
    //   block [0x826C2060..0x826C20C4)
	// 826C2060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C206C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2074: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C2078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C207C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2080: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 826C2084: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C208C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2094: 386A40DC  addi r3, r10, 0x40dc
	ctx.r[3].s64 = ctx.r[10].s64 + 16604;
	// 826C2098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C209C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C20A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C20A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C20A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C20AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C20B0: 4BDA4D71  bl 0x82466e20
	ctx.lr = 0x826C20B4;
	sub_82466E20(ctx, base);
	// 826C20B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C20B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C20BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C20C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C20C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C20C8 size=112
    let mut pc: u32 = 0x826C20C8;
    'dispatch: loop {
        match pc {
            0x826C20C8 => {
    //   block [0x826C20C8..0x826C2138)
	// 826C20C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C20CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C20D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C20D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C20D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C20DC: 38AA40DC  addi r5, r10, 0x40dc
	ctx.r[5].s64 = ctx.r[10].s64 + 16604;
	// 826C20E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C20E4: 390B27E0  addi r8, r11, 0x27e0
	ctx.r[8].s64 = ctx.r[11].s64 + 10208;
	// 826C20E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C20EC: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 826C20F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C20F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C20F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C20FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2100: 386A410C  addi r3, r10, 0x410c
	ctx.r[3].s64 = ctx.r[10].s64 + 16652;
	// 826C2104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C210C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C211C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2124: 4BDA4CFD  bl 0x82466e20
	ctx.lr = 0x826C2128;
	sub_82466E20(ctx, base);
	// 826C2128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C212C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2138 size=112
    let mut pc: u32 = 0x826C2138;
    'dispatch: loop {
        match pc {
            0x826C2138 => {
    //   block [0x826C2138..0x826C21A8)
	// 826C2138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C213C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2144: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2148: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C214C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C2150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2154: 390B2828  addi r8, r11, 0x2828
	ctx.r[8].s64 = ctx.r[11].s64 + 10280;
	// 826C2158: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C215C: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 826C2160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2164: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C216C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2170: 386A413C  addi r3, r10, 0x413c
	ctx.r[3].s64 = ctx.r[10].s64 + 16700;
	// 826C2174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C217C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C218C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2194: 4BDA4C8D  bl 0x82466e20
	ctx.lr = 0x826C2198;
	sub_82466E20(ctx, base);
	// 826C2198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C219C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C21A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C21A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C21A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C21A8 size=112
    let mut pc: u32 = 0x826C21A8;
    'dispatch: loop {
        match pc {
            0x826C21A8 => {
    //   block [0x826C21A8..0x826C2218)
	// 826C21A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C21AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C21B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C21B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C21B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C21BC: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C21C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C21C4: 390B2840  addi r8, r11, 0x2840
	ctx.r[8].s64 = ctx.r[11].s64 + 10304;
	// 826C21C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C21CC: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 826C21D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C21D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C21D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C21DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C21E0: 386A416C  addi r3, r10, 0x416c
	ctx.r[3].s64 = ctx.r[10].s64 + 16748;
	// 826C21E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C21E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C21EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C21F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C21F4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C21F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C21FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2204: 4BDA4C1D  bl 0x82466e20
	ctx.lr = 0x826C2208;
	sub_82466E20(ctx, base);
	// 826C2208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C220C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2218 size=112
    let mut pc: u32 = 0x826C2218;
    'dispatch: loop {
        match pc {
            0x826C2218 => {
    //   block [0x826C2218..0x826C2288)
	// 826C2218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C221C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2224: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2228: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C222C: 38AA413C  addi r5, r10, 0x413c
	ctx.r[5].s64 = ctx.r[10].s64 + 16700;
	// 826C2230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2234: 390B2858  addi r8, r11, 0x2858
	ctx.r[8].s64 = ctx.r[11].s64 + 10328;
	// 826C2238: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C223C: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826C2240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2244: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C224C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2250: 386A419C  addi r3, r10, 0x419c
	ctx.r[3].s64 = ctx.r[10].s64 + 16796;
	// 826C2254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C225C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C226C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2274: 4BDA4BAD  bl 0x82466e20
	ctx.lr = 0x826C2278;
	sub_82466E20(ctx, base);
	// 826C2278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C227C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2288 size=72
    let mut pc: u32 = 0x826C2288;
    'dispatch: loop {
        match pc {
            0x826C2288 => {
    //   block [0x826C2288..0x826C22D0)
	// 826C2288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C228C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2294: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C2298: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 826C229C: 38CB02B8  addi r6, r11, 0x2b8
	ctx.r[6].s64 = ctx.r[11].s64 + 696;
	// 826C22A0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C22A4: 388B1530  addi r4, r11, 0x1530
	ctx.r[4].s64 = ctx.r[11].s64 + 5424;
	// 826C22A8: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C22AC: 386B41CC  addi r3, r11, 0x41cc
	ctx.r[3].s64 = ctx.r[11].s64 + 16844;
	// 826C22B0: 4BDB97D9  bl 0x8247ba88
	ctx.lr = 0x826C22B4;
	sub_8247BA88(ctx, base);
	// 826C22B4: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826C22B8: 386BCED0  addi r3, r11, -0x3130
	ctx.r[3].s64 = ctx.r[11].s64 + -12592;
	// 826C22BC: 4BE7087D  bl 0x82532b38
	ctx.lr = 0x826C22C0;
	sub_82532B38(ctx, base);
	// 826C22C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826C22C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C22C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C22CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C22D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C22D0 size=108
    let mut pc: u32 = 0x826C22D0;
    'dispatch: loop {
        match pc {
            0x826C22D0 => {
    //   block [0x826C22D0..0x826C233C)
	// 826C22D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C22D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C22D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C22DC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C22E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C22E4: 38EB37D0  addi r7, r11, 0x37d0
	ctx.r[7].s64 = ctx.r[11].s64 + 14288;
	// 826C22E8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826C22EC: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826C22F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C22F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C22F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C22FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2300: 386A41E4  addi r3, r10, 0x41e4
	ctx.r[3].s64 = ctx.r[10].s64 + 16868;
	// 826C2304: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C230C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C231C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2328: 4BDA4AF9  bl 0x82466e20
	ctx.lr = 0x826C232C;
	sub_82466E20(ctx, base);
	// 826C232C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C2340 size=24
    let mut pc: u32 = 0x826C2340;
    'dispatch: loop {
        match pc {
            0x826C2340 => {
    //   block [0x826C2340..0x826C2358)
	// 826C2340: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2344: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C2348: 394A9AD0  addi r10, r10, -0x6530
	ctx.r[10].s64 = ctx.r[10].s64 + -25904;
	// 826C234C: 816B3848  lwz r11, 0x3848(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14408 as u32) ) } as u64;
	// 826C2350: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826C2354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2358 size=112
    let mut pc: u32 = 0x826C2358;
    'dispatch: loop {
        match pc {
            0x826C2358 => {
    //   block [0x826C2358..0x826C23C8)
	// 826C2358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C235C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2364: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C2368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C236C: 392B1AC4  addi r9, r11, 0x1ac4
	ctx.r[9].s64 = ctx.r[11].s64 + 6852;
	// 826C2370: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 826C2374: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826C2378: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C237C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 826C2380: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2384: 396B9AD0  addi r11, r11, -0x6530
	ctx.r[11].s64 = ctx.r[11].s64 + -25904;
	// 826C2388: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C238C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2390: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C2394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2398: 386A4214  addi r3, r10, 0x4214
	ctx.r[3].s64 = ctx.r[10].s64 + 16916;
	// 826C239C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C23A0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C23A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C23A8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C23AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C23B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C23B4: 4BDA4A6D  bl 0x82466e20
	ctx.lr = 0x826C23B8;
	sub_82466E20(ctx, base);
	// 826C23B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C23BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C23C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C23C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C23C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C23C8 size=108
    let mut pc: u32 = 0x826C23C8;
    'dispatch: loop {
        match pc {
            0x826C23C8 => {
    //   block [0x826C23C8..0x826C2434)
	// 826C23C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C23CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C23D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C23D4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C23D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C23DC: 38EB384C  addi r7, r11, 0x384c
	ctx.r[7].s64 = ctx.r[11].s64 + 14412;
	// 826C23E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C23E4: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 826C23E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C23EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C23F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C23F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C23F8: 386A4244  addi r3, r10, 0x4244
	ctx.r[3].s64 = ctx.r[10].s64 + 16964;
	// 826C23FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C240C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C241C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2420: 4BDA4A01  bl 0x82466e20
	ctx.lr = 0x826C2424;
	sub_82466E20(ctx, base);
	// 826C2424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C242C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2438 size=108
    let mut pc: u32 = 0x826C2438;
    'dispatch: loop {
        match pc {
            0x826C2438 => {
    //   block [0x826C2438..0x826C24A4)
	// 826C2438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C243C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2444: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C244C: 38EB387C  addi r7, r11, 0x387c
	ctx.r[7].s64 = ctx.r[11].s64 + 14460;
	// 826C2450: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C2454: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 826C2458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C245C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2460: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2468: 386A4274  addi r3, r10, 0x4274
	ctx.r[3].s64 = ctx.r[10].s64 + 17012;
	// 826C246C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C247C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C248C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2490: 4BDA4991  bl 0x82466e20
	ctx.lr = 0x826C2494;
	sub_82466E20(ctx, base);
	// 826C2494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C249C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C24A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C24A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C24A8 size=112
    let mut pc: u32 = 0x826C24A8;
    'dispatch: loop {
        match pc {
            0x826C24A8 => {
    //   block [0x826C24A8..0x826C2518)
	// 826C24A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C24AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C24B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C24B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C24B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C24BC: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C24C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C24C4: 390B38B0  addi r8, r11, 0x38b0
	ctx.r[8].s64 = ctx.r[11].s64 + 14512;
	// 826C24C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C24CC: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826C24D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C24D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C24D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C24DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C24E0: 386A42A4  addi r3, r10, 0x42a4
	ctx.r[3].s64 = ctx.r[10].s64 + 17060;
	// 826C24E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C24E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C24EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C24F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C24F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C24F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C24FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2504: 4BDA491D  bl 0x82466e20
	ctx.lr = 0x826C2508;
	sub_82466E20(ctx, base);
	// 826C2508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C250C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2518 size=108
    let mut pc: u32 = 0x826C2518;
    'dispatch: loop {
        match pc {
            0x826C2518 => {
    //   block [0x826C2518..0x826C2584)
	// 826C2518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C251C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2524: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C252C: 38EB3910  addi r7, r11, 0x3910
	ctx.r[7].s64 = ctx.r[11].s64 + 14608;
	// 826C2530: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826C2534: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 826C2538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C253C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2540: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2548: 386A42D4  addi r3, r10, 0x42d4
	ctx.r[3].s64 = ctx.r[10].s64 + 17108;
	// 826C254C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C255C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C256C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2570: 4BDA48B1  bl 0x82466e20
	ctx.lr = 0x826C2574;
	sub_82466E20(ctx, base);
	// 826C2574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C257C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2588 size=112
    let mut pc: u32 = 0x826C2588;
    'dispatch: loop {
        match pc {
            0x826C2588 => {
    //   block [0x826C2588..0x826C25F8)
	// 826C2588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C258C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2594: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2598: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C259C: 38AA42A4  addi r5, r10, 0x42a4
	ctx.r[5].s64 = ctx.r[10].s64 + 17060;
	// 826C25A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C25A4: 390B3988  addi r8, r11, 0x3988
	ctx.r[8].s64 = ctx.r[11].s64 + 14728;
	// 826C25A8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826C25AC: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 826C25B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C25B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C25B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C25BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C25C0: 386A4304  addi r3, r10, 0x4304
	ctx.r[3].s64 = ctx.r[10].s64 + 17156;
	// 826C25C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C25C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C25CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C25D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C25D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C25D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C25DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C25E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C25E4: 4BDA483D  bl 0x82466e20
	ctx.lr = 0x826C25E8;
	sub_82466E20(ctx, base);
	// 826C25E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C25EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C25F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C25F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C25F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C25F8 size=112
    let mut pc: u32 = 0x826C25F8;
    'dispatch: loop {
        match pc {
            0x826C25F8 => {
    //   block [0x826C25F8..0x826C2668)
	// 826C25F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C25FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2608: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C260C: 38AA42A4  addi r5, r10, 0x42a4
	ctx.r[5].s64 = ctx.r[10].s64 + 17060;
	// 826C2610: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2614: 390B3A30  addi r8, r11, 0x3a30
	ctx.r[8].s64 = ctx.r[11].s64 + 14896;
	// 826C2618: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C261C: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826C2620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2624: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2628: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C262C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2630: 386A4334  addi r3, r10, 0x4334
	ctx.r[3].s64 = ctx.r[10].s64 + 17204;
	// 826C2634: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2638: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C263C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C264C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2654: 4BDA47CD  bl 0x82466e20
	ctx.lr = 0x826C2658;
	sub_82466E20(ctx, base);
	// 826C2658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C265C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2668 size=108
    let mut pc: u32 = 0x826C2668;
    'dispatch: loop {
        match pc {
            0x826C2668 => {
    //   block [0x826C2668..0x826C26D4)
	// 826C2668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C266C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2674: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C267C: 38EB3A48  addi r7, r11, 0x3a48
	ctx.r[7].s64 = ctx.r[11].s64 + 14920;
	// 826C2680: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826C2684: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 826C2688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C268C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2698: 386A4364  addi r3, r10, 0x4364
	ctx.r[3].s64 = ctx.r[10].s64 + 17252;
	// 826C269C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C26A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C26A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C26A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C26AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C26B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C26B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C26B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C26BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C26C0: 4BDA4761  bl 0x82466e20
	ctx.lr = 0x826C26C4;
	sub_82466E20(ctx, base);
	// 826C26C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C26C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C26CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C26D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C26D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C26D8 size=112
    let mut pc: u32 = 0x826C26D8;
    'dispatch: loop {
        match pc {
            0x826C26D8 => {
    //   block [0x826C26D8..0x826C2748)
	// 826C26D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C26DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C26E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C26E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C26E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C26EC: 38AA42A4  addi r5, r10, 0x42a4
	ctx.r[5].s64 = ctx.r[10].s64 + 17060;
	// 826C26F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C26F4: 390B3AC0  addi r8, r11, 0x3ac0
	ctx.r[8].s64 = ctx.r[11].s64 + 15040;
	// 826C26F8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826C26FC: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826C2700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2704: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C270C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2710: 386A4394  addi r3, r10, 0x4394
	ctx.r[3].s64 = ctx.r[10].s64 + 17300;
	// 826C2714: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C271C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C272C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2734: 4BDA46ED  bl 0x82466e20
	ctx.lr = 0x826C2738;
	sub_82466E20(ctx, base);
	// 826C2738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C273C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2748 size=108
    let mut pc: u32 = 0x826C2748;
    'dispatch: loop {
        match pc {
            0x826C2748 => {
    //   block [0x826C2748..0x826C27B4)
	// 826C2748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C274C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2754: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C275C: 38EB3B68  addi r7, r11, 0x3b68
	ctx.r[7].s64 = ctx.r[11].s64 + 15208;
	// 826C2760: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C2764: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 826C2768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C276C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2778: 386A43C4  addi r3, r10, 0x43c4
	ctx.r[3].s64 = ctx.r[10].s64 + 17348;
	// 826C277C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C278C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C279C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C27A0: 4BDA4681  bl 0x82466e20
	ctx.lr = 0x826C27A4;
	sub_82466E20(ctx, base);
	// 826C27A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C27A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C27AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C27B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C27B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C27B8 size=108
    let mut pc: u32 = 0x826C27B8;
    'dispatch: loop {
        match pc {
            0x826C27B8 => {
    //   block [0x826C27B8..0x826C2824)
	// 826C27B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C27BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C27C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C27C4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C27C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C27CC: 38EB3B80  addi r7, r11, 0x3b80
	ctx.r[7].s64 = ctx.r[11].s64 + 15232;
	// 826C27D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C27D4: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826C27D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C27DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C27E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C27E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C27E8: 386A43F4  addi r3, r10, 0x43f4
	ctx.r[3].s64 = ctx.r[10].s64 + 17396;
	// 826C27EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C27F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C27F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C27F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C27FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C280C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2810: 4BDA4611  bl 0x82466e20
	ctx.lr = 0x826C2814;
	sub_82466E20(ctx, base);
	// 826C2814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C281C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2828 size=116
    let mut pc: u32 = 0x826C2828;
    'dispatch: loop {
        match pc {
            0x826C2828 => {
    //   block [0x826C2828..0x826C289C)
	// 826C2828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C282C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2834: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2838: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C283C: 390B3BE0  addi r8, r11, 0x3be0
	ctx.r[8].s64 = ctx.r[11].s64 + 15328;
	// 826C2840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2844: 392A1B00  addi r9, r10, 0x1b00
	ctx.r[9].s64 = ctx.r[10].s64 + 6912;
	// 826C2848: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C284C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826C2850: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C2854: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C285C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C286C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C2870: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 826C2874: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C2878: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 826C287C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C2880: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2888: 4BDA4599  bl 0x82466e20
	ctx.lr = 0x826C288C;
	sub_82466E20(ctx, base);
	// 826C288C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C28A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C28A0 size=108
    let mut pc: u32 = 0x826C28A0;
    'dispatch: loop {
        match pc {
            0x826C28A0 => {
    //   block [0x826C28A0..0x826C290C)
	// 826C28A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C28A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C28A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C28AC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C28B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C28B4: 38EB3BF8  addi r7, r11, 0x3bf8
	ctx.r[7].s64 = ctx.r[11].s64 + 15352;
	// 826C28B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C28BC: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 826C28C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C28C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C28C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C28CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C28D0: 386A4454  addi r3, r10, 0x4454
	ctx.r[3].s64 = ctx.r[10].s64 + 17492;
	// 826C28D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C28D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C28DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C28E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C28E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C28E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C28EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C28F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C28F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C28F8: 4BDA4529  bl 0x82466e20
	ctx.lr = 0x826C28FC;
	sub_82466E20(ctx, base);
	// 826C28FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2910 size=108
    let mut pc: u32 = 0x826C2910;
    'dispatch: loop {
        match pc {
            0x826C2910 => {
    //   block [0x826C2910..0x826C297C)
	// 826C2910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C291C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2924: 38EB3C40  addi r7, r11, 0x3c40
	ctx.r[7].s64 = ctx.r[11].s64 + 15424;
	// 826C2928: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826C292C: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826C2930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2934: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2938: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C293C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2940: 386A4484  addi r3, r10, 0x4484
	ctx.r[3].s64 = ctx.r[10].s64 + 17540;
	// 826C2944: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C294C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C295C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2964: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2968: 4BDA44B9  bl 0x82466e20
	ctx.lr = 0x826C296C;
	sub_82466E20(ctx, base);
	// 826C296C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2980 size=108
    let mut pc: u32 = 0x826C2980;
    'dispatch: loop {
        match pc {
            0x826C2980 => {
    //   block [0x826C2980..0x826C29EC)
	// 826C2980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C298C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2994: 38EB3CD0  addi r7, r11, 0x3cd0
	ctx.r[7].s64 = ctx.r[11].s64 + 15568;
	// 826C2998: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826C299C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 826C29A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C29A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C29A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C29AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C29B0: 386A44B4  addi r3, r10, 0x44b4
	ctx.r[3].s64 = ctx.r[10].s64 + 17588;
	// 826C29B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C29B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C29BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C29C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C29C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C29C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C29CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C29D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C29D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C29D8: 4BDA4449  bl 0x82466e20
	ctx.lr = 0x826C29DC;
	sub_82466E20(ctx, base);
	// 826C29DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C29E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C29E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C29E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C29F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C29F0 size=100
    let mut pc: u32 = 0x826C29F0;
    'dispatch: loop {
        match pc {
            0x826C29F0 => {
    //   block [0x826C29F0..0x826C2A54)
	// 826C29F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C29F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C29F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C29FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2A04: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C2A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2A10: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826C2A14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2A24: 386A44E4  addi r3, r10, 0x44e4
	ctx.r[3].s64 = ctx.r[10].s64 + 17636;
	// 826C2A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2A2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2A30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C2A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2A38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C2A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2A40: 4BDA43E1  bl 0x82466e20
	ctx.lr = 0x826C2A44;
	sub_82466E20(ctx, base);
	// 826C2A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2A58 size=112
    let mut pc: u32 = 0x826C2A58;
    'dispatch: loop {
        match pc {
            0x826C2A58 => {
    //   block [0x826C2A58..0x826C2AC8)
	// 826C2A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2A64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2A68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2A6C: 38AA44E4  addi r5, r10, 0x44e4
	ctx.r[5].s64 = ctx.r[10].s64 + 17636;
	// 826C2A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2A74: 390B3D60  addi r8, r11, 0x3d60
	ctx.r[8].s64 = ctx.r[11].s64 + 15712;
	// 826C2A78: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C2A7C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 826C2A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2A84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2A90: 386A4514  addi r3, r10, 0x4514
	ctx.r[3].s64 = ctx.r[10].s64 + 17684;
	// 826C2A94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2AB4: 4BDA436D  bl 0x82466e20
	ctx.lr = 0x826C2AB8;
	sub_82466E20(ctx, base);
	// 826C2AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2AC8 size=108
    let mut pc: u32 = 0x826C2AC8;
    'dispatch: loop {
        match pc {
            0x826C2AC8 => {
    //   block [0x826C2AC8..0x826C2B34)
	// 826C2AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2AD4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2ADC: 38EB3DC0  addi r7, r11, 0x3dc0
	ctx.r[7].s64 = ctx.r[11].s64 + 15808;
	// 826C2AE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C2AE4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826C2AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2AEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2AF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2AF8: 386A4544  addi r3, r10, 0x4544
	ctx.r[3].s64 = ctx.r[10].s64 + 17732;
	// 826C2AFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2B1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2B20: 4BDA4301  bl 0x82466e20
	ctx.lr = 0x826C2B24;
	sub_82466E20(ctx, base);
	// 826C2B24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2B38 size=108
    let mut pc: u32 = 0x826C2B38;
    'dispatch: loop {
        match pc {
            0x826C2B38 => {
    //   block [0x826C2B38..0x826C2BA4)
	// 826C2B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2B44: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2B4C: 38EB3DF0  addi r7, r11, 0x3df0
	ctx.r[7].s64 = ctx.r[11].s64 + 15856;
	// 826C2B50: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C2B54: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 826C2B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2B5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2B60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2B68: 386A4574  addi r3, r10, 0x4574
	ctx.r[3].s64 = ctx.r[10].s64 + 17780;
	// 826C2B6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2B8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2B90: 4BDA4291  bl 0x82466e20
	ctx.lr = 0x826C2B94;
	sub_82466E20(ctx, base);
	// 826C2B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2BA8 size=108
    let mut pc: u32 = 0x826C2BA8;
    'dispatch: loop {
        match pc {
            0x826C2BA8 => {
    //   block [0x826C2BA8..0x826C2C14)
	// 826C2BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2BB4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2BBC: 38EB3E50  addi r7, r11, 0x3e50
	ctx.r[7].s64 = ctx.r[11].s64 + 15952;
	// 826C2BC0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C2BC4: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 826C2BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2BCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2BD8: 386A45A4  addi r3, r10, 0x45a4
	ctx.r[3].s64 = ctx.r[10].s64 + 17828;
	// 826C2BDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2BFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2C00: 4BDA4221  bl 0x82466e20
	ctx.lr = 0x826C2C04;
	sub_82466E20(ctx, base);
	// 826C2C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2C18 size=108
    let mut pc: u32 = 0x826C2C18;
    'dispatch: loop {
        match pc {
            0x826C2C18 => {
    //   block [0x826C2C18..0x826C2C84)
	// 826C2C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2C24: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2C28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2C2C: 38EB3EB0  addi r7, r11, 0x3eb0
	ctx.r[7].s64 = ctx.r[11].s64 + 16048;
	// 826C2C30: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826C2C34: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 826C2C38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2C3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2C40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2C44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2C48: 386A45D4  addi r3, r10, 0x45d4
	ctx.r[3].s64 = ctx.r[10].s64 + 17876;
	// 826C2C4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2C50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2C58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2C60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2C68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2C6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2C70: 4BDA41B1  bl 0x82466e20
	ctx.lr = 0x826C2C74;
	sub_82466E20(ctx, base);
	// 826C2C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2C88 size=112
    let mut pc: u32 = 0x826C2C88;
    'dispatch: loop {
        match pc {
            0x826C2C88 => {
    //   block [0x826C2C88..0x826C2CF8)
	// 826C2C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2C94: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C2C98: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2C9C: 392A1B34  addi r9, r10, 0x1b34
	ctx.r[9].s64 = ctx.r[10].s64 + 6964;
	// 826C2CA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2CA4: 390B3F30  addi r8, r11, 0x3f30
	ctx.r[8].s64 = ctx.r[11].s64 + 16176;
	// 826C2CA8: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826C2CAC: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826C2CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2CB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2CB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2CC0: 386A4604  addi r3, r10, 0x4604
	ctx.r[3].s64 = ctx.r[10].s64 + 17924;
	// 826C2CC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C2CC8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C2CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2CD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2CD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2CD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2CDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2CE4: 4BDA413D  bl 0x82466e20
	ctx.lr = 0x826C2CE8;
	sub_82466E20(ctx, base);
	// 826C2CE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2CF8 size=112
    let mut pc: u32 = 0x826C2CF8;
    'dispatch: loop {
        match pc {
            0x826C2CF8 => {
    //   block [0x826C2CF8..0x826C2D68)
	// 826C2CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2D04: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C2D08: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826C2D0C: 38EA4038  addi r7, r10, 0x4038
	ctx.r[7].s64 = ctx.r[10].s64 + 16440;
	// 826C2D10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2D14: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C2D18: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 826C2D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2D20: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2D24: 396B1B48  addi r11, r11, 0x1b48
	ctx.r[11].s64 = ctx.r[11].s64 + 6984;
	// 826C2D28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2D2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2D30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2D34: 386A4634  addi r3, r10, 0x4634
	ctx.r[3].s64 = ctx.r[10].s64 + 17972;
	// 826C2D38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2D3C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C2D40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2D44: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C2D48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2D4C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2D50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2D54: 4BDA40CD  bl 0x82466e20
	ctx.lr = 0x826C2D58;
	sub_82466E20(ctx, base);
	// 826C2D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2D68 size=112
    let mut pc: u32 = 0x826C2D68;
    'dispatch: loop {
        match pc {
            0x826C2D68 => {
    //   block [0x826C2D68..0x826C2DD8)
	// 826C2D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2D74: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C2D78: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2D7C: 392A1B8C  addi r9, r10, 0x1b8c
	ctx.r[9].s64 = ctx.r[10].s64 + 7052;
	// 826C2D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2D84: 390B4140  addi r8, r11, 0x4140
	ctx.r[8].s64 = ctx.r[11].s64 + 16704;
	// 826C2D88: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826C2D8C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826C2D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2D94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2D98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2DA0: 386A4664  addi r3, r10, 0x4664
	ctx.r[3].s64 = ctx.r[10].s64 + 18020;
	// 826C2DA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C2DA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C2DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2DBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2DC4: 4BDA405D  bl 0x82466e20
	ctx.lr = 0x826C2DC8;
	sub_82466E20(ctx, base);
	// 826C2DC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2DD8 size=100
    let mut pc: u32 = 0x826C2DD8;
    'dispatch: loop {
        match pc {
            0x826C2DD8 => {
    //   block [0x826C2DD8..0x826C2E3C)
	// 826C2DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2DE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2DEC: 38AA4C34  addi r5, r10, 0x4c34
	ctx.r[5].s64 = ctx.r[10].s64 + 19508;
	// 826C2DF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2DF8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826C2DFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2E0C: 386A4694  addi r3, r10, 0x4694
	ctx.r[3].s64 = ctx.r[10].s64 + 18068;
	// 826C2E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2E14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2E18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C2E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2E20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C2E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2E28: 4BDA3FF9  bl 0x82466e20
	ctx.lr = 0x826C2E2C;
	sub_82466E20(ctx, base);
	// 826C2E2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2E40 size=116
    let mut pc: u32 = 0x826C2E40;
    'dispatch: loop {
        match pc {
            0x826C2E40 => {
    //   block [0x826C2E40..0x826C2EB4)
	// 826C2E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2E4C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C2E50: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C2E54: 390A4170  addi r8, r10, 0x4170
	ctx.r[8].s64 = ctx.r[10].s64 + 16752;
	// 826C2E58: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2E5C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C2E60: 38AA4694  addi r5, r10, 0x4694
	ctx.r[5].s64 = ctx.r[10].s64 + 18068;
	// 826C2E64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2E68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C2E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2E70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2E74: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826C2E78: 396B1BA0  addi r11, r11, 0x1ba0
	ctx.r[11].s64 = ctx.r[11].s64 + 7072;
	// 826C2E7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2E80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2E84: 386A46C4  addi r3, r10, 0x46c4
	ctx.r[3].s64 = ctx.r[10].s64 + 18116;
	// 826C2E88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C2E8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2E90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C2E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2E9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2EA0: 4BDA3F81  bl 0x82466e20
	ctx.lr = 0x826C2EA4;
	sub_82466E20(ctx, base);
	// 826C2EA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2EB8 size=100
    let mut pc: u32 = 0x826C2EB8;
    'dispatch: loop {
        match pc {
            0x826C2EB8 => {
    //   block [0x826C2EB8..0x826C2F1C)
	// 826C2EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2EC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2ECC: 38AA46C4  addi r5, r10, 0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + 18116;
	// 826C2ED0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2ED8: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826C2EDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2EEC: 386A46F4  addi r3, r10, 0x46f4
	ctx.r[3].s64 = ctx.r[10].s64 + 18164;
	// 826C2EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2EF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2EF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C2EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2F00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C2F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2F08: 4BDA3F19  bl 0x82466e20
	ctx.lr = 0x826C2F0C;
	sub_82466E20(ctx, base);
	// 826C2F0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2F20 size=112
    let mut pc: u32 = 0x826C2F20;
    'dispatch: loop {
        match pc {
            0x826C2F20 => {
    //   block [0x826C2F20..0x826C2F90)
	// 826C2F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2F2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2F30: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2F34: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C2F38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2F3C: 390B4218  addi r8, r11, 0x4218
	ctx.r[8].s64 = ctx.r[11].s64 + 16920;
	// 826C2F40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C2F44: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826C2F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2F4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2F50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2F58: 386A4724  addi r3, r10, 0x4724
	ctx.r[3].s64 = ctx.r[10].s64 + 18212;
	// 826C2F5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2F60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2F64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2F68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2F6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2F70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2F74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2F78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2F7C: 4BDA3EA5  bl 0x82466e20
	ctx.lr = 0x826C2F80;
	sub_82466E20(ctx, base);
	// 826C2F80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2F84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2F88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2F90 size=116
    let mut pc: u32 = 0x826C2F90;
    'dispatch: loop {
        match pc {
            0x826C2F90 => {
    //   block [0x826C2F90..0x826C3004)
	// 826C2F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2F9C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C2FA0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C2FA4: 390A4260  addi r8, r10, 0x4260
	ctx.r[8].s64 = ctx.r[10].s64 + 16992;
	// 826C2FA8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2FAC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C2FB0: 38AA4694  addi r5, r10, 0x4694
	ctx.r[5].s64 = ctx.r[10].s64 + 18068;
	// 826C2FB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2FB8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C2FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2FC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2FC4: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826C2FC8: 396B1BCC  addi r11, r11, 0x1bcc
	ctx.r[11].s64 = ctx.r[11].s64 + 7116;
	// 826C2FCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2FD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2FD4: 386A4754  addi r3, r10, 0x4754
	ctx.r[3].s64 = ctx.r[10].s64 + 18260;
	// 826C2FD8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C2FDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2FE0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C2FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2FF0: 4BDA3E31  bl 0x82466e20
	ctx.lr = 0x826C2FF4;
	sub_82466E20(ctx, base);
	// 826C2FF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3008 size=108
    let mut pc: u32 = 0x826C3008;
    'dispatch: loop {
        match pc {
            0x826C3008 => {
    //   block [0x826C3008..0x826C3074)
	// 826C3008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C300C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3014: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C301C: 38EB4308  addi r7, r11, 0x4308
	ctx.r[7].s64 = ctx.r[11].s64 + 17160;
	// 826C3020: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C3024: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826C3028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C302C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3030: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C3034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3038: 386A4784  addi r3, r10, 0x4784
	ctx.r[3].s64 = ctx.r[10].s64 + 18308;
	// 826C303C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C304C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C305C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3060: 4BDA3DC1  bl 0x82466e20
	ctx.lr = 0x826C3064;
	sub_82466E20(ctx, base);
	// 826C3064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C306C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C3078 size=24
    let mut pc: u32 = 0x826C3078;
    'dispatch: loop {
        match pc {
            0x826C3078 => {
    //   block [0x826C3078..0x826C3090)
	// 826C3078: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C307C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C3080: 394A9B18  addi r10, r10, -0x64e8
	ctx.r[10].s64 = ctx.r[10].s64 + -25832;
	// 826C3084: 816B4350  lwz r11, 0x4350(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17232 as u32) ) } as u64;
	// 826C3088: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826C308C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3090 size=116
    let mut pc: u32 = 0x826C3090;
    'dispatch: loop {
        match pc {
            0x826C3090 => {
    //   block [0x826C3090..0x826C3104)
	// 826C3090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C309C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C30A0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C30A4: 392B1C14  addi r9, r11, 0x1c14
	ctx.r[9].s64 = ctx.r[11].s64 + 7188;
	// 826C30A8: 38AA4694  addi r5, r10, 0x4694
	ctx.r[5].s64 = ctx.r[10].s64 + 18068;
	// 826C30AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C30B0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826C30B4: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826C30B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C30BC: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826C30C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C30C4: 396B9B18  addi r11, r11, -0x64e8
	ctx.r[11].s64 = ctx.r[11].s64 + -25832;
	// 826C30C8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C30CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C30D0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C30D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C30D8: 386A47B4  addi r3, r10, 0x47b4
	ctx.r[3].s64 = ctx.r[10].s64 + 18356;
	// 826C30DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C30E0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C30E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C30E8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C30EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C30F0: 4BDA3D31  bl 0x82466e20
	ctx.lr = 0x826C30F4;
	sub_82466E20(ctx, base);
	// 826C30F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C30F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C30FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3108 size=112
    let mut pc: u32 = 0x826C3108;
    'dispatch: loop {
        match pc {
            0x826C3108 => {
    //   block [0x826C3108..0x826C3178)
	// 826C3108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C310C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3114: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3118: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C311C: 38AA4694  addi r5, r10, 0x4694
	ctx.r[5].s64 = ctx.r[10].s64 + 18068;
	// 826C3120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3124: 390B4354  addi r8, r11, 0x4354
	ctx.r[8].s64 = ctx.r[11].s64 + 17236;
	// 826C3128: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C312C: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 826C3130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3134: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C313C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3140: 386A47E4  addi r3, r10, 0x47e4
	ctx.r[3].s64 = ctx.r[10].s64 + 18404;
	// 826C3144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C314C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C315C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3164: 4BDA3CBD  bl 0x82466e20
	ctx.lr = 0x826C3168;
	sub_82466E20(ctx, base);
	// 826C3168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C316C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3178 size=112
    let mut pc: u32 = 0x826C3178;
    'dispatch: loop {
        match pc {
            0x826C3178 => {
    //   block [0x826C3178..0x826C31E8)
	// 826C3178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C317C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3188: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C318C: 38AA4694  addi r5, r10, 0x4694
	ctx.r[5].s64 = ctx.r[10].s64 + 18068;
	// 826C3190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3194: 390B4384  addi r8, r11, 0x4384
	ctx.r[8].s64 = ctx.r[11].s64 + 17284;
	// 826C3198: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C319C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826C31A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C31A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C31A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C31AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C31B0: 386A4814  addi r3, r10, 0x4814
	ctx.r[3].s64 = ctx.r[10].s64 + 18452;
	// 826C31B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C31B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C31BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C31C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C31C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C31C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C31CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C31D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C31D4: 4BDA3C4D  bl 0x82466e20
	ctx.lr = 0x826C31D8;
	sub_82466E20(ctx, base);
	// 826C31D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C31DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C31E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C31E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C31E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C31E8 size=100
    let mut pc: u32 = 0x826C31E8;
    'dispatch: loop {
        match pc {
            0x826C31E8 => {
    //   block [0x826C31E8..0x826C324C)
	// 826C31E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C31EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C31F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C31F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C31F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C31FC: 38AA4C34  addi r5, r10, 0x4c34
	ctx.r[5].s64 = ctx.r[10].s64 + 19508;
	// 826C3200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3208: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826C320C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C321C: 386A4844  addi r3, r10, 0x4844
	ctx.r[3].s64 = ctx.r[10].s64 + 18500;
	// 826C3220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3224: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3228: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C322C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3230: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C3234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3238: 4BDA3BE9  bl 0x82466e20
	ctx.lr = 0x826C323C;
	sub_82466E20(ctx, base);
	// 826C323C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3250 size=112
    let mut pc: u32 = 0x826C3250;
    'dispatch: loop {
        match pc {
            0x826C3250 => {
    //   block [0x826C3250..0x826C32C0)
	// 826C3250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C325C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3260: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3264: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C326C: 390B439C  addi r8, r11, 0x439c
	ctx.r[8].s64 = ctx.r[11].s64 + 17308;
	// 826C3270: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C3274: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826C3278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C327C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3280: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3288: 386A4874  addi r3, r10, 0x4874
	ctx.r[3].s64 = ctx.r[10].s64 + 18548;
	// 826C328C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C329C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C32A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C32A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C32A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C32AC: 4BDA3B75  bl 0x82466e20
	ctx.lr = 0x826C32B0;
	sub_82466E20(ctx, base);
	// 826C32B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C32B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C32B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C32BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C32C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C32C0 size=108
    let mut pc: u32 = 0x826C32C0;
    'dispatch: loop {
        match pc {
            0x826C32C0 => {
    //   block [0x826C32C0..0x826C332C)
	// 826C32C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C32C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C32C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C32CC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C32D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C32D4: 38EB43D0  addi r7, r11, 0x43d0
	ctx.r[7].s64 = ctx.r[11].s64 + 17360;
	// 826C32D8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C32DC: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 826C32E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C32E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C32E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C32EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C32F0: 386A48A4  addi r3, r10, 0x48a4
	ctx.r[3].s64 = ctx.r[10].s64 + 18596;
	// 826C32F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C32F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C32FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C330C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3318: 4BDA3B09  bl 0x82466e20
	ctx.lr = 0x826C331C;
	sub_82466E20(ctx, base);
	// 826C331C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3330 size=108
    let mut pc: u32 = 0x826C3330;
    'dispatch: loop {
        match pc {
            0x826C3330 => {
    //   block [0x826C3330..0x826C339C)
	// 826C3330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C333C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3344: 38EB4430  addi r7, r11, 0x4430
	ctx.r[7].s64 = ctx.r[11].s64 + 17456;
	// 826C3348: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C334C: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826C3350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3354: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3358: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C335C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3360: 386A48D4  addi r3, r10, 0x48d4
	ctx.r[3].s64 = ctx.r[10].s64 + 18644;
	// 826C3364: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C336C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C337C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3384: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3388: 4BDA3A99  bl 0x82466e20
	ctx.lr = 0x826C338C;
	sub_82466E20(ctx, base);
	// 826C338C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C33A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C33A0 size=116
    let mut pc: u32 = 0x826C33A0;
    'dispatch: loop {
        match pc {
            0x826C33A0 => {
    //   block [0x826C33A0..0x826C3414)
	// 826C33A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C33A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C33A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C33AC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C33B0: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 826C33B4: 390A4460  addi r8, r10, 0x4460
	ctx.r[8].s64 = ctx.r[10].s64 + 17504;
	// 826C33B8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C33BC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C33C0: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C33C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C33C8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C33CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C33D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C33D4: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826C33D8: 396B1C70  addi r11, r11, 0x1c70
	ctx.r[11].s64 = ctx.r[11].s64 + 7280;
	// 826C33DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C33E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C33E4: 386A4904  addi r3, r10, 0x4904
	ctx.r[3].s64 = ctx.r[10].s64 + 18692;
	// 826C33E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C33EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C33F0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C33F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C33F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C33FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3400: 4BDA3A21  bl 0x82466e20
	ctx.lr = 0x826C3404;
	sub_82466E20(ctx, base);
	// 826C3404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C340C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3418 size=112
    let mut pc: u32 = 0x826C3418;
    'dispatch: loop {
        match pc {
            0x826C3418 => {
    //   block [0x826C3418..0x826C3488)
	// 826C3418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C341C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3424: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3428: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C342C: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3434: 390B45E0  addi r8, r11, 0x45e0
	ctx.r[8].s64 = ctx.r[11].s64 + 17888;
	// 826C3438: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C343C: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826C3440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C344C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3450: 386A4934  addi r3, r10, 0x4934
	ctx.r[3].s64 = ctx.r[10].s64 + 18740;
	// 826C3454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C345C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C346C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3474: 4BDA39AD  bl 0x82466e20
	ctx.lr = 0x826C3478;
	sub_82466E20(ctx, base);
	// 826C3478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C347C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3488 size=116
    let mut pc: u32 = 0x826C3488;
    'dispatch: loop {
        match pc {
            0x826C3488 => {
    //   block [0x826C3488..0x826C34FC)
	// 826C3488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C348C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3494: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C3498: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826C349C: 390A45F8  addi r8, r10, 0x45f8
	ctx.r[8].s64 = ctx.r[10].s64 + 17912;
	// 826C34A0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C34A4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C34A8: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C34AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C34B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C34B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C34B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C34BC: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826C34C0: 396B1CBC  addi r11, r11, 0x1cbc
	ctx.r[11].s64 = ctx.r[11].s64 + 7356;
	// 826C34C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C34C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C34CC: 386A4964  addi r3, r10, 0x4964
	ctx.r[3].s64 = ctx.r[10].s64 + 18788;
	// 826C34D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C34D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C34D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C34DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C34E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C34E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C34E8: 4BDA3939  bl 0x82466e20
	ctx.lr = 0x826C34EC;
	sub_82466E20(ctx, base);
	// 826C34EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C34F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C34F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C34F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3500 size=112
    let mut pc: u32 = 0x826C3500;
    'dispatch: loop {
        match pc {
            0x826C3500 => {
    //   block [0x826C3500..0x826C3570)
	// 826C3500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C350C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3510: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3514: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C351C: 390B4658  addi r8, r11, 0x4658
	ctx.r[8].s64 = ctx.r[11].s64 + 18008;
	// 826C3520: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826C3524: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826C3528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C352C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3538: 386A4994  addi r3, r10, 0x4994
	ctx.r[3].s64 = ctx.r[10].s64 + 18836;
	// 826C353C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C354C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C355C: 4BDA38C5  bl 0x82466e20
	ctx.lr = 0x826C3560;
	sub_82466E20(ctx, base);
	// 826C3560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C356C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3570 size=112
    let mut pc: u32 = 0x826C3570;
    'dispatch: loop {
        match pc {
            0x826C3570 => {
    //   block [0x826C3570..0x826C35E0)
	// 826C3570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C357C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3580: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3584: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C358C: 390B4730  addi r8, r11, 0x4730
	ctx.r[8].s64 = ctx.r[11].s64 + 18224;
	// 826C3590: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826C3594: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826C3598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C359C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C35A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C35A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C35A8: 386A49C4  addi r3, r10, 0x49c4
	ctx.r[3].s64 = ctx.r[10].s64 + 18884;
	// 826C35AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C35B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C35B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C35B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C35BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C35C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C35C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C35C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C35CC: 4BDA3855  bl 0x82466e20
	ctx.lr = 0x826C35D0;
	sub_82466E20(ctx, base);
	// 826C35D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C35D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C35D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C35DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C35E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C35E0 size=112
    let mut pc: u32 = 0x826C35E0;
    'dispatch: loop {
        match pc {
            0x826C35E0 => {
    //   block [0x826C35E0..0x826C3650)
	// 826C35E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C35E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C35E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C35EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C35F0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C35F4: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C35F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826C35FC: 390B4838  addi r8, r11, 0x4838
	ctx.r[8].s64 = ctx.r[11].s64 + 18488;
	// 826C3600: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C3604: 388A43BC  addi r4, r10, 0x43bc
	ctx.r[4].s64 = ctx.r[10].s64 + 17340;
	// 826C3608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C360C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3618: 386A49F4  addi r3, r10, 0x49f4
	ctx.r[3].s64 = ctx.r[10].s64 + 18932;
	// 826C361C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C362C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C363C: 4BDA37E5  bl 0x82466e20
	ctx.lr = 0x826C3640;
	sub_82466E20(ctx, base);
	// 826C3640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C364C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C3650 size=24
    let mut pc: u32 = 0x826C3650;
    'dispatch: loop {
        match pc {
            0x826C3650 => {
    //   block [0x826C3650..0x826C3668)
	// 826C3650: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3654: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C3658: 394A9C68  addi r10, r10, -0x6398
	ctx.r[10].s64 = ctx.r[10].s64 + -25496;
	// 826C365C: 816B43CC  lwz r11, 0x43cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17356 as u32) ) } as u64;
	// 826C3660: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826C3664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3668 size=116
    let mut pc: u32 = 0x826C3668;
    'dispatch: loop {
        match pc {
            0x826C3668 => {
    //   block [0x826C3668..0x826C36DC)
	// 826C3668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C366C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3674: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C3678: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C367C: 392B1CEC  addi r9, r11, 0x1cec
	ctx.r[9].s64 = ctx.r[11].s64 + 7404;
	// 826C3680: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3684: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826C3688: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826C368C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 826C3690: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C3694: 388A4454  addi r4, r10, 0x4454
	ctx.r[4].s64 = ctx.r[10].s64 + 17492;
	// 826C3698: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C369C: 396B9C68  addi r11, r11, -0x6398
	ctx.r[11].s64 = ctx.r[11].s64 + -25496;
	// 826C36A0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C36A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C36A8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C36AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C36B0: 386A4A24  addi r3, r10, 0x4a24
	ctx.r[3].s64 = ctx.r[10].s64 + 18980;
	// 826C36B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C36B8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C36BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C36C0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C36C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C36C8: 4BDA3759  bl 0x82466e20
	ctx.lr = 0x826C36CC;
	sub_82466E20(ctx, base);
	// 826C36CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C36D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C36D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C36D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C36E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C36E0 size=116
    let mut pc: u32 = 0x826C36E0;
    'dispatch: loop {
        match pc {
            0x826C36E0 => {
    //   block [0x826C36E0..0x826C3754)
	// 826C36E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C36E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C36E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C36EC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C36F0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826C36F4: 390A4868  addi r8, r10, 0x4868
	ctx.r[8].s64 = ctx.r[10].s64 + 18536;
	// 826C36F8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C36FC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C3700: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C3704: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3708: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C370C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3714: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826C3718: 396B1D5C  addi r11, r11, 0x1d5c
	ctx.r[11].s64 = ctx.r[11].s64 + 7516;
	// 826C371C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3720: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3724: 386A4A54  addi r3, r10, 0x4a54
	ctx.r[3].s64 = ctx.r[10].s64 + 19028;
	// 826C3728: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C372C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3730: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C3734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C373C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3740: 4BDA36E1  bl 0x82466e20
	ctx.lr = 0x826C3744;
	sub_82466E20(ctx, base);
	// 826C3744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C374C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3758 size=108
    let mut pc: u32 = 0x826C3758;
    'dispatch: loop {
        match pc {
            0x826C3758 => {
    //   block [0x826C3758..0x826C37C4)
	// 826C3758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C375C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3764: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C376C: 38EB4898  addi r7, r11, 0x4898
	ctx.r[7].s64 = ctx.r[11].s64 + 18584;
	// 826C3770: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826C3774: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826C3778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C377C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3780: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C3784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3788: 386A4A84  addi r3, r10, 0x4a84
	ctx.r[3].s64 = ctx.r[10].s64 + 19076;
	// 826C378C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C379C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C37A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C37A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C37A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C37AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C37B0: 4BDA3671  bl 0x82466e20
	ctx.lr = 0x826C37B4;
	sub_82466E20(ctx, base);
	// 826C37B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C37B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C37BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C37C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C37C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C37C8 size=24
    let mut pc: u32 = 0x826C37C8;
    'dispatch: loop {
        match pc {
            0x826C37C8 => {
    //   block [0x826C37C8..0x826C37E0)
	// 826C37C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C37CC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C37D0: 394A9E00  addi r10, r10, -0x6200
	ctx.r[10].s64 = ctx.r[10].s64 + -25088;
	// 826C37D4: 816B4928  lwz r11, 0x4928(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18728 as u32) ) } as u64;
	// 826C37D8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826C37DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C37E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C37E0 size=116
    let mut pc: u32 = 0x826C37E0;
    'dispatch: loop {
        match pc {
            0x826C37E0 => {
    //   block [0x826C37E0..0x826C3854)
	// 826C37E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C37E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C37E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C37EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C37F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C37F4: 392B1D80  addi r9, r11, 0x1d80
	ctx.r[9].s64 = ctx.r[11].s64 + 7552;
	// 826C37F8: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C37FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3800: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826C3804: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 826C3808: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C380C: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826C3810: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3814: 396B9E00  addi r11, r11, -0x6200
	ctx.r[11].s64 = ctx.r[11].s64 + -25088;
	// 826C3818: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C381C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3820: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C3824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3828: 386A4AB4  addi r3, r10, 0x4ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 19124;
	// 826C382C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C3830: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C3834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3838: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C383C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C3840: 4BDA35E1  bl 0x82466e20
	ctx.lr = 0x826C3844;
	sub_82466E20(ctx, base);
	// 826C3844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C384C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3858 size=112
    let mut pc: u32 = 0x826C3858;
    'dispatch: loop {
        match pc {
            0x826C3858 => {
    //   block [0x826C3858..0x826C38C8)
	// 826C3858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C385C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3864: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3868: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C386C: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3874: 390B492C  addi r8, r11, 0x492c
	ctx.r[8].s64 = ctx.r[11].s64 + 18732;
	// 826C3878: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C387C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826C3880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3884: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C388C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3890: 386A4AE4  addi r3, r10, 0x4ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 19172;
	// 826C3894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C389C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C38A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C38A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C38A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C38AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C38B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C38B4: 4BDA356D  bl 0x82466e20
	ctx.lr = 0x826C38B8;
	sub_82466E20(ctx, base);
	// 826C38B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C38BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C38C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C38C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C38C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C38C8 size=116
    let mut pc: u32 = 0x826C38C8;
    'dispatch: loop {
        match pc {
            0x826C38C8 => {
    //   block [0x826C38C8..0x826C393C)
	// 826C38C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C38CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C38D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C38D4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C38D8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826C38DC: 390A4960  addi r8, r10, 0x4960
	ctx.r[8].s64 = ctx.r[10].s64 + 18784;
	// 826C38E0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C38E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C38E8: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C38EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C38F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C38F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C38F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C38FC: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826C3900: 396B1DC8  addi r11, r11, 0x1dc8
	ctx.r[11].s64 = ctx.r[11].s64 + 7624;
	// 826C3904: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3908: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C390C: 386A4B14  addi r3, r10, 0x4b14
	ctx.r[3].s64 = ctx.r[10].s64 + 19220;
	// 826C3910: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C3914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3918: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C391C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3928: 4BDA34F9  bl 0x82466e20
	ctx.lr = 0x826C392C;
	sub_82466E20(ctx, base);
	// 826C392C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3940 size=108
    let mut pc: u32 = 0x826C3940;
    'dispatch: loop {
        match pc {
            0x826C3940 => {
    //   block [0x826C3940..0x826C39AC)
	// 826C3940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C394C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3954: 38EB49F0  addi r7, r11, 0x49f0
	ctx.r[7].s64 = ctx.r[11].s64 + 18928;
	// 826C3958: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 826C395C: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826C3960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3964: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3968: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C396C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3970: 386A4B44  addi r3, r10, 0x4b44
	ctx.r[3].s64 = ctx.r[10].s64 + 19268;
	// 826C3974: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C397C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C398C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3994: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3998: 4BDA3489  bl 0x82466e20
	ctx.lr = 0x826C399C;
	sub_82466E20(ctx, base);
	// 826C399C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C39A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C39A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C39A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C39B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C39B0 size=116
    let mut pc: u32 = 0x826C39B0;
    'dispatch: loop {
        match pc {
            0x826C39B0 => {
    //   block [0x826C39B0..0x826C3A24)
	// 826C39B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C39B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C39B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C39BC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C39C0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826C39C4: 390A4B40  addi r8, r10, 0x4b40
	ctx.r[8].s64 = ctx.r[10].s64 + 19264;
	// 826C39C8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C39CC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C39D0: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C39D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C39D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C39DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C39E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C39E4: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826C39E8: 396B1DEC  addi r11, r11, 0x1dec
	ctx.r[11].s64 = ctx.r[11].s64 + 7660;
	// 826C39EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C39F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C39F4: 386A4B74  addi r3, r10, 0x4b74
	ctx.r[3].s64 = ctx.r[10].s64 + 19316;
	// 826C39F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C39FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3A00: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C3A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3A10: 4BDA3411  bl 0x82466e20
	ctx.lr = 0x826C3A14;
	sub_82466E20(ctx, base);
	// 826C3A14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3A28 size=112
    let mut pc: u32 = 0x826C3A28;
    'dispatch: loop {
        match pc {
            0x826C3A28 => {
    //   block [0x826C3A28..0x826C3A98)
	// 826C3A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3A34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3A38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3A3C: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3A40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3A44: 390B4C00  addi r8, r11, 0x4c00
	ctx.r[8].s64 = ctx.r[11].s64 + 19456;
	// 826C3A48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C3A4C: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826C3A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3A54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3A58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3A60: 386A4BA4  addi r3, r10, 0x4ba4
	ctx.r[3].s64 = ctx.r[10].s64 + 19364;
	// 826C3A64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3A68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3A84: 4BDA339D  bl 0x82466e20
	ctx.lr = 0x826C3A88;
	sub_82466E20(ctx, base);
	// 826C3A88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3A98 size=112
    let mut pc: u32 = 0x826C3A98;
    'dispatch: loop {
        match pc {
            0x826C3A98 => {
    //   block [0x826C3A98..0x826C3B08)
	// 826C3A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3AA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3AA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3AAC: 38AA4994  addi r5, r10, 0x4994
	ctx.r[5].s64 = ctx.r[10].s64 + 18836;
	// 826C3AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3AB4: 390B4C18  addi r8, r11, 0x4c18
	ctx.r[8].s64 = ctx.r[11].s64 + 19480;
	// 826C3AB8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826C3ABC: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826C3AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3AC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3AD0: 386A4BD4  addi r3, r10, 0x4bd4
	ctx.r[3].s64 = ctx.r[10].s64 + 19412;
	// 826C3AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3AF4: 4BDA332D  bl 0x82466e20
	ctx.lr = 0x826C3AF8;
	sub_82466E20(ctx, base);
	// 826C3AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3B08 size=112
    let mut pc: u32 = 0x826C3B08;
    'dispatch: loop {
        match pc {
            0x826C3B08 => {
    //   block [0x826C3B08..0x826C3B78)
	// 826C3B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3B14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3B18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3B1C: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3B24: 390B4CA8  addi r8, r11, 0x4ca8
	ctx.r[8].s64 = ctx.r[11].s64 + 19624;
	// 826C3B28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C3B2C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826C3B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3B34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3B38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3B40: 386A4C04  addi r3, r10, 0x4c04
	ctx.r[3].s64 = ctx.r[10].s64 + 19460;
	// 826C3B44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3B54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3B64: 4BDA32BD  bl 0x82466e20
	ctx.lr = 0x826C3B68;
	sub_82466E20(ctx, base);
	// 826C3B68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3B78 size=116
    let mut pc: u32 = 0x826C3B78;
    'dispatch: loop {
        match pc {
            0x826C3B78 => {
    //   block [0x826C3B78..0x826C3BEC)
	// 826C3B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3B84: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3B88: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C3B8C: 390B4CC0  addi r8, r11, 0x4cc0
	ctx.r[8].s64 = ctx.r[11].s64 + 19648;
	// 826C3B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3B94: 392A1E34  addi r9, r10, 0x1e34
	ctx.r[9].s64 = ctx.r[10].s64 + 7732;
	// 826C3B98: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3B9C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826C3BA0: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C3BA4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3BAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3BB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3BBC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C3BC0: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826C3BC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C3BC8: 386B4C34  addi r3, r11, 0x4c34
	ctx.r[3].s64 = ctx.r[11].s64 + 19508;
	// 826C3BCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C3BD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3BD8: 4BDA3249  bl 0x82466e20
	ctx.lr = 0x826C3BDC;
	sub_82466E20(ctx, base);
	// 826C3BDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3BF0 size=100
    let mut pc: u32 = 0x826C3BF0;
    'dispatch: loop {
        match pc {
            0x826C3BF0 => {
    //   block [0x826C3BF0..0x826C3C54)
	// 826C3BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3BFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3C04: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C3C08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3C10: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 826C3C14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3C24: 386A4C64  addi r3, r10, 0x4c64
	ctx.r[3].s64 = ctx.r[10].s64 + 19556;
	// 826C3C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3C2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3C30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C3C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3C38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C3C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3C40: 4BDA31E1  bl 0x82466e20
	ctx.lr = 0x826C3C44;
	sub_82466E20(ctx, base);
	// 826C3C44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3C58 size=112
    let mut pc: u32 = 0x826C3C58;
    'dispatch: loop {
        match pc {
            0x826C3C58 => {
    //   block [0x826C3C58..0x826C3CC8)
	// 826C3C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3C64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3C68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3C6C: 38AA4C64  addi r5, r10, 0x4c64
	ctx.r[5].s64 = ctx.r[10].s64 + 19556;
	// 826C3C70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3C74: 390B4CD8  addi r8, r11, 0x4cd8
	ctx.r[8].s64 = ctx.r[11].s64 + 19672;
	// 826C3C78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C3C7C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826C3C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3C84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3C88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3C90: 386A4C94  addi r3, r10, 0x4c94
	ctx.r[3].s64 = ctx.r[10].s64 + 19604;
	// 826C3C94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3C98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3C9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3CA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3CB4: 4BDA316D  bl 0x82466e20
	ctx.lr = 0x826C3CB8;
	sub_82466E20(ctx, base);
	// 826C3CB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3CC8 size=108
    let mut pc: u32 = 0x826C3CC8;
    'dispatch: loop {
        match pc {
            0x826C3CC8 => {
    //   block [0x826C3CC8..0x826C3D34)
	// 826C3CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3CD4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3CDC: 38EB4CF0  addi r7, r11, 0x4cf0
	ctx.r[7].s64 = ctx.r[11].s64 + 19696;
	// 826C3CE0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C3CE4: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 826C3CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3CEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3CF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C3CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3CF8: 386A4CC4  addi r3, r10, 0x4cc4
	ctx.r[3].s64 = ctx.r[10].s64 + 19652;
	// 826C3CFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3D20: 4BDA3101  bl 0x82466e20
	ctx.lr = 0x826C3D24;
	sub_82466E20(ctx, base);
	// 826C3D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3D38 size=108
    let mut pc: u32 = 0x826C3D38;
    'dispatch: loop {
        match pc {
            0x826C3D38 => {
    //   block [0x826C3D38..0x826C3DA4)
	// 826C3D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3D44: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3D48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3D4C: 38EB4D50  addi r7, r11, 0x4d50
	ctx.r[7].s64 = ctx.r[11].s64 + 19792;
	// 826C3D50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C3D54: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826C3D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3D5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3D60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C3D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3D68: 386A4CF4  addi r3, r10, 0x4cf4
	ctx.r[3].s64 = ctx.r[10].s64 + 19700;
	// 826C3D6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3D70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3D90: 4BDA3091  bl 0x82466e20
	ctx.lr = 0x826C3D94;
	sub_82466E20(ctx, base);
	// 826C3D94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3DA8 size=116
    let mut pc: u32 = 0x826C3DA8;
    'dispatch: loop {
        match pc {
            0x826C3DA8 => {
    //   block [0x826C3DA8..0x826C3E1C)
	// 826C3DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3DB4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C3DB8: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826C3DBC: 390A4D98  addi r8, r10, 0x4d98
	ctx.r[8].s64 = ctx.r[10].s64 + 19864;
	// 826C3DC0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3DC4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C3DC8: 38AA4694  addi r5, r10, 0x4694
	ctx.r[5].s64 = ctx.r[10].s64 + 18068;
	// 826C3DCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3DD0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C3DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3DDC: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826C3DE0: 396B1E48  addi r11, r11, 0x1e48
	ctx.r[11].s64 = ctx.r[11].s64 + 7752;
	// 826C3DE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3DE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3DEC: 386A4D24  addi r3, r10, 0x4d24
	ctx.r[3].s64 = ctx.r[10].s64 + 19748;
	// 826C3DF0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C3DF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3DF8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C3DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3E08: 4BDA3019  bl 0x82466e20
	ctx.lr = 0x826C3E0C;
	sub_82466E20(ctx, base);
	// 826C3E0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3E20 size=108
    let mut pc: u32 = 0x826C3E20;
    'dispatch: loop {
        match pc {
            0x826C3E20 => {
    //   block [0x826C3E20..0x826C3E8C)
	// 826C3E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3E2C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3E30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826C3E34: 38EB4EB8  addi r7, r11, 0x4eb8
	ctx.r[7].s64 = ctx.r[11].s64 + 20152;
	// 826C3E38: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C3E3C: 388A251C  addi r4, r10, 0x251c
	ctx.r[4].s64 = ctx.r[10].s64 + 9500;
	// 826C3E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3E44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3E48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C3E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3E50: 386A4D54  addi r3, r10, 0x4d54
	ctx.r[3].s64 = ctx.r[10].s64 + 19796;
	// 826C3E54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3E74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3E78: 4BDA2FA9  bl 0x82466e20
	ctx.lr = 0x826C3E7C;
	sub_82466E20(ctx, base);
	// 826C3E7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3E90 size=104
    let mut pc: u32 = 0x826C3E90;
    'dispatch: loop {
        match pc {
            0x826C3E90 => {
    //   block [0x826C3E90..0x826C3EF8)
	// 826C3E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3E9C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C3EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3EA4: 392A1E9C  addi r9, r10, 0x1e9c
	ctx.r[9].s64 = ctx.r[10].s64 + 7836;
	// 826C3EA8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3EB0: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C3EB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3EB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3EC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3EC4: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826C3EC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3ECC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3ED0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C3ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3ED8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C3EDC: 386A4D84  addi r3, r10, 0x4d84
	ctx.r[3].s64 = ctx.r[10].s64 + 19844;
	// 826C3EE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C3EE4: 4BDA2F3D  bl 0x82466e20
	ctx.lr = 0x826C3EE8;
	sub_82466E20(ctx, base);
	// 826C3EE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C3EF8 size=24
    let mut pc: u32 = 0x826C3EF8;
    'dispatch: loop {
        match pc {
            0x826C3EF8 => {
    //   block [0x826C3EF8..0x826C3F10)
	// 826C3EF8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3EFC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C3F00: 394A9F08  addi r10, r10, -0x60f8
	ctx.r[10].s64 = ctx.r[10].s64 + -24824;
	// 826C3F04: 816B4F08  lwz r11, 0x4f08(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20232 as u32) ) } as u64;
	// 826C3F08: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826C3F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3F10 size=116
    let mut pc: u32 = 0x826C3F10;
    'dispatch: loop {
        match pc {
            0x826C3F10 => {
    //   block [0x826C3F10..0x826C3F84)
	// 826C3F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3F1C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C3F20: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3F24: 392B1EE0  addi r9, r11, 0x1ee0
	ctx.r[9].s64 = ctx.r[11].s64 + 7904;
	// 826C3F28: 38AA4D84  addi r5, r10, 0x4d84
	ctx.r[5].s64 = ctx.r[10].s64 + 19844;
	// 826C3F2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3F30: 38E90028  addi r7, r9, 0x28
	ctx.r[7].s64 = ctx.r[9].s64 + 40;
	// 826C3F34: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826C3F38: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C3F3C: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826C3F40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3F44: 396B9F08  addi r11, r11, -0x60f8
	ctx.r[11].s64 = ctx.r[11].s64 + -24824;
	// 826C3F48: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C3F4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3F50: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C3F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3F58: 386A4DB4  addi r3, r10, 0x4db4
	ctx.r[3].s64 = ctx.r[10].s64 + 19892;
	// 826C3F5C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826C3F60: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C3F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3F68: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C3F6C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C3F70: 4BDA2EB1  bl 0x82466e20
	ctx.lr = 0x826C3F74;
	sub_82466E20(ctx, base);
	// 826C3F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3F88 size=112
    let mut pc: u32 = 0x826C3F88;
    'dispatch: loop {
        match pc {
            0x826C3F88 => {
    //   block [0x826C3F88..0x826C3FF8)
	// 826C3F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3F94: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C3F98: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C3F9C: 38EA4F10  addi r7, r10, 0x4f10
	ctx.r[7].s64 = ctx.r[10].s64 + 20240;
	// 826C3FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3FA4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C3FA8: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826C3FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3FB0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3FB4: 396B1F68  addi r11, r11, 0x1f68
	ctx.r[11].s64 = ctx.r[11].s64 + 8040;
	// 826C3FB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C3FBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3FC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3FC4: 386A4DE4  addi r3, r10, 0x4de4
	ctx.r[3].s64 = ctx.r[10].s64 + 19940;
	// 826C3FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3FCC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C3FD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3FD4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C3FD8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3FDC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3FE0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3FE4: 4BDA2E3D  bl 0x82466e20
	ctx.lr = 0x826C3FE8;
	sub_82466E20(ctx, base);
	// 826C3FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3FF8 size=108
    let mut pc: u32 = 0x826C3FF8;
    'dispatch: loop {
        match pc {
            0x826C3FF8 => {
    //   block [0x826C3FF8..0x826C4064)
	// 826C3FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4004: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C400C: 38EB4F70  addi r7, r11, 0x4f70
	ctx.r[7].s64 = ctx.r[11].s64 + 20336;
	// 826C4010: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4014: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826C4018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C401C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4028: 386A4E14  addi r3, r10, 0x4e14
	ctx.r[3].s64 = ctx.r[10].s64 + 19988;
	// 826C402C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C403C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C404C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4050: 4BDA2DD1  bl 0x82466e20
	ctx.lr = 0x826C4054;
	sub_82466E20(ctx, base);
	// 826C4054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C405C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4068 size=116
    let mut pc: u32 = 0x826C4068;
    'dispatch: loop {
        match pc {
            0x826C4068 => {
    //   block [0x826C4068..0x826C40DC)
	// 826C4068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C406C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4074: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4078: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C407C: 390B4FA0  addi r8, r11, 0x4fa0
	ctx.r[8].s64 = ctx.r[11].s64 + 20384;
	// 826C4080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4084: 392A1F54  addi r9, r10, 0x1f54
	ctx.r[9].s64 = ctx.r[10].s64 + 8020;
	// 826C4088: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C408C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826C4090: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C4094: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C4098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C409C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C40A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C40A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C40A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C40AC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C40B0: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826C40B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C40B8: 386B4E44  addi r3, r11, 0x4e44
	ctx.r[3].s64 = ctx.r[11].s64 + 20036;
	// 826C40BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C40C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C40C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C40C8: 4BDA2D59  bl 0x82466e20
	ctx.lr = 0x826C40CC;
	sub_82466E20(ctx, base);
	// 826C40CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C40D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C40D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C40D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C40E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C40E0 size=96
    let mut pc: u32 = 0x826C40E0;
    'dispatch: loop {
        match pc {
            0x826C40E0 => {
    //   block [0x826C40E0..0x826C4140)
	// 826C40E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C40E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C40E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C40EC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C40F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C40F4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826C40F8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C40FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4100: 386A4E74  addi r3, r10, 0x4e74
	ctx.r[3].s64 = ctx.r[10].s64 + 20084;
	// 826C4104: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C410C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C4110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C411C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4120: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C4124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4128: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C412C: 4BDA2CF5  bl 0x82466e20
	ctx.lr = 0x826C4130;
	sub_82466E20(ctx, base);
	// 826C4130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C413C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4140 size=112
    let mut pc: u32 = 0x826C4140;
    'dispatch: loop {
        match pc {
            0x826C4140 => {
    //   block [0x826C4140..0x826C41B0)
	// 826C4140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C414C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4150: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4154: 38AA4E74  addi r5, r10, 0x4e74
	ctx.r[5].s64 = ctx.r[10].s64 + 20084;
	// 826C4158: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C415C: 390B4FB8  addi r8, r11, 0x4fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 20408;
	// 826C4160: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C4164: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826C4168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C416C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C4174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4178: 386A4EA4  addi r3, r10, 0x4ea4
	ctx.r[3].s64 = ctx.r[10].s64 + 20132;
	// 826C417C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C4180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C418C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C419C: 4BDA2C85  bl 0x82466e20
	ctx.lr = 0x826C41A0;
	sub_82466E20(ctx, base);
	// 826C41A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C41A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C41A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C41AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C41B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C41B0 size=112
    let mut pc: u32 = 0x826C41B0;
    'dispatch: loop {
        match pc {
            0x826C41B0 => {
    //   block [0x826C41B0..0x826C4220)
	// 826C41B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C41B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C41B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C41BC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C41C0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C41C4: 392A1F84  addi r9, r10, 0x1f84
	ctx.r[9].s64 = ctx.r[10].s64 + 8068;
	// 826C41C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C41CC: 390B4FF0  addi r8, r11, 0x4ff0
	ctx.r[8].s64 = ctx.r[11].s64 + 20464;
	// 826C41D0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C41D4: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826C41D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C41DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C41E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C41E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C41E8: 386A4ED4  addi r3, r10, 0x4ed4
	ctx.r[3].s64 = ctx.r[10].s64 + 20180;
	// 826C41EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C41F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C41F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C41F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C41FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C420C: 4BDA2C15  bl 0x82466e20
	ctx.lr = 0x826C4210;
	sub_82466E20(ctx, base);
	// 826C4210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C421C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4220 size=108
    let mut pc: u32 = 0x826C4220;
    'dispatch: loop {
        match pc {
            0x826C4220 => {
    //   block [0x826C4220..0x826C428C)
	// 826C4220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C422C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4230: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C4234: 38EB5098  addi r7, r11, 0x5098
	ctx.r[7].s64 = ctx.r[11].s64 + 20632;
	// 826C4238: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C423C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826C4240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4244: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C424C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4250: 386A4F04  addi r3, r10, 0x4f04
	ctx.r[3].s64 = ctx.r[10].s64 + 20228;
	// 826C4254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C425C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C426C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4278: 4BDA2BA9  bl 0x82466e20
	ctx.lr = 0x826C427C;
	sub_82466E20(ctx, base);
	// 826C427C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4290 size=108
    let mut pc: u32 = 0x826C4290;
    'dispatch: loop {
        match pc {
            0x826C4290 => {
    //   block [0x826C4290..0x826C42FC)
	// 826C4290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C429C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C42A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C42A4: 38EB50C8  addi r7, r11, 0x50c8
	ctx.r[7].s64 = ctx.r[11].s64 + 20680;
	// 826C42A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C42AC: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 826C42B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C42B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C42B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C42BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C42C0: 386A4F34  addi r3, r10, 0x4f34
	ctx.r[3].s64 = ctx.r[10].s64 + 20276;
	// 826C42C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C42C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C42CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C42D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C42D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C42D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C42DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C42E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C42E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C42E8: 4BDA2B39  bl 0x82466e20
	ctx.lr = 0x826C42EC;
	sub_82466E20(ctx, base);
	// 826C42EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C42F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C42F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C42F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C4300 size=28
    let mut pc: u32 = 0x826C4300;
    'dispatch: loop {
        match pc {
            0x826C4300 => {
    //   block [0x826C4300..0x826C431C)
	// 826C4300: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4304: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C4308: 394A9FC8  addi r10, r10, -0x6038
	ctx.r[10].s64 = ctx.r[10].s64 + -24632;
	// 826C430C: 816B4FEC  lwz r11, 0x4fec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20460 as u32) ) } as u64;
	// 826C4310: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826C4314: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826C4318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4320 size=112
    let mut pc: u32 = 0x826C4320;
    'dispatch: loop {
        match pc {
            0x826C4320 => {
    //   block [0x826C4320..0x826C4390)
	// 826C4320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C432C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C4330: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C4334: 392A20F8  addi r9, r10, 0x20f8
	ctx.r[9].s64 = ctx.r[10].s64 + 8440;
	// 826C4338: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C433C: 390B9FC8  addi r8, r11, -0x6038
	ctx.r[8].s64 = ctx.r[11].s64 + -24632;
	// 826C4340: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826C4344: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826C4348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C434C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C4354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4358: 386A4F64  addi r3, r10, 0x4f64
	ctx.r[3].s64 = ctx.r[10].s64 + 20324;
	// 826C435C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C4360: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826C4364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C436C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C437C: 4BDA2AA5  bl 0x82466e20
	ctx.lr = 0x826C4380;
	sub_82466E20(ctx, base);
	// 826C4380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C438C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4390 size=108
    let mut pc: u32 = 0x826C4390;
    'dispatch: loop {
        match pc {
            0x826C4390 => {
    //   block [0x826C4390..0x826C43FC)
	// 826C4390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C439C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C43A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C43A4: 38EB5100  addi r7, r11, 0x5100
	ctx.r[7].s64 = ctx.r[11].s64 + 20736;
	// 826C43A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C43AC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826C43B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C43B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C43B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C43BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C43C0: 386A4F94  addi r3, r10, 0x4f94
	ctx.r[3].s64 = ctx.r[10].s64 + 20372;
	// 826C43C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C43C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C43CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C43D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C43D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C43D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C43DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C43E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C43E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C43E8: 4BDA2A39  bl 0x82466e20
	ctx.lr = 0x826C43EC;
	sub_82466E20(ctx, base);
	// 826C43EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C43F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C43F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C43F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4400 size=108
    let mut pc: u32 = 0x826C4400;
    'dispatch: loop {
        match pc {
            0x826C4400 => {
    //   block [0x826C4400..0x826C446C)
	// 826C4400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C440C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4410: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C4414: 38EB5130  addi r7, r11, 0x5130
	ctx.r[7].s64 = ctx.r[11].s64 + 20784;
	// 826C4418: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C441C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826C4420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4424: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4428: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C442C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4430: 386A4FC4  addi r3, r10, 0x4fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 20420;
	// 826C4434: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C443C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C444C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4458: 4BDA29C9  bl 0x82466e20
	ctx.lr = 0x826C445C;
	sub_82466E20(ctx, base);
	// 826C445C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C4470 size=24
    let mut pc: u32 = 0x826C4470;
    'dispatch: loop {
        match pc {
            0x826C4470 => {
    //   block [0x826C4470..0x826C4488)
	// 826C4470: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4474: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C4478: 394AA088  addi r10, r10, -0x5f78
	ctx.r[10].s64 = ctx.r[10].s64 + -24440;
	// 826C447C: 816B5148  lwz r11, 0x5148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20808 as u32) ) } as u64;
	// 826C4480: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826C4484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4488 size=112
    let mut pc: u32 = 0x826C4488;
    'dispatch: loop {
        match pc {
            0x826C4488 => {
    //   block [0x826C4488..0x826C44F8)
	// 826C4488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C448C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4494: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C4498: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C449C: 392A214C  addi r9, r10, 0x214c
	ctx.r[9].s64 = ctx.r[10].s64 + 8524;
	// 826C44A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C44A4: 390BA088  addi r8, r11, -0x5f78
	ctx.r[8].s64 = ctx.r[11].s64 + -24440;
	// 826C44A8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826C44AC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826C44B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C44B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C44B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C44BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C44C0: 386A4FF4  addi r3, r10, 0x4ff4
	ctx.r[3].s64 = ctx.r[10].s64 + 20468;
	// 826C44C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C44C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C44CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C44D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C44D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C44D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C44DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C44E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C44E4: 4BDA293D  bl 0x82466e20
	ctx.lr = 0x826C44E8;
	sub_82466E20(ctx, base);
	// 826C44E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C44EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C44F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C44F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C44F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C44F8 size=112
    let mut pc: u32 = 0x826C44F8;
    'dispatch: loop {
        match pc {
            0x826C44F8 => {
    //   block [0x826C44F8..0x826C4568)
	// 826C44F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C44FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4504: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C4508: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C450C: 392A2188  addi r9, r10, 0x2188
	ctx.r[9].s64 = ctx.r[10].s64 + 8584;
	// 826C4510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4514: 390B5158  addi r8, r11, 0x5158
	ctx.r[8].s64 = ctx.r[11].s64 + 20824;
	// 826C4518: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826C451C: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 826C4520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C452C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4530: 386A5024  addi r3, r10, 0x5024
	ctx.r[3].s64 = ctx.r[10].s64 + 20516;
	// 826C4534: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C4538: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826C453C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C454C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4554: 4BDA28CD  bl 0x82466e20
	ctx.lr = 0x826C4558;
	sub_82466E20(ctx, base);
	// 826C4558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C455C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4568 size=108
    let mut pc: u32 = 0x826C4568;
    'dispatch: loop {
        match pc {
            0x826C4568 => {
    //   block [0x826C4568..0x826C45D4)
	// 826C4568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C456C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4574: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4578: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C457C: 38EB51A0  addi r7, r11, 0x51a0
	ctx.r[7].s64 = ctx.r[11].s64 + 20896;
	// 826C4580: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4584: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826C4588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C458C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4598: 386A5054  addi r3, r10, 0x5054
	ctx.r[3].s64 = ctx.r[10].s64 + 20564;
	// 826C459C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C45A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C45A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C45A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C45AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C45B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C45B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C45B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C45BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C45C0: 4BDA2861  bl 0x82466e20
	ctx.lr = 0x826C45C4;
	sub_82466E20(ctx, base);
	// 826C45C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C45C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C45CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C45D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C45D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C45D8 size=108
    let mut pc: u32 = 0x826C45D8;
    'dispatch: loop {
        match pc {
            0x826C45D8 => {
    //   block [0x826C45D8..0x826C4644)
	// 826C45D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C45DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C45E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C45E4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C45E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C45EC: 38EB51D0  addi r7, r11, 0x51d0
	ctx.r[7].s64 = ctx.r[11].s64 + 20944;
	// 826C45F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C45F4: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826C45F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C45FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4608: 386A5084  addi r3, r10, 0x5084
	ctx.r[3].s64 = ctx.r[10].s64 + 20612;
	// 826C460C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C461C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C462C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4630: 4BDA27F1  bl 0x82466e20
	ctx.lr = 0x826C4634;
	sub_82466E20(ctx, base);
	// 826C4634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C463C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4648 size=112
    let mut pc: u32 = 0x826C4648;
    'dispatch: loop {
        match pc {
            0x826C4648 => {
    //   block [0x826C4648..0x826C46B8)
	// 826C4648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C464C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4654: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C4658: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C465C: 392A21C0  addi r9, r10, 0x21c0
	ctx.r[9].s64 = ctx.r[10].s64 + 8640;
	// 826C4660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4664: 390B5200  addi r8, r11, 0x5200
	ctx.r[8].s64 = ctx.r[11].s64 + 20992;
	// 826C4668: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826C466C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826C4670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4674: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C467C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4680: 386A50B4  addi r3, r10, 0x50b4
	ctx.r[3].s64 = ctx.r[10].s64 + 20660;
	// 826C4684: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C4688: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C468C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C469C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C46A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C46A4: 4BDA277D  bl 0x82466e20
	ctx.lr = 0x826C46A8;
	sub_82466E20(ctx, base);
	// 826C46A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C46AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C46B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C46B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C46B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C46B8 size=108
    let mut pc: u32 = 0x826C46B8;
    'dispatch: loop {
        match pc {
            0x826C46B8 => {
    //   block [0x826C46B8..0x826C4724)
	// 826C46B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C46BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C46C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C46C4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C46C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C46CC: 38EB5260  addi r7, r11, 0x5260
	ctx.r[7].s64 = ctx.r[11].s64 + 21088;
	// 826C46D0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826C46D4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826C46D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C46DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C46E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C46E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C46E8: 386A50E4  addi r3, r10, 0x50e4
	ctx.r[3].s64 = ctx.r[10].s64 + 20708;
	// 826C46EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C46F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C46F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C46F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C46FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C470C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4710: 4BDA2711  bl 0x82466e20
	ctx.lr = 0x826C4714;
	sub_82466E20(ctx, base);
	// 826C4714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C471C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4728 size=108
    let mut pc: u32 = 0x826C4728;
    'dispatch: loop {
        match pc {
            0x826C4728 => {
    //   block [0x826C4728..0x826C4794)
	// 826C4728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C472C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4734: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4738: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C473C: 38EB5350  addi r7, r11, 0x5350
	ctx.r[7].s64 = ctx.r[11].s64 + 21328;
	// 826C4740: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C4744: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826C4748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C474C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4750: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4758: 386A5114  addi r3, r10, 0x5114
	ctx.r[3].s64 = ctx.r[10].s64 + 20756;
	// 826C475C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C476C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C477C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4780: 4BDA26A1  bl 0x82466e20
	ctx.lr = 0x826C4784;
	sub_82466E20(ctx, base);
	// 826C4784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C478C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4798 size=108
    let mut pc: u32 = 0x826C4798;
    'dispatch: loop {
        match pc {
            0x826C4798 => {
    //   block [0x826C4798..0x826C4804)
	// 826C4798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C479C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C47A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C47A4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C47A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C47AC: 38EB5368  addi r7, r11, 0x5368
	ctx.r[7].s64 = ctx.r[11].s64 + 21352;
	// 826C47B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826C47B4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826C47B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C47BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C47C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C47C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C47C8: 386A5144  addi r3, r10, 0x5144
	ctx.r[3].s64 = ctx.r[10].s64 + 20804;
	// 826C47CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C47D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C47D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C47D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C47DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C47E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C47E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C47E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C47EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C47F0: 4BDA2631  bl 0x82466e20
	ctx.lr = 0x826C47F4;
	sub_82466E20(ctx, base);
	// 826C47F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C47F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C47FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C4808 size=24
    let mut pc: u32 = 0x826C4808;
    'dispatch: loop {
        match pc {
            0x826C4808 => {
    //   block [0x826C4808..0x826C4820)
	// 826C4808: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C480C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C4810: 394AA118  addi r10, r10, -0x5ee8
	ctx.r[10].s64 = ctx.r[10].s64 + -24296;
	// 826C4814: 816B53F8  lwz r11, 0x53f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21496 as u32) ) } as u64;
	// 826C4818: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826C481C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4820 size=108
    let mut pc: u32 = 0x826C4820;
    'dispatch: loop {
        match pc {
            0x826C4820 => {
    //   block [0x826C4820..0x826C488C)
	// 826C4820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C482C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C4830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4834: 38EBA118  addi r7, r11, -0x5ee8
	ctx.r[7].s64 = ctx.r[11].s64 + -24296;
	// 826C4838: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C483C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826C4840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4844: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C484C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4850: 386A5174  addi r3, r10, 0x5174
	ctx.r[3].s64 = ctx.r[10].s64 + 20852;
	// 826C4854: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C485C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C486C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4878: 4BDA25A9  bl 0x82466e20
	ctx.lr = 0x826C487C;
	sub_82466E20(ctx, base);
	// 826C487C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C4890 size=24
    let mut pc: u32 = 0x826C4890;
    'dispatch: loop {
        match pc {
            0x826C4890 => {
    //   block [0x826C4890..0x826C48A8)
	// 826C4890: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4894: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C4898: 394AA148  addi r10, r10, -0x5eb8
	ctx.r[10].s64 = ctx.r[10].s64 + -24248;
	// 826C489C: 816B53F8  lwz r11, 0x53f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21496 as u32) ) } as u64;
	// 826C48A0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826C48A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C48A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C48A8 size=108
    let mut pc: u32 = 0x826C48A8;
    'dispatch: loop {
        match pc {
            0x826C48A8 => {
    //   block [0x826C48A8..0x826C4914)
	// 826C48A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C48AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C48B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C48B4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C48B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C48BC: 38EBA148  addi r7, r11, -0x5eb8
	ctx.r[7].s64 = ctx.r[11].s64 + -24248;
	// 826C48C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C48C4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826C48C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C48CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C48D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C48D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C48D8: 386A51A4  addi r3, r10, 0x51a4
	ctx.r[3].s64 = ctx.r[10].s64 + 20900;
	// 826C48DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C48E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C48E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C48E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C48EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C48F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C48F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C48F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C48FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4900: 4BDA2521  bl 0x82466e20
	ctx.lr = 0x826C4904;
	sub_82466E20(ctx, base);
	// 826C4904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C490C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4918 size=108
    let mut pc: u32 = 0x826C4918;
    'dispatch: loop {
        match pc {
            0x826C4918 => {
    //   block [0x826C4918..0x826C4984)
	// 826C4918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4924: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C492C: 38EB53E0  addi r7, r11, 0x53e0
	ctx.r[7].s64 = ctx.r[11].s64 + 21472;
	// 826C4930: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C4934: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826C4938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C493C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4940: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4948: 386A51D4  addi r3, r10, 0x51d4
	ctx.r[3].s64 = ctx.r[10].s64 + 20948;
	// 826C494C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C495C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C496C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4970: 4BDA24B1  bl 0x82466e20
	ctx.lr = 0x826C4974;
	sub_82466E20(ctx, base);
	// 826C4974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C497C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C4988 size=24
    let mut pc: u32 = 0x826C4988;
    'dispatch: loop {
        match pc {
            0x826C4988 => {
    //   block [0x826C4988..0x826C49A0)
	// 826C4988: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C498C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C4990: 394AA178  addi r10, r10, -0x5e88
	ctx.r[10].s64 = ctx.r[10].s64 + -24200;
	// 826C4994: 816B53F8  lwz r11, 0x53f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21496 as u32) ) } as u64;
	// 826C4998: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826C499C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C49A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C49A0 size=108
    let mut pc: u32 = 0x826C49A0;
    'dispatch: loop {
        match pc {
            0x826C49A0 => {
    //   block [0x826C49A0..0x826C4A0C)
	// 826C49A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C49A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C49A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C49AC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C49B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C49B4: 38EBA178  addi r7, r11, -0x5e88
	ctx.r[7].s64 = ctx.r[11].s64 + -24200;
	// 826C49B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C49BC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826C49C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C49C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C49C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C49CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C49D0: 386A5204  addi r3, r10, 0x5204
	ctx.r[3].s64 = ctx.r[10].s64 + 20996;
	// 826C49D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C49D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C49DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C49E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C49E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C49E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C49EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C49F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C49F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C49F8: 4BDA2429  bl 0x82466e20
	ctx.lr = 0x826C49FC;
	sub_82466E20(ctx, base);
	// 826C49FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4A10 size=112
    let mut pc: u32 = 0x826C4A10;
    'dispatch: loop {
        match pc {
            0x826C4A10 => {
    //   block [0x826C4A10..0x826C4A80)
	// 826C4A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4A1C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C4A20: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4A24: 392A2204  addi r9, r10, 0x2204
	ctx.r[9].s64 = ctx.r[10].s64 + 8708;
	// 826C4A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4A2C: 390B53FC  addi r8, r11, 0x53fc
	ctx.r[8].s64 = ctx.r[11].s64 + 21500;
	// 826C4A30: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826C4A34: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826C4A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4A3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4A40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C4A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4A48: 386A5234  addi r3, r10, 0x5234
	ctx.r[3].s64 = ctx.r[10].s64 + 21044;
	// 826C4A4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C4A50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C4A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4A6C: 4BDA23B5  bl 0x82466e20
	ctx.lr = 0x826C4A70;
	sub_82466E20(ctx, base);
	// 826C4A70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4A80 size=108
    let mut pc: u32 = 0x826C4A80;
    'dispatch: loop {
        match pc {
            0x826C4A80 => {
    //   block [0x826C4A80..0x826C4AEC)
	// 826C4A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4A8C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4A94: 38EB542C  addi r7, r11, 0x542c
	ctx.r[7].s64 = ctx.r[11].s64 + 21548;
	// 826C4A98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4A9C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826C4AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4AA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4AB0: 386A5264  addi r3, r10, 0x5264
	ctx.r[3].s64 = ctx.r[10].s64 + 21092;
	// 826C4AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4AD8: 4BDA2349  bl 0x82466e20
	ctx.lr = 0x826C4ADC;
	sub_82466E20(ctx, base);
	// 826C4ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4AF0 size=108
    let mut pc: u32 = 0x826C4AF0;
    'dispatch: loop {
        match pc {
            0x826C4AF0 => {
    //   block [0x826C4AF0..0x826C4B5C)
	// 826C4AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4AFC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4B04: 38EB545C  addi r7, r11, 0x545c
	ctx.r[7].s64 = ctx.r[11].s64 + 21596;
	// 826C4B08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4B0C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826C4B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4B14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4B18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4B20: 386A5294  addi r3, r10, 0x5294
	ctx.r[3].s64 = ctx.r[10].s64 + 21140;
	// 826C4B24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4B48: 4BDA22D9  bl 0x82466e20
	ctx.lr = 0x826C4B4C;
	sub_82466E20(ctx, base);
	// 826C4B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4B60 size=112
    let mut pc: u32 = 0x826C4B60;
    'dispatch: loop {
        match pc {
            0x826C4B60 => {
    //   block [0x826C4B60..0x826C4BD0)
	// 826C4B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4B6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4B70: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4B74: 38AA52F4  addi r5, r10, 0x52f4
	ctx.r[5].s64 = ctx.r[10].s64 + 21236;
	// 826C4B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4B7C: 390B548C  addi r8, r11, 0x548c
	ctx.r[8].s64 = ctx.r[11].s64 + 21644;
	// 826C4B80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C4B84: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 826C4B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4B8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C4B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4B98: 386A52C4  addi r3, r10, 0x52c4
	ctx.r[3].s64 = ctx.r[10].s64 + 21188;
	// 826C4B9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C4BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4BBC: 4BDA2265  bl 0x82466e20
	ctx.lr = 0x826C4BC0;
	sub_82466E20(ctx, base);
	// 826C4BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4BD0 size=108
    let mut pc: u32 = 0x826C4BD0;
    'dispatch: loop {
        match pc {
            0x826C4BD0 => {
    //   block [0x826C4BD0..0x826C4C3C)
	// 826C4BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4BDC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4BE4: 38EB54A4  addi r7, r11, 0x54a4
	ctx.r[7].s64 = ctx.r[11].s64 + 21668;
	// 826C4BE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4BEC: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826C4BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4BF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4BF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4C00: 386A52F4  addi r3, r10, 0x52f4
	ctx.r[3].s64 = ctx.r[10].s64 + 21236;
	// 826C4C04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4C28: 4BDA21F9  bl 0x82466e20
	ctx.lr = 0x826C4C2C;
	sub_82466E20(ctx, base);
	// 826C4C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4C40 size=108
    let mut pc: u32 = 0x826C4C40;
    'dispatch: loop {
        match pc {
            0x826C4C40 => {
    //   block [0x826C4C40..0x826C4CAC)
	// 826C4C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4C4C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4C54: 38EB54D4  addi r7, r11, 0x54d4
	ctx.r[7].s64 = ctx.r[11].s64 + 21716;
	// 826C4C58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C4C5C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826C4C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4C64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4C70: 386A5324  addi r3, r10, 0x5324
	ctx.r[3].s64 = ctx.r[10].s64 + 21284;
	// 826C4C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4C98: 4BDA2189  bl 0x82466e20
	ctx.lr = 0x826C4C9C;
	sub_82466E20(ctx, base);
	// 826C4C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4CB0 size=108
    let mut pc: u32 = 0x826C4CB0;
    'dispatch: loop {
        match pc {
            0x826C4CB0 => {
    //   block [0x826C4CB0..0x826C4D1C)
	// 826C4CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4CBC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4CC4: 38EB54EC  addi r7, r11, 0x54ec
	ctx.r[7].s64 = ctx.r[11].s64 + 21740;
	// 826C4CC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4CCC: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 826C4CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4CD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4CE0: 386A5354  addi r3, r10, 0x5354
	ctx.r[3].s64 = ctx.r[10].s64 + 21332;
	// 826C4CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4D08: 4BDA2119  bl 0x82466e20
	ctx.lr = 0x826C4D0C;
	sub_82466E20(ctx, base);
	// 826C4D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4D20 size=108
    let mut pc: u32 = 0x826C4D20;
    'dispatch: loop {
        match pc {
            0x826C4D20 => {
    //   block [0x826C4D20..0x826C4D8C)
	// 826C4D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4D2C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4D34: 38EB5520  addi r7, r11, 0x5520
	ctx.r[7].s64 = ctx.r[11].s64 + 21792;
	// 826C4D38: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826C4D3C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826C4D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4D44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4D48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4D50: 386A5384  addi r3, r10, 0x5384
	ctx.r[3].s64 = ctx.r[10].s64 + 21380;
	// 826C4D54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4D78: 4BDA20A9  bl 0x82466e20
	ctx.lr = 0x826C4D7C;
	sub_82466E20(ctx, base);
	// 826C4D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4D90 size=108
    let mut pc: u32 = 0x826C4D90;
    'dispatch: loop {
        match pc {
            0x826C4D90 => {
    //   block [0x826C4D90..0x826C4DFC)
	// 826C4D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4D9C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4DA4: 38EB55C8  addi r7, r11, 0x55c8
	ctx.r[7].s64 = ctx.r[11].s64 + 21960;
	// 826C4DA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4DAC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 826C4DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4DB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4DB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4DC0: 386A53B4  addi r3, r10, 0x53b4
	ctx.r[3].s64 = ctx.r[10].s64 + 21428;
	// 826C4DC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4DE8: 4BDA2039  bl 0x82466e20
	ctx.lr = 0x826C4DEC;
	sub_82466E20(ctx, base);
	// 826C4DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4E00 size=108
    let mut pc: u32 = 0x826C4E00;
    'dispatch: loop {
        match pc {
            0x826C4E00 => {
    //   block [0x826C4E00..0x826C4E6C)
	// 826C4E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4E0C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4E14: 38EB55F8  addi r7, r11, 0x55f8
	ctx.r[7].s64 = ctx.r[11].s64 + 22008;
	// 826C4E18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C4E1C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826C4E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4E24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4E28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4E30: 386A53E4  addi r3, r10, 0x53e4
	ctx.r[3].s64 = ctx.r[10].s64 + 21476;
	// 826C4E34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4E54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4E58: 4BDA1FC9  bl 0x82466e20
	ctx.lr = 0x826C4E5C;
	sub_82466E20(ctx, base);
	// 826C4E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4E70 size=108
    let mut pc: u32 = 0x826C4E70;
    'dispatch: loop {
        match pc {
            0x826C4E70 => {
    //   block [0x826C4E70..0x826C4EDC)
	// 826C4E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4E7C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4E84: 38EB5610  addi r7, r11, 0x5610
	ctx.r[7].s64 = ctx.r[11].s64 + 22032;
	// 826C4E88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4E8C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 826C4E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4E94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4E98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4EA0: 386A5414  addi r3, r10, 0x5414
	ctx.r[3].s64 = ctx.r[10].s64 + 21524;
	// 826C4EA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4EC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4EC8: 4BDA1F59  bl 0x82466e20
	ctx.lr = 0x826C4ECC;
	sub_82466E20(ctx, base);
	// 826C4ECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4EE0 size=108
    let mut pc: u32 = 0x826C4EE0;
    'dispatch: loop {
        match pc {
            0x826C4EE0 => {
    //   block [0x826C4EE0..0x826C4F4C)
	// 826C4EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4EEC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4EF4: 38EB5640  addi r7, r11, 0x5640
	ctx.r[7].s64 = ctx.r[11].s64 + 22080;
	// 826C4EF8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826C4EFC: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826C4F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4F04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4F08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4F10: 386A5444  addi r3, r10, 0x5444
	ctx.r[3].s64 = ctx.r[10].s64 + 21572;
	// 826C4F14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4F34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4F38: 4BDA1EE9  bl 0x82466e20
	ctx.lr = 0x826C4F3C;
	sub_82466E20(ctx, base);
	// 826C4F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C4F50 size=24
    let mut pc: u32 = 0x826C4F50;
    'dispatch: loop {
        match pc {
            0x826C4F50 => {
    //   block [0x826C4F50..0x826C4F68)
	// 826C4F50: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4F54: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C4F58: 394AA1A8  addi r10, r10, -0x5e58
	ctx.r[10].s64 = ctx.r[10].s64 + -24152;
	// 826C4F5C: 816B551C  lwz r11, 0x551c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21788 as u32) ) } as u64;
	// 826C4F60: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826C4F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4F68 size=112
    let mut pc: u32 = 0x826C4F68;
    'dispatch: loop {
        match pc {
            0x826C4F68 => {
    //   block [0x826C4F68..0x826C4FD8)
	// 826C4F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4F74: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C4F78: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C4F7C: 392A2230  addi r9, r10, 0x2230
	ctx.r[9].s64 = ctx.r[10].s64 + 8752;
	// 826C4F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4F84: 390BA1A8  addi r8, r11, -0x5e58
	ctx.r[8].s64 = ctx.r[11].s64 + -24152;
	// 826C4F88: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826C4F8C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826C4F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4F94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C4F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4FA0: 386A5474  addi r3, r10, 0x5474
	ctx.r[3].s64 = ctx.r[10].s64 + 21620;
	// 826C4FA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C4FA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C4FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4FC4: 4BDA1E5D  bl 0x82466e20
	ctx.lr = 0x826C4FC8;
	sub_82466E20(ctx, base);
	// 826C4FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4FD8 size=108
    let mut pc: u32 = 0x826C4FD8;
    'dispatch: loop {
        match pc {
            0x826C4FD8 => {
    //   block [0x826C4FD8..0x826C5044)
	// 826C4FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4FE4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4FE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4FEC: 38EB5704  addi r7, r11, 0x5704
	ctx.r[7].s64 = ctx.r[11].s64 + 22276;
	// 826C4FF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4FF4: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826C4FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4FFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5008: 386A54A4  addi r3, r10, 0x54a4
	ctx.r[3].s64 = ctx.r[10].s64 + 21668;
	// 826C500C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C501C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C502C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5030: 4BDA1DF1  bl 0x82466e20
	ctx.lr = 0x826C5034;
	sub_82466E20(ctx, base);
	// 826C5034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C503C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5048 size=112
    let mut pc: u32 = 0x826C5048;
    'dispatch: loop {
        match pc {
            0x826C5048 => {
    //   block [0x826C5048..0x826C50B8)
	// 826C5048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C504C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5054: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C5058: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C505C: 392A2274  addi r9, r10, 0x2274
	ctx.r[9].s64 = ctx.r[10].s64 + 8820;
	// 826C5060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5064: 390B5738  addi r8, r11, 0x5738
	ctx.r[8].s64 = ctx.r[11].s64 + 22328;
	// 826C5068: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826C506C: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826C5070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5074: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5078: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C507C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5080: 386A54D4  addi r3, r10, 0x54d4
	ctx.r[3].s64 = ctx.r[10].s64 + 21716;
	// 826C5084: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C5088: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C508C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C509C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C50A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C50A4: 4BDA1D7D  bl 0x82466e20
	ctx.lr = 0x826C50A8;
	sub_82466E20(ctx, base);
	// 826C50A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C50AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C50B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C50B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C50B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C50B8 size=24
    let mut pc: u32 = 0x826C50B8;
    'dispatch: loop {
        match pc {
            0x826C50B8 => {
    //   block [0x826C50B8..0x826C50D0)
	// 826C50B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C50BC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C50C0: 394AA220  addi r10, r10, -0x5de0
	ctx.r[10].s64 = ctx.r[10].s64 + -24032;
	// 826C50C4: 816B5734  lwz r11, 0x5734(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22324 as u32) ) } as u64;
	// 826C50C8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826C50CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C50D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C50D0 size=112
    let mut pc: u32 = 0x826C50D0;
    'dispatch: loop {
        match pc {
            0x826C50D0 => {
    //   block [0x826C50D0..0x826C5140)
	// 826C50D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C50D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C50D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C50DC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C50E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C50E4: 392A22B0  addi r9, r10, 0x22b0
	ctx.r[9].s64 = ctx.r[10].s64 + 8880;
	// 826C50E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C50EC: 390BA220  addi r8, r11, -0x5de0
	ctx.r[8].s64 = ctx.r[11].s64 + -24032;
	// 826C50F0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826C50F4: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826C50F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C50FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C5104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5108: 386A5504  addi r3, r10, 0x5504
	ctx.r[3].s64 = ctx.r[10].s64 + 21764;
	// 826C510C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C5110: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C5114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C511C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C512C: 4BDA1CF5  bl 0x82466e20
	ctx.lr = 0x826C5130;
	sub_82466E20(ctx, base);
	// 826C5130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C513C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5140 size=108
    let mut pc: u32 = 0x826C5140;
    'dispatch: loop {
        match pc {
            0x826C5140 => {
    //   block [0x826C5140..0x826C51AC)
	// 826C5140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C514C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5154: 38EB57F8  addi r7, r11, 0x57f8
	ctx.r[7].s64 = ctx.r[11].s64 + 22520;
	// 826C5158: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C515C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826C5160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5164: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5168: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C516C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5170: 386A5534  addi r3, r10, 0x5534
	ctx.r[3].s64 = ctx.r[10].s64 + 21812;
	// 826C5174: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C517C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C518C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5198: 4BDA1C89  bl 0x82466e20
	ctx.lr = 0x826C519C;
	sub_82466E20(ctx, base);
	// 826C519C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C51A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C51A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C51A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C51B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C51B0 size=108
    let mut pc: u32 = 0x826C51B0;
    'dispatch: loop {
        match pc {
            0x826C51B0 => {
    //   block [0x826C51B0..0x826C521C)
	// 826C51B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C51B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C51B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C51BC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C51C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C51C4: 38EB5810  addi r7, r11, 0x5810
	ctx.r[7].s64 = ctx.r[11].s64 + 22544;
	// 826C51C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C51CC: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826C51D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C51D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C51D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C51DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C51E0: 386A5564  addi r3, r10, 0x5564
	ctx.r[3].s64 = ctx.r[10].s64 + 21860;
	// 826C51E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C51E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C51EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C51F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C51F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C51F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C51FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5208: 4BDA1C19  bl 0x82466e20
	ctx.lr = 0x826C520C;
	sub_82466E20(ctx, base);
	// 826C520C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C5220 size=24
    let mut pc: u32 = 0x826C5220;
    'dispatch: loop {
        match pc {
            0x826C5220 => {
    //   block [0x826C5220..0x826C5238)
	// 826C5220: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5224: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C5228: 394AA268  addi r10, r10, -0x5d98
	ctx.r[10].s64 = ctx.r[10].s64 + -23960;
	// 826C522C: 816B5840  lwz r11, 0x5840(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22592 as u32) ) } as u64;
	// 826C5230: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826C5234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5238 size=112
    let mut pc: u32 = 0x826C5238;
    'dispatch: loop {
        match pc {
            0x826C5238 => {
    //   block [0x826C5238..0x826C52A8)
	// 826C5238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C523C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5244: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C5248: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C524C: 392A22EC  addi r9, r10, 0x22ec
	ctx.r[9].s64 = ctx.r[10].s64 + 8940;
	// 826C5250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5254: 390BA268  addi r8, r11, -0x5d98
	ctx.r[8].s64 = ctx.r[11].s64 + -23960;
	// 826C5258: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826C525C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826C5260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5264: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C526C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5270: 386A5594  addi r3, r10, 0x5594
	ctx.r[3].s64 = ctx.r[10].s64 + 21908;
	// 826C5274: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C5278: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C527C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C528C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5294: 4BDA1B8D  bl 0x82466e20
	ctx.lr = 0x826C5298;
	sub_82466E20(ctx, base);
	// 826C5298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C529C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C52A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C52A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C52A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C52A8 size=108
    let mut pc: u32 = 0x826C52A8;
    'dispatch: loop {
        match pc {
            0x826C52A8 => {
    //   block [0x826C52A8..0x826C5314)
	// 826C52A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C52AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C52B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C52B4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C52B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C52BC: 38EB5844  addi r7, r11, 0x5844
	ctx.r[7].s64 = ctx.r[11].s64 + 22596;
	// 826C52C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C52C4: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826C52C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C52CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C52D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C52D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C52D8: 386A55C4  addi r3, r10, 0x55c4
	ctx.r[3].s64 = ctx.r[10].s64 + 21956;
	// 826C52DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C52E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C52E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C52E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C52EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C52F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C52F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C52F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C52FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5300: 4BDA1B21  bl 0x82466e20
	ctx.lr = 0x826C5304;
	sub_82466E20(ctx, base);
	// 826C5304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C530C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5318 size=108
    let mut pc: u32 = 0x826C5318;
    'dispatch: loop {
        match pc {
            0x826C5318 => {
    //   block [0x826C5318..0x826C5384)
	// 826C5318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C531C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5324: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C532C: 38EB5860  addi r7, r11, 0x5860
	ctx.r[7].s64 = ctx.r[11].s64 + 22624;
	// 826C5330: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C5334: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826C5338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C533C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5340: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5348: 386A55F4  addi r3, r10, 0x55f4
	ctx.r[3].s64 = ctx.r[10].s64 + 22004;
	// 826C534C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C535C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C536C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5370: 4BDA1AB1  bl 0x82466e20
	ctx.lr = 0x826C5374;
	sub_82466E20(ctx, base);
	// 826C5374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C537C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5388 size=108
    let mut pc: u32 = 0x826C5388;
    'dispatch: loop {
        match pc {
            0x826C5388 => {
    //   block [0x826C5388..0x826C53F4)
	// 826C5388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C538C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5394: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C539C: 38EB58A8  addi r7, r11, 0x58a8
	ctx.r[7].s64 = ctx.r[11].s64 + 22696;
	// 826C53A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C53A4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826C53A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C53AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C53B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C53B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C53B8: 386A5624  addi r3, r10, 0x5624
	ctx.r[3].s64 = ctx.r[10].s64 + 22052;
	// 826C53BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C53C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C53C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C53C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C53CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C53D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C53D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C53D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C53DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C53E0: 4BDA1A41  bl 0x82466e20
	ctx.lr = 0x826C53E4;
	sub_82466E20(ctx, base);
	// 826C53E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C53E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C53EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C53F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C53F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C53F8 size=108
    let mut pc: u32 = 0x826C53F8;
    'dispatch: loop {
        match pc {
            0x826C53F8 => {
    //   block [0x826C53F8..0x826C5464)
	// 826C53F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C53FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5404: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C540C: 38EB58D8  addi r7, r11, 0x58d8
	ctx.r[7].s64 = ctx.r[11].s64 + 22744;
	// 826C5410: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826C5414: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826C5418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C541C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5428: 386A5654  addi r3, r10, 0x5654
	ctx.r[3].s64 = ctx.r[10].s64 + 22100;
	// 826C542C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C543C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C544C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5450: 4BDA19D1  bl 0x82466e20
	ctx.lr = 0x826C5454;
	sub_82466E20(ctx, base);
	// 826C5454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C545C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5468 size=108
    let mut pc: u32 = 0x826C5468;
    'dispatch: loop {
        match pc {
            0x826C5468 => {
    //   block [0x826C5468..0x826C54D4)
	// 826C5468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C546C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5474: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C547C: 38EB59F8  addi r7, r11, 0x59f8
	ctx.r[7].s64 = ctx.r[11].s64 + 23032;
	// 826C5480: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826C5484: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826C5488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C548C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5490: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5498: 386A5684  addi r3, r10, 0x5684
	ctx.r[3].s64 = ctx.r[10].s64 + 22148;
	// 826C549C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C54A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C54A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C54A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C54AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C54B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C54B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C54B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C54BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C54C0: 4BDA1961  bl 0x82466e20
	ctx.lr = 0x826C54C4;
	sub_82466E20(ctx, base);
	// 826C54C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C54C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C54CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C54D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C54D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C54D8 size=108
    let mut pc: u32 = 0x826C54D8;
    'dispatch: loop {
        match pc {
            0x826C54D8 => {
    //   block [0x826C54D8..0x826C5544)
	// 826C54D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C54DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C54E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C54E4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C54E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C54EC: 38EB5A88  addi r7, r11, 0x5a88
	ctx.r[7].s64 = ctx.r[11].s64 + 23176;
	// 826C54F0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826C54F4: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826C54F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C54FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5508: 386A56B4  addi r3, r10, 0x56b4
	ctx.r[3].s64 = ctx.r[10].s64 + 22196;
	// 826C550C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C551C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C552C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5530: 4BDA18F1  bl 0x82466e20
	ctx.lr = 0x826C5534;
	sub_82466E20(ctx, base);
	// 826C5534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C553C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5548 size=108
    let mut pc: u32 = 0x826C5548;
    'dispatch: loop {
        match pc {
            0x826C5548 => {
    //   block [0x826C5548..0x826C55B4)
	// 826C5548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C554C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5554: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C555C: 38EB5B48  addi r7, r11, 0x5b48
	ctx.r[7].s64 = ctx.r[11].s64 + 23368;
	// 826C5560: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826C5564: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826C5568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C556C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5578: 386A56E4  addi r3, r10, 0x56e4
	ctx.r[3].s64 = ctx.r[10].s64 + 22244;
	// 826C557C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C558C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C559C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C55A0: 4BDA1881  bl 0x82466e20
	ctx.lr = 0x826C55A4;
	sub_82466E20(ctx, base);
	// 826C55A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C55A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C55AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C55B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C55B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C55B8 size=108
    let mut pc: u32 = 0x826C55B8;
    'dispatch: loop {
        match pc {
            0x826C55B8 => {
    //   block [0x826C55B8..0x826C5624)
	// 826C55B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C55BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C55C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C55C4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C55C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C55CC: 38EB5C20  addi r7, r11, 0x5c20
	ctx.r[7].s64 = ctx.r[11].s64 + 23584;
	// 826C55D0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826C55D4: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826C55D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C55DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C55E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C55E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C55E8: 386A5714  addi r3, r10, 0x5714
	ctx.r[3].s64 = ctx.r[10].s64 + 22292;
	// 826C55EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C55F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C55F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C55F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C55FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C560C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5610: 4BDA1811  bl 0x82466e20
	ctx.lr = 0x826C5614;
	sub_82466E20(ctx, base);
	// 826C5614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C561C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5628 size=108
    let mut pc: u32 = 0x826C5628;
    'dispatch: loop {
        match pc {
            0x826C5628 => {
    //   block [0x826C5628..0x826C5694)
	// 826C5628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C562C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5634: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C563C: 38EB5CE0  addi r7, r11, 0x5ce0
	ctx.r[7].s64 = ctx.r[11].s64 + 23776;
	// 826C5640: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826C5644: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826C5648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C564C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5658: 386A5744  addi r3, r10, 0x5744
	ctx.r[3].s64 = ctx.r[10].s64 + 22340;
	// 826C565C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C566C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C567C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5680: 4BDA17A1  bl 0x82466e20
	ctx.lr = 0x826C5684;
	sub_82466E20(ctx, base);
	// 826C5684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C568C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5698 size=112
    let mut pc: u32 = 0x826C5698;
    'dispatch: loop {
        match pc {
            0x826C5698 => {
    //   block [0x826C5698..0x826C5708)
	// 826C5698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C569C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C56A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C56A4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C56A8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826C56AC: 38EA5D88  addi r7, r10, 0x5d88
	ctx.r[7].s64 = ctx.r[10].s64 + 23944;
	// 826C56B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C56B4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C56B8: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826C56BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C56C0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C56C4: 396B2300  addi r11, r11, 0x2300
	ctx.r[11].s64 = ctx.r[11].s64 + 8960;
	// 826C56C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C56CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C56D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C56D4: 386A5774  addi r3, r10, 0x5774
	ctx.r[3].s64 = ctx.r[10].s64 + 22388;
	// 826C56D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C56DC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C56E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C56E4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C56E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C56EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C56F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C56F4: 4BDA172D  bl 0x82466e20
	ctx.lr = 0x826C56F8;
	sub_82466E20(ctx, base);
	// 826C56F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C56FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5708 size=108
    let mut pc: u32 = 0x826C5708;
    'dispatch: loop {
        match pc {
            0x826C5708 => {
    //   block [0x826C5708..0x826C5774)
	// 826C5708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C570C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5714: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C571C: 38EB5EA8  addi r7, r11, 0x5ea8
	ctx.r[7].s64 = ctx.r[11].s64 + 24232;
	// 826C5720: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C5724: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826C5728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C572C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5738: 386A57A4  addi r3, r10, 0x57a4
	ctx.r[3].s64 = ctx.r[10].s64 + 22436;
	// 826C573C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C574C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C575C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5760: 4BDA16C1  bl 0x82466e20
	ctx.lr = 0x826C5764;
	sub_82466E20(ctx, base);
	// 826C5764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C576C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5778 size=108
    let mut pc: u32 = 0x826C5778;
    'dispatch: loop {
        match pc {
            0x826C5778 => {
    //   block [0x826C5778..0x826C57E4)
	// 826C5778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C577C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5784: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C578C: 38EB5F08  addi r7, r11, 0x5f08
	ctx.r[7].s64 = ctx.r[11].s64 + 24328;
	// 826C5790: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826C5794: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826C5798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C579C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C57A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C57A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C57A8: 386A57D4  addi r3, r10, 0x57d4
	ctx.r[3].s64 = ctx.r[10].s64 + 22484;
	// 826C57AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C57B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C57B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C57B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C57BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C57C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C57C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C57C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C57CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C57D0: 4BDA1651  bl 0x82466e20
	ctx.lr = 0x826C57D4;
	sub_82466E20(ctx, base);
	// 826C57D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C57D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C57DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C57E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C57E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C57E8 size=108
    let mut pc: u32 = 0x826C57E8;
    'dispatch: loop {
        match pc {
            0x826C57E8 => {
    //   block [0x826C57E8..0x826C5854)
	// 826C57E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C57EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C57F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C57F4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C57F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C57FC: 38EB6010  addi r7, r11, 0x6010
	ctx.r[7].s64 = ctx.r[11].s64 + 24592;
	// 826C5800: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826C5804: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826C5808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C580C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5818: 386A5804  addi r3, r10, 0x5804
	ctx.r[3].s64 = ctx.r[10].s64 + 22532;
	// 826C581C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C582C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C583C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5840: 4BDA15E1  bl 0x82466e20
	ctx.lr = 0x826C5844;
	sub_82466E20(ctx, base);
	// 826C5844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C584C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5858 size=108
    let mut pc: u32 = 0x826C5858;
    'dispatch: loop {
        match pc {
            0x826C5858 => {
    //   block [0x826C5858..0x826C58C4)
	// 826C5858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C585C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5864: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C586C: 38EB60E8  addi r7, r11, 0x60e8
	ctx.r[7].s64 = ctx.r[11].s64 + 24808;
	// 826C5870: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C5874: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826C5878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C587C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5880: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5888: 386A5834  addi r3, r10, 0x5834
	ctx.r[3].s64 = ctx.r[10].s64 + 22580;
	// 826C588C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C589C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C58A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C58A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C58A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C58AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C58B0: 4BDA1571  bl 0x82466e20
	ctx.lr = 0x826C58B4;
	sub_82466E20(ctx, base);
	// 826C58B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C58B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C58BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C58C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C58C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C58C8 size=108
    let mut pc: u32 = 0x826C58C8;
    'dispatch: loop {
        match pc {
            0x826C58C8 => {
    //   block [0x826C58C8..0x826C5934)
	// 826C58C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C58CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C58D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C58D4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C58D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C58DC: 38EB6130  addi r7, r11, 0x6130
	ctx.r[7].s64 = ctx.r[11].s64 + 24880;
	// 826C58E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C58E4: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826C58E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C58EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C58F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C58F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C58F8: 386A5864  addi r3, r10, 0x5864
	ctx.r[3].s64 = ctx.r[10].s64 + 22628;
	// 826C58FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C590C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C591C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5920: 4BDA1501  bl 0x82466e20
	ctx.lr = 0x826C5924;
	sub_82466E20(ctx, base);
	// 826C5924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C592C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5938 size=108
    let mut pc: u32 = 0x826C5938;
    'dispatch: loop {
        match pc {
            0x826C5938 => {
    //   block [0x826C5938..0x826C59A4)
	// 826C5938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C593C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5944: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C594C: 38EB6148  addi r7, r11, 0x6148
	ctx.r[7].s64 = ctx.r[11].s64 + 24904;
	// 826C5950: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C5954: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 826C5958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C595C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5968: 386A5894  addi r3, r10, 0x5894
	ctx.r[3].s64 = ctx.r[10].s64 + 22676;
	// 826C596C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C597C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C598C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5990: 4BDA1491  bl 0x82466e20
	ctx.lr = 0x826C5994;
	sub_82466E20(ctx, base);
	// 826C5994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C599C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C59A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C59A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C59A8 size=108
    let mut pc: u32 = 0x826C59A8;
    'dispatch: loop {
        match pc {
            0x826C59A8 => {
    //   block [0x826C59A8..0x826C5A14)
	// 826C59A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C59AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C59B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C59B4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C59B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C59BC: 38EB6190  addi r7, r11, 0x6190
	ctx.r[7].s64 = ctx.r[11].s64 + 24976;
	// 826C59C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C59C4: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 826C59C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C59CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C59D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C59D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C59D8: 386A58C4  addi r3, r10, 0x58c4
	ctx.r[3].s64 = ctx.r[10].s64 + 22724;
	// 826C59DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C59E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C59E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C59E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C59EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C59F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C59F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C59F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C59FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5A00: 4BDA1421  bl 0x82466e20
	ctx.lr = 0x826C5A04;
	sub_82466E20(ctx, base);
	// 826C5A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5A18 size=112
    let mut pc: u32 = 0x826C5A18;
    'dispatch: loop {
        match pc {
            0x826C5A18 => {
    //   block [0x826C5A18..0x826C5A88)
	// 826C5A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5A24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5A28: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5A2C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C5A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5A34: 390B61A8  addi r8, r11, 0x61a8
	ctx.r[8].s64 = ctx.r[11].s64 + 25000;
	// 826C5A38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C5A3C: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 826C5A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5A44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5A48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C5A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5A50: 386A58F4  addi r3, r10, 0x58f4
	ctx.r[3].s64 = ctx.r[10].s64 + 22772;
	// 826C5A54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C5A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5A74: 4BDA13AD  bl 0x82466e20
	ctx.lr = 0x826C5A78;
	sub_82466E20(ctx, base);
	// 826C5A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5A88 size=108
    let mut pc: u32 = 0x826C5A88;
    'dispatch: loop {
        match pc {
            0x826C5A88 => {
    //   block [0x826C5A88..0x826C5AF4)
	// 826C5A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5A94: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5A9C: 38EB61F0  addi r7, r11, 0x61f0
	ctx.r[7].s64 = ctx.r[11].s64 + 25072;
	// 826C5AA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C5AA4: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 826C5AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5AAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5AB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5AB8: 386A5924  addi r3, r10, 0x5924
	ctx.r[3].s64 = ctx.r[10].s64 + 22820;
	// 826C5ABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5AE0: 4BDA1341  bl 0x82466e20
	ctx.lr = 0x826C5AE4;
	sub_82466E20(ctx, base);
	// 826C5AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5AF8 size=112
    let mut pc: u32 = 0x826C5AF8;
    'dispatch: loop {
        match pc {
            0x826C5AF8 => {
    //   block [0x826C5AF8..0x826C5B68)
	// 826C5AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5B04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5B08: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5B0C: 38AA5924  addi r5, r10, 0x5924
	ctx.r[5].s64 = ctx.r[10].s64 + 22820;
	// 826C5B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5B14: 390B6250  addi r8, r11, 0x6250
	ctx.r[8].s64 = ctx.r[11].s64 + 25168;
	// 826C5B18: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C5B1C: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 826C5B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5B24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5B28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C5B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5B30: 386A5954  addi r3, r10, 0x5954
	ctx.r[3].s64 = ctx.r[10].s64 + 22868;
	// 826C5B34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C5B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5B54: 4BDA12CD  bl 0x82466e20
	ctx.lr = 0x826C5B58;
	sub_82466E20(ctx, base);
	// 826C5B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5B68 size=96
    let mut pc: u32 = 0x826C5B68;
    'dispatch: loop {
        match pc {
            0x826C5B68 => {
    //   block [0x826C5B68..0x826C5BC8)
	// 826C5B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5B74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5B7C: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 826C5B80: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5B88: 386A5984  addi r3, r10, 0x5984
	ctx.r[3].s64 = ctx.r[10].s64 + 22916;
	// 826C5B8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5B94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C5B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5BA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C5BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5BB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C5BB4: 4BDA126D  bl 0x82466e20
	ctx.lr = 0x826C5BB8;
	sub_82466E20(ctx, base);
	// 826C5BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5BC8 size=112
    let mut pc: u32 = 0x826C5BC8;
    'dispatch: loop {
        match pc {
            0x826C5BC8 => {
    //   block [0x826C5BC8..0x826C5C38)
	// 826C5BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5BD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5BD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5BDC: 38AA7664  addi r5, r10, 0x7664
	ctx.r[5].s64 = ctx.r[10].s64 + 30308;
	// 826C5BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5BE4: 390B6298  addi r8, r11, 0x6298
	ctx.r[8].s64 = ctx.r[11].s64 + 25240;
	// 826C5BE8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C5BEC: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 826C5BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5BF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C5BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5C00: 386A59B4  addi r3, r10, 0x59b4
	ctx.r[3].s64 = ctx.r[10].s64 + 22964;
	// 826C5C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C5C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5C24: 4BDA11FD  bl 0x82466e20
	ctx.lr = 0x826C5C28;
	sub_82466E20(ctx, base);
	// 826C5C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5C38 size=96
    let mut pc: u32 = 0x826C5C38;
    'dispatch: loop {
        match pc {
            0x826C5C38 => {
    //   block [0x826C5C38..0x826C5C98)
	// 826C5C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5C44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5C4C: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 826C5C50: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5C58: 386A59E4  addi r3, r10, 0x59e4
	ctx.r[3].s64 = ctx.r[10].s64 + 23012;
	// 826C5C5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5C64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C5C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5C78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C5C7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5C80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C5C84: 4BDA119D  bl 0x82466e20
	ctx.lr = 0x826C5C88;
	sub_82466E20(ctx, base);
	// 826C5C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5C98 size=100
    let mut pc: u32 = 0x826C5C98;
    'dispatch: loop {
        match pc {
            0x826C5C98 => {
    //   block [0x826C5C98..0x826C5CFC)
	// 826C5C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5CA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5CAC: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C5CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5CB8: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 826C5CBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5CC4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C5CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5CCC: 386A5A14  addi r3, r10, 0x5a14
	ctx.r[3].s64 = ctx.r[10].s64 + 23060;
	// 826C5CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5CD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5CD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C5CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5CE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C5CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5CE8: 4BDA1139  bl 0x82466e20
	ctx.lr = 0x826C5CEC;
	sub_82466E20(ctx, base);
	// 826C5CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5D00 size=96
    let mut pc: u32 = 0x826C5D00;
    'dispatch: loop {
        match pc {
            0x826C5D00 => {
    //   block [0x826C5D00..0x826C5D60)
	// 826C5D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5D0C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5D14: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826C5D18: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5D20: 386A5A44  addi r3, r10, 0x5a44
	ctx.r[3].s64 = ctx.r[10].s64 + 23108;
	// 826C5D24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5D2C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C5D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5D40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C5D44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5D48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C5D4C: 4BDA10D5  bl 0x82466e20
	ctx.lr = 0x826C5D50;
	sub_82466E20(ctx, base);
	// 826C5D50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5D60 size=112
    let mut pc: u32 = 0x826C5D60;
    'dispatch: loop {
        match pc {
            0x826C5D60 => {
    //   block [0x826C5D60..0x826C5DD0)
	// 826C5D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5D6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5D70: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5D74: 38AA5A14  addi r5, r10, 0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + 23060;
	// 826C5D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5D7C: 390B62F8  addi r8, r11, 0x62f8
	ctx.r[8].s64 = ctx.r[11].s64 + 25336;
	// 826C5D80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C5D84: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 826C5D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5D8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5D90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C5D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5D98: 386A5A74  addi r3, r10, 0x5a74
	ctx.r[3].s64 = ctx.r[10].s64 + 23156;
	// 826C5D9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C5DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5DBC: 4BDA1065  bl 0x82466e20
	ctx.lr = 0x826C5DC0;
	sub_82466E20(ctx, base);
	// 826C5DC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5DD0 size=112
    let mut pc: u32 = 0x826C5DD0;
    'dispatch: loop {
        match pc {
            0x826C5DD0 => {
    //   block [0x826C5DD0..0x826C5E40)
	// 826C5DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5DDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5DE0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5DE4: 38AA5A14  addi r5, r10, 0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + 23060;
	// 826C5DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5DEC: 390B6328  addi r8, r11, 0x6328
	ctx.r[8].s64 = ctx.r[11].s64 + 25384;
	// 826C5DF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C5DF4: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 826C5DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5DFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5E00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C5E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5E08: 386A5AA4  addi r3, r10, 0x5aa4
	ctx.r[3].s64 = ctx.r[10].s64 + 23204;
	// 826C5E0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C5E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5E2C: 4BDA0FF5  bl 0x82466e20
	ctx.lr = 0x826C5E30;
	sub_82466E20(ctx, base);
	// 826C5E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5E40 size=100
    let mut pc: u32 = 0x826C5E40;
    'dispatch: loop {
        match pc {
            0x826C5E40 => {
    //   block [0x826C5E40..0x826C5EA4)
	// 826C5E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5E4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5E54: 38AA5A14  addi r5, r10, 0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + 23060;
	// 826C5E58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5E60: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 826C5E64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5E74: 386A5AD4  addi r3, r10, 0x5ad4
	ctx.r[3].s64 = ctx.r[10].s64 + 23252;
	// 826C5E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5E7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5E80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C5E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5E88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C5E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5E90: 4BDA0F91  bl 0x82466e20
	ctx.lr = 0x826C5E94;
	sub_82466E20(ctx, base);
	// 826C5E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5EA8 size=96
    let mut pc: u32 = 0x826C5EA8;
    'dispatch: loop {
        match pc {
            0x826C5EA8 => {
    //   block [0x826C5EA8..0x826C5F08)
	// 826C5EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5EB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5EBC: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 826C5EC0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5EC8: 386A5B04  addi r3, r10, 0x5b04
	ctx.r[3].s64 = ctx.r[10].s64 + 23300;
	// 826C5ECC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5ED4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C5ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5EE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C5EEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5EF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C5EF4: 4BDA0F2D  bl 0x82466e20
	ctx.lr = 0x826C5EF8;
	sub_82466E20(ctx, base);
	// 826C5EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5F08 size=112
    let mut pc: u32 = 0x826C5F08;
    'dispatch: loop {
        match pc {
            0x826C5F08 => {
    //   block [0x826C5F08..0x826C5F78)
	// 826C5F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5F14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5F18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5F1C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C5F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5F24: 390B6340  addi r8, r11, 0x6340
	ctx.r[8].s64 = ctx.r[11].s64 + 25408;
	// 826C5F28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C5F2C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826C5F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5F34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C5F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5F40: 386A5B34  addi r3, r10, 0x5b34
	ctx.r[3].s64 = ctx.r[10].s64 + 23348;
	// 826C5F44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C5F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5F64: 4BDA0EBD  bl 0x82466e20
	ctx.lr = 0x826C5F68;
	sub_82466E20(ctx, base);
	// 826C5F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5F78 size=96
    let mut pc: u32 = 0x826C5F78;
    'dispatch: loop {
        match pc {
            0x826C5F78 => {
    //   block [0x826C5F78..0x826C5FD8)
	// 826C5F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5F84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5F8C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 826C5F90: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5F98: 386A5B64  addi r3, r10, 0x5b64
	ctx.r[3].s64 = ctx.r[10].s64 + 23396;
	// 826C5F9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5FA4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C5FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5FB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C5FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5FC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C5FC4: 4BDA0E5D  bl 0x82466e20
	ctx.lr = 0x826C5FC8;
	sub_82466E20(ctx, base);
	// 826C5FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5FD8 size=112
    let mut pc: u32 = 0x826C5FD8;
    'dispatch: loop {
        match pc {
            0x826C5FD8 => {
    //   block [0x826C5FD8..0x826C6048)
	// 826C5FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5FE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5FE8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5FEC: 38AA5B64  addi r5, r10, 0x5b64
	ctx.r[5].s64 = ctx.r[10].s64 + 23396;
	// 826C5FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5FF4: 390B6358  addi r8, r11, 0x6358
	ctx.r[8].s64 = ctx.r[11].s64 + 25432;
	// 826C5FF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C5FFC: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826C6000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6004: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C600C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6010: 386A5B94  addi r3, r10, 0x5b94
	ctx.r[3].s64 = ctx.r[10].s64 + 23444;
	// 826C6014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C601C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C602C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6034: 4BDA0DED  bl 0x82466e20
	ctx.lr = 0x826C6038;
	sub_82466E20(ctx, base);
	// 826C6038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C603C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6048 size=108
    let mut pc: u32 = 0x826C6048;
    'dispatch: loop {
        match pc {
            0x826C6048 => {
    //   block [0x826C6048..0x826C60B4)
	// 826C6048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C604C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6054: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C605C: 38EB6370  addi r7, r11, 0x6370
	ctx.r[7].s64 = ctx.r[11].s64 + 25456;
	// 826C6060: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C6064: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 826C6068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C606C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C6074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6078: 386A5BC4  addi r3, r10, 0x5bc4
	ctx.r[3].s64 = ctx.r[10].s64 + 23492;
	// 826C607C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C6080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C608C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C609C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C60A0: 4BDA0D81  bl 0x82466e20
	ctx.lr = 0x826C60A4;
	sub_82466E20(ctx, base);
	// 826C60A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C60A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C60AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C60B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C60B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C60B8 size=112
    let mut pc: u32 = 0x826C60B8;
    'dispatch: loop {
        match pc {
            0x826C60B8 => {
    //   block [0x826C60B8..0x826C6128)
	// 826C60B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C60BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C60C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C60C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C60C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C60CC: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C60D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C60D4: 390B63D0  addi r8, r11, 0x63d0
	ctx.r[8].s64 = ctx.r[11].s64 + 25552;
	// 826C60D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C60DC: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 826C60E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C60E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C60E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C60EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C60F0: 386A5BF4  addi r3, r10, 0x5bf4
	ctx.r[3].s64 = ctx.r[10].s64 + 23540;
	// 826C60F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C60F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C60FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C610C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6114: 4BDA0D0D  bl 0x82466e20
	ctx.lr = 0x826C6118;
	sub_82466E20(ctx, base);
	// 826C6118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C611C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6128 size=112
    let mut pc: u32 = 0x826C6128;
    'dispatch: loop {
        match pc {
            0x826C6128 => {
    //   block [0x826C6128..0x826C6198)
	// 826C6128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C612C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6134: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6138: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C613C: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C6140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6144: 390B63E8  addi r8, r11, 0x63e8
	ctx.r[8].s64 = ctx.r[11].s64 + 25576;
	// 826C6148: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C614C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 826C6150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6154: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6158: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C615C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6160: 386A5C24  addi r3, r10, 0x5c24
	ctx.r[3].s64 = ctx.r[10].s64 + 23588;
	// 826C6164: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C616C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C617C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6184: 4BDA0C9D  bl 0x82466e20
	ctx.lr = 0x826C6188;
	sub_82466E20(ctx, base);
	// 826C6188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C618C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6198 size=112
    let mut pc: u32 = 0x826C6198;
    'dispatch: loop {
        match pc {
            0x826C6198 => {
    //   block [0x826C6198..0x826C6208)
	// 826C6198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C619C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C61A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C61A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C61A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C61AC: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C61B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C61B4: 390B6418  addi r8, r11, 0x6418
	ctx.r[8].s64 = ctx.r[11].s64 + 25624;
	// 826C61B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C61BC: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 826C61C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C61C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C61C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C61CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C61D0: 386A5C54  addi r3, r10, 0x5c54
	ctx.r[3].s64 = ctx.r[10].s64 + 23636;
	// 826C61D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C61D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C61DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C61E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C61E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C61E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C61EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C61F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C61F4: 4BDA0C2D  bl 0x82466e20
	ctx.lr = 0x826C61F8;
	sub_82466E20(ctx, base);
	// 826C61F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C61FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6208 size=116
    let mut pc: u32 = 0x826C6208;
    'dispatch: loop {
        match pc {
            0x826C6208 => {
    //   block [0x826C6208..0x826C627C)
	// 826C6208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C620C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6214: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6218: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C621C: 390B6430  addi r8, r11, 0x6430
	ctx.r[8].s64 = ctx.r[11].s64 + 25648;
	// 826C6220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6224: 392A2378  addi r9, r10, 0x2378
	ctx.r[9].s64 = ctx.r[10].s64 + 9080;
	// 826C6228: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C622C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826C6230: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C6234: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C623C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C624C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C6250: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826C6254: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C6258: 386B5C84  addi r3, r11, 0x5c84
	ctx.r[3].s64 = ctx.r[11].s64 + 23684;
	// 826C625C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C6260: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6268: 4BDA0BB9  bl 0x82466e20
	ctx.lr = 0x826C626C;
	sub_82466E20(ctx, base);
	// 826C626C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6280 size=112
    let mut pc: u32 = 0x826C6280;
    'dispatch: loop {
        match pc {
            0x826C6280 => {
    //   block [0x826C6280..0x826C62F0)
	// 826C6280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C628C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6290: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6294: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C6298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C629C: 390B6460  addi r8, r11, 0x6460
	ctx.r[8].s64 = ctx.r[11].s64 + 25696;
	// 826C62A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C62A4: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 826C62A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C62AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C62B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C62B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C62B8: 386A5CB4  addi r3, r10, 0x5cb4
	ctx.r[3].s64 = ctx.r[10].s64 + 23732;
	// 826C62BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C62C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C62C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C62C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C62CC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C62D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C62D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C62D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C62DC: 4BDA0B45  bl 0x82466e20
	ctx.lr = 0x826C62E0;
	sub_82466E20(ctx, base);
	// 826C62E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C62E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C62E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C62EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C62F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C62F0 size=112
    let mut pc: u32 = 0x826C62F0;
    'dispatch: loop {
        match pc {
            0x826C62F0 => {
    //   block [0x826C62F0..0x826C6360)
	// 826C62F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C62F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C62F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C62FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6300: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6304: 38AA6194  addi r5, r10, 0x6194
	ctx.r[5].s64 = ctx.r[10].s64 + 24980;
	// 826C6308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C630C: 390B6478  addi r8, r11, 0x6478
	ctx.r[8].s64 = ctx.r[11].s64 + 25720;
	// 826C6310: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C6314: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826C6318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C631C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6320: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6328: 386A5CE4  addi r3, r10, 0x5ce4
	ctx.r[3].s64 = ctx.r[10].s64 + 23780;
	// 826C632C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C633C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C634C: 4BDA0AD5  bl 0x82466e20
	ctx.lr = 0x826C6350;
	sub_82466E20(ctx, base);
	// 826C6350: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C635C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6360 size=112
    let mut pc: u32 = 0x826C6360;
    'dispatch: loop {
        match pc {
            0x826C6360 => {
    //   block [0x826C6360..0x826C63D0)
	// 826C6360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C636C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6370: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6374: 38AA5EF4  addi r5, r10, 0x5ef4
	ctx.r[5].s64 = ctx.r[10].s64 + 24308;
	// 826C6378: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C637C: 390B6490  addi r8, r11, 0x6490
	ctx.r[8].s64 = ctx.r[11].s64 + 25744;
	// 826C6380: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C6384: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 826C6388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C638C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6390: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6398: 386A5D14  addi r3, r10, 0x5d14
	ctx.r[3].s64 = ctx.r[10].s64 + 23828;
	// 826C639C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C63A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C63A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C63A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C63AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C63B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C63B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C63B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C63BC: 4BDA0A65  bl 0x82466e20
	ctx.lr = 0x826C63C0;
	sub_82466E20(ctx, base);
	// 826C63C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C63C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C63C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C63CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C63D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C63D0 size=112
    let mut pc: u32 = 0x826C63D0;
    'dispatch: loop {
        match pc {
            0x826C63D0 => {
    //   block [0x826C63D0..0x826C6440)
	// 826C63D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C63D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C63D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C63DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C63E0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C63E4: 38AA5CB4  addi r5, r10, 0x5cb4
	ctx.r[5].s64 = ctx.r[10].s64 + 23732;
	// 826C63E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C63EC: 390B64A8  addi r8, r11, 0x64a8
	ctx.r[8].s64 = ctx.r[11].s64 + 25768;
	// 826C63F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C63F4: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 826C63F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C63FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6408: 386A5D44  addi r3, r10, 0x5d44
	ctx.r[3].s64 = ctx.r[10].s64 + 23876;
	// 826C640C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C641C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C642C: 4BDA09F5  bl 0x82466e20
	ctx.lr = 0x826C6430;
	sub_82466E20(ctx, base);
	// 826C6430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C643C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6440 size=112
    let mut pc: u32 = 0x826C6440;
    'dispatch: loop {
        match pc {
            0x826C6440 => {
    //   block [0x826C6440..0x826C64B0)
	// 826C6440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C644C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6450: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6454: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C6458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C645C: 390B64F0  addi r8, r11, 0x64f0
	ctx.r[8].s64 = ctx.r[11].s64 + 25840;
	// 826C6460: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C6464: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 826C6468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C646C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6470: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6478: 386A5D74  addi r3, r10, 0x5d74
	ctx.r[3].s64 = ctx.r[10].s64 + 23924;
	// 826C647C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C648C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C649C: 4BDA0985  bl 0x82466e20
	ctx.lr = 0x826C64A0;
	sub_82466E20(ctx, base);
	// 826C64A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C64A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C64A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C64AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C64B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C64B0 size=112
    let mut pc: u32 = 0x826C64B0;
    'dispatch: loop {
        match pc {
            0x826C64B0 => {
    //   block [0x826C64B0..0x826C6520)
	// 826C64B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C64B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C64B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C64BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C64C0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C64C4: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C64C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C64CC: 390B6520  addi r8, r11, 0x6520
	ctx.r[8].s64 = ctx.r[11].s64 + 25888;
	// 826C64D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C64D4: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 826C64D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C64DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C64E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C64E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C64E8: 386A5DA4  addi r3, r10, 0x5da4
	ctx.r[3].s64 = ctx.r[10].s64 + 23972;
	// 826C64EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C64F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C64F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C64F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C64FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C650C: 4BDA0915  bl 0x82466e20
	ctx.lr = 0x826C6510;
	sub_82466E20(ctx, base);
	// 826C6510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C651C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6520 size=108
    let mut pc: u32 = 0x826C6520;
    'dispatch: loop {
        match pc {
            0x826C6520 => {
    //   block [0x826C6520..0x826C658C)
	// 826C6520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C652C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6534: 38EB6550  addi r7, r11, 0x6550
	ctx.r[7].s64 = ctx.r[11].s64 + 25936;
	// 826C6538: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C653C: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826C6540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6544: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6548: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C654C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6550: 386A5DD4  addi r3, r10, 0x5dd4
	ctx.r[3].s64 = ctx.r[10].s64 + 24020;
	// 826C6554: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C6558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C655C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C656C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6574: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C6578: 4BDA08A9  bl 0x82466e20
	ctx.lr = 0x826C657C;
	sub_82466E20(ctx, base);
	// 826C657C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6590 size=112
    let mut pc: u32 = 0x826C6590;
    'dispatch: loop {
        match pc {
            0x826C6590 => {
    //   block [0x826C6590..0x826C6600)
	// 826C6590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C659C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C65A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C65A4: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C65A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C65AC: 390B6598  addi r8, r11, 0x6598
	ctx.r[8].s64 = ctx.r[11].s64 + 26008;
	// 826C65B0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826C65B4: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 826C65B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C65BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C65C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C65C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C65C8: 386A5E04  addi r3, r10, 0x5e04
	ctx.r[3].s64 = ctx.r[10].s64 + 24068;
	// 826C65CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C65D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C65D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C65D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C65DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C65E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C65E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C65E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C65EC: 4BDA0835  bl 0x82466e20
	ctx.lr = 0x826C65F0;
	sub_82466E20(ctx, base);
	// 826C65F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C65F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C65F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C65FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6600 size=116
    let mut pc: u32 = 0x826C6600;
    'dispatch: loop {
        match pc {
            0x826C6600 => {
    //   block [0x826C6600..0x826C6674)
	// 826C6600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C660C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C6610: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6614: 392B23B4  addi r9, r11, 0x23b4
	ctx.r[9].s64 = ctx.r[11].s64 + 9140;
	// 826C6618: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C661C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6620: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826C6624: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826C6628: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C662C: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826C6630: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6634: 396B6618  addi r11, r11, 0x6618
	ctx.r[11].s64 = ctx.r[11].s64 + 26136;
	// 826C6638: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C663C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6640: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C6644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6648: 386A5E34  addi r3, r10, 0x5e34
	ctx.r[3].s64 = ctx.r[10].s64 + 24116;
	// 826C664C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C6650: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C6654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6658: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C665C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C6660: 4BDA07C1  bl 0x82466e20
	ctx.lr = 0x826C6664;
	sub_82466E20(ctx, base);
	// 826C6664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C666C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6678 size=100
    let mut pc: u32 = 0x826C6678;
    'dispatch: loop {
        match pc {
            0x826C6678 => {
    //   block [0x826C6678..0x826C66DC)
	// 826C6678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C667C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6684: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C668C: 38AA5F84  addi r5, r10, 0x5f84
	ctx.r[5].s64 = ctx.r[10].s64 + 24452;
	// 826C6690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6698: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 826C669C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C66A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C66A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C66A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C66AC: 386A5E64  addi r3, r10, 0x5e64
	ctx.r[3].s64 = ctx.r[10].s64 + 24164;
	// 826C66B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C66B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C66B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C66BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C66C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C66C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C66C8: 4BDA0759  bl 0x82466e20
	ctx.lr = 0x826C66CC;
	sub_82466E20(ctx, base);
	// 826C66CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C66D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C66D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C66D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C66E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C66E0 size=100
    let mut pc: u32 = 0x826C66E0;
    'dispatch: loop {
        match pc {
            0x826C66E0 => {
    //   block [0x826C66E0..0x826C6744)
	// 826C66E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C66E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C66E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C66EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C66F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C66F4: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C66F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C66FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6700: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826C6704: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C670C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6714: 386A5E94  addi r3, r10, 0x5e94
	ctx.r[3].s64 = ctx.r[10].s64 + 24212;
	// 826C6718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C671C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6720: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C6724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6728: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C672C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6730: 4BDA06F1  bl 0x82466e20
	ctx.lr = 0x826C6734;
	sub_82466E20(ctx, base);
	// 826C6734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C673C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6748 size=108
    let mut pc: u32 = 0x826C6748;
    'dispatch: loop {
        match pc {
            0x826C6748 => {
    //   block [0x826C6748..0x826C67B4)
	// 826C6748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C674C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6754: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C675C: 38EB66A8  addi r7, r11, 0x66a8
	ctx.r[7].s64 = ctx.r[11].s64 + 26280;
	// 826C6760: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C6764: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826C6768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C676C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C6774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6778: 386A5EC4  addi r3, r10, 0x5ec4
	ctx.r[3].s64 = ctx.r[10].s64 + 24260;
	// 826C677C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C6780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C678C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C679C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C67A0: 4BDA0681  bl 0x82466e20
	ctx.lr = 0x826C67A4;
	sub_82466E20(ctx, base);
	// 826C67A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C67A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C67AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C67B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C67B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C67B8 size=112
    let mut pc: u32 = 0x826C67B8;
    'dispatch: loop {
        match pc {
            0x826C67B8 => {
    //   block [0x826C67B8..0x826C6828)
	// 826C67B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C67BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C67C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C67C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C67C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C67CC: 38AA5CB4  addi r5, r10, 0x5cb4
	ctx.r[5].s64 = ctx.r[10].s64 + 23732;
	// 826C67D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C67D4: 390B66D8  addi r8, r11, 0x66d8
	ctx.r[8].s64 = ctx.r[11].s64 + 26328;
	// 826C67D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C67DC: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 826C67E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C67E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C67E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C67EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C67F0: 386A5EF4  addi r3, r10, 0x5ef4
	ctx.r[3].s64 = ctx.r[10].s64 + 24308;
	// 826C67F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C67F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C67FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C680C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6814: 4BDA060D  bl 0x82466e20
	ctx.lr = 0x826C6818;
	sub_82466E20(ctx, base);
	// 826C6818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C681C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6828 size=108
    let mut pc: u32 = 0x826C6828;
    'dispatch: loop {
        match pc {
            0x826C6828 => {
    //   block [0x826C6828..0x826C6894)
	// 826C6828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C682C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6834: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C683C: 38EB66F0  addi r7, r11, 0x66f0
	ctx.r[7].s64 = ctx.r[11].s64 + 26352;
	// 826C6840: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C6844: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 826C6848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C684C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C6854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6858: 386A5F24  addi r3, r10, 0x5f24
	ctx.r[3].s64 = ctx.r[10].s64 + 24356;
	// 826C685C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C6860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C686C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C687C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C6880: 4BDA05A1  bl 0x82466e20
	ctx.lr = 0x826C6884;
	sub_82466E20(ctx, base);
	// 826C6884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C688C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C6898 size=28
    let mut pc: u32 = 0x826C6898;
    'dispatch: loop {
        match pc {
            0x826C6898 => {
    //   block [0x826C6898..0x826C68B4)
	// 826C6898: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C689C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C68A0: 394AA2E0  addi r10, r10, -0x5d20
	ctx.r[10].s64 = ctx.r[10].s64 + -23840;
	// 826C68A4: 816B6614  lwz r11, 0x6614(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26132 as u32) ) } as u64;
	// 826C68A8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826C68AC: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826C68B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C68B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C68B8 size=108
    let mut pc: u32 = 0x826C68B8;
    'dispatch: loop {
        match pc {
            0x826C68B8 => {
    //   block [0x826C68B8..0x826C6924)
	// 826C68B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C68BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C68C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C68C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C68C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C68CC: 38EBA2E0  addi r7, r11, -0x5d20
	ctx.r[7].s64 = ctx.r[11].s64 + -23840;
	// 826C68D0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826C68D4: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 826C68D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C68DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C68E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C68E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C68E8: 386A5F54  addi r3, r10, 0x5f54
	ctx.r[3].s64 = ctx.r[10].s64 + 24404;
	// 826C68EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C68F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C68F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C68F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C68FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C690C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C6910: 4BDA0511  bl 0x82466e20
	ctx.lr = 0x826C6914;
	sub_82466E20(ctx, base);
	// 826C6914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C691C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6928 size=116
    let mut pc: u32 = 0x826C6928;
    'dispatch: loop {
        match pc {
            0x826C6928 => {
    //   block [0x826C6928..0x826C699C)
	// 826C6928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C692C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6934: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6938: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C693C: 390B6710  addi r8, r11, 0x6710
	ctx.r[8].s64 = ctx.r[11].s64 + 26384;
	// 826C6940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6944: 392A2424  addi r9, r10, 0x2424
	ctx.r[9].s64 = ctx.r[10].s64 + 9252;
	// 826C6948: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C694C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826C6950: 38AA5CB4  addi r5, r10, 0x5cb4
	ctx.r[5].s64 = ctx.r[10].s64 + 23732;
	// 826C6954: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C695C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C696C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C6970: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826C6974: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C6978: 386B5F84  addi r3, r11, 0x5f84
	ctx.r[3].s64 = ctx.r[11].s64 + 24452;
	// 826C697C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826C6980: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6988: 4BDA0499  bl 0x82466e20
	ctx.lr = 0x826C698C;
	sub_82466E20(ctx, base);
	// 826C698C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C69A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C69A0 size=112
    let mut pc: u32 = 0x826C69A0;
    'dispatch: loop {
        match pc {
            0x826C69A0 => {
    //   block [0x826C69A0..0x826C6A10)
	// 826C69A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C69A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C69A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C69AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C69B0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C69B4: 38AA5C54  addi r5, r10, 0x5c54
	ctx.r[5].s64 = ctx.r[10].s64 + 23636;
	// 826C69B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C69BC: 390B6788  addi r8, r11, 0x6788
	ctx.r[8].s64 = ctx.r[11].s64 + 26504;
	// 826C69C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C69C4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 826C69C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C69CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C69D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C69D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C69D8: 386A5FB4  addi r3, r10, 0x5fb4
	ctx.r[3].s64 = ctx.r[10].s64 + 24500;
	// 826C69DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C69E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C69E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C69E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C69EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C69F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C69F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C69F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C69FC: 4BDA0425  bl 0x82466e20
	ctx.lr = 0x826C6A00;
	sub_82466E20(ctx, base);
	// 826C6A00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6A10 size=108
    let mut pc: u32 = 0x826C6A10;
    'dispatch: loop {
        match pc {
            0x826C6A10 => {
    //   block [0x826C6A10..0x826C6A7C)
	// 826C6A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6A1C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6A24: 38EB67A0  addi r7, r11, 0x67a0
	ctx.r[7].s64 = ctx.r[11].s64 + 26528;
	// 826C6A28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C6A2C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826C6A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6A34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6A38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C6A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6A40: 386A5FE4  addi r3, r10, 0x5fe4
	ctx.r[3].s64 = ctx.r[10].s64 + 24548;
	// 826C6A44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C6A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C6A68: 4BDA03B9  bl 0x82466e20
	ctx.lr = 0x826C6A6C;
	sub_82466E20(ctx, base);
	// 826C6A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6A80 size=112
    let mut pc: u32 = 0x826C6A80;
    'dispatch: loop {
        match pc {
            0x826C6A80 => {
    //   block [0x826C6A80..0x826C6AF0)
	// 826C6A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6A8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6A90: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6A94: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C6A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6A9C: 390B67D0  addi r8, r11, 0x67d0
	ctx.r[8].s64 = ctx.r[11].s64 + 26576;
	// 826C6AA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C6AA4: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 826C6AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6AAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6AB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6AB8: 386A6014  addi r3, r10, 0x6014
	ctx.r[3].s64 = ctx.r[10].s64 + 24596;
	// 826C6ABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6ADC: 4BDA0345  bl 0x82466e20
	ctx.lr = 0x826C6AE0;
	sub_82466E20(ctx, base);
	// 826C6AE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


