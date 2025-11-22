pub fn sub_827045A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827045A0 size=112
    let mut pc: u32 = 0x827045A0;
    'dispatch: loop {
        match pc {
            0x827045A0 => {
    //   block [0x827045A0..0x82704610)
	// 827045A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827045A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827045A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827045AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827045B0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827045B4: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 827045B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827045BC: 390BE0D0  addi r8, r11, -0x1f30
	ctx.r[8].s64 = ctx.r[11].s64 + -7984;
	// 827045C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827045C4: 388ABFFC  addi r4, r10, -0x4004
	ctx.r[4].s64 = ctx.r[10].s64 + -16388;
	// 827045C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827045CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827045D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827045D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827045D8: 386AFEE4  addi r3, r10, -0x11c
	ctx.r[3].s64 = ctx.r[10].s64 + -284;
	// 827045DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827045E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827045E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827045E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827045EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827045F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827045F4: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 827045F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827045FC: 4BD62825  bl 0x82466e20
	ctx.lr = 0x82704600;
	sub_82466E20(ctx, base);
	// 82704600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270460C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704610 size=108
    let mut pc: u32 = 0x82704610;
    'dispatch: loop {
        match pc {
            0x82704610 => {
    //   block [0x82704610..0x8270467C)
	// 82704610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270461C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704620: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704624: 392BE138  addi r9, r11, -0x1ec8
	ctx.r[9].s64 = ctx.r[11].s64 + -7880;
	// 82704628: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8270462C: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 82704630: 388AC200  addi r4, r10, -0x3e00
	ctx.r[4].s64 = ctx.r[10].s64 + -15872;
	// 82704634: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704638: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270463C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82704640: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 82704644: 386AFF14  addi r3, r10, -0xec
	ctx.r[3].s64 = ctx.r[10].s64 + -236;
	// 82704648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270464C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82704650: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704654: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704658: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270465C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704660: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82704664: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704668: 4BD627B9  bl 0x82466e20
	ctx.lr = 0x8270466C;
	sub_82466E20(ctx, base);
	// 8270466C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704680 size=112
    let mut pc: u32 = 0x82704680;
    'dispatch: loop {
        match pc {
            0x82704680 => {
    //   block [0x82704680..0x827046F0)
	// 82704680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270468C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704690: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704694: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82704698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270469C: 390BE198  addi r8, r11, -0x1e68
	ctx.r[8].s64 = ctx.r[11].s64 + -7784;
	// 827046A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827046A4: 388AC218  addi r4, r10, -0x3de8
	ctx.r[4].s64 = ctx.r[10].s64 + -15848;
	// 827046A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827046AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827046B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827046B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827046B8: 386AFF44  addi r3, r10, -0xbc
	ctx.r[3].s64 = ctx.r[10].s64 + -188;
	// 827046BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827046C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827046C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827046C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827046CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827046D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827046D4: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 827046D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827046DC: 4BD62745  bl 0x82466e20
	ctx.lr = 0x827046E0;
	sub_82466E20(ctx, base);
	// 827046E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827046E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827046E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827046EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827046F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827046F0 size=108
    let mut pc: u32 = 0x827046F0;
    'dispatch: loop {
        match pc {
            0x827046F0 => {
    //   block [0x827046F0..0x8270475C)
	// 827046F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827046F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827046F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827046FC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704704: 392BE200  addi r9, r11, -0x1e00
	ctx.r[9].s64 = ctx.r[11].s64 + -7680;
	// 82704708: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8270470C: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 82704710: 388AC264  addi r4, r10, -0x3d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15772;
	// 82704714: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704718: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270471C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82704720: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 82704724: 386AFF74  addi r3, r10, -0x8c
	ctx.r[3].s64 = ctx.r[10].s64 + -140;
	// 82704728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270472C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82704730: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704734: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704738: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270473C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704740: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82704744: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704748: 4BD626D9  bl 0x82466e20
	ctx.lr = 0x8270474C;
	sub_82466E20(ctx, base);
	// 8270474C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704760 size=112
    let mut pc: u32 = 0x82704760;
    'dispatch: loop {
        match pc {
            0x82704760 => {
    //   block [0x82704760..0x827047D0)
	// 82704760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270476C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704770: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704774: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82704778: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270477C: 390BE278  addi r8, r11, -0x1d88
	ctx.r[8].s64 = ctx.r[11].s64 + -7560;
	// 82704780: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82704784: 388AC280  addi r4, r10, -0x3d80
	ctx.r[4].s64 = ctx.r[10].s64 + -15744;
	// 82704788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270478C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704798: 386AFFA4  addi r3, r10, -0x5c
	ctx.r[3].s64 = ctx.r[10].s64 + -92;
	// 8270479C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827047A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827047A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827047A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827047AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827047B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827047B4: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 827047B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827047BC: 4BD62665  bl 0x82466e20
	ctx.lr = 0x827047C0;
	sub_82466E20(ctx, base);
	// 827047C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827047C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827047C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827047CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827047D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827047D0 size=112
    let mut pc: u32 = 0x827047D0;
    'dispatch: loop {
        match pc {
            0x827047D0 => {
    //   block [0x827047D0..0x82704840)
	// 827047D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827047D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827047D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827047DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827047E0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827047E4: 38AA0C78  addi r5, r10, 0xc78
	ctx.r[5].s64 = ctx.r[10].s64 + 3192;
	// 827047E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827047EC: 390BE2C4  addi r8, r11, -0x1d3c
	ctx.r[8].s64 = ctx.r[11].s64 + -7484;
	// 827047F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827047F4: 388AC3CC  addi r4, r10, -0x3c34
	ctx.r[4].s64 = ctx.r[10].s64 + -15412;
	// 827047F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827047FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704808: 386AFFD4  addi r3, r10, -0x2c
	ctx.r[3].s64 = ctx.r[10].s64 + -44;
	// 8270480C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270481C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704824: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 82704828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270482C: 4BD625F5  bl 0x82466e20
	ctx.lr = 0x82704830;
	sub_82466E20(ctx, base);
	// 82704830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270483C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704840 size=100
    let mut pc: u32 = 0x82704840;
    'dispatch: loop {
        match pc {
            0x82704840 => {
    //   block [0x82704840..0x827048A4)
	// 82704840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270484C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704854: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82704858: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270485C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704860: 388AC168  addi r4, r10, -0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + -16024;
	// 82704864: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270486C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704874: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82704878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270487C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704880: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82704884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704888: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270488C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82704890: 4BD62591  bl 0x82466e20
	ctx.lr = 0x82704894;
	sub_82466E20(ctx, base);
	// 82704894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270489C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827048A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827048A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827048A8 size=24
    let mut pc: u32 = 0x827048A8;
    'dispatch: loop {
        match pc {
            0x827048A8 => {
    //   block [0x827048A8..0x827048C0)
	// 827048A8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827048AC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827048B0: 394AA288  addi r10, r10, -0x5d78
	ctx.r[10].s64 = ctx.r[10].s64 + -23928;
	// 827048B4: 816BA274  lwz r11, -0x5d8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23948 as u32) ) } as u64;
	// 827048B8: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 827048BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827048C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827048C0 size=116
    let mut pc: u32 = 0x827048C0;
    'dispatch: loop {
        match pc {
            0x827048C0 => {
    //   block [0x827048C0..0x82704934)
	// 827048C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827048C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827048C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827048CC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827048D0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827048D4: 392AE370  addi r9, r10, -0x1c90
	ctx.r[9].s64 = ctx.r[10].s64 + -7312;
	// 827048D8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 827048DC: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 827048E0: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 827048E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827048E8: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 827048EC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827048F0: 388AC75C  addi r4, r10, -0x38a4
	ctx.r[4].s64 = ctx.r[10].s64 + -14500;
	// 827048F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827048F8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 827048FC: 396BA288  addi r11, r11, -0x5d78
	ctx.r[11].s64 = ctx.r[11].s64 + -23928;
	// 82704900: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704908: 386A0034  addi r3, r10, 0x34
	ctx.r[3].s64 = ctx.r[10].s64 + 52;
	// 8270490C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82704910: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82704914: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82704918: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8270491C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82704920: 4BD62501  bl 0x82466e20
	ctx.lr = 0x82704924;
	sub_82466E20(ctx, base);
	// 82704924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270492C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704938 size=112
    let mut pc: u32 = 0x82704938;
    'dispatch: loop {
        match pc {
            0x82704938 => {
    //   block [0x82704938..0x827049A8)
	// 82704938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270493C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704940: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704944: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704948: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8270494C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82704950: 396BE418  addi r11, r11, -0x1be8
	ctx.r[11].s64 = ctx.r[11].s64 + -7144;
	// 82704954: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82704958: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 8270495C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82704960: 4BD9E559  bl 0x824a2eb8
	ctx.lr = 0x82704964;
	sub_824A2EB8(ctx, base);
	// 82704964: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82704968: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 8270496C: 394BBFB8  addi r10, r11, -0x4048
	ctx.r[10].s64 = ctx.r[11].s64 + -16456;
	// 82704970: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82704974: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 82704978: 396B0064  addi r11, r11, 0x64
	ctx.r[11].s64 = ctx.r[11].s64 + 100;
	// 8270497C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82704980: 394813D0  addi r10, r8, 0x13d0
	ctx.r[10].s64 = ctx.r[8].s64 + 5072;
	// 82704984: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82704988: 394913B8  addi r10, r9, 0x13b8
	ctx.r[10].s64 = ctx.r[9].s64 + 5048;
	// 8270498C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82704990: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82704994: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82704998: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 8270499C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827049A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827049A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827049A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827049A8 size=24
    let mut pc: u32 = 0x827049A8;
    'dispatch: loop {
        match pc {
            0x827049A8 => {
    //   block [0x827049A8..0x827049C0)
	// 827049A8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827049AC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827049B0: 394AA388  addi r10, r10, -0x5c78
	ctx.r[10].s64 = ctx.r[10].s64 + -23672;
	// 827049B4: 816BA380  lwz r11, -0x5c80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23680 as u32) ) } as u64;
	// 827049B8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827049BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827049C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827049C0 size=116
    let mut pc: u32 = 0x827049C0;
    'dispatch: loop {
        match pc {
            0x827049C0 => {
    //   block [0x827049C0..0x82704A34)
	// 827049C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827049C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827049C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827049CC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827049D0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827049D4: 390BA388  addi r8, r11, -0x5c78
	ctx.r[8].s64 = ctx.r[11].s64 + -23672;
	// 827049D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827049DC: 392AE400  addi r9, r10, -0x1c00
	ctx.r[9].s64 = ctx.r[10].s64 + -7168;
	// 827049E0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827049E4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 827049E8: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 827049EC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827049F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827049F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827049F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827049FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704A04: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82704A08: 388ABFB8  addi r4, r10, -0x4048
	ctx.r[4].s64 = ctx.r[10].s64 + -16456;
	// 82704A0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82704A10: 386B0074  addi r3, r11, 0x74
	ctx.r[3].s64 = ctx.r[11].s64 + 116;
	// 82704A14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82704A18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704A1C: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 82704A20: 4BD62401  bl 0x82466e20
	ctx.lr = 0x82704A24;
	sub_82466E20(ctx, base);
	// 82704A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704A38 size=112
    let mut pc: u32 = 0x82704A38;
    'dispatch: loop {
        match pc {
            0x82704A38 => {
    //   block [0x82704A38..0x82704AA8)
	// 82704A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704A44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704A48: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704A4C: 38AA0C48  addi r5, r10, 0xc48
	ctx.r[5].s64 = ctx.r[10].s64 + 3144;
	// 82704A50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704A54: 390BE448  addi r8, r11, -0x1bb8
	ctx.r[8].s64 = ctx.r[11].s64 + -7096;
	// 82704A58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82704A5C: 388ABE44  addi r4, r10, -0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16828;
	// 82704A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704A64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704A68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704A70: 386A00A4  addi r3, r10, 0xa4
	ctx.r[3].s64 = ctx.r[10].s64 + 164;
	// 82704A74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704A78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704A8C: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82704A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704A94: 4BD6238D  bl 0x82466e20
	ctx.lr = 0x82704A98;
	sub_82466E20(ctx, base);
	// 82704A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704AA8 size=100
    let mut pc: u32 = 0x82704AA8;
    'dispatch: loop {
        match pc {
            0x82704AA8 => {
    //   block [0x82704AA8..0x82704B0C)
	// 82704AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704AB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704ABC: 38AA0BD8  addi r5, r10, 0xbd8
	ctx.r[5].s64 = ctx.r[10].s64 + 3032;
	// 82704AC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704AC8: 388AC59C  addi r4, r10, -0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + -14948;
	// 82704ACC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704ADC: 386A00D4  addi r3, r10, 0xd4
	ctx.r[3].s64 = ctx.r[10].s64 + 212;
	// 82704AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704AE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704AE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82704AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704AF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82704AF4: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 82704AF8: 4BD62329  bl 0x82466e20
	ctx.lr = 0x82704AFC;
	sub_82466E20(ctx, base);
	// 82704AFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704B10 size=100
    let mut pc: u32 = 0x82704B10;
    'dispatch: loop {
        match pc {
            0x82704B10 => {
    //   block [0x82704B10..0x82704B74)
	// 82704B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704B1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704B24: 38AA00D4  addi r5, r10, 0xd4
	ctx.r[5].s64 = ctx.r[10].s64 + 212;
	// 82704B28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704B30: 388AC5B4  addi r4, r10, -0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + -14924;
	// 82704B34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704B44: 386A0104  addi r3, r10, 0x104
	ctx.r[3].s64 = ctx.r[10].s64 + 260;
	// 82704B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704B4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704B50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82704B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704B58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82704B5C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 82704B60: 4BD622C1  bl 0x82466e20
	ctx.lr = 0x82704B64;
	sub_82466E20(ctx, base);
	// 82704B64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704B78 size=100
    let mut pc: u32 = 0x82704B78;
    'dispatch: loop {
        match pc {
            0x82704B78 => {
    //   block [0x82704B78..0x82704BDC)
	// 82704B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704B84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704B8C: 38AA04B4  addi r5, r10, 0x4b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1204;
	// 82704B90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704B98: 388AC5D8  addi r4, r10, -0x3a28
	ctx.r[4].s64 = ctx.r[10].s64 + -14888;
	// 82704B9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704BAC: 386A0134  addi r3, r10, 0x134
	ctx.r[3].s64 = ctx.r[10].s64 + 308;
	// 82704BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704BB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704BB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82704BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704BC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82704BC4: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 82704BC8: 4BD62259  bl 0x82466e20
	ctx.lr = 0x82704BCC;
	sub_82466E20(ctx, base);
	// 82704BCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704BE0 size=92
    let mut pc: u32 = 0x82704BE0;
    'dispatch: loop {
        match pc {
            0x82704BE0 => {
    //   block [0x82704BE0..0x82704C3C)
	// 82704BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704BE8: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704BEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82704BF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82704BF4: 4BD8CA55  bl 0x82491648
	ctx.lr = 0x82704BF8;
	sub_82491648(ctx, base);
	// 82704BF8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82704BFC: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 82704C00: 394BC630  addi r10, r11, -0x39d0
	ctx.r[10].s64 = ctx.r[11].s64 + -14800;
	// 82704C04: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82704C08: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 82704C0C: 396B0164  addi r11, r11, 0x164
	ctx.r[11].s64 = ctx.r[11].s64 + 356;
	// 82704C10: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82704C14: 39481598  addi r10, r8, 0x1598
	ctx.r[10].s64 = ctx.r[8].s64 + 5528;
	// 82704C18: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82704C1C: 39491580  addi r10, r9, 0x1580
	ctx.r[10].s64 = ctx.r[9].s64 + 5504;
	// 82704C20: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82704C24: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82704C28: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82704C2C: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 82704C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704C40 size=112
    let mut pc: u32 = 0x82704C40;
    'dispatch: loop {
        match pc {
            0x82704C40 => {
    //   block [0x82704C40..0x82704CB0)
	// 82704C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704C4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704C50: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704C54: 38AA0E18  addi r5, r10, 0xe18
	ctx.r[5].s64 = ctx.r[10].s64 + 3608;
	// 82704C58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704C5C: 390BE460  addi r8, r11, -0x1ba0
	ctx.r[8].s64 = ctx.r[11].s64 + -7072;
	// 82704C60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82704C64: 388AC630  addi r4, r10, -0x39d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14800;
	// 82704C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704C6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704C70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704C74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704C78: 386A0174  addi r3, r10, 0x174
	ctx.r[3].s64 = ctx.r[10].s64 + 372;
	// 82704C7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704C8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704C94: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 82704C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704C9C: 4BD62185  bl 0x82466e20
	ctx.lr = 0x82704CA0;
	sub_82466E20(ctx, base);
	// 82704CA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704CB0 size=108
    let mut pc: u32 = 0x82704CB0;
    'dispatch: loop {
        match pc {
            0x82704CB0 => {
    //   block [0x82704CB0..0x82704D1C)
	// 82704CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704CBC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704CC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704CC4: 38EBE580  addi r7, r11, -0x1a80
	ctx.r[7].s64 = ctx.r[11].s64 + -6784;
	// 82704CC8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82704CCC: 388AC440  addi r4, r10, -0x3bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -15296;
	// 82704CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704CD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82704CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704CE0: 386A01A4  addi r3, r10, 0x1a4
	ctx.r[3].s64 = ctx.r[10].s64 + 420;
	// 82704CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82704CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704CFC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82704D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82704D08: 4BD62119  bl 0x82466e20
	ctx.lr = 0x82704D0C;
	sub_82466E20(ctx, base);
	// 82704D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704D20 size=108
    let mut pc: u32 = 0x82704D20;
    'dispatch: loop {
        match pc {
            0x82704D20 => {
    //   block [0x82704D20..0x82704D8C)
	// 82704D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704D2C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704D30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704D34: 38EBE5C8  addi r7, r11, -0x1a38
	ctx.r[7].s64 = ctx.r[11].s64 + -6712;
	// 82704D38: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82704D3C: 388AC46C  addi r4, r10, -0x3b94
	ctx.r[4].s64 = ctx.r[10].s64 + -15252;
	// 82704D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704D44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704D48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82704D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704D50: 386A01D4  addi r3, r10, 0x1d4
	ctx.r[3].s64 = ctx.r[10].s64 + 468;
	// 82704D54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82704D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704D6C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82704D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82704D78: 4BD620A9  bl 0x82466e20
	ctx.lr = 0x82704D7C;
	sub_82466E20(ctx, base);
	// 82704D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704D90 size=108
    let mut pc: u32 = 0x82704D90;
    'dispatch: loop {
        match pc {
            0x82704D90 => {
    //   block [0x82704D90..0x82704DFC)
	// 82704D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704D9C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704DA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704DA4: 38EBE628  addi r7, r11, -0x19d8
	ctx.r[7].s64 = ctx.r[11].s64 + -6616;
	// 82704DA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82704DAC: 388AC48C  addi r4, r10, -0x3b74
	ctx.r[4].s64 = ctx.r[10].s64 + -15220;
	// 82704DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704DB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704DB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82704DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704DC0: 386A0204  addi r3, r10, 0x204
	ctx.r[3].s64 = ctx.r[10].s64 + 516;
	// 82704DC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82704DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704DDC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82704DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82704DE8: 4BD62039  bl 0x82466e20
	ctx.lr = 0x82704DEC;
	sub_82466E20(ctx, base);
	// 82704DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704E00 size=92
    let mut pc: u32 = 0x82704E00;
    'dispatch: loop {
        match pc {
            0x82704E00 => {
    //   block [0x82704E00..0x82704E5C)
	// 82704E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704E08: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704E0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82704E10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82704E14: 4BD89A75  bl 0x8248e888
	ctx.lr = 0x82704E18;
	sub_8248E888(ctx, base);
	// 82704E18: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82704E1C: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 82704E20: 394BC4A8  addi r10, r11, -0x3b58
	ctx.r[10].s64 = ctx.r[11].s64 + -15192;
	// 82704E24: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82704E28: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 82704E2C: 396B0234  addi r11, r11, 0x234
	ctx.r[11].s64 = ctx.r[11].s64 + 564;
	// 82704E30: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82704E34: 39481740  addi r10, r8, 0x1740
	ctx.r[10].s64 = ctx.r[8].s64 + 5952;
	// 82704E38: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82704E3C: 39491728  addi r10, r9, 0x1728
	ctx.r[10].s64 = ctx.r[9].s64 + 5928;
	// 82704E40: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82704E44: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82704E48: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82704E4C: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 82704E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704E60 size=116
    let mut pc: u32 = 0x82704E60;
    'dispatch: loop {
        match pc {
            0x82704E60 => {
    //   block [0x82704E60..0x82704ED4)
	// 82704E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704E6C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704E70: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704E74: 392BE568  addi r9, r11, -0x1a98
	ctx.r[9].s64 = ctx.r[11].s64 + -6808;
	// 82704E78: 38AA0DE8  addi r5, r10, 0xde8
	ctx.r[5].s64 = ctx.r[10].s64 + 3560;
	// 82704E7C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704E80: 38E90288  addi r7, r9, 0x288
	ctx.r[7].s64 = ctx.r[9].s64 + 648;
	// 82704E84: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 82704E88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704E8C: 388AC4A8  addi r4, r10, -0x3b58
	ctx.r[4].s64 = ctx.r[10].s64 + -15192;
	// 82704E90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704E94: 396BE658  addi r11, r11, -0x19a8
	ctx.r[11].s64 = ctx.r[11].s64 + -6568;
	// 82704E98: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82704E9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704EA0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82704EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704EA8: 386A0244  addi r3, r10, 0x244
	ctx.r[3].s64 = ctx.r[10].s64 + 580;
	// 82704EAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82704EB0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82704EB4: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 82704EB8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82704EBC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82704EC0: 4BD61F61  bl 0x82466e20
	ctx.lr = 0x82704EC4;
	sub_82466E20(ctx, base);
	// 82704EC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704ED8 size=112
    let mut pc: u32 = 0x82704ED8;
    'dispatch: loop {
        match pc {
            0x82704ED8 => {
    //   block [0x82704ED8..0x82704F48)
	// 82704ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704EE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82704EE8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704EEC: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82704EF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704EF4: 390BE838  addi r8, r11, -0x17c8
	ctx.r[8].s64 = ctx.r[11].s64 + -6088;
	// 82704EF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82704EFC: 388AC6E4  addi r4, r10, -0x391c
	ctx.r[4].s64 = ctx.r[10].s64 + -14620;
	// 82704F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704F04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704F10: 386A0274  addi r3, r10, 0x274
	ctx.r[3].s64 = ctx.r[10].s64 + 628;
	// 82704F14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704F2C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82704F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704F34: 4BD61EED  bl 0x82466e20
	ctx.lr = 0x82704F38;
	sub_82466E20(ctx, base);
	// 82704F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704F48 size=108
    let mut pc: u32 = 0x82704F48;
    'dispatch: loop {
        match pc {
            0x82704F48 => {
    //   block [0x82704F48..0x82704FB4)
	// 82704F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704F54: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704F58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704F5C: 38EBE850  addi r7, r11, -0x17b0
	ctx.r[7].s64 = ctx.r[11].s64 + -6064;
	// 82704F60: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82704F64: 388ABE80  addi r4, r10, -0x4180
	ctx.r[4].s64 = ctx.r[10].s64 + -16768;
	// 82704F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704F6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704F70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82704F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704F78: 386A02A4  addi r3, r10, 0x2a4
	ctx.r[3].s64 = ctx.r[10].s64 + 676;
	// 82704F7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82704F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704F94: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82704F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704F9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82704FA0: 4BD61E81  bl 0x82466e20
	ctx.lr = 0x82704FA4;
	sub_82466E20(ctx, base);
	// 82704FA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704FB8 size=108
    let mut pc: u32 = 0x82704FB8;
    'dispatch: loop {
        match pc {
            0x82704FB8 => {
    //   block [0x82704FB8..0x82705024)
	// 82704FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704FC4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704FC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704FCC: 38EBE868  addi r7, r11, -0x1798
	ctx.r[7].s64 = ctx.r[11].s64 + -6040;
	// 82704FD0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82704FD4: 388ABE94  addi r4, r10, -0x416c
	ctx.r[4].s64 = ctx.r[10].s64 + -16748;
	// 82704FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704FDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704FE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82704FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704FE8: 386A02D4  addi r3, r10, 0x2d4
	ctx.r[3].s64 = ctx.r[10].s64 + 724;
	// 82704FEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82704FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705004: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82705008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270500C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82705010: 4BD61E11  bl 0x82466e20
	ctx.lr = 0x82705014;
	sub_82466E20(ctx, base);
	// 82705014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270501C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705028 size=112
    let mut pc: u32 = 0x82705028;
    'dispatch: loop {
        match pc {
            0x82705028 => {
    //   block [0x82705028..0x82705098)
	// 82705028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270502C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705034: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705038: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270503C: 38AA0C78  addi r5, r10, 0xc78
	ctx.r[5].s64 = ctx.r[10].s64 + 3192;
	// 82705040: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705044: 390BE8B0  addi r8, r11, -0x1750
	ctx.r[8].s64 = ctx.r[11].s64 + -5968;
	// 82705048: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8270504C: 388AC3B0  addi r4, r10, -0x3c50
	ctx.r[4].s64 = ctx.r[10].s64 + -15440;
	// 82705050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705054: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270505C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705060: 386A0304  addi r3, r10, 0x304
	ctx.r[3].s64 = ctx.r[10].s64 + 772;
	// 82705064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270506C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270507C: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82705080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705084: 4BD61D9D  bl 0x82466e20
	ctx.lr = 0x82705088;
	sub_82466E20(ctx, base);
	// 82705088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270508C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705098 size=92
    let mut pc: u32 = 0x82705098;
    'dispatch: loop {
        match pc {
            0x82705098 => {
    //   block [0x82705098..0x827050F4)
	// 82705098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270509C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827050A0: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827050A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827050A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827050AC: 4BD961E5  bl 0x8249b290
	ctx.lr = 0x827050B0;
	sub_8249B290(ctx, base);
	// 827050B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827050B4: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 827050B8: 394BC6C4  addi r10, r11, -0x393c
	ctx.r[10].s64 = ctx.r[11].s64 + -14652;
	// 827050BC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827050C0: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 827050C4: 396B0334  addi r11, r11, 0x334
	ctx.r[11].s64 = ctx.r[11].s64 + 820;
	// 827050C8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827050CC: 394817E8  addi r10, r8, 0x17e8
	ctx.r[10].s64 = ctx.r[8].s64 + 6120;
	// 827050D0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827050D4: 39491800  addi r10, r9, 0x1800
	ctx.r[10].s64 = ctx.r[9].s64 + 6144;
	// 827050D8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827050DC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827050E0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827050E4: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 827050E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827050EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827050F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827050F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827050F8 size=48
    let mut pc: u32 = 0x827050F8;
    'dispatch: loop {
        match pc {
            0x827050F8 => {
    //   block [0x827050F8..0x82705128)
	// 827050F8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827050FC: 814BA55C  lwz r10, -0x5aa4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23204 as u32) ) } as u64;
	// 82705100: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82705104: 396BA560  addi r11, r11, -0x5aa0
	ctx.r[11].s64 = ctx.r[11].s64 + -23200;
	// 82705108: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8270510C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82705110: 814AA558  lwz r10, -0x5aa8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-23208 as u32) ) } as u64;
	// 82705114: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 82705118: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8270511C: 814AA554  lwz r10, -0x5aac(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-23212 as u32) ) } as u64;
	// 82705120: 914B0380  stw r10, 0x380(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(896 as u32), ctx.r[10].u32 ) };
	// 82705124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705128 size=116
    let mut pc: u32 = 0x82705128;
    'dispatch: loop {
        match pc {
            0x82705128 => {
    //   block [0x82705128..0x8270519C)
	// 82705128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270512C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705134: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705138: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270513C: 392BE9D8  addi r9, r11, -0x1628
	ctx.r[9].s64 = ctx.r[11].s64 + -5672;
	// 82705140: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82705144: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705148: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8270514C: 38C0002A  li r6, 0x2a
	ctx.r[6].s64 = 42;
	// 82705150: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82705154: 388AC6C4  addi r4, r10, -0x393c
	ctx.r[4].s64 = ctx.r[10].s64 + -14652;
	// 82705158: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270515C: 396BA560  addi r11, r11, -0x5aa0
	ctx.r[11].s64 = ctx.r[11].s64 + -23200;
	// 82705160: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82705164: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705168: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8270516C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705170: 386A0344  addi r3, r10, 0x344
	ctx.r[3].s64 = ctx.r[10].s64 + 836;
	// 82705174: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82705178: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8270517C: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 82705180: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82705184: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82705188: 4BD61C99  bl 0x82466e20
	ctx.lr = 0x8270518C;
	sub_82466E20(ctx, base);
	// 8270518C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827051A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827051A0 size=108
    let mut pc: u32 = 0x827051A0;
    'dispatch: loop {
        match pc {
            0x827051A0 => {
    //   block [0x827051A0..0x8270520C)
	// 827051A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827051A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827051A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827051AC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827051B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827051B4: 392BEB48  addi r9, r11, -0x14b8
	ctx.r[9].s64 = ctx.r[11].s64 + -5304;
	// 827051B8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 827051BC: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 827051C0: 388ABF10  addi r4, r10, -0x40f0
	ctx.r[4].s64 = ctx.r[10].s64 + -16624;
	// 827051C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827051C8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827051CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827051D0: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 827051D4: 386A0374  addi r3, r10, 0x374
	ctx.r[3].s64 = ctx.r[10].s64 + 884;
	// 827051D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827051DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827051E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827051E4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827051E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827051EC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827051F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827051F4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827051F8: 4BD61C29  bl 0x82466e20
	ctx.lr = 0x827051FC;
	sub_82466E20(ctx, base);
	// 827051FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705210 size=112
    let mut pc: u32 = 0x82705210;
    'dispatch: loop {
        match pc {
            0x82705210 => {
    //   block [0x82705210..0x82705280)
	// 82705210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270521C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705220: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705224: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82705228: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270522C: 390BEBA8  addi r8, r11, -0x1458
	ctx.r[8].s64 = ctx.r[11].s64 + -5208;
	// 82705230: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82705234: 388ABF2C  addi r4, r10, -0x40d4
	ctx.r[4].s64 = ctx.r[10].s64 + -16596;
	// 82705238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270523C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705240: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705248: 386A03A4  addi r3, r10, 0x3a4
	ctx.r[3].s64 = ctx.r[10].s64 + 932;
	// 8270524C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270525C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705264: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 82705268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270526C: 4BD61BB5  bl 0x82466e20
	ctx.lr = 0x82705270;
	sub_82466E20(ctx, base);
	// 82705270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270527C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705280 size=108
    let mut pc: u32 = 0x82705280;
    'dispatch: loop {
        match pc {
            0x82705280 => {
    //   block [0x82705280..0x827052EC)
	// 82705280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270528C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705290: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705294: 38EBEBF8  addi r7, r11, -0x1408
	ctx.r[7].s64 = ctx.r[11].s64 + -5128;
	// 82705298: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8270529C: 388AC22C  addi r4, r10, -0x3dd4
	ctx.r[4].s64 = ctx.r[10].s64 + -15828;
	// 827052A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827052A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827052A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827052AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827052B0: 386A03D4  addi r3, r10, 0x3d4
	ctx.r[3].s64 = ctx.r[10].s64 + 980;
	// 827052B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827052B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827052BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827052C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827052C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827052C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827052CC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 827052D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827052D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827052D8: 4BD61B49  bl 0x82466e20
	ctx.lr = 0x827052DC;
	sub_82466E20(ctx, base);
	// 827052DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827052E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827052E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827052E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827052F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827052F0 size=92
    let mut pc: u32 = 0x827052F0;
    'dispatch: loop {
        match pc {
            0x827052F0 => {
    //   block [0x827052F0..0x8270534C)
	// 827052F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827052F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827052F8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827052FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82705300: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82705304: 4BDA64A5  bl 0x824ab7a8
	ctx.lr = 0x82705308;
	sub_824AB7A8(ctx, base);
	// 82705308: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8270530C: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 82705310: 394BC250  addi r10, r11, -0x3db0
	ctx.r[10].s64 = ctx.r[11].s64 + -15792;
	// 82705314: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82705318: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 8270531C: 396B0404  addi r11, r11, 0x404
	ctx.r[11].s64 = ctx.r[11].s64 + 1028;
	// 82705320: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82705324: 394818A8  addi r10, r8, 0x18a8
	ctx.r[10].s64 = ctx.r[8].s64 + 6312;
	// 82705328: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270532C: 394918C0  addi r10, r9, 0x18c0
	ctx.r[10].s64 = ctx.r[9].s64 + 6336;
	// 82705330: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82705334: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82705338: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8270533C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82705340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705350 size=112
    let mut pc: u32 = 0x82705350;
    'dispatch: loop {
        match pc {
            0x82705350 => {
    //   block [0x82705350..0x827053C0)
	// 82705350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270535C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705360: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705364: 396BEC88  addi r11, r11, -0x1378
	ctx.r[11].s64 = ctx.r[11].s64 + -4984;
	// 82705368: 38AA0004  addi r5, r10, 4
	ctx.r[5].s64 = ctx.r[10].s64 + 4;
	// 8270536C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705370: 390B00D8  addi r8, r11, 0xd8
	ctx.r[8].s64 = ctx.r[11].s64 + 216;
	// 82705374: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82705378: 388AC250  addi r4, r10, -0x3db0
	ctx.r[4].s64 = ctx.r[10].s64 + -15792;
	// 8270537C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82705380: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705384: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705388: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8270538C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82705390: 386A0414  addi r3, r10, 0x414
	ctx.r[3].s64 = ctx.r[10].s64 + 1044;
	// 82705394: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82705398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270539C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827053A0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 827053A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827053A8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 827053AC: 4BD61A75  bl 0x82466e20
	ctx.lr = 0x827053B0;
	sub_82466E20(ctx, base);
	// 827053B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827053B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827053B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827053BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827053C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827053C0 size=108
    let mut pc: u32 = 0x827053C0;
    'dispatch: loop {
        match pc {
            0x827053C0 => {
    //   block [0x827053C0..0x8270542C)
	// 827053C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827053C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827053C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827053CC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827053D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827053D4: 38EBED94  addi r7, r11, -0x126c
	ctx.r[7].s64 = ctx.r[11].s64 + -4716;
	// 827053D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827053DC: 388AC664  addi r4, r10, -0x399c
	ctx.r[4].s64 = ctx.r[10].s64 + -14748;
	// 827053E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827053E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827053E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827053EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827053F0: 386A0444  addi r3, r10, 0x444
	ctx.r[3].s64 = ctx.r[10].s64 + 1092;
	// 827053F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827053F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827053FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270540C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82705410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705414: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82705418: 4BD61A09  bl 0x82466e20
	ctx.lr = 0x8270541C;
	sub_82466E20(ctx, base);
	// 8270541C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705430 size=92
    let mut pc: u32 = 0x82705430;
    'dispatch: loop {
        match pc {
            0x82705430 => {
    //   block [0x82705430..0x8270548C)
	// 82705430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705438: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270543C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82705440: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82705444: 4BD8C52D  bl 0x82491970
	ctx.lr = 0x82705448;
	sub_82491970(ctx, base);
	// 82705448: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8270544C: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 82705450: 394BC68C  addi r10, r11, -0x3974
	ctx.r[10].s64 = ctx.r[11].s64 + -14708;
	// 82705454: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82705458: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 8270545C: 396B0474  addi r11, r11, 0x474
	ctx.r[11].s64 = ctx.r[11].s64 + 1140;
	// 82705460: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82705464: 39481928  addi r10, r8, 0x1928
	ctx.r[10].s64 = ctx.r[8].s64 + 6440;
	// 82705468: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270546C: 39491910  addi r10, r9, 0x1910
	ctx.r[10].s64 = ctx.r[9].s64 + 6416;
	// 82705470: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82705474: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82705478: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8270547C: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 82705480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705490 size=112
    let mut pc: u32 = 0x82705490;
    'dispatch: loop {
        match pc {
            0x82705490 => {
    //   block [0x82705490..0x82705500)
	// 82705490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270549C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827054A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827054A4: 38AA0E18  addi r5, r10, 0xe18
	ctx.r[5].s64 = ctx.r[10].s64 + 3608;
	// 827054A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827054AC: 390BEDAC  addi r8, r11, -0x1254
	ctx.r[8].s64 = ctx.r[11].s64 + -4692;
	// 827054B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827054B4: 388AC68C  addi r4, r10, -0x3974
	ctx.r[4].s64 = ctx.r[10].s64 + -14708;
	// 827054B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827054BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827054C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827054C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827054C8: 386A0484  addi r3, r10, 0x484
	ctx.r[3].s64 = ctx.r[10].s64 + 1156;
	// 827054CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827054D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827054D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827054D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827054DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827054E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827054E4: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 827054E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827054EC: 4BD61935  bl 0x82466e20
	ctx.lr = 0x827054F0;
	sub_82466E20(ctx, base);
	// 827054F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827054F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827054F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827054FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705500 size=100
    let mut pc: u32 = 0x82705500;
    'dispatch: loop {
        match pc {
            0x82705500 => {
    //   block [0x82705500..0x82705564)
	// 82705500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270550C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705514: 38AA0BD8  addi r5, r10, 0xbd8
	ctx.r[5].s64 = ctx.r[10].s64 + 3032;
	// 82705518: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270551C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705520: 388AC564  addi r4, r10, -0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15004;
	// 82705524: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270552C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705534: 386A04B4  addi r3, r10, 0x4b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1204;
	// 82705538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270553C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705540: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82705544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705548: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270554C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 82705550: 4BD618D1  bl 0x82466e20
	ctx.lr = 0x82705554;
	sub_82466E20(ctx, base);
	// 82705554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270555C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705568 size=100
    let mut pc: u32 = 0x82705568;
    'dispatch: loop {
        match pc {
            0x82705568 => {
    //   block [0x82705568..0x827055CC)
	// 82705568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270556C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705574: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270557C: 38AA0BD8  addi r5, r10, 0xbd8
	ctx.r[5].s64 = ctx.r[10].s64 + 3032;
	// 82705580: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705588: 388AC5C8  addi r4, r10, -0x3a38
	ctx.r[4].s64 = ctx.r[10].s64 + -14904;
	// 8270558C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270559C: 386A04E4  addi r3, r10, 0x4e4
	ctx.r[3].s64 = ctx.r[10].s64 + 1252;
	// 827055A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827055A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827055A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827055AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827055B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827055B4: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 827055B8: 4BD61869  bl 0x82466e20
	ctx.lr = 0x827055BC;
	sub_82466E20(ctx, base);
	// 827055BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827055C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827055C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827055C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827055D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827055D0 size=24
    let mut pc: u32 = 0x827055D0;
    'dispatch: loop {
        match pc {
            0x827055D0 => {
    //   block [0x827055D0..0x827055E8)
	// 827055D0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827055D4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827055D8: 394AAA04  addi r10, r10, -0x55fc
	ctx.r[10].s64 = ctx.r[10].s64 + -22012;
	// 827055DC: 816BAA00  lwz r11, -0x5600(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22016 as u32) ) } as u64;
	// 827055E0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827055E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827055E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827055E8 size=116
    let mut pc: u32 = 0x827055E8;
    'dispatch: loop {
        match pc {
            0x827055E8 => {
    //   block [0x827055E8..0x8270565C)
	// 827055E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827055EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827055F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827055F4: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827055F8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827055FC: 390BAA04  addi r8, r11, -0x55fc
	ctx.r[8].s64 = ctx.r[11].s64 + -22012;
	// 82705600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705604: 392AEE3C  addi r9, r10, -0x11c4
	ctx.r[9].s64 = ctx.r[10].s64 + -4548;
	// 82705608: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270560C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82705610: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82705614: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270561C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270562C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82705630: 388AC360  addi r4, r10, -0x3ca0
	ctx.r[4].s64 = ctx.r[10].s64 + -15520;
	// 82705634: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82705638: 386B0514  addi r3, r11, 0x514
	ctx.r[3].s64 = ctx.r[11].s64 + 1300;
	// 8270563C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82705640: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705644: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82705648: 4BD617D9  bl 0x82466e20
	ctx.lr = 0x8270564C;
	sub_82466E20(ctx, base);
	// 8270564C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705660 size=108
    let mut pc: u32 = 0x82705660;
    'dispatch: loop {
        match pc {
            0x82705660 => {
    //   block [0x82705660..0x827056CC)
	// 82705660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270566C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705670: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705674: 38EBEE50  addi r7, r11, -0x11b0
	ctx.r[7].s64 = ctx.r[11].s64 + -4528;
	// 82705678: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8270567C: 388AC1C0  addi r4, r10, -0x3e40
	ctx.r[4].s64 = ctx.r[10].s64 + -15936;
	// 82705680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705684: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705688: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270568C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705690: 386A0544  addi r3, r10, 0x544
	ctx.r[3].s64 = ctx.r[10].s64 + 1348;
	// 82705694: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82705698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270569C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827056A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827056A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827056A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827056AC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 827056B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827056B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827056B8: 4BD61769  bl 0x82466e20
	ctx.lr = 0x827056BC;
	sub_82466E20(ctx, base);
	// 827056BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827056C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827056C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827056C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827056D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827056D0 size=92
    let mut pc: u32 = 0x827056D0;
    'dispatch: loop {
        match pc {
            0x827056D0 => {
    //   block [0x827056D0..0x8270572C)
	// 827056D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827056D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827056D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827056DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827056E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827056E4: 4BDA7595  bl 0x824acc78
	ctx.lr = 0x827056E8;
	sub_824ACC78(ctx, base);
	// 827056E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827056EC: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 827056F0: 394BC1E8  addi r10, r11, -0x3e18
	ctx.r[10].s64 = ctx.r[11].s64 + -15896;
	// 827056F4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827056F8: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 827056FC: 396B0574  addi r11, r11, 0x574
	ctx.r[11].s64 = ctx.r[11].s64 + 1396;
	// 82705700: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82705704: 39481AD8  addi r10, r8, 0x1ad8
	ctx.r[10].s64 = ctx.r[8].s64 + 6872;
	// 82705708: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270570C: 39491AF0  addi r10, r9, 0x1af0
	ctx.r[10].s64 = ctx.r[9].s64 + 6896;
	// 82705710: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82705714: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82705718: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8270571C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82705720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705730 size=112
    let mut pc: u32 = 0x82705730;
    'dispatch: loop {
        match pc {
            0x82705730 => {
    //   block [0x82705730..0x827057A0)
	// 82705730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270573C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705740: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705744: 38AA0004  addi r5, r10, 4
	ctx.r[5].s64 = ctx.r[10].s64 + 4;
	// 82705748: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270574C: 390BEE80  addi r8, r11, -0x1180
	ctx.r[8].s64 = ctx.r[11].s64 + -4480;
	// 82705750: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82705754: 388AC1E8  addi r4, r10, -0x3e18
	ctx.r[4].s64 = ctx.r[10].s64 + -15896;
	// 82705758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270575C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705768: 386A0584  addi r3, r10, 0x584
	ctx.r[3].s64 = ctx.r[10].s64 + 1412;
	// 8270576C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270577C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705784: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82705788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270578C: 4BD61695  bl 0x82466e20
	ctx.lr = 0x82705790;
	sub_82466E20(ctx, base);
	// 82705790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270579C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827057A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827057A0 size=108
    let mut pc: u32 = 0x827057A0;
    'dispatch: loop {
        match pc {
            0x827057A0 => {
    //   block [0x827057A0..0x8270580C)
	// 827057A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827057A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827057A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827057AC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827057B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827057B4: 38EBEF10  addi r7, r11, -0x10f0
	ctx.r[7].s64 = ctx.r[11].s64 + -4336;
	// 827057B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 827057BC: 388AC298  addi r4, r10, -0x3d68
	ctx.r[4].s64 = ctx.r[10].s64 + -15720;
	// 827057C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827057C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827057C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827057CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827057D0: 386A05B4  addi r3, r10, 0x5b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1460;
	// 827057D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827057D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827057DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827057E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827057E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827057E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827057EC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 827057F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827057F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827057F8: 4BD61629  bl 0x82466e20
	ctx.lr = 0x827057FC;
	sub_82466E20(ctx, base);
	// 827057FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705810 size=92
    let mut pc: u32 = 0x82705810;
    'dispatch: loop {
        match pc {
            0x82705810 => {
    //   block [0x82705810..0x8270586C)
	// 82705810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705818: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270581C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82705820: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82705824: 4BDA79E5  bl 0x824ad208
	ctx.lr = 0x82705828;
	sub_824AD208(ctx, base);
	// 82705828: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8270582C: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 82705830: 394BC2C0  addi r10, r11, -0x3d40
	ctx.r[10].s64 = ctx.r[11].s64 + -15680;
	// 82705834: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82705838: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 8270583C: 396B05E4  addi r11, r11, 0x5e4
	ctx.r[11].s64 = ctx.r[11].s64 + 1508;
	// 82705840: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82705844: 39481B40  addi r10, r8, 0x1b40
	ctx.r[10].s64 = ctx.r[8].s64 + 6976;
	// 82705848: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270584C: 39491B58  addi r10, r9, 0x1b58
	ctx.r[10].s64 = ctx.r[9].s64 + 7000;
	// 82705850: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82705854: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82705858: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8270585C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82705860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705870 size=112
    let mut pc: u32 = 0x82705870;
    'dispatch: loop {
        match pc {
            0x82705870 => {
    //   block [0x82705870..0x827058E0)
	// 82705870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270587C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705880: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705884: 38AA0004  addi r5, r10, 4
	ctx.r[5].s64 = ctx.r[10].s64 + 4;
	// 82705888: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270588C: 390BEF58  addi r8, r11, -0x10a8
	ctx.r[8].s64 = ctx.r[11].s64 + -4264;
	// 82705890: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82705894: 388AC2C0  addi r4, r10, -0x3d40
	ctx.r[4].s64 = ctx.r[10].s64 + -15680;
	// 82705898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270589C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827058A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827058A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827058A8: 386A05F4  addi r3, r10, 0x5f4
	ctx.r[3].s64 = ctx.r[10].s64 + 1524;
	// 827058AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827058B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827058B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827058B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827058BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827058C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827058C4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 827058C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827058CC: 4BD61555  bl 0x82466e20
	ctx.lr = 0x827058D0;
	sub_82466E20(ctx, base);
	// 827058D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827058D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827058D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827058DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827058E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827058E0 size=24
    let mut pc: u32 = 0x827058E0;
    'dispatch: loop {
        match pc {
            0x827058E0 => {
    //   block [0x827058E0..0x827058F8)
	// 827058E0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827058E4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827058E8: 394AAA60  addi r10, r10, -0x55a0
	ctx.r[10].s64 = ctx.r[10].s64 + -21920;
	// 827058EC: 816BAA40  lwz r11, -0x55c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21952 as u32) ) } as u64;
	// 827058F0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 827058F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827058F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827058F8 size=116
    let mut pc: u32 = 0x827058F8;
    'dispatch: loop {
        match pc {
            0x827058F8 => {
    //   block [0x827058F8..0x8270596C)
	// 827058F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827058FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705904: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82705908: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8270590C: 390BAA60  addi r8, r11, -0x55a0
	ctx.r[8].s64 = ctx.r[11].s64 + -21920;
	// 82705910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705914: 392AF028  addi r9, r10, -0xfd8
	ctx.r[9].s64 = ctx.r[10].s64 + -4056;
	// 82705918: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270591C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82705920: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82705924: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270592C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270593C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82705940: 388ABEB4  addi r4, r10, -0x414c
	ctx.r[4].s64 = ctx.r[10].s64 + -16716;
	// 82705944: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82705948: 386B0624  addi r3, r11, 0x624
	ctx.r[3].s64 = ctx.r[11].s64 + 1572;
	// 8270594C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82705950: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705954: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82705958: 4BD614C9  bl 0x82466e20
	ctx.lr = 0x8270595C;
	sub_82466E20(ctx, base);
	// 8270595C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705970 size=100
    let mut pc: u32 = 0x82705970;
    'dispatch: loop {
        match pc {
            0x82705970 => {
    //   block [0x82705970..0x827059D4)
	// 82705970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270597C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705984: 38AA04B4  addi r5, r10, 0x4b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1204;
	// 82705988: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270598C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705990: 388AC60C  addi r4, r10, -0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + -14836;
	// 82705994: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270599C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827059A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827059A4: 386A0654  addi r3, r10, 0x654
	ctx.r[3].s64 = ctx.r[10].s64 + 1620;
	// 827059A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827059AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827059B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827059B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827059B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827059BC: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 827059C0: 4BD61461  bl 0x82466e20
	ctx.lr = 0x827059C4;
	sub_82466E20(ctx, base);
	// 827059C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827059C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827059CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827059D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827059D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827059D8 size=112
    let mut pc: u32 = 0x827059D8;
    'dispatch: loop {
        match pc {
            0x827059D8 => {
    //   block [0x827059D8..0x82705A48)
	// 827059D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827059DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827059E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827059E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827059E8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827059EC: 38AA0C78  addi r5, r10, 0xc78
	ctx.r[5].s64 = ctx.r[10].s64 + 3192;
	// 827059F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827059F4: 390BF098  addi r8, r11, -0xf68
	ctx.r[8].s64 = ctx.r[11].s64 + -3944;
	// 827059F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 827059FC: 388AC3EC  addi r4, r10, -0x3c14
	ctx.r[4].s64 = ctx.r[10].s64 + -15380;
	// 82705A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705A04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705A08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705A10: 386A0684  addi r3, r10, 0x684
	ctx.r[3].s64 = ctx.r[10].s64 + 1668;
	// 82705A14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705A2C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82705A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705A34: 4BD613ED  bl 0x82466e20
	ctx.lr = 0x82705A38;
	sub_82466E20(ctx, base);
	// 82705A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705A48 size=112
    let mut pc: u32 = 0x82705A48;
    'dispatch: loop {
        match pc {
            0x82705A48 => {
    //   block [0x82705A48..0x82705AB8)
	// 82705A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705A54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705A58: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705A5C: 38AA0C48  addi r5, r10, 0xc48
	ctx.r[5].s64 = ctx.r[10].s64 + 3144;
	// 82705A60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705A64: 390BF0F4  addi r8, r11, -0xf0c
	ctx.r[8].s64 = ctx.r[11].s64 + -3852;
	// 82705A68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82705A6C: 388ABE64  addi r4, r10, -0x419c
	ctx.r[4].s64 = ctx.r[10].s64 + -16796;
	// 82705A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705A74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705A78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705A80: 386A06B4  addi r3, r10, 0x6b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1716;
	// 82705A84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705A9C: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 82705AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705AA4: 4BD6137D  bl 0x82466e20
	ctx.lr = 0x82705AA8;
	sub_82466E20(ctx, base);
	// 82705AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705AB8 size=92
    let mut pc: u32 = 0x82705AB8;
    'dispatch: loop {
        match pc {
            0x82705AB8 => {
    //   block [0x82705AB8..0x82705B14)
	// 82705AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705AC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82705AC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82705ACC: 4BDA7DFD  bl 0x824ad8c8
	ctx.lr = 0x82705AD0;
	sub_824AD8C8(ctx, base);
	// 82705AD0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82705AD4: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 82705AD8: 394BC344  addi r10, r11, -0x3cbc
	ctx.r[10].s64 = ctx.r[11].s64 + -15548;
	// 82705ADC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82705AE0: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 82705AE4: 396B06E4  addi r11, r11, 0x6e4
	ctx.r[11].s64 = ctx.r[11].s64 + 1764;
	// 82705AE8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82705AEC: 39481CF8  addi r10, r8, 0x1cf8
	ctx.r[10].s64 = ctx.r[8].s64 + 7416;
	// 82705AF0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82705AF4: 39491CE0  addi r10, r9, 0x1ce0
	ctx.r[10].s64 = ctx.r[9].s64 + 7392;
	// 82705AF8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82705AFC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82705B00: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82705B04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705B18 size=112
    let mut pc: u32 = 0x82705B18;
    'dispatch: loop {
        match pc {
            0x82705B18 => {
    //   block [0x82705B18..0x82705B88)
	// 82705B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705B24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705B28: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705B2C: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82705B30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705B34: 390BF110  addi r8, r11, -0xef0
	ctx.r[8].s64 = ctx.r[11].s64 + -3824;
	// 82705B38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82705B3C: 388AC344  addi r4, r10, -0x3cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -15548;
	// 82705B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705B44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705B50: 386A06F4  addi r3, r10, 0x6f4
	ctx.r[3].s64 = ctx.r[10].s64 + 1780;
	// 82705B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705B6C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82705B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705B74: 4BD612AD  bl 0x82466e20
	ctx.lr = 0x82705B78;
	sub_82466E20(ctx, base);
	// 82705B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705B88 size=108
    let mut pc: u32 = 0x82705B88;
    'dispatch: loop {
        match pc {
            0x82705B88 => {
    //   block [0x82705B88..0x82705BF4)
	// 82705B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705B94: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705B98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705B9C: 38EBF158  addi r7, r11, -0xea8
	ctx.r[7].s64 = ctx.r[11].s64 + -3752;
	// 82705BA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82705BA4: 388AC10C  addi r4, r10, -0x3ef4
	ctx.r[4].s64 = ctx.r[10].s64 + -16116;
	// 82705BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705BAC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705BB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82705BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705BB8: 386A0724  addi r3, r10, 0x724
	ctx.r[3].s64 = ctx.r[10].s64 + 1828;
	// 82705BBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82705BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705BD4: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 82705BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705BDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82705BE0: 4BD61241  bl 0x82466e20
	ctx.lr = 0x82705BE4;
	sub_82466E20(ctx, base);
	// 82705BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705BF8 size=112
    let mut pc: u32 = 0x82705BF8;
    'dispatch: loop {
        match pc {
            0x82705BF8 => {
    //   block [0x82705BF8..0x82705C68)
	// 82705BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705C04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705C08: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705C0C: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82705C10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705C14: 390BF188  addi r8, r11, -0xe78
	ctx.r[8].s64 = ctx.r[11].s64 + -3704;
	// 82705C18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82705C1C: 388AC130  addi r4, r10, -0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -16080;
	// 82705C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705C24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705C30: 386A0754  addi r3, r10, 0x754
	ctx.r[3].s64 = ctx.r[10].s64 + 1876;
	// 82705C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705C4C: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 82705C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705C54: 4BD611CD  bl 0x82466e20
	ctx.lr = 0x82705C58;
	sub_82466E20(ctx, base);
	// 82705C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82705C68 size=24
    let mut pc: u32 = 0x82705C68;
    'dispatch: loop {
        match pc {
            0x82705C68 => {
    //   block [0x82705C68..0x82705C80)
	// 82705C68: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82705C6C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82705C70: 394AAC10  addi r10, r10, -0x53f0
	ctx.r[10].s64 = ctx.r[10].s64 + -21488;
	// 82705C74: 816BABF8  lwz r11, -0x5408(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21512 as u32) ) } as u64;
	// 82705C78: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82705C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705C80 size=116
    let mut pc: u32 = 0x82705C80;
    'dispatch: loop {
        match pc {
            0x82705C80 => {
    //   block [0x82705C80..0x82705CF4)
	// 82705C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705C8C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82705C90: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82705C94: 390BAC10  addi r8, r11, -0x53f0
	ctx.r[8].s64 = ctx.r[11].s64 + -21488;
	// 82705C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705C9C: 392AF1FC  addi r9, r10, -0xe04
	ctx.r[9].s64 = ctx.r[10].s64 + -3588;
	// 82705CA0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705CA4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82705CA8: 38AA0C78  addi r5, r10, 0xc78
	ctx.r[5].s64 = ctx.r[10].s64 + 3192;
	// 82705CAC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705CB4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705CC4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82705CC8: 388AC394  addi r4, r10, -0x3c6c
	ctx.r[4].s64 = ctx.r[10].s64 + -15468;
	// 82705CCC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82705CD0: 386B0784  addi r3, r11, 0x784
	ctx.r[3].s64 = ctx.r[11].s64 + 1924;
	// 82705CD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82705CD8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705CDC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 82705CE0: 4BD61141  bl 0x82466e20
	ctx.lr = 0x82705CE4;
	sub_82466E20(ctx, base);
	// 82705CE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705CE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705CEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705CF8 size=116
    let mut pc: u32 = 0x82705CF8;
    'dispatch: loop {
        match pc {
            0x82705CF8 => {
    //   block [0x82705CF8..0x82705D6C)
	// 82705CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705D04: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82705D08: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82705D0C: 390AF228  addi r8, r10, -0xdd8
	ctx.r[8].s64 = ctx.r[10].s64 + -3544;
	// 82705D10: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82705D14: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705D18: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82705D1C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705D20: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82705D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705D28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705D2C: 388AC6A4  addi r4, r10, -0x395c
	ctx.r[4].s64 = ctx.r[10].s64 + -14684;
	// 82705D30: 396BF2D0  addi r11, r11, -0xd30
	ctx.r[11].s64 = ctx.r[11].s64 + -3376;
	// 82705D34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705D38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705D3C: 386A07B4  addi r3, r10, 0x7b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1972;
	// 82705D40: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82705D44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705D48: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82705D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705D50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705D54: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 82705D58: 4BD610C9  bl 0x82466e20
	ctx.lr = 0x82705D5C;
	sub_82466E20(ctx, base);
	// 82705D5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705D70 size=112
    let mut pc: u32 = 0x82705D70;
    'dispatch: loop {
        match pc {
            0x82705D70 => {
    //   block [0x82705D70..0x82705DE0)
	// 82705D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705D7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705D80: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705D84: 38AA0C48  addi r5, r10, 0xc48
	ctx.r[5].s64 = ctx.r[10].s64 + 3144;
	// 82705D88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705D8C: 390BF308  addi r8, r11, -0xcf8
	ctx.r[8].s64 = ctx.r[11].s64 + -3320;
	// 82705D90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82705D94: 388ABE54  addi r4, r10, -0x41ac
	ctx.r[4].s64 = ctx.r[10].s64 + -16812;
	// 82705D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705D9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705DA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705DA8: 386A07E4  addi r3, r10, 0x7e4
	ctx.r[3].s64 = ctx.r[10].s64 + 2020;
	// 82705DAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705DC4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82705DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705DCC: 4BD61055  bl 0x82466e20
	ctx.lr = 0x82705DD0;
	sub_82466E20(ctx, base);
	// 82705DD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705DE0 size=108
    let mut pc: u32 = 0x82705DE0;
    'dispatch: loop {
        match pc {
            0x82705DE0 => {
    //   block [0x82705DE0..0x82705E4C)
	// 82705DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705DEC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705DF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705DF4: 38EBF338  addi r7, r11, -0xcc8
	ctx.r[7].s64 = ctx.r[11].s64 + -3272;
	// 82705DF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82705DFC: 388AC094  addi r4, r10, -0x3f6c
	ctx.r[4].s64 = ctx.r[10].s64 + -16236;
	// 82705E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705E04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705E08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82705E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705E10: 386A0814  addi r3, r10, 0x814
	ctx.r[3].s64 = ctx.r[10].s64 + 2068;
	// 82705E14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82705E18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705E2C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82705E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705E34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82705E38: 4BD60FE9  bl 0x82466e20
	ctx.lr = 0x82705E3C;
	sub_82466E20(ctx, base);
	// 82705E3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705E50 size=112
    let mut pc: u32 = 0x82705E50;
    'dispatch: loop {
        match pc {
            0x82705E50 => {
    //   block [0x82705E50..0x82705EC0)
	// 82705E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705E5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705E60: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705E64: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82705E68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705E6C: 390BF368  addi r8, r11, -0xc98
	ctx.r[8].s64 = ctx.r[11].s64 + -3224;
	// 82705E70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82705E74: 388AC0B8  addi r4, r10, -0x3f48
	ctx.r[4].s64 = ctx.r[10].s64 + -16200;
	// 82705E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705E7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705E80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705E88: 386A0844  addi r3, r10, 0x844
	ctx.r[3].s64 = ctx.r[10].s64 + 2116;
	// 82705E8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705E90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705E94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705E98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705EA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705EA4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82705EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705EAC: 4BD60F75  bl 0x82466e20
	ctx.lr = 0x82705EB0;
	sub_82466E20(ctx, base);
	// 82705EB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705EC0 size=100
    let mut pc: u32 = 0x82705EC0;
    'dispatch: loop {
        match pc {
            0x82705EC0 => {
    //   block [0x82705EC0..0x82705F24)
	// 82705EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705ECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82705ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705ED4: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82705ED8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705EE0: 388ABFA4  addi r4, r10, -0x405c
	ctx.r[4].s64 = ctx.r[10].s64 + -16476;
	// 82705EE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705EF4: 386A0874  addi r3, r10, 0x874
	ctx.r[3].s64 = ctx.r[10].s64 + 2164;
	// 82705EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705EFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705F00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82705F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705F08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82705F0C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82705F10: 4BD60F11  bl 0x82466e20
	ctx.lr = 0x82705F14;
	sub_82466E20(ctx, base);
	// 82705F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705F28 size=100
    let mut pc: u32 = 0x82705F28;
    'dispatch: loop {
        match pc {
            0x82705F28 => {
    //   block [0x82705F28..0x82705F8C)
	// 82705F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705F34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705F3C: 38AA0E48  addi r5, r10, 0xe48
	ctx.r[5].s64 = ctx.r[10].s64 + 3656;
	// 82705F40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705F48: 388AC4CC  addi r4, r10, -0x3b34
	ctx.r[4].s64 = ctx.r[10].s64 + -15156;
	// 82705F4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705F5C: 386A08A4  addi r3, r10, 0x8a4
	ctx.r[3].s64 = ctx.r[10].s64 + 2212;
	// 82705F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705F64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705F68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82705F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705F70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82705F74: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82705F78: 4BD60EA9  bl 0x82466e20
	ctx.lr = 0x82705F7C;
	sub_82466E20(ctx, base);
	// 82705F7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705F90 size=108
    let mut pc: u32 = 0x82705F90;
    'dispatch: loop {
        match pc {
            0x82705F90 => {
    //   block [0x82705F90..0x82705FFC)
	// 82705F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705F9C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705FA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705FA4: 38EBF3D8  addi r7, r11, -0xc28
	ctx.r[7].s64 = ctx.r[11].s64 + -3112;
	// 82705FA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82705FAC: 388AC510  addi r4, r10, -0x3af0
	ctx.r[4].s64 = ctx.r[10].s64 + -15088;
	// 82705FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705FB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705FB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82705FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705FC0: 386A08D4  addi r3, r10, 0x8d4
	ctx.r[3].s64 = ctx.r[10].s64 + 2260;
	// 82705FC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82705FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705FDC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82705FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705FE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82705FE8: 4BD60E39  bl 0x82466e20
	ctx.lr = 0x82705FEC;
	sub_82466E20(ctx, base);
	// 82705FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706000 size=112
    let mut pc: u32 = 0x82706000;
    'dispatch: loop {
        match pc {
            0x82706000 => {
    //   block [0x82706000..0x82706070)
	// 82706000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270600C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706010: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706014: 38AA0E48  addi r5, r10, 0xe48
	ctx.r[5].s64 = ctx.r[10].s64 + 3656;
	// 82706018: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270601C: 390BF408  addi r8, r11, -0xbf8
	ctx.r[8].s64 = ctx.r[11].s64 + -3064;
	// 82706020: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82706024: 388AC538  addi r4, r10, -0x3ac8
	ctx.r[4].s64 = ctx.r[10].s64 + -15048;
	// 82706028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270602C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706038: 386A0904  addi r3, r10, 0x904
	ctx.r[3].s64 = ctx.r[10].s64 + 2308;
	// 8270603C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270604C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706054: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 82706058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270605C: 4BD60DC5  bl 0x82466e20
	ctx.lr = 0x82706060;
	sub_82466E20(ctx, base);
	// 82706060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270606C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706070 size=108
    let mut pc: u32 = 0x82706070;
    'dispatch: loop {
        match pc {
            0x82706070 => {
    //   block [0x82706070..0x827060DC)
	// 82706070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270607C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706084: 392BF4E8  addi r9, r11, -0xb18
	ctx.r[9].s64 = ctx.r[11].s64 + -2840;
	// 82706088: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8270608C: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 82706090: 388ABF44  addi r4, r10, -0x40bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16572;
	// 82706094: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706098: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270609C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827060A0: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 827060A4: 386A0934  addi r3, r10, 0x934
	ctx.r[3].s64 = ctx.r[10].s64 + 2356;
	// 827060A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827060AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827060B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827060B4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827060B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827060BC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827060C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827060C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827060C8: 4BD60D59  bl 0x82466e20
	ctx.lr = 0x827060CC;
	sub_82466E20(ctx, base);
	// 827060CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827060D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827060D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827060D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827060E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827060E0 size=112
    let mut pc: u32 = 0x827060E0;
    'dispatch: loop {
        match pc {
            0x827060E0 => {
    //   block [0x827060E0..0x82706150)
	// 827060E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827060E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827060E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827060EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827060F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827060F4: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 827060F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827060FC: 390BF590  addi r8, r11, -0xa70
	ctx.r[8].s64 = ctx.r[11].s64 + -2672;
	// 82706100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82706104: 388ABF68  addi r4, r10, -0x4098
	ctx.r[4].s64 = ctx.r[10].s64 + -16536;
	// 82706108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270610C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706118: 386A0964  addi r3, r10, 0x964
	ctx.r[3].s64 = ctx.r[10].s64 + 2404;
	// 8270611C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270612C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706134: 38C000E0  li r6, 0xe0
	ctx.r[6].s64 = 224;
	// 82706138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270613C: 4BD60CE5  bl 0x82466e20
	ctx.lr = 0x82706140;
	sub_82466E20(ctx, base);
	// 82706140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270614C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706150 size=108
    let mut pc: u32 = 0x82706150;
    'dispatch: loop {
        match pc {
            0x82706150 => {
    //   block [0x82706150..0x827061BC)
	// 82706150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270615C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706160: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706164: 392BF624  addi r9, r11, -0x9dc
	ctx.r[9].s64 = ctx.r[11].s64 + -2524;
	// 82706168: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8270616C: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 82706170: 388AC058  addi r4, r10, -0x3fa8
	ctx.r[4].s64 = ctx.r[10].s64 + -16296;
	// 82706174: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706178: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270617C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82706180: 38C00130  li r6, 0x130
	ctx.r[6].s64 = 304;
	// 82706184: 386A0994  addi r3, r10, 0x994
	ctx.r[3].s64 = ctx.r[10].s64 + 2452;
	// 82706188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270618C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82706190: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706194: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706198: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270619C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827061A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827061A4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827061A8: 4BD60C79  bl 0x82466e20
	ctx.lr = 0x827061AC;
	sub_82466E20(ctx, base);
	// 827061AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827061B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827061B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827061B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827061C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827061C0 size=112
    let mut pc: u32 = 0x827061C0;
    'dispatch: loop {
        match pc {
            0x827061C0 => {
    //   block [0x827061C0..0x82706230)
	// 827061C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827061C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827061C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827061CC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827061D0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827061D4: 392BF5F8  addi r9, r11, -0xa08
	ctx.r[9].s64 = ctx.r[11].s64 + -2568;
	// 827061D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827061DC: 390900E8  addi r8, r9, 0xe8
	ctx.r[8].s64 = ctx.r[9].s64 + 232;
	// 827061E0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827061E4: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 827061E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827061EC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827061F0: 38C00140  li r6, 0x140
	ctx.r[6].s64 = 320;
	// 827061F4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827061F8: 388AC078  addi r4, r10, -0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + -16264;
	// 827061FC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706200: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82706204: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82706208: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8270620C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706210: 386B09C4  addi r3, r11, 0x9c4
	ctx.r[3].s64 = ctx.r[11].s64 + 2500;
	// 82706214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706218: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270621C: 4BD60C05  bl 0x82466e20
	ctx.lr = 0x82706220;
	sub_82466E20(ctx, base);
	// 82706220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270622C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82706230 size=24
    let mut pc: u32 = 0x82706230;
    'dispatch: loop {
        match pc {
            0x82706230 => {
    //   block [0x82706230..0x82706248)
	// 82706230: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82706234: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82706238: 394AAEC0  addi r10, r10, -0x5140
	ctx.r[10].s64 = ctx.r[10].s64 + -20800;
	// 8270623C: 816BAEA8  lwz r11, -0x5158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20824 as u32) ) } as u64;
	// 82706240: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82706244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706248 size=112
    let mut pc: u32 = 0x82706248;
    'dispatch: loop {
        match pc {
            0x82706248 => {
    //   block [0x82706248..0x827062B8)
	// 82706248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270624C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706254: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82706258: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270625C: 392AF754  addi r9, r10, -0x8ac
	ctx.r[9].s64 = ctx.r[10].s64 + -2220;
	// 82706260: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706264: 390BAEC0  addi r8, r11, -0x5140
	ctx.r[8].s64 = ctx.r[11].s64 + -20800;
	// 82706268: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8270626C: 388ABE74  addi r4, r10, -0x418c
	ctx.r[4].s64 = ctx.r[10].s64 + -16780;
	// 82706270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706274: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270627C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82706280: 386A09F4  addi r3, r10, 0x9f4
	ctx.r[3].s64 = ctx.r[10].s64 + 2548;
	// 82706284: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82706288: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8270628C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270629C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827062A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827062A4: 4BD60B7D  bl 0x82466e20
	ctx.lr = 0x827062A8;
	sub_82466E20(ctx, base);
	// 827062A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827062AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827062B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827062B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827062B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827062B8 size=112
    let mut pc: u32 = 0x827062B8;
    'dispatch: loop {
        match pc {
            0x827062B8 => {
    //   block [0x827062B8..0x82706328)
	// 827062B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827062BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827062C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827062C4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827062C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 827062CC: 392BF818  addi r9, r11, -0x7e8
	ctx.r[9].s64 = ctx.r[11].s64 + -2024;
	// 827062D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827062D4: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 827062D8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827062DC: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 827062E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827062E4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827062E8: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 827062EC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827062F0: 388ABEA0  addi r4, r10, -0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + -16736;
	// 827062F4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827062F8: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827062FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82706300: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82706304: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706308: 386B0A24  addi r3, r11, 0xa24
	ctx.r[3].s64 = ctx.r[11].s64 + 2596;
	// 8270630C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706310: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706314: 4BD60B0D  bl 0x82466e20
	ctx.lr = 0x82706318;
	sub_82466E20(ctx, base);
	// 82706318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270631C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706328 size=108
    let mut pc: u32 = 0x82706328;
    'dispatch: loop {
        match pc {
            0x82706328 => {
    //   block [0x82706328..0x82706394)
	// 82706328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270632C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706334: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706338: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270633C: 38EBF848  addi r7, r11, -0x7b8
	ctx.r[7].s64 = ctx.r[11].s64 + -1976;
	// 82706340: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82706344: 388AC2F4  addi r4, r10, -0x3d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -15628;
	// 82706348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270634C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706350: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82706354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706358: 386A0A54  addi r3, r10, 0xa54
	ctx.r[3].s64 = ctx.r[10].s64 + 2644;
	// 8270635C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82706360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270636C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706374: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82706378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270637C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82706380: 4BD60AA1  bl 0x82466e20
	ctx.lr = 0x82706384;
	sub_82466E20(ctx, base);
	// 82706384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270638C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706398 size=108
    let mut pc: u32 = 0x82706398;
    'dispatch: loop {
        match pc {
            0x82706398 => {
    //   block [0x82706398..0x82706404)
	// 82706398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270639C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827063A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827063A4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827063A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827063AC: 38EBF8A8  addi r7, r11, -0x758
	ctx.r[7].s64 = ctx.r[11].s64 + -1880;
	// 827063B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 827063B4: 388AC324  addi r4, r10, -0x3cdc
	ctx.r[4].s64 = ctx.r[10].s64 + -15580;
	// 827063B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827063BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827063C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827063C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827063C8: 386A0A84  addi r3, r10, 0xa84
	ctx.r[3].s64 = ctx.r[10].s64 + 2692;
	// 827063CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827063D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827063D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827063D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827063DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827063E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827063E4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 827063E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827063EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827063F0: 4BD60A31  bl 0x82466e20
	ctx.lr = 0x827063F4;
	sub_82466E20(ctx, base);
	// 827063F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827063F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827063FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706408 size=112
    let mut pc: u32 = 0x82706408;
    'dispatch: loop {
        match pc {
            0x82706408 => {
    //   block [0x82706408..0x82706478)
	// 82706408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270640C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706414: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706418: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270641C: 38AA0874  addi r5, r10, 0x874
	ctx.r[5].s64 = ctx.r[10].s64 + 2164;
	// 82706420: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706424: 390BF920  addi r8, r11, -0x6e0
	ctx.r[8].s64 = ctx.r[11].s64 + -1760;
	// 82706428: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8270642C: 388ABF88  addi r4, r10, -0x4078
	ctx.r[4].s64 = ctx.r[10].s64 + -16504;
	// 82706430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706434: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270643C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706440: 386A0AB4  addi r3, r10, 0xab4
	ctx.r[3].s64 = ctx.r[10].s64 + 2740;
	// 82706444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270644C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270645C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82706460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706464: 4BD609BD  bl 0x82466e20
	ctx.lr = 0x82706468;
	sub_82466E20(ctx, base);
	// 82706468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270646C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706478 size=100
    let mut pc: u32 = 0x82706478;
    'dispatch: loop {
        match pc {
            0x82706478 => {
    //   block [0x82706478..0x827064DC)
	// 82706478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270647C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706484: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270648C: 38AA00D4  addi r5, r10, 0xd4
	ctx.r[5].s64 = ctx.r[10].s64 + 212;
	// 82706490: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706498: 388AC588  addi r4, r10, -0x3a78
	ctx.r[4].s64 = ctx.r[10].s64 + -14968;
	// 8270649C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827064A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827064A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827064A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827064AC: 386A0AE4  addi r3, r10, 0xae4
	ctx.r[3].s64 = ctx.r[10].s64 + 2788;
	// 827064B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827064B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827064B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827064BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827064C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827064C4: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 827064C8: 4BD60959  bl 0x82466e20
	ctx.lr = 0x827064CC;
	sub_82466E20(ctx, base);
	// 827064CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827064D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827064D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827064D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827064E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827064E0 size=108
    let mut pc: u32 = 0x827064E0;
    'dispatch: loop {
        match pc {
            0x827064E0 => {
    //   block [0x827064E0..0x8270654C)
	// 827064E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827064E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827064E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827064EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827064F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827064F4: 38EBF9CC  addi r7, r11, -0x634
	ctx.r[7].s64 = ctx.r[11].s64 + -1588;
	// 827064F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 827064FC: 388ABECC  addi r4, r10, -0x4134
	ctx.r[4].s64 = ctx.r[10].s64 + -16692;
	// 82706500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706504: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706508: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270650C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706510: 386A0B14  addi r3, r10, 0xb14
	ctx.r[3].s64 = ctx.r[10].s64 + 2836;
	// 82706514: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82706518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270651C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270652C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82706530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82706538: 4BD608E9  bl 0x82466e20
	ctx.lr = 0x8270653C;
	sub_82466E20(ctx, base);
	// 8270653C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706550 size=112
    let mut pc: u32 = 0x82706550;
    'dispatch: loop {
        match pc {
            0x82706550 => {
    //   block [0x82706550..0x827065C0)
	// 82706550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270655C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706560: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706564: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82706568: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270656C: 390BF9FC  addi r8, r11, -0x604
	ctx.r[8].s64 = ctx.r[11].s64 + -1540;
	// 82706570: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82706574: 388ABEF0  addi r4, r10, -0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + -16656;
	// 82706578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270657C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706588: 386A0B44  addi r3, r10, 0xb44
	ctx.r[3].s64 = ctx.r[10].s64 + 2884;
	// 8270658C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270659C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827065A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827065A4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 827065A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827065AC: 4BD60875  bl 0x82466e20
	ctx.lr = 0x827065B0;
	sub_82466E20(ctx, base);
	// 827065B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827065B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827065B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827065BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827065C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827065C0 size=100
    let mut pc: u32 = 0x827065C0;
    'dispatch: loop {
        match pc {
            0x827065C0 => {
    //   block [0x827065C0..0x82706624)
	// 827065C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827065C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827065C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827065CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827065D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827065D4: 38AA0BD8  addi r5, r10, 0xbd8
	ctx.r[5].s64 = ctx.r[10].s64 + 3032;
	// 827065D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827065DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827065E0: 388AC574  addi r4, r10, -0x3a8c
	ctx.r[4].s64 = ctx.r[10].s64 + -14988;
	// 827065E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827065E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827065EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827065F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827065F4: 386A0B74  addi r3, r10, 0xb74
	ctx.r[3].s64 = ctx.r[10].s64 + 2932;
	// 827065F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827065FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706600: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82706604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706608: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270660C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 82706610: 4BD60811  bl 0x82466e20
	ctx.lr = 0x82706614;
	sub_82466E20(ctx, base);
	// 82706614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270661C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706628 size=108
    let mut pc: u32 = 0x82706628;
    'dispatch: loop {
        match pc {
            0x82706628 => {
    //   block [0x82706628..0x82706694)
	// 82706628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270662C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706634: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706638: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270663C: 38EBFF98  addi r7, r11, -0x68
	ctx.r[7].s64 = ctx.r[11].s64 + -104;
	// 82706640: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82706644: 388ABE14  addi r4, r10, -0x41ec
	ctx.r[4].s64 = ctx.r[10].s64 + -16876;
	// 82706648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270664C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82706654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706658: 386A0BA8  addi r3, r10, 0xba8
	ctx.r[3].s64 = ctx.r[10].s64 + 2984;
	// 8270665C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82706660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270666C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706674: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82706678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270667C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82706680: 4BD607A1  bl 0x82466e20
	ctx.lr = 0x82706684;
	sub_82466E20(ctx, base);
	// 82706684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270668C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82706698 size=24
    let mut pc: u32 = 0x82706698;
    'dispatch: loop {
        match pc {
            0x82706698 => {
    //   block [0x82706698..0x827066B0)
	// 82706698: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270669C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827066A0: 394AB320  addi r10, r10, -0x4ce0
	ctx.r[10].s64 = ctx.r[10].s64 + -19680;
	// 827066A4: 816BB318  lwz r11, -0x4ce8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19688 as u32) ) } as u64;
	// 827066A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827066AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827066B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827066B0 size=116
    let mut pc: u32 = 0x827066B0;
    'dispatch: loop {
        match pc {
            0x827066B0 => {
    //   block [0x827066B0..0x82706724)
	// 827066B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827066B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827066B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827066BC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827066C0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827066C4: 390BB320  addi r8, r11, -0x4ce0
	ctx.r[8].s64 = ctx.r[11].s64 + -19680;
	// 827066C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827066CC: 392A0038  addi r9, r10, 0x38
	ctx.r[9].s64 = ctx.r[10].s64 + 56;
	// 827066D0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 827066D4: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 827066D8: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 827066DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827066E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827066E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827066E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827066EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827066F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827066F4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827066F8: 388AC558  addi r4, r10, -0x3aa8
	ctx.r[4].s64 = ctx.r[10].s64 + -15016;
	// 827066FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82706700: 386B0BD8  addi r3, r11, 0xbd8
	ctx.r[3].s64 = ctx.r[11].s64 + 3032;
	// 82706704: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82706708: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270670C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 82706710: 4BD60711  bl 0x82466e20
	ctx.lr = 0x82706714;
	sub_82466E20(ctx, base);
	// 82706714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270671C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706728 size=96
    let mut pc: u32 = 0x82706728;
    'dispatch: loop {
        match pc {
            0x82706728 => {
    //   block [0x82706728..0x82706788)
	// 82706728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270672C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706730: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706734: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82706738: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8270673C: 4BD8814D  bl 0x8248e888
	ctx.lr = 0x82706740;
	sub_8248E888(ctx, base);
	// 82706740: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82706744: 3CE0824A  lis r7, -0x7db6
	ctx.r[7].s64 = -2109079552;
	// 82706748: 394BC4E8  addi r10, r11, -0x3b18
	ctx.r[10].s64 = ctx.r[11].s64 + -15128;
	// 8270674C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706750: 3D00824A  lis r8, -0x7db6
	ctx.r[8].s64 = -2109079552;
	// 82706754: 392BD8C4  addi r9, r11, -0x273c
	ctx.r[9].s64 = ctx.r[11].s64 + -10044;
	// 82706758: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8270675C: 396B0C08  addi r11, r11, 0xc08
	ctx.r[11].s64 = ctx.r[11].s64 + 3080;
	// 82706760: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82706764: 3947AEC0  addi r10, r7, -0x5140
	ctx.r[10].s64 = ctx.r[7].s64 + -20800;
	// 82706768: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270676C: 3948AEA8  addi r10, r8, -0x5158
	ctx.r[10].s64 = ctx.r[8].s64 + -20824;
	// 82706770: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82706774: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82706778: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 8270677C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706788 size=100
    let mut pc: u32 = 0x82706788;
    'dispatch: loop {
        match pc {
            0x82706788 => {
    //   block [0x82706788..0x827067EC)
	// 82706788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270678C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706794: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270679C: 38AA0244  addi r5, r10, 0x244
	ctx.r[5].s64 = ctx.r[10].s64 + 580;
	// 827067A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827067A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827067A8: 388AC4E8  addi r4, r10, -0x3b18
	ctx.r[4].s64 = ctx.r[10].s64 + -15128;
	// 827067AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827067B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827067B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827067B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827067BC: 386A0C18  addi r3, r10, 0xc18
	ctx.r[3].s64 = ctx.r[10].s64 + 3096;
	// 827067C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827067C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827067C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827067CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827067D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827067D4: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 827067D8: 4BD60649  bl 0x82466e20
	ctx.lr = 0x827067DC;
	sub_82466E20(ctx, base);
	// 827067DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827067E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827067E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827067E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827067F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827067F0 size=112
    let mut pc: u32 = 0x827067F0;
    'dispatch: loop {
        match pc {
            0x827067F0 => {
    //   block [0x827067F0..0x82706860)
	// 827067F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827067F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827067F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827067FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82706800: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706804: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82706808: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270680C: 390B0068  addi r8, r11, 0x68
	ctx.r[8].s64 = ctx.r[11].s64 + 104;
	// 82706810: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82706814: 388ABE38  addi r4, r10, -0x41c8
	ctx.r[4].s64 = ctx.r[10].s64 + -16840;
	// 82706818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270681C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706828: 386A0C48  addi r3, r10, 0xc48
	ctx.r[3].s64 = ctx.r[10].s64 + 3144;
	// 8270682C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270683C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706844: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 82706848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270684C: 4BD605D5  bl 0x82466e20
	ctx.lr = 0x82706850;
	sub_82466E20(ctx, base);
	// 82706850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270685C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706860 size=112
    let mut pc: u32 = 0x82706860;
    'dispatch: loop {
        match pc {
            0x82706860 => {
    //   block [0x82706860..0x827068D0)
	// 82706860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270686C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706870: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706874: 38AA0514  addi r5, r10, 0x514
	ctx.r[5].s64 = ctx.r[10].s64 + 1300;
	// 82706878: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270687C: 390B00C8  addi r8, r11, 0xc8
	ctx.r[8].s64 = ctx.r[11].s64 + 200;
	// 82706880: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82706884: 388AC374  addi r4, r10, -0x3c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -15500;
	// 82706888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270688C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706898: 386A0C78  addi r3, r10, 0xc78
	ctx.r[3].s64 = ctx.r[10].s64 + 3192;
	// 8270689C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827068A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827068A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827068A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827068AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827068B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827068B4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 827068B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827068BC: 4BD60565  bl 0x82466e20
	ctx.lr = 0x827068C0;
	sub_82466E20(ctx, base);
	// 827068C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827068C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827068C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827068CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827068D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827068D0 size=92
    let mut pc: u32 = 0x827068D0;
    'dispatch: loop {
        match pc {
            0x827068D0 => {
    //   block [0x827068D0..0x8270692C)
	// 827068D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827068D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827068D8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827068DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827068E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827068E4: 4BDB1BF5  bl 0x824b84d8
	ctx.lr = 0x827068E8;
	sub_824B84D8(ctx, base);
	// 827068E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827068EC: 3D00824A  lis r8, -0x7db6
	ctx.r[8].s64 = -2109079552;
	// 827068F0: 394BC2D8  addi r10, r11, -0x3d28
	ctx.r[10].s64 = ctx.r[11].s64 + -15656;
	// 827068F4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827068F8: 3D20824A  lis r9, -0x7db6
	ctx.r[9].s64 = -2109079552;
	// 827068FC: 396B0CA8  addi r11, r11, 0xca8
	ctx.r[11].s64 = ctx.r[11].s64 + 3240;
	// 82706900: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82706904: 3948B2E8  addi r10, r8, -0x4d18
	ctx.r[10].s64 = ctx.r[8].s64 + -19736;
	// 82706908: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270690C: 3949B2D0  addi r10, r9, -0x4d30
	ctx.r[10].s64 = ctx.r[9].s64 + -19760;
	// 82706910: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82706914: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82706918: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8270691C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82706920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706930 size=112
    let mut pc: u32 = 0x82706930;
    'dispatch: loop {
        match pc {
            0x82706930 => {
    //   block [0x82706930..0x827069A0)
	// 82706930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270693C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706940: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706944: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82706948: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270694C: 390B00F8  addi r8, r11, 0xf8
	ctx.r[8].s64 = ctx.r[11].s64 + 248;
	// 82706950: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82706954: 388AC2D8  addi r4, r10, -0x3d28
	ctx.r[4].s64 = ctx.r[10].s64 + -15656;
	// 82706958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270695C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706968: 386A0CB8  addi r3, r10, 0xcb8
	ctx.r[3].s64 = ctx.r[10].s64 + 3256;
	// 8270696C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270697C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706984: 38C00058  li r6, 0x58
	ctx.r[6].s64 = 88;
	// 82706988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270698C: 4BD60495  bl 0x82466e20
	ctx.lr = 0x82706990;
	sub_82466E20(ctx, base);
	// 82706990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270699C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827069A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827069A0 size=100
    let mut pc: u32 = 0x827069A0;
    'dispatch: loop {
        match pc {
            0x827069A0 => {
    //   block [0x827069A0..0x82706A04)
	// 827069A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827069A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827069A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827069AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 827069B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827069B4: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 827069B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827069BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827069C0: 388AC4B4  addi r4, r10, -0x3b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -15180;
	// 827069C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827069C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827069CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827069D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827069D4: 386A0CE8  addi r3, r10, 0xce8
	ctx.r[3].s64 = ctx.r[10].s64 + 3304;
	// 827069D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827069DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827069E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827069E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827069E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827069EC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 827069F0: 4BD60431  bl 0x82466e20
	ctx.lr = 0x827069F4;
	sub_82466E20(ctx, base);
	// 827069F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827069F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827069FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706A08 size=108
    let mut pc: u32 = 0x82706A08;
    'dispatch: loop {
        match pc {
            0x82706A08 => {
    //   block [0x82706A08..0x82706A74)
	// 82706A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706A14: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706A18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706A1C: 392B0138  addi r9, r11, 0x138
	ctx.r[9].s64 = ctx.r[11].s64 + 312;
	// 82706A20: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82706A24: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 82706A28: 388AC01C  addi r4, r10, -0x3fe4
	ctx.r[4].s64 = ctx.r[10].s64 + -16356;
	// 82706A2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706A30: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706A34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82706A38: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 82706A3C: 386A0D18  addi r3, r10, 0xd18
	ctx.r[3].s64 = ctx.r[10].s64 + 3352;
	// 82706A40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706A44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82706A48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706A4C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706A50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706A54: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706A58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82706A5C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706A60: 4BD603C1  bl 0x82466e20
	ctx.lr = 0x82706A64;
	sub_82466E20(ctx, base);
	// 82706A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706A78 size=112
    let mut pc: u32 = 0x82706A78;
    'dispatch: loop {
        match pc {
            0x82706A78 => {
    //   block [0x82706A78..0x82706AE8)
	// 82706A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706A84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706A88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706A8C: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82706A90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706A94: 390B01F8  addi r8, r11, 0x1f8
	ctx.r[8].s64 = ctx.r[11].s64 + 504;
	// 82706A98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82706A9C: 388AC03C  addi r4, r10, -0x3fc4
	ctx.r[4].s64 = ctx.r[10].s64 + -16324;
	// 82706AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706AA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706AB0: 386A0D48  addi r3, r10, 0xd48
	ctx.r[3].s64 = ctx.r[10].s64 + 3400;
	// 82706AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706ACC: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 82706AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706AD4: 4BD6034D  bl 0x82466e20
	ctx.lr = 0x82706AD8;
	sub_82466E20(ctx, base);
	// 82706AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706AE8 size=112
    let mut pc: u32 = 0x82706AE8;
    'dispatch: loop {
        match pc {
            0x82706AE8 => {
    //   block [0x82706AE8..0x82706B58)
	// 82706AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706AF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706AF8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706AFC: 38AA0624  addi r5, r10, 0x624
	ctx.r[5].s64 = ctx.r[10].s64 + 1572;
	// 82706B00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706B04: 390B0244  addi r8, r11, 0x244
	ctx.r[8].s64 = ctx.r[11].s64 + 580;
	// 82706B08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82706B0C: 388AC180  addi r4, r10, -0x3e80
	ctx.r[4].s64 = ctx.r[10].s64 + -16000;
	// 82706B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706B14: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706B20: 386A0D78  addi r3, r10, 0xd78
	ctx.r[3].s64 = ctx.r[10].s64 + 3448;
	// 82706B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706B3C: 38C0003C  li r6, 0x3c
	ctx.r[6].s64 = 60;
	// 82706B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706B44: 4BD602DD  bl 0x82466e20
	ctx.lr = 0x82706B48;
	sub_82466E20(ctx, base);
	// 82706B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706B58 size=92
    let mut pc: u32 = 0x82706B58;
    'dispatch: loop {
        match pc {
            0x82706B58 => {
    //   block [0x82706B58..0x82706BB4)
	// 82706B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706B60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706B64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82706B68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82706B6C: 4BDB373D  bl 0x824ba2a8
	ctx.lr = 0x82706B70;
	sub_824BA2A8(ctx, base);
	// 82706B70: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82706B74: 3D00824A  lis r8, -0x7db6
	ctx.r[8].s64 = -2109079552;
	// 82706B78: 394BC14C  addi r10, r11, -0x3eb4
	ctx.r[10].s64 = ctx.r[11].s64 + -16052;
	// 82706B7C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82706B80: 3D20824A  lis r9, -0x7db6
	ctx.r[9].s64 = -2109079552;
	// 82706B84: 396B0DA8  addi r11, r11, 0xda8
	ctx.r[11].s64 = ctx.r[11].s64 + 3496;
	// 82706B88: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82706B8C: 3948B828  addi r10, r8, -0x47d8
	ctx.r[10].s64 = ctx.r[8].s64 + -18392;
	// 82706B90: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82706B94: 3949B810  addi r10, r9, -0x47f0
	ctx.r[10].s64 = ctx.r[9].s64 + -18416;
	// 82706B98: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82706B9C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82706BA0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82706BA4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82706BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706BB8 size=112
    let mut pc: u32 = 0x82706BB8;
    'dispatch: loop {
        match pc {
            0x82706BB8 => {
    //   block [0x82706BB8..0x82706C28)
	// 82706BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706BC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706BC8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706BCC: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82706BD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706BD4: 390B0298  addi r8, r11, 0x298
	ctx.r[8].s64 = ctx.r[11].s64 + 664;
	// 82706BD8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82706BDC: 388AC14C  addi r4, r10, -0x3eb4
	ctx.r[4].s64 = ctx.r[10].s64 + -16052;
	// 82706BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706BE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706BE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706BF0: 386A0DB8  addi r3, r10, 0xdb8
	ctx.r[3].s64 = ctx.r[10].s64 + 3512;
	// 82706BF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706C0C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82706C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706C14: 4BD6020D  bl 0x82466e20
	ctx.lr = 0x82706C18;
	sub_82466E20(ctx, base);
	// 82706C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706C28 size=116
    let mut pc: u32 = 0x82706C28;
    'dispatch: loop {
        match pc {
            0x82706C28 => {
    //   block [0x82706C28..0x82706C9C)
	// 82706C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706C34: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706C38: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82706C3C: 390B03B8  addi r8, r11, 0x3b8
	ctx.r[8].s64 = ctx.r[11].s64 + 952;
	// 82706C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706C44: 392A03A4  addi r9, r10, 0x3a4
	ctx.r[9].s64 = ctx.r[10].s64 + 932;
	// 82706C48: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82706C4C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82706C50: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82706C54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706C58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706C5C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706C60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706C68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706C6C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82706C70: 388AC6D4  addi r4, r10, -0x392c
	ctx.r[4].s64 = ctx.r[10].s64 + -14636;
	// 82706C74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82706C78: 386B0DE8  addi r3, r11, 0xde8
	ctx.r[3].s64 = ctx.r[11].s64 + 3560;
	// 82706C7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82706C80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706C84: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 82706C88: 4BD60199  bl 0x82466e20
	ctx.lr = 0x82706C8C;
	sub_82466E20(ctx, base);
	// 82706C8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706CA0 size=112
    let mut pc: u32 = 0x82706CA0;
    'dispatch: loop {
        match pc {
            0x82706CA0 => {
    //   block [0x82706CA0..0x82706D10)
	// 82706CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706CAC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706CB0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706CB4: 38AA0EB8  addi r5, r10, 0xeb8
	ctx.r[5].s64 = ctx.r[10].s64 + 3768;
	// 82706CB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706CBC: 390B0448  addi r8, r11, 0x448
	ctx.r[8].s64 = ctx.r[11].s64 + 1096;
	// 82706CC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82706CC4: 388AC654  addi r4, r10, -0x39ac
	ctx.r[4].s64 = ctx.r[10].s64 + -14764;
	// 82706CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706CCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706CD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706CD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706CD8: 386A0E18  addi r3, r10, 0xe18
	ctx.r[3].s64 = ctx.r[10].s64 + 3608;
	// 82706CDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706CE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706CF4: 38C00150  li r6, 0x150
	ctx.r[6].s64 = 336;
	// 82706CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706CFC: 4BD60125  bl 0x82466e20
	ctx.lr = 0x82706D00;
	sub_82466E20(ctx, base);
	// 82706D00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706D10 size=104
    let mut pc: u32 = 0x82706D10;
    'dispatch: loop {
        match pc {
            0x82706D10 => {
    //   block [0x82706D10..0x82706D78)
	// 82706D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706D1C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82706D20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706D24: 392A0480  addi r9, r10, 0x480
	ctx.r[9].s64 = ctx.r[10].s64 + 1152;
	// 82706D28: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706D30: 38AA0CE8  addi r5, r10, 0xce8
	ctx.r[5].s64 = ctx.r[10].s64 + 3304;
	// 82706D34: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706D38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706D40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706D44: 388AC4F8  addi r4, r10, -0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + -15112;
	// 82706D48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706D4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706D50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82706D54: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82706D58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82706D5C: 386A0E48  addi r3, r10, 0xe48
	ctx.r[3].s64 = ctx.r[10].s64 + 3656;
	// 82706D60: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82706D64: 4BD600BD  bl 0x82466e20
	ctx.lr = 0x82706D68;
	sub_82466E20(ctx, base);
	// 82706D68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706D78 size=92
    let mut pc: u32 = 0x82706D78;
    'dispatch: loop {
        match pc {
            0x82706D78 => {
    //   block [0x82706D78..0x82706DD4)
	// 82706D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706D80: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706D84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82706D88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82706D8C: 4BD94B45  bl 0x8249b8d0
	ctx.lr = 0x82706D90;
	sub_8249B8D0(ctx, base);
	// 82706D90: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82706D94: 3D00824A  lis r8, -0x7db6
	ctx.r[8].s64 = -2109079552;
	// 82706D98: 394BC620  addi r10, r11, -0x39e0
	ctx.r[10].s64 = ctx.r[11].s64 + -14816;
	// 82706D9C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82706DA0: 3D20824A  lis r9, -0x7db6
	ctx.r[9].s64 = -2109079552;
	// 82706DA4: 396B0E78  addi r11, r11, 0xe78
	ctx.r[11].s64 = ctx.r[11].s64 + 3704;
	// 82706DA8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82706DAC: 3948B888  addi r10, r8, -0x4778
	ctx.r[10].s64 = ctx.r[8].s64 + -18296;
	// 82706DB0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82706DB4: 3949B870  addi r10, r9, -0x4790
	ctx.r[10].s64 = ctx.r[9].s64 + -18320;
	// 82706DB8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82706DBC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82706DC0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82706DC4: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82706DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706DD8 size=112
    let mut pc: u32 = 0x82706DD8;
    'dispatch: loop {
        match pc {
            0x82706DD8 => {
    //   block [0x82706DD8..0x82706E48)
	// 82706DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706DE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706DE8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706DEC: 38AA0EB8  addi r5, r10, 0xeb8
	ctx.r[5].s64 = ctx.r[10].s64 + 3768;
	// 82706DF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706DF4: 390B0494  addi r8, r11, 0x494
	ctx.r[8].s64 = ctx.r[11].s64 + 1172;
	// 82706DF8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82706DFC: 388AC620  addi r4, r10, -0x39e0
	ctx.r[4].s64 = ctx.r[10].s64 + -14816;
	// 82706E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706E04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706E08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706E10: 386A0E88  addi r3, r10, 0xe88
	ctx.r[3].s64 = ctx.r[10].s64 + 3720;
	// 82706E14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706E18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706E2C: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 82706E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706E34: 4BD5FFED  bl 0x82466e20
	ctx.lr = 0x82706E38;
	sub_82466E20(ctx, base);
	// 82706E38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706E48 size=112
    let mut pc: u32 = 0x82706E48;
    'dispatch: loop {
        match pc {
            0x82706E48 => {
    //   block [0x82706E48..0x82706EB8)
	// 82706E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706E54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706E58: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706E5C: 38AA0DE8  addi r5, r10, 0xde8
	ctx.r[5].s64 = ctx.r[10].s64 + 3560;
	// 82706E60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706E64: 390B04FC  addi r8, r11, 0x4fc
	ctx.r[8].s64 = ctx.r[11].s64 + 1276;
	// 82706E68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82706E6C: 388AC648  addi r4, r10, -0x39b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14776;
	// 82706E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706E74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706E78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706E80: 386A0EB8  addi r3, r10, 0xeb8
	ctx.r[3].s64 = ctx.r[10].s64 + 3768;
	// 82706E84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706E9C: 38C00098  li r6, 0x98
	ctx.r[6].s64 = 152;
	// 82706EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706EA4: 4BD5FF7D  bl 0x82466e20
	ctx.lr = 0x82706EA8;
	sub_82466E20(ctx, base);
	// 82706EA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82706EB8 size=12
    let mut pc: u32 = 0x82706EB8;
    'dispatch: loop {
        match pc {
            0x82706EB8 => {
    //   block [0x82706EB8..0x82706EC4)
	// 82706EB8: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82706EBC: 386BCF78  addi r3, r11, -0x3088
	ctx.r[3].s64 = ctx.r[11].s64 + -12424;
	// 82706EC0: 4BE2BC78  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706EC8 size=112
    let mut pc: u32 = 0x82706EC8;
    'dispatch: loop {
        match pc {
            0x82706EC8 => {
    //   block [0x82706EC8..0x82706F38)
	// 82706EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706ED4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82706ED8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706EDC: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82706EE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706EE4: 390B1020  addi r8, r11, 0x1020
	ctx.r[8].s64 = ctx.r[11].s64 + 4128;
	// 82706EE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82706EEC: 388AC7A8  addi r4, r10, -0x3858
	ctx.r[4].s64 = ctx.r[10].s64 + -14424;
	// 82706EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706EF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706F00: 386A0F34  addi r3, r10, 0xf34
	ctx.r[3].s64 = ctx.r[10].s64 + 3892;
	// 82706F04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706F1C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82706F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706F24: 4BD5FEFD  bl 0x82466e20
	ctx.lr = 0x82706F28;
	sub_82466E20(ctx, base);
	// 82706F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706F38 size=112
    let mut pc: u32 = 0x82706F38;
    'dispatch: loop {
        match pc {
            0x82706F38 => {
    //   block [0x82706F38..0x82706FA8)
	// 82706F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706F44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706F48: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706F4C: 38AA1C30  addi r5, r10, 0x1c30
	ctx.r[5].s64 = ctx.r[10].s64 + 7216;
	// 82706F50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706F54: 390B1078  addi r8, r11, 0x1078
	ctx.r[8].s64 = ctx.r[11].s64 + 4216;
	// 82706F58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82706F5C: 388AC780  addi r4, r10, -0x3880
	ctx.r[4].s64 = ctx.r[10].s64 + -14464;
	// 82706F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706F64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706F68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706F70: 386A0F64  addi r3, r10, 0xf64
	ctx.r[3].s64 = ctx.r[10].s64 + 3940;
	// 82706F74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706F8C: 38C00058  li r6, 0x58
	ctx.r[6].s64 = 88;
	// 82706F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706F94: 4BD5FE8D  bl 0x82466e20
	ctx.lr = 0x82706F98;
	sub_82466E20(ctx, base);
	// 82706F98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706FA8 size=108
    let mut pc: u32 = 0x82706FA8;
    'dispatch: loop {
        match pc {
            0x82706FA8 => {
    //   block [0x82706FA8..0x82707014)
	// 82706FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706FB4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706FB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706FBC: 38EB1090  addi r7, r11, 0x1090
	ctx.r[7].s64 = ctx.r[11].s64 + 4240;
	// 82706FC0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82706FC4: 388AC76C  addi r4, r10, -0x3894
	ctx.r[4].s64 = ctx.r[10].s64 + -14484;
	// 82706FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706FCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706FD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82706FD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706FD8: 386A0F94  addi r3, r10, 0xf94
	ctx.r[3].s64 = ctx.r[10].s64 + 3988;
	// 82706FDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82706FE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706FE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706FEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706FF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706FF4: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 82706FF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706FFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707000: 4BD5FE21  bl 0x82466e20
	ctx.lr = 0x82707004;
	sub_82466E20(ctx, base);
	// 82707004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270700C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707018 size=108
    let mut pc: u32 = 0x82707018;
    'dispatch: loop {
        match pc {
            0x82707018 => {
    //   block [0x82707018..0x82707084)
	// 82707018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270701C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707024: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707028: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270702C: 38EB1108  addi r7, r11, 0x1108
	ctx.r[7].s64 = ctx.r[11].s64 + 4360;
	// 82707030: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82707034: 388AC7C4  addi r4, r10, -0x383c
	ctx.r[4].s64 = ctx.r[10].s64 + -14396;
	// 82707038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270703C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707040: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82707044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707048: 386A0FC4  addi r3, r10, 0xfc4
	ctx.r[3].s64 = ctx.r[10].s64 + 4036;
	// 8270704C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82707050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270705C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707064: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82707068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270706C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707070: 4BD5FDB1  bl 0x82466e20
	ctx.lr = 0x82707074;
	sub_82466E20(ctx, base);
	// 82707074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270707C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707088 size=108
    let mut pc: u32 = 0x82707088;
    'dispatch: loop {
        match pc {
            0x82707088 => {
    //   block [0x82707088..0x827070F4)
	// 82707088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270708C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707094: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707098: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270709C: 38EB115C  addi r7, r11, 0x115c
	ctx.r[7].s64 = ctx.r[11].s64 + 4444;
	// 827070A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827070A4: 388AC7E4  addi r4, r10, -0x381c
	ctx.r[4].s64 = ctx.r[10].s64 + -14364;
	// 827070A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827070AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827070B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827070B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827070B8: 386A0FF4  addi r3, r10, 0xff4
	ctx.r[3].s64 = ctx.r[10].s64 + 4084;
	// 827070BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827070C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827070C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827070C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827070CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827070D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827070D4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 827070D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827070DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827070E0: 4BD5FD41  bl 0x82466e20
	ctx.lr = 0x827070E4;
	sub_82466E20(ctx, base);
	// 827070E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827070E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827070EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827070F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827070F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827070F8 size=24
    let mut pc: u32 = 0x827070F8;
    'dispatch: loop {
        match pc {
            0x827070F8 => {
    //   block [0x827070F8..0x82707110)
	// 827070F8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827070FC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82707100: 394ABB50  addi r10, r10, -0x44b0
	ctx.r[10].s64 = ctx.r[10].s64 + -17584;
	// 82707104: 816BBB18  lwz r11, -0x44e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17640 as u32) ) } as u64;
	// 82707108: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8270710C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707110 size=116
    let mut pc: u32 = 0x82707110;
    'dispatch: loop {
        match pc {
            0x82707110 => {
    //   block [0x82707110..0x82707184)
	// 82707110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270711C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82707120: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82707124: 390BBB50  addi r8, r11, -0x44b0
	ctx.r[8].s64 = ctx.r[11].s64 + -17584;
	// 82707128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270712C: 392A1148  addi r9, r10, 0x1148
	ctx.r[9].s64 = ctx.r[10].s64 + 4424;
	// 82707130: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82707134: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82707138: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270713C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707144: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270714C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707154: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707158: 388AC7F8  addi r4, r10, -0x3808
	ctx.r[4].s64 = ctx.r[10].s64 + -14344;
	// 8270715C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82707160: 386B1024  addi r3, r11, 0x1024
	ctx.r[3].s64 = ctx.r[11].s64 + 4132;
	// 82707164: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82707168: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270716C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82707170: 4BD5FCB1  bl 0x82466e20
	ctx.lr = 0x82707174;
	sub_82466E20(ctx, base);
	// 82707174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270717C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707188 size=108
    let mut pc: u32 = 0x82707188;
    'dispatch: loop {
        match pc {
            0x82707188 => {
    //   block [0x82707188..0x827071F4)
	// 82707188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270718C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707194: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707198: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270719C: 38EB1184  addi r7, r11, 0x1184
	ctx.r[7].s64 = ctx.r[11].s64 + 4484;
	// 827071A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827071A4: 388AC794  addi r4, r10, -0x386c
	ctx.r[4].s64 = ctx.r[10].s64 + -14444;
	// 827071A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827071AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827071B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827071B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827071B8: 386A1054  addi r3, r10, 0x1054
	ctx.r[3].s64 = ctx.r[10].s64 + 4180;
	// 827071BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827071C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827071C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827071C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827071CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827071D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827071D4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 827071D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827071DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827071E0: 4BD5FC41  bl 0x82466e20
	ctx.lr = 0x827071E4;
	sub_82466E20(ctx, base);
	// 827071E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827071E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827071EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827071F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827071F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827071F8 size=112
    let mut pc: u32 = 0x827071F8;
    'dispatch: loop {
        match pc {
            0x827071F8 => {
    //   block [0x827071F8..0x82707268)
	// 827071F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827071FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707204: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707208: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270720C: 38AA07E4  addi r5, r10, 0x7e4
	ctx.r[5].s64 = ctx.r[10].s64 + 2020;
	// 82707210: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707214: 390B1580  addi r8, r11, 0x1580
	ctx.r[8].s64 = ctx.r[11].s64 + 5504;
	// 82707218: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270721C: 388AC804  addi r4, r10, -0x37fc
	ctx.r[4].s64 = ctx.r[10].s64 + -14332;
	// 82707220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707224: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270722C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707230: 386A1088  addi r3, r10, 0x1088
	ctx.r[3].s64 = ctx.r[10].s64 + 4232;
	// 82707234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270723C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270724C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82707250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707254: 4BD5FBCD  bl 0x82466e20
	ctx.lr = 0x82707258;
	sub_82466E20(ctx, base);
	// 82707258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270725C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707268 size=108
    let mut pc: u32 = 0x82707268;
    'dispatch: loop {
        match pc {
            0x82707268 => {
    //   block [0x82707268..0x827072D4)
	// 82707268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270726C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707274: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707278: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270727C: 38EB1610  addi r7, r11, 0x1610
	ctx.r[7].s64 = ctx.r[11].s64 + 5648;
	// 82707280: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82707284: 388AC9A0  addi r4, r10, -0x3660
	ctx.r[4].s64 = ctx.r[10].s64 + -13920;
	// 82707288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270728C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82707294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707298: 386A10F8  addi r3, r10, 0x10f8
	ctx.r[3].s64 = ctx.r[10].s64 + 4344;
	// 8270729C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827072A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827072A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827072A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827072AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827072B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827072B4: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 827072B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827072BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827072C0: 4BD5FB61  bl 0x82466e20
	ctx.lr = 0x827072C4;
	sub_82466E20(ctx, base);
	// 827072C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827072C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827072CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827072D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827072D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827072D8 size=112
    let mut pc: u32 = 0x827072D8;
    'dispatch: loop {
        match pc {
            0x827072D8 => {
    //   block [0x827072D8..0x82707348)
	// 827072D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827072DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827072E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827072E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827072E8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827072EC: 38AA10F8  addi r5, r10, 0x10f8
	ctx.r[5].s64 = ctx.r[10].s64 + 4344;
	// 827072F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827072F4: 390B1640  addi r8, r11, 0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + 5696;
	// 827072F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827072FC: 388AC9BC  addi r4, r10, -0x3644
	ctx.r[4].s64 = ctx.r[10].s64 + -13892;
	// 82707300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707304: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270730C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707310: 386A10B8  addi r3, r10, 0x10b8
	ctx.r[3].s64 = ctx.r[10].s64 + 4280;
	// 82707314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270731C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270732C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82707330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707334: 4BD5FAED  bl 0x82466e20
	ctx.lr = 0x82707338;
	sub_82466E20(ctx, base);
	// 82707338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270733C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707348 size=112
    let mut pc: u32 = 0x82707348;
    'dispatch: loop {
        match pc {
            0x82707348 => {
    //   block [0x82707348..0x827073B8)
	// 82707348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270734C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707350: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707354: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707358: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8270735C: 396B16D0  addi r11, r11, 0x16d0
	ctx.r[11].s64 = ctx.r[11].s64 + 5840;
	// 82707360: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82707364: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82707368: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 8270736C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82707370: 4BD9BCE9  bl 0x824a3058
	ctx.lr = 0x82707374;
	sub_824A3058(ctx, base);
	// 82707374: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82707378: 3D00824D  lis r8, -0x7db3
	ctx.r[8].s64 = -2108882944;
	// 8270737C: 394BCA40  addi r10, r11, -0x35c0
	ctx.r[10].s64 = ctx.r[11].s64 + -13760;
	// 82707380: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707384: 3D20824D  lis r9, -0x7db3
	ctx.r[9].s64 = -2108882944;
	// 82707388: 396B10E8  addi r11, r11, 0x10e8
	ctx.r[11].s64 = ctx.r[11].s64 + 4328;
	// 8270738C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82707390: 39486B18  addi r10, r8, 0x6b18
	ctx.r[10].s64 = ctx.r[8].s64 + 27416;
	// 82707394: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82707398: 39496AE0  addi r10, r9, 0x6ae0
	ctx.r[10].s64 = ctx.r[9].s64 + 27360;
	// 8270739C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827073A0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827073A4: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827073A8: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 827073AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827073B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827073B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827073B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827073B8 size=24
    let mut pc: u32 = 0x827073B8;
    'dispatch: loop {
        match pc {
            0x827073B8 => {
    //   block [0x827073B8..0x827073D0)
	// 827073B8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827073BC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827073C0: 394ABD38  addi r10, r10, -0x42c8
	ctx.r[10].s64 = ctx.r[10].s64 + -17096;
	// 827073C4: 816BBD10  lwz r11, -0x42f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17136 as u32) ) } as u64;
	// 827073C8: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 827073CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827073D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827073D0 size=116
    let mut pc: u32 = 0x827073D0;
    'dispatch: loop {
        match pc {
            0x827073D0 => {
    //   block [0x827073D0..0x82707444)
	// 827073D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827073D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827073D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827073DC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827073E0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827073E4: 390BBD38  addi r8, r11, -0x42c8
	ctx.r[8].s64 = ctx.r[11].s64 + -17096;
	// 827073E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827073EC: 392A16B8  addi r9, r10, 0x16b8
	ctx.r[9].s64 = ctx.r[10].s64 + 5816;
	// 827073F0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 827073F4: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 827073F8: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 827073FC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707404: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270740C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707414: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707418: 388ACA40  addi r4, r10, -0x35c0
	ctx.r[4].s64 = ctx.r[10].s64 + -13760;
	// 8270741C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82707420: 386B1128  addi r3, r11, 0x1128
	ctx.r[3].s64 = ctx.r[11].s64 + 4392;
	// 82707424: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82707428: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270742C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 82707430: 4BD5F9F1  bl 0x82466e20
	ctx.lr = 0x82707434;
	sub_82466E20(ctx, base);
	// 82707434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270743C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707448 size=108
    let mut pc: u32 = 0x82707448;
    'dispatch: loop {
        match pc {
            0x82707448 => {
    //   block [0x82707448..0x827074B4)
	// 82707448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270744C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707454: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707458: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270745C: 38EB16E0  addi r7, r11, 0x16e0
	ctx.r[7].s64 = ctx.r[11].s64 + 5856;
	// 82707460: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82707464: 388ACA5C  addi r4, r10, -0x35a4
	ctx.r[4].s64 = ctx.r[10].s64 + -13732;
	// 82707468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270746C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82707474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707478: 386A1158  addi r3, r10, 0x1158
	ctx.r[3].s64 = ctx.r[10].s64 + 4440;
	// 8270747C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82707480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270748C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707494: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82707498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270749C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827074A0: 4BD5F981  bl 0x82466e20
	ctx.lr = 0x827074A4;
	sub_82466E20(ctx, base);
	// 827074A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827074A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827074AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827074B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827074B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827074B8 size=108
    let mut pc: u32 = 0x827074B8;
    'dispatch: loop {
        match pc {
            0x827074B8 => {
    //   block [0x827074B8..0x82707524)
	// 827074B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827074BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827074C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827074C4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827074C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827074CC: 38EB1728  addi r7, r11, 0x1728
	ctx.r[7].s64 = ctx.r[11].s64 + 5928;
	// 827074D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 827074D4: 388ACA78  addi r4, r10, -0x3588
	ctx.r[4].s64 = ctx.r[10].s64 + -13704;
	// 827074D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827074DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827074E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827074E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827074E8: 386A1188  addi r3, r10, 0x1188
	ctx.r[3].s64 = ctx.r[10].s64 + 4488;
	// 827074EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827074F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827074F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827074F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827074FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707504: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82707508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270750C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707510: 4BD5F911  bl 0x82466e20
	ctx.lr = 0x82707514;
	sub_82466E20(ctx, base);
	// 82707514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270751C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707528 size=108
    let mut pc: u32 = 0x82707528;
    'dispatch: loop {
        match pc {
            0x82707528 => {
    //   block [0x82707528..0x82707594)
	// 82707528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270752C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707534: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707538: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270753C: 38EB1758  addi r7, r11, 0x1758
	ctx.r[7].s64 = ctx.r[11].s64 + 5976;
	// 82707540: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82707544: 388ACA98  addi r4, r10, -0x3568
	ctx.r[4].s64 = ctx.r[10].s64 + -13672;
	// 82707548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270754C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82707554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707558: 386A11B8  addi r3, r10, 0x11b8
	ctx.r[3].s64 = ctx.r[10].s64 + 4536;
	// 8270755C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82707560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270756C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707574: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 82707578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270757C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707580: 4BD5F8A1  bl 0x82466e20
	ctx.lr = 0x82707584;
	sub_82466E20(ctx, base);
	// 82707584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270758C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707598 size=108
    let mut pc: u32 = 0x82707598;
    'dispatch: loop {
        match pc {
            0x82707598 => {
    //   block [0x82707598..0x82707604)
	// 82707598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270759C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827075A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827075A4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827075A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827075AC: 38EB1788  addi r7, r11, 0x1788
	ctx.r[7].s64 = ctx.r[11].s64 + 6024;
	// 827075B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 827075B4: 388AC8F4  addi r4, r10, -0x370c
	ctx.r[4].s64 = ctx.r[10].s64 + -14092;
	// 827075B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827075BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827075C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827075C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827075C8: 386A1248  addi r3, r10, 0x1248
	ctx.r[3].s64 = ctx.r[10].s64 + 4680;
	// 827075CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827075D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827075D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827075D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827075DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827075E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827075E4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 827075E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827075EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827075F0: 4BD5F831  bl 0x82466e20
	ctx.lr = 0x827075F4;
	sub_82466E20(ctx, base);
	// 827075F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827075F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827075FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707608 size=108
    let mut pc: u32 = 0x82707608;
    'dispatch: loop {
        match pc {
            0x82707608 => {
    //   block [0x82707608..0x82707674)
	// 82707608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270760C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707614: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707618: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270761C: 38EB17B8  addi r7, r11, 0x17b8
	ctx.r[7].s64 = ctx.r[11].s64 + 6072;
	// 82707620: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82707624: 388AC910  addi r4, r10, -0x36f0
	ctx.r[4].s64 = ctx.r[10].s64 + -14064;
	// 82707628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270762C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82707634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707638: 386A1218  addi r3, r10, 0x1218
	ctx.r[3].s64 = ctx.r[10].s64 + 4632;
	// 8270763C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82707640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270764C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707654: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82707658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270765C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707660: 4BD5F7C1  bl 0x82466e20
	ctx.lr = 0x82707664;
	sub_82466E20(ctx, base);
	// 82707664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270766C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707678 size=112
    let mut pc: u32 = 0x82707678;
    'dispatch: loop {
        match pc {
            0x82707678 => {
    //   block [0x82707678..0x827076E8)
	// 82707678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270767C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707684: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82707688: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270768C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82707690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707694: 390B1800  addi r8, r11, 0x1800
	ctx.r[8].s64 = ctx.r[11].s64 + 6144;
	// 82707698: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270769C: 388AC930  addi r4, r10, -0x36d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14032;
	// 827076A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827076A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827076A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827076AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827076B0: 386A11E8  addi r3, r10, 0x11e8
	ctx.r[3].s64 = ctx.r[10].s64 + 4584;
	// 827076B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827076B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827076BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827076C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827076C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827076C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827076CC: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 827076D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827076D4: 4BD5F74D  bl 0x82466e20
	ctx.lr = 0x827076D8;
	sub_82466E20(ctx, base);
	// 827076D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827076DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827076E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827076E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827076E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827076E8 size=92
    let mut pc: u32 = 0x827076E8;
    'dispatch: loop {
        match pc {
            0x827076E8 => {
    //   block [0x827076E8..0x82707744)
	// 827076E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827076EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827076F0: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827076F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827076F8: 4BDF70C9  bl 0x824fe7c0
	ctx.lr = 0x827076FC;
	sub_824FE7C0(ctx, base);
	// 827076FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82707700: 3CE0824D  lis r7, -0x7db3
	ctx.r[7].s64 = -2108882944;
	// 82707704: 394BC968  addi r10, r11, -0x3698
	ctx.r[10].s64 = ctx.r[11].s64 + -13976;
	// 82707708: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270770C: 3D00824D  lis r8, -0x7db3
	ctx.r[8].s64 = -2108882944;
	// 82707710: 392B18C4  addi r9, r11, 0x18c4
	ctx.r[9].s64 = ctx.r[11].s64 + 6340;
	// 82707714: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707718: 396B1278  addi r11, r11, 0x1278
	ctx.r[11].s64 = ctx.r[11].s64 + 4728;
	// 8270771C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82707720: 39476E28  addi r10, r7, 0x6e28
	ctx.r[10].s64 = ctx.r[7].s64 + 28200;
	// 82707724: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82707728: 39486E10  addi r10, r8, 0x6e10
	ctx.r[10].s64 = ctx.r[8].s64 + 28176;
	// 8270772C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82707730: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82707734: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82707738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270773C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707748 size=112
    let mut pc: u32 = 0x82707748;
    'dispatch: loop {
        match pc {
            0x82707748 => {
    //   block [0x82707748..0x827077B8)
	// 82707748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270774C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707754: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707758: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270775C: 38AA1870  addi r5, r10, 0x1870
	ctx.r[5].s64 = ctx.r[10].s64 + 6256;
	// 82707760: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707764: 390B1858  addi r8, r11, 0x1858
	ctx.r[8].s64 = ctx.r[11].s64 + 6232;
	// 82707768: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270776C: 388AC968  addi r4, r10, -0x3698
	ctx.r[4].s64 = ctx.r[10].s64 + -13976;
	// 82707770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707774: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707778: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270777C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707780: 386A1288  addi r3, r10, 0x1288
	ctx.r[3].s64 = ctx.r[10].s64 + 4744;
	// 82707784: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270778C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270779C: 38C000B4  li r6, 0xb4
	ctx.r[6].s64 = 180;
	// 827077A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827077A4: 4BD5F67D  bl 0x82466e20
	ctx.lr = 0x827077A8;
	sub_82466E20(ctx, base);
	// 827077A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827077AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827077B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827077B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827077B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827077B8 size=108
    let mut pc: u32 = 0x827077B8;
    'dispatch: loop {
        match pc {
            0x827077B8 => {
    //   block [0x827077B8..0x82707824)
	// 827077B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827077BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827077C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827077C4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827077C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827077CC: 38EB18D0  addi r7, r11, 0x18d0
	ctx.r[7].s64 = ctx.r[11].s64 + 6352;
	// 827077D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 827077D4: 388AC8B0  addi r4, r10, -0x3750
	ctx.r[4].s64 = ctx.r[10].s64 + -14160;
	// 827077D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827077DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827077E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827077E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827077E8: 386A12C8  addi r3, r10, 0x12c8
	ctx.r[3].s64 = ctx.r[10].s64 + 4808;
	// 827077EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827077F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827077F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827077F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827077FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707804: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82707808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270780C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707810: 4BD5F611  bl 0x82466e20
	ctx.lr = 0x82707814;
	sub_82466E20(ctx, base);
	// 82707814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270781C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707828 size=92
    let mut pc: u32 = 0x82707828;
    'dispatch: loop {
        match pc {
            0x82707828 => {
    //   block [0x82707828..0x82707884)
	// 82707828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270782C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707830: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707834: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82707838: 4BDF6F89  bl 0x824fe7c0
	ctx.lr = 0x8270783C;
	sub_824FE7C0(ctx, base);
	// 8270783C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82707840: 3CE0824D  lis r7, -0x7db3
	ctx.r[7].s64 = -2108882944;
	// 82707844: 394BC8D8  addi r10, r11, -0x3728
	ctx.r[10].s64 = ctx.r[11].s64 + -14120;
	// 82707848: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270784C: 3D00824D  lis r8, -0x7db3
	ctx.r[8].s64 = -2108882944;
	// 82707850: 392B1984  addi r9, r11, 0x1984
	ctx.r[9].s64 = ctx.r[11].s64 + 6532;
	// 82707854: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707858: 396B12B8  addi r11, r11, 0x12b8
	ctx.r[11].s64 = ctx.r[11].s64 + 4792;
	// 8270785C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82707860: 39476FC0  addi r10, r7, 0x6fc0
	ctx.r[10].s64 = ctx.r[7].s64 + 28608;
	// 82707864: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82707868: 39486FA8  addi r10, r8, 0x6fa8
	ctx.r[10].s64 = ctx.r[8].s64 + 28584;
	// 8270786C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82707870: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82707874: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82707878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270787C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707888 size=112
    let mut pc: u32 = 0x82707888;
    'dispatch: loop {
        match pc {
            0x82707888 => {
    //   block [0x82707888..0x827078F8)
	// 82707888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270788C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707894: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707898: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270789C: 38AA1870  addi r5, r10, 0x1870
	ctx.r[5].s64 = ctx.r[10].s64 + 6256;
	// 827078A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827078A4: 390B1900  addi r8, r11, 0x1900
	ctx.r[8].s64 = ctx.r[11].s64 + 6400;
	// 827078A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827078AC: 388AC8D8  addi r4, r10, -0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + -14120;
	// 827078B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827078B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827078B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827078BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827078C0: 386A12F8  addi r3, r10, 0x12f8
	ctx.r[3].s64 = ctx.r[10].s64 + 4856;
	// 827078C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827078C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827078CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827078D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827078D4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827078D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827078DC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 827078E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827078E4: 4BD5F53D  bl 0x82466e20
	ctx.lr = 0x827078E8;
	sub_82466E20(ctx, base);
	// 827078E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827078EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827078F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827078F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827078F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827078F8 size=92
    let mut pc: u32 = 0x827078F8;
    'dispatch: loop {
        match pc {
            0x827078F8 => {
    //   block [0x827078F8..0x82707954)
	// 827078F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827078FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707900: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707904: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82707908: 4BDF6EB9  bl 0x824fe7c0
	ctx.lr = 0x8270790C;
	sub_824FE7C0(ctx, base);
	// 8270790C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82707910: 3CE0824D  lis r7, -0x7db3
	ctx.r[7].s64 = -2108882944;
	// 82707914: 394BC948  addi r10, r11, -0x36b8
	ctx.r[10].s64 = ctx.r[11].s64 + -14008;
	// 82707918: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270791C: 3D00824D  lis r8, -0x7db3
	ctx.r[8].s64 = -2108882944;
	// 82707920: 392B1A00  addi r9, r11, 0x1a00
	ctx.r[9].s64 = ctx.r[11].s64 + 6656;
	// 82707924: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707928: 396B1328  addi r11, r11, 0x1328
	ctx.r[11].s64 = ctx.r[11].s64 + 4904;
	// 8270792C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82707930: 39477148  addi r10, r7, 0x7148
	ctx.r[10].s64 = ctx.r[7].s64 + 29000;
	// 82707934: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82707938: 39487130  addi r10, r8, 0x7130
	ctx.r[10].s64 = ctx.r[8].s64 + 28976;
	// 8270793C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82707940: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82707944: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82707948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270794C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707958 size=112
    let mut pc: u32 = 0x82707958;
    'dispatch: loop {
        match pc {
            0x82707958 => {
    //   block [0x82707958..0x827079C8)
	// 82707958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270795C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707964: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707968: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270796C: 38AA1870  addi r5, r10, 0x1870
	ctx.r[5].s64 = ctx.r[10].s64 + 6256;
	// 82707970: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707974: 390B1994  addi r8, r11, 0x1994
	ctx.r[8].s64 = ctx.r[11].s64 + 6548;
	// 82707978: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270797C: 388AC948  addi r4, r10, -0x36b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14008;
	// 82707980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707984: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707988: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270798C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707990: 386A1338  addi r3, r10, 0x1338
	ctx.r[3].s64 = ctx.r[10].s64 + 4920;
	// 82707994: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270799C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827079A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827079A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827079A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827079AC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 827079B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827079B4: 4BD5F46D  bl 0x82466e20
	ctx.lr = 0x827079B8;
	sub_82466E20(ctx, base);
	// 827079B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827079BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827079C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827079C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827079C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827079C8 size=112
    let mut pc: u32 = 0x827079C8;
    'dispatch: loop {
        match pc {
            0x827079C8 => {
    //   block [0x827079C8..0x82707A38)
	// 827079C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827079CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827079D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827079D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827079D8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827079DC: 38AA06B4  addi r5, r10, 0x6b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1716;
	// 827079E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827079E4: 390B1A10  addi r8, r11, 0x1a10
	ctx.r[8].s64 = ctx.r[11].s64 + 6672;
	// 827079E8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 827079EC: 388AC830  addi r4, r10, -0x37d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14288;
	// 827079F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827079F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827079F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827079FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707A00: 386A1368  addi r3, r10, 0x1368
	ctx.r[3].s64 = ctx.r[10].s64 + 4968;
	// 82707A04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707A08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707A10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707A18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707A1C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82707A20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707A24: 4BD5F3FD  bl 0x82466e20
	ctx.lr = 0x82707A28;
	sub_82466E20(ctx, base);
	// 82707A28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707A38 size=112
    let mut pc: u32 = 0x82707A38;
    'dispatch: loop {
        match pc {
            0x82707A38 => {
    //   block [0x82707A38..0x82707AA8)
	// 82707A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707A44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82707A48: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707A4C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82707A50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707A54: 390B1AB8  addi r8, r11, 0x1ab8
	ctx.r[8].s64 = ctx.r[11].s64 + 6840;
	// 82707A58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82707A5C: 388ACADC  addi r4, r10, -0x3524
	ctx.r[4].s64 = ctx.r[10].s64 + -13604;
	// 82707A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707A64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707A68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707A70: 386A1398  addi r3, r10, 0x1398
	ctx.r[3].s64 = ctx.r[10].s64 + 5016;
	// 82707A74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707A78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707A8C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82707A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707A94: 4BD5F38D  bl 0x82466e20
	ctx.lr = 0x82707A98;
	sub_82466E20(ctx, base);
	// 82707A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707AA8 size=108
    let mut pc: u32 = 0x82707AA8;
    'dispatch: loop {
        match pc {
            0x82707AA8 => {
    //   block [0x82707AA8..0x82707B14)
	// 82707AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707AB4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707AB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707ABC: 38EB1AE0  addi r7, r11, 0x1ae0
	ctx.r[7].s64 = ctx.r[11].s64 + 6880;
	// 82707AC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82707AC4: 388ACAFC  addi r4, r10, -0x3504
	ctx.r[4].s64 = ctx.r[10].s64 + -13572;
	// 82707AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707ACC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707AD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82707AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707AD8: 386A13F8  addi r3, r10, 0x13f8
	ctx.r[3].s64 = ctx.r[10].s64 + 5112;
	// 82707ADC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82707AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707AE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707AF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707AF4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82707AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707AFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707B00: 4BD5F321  bl 0x82466e20
	ctx.lr = 0x82707B04;
	sub_82466E20(ctx, base);
	// 82707B04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707B18 size=112
    let mut pc: u32 = 0x82707B18;
    'dispatch: loop {
        match pc {
            0x82707B18 => {
    //   block [0x82707B18..0x82707B88)
	// 82707B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707B24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82707B28: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707B2C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82707B30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707B34: 390B1B10  addi r8, r11, 0x1b10
	ctx.r[8].s64 = ctx.r[11].s64 + 6928;
	// 82707B38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82707B3C: 388ACB34  addi r4, r10, -0x34cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13516;
	// 82707B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707B44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707B50: 386A13C8  addi r3, r10, 0x13c8
	ctx.r[3].s64 = ctx.r[10].s64 + 5064;
	// 82707B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707B6C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82707B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707B74: 4BD5F2AD  bl 0x82466e20
	ctx.lr = 0x82707B78;
	sub_82466E20(ctx, base);
	// 82707B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707B88 size=112
    let mut pc: u32 = 0x82707B88;
    'dispatch: loop {
        match pc {
            0x82707B88 => {
    //   block [0x82707B88..0x82707BF8)
	// 82707B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707B94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707B98: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707B9C: 38AA07E4  addi r5, r10, 0x7e4
	ctx.r[5].s64 = ctx.r[10].s64 + 2020;
	// 82707BA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707BA4: 390B1B38  addi r8, r11, 0x1b38
	ctx.r[8].s64 = ctx.r[11].s64 + 6968;
	// 82707BA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82707BAC: 388AC81C  addi r4, r10, -0x37e4
	ctx.r[4].s64 = ctx.r[10].s64 + -14308;
	// 82707BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707BB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707BB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707BC0: 386A1428  addi r3, r10, 0x1428
	ctx.r[3].s64 = ctx.r[10].s64 + 5160;
	// 82707BC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707BDC: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82707BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707BE4: 4BD5F23D  bl 0x82466e20
	ctx.lr = 0x82707BE8;
	sub_82466E20(ctx, base);
	// 82707BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707BF8 size=112
    let mut pc: u32 = 0x82707BF8;
    'dispatch: loop {
        match pc {
            0x82707BF8 => {
    //   block [0x82707BF8..0x82707C68)
	// 82707BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707C04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82707C08: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707C0C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82707C10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707C14: 390B1BBC  addi r8, r11, 0x1bbc
	ctx.r[8].s64 = ctx.r[11].s64 + 7100;
	// 82707C18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82707C1C: 388ACAC0  addi r4, r10, -0x3540
	ctx.r[4].s64 = ctx.r[10].s64 + -13632;
	// 82707C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707C24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707C30: 386A1458  addi r3, r10, 0x1458
	ctx.r[3].s64 = ctx.r[10].s64 + 5208;
	// 82707C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707C4C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82707C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707C54: 4BD5F1CD  bl 0x82466e20
	ctx.lr = 0x82707C58;
	sub_82466E20(ctx, base);
	// 82707C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707C68 size=92
    let mut pc: u32 = 0x82707C68;
    'dispatch: loop {
        match pc {
            0x82707C68 => {
    //   block [0x82707C68..0x82707CC4)
	// 82707C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707C70: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707C74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82707C78: 4BDF6B49  bl 0x824fe7c0
	ctx.lr = 0x82707C7C;
	sub_824FE7C0(ctx, base);
	// 82707C7C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82707C80: 3CE0824E  lis r7, -0x7db2
	ctx.r[7].s64 = -2108817408;
	// 82707C84: 394BC894  addi r10, r11, -0x376c
	ctx.r[10].s64 = ctx.r[11].s64 + -14188;
	// 82707C88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707C8C: 3D00824E  lis r8, -0x7db2
	ctx.r[8].s64 = -2108817408;
	// 82707C90: 392B1C64  addi r9, r11, 0x1c64
	ctx.r[9].s64 = ctx.r[11].s64 + 7268;
	// 82707C94: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707C98: 396B1488  addi r11, r11, 0x1488
	ctx.r[11].s64 = ctx.r[11].s64 + 5256;
	// 82707C9C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82707CA0: 3947C6E8  addi r10, r7, -0x3918
	ctx.r[10].s64 = ctx.r[7].s64 + -14616;
	// 82707CA4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82707CA8: 3948C6D0  addi r10, r8, -0x3930
	ctx.r[10].s64 = ctx.r[8].s64 + -14640;
	// 82707CAC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82707CB0: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82707CB4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82707CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707CC8 size=112
    let mut pc: u32 = 0x82707CC8;
    'dispatch: loop {
        match pc {
            0x82707CC8 => {
    //   block [0x82707CC8..0x82707D38)
	// 82707CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707CD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707CD8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707CDC: 38AA1870  addi r5, r10, 0x1870
	ctx.r[5].s64 = ctx.r[10].s64 + 6256;
	// 82707CE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707CE4: 390B1BF0  addi r8, r11, 0x1bf0
	ctx.r[8].s64 = ctx.r[11].s64 + 7152;
	// 82707CE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82707CEC: 388AC894  addi r4, r10, -0x376c
	ctx.r[4].s64 = ctx.r[10].s64 + -14188;
	// 82707CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707CF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707D00: 386A1498  addi r3, r10, 0x1498
	ctx.r[3].s64 = ctx.r[10].s64 + 5272;
	// 82707D04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707D14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82707D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707D1C: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 82707D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707D24: 4BD5F0FD  bl 0x82466e20
	ctx.lr = 0x82707D28;
	sub_82466E20(ctx, base);
	// 82707D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707D38 size=112
    let mut pc: u32 = 0x82707D38;
    'dispatch: loop {
        match pc {
            0x82707D38 => {
    //   block [0x82707D38..0x82707DA8)
	// 82707D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707D44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707D48: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707D4C: 38AA06B4  addi r5, r10, 0x6b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1716;
	// 82707D50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707D54: 390B1C70  addi r8, r11, 0x1c70
	ctx.r[8].s64 = ctx.r[11].s64 + 7280;
	// 82707D58: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82707D5C: 388AC858  addi r4, r10, -0x37a8
	ctx.r[4].s64 = ctx.r[10].s64 + -14248;
	// 82707D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707D64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707D70: 386A14C8  addi r3, r10, 0x14c8
	ctx.r[3].s64 = ctx.r[10].s64 + 5320;
	// 82707D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707D8C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82707D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707D94: 4BD5F08D  bl 0x82466e20
	ctx.lr = 0x82707D98;
	sub_82466E20(ctx, base);
	// 82707D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707DA8 size=112
    let mut pc: u32 = 0x82707DA8;
    'dispatch: loop {
        match pc {
            0x82707DA8 => {
    //   block [0x82707DA8..0x82707E18)
	// 82707DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707DB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82707DB8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707DBC: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82707DC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707DC4: 390B1CF4  addi r8, r11, 0x1cf4
	ctx.r[8].s64 = ctx.r[11].s64 + 7412;
	// 82707DC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82707DCC: 388ACAB0  addi r4, r10, -0x3550
	ctx.r[4].s64 = ctx.r[10].s64 + -13648;
	// 82707DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707DD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707DE0: 386A14F8  addi r3, r10, 0x14f8
	ctx.r[3].s64 = ctx.r[10].s64 + 5368;
	// 82707DE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707DFC: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 82707E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707E04: 4BD5F01D  bl 0x82466e20
	ctx.lr = 0x82707E08;
	sub_82466E20(ctx, base);
	// 82707E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707E18 size=112
    let mut pc: u32 = 0x82707E18;
    'dispatch: loop {
        match pc {
            0x82707E18 => {
    //   block [0x82707E18..0x82707E88)
	// 82707E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707E24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707E28: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707E2C: 38AA07E4  addi r5, r10, 0x7e4
	ctx.r[5].s64 = ctx.r[10].s64 + 2020;
	// 82707E30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707E34: 390B1D38  addi r8, r11, 0x1d38
	ctx.r[8].s64 = ctx.r[11].s64 + 7480;
	// 82707E38: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82707E3C: 388AC86C  addi r4, r10, -0x3794
	ctx.r[4].s64 = ctx.r[10].s64 + -14228;
	// 82707E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707E44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707E48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707E50: 386A1528  addi r3, r10, 0x1528
	ctx.r[3].s64 = ctx.r[10].s64 + 5416;
	// 82707E54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707E6C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82707E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707E74: 4BD5EFAD  bl 0x82466e20
	ctx.lr = 0x82707E78;
	sub_82466E20(ctx, base);
	// 82707E78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707E88 size=100
    let mut pc: u32 = 0x82707E88;
    'dispatch: loop {
        match pc {
            0x82707E88 => {
    //   block [0x82707E88..0x82707EEC)
	// 82707E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707E90: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707E94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707E98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82707E9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82707EA0: 4BEA6B61  bl 0x825aea00
	ctx.lr = 0x82707EA4;
	sub_825AEA00(ctx, base);
	// 82707EA4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82707EA8: 3CE0824E  lis r7, -0x7db2
	ctx.r[7].s64 = -2108817408;
	// 82707EAC: 394BC840  addi r10, r11, -0x37c0
	ctx.r[10].s64 = ctx.r[11].s64 + -14272;
	// 82707EB0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707EB4: 3D00824E  lis r8, -0x7db2
	ctx.r[8].s64 = -2108817408;
	// 82707EB8: 392B1EE4  addi r9, r11, 0x1ee4
	ctx.r[9].s64 = ctx.r[11].s64 + 7908;
	// 82707EBC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707EC0: 396B1588  addi r11, r11, 0x1588
	ctx.r[11].s64 = ctx.r[11].s64 + 5512;
	// 82707EC4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82707EC8: 3947CA08  addi r10, r7, -0x35f8
	ctx.r[10].s64 = ctx.r[7].s64 + -13816;
	// 82707ECC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82707ED0: 3948C9F0  addi r10, r8, -0x3610
	ctx.r[10].s64 = ctx.r[8].s64 + -13840;
	// 82707ED4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82707ED8: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82707EDC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82707EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707EF0 size=112
    let mut pc: u32 = 0x82707EF0;
    'dispatch: loop {
        match pc {
            0x82707EF0 => {
    //   block [0x82707EF0..0x82707F60)
	// 82707EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707EFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707F00: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707F04: 38AA06B4  addi r5, r10, 0x6b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1716;
	// 82707F08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707F0C: 390B1E20  addi r8, r11, 0x1e20
	ctx.r[8].s64 = ctx.r[11].s64 + 7712;
	// 82707F10: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82707F14: 388AC840  addi r4, r10, -0x37c0
	ctx.r[4].s64 = ctx.r[10].s64 + -14272;
	// 82707F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707F1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707F20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707F28: 386A1558  addi r3, r10, 0x1558
	ctx.r[3].s64 = ctx.r[10].s64 + 5464;
	// 82707F2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707F30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707F44: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82707F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707F4C: 4BD5EED5  bl 0x82466e20
	ctx.lr = 0x82707F50;
	sub_82466E20(ctx, base);
	// 82707F50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707F60 size=112
    let mut pc: u32 = 0x82707F60;
    'dispatch: loop {
        match pc {
            0x82707F60 => {
    //   block [0x82707F60..0x82707FD0)
	// 82707F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707F6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707F70: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707F74: 38AA07B4  addi r5, r10, 0x7b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1972;
	// 82707F78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707F7C: 390B1F04  addi r8, r11, 0x1f04
	ctx.r[8].s64 = ctx.r[11].s64 + 7940;
	// 82707F80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82707F84: 388AC980  addi r4, r10, -0x3680
	ctx.r[4].s64 = ctx.r[10].s64 + -13952;
	// 82707F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707F8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707F90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707F98: 386A1598  addi r3, r10, 0x1598
	ctx.r[3].s64 = ctx.r[10].s64 + 5528;
	// 82707F9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707FB4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82707FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707FBC: 4BD5EE65  bl 0x82466e20
	ctx.lr = 0x82707FC0;
	sub_82466E20(ctx, base);
	// 82707FC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707FD0 size=108
    let mut pc: u32 = 0x82707FD0;
    'dispatch: loop {
        match pc {
            0x82707FD0 => {
    //   block [0x82707FD0..0x8270803C)
	// 82707FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707FDC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707FE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707FE4: 396B1F38  addi r11, r11, 0x1f38
	ctx.r[11].s64 = ctx.r[11].s64 + 7992;
	// 82707FE8: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 82707FEC: 390B01C8  addi r8, r11, 0x1c8
	ctx.r[8].s64 = ctx.r[11].s64 + 456;
	// 82707FF0: 388AC87C  addi r4, r10, -0x3784
	ctx.r[4].s64 = ctx.r[10].s64 + -14212;
	// 82707FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707FF8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707FFC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82708000: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82708004: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 82708008: 386A15C8  addi r3, r10, 0x15c8
	ctx.r[3].s64 = ctx.r[10].s64 + 5576;
	// 8270800C: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82708010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708018: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8270801C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708020: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82708024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82708028: 4BD5EDF9  bl 0x82466e20
	ctx.lr = 0x8270802C;
	sub_82466E20(ctx, base);
	// 8270802C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708040 size=116
    let mut pc: u32 = 0x82708040;
    'dispatch: loop {
        match pc {
            0x82708040 => {
    //   block [0x82708040..0x827080B4)
	// 82708040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270804C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708050: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82708054: 390B28E4  addi r8, r11, 0x28e4
	ctx.r[8].s64 = ctx.r[11].s64 + 10468;
	// 82708058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270805C: 392A28D0  addi r9, r10, 0x28d0
	ctx.r[9].s64 = ctx.r[10].s64 + 10448;
	// 82708060: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708064: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82708068: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 8270806C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708074: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270807C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708084: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708088: 388AB7E0  addi r4, r10, -0x4820
	ctx.r[4].s64 = ctx.r[10].s64 + -18464;
	// 8270808C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82708090: 386B1600  addi r3, r11, 0x1600
	ctx.r[3].s64 = ctx.r[11].s64 + 5632;
	// 82708094: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82708098: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270809C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 827080A0: 4BD5ED81  bl 0x82466e20
	ctx.lr = 0x827080A4;
	sub_82466E20(ctx, base);
	// 827080A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827080A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827080AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827080B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827080B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827080B8 size=92
    let mut pc: u32 = 0x827080B8;
    'dispatch: loop {
        match pc {
            0x827080B8 => {
    //   block [0x827080B8..0x82708114)
	// 827080B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827080BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827080C0: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827080C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827080C8: 4BDF66F9  bl 0x824fe7c0
	ctx.lr = 0x827080CC;
	sub_824FE7C0(ctx, base);
	// 827080CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827080D0: 3CE08250  lis r7, -0x7db0
	ctx.r[7].s64 = -2108686336;
	// 827080D4: 394BB4A4  addi r10, r11, -0x4b5c
	ctx.r[10].s64 = ctx.r[11].s64 + -19292;
	// 827080D8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827080DC: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 827080E0: 392B29E4  addi r9, r11, 0x29e4
	ctx.r[9].s64 = ctx.r[11].s64 + 10724;
	// 827080E4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827080E8: 396B1630  addi r11, r11, 0x1630
	ctx.r[11].s64 = ctx.r[11].s64 + 5680;
	// 827080EC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827080F0: 39479560  addi r10, r7, -0x6aa0
	ctx.r[10].s64 = ctx.r[7].s64 + -27296;
	// 827080F4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827080F8: 39489548  addi r10, r8, -0x6ab8
	ctx.r[10].s64 = ctx.r[8].s64 + -27320;
	// 827080FC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82708100: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82708104: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82708108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270810C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708118 size=112
    let mut pc: u32 = 0x82708118;
    'dispatch: loop {
        match pc {
            0x82708118 => {
    //   block [0x82708118..0x82708188)
	// 82708118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270811C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708124: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708128: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270812C: 38AA1870  addi r5, r10, 0x1870
	ctx.r[5].s64 = ctx.r[10].s64 + 6256;
	// 82708130: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708134: 390B2960  addi r8, r11, 0x2960
	ctx.r[8].s64 = ctx.r[11].s64 + 10592;
	// 82708138: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270813C: 388AB4A4  addi r4, r10, -0x4b5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19292;
	// 82708140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708144: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270814C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708150: 386A1640  addi r3, r10, 0x1640
	ctx.r[3].s64 = ctx.r[10].s64 + 5696;
	// 82708154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270815C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270816C: 38C00100  li r6, 0x100
	ctx.r[6].s64 = 256;
	// 82708170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708174: 4BD5ECAD  bl 0x82466e20
	ctx.lr = 0x82708178;
	sub_82466E20(ctx, base);
	// 82708178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270817C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708188 size=112
    let mut pc: u32 = 0x82708188;
    'dispatch: loop {
        match pc {
            0x82708188 => {
    //   block [0x82708188..0x827081F8)
	// 82708188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270818C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708194: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708198: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270819C: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 827081A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827081A4: 390B29F8  addi r8, r11, 0x29f8
	ctx.r[8].s64 = ctx.r[11].s64 + 10744;
	// 827081A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 827081AC: 388ABA38  addi r4, r10, -0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + -17864;
	// 827081B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827081B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827081B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827081BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827081C0: 386A1670  addi r3, r10, 0x1670
	ctx.r[3].s64 = ctx.r[10].s64 + 5744;
	// 827081C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827081C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827081CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827081D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827081D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827081D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827081DC: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 827081E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827081E4: 4BD5EC3D  bl 0x82466e20
	ctx.lr = 0x827081E8;
	sub_82466E20(ctx, base);
	// 827081E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827081EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827081F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827081F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827081F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827081F8 size=92
    let mut pc: u32 = 0x827081F8;
    'dispatch: loop {
        match pc {
            0x827081F8 => {
    //   block [0x827081F8..0x82708254)
	// 827081F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827081FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708200: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708204: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82708208: 4BDF65B9  bl 0x824fe7c0
	ctx.lr = 0x8270820C;
	sub_824FE7C0(ctx, base);
	// 8270820C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82708210: 3CE08250  lis r7, -0x7db0
	ctx.r[7].s64 = -2108686336;
	// 82708214: 394BB4B4  addi r10, r11, -0x4b4c
	ctx.r[10].s64 = ctx.r[11].s64 + -19276;
	// 82708218: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270821C: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82708220: 392B2B04  addi r9, r11, 0x2b04
	ctx.r[9].s64 = ctx.r[11].s64 + 11012;
	// 82708224: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708228: 396B16A0  addi r11, r11, 0x16a0
	ctx.r[11].s64 = ctx.r[11].s64 + 5792;
	// 8270822C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82708230: 39479818  addi r10, r7, -0x67e8
	ctx.r[10].s64 = ctx.r[7].s64 + -26600;
	// 82708234: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82708238: 39489800  addi r10, r8, -0x6800
	ctx.r[10].s64 = ctx.r[8].s64 + -26624;
	// 8270823C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82708240: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82708244: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82708248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270824C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708258 size=112
    let mut pc: u32 = 0x82708258;
    'dispatch: loop {
        match pc {
            0x82708258 => {
    //   block [0x82708258..0x827082C8)
	// 82708258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270825C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708264: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708268: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270826C: 38AA1870  addi r5, r10, 0x1870
	ctx.r[5].s64 = ctx.r[10].s64 + 6256;
	// 82708270: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708274: 390B2AB0  addi r8, r11, 0x2ab0
	ctx.r[8].s64 = ctx.r[11].s64 + 10928;
	// 82708278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270827C: 388AB4B4  addi r4, r10, -0x4b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19276;
	// 82708280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708284: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270828C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708290: 386A16B0  addi r3, r10, 0x16b0
	ctx.r[3].s64 = ctx.r[10].s64 + 5808;
	// 82708294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270829C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827082A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827082A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827082A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827082AC: 38C0003C  li r6, 0x3c
	ctx.r[6].s64 = 60;
	// 827082B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827082B4: 4BD5EB6D  bl 0x82466e20
	ctx.lr = 0x827082B8;
	sub_82466E20(ctx, base);
	// 827082B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827082BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827082C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827082C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827082C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827082C8 size=36
    let mut pc: u32 = 0x827082C8;
    'dispatch: loop {
        match pc {
            0x827082C8 => {
    //   block [0x827082C8..0x827082EC)
	// 827082C8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827082CC: 814BC540  lwz r10, -0x3ac0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15040 as u32) ) } as u64;
	// 827082D0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827082D4: 396BC578  addi r11, r11, -0x3a88
	ctx.r[11].s64 = ctx.r[11].s64 + -14984;
	// 827082D8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827082DC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827082E0: 814AC538  lwz r10, -0x3ac8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-15048 as u32) ) } as u64;
	// 827082E4: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 827082E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827082F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827082F0 size=108
    let mut pc: u32 = 0x827082F0;
    'dispatch: loop {
        match pc {
            0x827082F0 => {
    //   block [0x827082F0..0x8270835C)
	// 827082F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827082F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827082F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827082FC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82708300: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708304: 38EBC578  addi r7, r11, -0x3a88
	ctx.r[7].s64 = ctx.r[11].s64 + -14984;
	// 82708308: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8270830C: 388AB548  addi r4, r10, -0x4ab8
	ctx.r[4].s64 = ctx.r[10].s64 + -19128;
	// 82708310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708314: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708318: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270831C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708320: 386A16E0  addi r3, r10, 0x16e0
	ctx.r[3].s64 = ctx.r[10].s64 + 5856;
	// 82708324: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82708328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270832C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270833C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82708340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708344: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82708348: 4BD5EAD9  bl 0x82466e20
	ctx.lr = 0x8270834C;
	sub_82466E20(ctx, base);
	// 8270834C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82708360 size=24
    let mut pc: u32 = 0x82708360;
    'dispatch: loop {
        match pc {
            0x82708360 => {
    //   block [0x82708360..0x82708378)
	// 82708360: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82708364: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82708368: 394AC620  addi r10, r10, -0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + -14816;
	// 8270836C: 816BC538  lwz r11, -0x3ac8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15048 as u32) ) } as u64;
	// 82708370: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82708374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708378 size=116
    let mut pc: u32 = 0x82708378;
    'dispatch: loop {
        match pc {
            0x82708378 => {
    //   block [0x82708378..0x827083EC)
	// 82708378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270837C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708384: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82708388: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8270838C: 390AC620  addi r8, r10, -0x39e0
	ctx.r[8].s64 = ctx.r[10].s64 + -14816;
	// 82708390: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708394: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708398: 38AA16E0  addi r5, r10, 0x16e0
	ctx.r[5].s64 = ctx.r[10].s64 + 5856;
	// 8270839C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827083A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827083A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827083A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827083AC: 388AB57C  addi r4, r10, -0x4a84
	ctx.r[4].s64 = ctx.r[10].s64 + -19076;
	// 827083B0: 396B2B9C  addi r11, r11, 0x2b9c
	ctx.r[11].s64 = ctx.r[11].s64 + 11164;
	// 827083B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827083B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827083BC: 386A1710  addi r3, r10, 0x1710
	ctx.r[3].s64 = ctx.r[10].s64 + 5904;
	// 827083C0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 827083C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827083C8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 827083CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827083D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827083D4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 827083D8: 4BD5EA49  bl 0x82466e20
	ctx.lr = 0x827083DC;
	sub_82466E20(ctx, base);
	// 827083DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827083E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827083E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827083E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827083F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827083F0 size=112
    let mut pc: u32 = 0x827083F0;
    'dispatch: loop {
        match pc {
            0x827083F0 => {
    //   block [0x827083F0..0x82708460)
	// 827083F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827083F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827083F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827083FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708400: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708404: 38AA16E0  addi r5, r10, 0x16e0
	ctx.r[5].s64 = ctx.r[10].s64 + 5856;
	// 82708408: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270840C: 390B2BC8  addi r8, r11, 0x2bc8
	ctx.r[8].s64 = ctx.r[11].s64 + 11208;
	// 82708410: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82708414: 388AB5A4  addi r4, r10, -0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19036;
	// 82708418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270841C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708420: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708428: 386A1740  addi r3, r10, 0x1740
	ctx.r[3].s64 = ctx.r[10].s64 + 5952;
	// 8270842C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270843C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708444: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82708448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270844C: 4BD5E9D5  bl 0x82466e20
	ctx.lr = 0x82708450;
	sub_82466E20(ctx, base);
	// 82708450: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270845C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708460 size=92
    let mut pc: u32 = 0x82708460;
    'dispatch: loop {
        match pc {
            0x82708460 => {
    //   block [0x82708460..0x827084BC)
	// 82708460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708468: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270846C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82708470: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82708474: 4BDFCB6D  bl 0x82504fe0
	ctx.lr = 0x82708478;
	sub_82504FE0(ctx, base);
	// 82708478: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8270847C: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82708480: 394BB5E4  addi r10, r11, -0x4a1c
	ctx.r[10].s64 = ctx.r[11].s64 + -18972;
	// 82708484: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708488: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 8270848C: 396B1770  addi r11, r11, 0x1770
	ctx.r[11].s64 = ctx.r[11].s64 + 6000;
	// 82708490: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82708494: 39489970  addi r10, r8, -0x6690
	ctx.r[10].s64 = ctx.r[8].s64 + -26256;
	// 82708498: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270849C: 39499988  addi r10, r9, -0x6678
	ctx.r[10].s64 = ctx.r[9].s64 + -26232;
	// 827084A0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827084A4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827084A8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827084AC: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 827084B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827084B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827084B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827084C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827084C0 size=24
    let mut pc: u32 = 0x827084C0;
    'dispatch: loop {
        match pc {
            0x827084C0 => {
    //   block [0x827084C0..0x827084D8)
	// 827084C0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827084C4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827084C8: 394AC710  addi r10, r10, -0x38f0
	ctx.r[10].s64 = ctx.r[10].s64 + -14576;
	// 827084CC: 816BCE44  lwz r11, -0x31bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12732 as u32) ) } as u64;
	// 827084D0: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 827084D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827084D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827084D8 size=116
    let mut pc: u32 = 0x827084D8;
    'dispatch: loop {
        match pc {
            0x827084D8 => {
    //   block [0x827084D8..0x8270854C)
	// 827084D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827084DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827084E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827084E4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827084E8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827084EC: 392B2B60  addi r9, r11, 0x2b60
	ctx.r[9].s64 = ctx.r[11].s64 + 11104;
	// 827084F0: 38AA22B4  addi r5, r10, 0x22b4
	ctx.r[5].s64 = ctx.r[10].s64 + 8884;
	// 827084F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827084F8: 38E900E0  addi r7, r9, 0xe0
	ctx.r[7].s64 = ctx.r[9].s64 + 224;
	// 827084FC: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 82708500: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82708504: 388AB5E4  addi r4, r10, -0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + -18972;
	// 82708508: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270850C: 396BC710  addi r11, r11, -0x38f0
	ctx.r[11].s64 = ctx.r[11].s64 + -14576;
	// 82708510: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82708514: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708518: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8270851C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708520: 386A1780  addi r3, r10, 0x1780
	ctx.r[3].s64 = ctx.r[10].s64 + 6016;
	// 82708524: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82708528: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8270852C: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 82708530: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82708534: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82708538: 4BD5E8E9  bl 0x82466e20
	ctx.lr = 0x8270853C;
	sub_82466E20(ctx, base);
	// 8270853C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708550 size=96
    let mut pc: u32 = 0x82708550;
    'dispatch: loop {
        match pc {
            0x82708550 => {
    //   block [0x82708550..0x827085B0)
	// 82708550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270855C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708564: 388AB4F0  addi r4, r10, -0x4b10
	ctx.r[4].s64 = ctx.r[10].s64 + -19216;
	// 82708568: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270856C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708570: 386A17B0  addi r3, r10, 0x17b0
	ctx.r[3].s64 = ctx.r[10].s64 + 6064;
	// 82708574: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270857C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82708580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270858C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82708590: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82708594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82708598: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270859C: 4BD5E885  bl 0x82466e20
	ctx.lr = 0x827085A0;
	sub_82466E20(ctx, base);
	// 827085A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827085A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827085A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827085AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827085B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827085B0 size=112
    let mut pc: u32 = 0x827085B0;
    'dispatch: loop {
        match pc {
            0x827085B0 => {
    //   block [0x827085B0..0x82708620)
	// 827085B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827085B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827085B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827085BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827085C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827085C4: 38AA17B0  addi r5, r10, 0x17b0
	ctx.r[5].s64 = ctx.r[10].s64 + 6064;
	// 827085C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827085CC: 390B2C70  addi r8, r11, 0x2c70
	ctx.r[8].s64 = ctx.r[11].s64 + 11376;
	// 827085D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827085D4: 388AB504  addi r4, r10, -0x4afc
	ctx.r[4].s64 = ctx.r[10].s64 + -19196;
	// 827085D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827085DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827085E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827085E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827085E8: 386A17E0  addi r3, r10, 0x17e0
	ctx.r[3].s64 = ctx.r[10].s64 + 6112;
	// 827085EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827085F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827085F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827085F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827085FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708604: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82708608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270860C: 4BD5E815  bl 0x82466e20
	ctx.lr = 0x82708610;
	sub_82466E20(ctx, base);
	// 82708610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270861C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708620 size=112
    let mut pc: u32 = 0x82708620;
    'dispatch: loop {
        match pc {
            0x82708620 => {
    //   block [0x82708620..0x82708690)
	// 82708620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270862C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708630: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708634: 38AA2210  addi r5, r10, 0x2210
	ctx.r[5].s64 = ctx.r[10].s64 + 8720;
	// 82708638: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270863C: 390B2C88  addi r8, r11, 0x2c88
	ctx.r[8].s64 = ctx.r[11].s64 + 11400;
	// 82708640: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82708644: 388AB760  addi r4, r10, -0x48a0
	ctx.r[4].s64 = ctx.r[10].s64 + -18592;
	// 82708648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270864C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708658: 386A1810  addi r3, r10, 0x1810
	ctx.r[3].s64 = ctx.r[10].s64 + 6160;
	// 8270865C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270866C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708674: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82708678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270867C: 4BD5E7A5  bl 0x82466e20
	ctx.lr = 0x82708680;
	sub_82466E20(ctx, base);
	// 82708680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270868C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708690 size=112
    let mut pc: u32 = 0x82708690;
    'dispatch: loop {
        match pc {
            0x82708690 => {
    //   block [0x82708690..0x82708700)
	// 82708690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270869C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827086A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827086A4: 38AA1810  addi r5, r10, 0x1810
	ctx.r[5].s64 = ctx.r[10].s64 + 6160;
	// 827086A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827086AC: 390B2CE8  addi r8, r11, 0x2ce8
	ctx.r[8].s64 = ctx.r[11].s64 + 11496;
	// 827086B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827086B4: 388AB778  addi r4, r10, -0x4888
	ctx.r[4].s64 = ctx.r[10].s64 + -18568;
	// 827086B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827086BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827086C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827086C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827086C8: 386A1840  addi r3, r10, 0x1840
	ctx.r[3].s64 = ctx.r[10].s64 + 6208;
	// 827086CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827086D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827086D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827086D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827086DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827086E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827086E4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 827086E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827086EC: 4BD5E735  bl 0x82466e20
	ctx.lr = 0x827086F0;
	sub_82466E20(ctx, base);
	// 827086F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827086F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827086F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827086FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82708700 size=24
    let mut pc: u32 = 0x82708700;
    'dispatch: loop {
        match pc {
            0x82708700 => {
    //   block [0x82708700..0x82708718)
	// 82708700: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82708704: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82708708: 394AC8A8  addi r10, r10, -0x3758
	ctx.r[10].s64 = ctx.r[10].s64 + -14168;
	// 8270870C: 816BC8A0  lwz r11, -0x3760(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14176 as u32) ) } as u64;
	// 82708710: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82708714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708718 size=116
    let mut pc: u32 = 0x82708718;
    'dispatch: loop {
        match pc {
            0x82708718 => {
    //   block [0x82708718..0x8270878C)
	// 82708718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270871C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708724: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82708728: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8270872C: 390BC8A8  addi r8, r11, -0x3758
	ctx.r[8].s64 = ctx.r[11].s64 + -14168;
	// 82708730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708734: 392A2DB8  addi r9, r10, 0x2db8
	ctx.r[9].s64 = ctx.r[10].s64 + 11704;
	// 82708738: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270873C: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82708740: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82708744: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270874C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708754: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82708758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270875C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708760: 388AB42C  addi r4, r10, -0x4bd4
	ctx.r[4].s64 = ctx.r[10].s64 + -19412;
	// 82708764: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82708768: 386B1870  addi r3, r11, 0x1870
	ctx.r[3].s64 = ctx.r[11].s64 + 6256;
	// 8270876C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82708770: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708774: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82708778: 4BD5E6A9  bl 0x82466e20
	ctx.lr = 0x8270877C;
	sub_82466E20(ctx, base);
	// 8270877C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708790 size=92
    let mut pc: u32 = 0x82708790;
    'dispatch: loop {
        match pc {
            0x82708790 => {
    //   block [0x82708790..0x827087EC)
	// 82708790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708798: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270879C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827087A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827087A4: 4BDFDDD5  bl 0x82506578
	ctx.lr = 0x827087A8;
	sub_82506578(ctx, base);
	// 827087A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827087AC: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 827087B0: 394BB718  addi r10, r11, -0x48e8
	ctx.r[10].s64 = ctx.r[11].s64 + -18664;
	// 827087B4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827087B8: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 827087BC: 396B18A0  addi r11, r11, 0x18a0
	ctx.r[11].s64 = ctx.r[11].s64 + 6304;
	// 827087C0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827087C4: 39489C18  addi r10, r8, -0x63e8
	ctx.r[10].s64 = ctx.r[8].s64 + -25576;
	// 827087C8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827087CC: 39499C30  addi r10, r9, -0x63d0
	ctx.r[10].s64 = ctx.r[9].s64 + -25552;
	// 827087D0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827087D4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827087D8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827087DC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827087E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827087E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827087E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827087F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827087F0 size=112
    let mut pc: u32 = 0x827087F0;
    'dispatch: loop {
        match pc {
            0x827087F0 => {
    //   block [0x827087F0..0x82708860)
	// 827087F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827087F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827087F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827087FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82708800: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708804: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82708808: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270880C: 390B2DD0  addi r8, r11, 0x2dd0
	ctx.r[8].s64 = ctx.r[11].s64 + 11728;
	// 82708810: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82708814: 388AB718  addi r4, r10, -0x48e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18664;
	// 82708818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270881C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708828: 386A18B0  addi r3, r10, 0x18b0
	ctx.r[3].s64 = ctx.r[10].s64 + 6320;
	// 8270882C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270883C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708844: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82708848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270884C: 4BD5E5D5  bl 0x82466e20
	ctx.lr = 0x82708850;
	sub_82466E20(ctx, base);
	// 82708850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270885C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708860 size=92
    let mut pc: u32 = 0x82708860;
    'dispatch: loop {
        match pc {
            0x82708860 => {
    //   block [0x82708860..0x827088BC)
	// 82708860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708868: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270886C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82708870: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82708874: 4BDFDD35  bl 0x825065a8
	ctx.lr = 0x82708878;
	sub_825065A8(ctx, base);
	// 82708878: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8270887C: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82708880: 394BB73C  addi r10, r11, -0x48c4
	ctx.r[10].s64 = ctx.r[11].s64 + -18628;
	// 82708884: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708888: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 8270888C: 396B18E0  addi r11, r11, 0x18e0
	ctx.r[11].s64 = ctx.r[11].s64 + 6368;
	// 82708890: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82708894: 39489C78  addi r10, r8, -0x6388
	ctx.r[10].s64 = ctx.r[8].s64 + -25480;
	// 82708898: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270889C: 39499C90  addi r10, r9, -0x6370
	ctx.r[10].s64 = ctx.r[9].s64 + -25456;
	// 827088A0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827088A4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827088A8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827088AC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 827088B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827088B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827088B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827088C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827088C0 size=112
    let mut pc: u32 = 0x827088C0;
    'dispatch: loop {
        match pc {
            0x827088C0 => {
    //   block [0x827088C0..0x82708930)
	// 827088C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827088C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827088C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827088CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827088D0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827088D4: 38AA2120  addi r5, r10, 0x2120
	ctx.r[5].s64 = ctx.r[10].s64 + 8480;
	// 827088D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827088DC: 390B2E60  addi r8, r11, 0x2e60
	ctx.r[8].s64 = ctx.r[11].s64 + 11872;
	// 827088E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827088E4: 388AB73C  addi r4, r10, -0x48c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18628;
	// 827088E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827088EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827088F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827088F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827088F8: 386A18F0  addi r3, r10, 0x18f0
	ctx.r[3].s64 = ctx.r[10].s64 + 6384;
	// 827088FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270890C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708914: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 82708918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270891C: 4BD5E505  bl 0x82466e20
	ctx.lr = 0x82708920;
	sub_82466E20(ctx, base);
	// 82708920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270892C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708930 size=108
    let mut pc: u32 = 0x82708930;
    'dispatch: loop {
        match pc {
            0x82708930 => {
    //   block [0x82708930..0x8270899C)
	// 82708930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270893C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708940: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708944: 38EB2E78  addi r7, r11, 0x2e78
	ctx.r[7].s64 = ctx.r[11].s64 + 11896;
	// 82708948: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8270894C: 388AB610  addi r4, r10, -0x49f0
	ctx.r[4].s64 = ctx.r[10].s64 + -18928;
	// 82708950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708954: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270895C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708960: 386A1920  addi r3, r10, 0x1920
	ctx.r[3].s64 = ctx.r[10].s64 + 6432;
	// 82708964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82708968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270896C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270897C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82708980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82708988: 4BD5E499  bl 0x82466e20
	ctx.lr = 0x8270898C;
	sub_82466E20(ctx, base);
	// 8270898C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827089A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827089A0 size=92
    let mut pc: u32 = 0x827089A0;
    'dispatch: loop {
        match pc {
            0x827089A0 => {
    //   block [0x827089A0..0x827089FC)
	// 827089A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827089A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827089A8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827089AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827089B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827089B4: 4BDFE9F5  bl 0x825073a8
	ctx.lr = 0x827089B8;
	sub_825073A8(ctx, base);
	// 827089B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827089BC: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 827089C0: 394BB628  addi r10, r11, -0x49d8
	ctx.r[10].s64 = ctx.r[11].s64 + -18904;
	// 827089C4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827089C8: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 827089CC: 396B1950  addi r11, r11, 0x1950
	ctx.r[11].s64 = ctx.r[11].s64 + 6480;
	// 827089D0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827089D4: 39489CF8  addi r10, r8, -0x6308
	ctx.r[10].s64 = ctx.r[8].s64 + -25352;
	// 827089D8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827089DC: 39499CE0  addi r10, r9, -0x6320
	ctx.r[10].s64 = ctx.r[9].s64 + -25376;
	// 827089E0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827089E4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827089E8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827089EC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 827089F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827089F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827089F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708A00 size=112
    let mut pc: u32 = 0x82708A00;
    'dispatch: loop {
        match pc {
            0x82708A00 => {
    //   block [0x82708A00..0x82708A70)
	// 82708A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708A0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708A10: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708A14: 38AA22B4  addi r5, r10, 0x22b4
	ctx.r[5].s64 = ctx.r[10].s64 + 8884;
	// 82708A18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708A1C: 390B2ED8  addi r8, r11, 0x2ed8
	ctx.r[8].s64 = ctx.r[11].s64 + 11992;
	// 82708A20: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82708A24: 388AB628  addi r4, r10, -0x49d8
	ctx.r[4].s64 = ctx.r[10].s64 + -18904;
	// 82708A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708A2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708A30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708A38: 386A1960  addi r3, r10, 0x1960
	ctx.r[3].s64 = ctx.r[10].s64 + 6496;
	// 82708A3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708A54: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 82708A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708A5C: 4BD5E3C5  bl 0x82466e20
	ctx.lr = 0x82708A60;
	sub_82466E20(ctx, base);
	// 82708A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708A70 size=108
    let mut pc: u32 = 0x82708A70;
    'dispatch: loop {
        match pc {
            0x82708A70 => {
    //   block [0x82708A70..0x82708ADC)
	// 82708A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708A7C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708A80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708A84: 38EB2F38  addi r7, r11, 0x2f38
	ctx.r[7].s64 = ctx.r[11].s64 + 12088;
	// 82708A88: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82708A8C: 388ABA68  addi r4, r10, -0x4598
	ctx.r[4].s64 = ctx.r[10].s64 + -17816;
	// 82708A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708A94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708A98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82708A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708AA0: 386A1990  addi r3, r10, 0x1990
	ctx.r[3].s64 = ctx.r[10].s64 + 6544;
	// 82708AA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82708AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708ABC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82708AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708AC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82708AC8: 4BD5E359  bl 0x82466e20
	ctx.lr = 0x82708ACC;
	sub_82466E20(ctx, base);
	// 82708ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708AE0 size=100
    let mut pc: u32 = 0x82708AE0;
    'dispatch: loop {
        match pc {
            0x82708AE0 => {
    //   block [0x82708AE0..0x82708B44)
	// 82708AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708AEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708AF4: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 82708AF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708B00: 388AB904  addi r4, r10, -0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + -18172;
	// 82708B04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708B14: 386A19C0  addi r3, r10, 0x19c0
	ctx.r[3].s64 = ctx.r[10].s64 + 6592;
	// 82708B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708B1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708B20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82708B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708B28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82708B2C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82708B30: 4BD5E2F1  bl 0x82466e20
	ctx.lr = 0x82708B34;
	sub_82466E20(ctx, base);
	// 82708B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708B48 size=92
    let mut pc: u32 = 0x82708B48;
    'dispatch: loop {
        match pc {
            0x82708B48 => {
    //   block [0x82708B48..0x82708BA4)
	// 82708B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708B50: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708B54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82708B58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82708B5C: 4BDFFE65  bl 0x825089c0
	ctx.lr = 0x82708B60;
	sub_825089C0(ctx, base);
	// 82708B60: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82708B64: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82708B68: 394BB69C  addi r10, r11, -0x4964
	ctx.r[10].s64 = ctx.r[11].s64 + -18788;
	// 82708B6C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708B70: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 82708B74: 396B19F0  addi r11, r11, 0x19f0
	ctx.r[11].s64 = ctx.r[11].s64 + 6640;
	// 82708B78: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82708B7C: 39489D48  addi r10, r8, -0x62b8
	ctx.r[10].s64 = ctx.r[8].s64 + -25272;
	// 82708B80: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82708B84: 39499D60  addi r10, r9, -0x62a0
	ctx.r[10].s64 = ctx.r[9].s64 + -25248;
	// 82708B88: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82708B8C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82708B90: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82708B94: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82708B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708BA8 size=112
    let mut pc: u32 = 0x82708BA8;
    'dispatch: loop {
        match pc {
            0x82708BA8 => {
    //   block [0x82708BA8..0x82708C18)
	// 82708BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708BB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82708BB8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708BBC: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82708BC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708BC4: 390B2F98  addi r8, r11, 0x2f98
	ctx.r[8].s64 = ctx.r[11].s64 + 12184;
	// 82708BC8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82708BCC: 388AB69C  addi r4, r10, -0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + -18788;
	// 82708BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708BD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708BD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708BE0: 386A1A00  addi r3, r10, 0x1a00
	ctx.r[3].s64 = ctx.r[10].s64 + 6656;
	// 82708BE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708BE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708BFC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82708C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708C04: 4BD5E21D  bl 0x82466e20
	ctx.lr = 0x82708C08;
	sub_82466E20(ctx, base);
	// 82708C08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708C18 size=92
    let mut pc: u32 = 0x82708C18;
    'dispatch: loop {
        match pc {
            0x82708C18 => {
    //   block [0x82708C18..0x82708C74)
	// 82708C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708C20: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708C24: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82708C28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82708C2C: 4BDFFB15  bl 0x82508740
	ctx.lr = 0x82708C30;
	sub_82508740(ctx, base);
	// 82708C30: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82708C34: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82708C38: 394BB6CC  addi r10, r11, -0x4934
	ctx.r[10].s64 = ctx.r[11].s64 + -18740;
	// 82708C3C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708C40: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 82708C44: 396B1A30  addi r11, r11, 0x1a30
	ctx.r[11].s64 = ctx.r[11].s64 + 6704;
	// 82708C48: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82708C4C: 39489DA8  addi r10, r8, -0x6258
	ctx.r[10].s64 = ctx.r[8].s64 + -25176;
	// 82708C50: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82708C54: 39499DC0  addi r10, r9, -0x6240
	ctx.r[10].s64 = ctx.r[9].s64 + -25152;
	// 82708C58: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82708C5C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82708C60: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82708C64: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82708C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708C78 size=112
    let mut pc: u32 = 0x82708C78;
    'dispatch: loop {
        match pc {
            0x82708C78 => {
    //   block [0x82708C78..0x82708CE8)
	// 82708C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708C84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82708C88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708C8C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82708C90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708C94: 390B3028  addi r8, r11, 0x3028
	ctx.r[8].s64 = ctx.r[11].s64 + 12328;
	// 82708C98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82708C9C: 388AB6CC  addi r4, r10, -0x4934
	ctx.r[4].s64 = ctx.r[10].s64 + -18740;
	// 82708CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708CA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708CB0: 386A1A40  addi r3, r10, 0x1a40
	ctx.r[3].s64 = ctx.r[10].s64 + 6720;
	// 82708CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708CCC: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 82708CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708CD4: 4BD5E14D  bl 0x82466e20
	ctx.lr = 0x82708CD8;
	sub_82466E20(ctx, base);
	// 82708CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708CE8 size=92
    let mut pc: u32 = 0x82708CE8;
    'dispatch: loop {
        match pc {
            0x82708CE8 => {
    //   block [0x82708CE8..0x82708D44)
	// 82708CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708CF0: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708CF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82708CF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82708CFC: 4BDFFCDD  bl 0x825089d8
	ctx.lr = 0x82708D00;
	sub_825089D8(ctx, base);
	// 82708D00: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82708D04: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82708D08: 394BB6FC  addi r10, r11, -0x4904
	ctx.r[10].s64 = ctx.r[11].s64 + -18692;
	// 82708D0C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708D10: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 82708D14: 396B1A70  addi r11, r11, 0x1a70
	ctx.r[11].s64 = ctx.r[11].s64 + 6768;
	// 82708D18: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82708D1C: 39489E08  addi r10, r8, -0x61f8
	ctx.r[10].s64 = ctx.r[8].s64 + -25080;
	// 82708D20: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82708D24: 39499E20  addi r10, r9, -0x61e0
	ctx.r[10].s64 = ctx.r[9].s64 + -25056;
	// 82708D28: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82708D2C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82708D30: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82708D34: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82708D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708D48 size=112
    let mut pc: u32 = 0x82708D48;
    'dispatch: loop {
        match pc {
            0x82708D48 => {
    //   block [0x82708D48..0x82708DB8)
	// 82708D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708D54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708D58: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708D5C: 38AA1780  addi r5, r10, 0x1780
	ctx.r[5].s64 = ctx.r[10].s64 + 6016;
	// 82708D60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708D64: 390B3088  addi r8, r11, 0x3088
	ctx.r[8].s64 = ctx.r[11].s64 + 12424;
	// 82708D68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82708D6C: 388AB6FC  addi r4, r10, -0x4904
	ctx.r[4].s64 = ctx.r[10].s64 + -18692;
	// 82708D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708D74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708D78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708D80: 386A1A80  addi r3, r10, 0x1a80
	ctx.r[3].s64 = ctx.r[10].s64 + 6784;
	// 82708D84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708D9C: 38C000E0  li r6, 0xe0
	ctx.r[6].s64 = 224;
	// 82708DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708DA4: 4BD5E07D  bl 0x82466e20
	ctx.lr = 0x82708DA8;
	sub_82466E20(ctx, base);
	// 82708DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708DB8 size=112
    let mut pc: u32 = 0x82708DB8;
    'dispatch: loop {
        match pc {
            0x82708DB8 => {
    //   block [0x82708DB8..0x82708E28)
	// 82708DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708DC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708DC8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708DCC: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 82708DD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708DD4: 390B30B8  addi r8, r11, 0x30b8
	ctx.r[8].s64 = ctx.r[11].s64 + 12472;
	// 82708DD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82708DDC: 388AB9C4  addi r4, r10, -0x463c
	ctx.r[4].s64 = ctx.r[10].s64 + -17980;
	// 82708DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708DE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708DF0: 386A1AB0  addi r3, r10, 0x1ab0
	ctx.r[3].s64 = ctx.r[10].s64 + 6832;
	// 82708DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708E0C: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 82708E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708E14: 4BD5E00D  bl 0x82466e20
	ctx.lr = 0x82708E18;
	sub_82466E20(ctx, base);
	// 82708E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82708E28 size=24
    let mut pc: u32 = 0x82708E28;
    'dispatch: loop {
        match pc {
            0x82708E28 => {
    //   block [0x82708E28..0x82708E40)
	// 82708E28: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82708E2C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82708E30: 394AC970  addi r10, r10, -0x3690
	ctx.r[10].s64 = ctx.r[10].s64 + -13968;
	// 82708E34: 816BCE44  lwz r11, -0x31bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12732 as u32) ) } as u64;
	// 82708E38: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82708E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708E40 size=116
    let mut pc: u32 = 0x82708E40;
    'dispatch: loop {
        match pc {
            0x82708E40 => {
    //   block [0x82708E40..0x82708EB4)
	// 82708E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708E4C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82708E50: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82708E54: 390AC970  addi r8, r10, -0x3690
	ctx.r[8].s64 = ctx.r[10].s64 + -13968;
	// 82708E58: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708E5C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708E60: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 82708E64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708E68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82708E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708E70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708E74: 388AB8F0  addi r4, r10, -0x4710
	ctx.r[4].s64 = ctx.r[10].s64 + -18192;
	// 82708E78: 396B3110  addi r11, r11, 0x3110
	ctx.r[11].s64 = ctx.r[11].s64 + 12560;
	// 82708E7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708E80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708E84: 386A1AE0  addi r3, r10, 0x1ae0
	ctx.r[3].s64 = ctx.r[10].s64 + 6880;
	// 82708E88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82708E8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708E90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82708E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708E9C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82708EA0: 4BD5DF81  bl 0x82466e20
	ctx.lr = 0x82708EA4;
	sub_82466E20(ctx, base);
	// 82708EA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708EB8 size=112
    let mut pc: u32 = 0x82708EB8;
    'dispatch: loop {
        match pc {
            0x82708EB8 => {
    //   block [0x82708EB8..0x82708F28)
	// 82708EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708EC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708EC8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708ECC: 38AA19C0  addi r5, r10, 0x19c0
	ctx.r[5].s64 = ctx.r[10].s64 + 6592;
	// 82708ED0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708ED4: 390B3178  addi r8, r11, 0x3178
	ctx.r[8].s64 = ctx.r[11].s64 + 12664;
	// 82708ED8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82708EDC: 388AB92C  addi r4, r10, -0x46d4
	ctx.r[4].s64 = ctx.r[10].s64 + -18132;
	// 82708EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708EE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708EE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708EF0: 386A1B10  addi r3, r10, 0x1b10
	ctx.r[3].s64 = ctx.r[10].s64 + 6928;
	// 82708EF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708F00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708F08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708F0C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82708F10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708F14: 4BD5DF0D  bl 0x82466e20
	ctx.lr = 0x82708F18;
	sub_82466E20(ctx, base);
	// 82708F18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708F28 size=100
    let mut pc: u32 = 0x82708F28;
    'dispatch: loop {
        match pc {
            0x82708F28 => {
    //   block [0x82708F28..0x82708F8C)
	// 82708F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708F34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708F3C: 38AA2254  addi r5, r10, 0x2254
	ctx.r[5].s64 = ctx.r[10].s64 + 8788;
	// 82708F40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708F48: 388AB488  addi r4, r10, -0x4b78
	ctx.r[4].s64 = ctx.r[10].s64 + -19320;
	// 82708F4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708F5C: 386A1B40  addi r3, r10, 0x1b40
	ctx.r[3].s64 = ctx.r[10].s64 + 6976;
	// 82708F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708F64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708F68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82708F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708F70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82708F74: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82708F78: 4BD5DEA9  bl 0x82466e20
	ctx.lr = 0x82708F7C;
	sub_82466E20(ctx, base);
	// 82708F7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708F90 size=112
    let mut pc: u32 = 0x82708F90;
    'dispatch: loop {
        match pc {
            0x82708F90 => {
    //   block [0x82708F90..0x82709000)
	// 82708F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708F9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708FA0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708FA4: 38AA22B4  addi r5, r10, 0x22b4
	ctx.r[5].s64 = ctx.r[10].s64 + 8884;
	// 82708FA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708FAC: 390B31F0  addi r8, r11, 0x31f0
	ctx.r[8].s64 = ctx.r[11].s64 + 12784;
	// 82708FB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82708FB4: 388AB9A0  addi r4, r10, -0x4660
	ctx.r[4].s64 = ctx.r[10].s64 + -18016;
	// 82708FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708FBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708FC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708FC8: 386A1B70  addi r3, r10, 0x1b70
	ctx.r[3].s64 = ctx.r[10].s64 + 7024;
	// 82708FCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708FD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708FD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708FE4: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82708FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708FEC: 4BD5DE35  bl 0x82466e20
	ctx.lr = 0x82708FF0;
	sub_82466E20(ctx, base);
	// 82708FF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709000 size=112
    let mut pc: u32 = 0x82709000;
    'dispatch: loop {
        match pc {
            0x82709000 => {
    //   block [0x82709000..0x82709070)
	// 82709000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270900C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709010: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709014: 38AA22B4  addi r5, r10, 0x22b4
	ctx.r[5].s64 = ctx.r[10].s64 + 8884;
	// 82709018: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270901C: 390B32D0  addi r8, r11, 0x32d0
	ctx.r[8].s64 = ctx.r[11].s64 + 13008;
	// 82709020: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82709024: 388AB530  addi r4, r10, -0x4ad0
	ctx.r[4].s64 = ctx.r[10].s64 + -19152;
	// 82709028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270902C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709038: 386A1BA0  addi r3, r10, 0x1ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 7072;
	// 8270903C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270904C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709054: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82709058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270905C: 4BD5DDC5  bl 0x82466e20
	ctx.lr = 0x82709060;
	sub_82466E20(ctx, base);
	// 82709060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270906C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709070 size=108
    let mut pc: u32 = 0x82709070;
    'dispatch: loop {
        match pc {
            0x82709070 => {
    //   block [0x82709070..0x827090DC)
	// 82709070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270907C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709084: 38EB3364  addi r7, r11, 0x3364
	ctx.r[7].s64 = ctx.r[11].s64 + 13156;
	// 82709088: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8270908C: 388AB638  addi r4, r10, -0x49c8
	ctx.r[4].s64 = ctx.r[10].s64 + -18888;
	// 82709090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709094: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270909C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827090A0: 386A1BD0  addi r3, r10, 0x1bd0
	ctx.r[3].s64 = ctx.r[10].s64 + 7120;
	// 827090A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827090A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827090AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827090B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827090B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827090B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827090BC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 827090C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827090C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827090C8: 4BD5DD59  bl 0x82466e20
	ctx.lr = 0x827090CC;
	sub_82466E20(ctx, base);
	// 827090CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827090D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827090D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827090D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827090E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827090E0 size=108
    let mut pc: u32 = 0x827090E0;
    'dispatch: loop {
        match pc {
            0x827090E0 => {
    //   block [0x827090E0..0x8270914C)
	// 827090E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827090E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827090E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827090EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827090F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827090F4: 38EB33B0  addi r7, r11, 0x33b0
	ctx.r[7].s64 = ctx.r[11].s64 + 13232;
	// 827090F8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 827090FC: 388AB3C0  addi r4, r10, -0x4c40
	ctx.r[4].s64 = ctx.r[10].s64 + -19520;
	// 82709100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709104: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709108: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270910C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709110: 386A1C00  addi r3, r10, 0x1c00
	ctx.r[3].s64 = ctx.r[10].s64 + 7168;
	// 82709114: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82709118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270911C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270912C: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 82709130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709134: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709138: 4BD5DCE9  bl 0x82466e20
	ctx.lr = 0x8270913C;
	sub_82466E20(ctx, base);
	// 8270913C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709150 size=116
    let mut pc: u32 = 0x82709150;
    'dispatch: loop {
        match pc {
            0x82709150 => {
    //   block [0x82709150..0x827091C4)
	// 82709150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270915C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709160: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82709164: 390B3470  addi r8, r11, 0x3470
	ctx.r[8].s64 = ctx.r[11].s64 + 13424;
	// 82709168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270916C: 392A3398  addi r9, r10, 0x3398
	ctx.r[9].s64 = ctx.r[10].s64 + 13208;
	// 82709170: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709174: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82709178: 38AA2180  addi r5, r10, 0x2180
	ctx.r[5].s64 = ctx.r[10].s64 + 8576;
	// 8270917C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709184: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270918C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709194: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82709198: 388AB3E0  addi r4, r10, -0x4c20
	ctx.r[4].s64 = ctx.r[10].s64 + -19488;
	// 8270919C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827091A0: 386B1C30  addi r3, r11, 0x1c30
	ctx.r[3].s64 = ctx.r[11].s64 + 7216;
	// 827091A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827091A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827091AC: 38C0004C  li r6, 0x4c
	ctx.r[6].s64 = 76;
	// 827091B0: 4BD5DC71  bl 0x82466e20
	ctx.lr = 0x827091B4;
	sub_82466E20(ctx, base);
	// 827091B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827091B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827091BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827091C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827091C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827091C8 size=116
    let mut pc: u32 = 0x827091C8;
    'dispatch: loop {
        match pc {
            0x827091C8 => {
    //   block [0x827091C8..0x8270923C)
	// 827091C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827091CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827091D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827091D4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827091D8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827091DC: 390B352C  addi r8, r11, 0x352c
	ctx.r[8].s64 = ctx.r[11].s64 + 13612;
	// 827091E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827091E4: 392A3518  addi r9, r10, 0x3518
	ctx.r[9].s64 = ctx.r[10].s64 + 13592;
	// 827091E8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827091EC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827091F0: 38AA1CF0  addi r5, r10, 0x1cf0
	ctx.r[5].s64 = ctx.r[10].s64 + 7408;
	// 827091F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827091F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827091FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270920C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82709210: 388AB7C4  addi r4, r10, -0x483c
	ctx.r[4].s64 = ctx.r[10].s64 + -18492;
	// 82709214: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82709218: 386B1C60  addi r3, r11, 0x1c60
	ctx.r[3].s64 = ctx.r[11].s64 + 7264;
	// 8270921C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82709220: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709224: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82709228: 4BD5DBF9  bl 0x82466e20
	ctx.lr = 0x8270922C;
	sub_82466E20(ctx, base);
	// 8270922C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709240 size=108
    let mut pc: u32 = 0x82709240;
    'dispatch: loop {
        match pc {
            0x82709240 => {
    //   block [0x82709240..0x827092AC)
	// 82709240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270924C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709250: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709254: 38EB3544  addi r7, r11, 0x3544
	ctx.r[7].s64 = ctx.r[11].s64 + 13636;
	// 82709258: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8270925C: 388AB9E4  addi r4, r10, -0x461c
	ctx.r[4].s64 = ctx.r[10].s64 + -17948;
	// 82709260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709264: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709268: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270926C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709270: 386A1C90  addi r3, r10, 0x1c90
	ctx.r[3].s64 = ctx.r[10].s64 + 7312;
	// 82709274: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82709278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270927C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270928C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82709290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709294: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709298: 4BD5DB89  bl 0x82466e20
	ctx.lr = 0x8270929C;
	sub_82466E20(ctx, base);
	// 8270929C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827092A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827092A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827092A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827092B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827092B0 size=112
    let mut pc: u32 = 0x827092B0;
    'dispatch: loop {
        match pc {
            0x827092B0 => {
    //   block [0x827092B0..0x82709320)
	// 827092B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827092B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827092B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827092BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827092C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827092C4: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 827092C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827092CC: 390B3574  addi r8, r11, 0x3574
	ctx.r[8].s64 = ctx.r[11].s64 + 13684;
	// 827092D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827092D4: 388AB9F8  addi r4, r10, -0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + -17928;
	// 827092D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827092DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827092E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827092E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827092E8: 386A1CC0  addi r3, r10, 0x1cc0
	ctx.r[3].s64 = ctx.r[10].s64 + 7360;
	// 827092EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827092F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827092F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827092F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827092FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709304: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82709308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270930C: 4BD5DB15  bl 0x82466e20
	ctx.lr = 0x82709310;
	sub_82466E20(ctx, base);
	// 82709310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270931C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709320 size=100
    let mut pc: u32 = 0x82709320;
    'dispatch: loop {
        match pc {
            0x82709320 => {
    //   block [0x82709320..0x82709384)
	// 82709320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270932C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709334: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 82709338: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270933C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709340: 388AB918  addi r4, r10, -0x46e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18152;
	// 82709344: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270934C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709354: 386A1CF0  addi r3, r10, 0x1cf0
	ctx.r[3].s64 = ctx.r[10].s64 + 7408;
	// 82709358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270935C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709360: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82709364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709368: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270936C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82709370: 4BD5DAB1  bl 0x82466e20
	ctx.lr = 0x82709374;
	sub_82466E20(ctx, base);
	// 82709374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270937C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709388 size=108
    let mut pc: u32 = 0x82709388;
    'dispatch: loop {
        match pc {
            0x82709388 => {
    //   block [0x82709388..0x827093F4)
	// 82709388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270938C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709394: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709398: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270939C: 38EB35D0  addi r7, r11, 0x35d0
	ctx.r[7].s64 = ctx.r[11].s64 + 13776;
	// 827093A0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 827093A4: 388AB66C  addi r4, r10, -0x4994
	ctx.r[4].s64 = ctx.r[10].s64 + -18836;
	// 827093A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827093AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827093B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827093B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827093B8: 386A1D20  addi r3, r10, 0x1d20
	ctx.r[3].s64 = ctx.r[10].s64 + 7456;
	// 827093BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827093C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827093C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827093C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827093CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827093D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827093D4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 827093D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827093DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827093E0: 4BD5DA41  bl 0x82466e20
	ctx.lr = 0x827093E4;
	sub_82466E20(ctx, base);
	// 827093E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827093E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827093EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827093F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827093F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827093F8 size=24
    let mut pc: u32 = 0x827093F8;
    'dispatch: loop {
        match pc {
            0x827093F8 => {
    //   block [0x827093F8..0x82709410)
	// 827093F8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827093FC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82709400: 394ACBD8  addi r10, r10, -0x3428
	ctx.r[10].s64 = ctx.r[10].s64 + -13352;
	// 82709404: 816BCE44  lwz r11, -0x31bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12732 as u32) ) } as u64;
	// 82709408: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8270940C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709410 size=116
    let mut pc: u32 = 0x82709410;
    'dispatch: loop {
        match pc {
            0x82709410 => {
    //   block [0x82709410..0x82709484)
	// 82709410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270941C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82709420: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82709424: 390ACBD8  addi r8, r10, -0x3428
	ctx.r[8].s64 = ctx.r[10].s64 + -13352;
	// 82709428: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270942C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709430: 38AA22B4  addi r5, r10, 0x22b4
	ctx.r[5].s64 = ctx.r[10].s64 + 8884;
	// 82709434: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709438: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8270943C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709444: 388AB688  addi r4, r10, -0x4978
	ctx.r[4].s64 = ctx.r[10].s64 + -18808;
	// 82709448: 396B3630  addi r11, r11, 0x3630
	ctx.r[11].s64 = ctx.r[11].s64 + 13872;
	// 8270944C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709450: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709454: 386A1D50  addi r3, r10, 0x1d50
	ctx.r[3].s64 = ctx.r[10].s64 + 7504;
	// 82709458: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8270945C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709460: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82709464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270946C: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 82709470: 4BD5D9B1  bl 0x82466e20
	ctx.lr = 0x82709474;
	sub_82466E20(ctx, base);
	// 82709474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270947C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709488 size=112
    let mut pc: u32 = 0x82709488;
    'dispatch: loop {
        match pc {
            0x82709488 => {
    //   block [0x82709488..0x827094F8)
	// 82709488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270948C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709494: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709498: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270949C: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 827094A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827094A4: 390B3698  addi r8, r11, 0x3698
	ctx.r[8].s64 = ctx.r[11].s64 + 13976;
	// 827094A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 827094AC: 388AB808  addi r4, r10, -0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + -18424;
	// 827094B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827094B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827094B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827094BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827094C0: 386A1D80  addi r3, r10, 0x1d80
	ctx.r[3].s64 = ctx.r[10].s64 + 7552;
	// 827094C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827094C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827094CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827094D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827094D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827094D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827094DC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 827094E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827094E4: 4BD5D93D  bl 0x82466e20
	ctx.lr = 0x827094E8;
	sub_82466E20(ctx, base);
	// 827094E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827094EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827094F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827094F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827094F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827094F8 size=108
    let mut pc: u32 = 0x827094F8;
    'dispatch: loop {
        match pc {
            0x827094F8 => {
    //   block [0x827094F8..0x82709564)
	// 827094F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827094FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709504: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709508: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270950C: 38EB3728  addi r7, r11, 0x3728
	ctx.r[7].s64 = ctx.r[11].s64 + 14120;
	// 82709510: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82709514: 388AB820  addi r4, r10, -0x47e0
	ctx.r[4].s64 = ctx.r[10].s64 + -18400;
	// 82709518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270951C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709520: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82709524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709528: 386A1DB0  addi r3, r10, 0x1db0
	ctx.r[3].s64 = ctx.r[10].s64 + 7600;
	// 8270952C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82709530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270953C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709544: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82709548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270954C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709550: 4BD5D8D1  bl 0x82466e20
	ctx.lr = 0x82709554;
	sub_82466E20(ctx, base);
	// 82709554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270955C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709568 size=112
    let mut pc: u32 = 0x82709568;
    'dispatch: loop {
        match pc {
            0x82709568 => {
    //   block [0x82709568..0x827095D8)
	// 82709568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270956C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709574: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709578: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270957C: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 82709580: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709584: 390B3770  addi r8, r11, 0x3770
	ctx.r[8].s64 = ctx.r[11].s64 + 14192;
	// 82709588: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8270958C: 388AB844  addi r4, r10, -0x47bc
	ctx.r[4].s64 = ctx.r[10].s64 + -18364;
	// 82709590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709594: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270959C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827095A0: 386A1DE0  addi r3, r10, 0x1de0
	ctx.r[3].s64 = ctx.r[10].s64 + 7648;
	// 827095A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827095A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827095AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827095B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827095B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827095B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827095BC: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 827095C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827095C4: 4BD5D85D  bl 0x82466e20
	ctx.lr = 0x827095C8;
	sub_82466E20(ctx, base);
	// 827095C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827095CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827095D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827095D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827095D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827095D8 size=112
    let mut pc: u32 = 0x827095D8;
    'dispatch: loop {
        match pc {
            0x827095D8 => {
    //   block [0x827095D8..0x82709648)
	// 827095D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827095DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827095E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827095E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827095E8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827095EC: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 827095F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827095F4: 390B3848  addi r8, r11, 0x3848
	ctx.r[8].s64 = ctx.r[11].s64 + 14408;
	// 827095F8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 827095FC: 388AB9D0  addi r4, r10, -0x4630
	ctx.r[4].s64 = ctx.r[10].s64 + -17968;
	// 82709600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709604: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270960C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709610: 386A1E10  addi r3, r10, 0x1e10
	ctx.r[3].s64 = ctx.r[10].s64 + 7696;
	// 82709614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270961C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709624: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82709628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270962C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82709630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709634: 4BD5D7ED  bl 0x82466e20
	ctx.lr = 0x82709638;
	sub_82466E20(ctx, base);
	// 82709638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270963C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709648 size=112
    let mut pc: u32 = 0x82709648;
    'dispatch: loop {
        match pc {
            0x82709648 => {
    //   block [0x82709648..0x827096B8)
	// 82709648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270964C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709654: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82709658: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270965C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82709660: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709664: 390B392C  addi r8, r11, 0x392c
	ctx.r[8].s64 = ctx.r[11].s64 + 14636;
	// 82709668: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270966C: 388AB7A4  addi r4, r10, -0x485c
	ctx.r[4].s64 = ctx.r[10].s64 + -18524;
	// 82709670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709674: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270967C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709680: 386A1E40  addi r3, r10, 0x1e40
	ctx.r[3].s64 = ctx.r[10].s64 + 7744;
	// 82709684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270968C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709694: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82709698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270969C: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 827096A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827096A4: 4BD5D77D  bl 0x82466e20
	ctx.lr = 0x827096A8;
	sub_82466E20(ctx, base);
	// 827096A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827096AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827096B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827096B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827096B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827096B8 size=92
    let mut pc: u32 = 0x827096B8;
    'dispatch: loop {
        match pc {
            0x827096B8 => {
    //   block [0x827096B8..0x82709714)
	// 827096B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827096BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827096C0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827096C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827096C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827096CC: 4BDF85B5  bl 0x82501c80
	ctx.lr = 0x827096D0;
	sub_82501C80(ctx, base);
	// 827096D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827096D4: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 827096D8: 394BB85C  addi r10, r11, -0x47a4
	ctx.r[10].s64 = ctx.r[11].s64 + -18340;
	// 827096DC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827096E0: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 827096E4: 396B1E70  addi r11, r11, 0x1e70
	ctx.r[11].s64 = ctx.r[11].s64 + 7792;
	// 827096E8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827096EC: 3948A8B8  addi r10, r8, -0x5748
	ctx.r[10].s64 = ctx.r[8].s64 + -22344;
	// 827096F0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827096F4: 3949A8A0  addi r10, r9, -0x5760
	ctx.r[10].s64 = ctx.r[9].s64 + -22368;
	// 827096F8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827096FC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82709700: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82709704: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82709708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270970C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709718 size=116
    let mut pc: u32 = 0x82709718;
    'dispatch: loop {
        match pc {
            0x82709718 => {
    //   block [0x82709718..0x8270978C)
	// 82709718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270971C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709724: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709728: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270972C: 392B39B8  addi r9, r11, 0x39b8
	ctx.r[9].s64 = ctx.r[11].s64 + 14776;
	// 82709730: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 82709734: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709738: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8270973C: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82709740: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709744: 388AB85C  addi r4, r10, -0x47a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18340;
	// 82709748: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270974C: 396B39E8  addi r11, r11, 0x39e8
	ctx.r[11].s64 = ctx.r[11].s64 + 14824;
	// 82709750: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82709754: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709758: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8270975C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709760: 386A1E80  addi r3, r10, 0x1e80
	ctx.r[3].s64 = ctx.r[10].s64 + 7808;
	// 82709764: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82709768: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8270976C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82709770: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82709774: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709778: 4BD5D6A9  bl 0x82466e20
	ctx.lr = 0x8270977C;
	sub_82466E20(ctx, base);
	// 8270977C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709790 size=112
    let mut pc: u32 = 0x82709790;
    'dispatch: loop {
        match pc {
            0x82709790 => {
    //   block [0x82709790..0x82709800)
	// 82709790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270979C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827097A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827097A4: 38AA1054  addi r5, r10, 0x1054
	ctx.r[5].s64 = ctx.r[10].s64 + 4180;
	// 827097A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827097AC: 390B3A78  addi r8, r11, 0x3a78
	ctx.r[8].s64 = ctx.r[11].s64 + 14968;
	// 827097B0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 827097B4: 388AB3F0  addi r4, r10, -0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + -19472;
	// 827097B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827097BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827097C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827097C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827097C8: 386A1EB0  addi r3, r10, 0x1eb0
	ctx.r[3].s64 = ctx.r[10].s64 + 7856;
	// 827097CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827097D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827097D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827097D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827097DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827097E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827097E4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 827097E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827097EC: 4BD5D635  bl 0x82466e20
	ctx.lr = 0x827097F0;
	sub_82466E20(ctx, base);
	// 827097F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827097F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827097F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827097FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709800 size=112
    let mut pc: u32 = 0x82709800;
    'dispatch: loop {
        match pc {
            0x82709800 => {
    //   block [0x82709800..0x82709870)
	// 82709800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270980C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82709810: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709814: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82709818: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8270981C: 390B3AD8  addi r8, r11, 0x3ad8
	ctx.r[8].s64 = ctx.r[11].s64 + 15064;
	// 82709820: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82709824: 388AB39C  addi r4, r10, -0x4c64
	ctx.r[4].s64 = ctx.r[10].s64 + -19556;
	// 82709828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270982C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709838: 386A1EE0  addi r3, r10, 0x1ee0
	ctx.r[3].s64 = ctx.r[10].s64 + 7904;
	// 8270983C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270984C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709854: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82709858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270985C: 4BD5D5C5  bl 0x82466e20
	ctx.lr = 0x82709860;
	sub_82466E20(ctx, base);
	// 82709860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270986C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709870 size=96
    let mut pc: u32 = 0x82709870;
    'dispatch: loop {
        match pc {
            0x82709870 => {
    //   block [0x82709870..0x827098D0)
	// 82709870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709878: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270987C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82709880: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82709884: 4BE05625  bl 0x8250eea8
	ctx.lr = 0x82709888;
	sub_8250EEA8(ctx, base);
	// 82709888: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8270988C: 3CE08250  lis r7, -0x7db0
	ctx.r[7].s64 = -2108686336;
	// 82709890: 394BB5FC  addi r10, r11, -0x4a04
	ctx.r[10].s64 = ctx.r[11].s64 + -18948;
	// 82709894: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709898: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 8270989C: 392B3BA0  addi r9, r11, 0x3ba0
	ctx.r[9].s64 = ctx.r[11].s64 + 15264;
	// 827098A0: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827098A4: 396B1F10  addi r11, r11, 0x1f10
	ctx.r[11].s64 = ctx.r[11].s64 + 7952;
	// 827098A8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827098AC: 3947ADC0  addi r10, r7, -0x5240
	ctx.r[10].s64 = ctx.r[7].s64 + -21056;
	// 827098B0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827098B4: 3948ADA8  addi r10, r8, -0x5258
	ctx.r[10].s64 = ctx.r[8].s64 + -21080;
	// 827098B8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827098BC: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 827098C0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 827098C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827098C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827098CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827098D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827098D0 size=100
    let mut pc: u32 = 0x827098D0;
    'dispatch: loop {
        match pc {
            0x827098D0 => {
    //   block [0x827098D0..0x82709934)
	// 827098D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827098D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827098D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827098DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827098E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827098E4: 38AA2120  addi r5, r10, 0x2120
	ctx.r[5].s64 = ctx.r[10].s64 + 8480;
	// 827098E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827098EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827098F0: 388AB5FC  addi r4, r10, -0x4a04
	ctx.r[4].s64 = ctx.r[10].s64 + -18948;
	// 827098F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827098F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827098FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709904: 386A1F20  addi r3, r10, 0x1f20
	ctx.r[3].s64 = ctx.r[10].s64 + 7968;
	// 82709908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270990C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709910: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82709914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709918: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270991C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82709920: 4BD5D501  bl 0x82466e20
	ctx.lr = 0x82709924;
	sub_82466E20(ctx, base);
	// 82709924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270992C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709938 size=100
    let mut pc: u32 = 0x82709938;
    'dispatch: loop {
        match pc {
            0x82709938 => {
    //   block [0x82709938..0x8270999C)
	// 82709938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270993C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709944: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270994C: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 82709950: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709958: 388ABA20  addi r4, r10, -0x45e0
	ctx.r[4].s64 = ctx.r[10].s64 + -17888;
	// 8270995C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270996C: 386A1F50  addi r3, r10, 0x1f50
	ctx.r[3].s64 = ctx.r[10].s64 + 8016;
	// 82709970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709974: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709978: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270997C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709980: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709984: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82709988: 4BD5D499  bl 0x82466e20
	ctx.lr = 0x8270998C;
	sub_82466E20(ctx, base);
	// 8270998C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827099A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827099A0 size=112
    let mut pc: u32 = 0x827099A0;
    'dispatch: loop {
        match pc {
            0x827099A0 => {
    //   block [0x827099A0..0x82709A10)
	// 827099A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827099A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827099A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827099AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827099B0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827099B4: 38AA1CF0  addi r5, r10, 0x1cf0
	ctx.r[5].s64 = ctx.r[10].s64 + 7408;
	// 827099B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827099BC: 390B3BC8  addi r8, r11, 0x3bc8
	ctx.r[8].s64 = ctx.r[11].s64 + 15304;
	// 827099C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827099C4: 388ABA0C  addi r4, r10, -0x45f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17908;
	// 827099C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827099CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827099D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827099D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827099D8: 386A1F80  addi r3, r10, 0x1f80
	ctx.r[3].s64 = ctx.r[10].s64 + 8064;
	// 827099DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827099E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827099E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827099E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827099EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827099F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827099F4: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 827099F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827099FC: 4BD5D425  bl 0x82466e20
	ctx.lr = 0x82709A00;
	sub_82466E20(ctx, base);
	// 82709A00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709A10 size=100
    let mut pc: u32 = 0x82709A10;
    'dispatch: loop {
        match pc {
            0x82709A10 => {
    //   block [0x82709A10..0x82709A74)
	// 82709A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709A1C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82709A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709A24: 392A3C78  addi r9, r10, 0x3c78
	ctx.r[9].s64 = ctx.r[10].s64 + 15480;
	// 82709A28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709A30: 388ABA80  addi r4, r10, -0x4580
	ctx.r[4].s64 = ctx.r[10].s64 + -17792;
	// 82709A34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709A44: 386A1FB0  addi r3, r10, 0x1fb0
	ctx.r[3].s64 = ctx.r[10].s64 + 8112;
	// 82709A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709A4C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82709A50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82709A54: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82709A58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709A5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709A60: 4BD5D3C1  bl 0x82466e20
	ctx.lr = 0x82709A64;
	sub_82466E20(ctx, base);
	// 82709A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709A78 size=112
    let mut pc: u32 = 0x82709A78;
    'dispatch: loop {
        match pc {
            0x82709A78 => {
    //   block [0x82709A78..0x82709AE8)
	// 82709A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709A84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709A88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709A8C: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 82709A90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709A94: 390B3CB4  addi r8, r11, 0x3cb4
	ctx.r[8].s64 = ctx.r[11].s64 + 15540;
	// 82709A98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82709A9C: 388AB8E0  addi r4, r10, -0x4720
	ctx.r[4].s64 = ctx.r[10].s64 + -18208;
	// 82709AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709AA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709AB0: 386A1FE0  addi r3, r10, 0x1fe0
	ctx.r[3].s64 = ctx.r[10].s64 + 8160;
	// 82709AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709ACC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82709AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709AD4: 4BD5D34D  bl 0x82466e20
	ctx.lr = 0x82709AD8;
	sub_82466E20(ctx, base);
	// 82709AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709AE8 size=96
    let mut pc: u32 = 0x82709AE8;
    'dispatch: loop {
        match pc {
            0x82709AE8 => {
    //   block [0x82709AE8..0x82709B48)
	// 82709AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709AF4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709AFC: 388AB46C  addi r4, r10, -0x4b94
	ctx.r[4].s64 = ctx.r[10].s64 + -19348;
	// 82709B00: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709B08: 386A2010  addi r3, r10, 0x2010
	ctx.r[3].s64 = ctx.r[10].s64 + 8208;
	// 82709B0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709B10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709B14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82709B18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709B20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709B24: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82709B28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82709B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709B30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709B34: 4BD5D2ED  bl 0x82466e20
	ctx.lr = 0x82709B38;
	sub_82466E20(ctx, base);
	// 82709B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709B48 size=92
    let mut pc: u32 = 0x82709B48;
    'dispatch: loop {
        match pc {
            0x82709B48 => {
    //   block [0x82709B48..0x82709BA4)
	// 82709B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709B50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709B54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82709B58: 4BDF4C69  bl 0x824fe7c0
	ctx.lr = 0x82709B5C;
	sub_824FE7C0(ctx, base);
	// 82709B5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82709B60: 3CE08250  lis r7, -0x7db0
	ctx.r[7].s64 = -2108686336;
	// 82709B64: 394BB4CC  addi r10, r11, -0x4b34
	ctx.r[10].s64 = ctx.r[11].s64 + -19252;
	// 82709B68: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709B6C: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82709B70: 392BFBA4  addi r9, r11, -0x45c
	ctx.r[9].s64 = ctx.r[11].s64 + -1116;
	// 82709B74: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82709B78: 396B2040  addi r11, r11, 0x2040
	ctx.r[11].s64 = ctx.r[11].s64 + 8256;
	// 82709B7C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82709B80: 3947B028  addi r10, r7, -0x4fd8
	ctx.r[10].s64 = ctx.r[7].s64 + -20440;
	// 82709B84: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82709B88: 3948B010  addi r10, r8, -0x4ff0
	ctx.r[10].s64 = ctx.r[8].s64 + -20464;
	// 82709B8C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82709B90: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82709B94: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82709B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709BA8 size=100
    let mut pc: u32 = 0x82709BA8;
    'dispatch: loop {
        match pc {
            0x82709BA8 => {
    //   block [0x82709BA8..0x82709C0C)
	// 82709BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709BB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709BBC: 38AA1870  addi r5, r10, 0x1870
	ctx.r[5].s64 = ctx.r[10].s64 + 6256;
	// 82709BC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709BC8: 388AB4CC  addi r4, r10, -0x4b34
	ctx.r[4].s64 = ctx.r[10].s64 + -19252;
	// 82709BCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709BDC: 386A2050  addi r3, r10, 0x2050
	ctx.r[3].s64 = ctx.r[10].s64 + 8272;
	// 82709BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709BE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709BE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82709BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709BF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709BF4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82709BF8: 4BD5D229  bl 0x82466e20
	ctx.lr = 0x82709BFC;
	sub_82466E20(ctx, base);
	// 82709BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709C10 size=96
    let mut pc: u32 = 0x82709C10;
    'dispatch: loop {
        match pc {
            0x82709C10 => {
    //   block [0x82709C10..0x82709C70)
	// 82709C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709C1C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709C24: 388AB454  addi r4, r10, -0x4bac
	ctx.r[4].s64 = ctx.r[10].s64 + -19372;
	// 82709C28: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709C30: 386A2080  addi r3, r10, 0x2080
	ctx.r[3].s64 = ctx.r[10].s64 + 8320;
	// 82709C34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709C3C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82709C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709C4C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82709C50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82709C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709C58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709C5C: 4BD5D1C5  bl 0x82466e20
	ctx.lr = 0x82709C60;
	sub_82466E20(ctx, base);
	// 82709C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709C70 size=112
    let mut pc: u32 = 0x82709C70;
    'dispatch: loop {
        match pc {
            0x82709C70 => {
    //   block [0x82709C70..0x82709CE0)
	// 82709C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709C7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709C80: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709C84: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 82709C88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709C8C: 390B3CD0  addi r8, r11, 0x3cd0
	ctx.r[8].s64 = ctx.r[11].s64 + 15568;
	// 82709C90: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82709C94: 388AB7F0  addi r4, r10, -0x4810
	ctx.r[4].s64 = ctx.r[10].s64 + -18448;
	// 82709C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709C9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709CA8: 386A20B0  addi r3, r10, 0x20b0
	ctx.r[3].s64 = ctx.r[10].s64 + 8368;
	// 82709CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709CC4: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82709CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709CCC: 4BD5D155  bl 0x82466e20
	ctx.lr = 0x82709CD0;
	sub_82466E20(ctx, base);
	// 82709CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82709CE0 size=28
    let mut pc: u32 = 0x82709CE0;
    'dispatch: loop {
        match pc {
            0x82709CE0 => {
    //   block [0x82709CE0..0x82709CFC)
	// 82709CE0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82709CE4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82709CE8: 394ACEC0  addi r10, r10, -0x3140
	ctx.r[10].s64 = ctx.r[10].s64 + -12608;
	// 82709CEC: 816BCEA8  lwz r11, -0x3158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12632 as u32) ) } as u64;
	// 82709CF0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82709CF4: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82709CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709D00 size=112
    let mut pc: u32 = 0x82709D00;
    'dispatch: loop {
        match pc {
            0x82709D00 => {
    //   block [0x82709D00..0x82709D70)
	// 82709D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709D0C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82709D10: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 82709D14: 38EACEC0  addi r7, r10, -0x3140
	ctx.r[7].s64 = ctx.r[10].s64 + -12608;
	// 82709D18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709D1C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709D20: 388AB648  addi r4, r10, -0x49b8
	ctx.r[4].s64 = ctx.r[10].s64 + -18872;
	// 82709D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709D28: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82709D2C: 396B3DC8  addi r11, r11, 0x3dc8
	ctx.r[11].s64 = ctx.r[11].s64 + 15816;
	// 82709D30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82709D34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709D38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709D3C: 386A20E0  addi r3, r10, 0x20e0
	ctx.r[3].s64 = ctx.r[10].s64 + 8416;
	// 82709D40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709D44: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82709D48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709D4C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82709D50: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 82709D54: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709D58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709D5C: 4BD5D0C5  bl 0x82466e20
	ctx.lr = 0x82709D60;
	sub_82466E20(ctx, base);
	// 82709D60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709D70 size=92
    let mut pc: u32 = 0x82709D70;
    'dispatch: loop {
        match pc {
            0x82709D70 => {
    //   block [0x82709D70..0x82709DCC)
	// 82709D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709D78: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709D7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82709D80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82709D84: 4BE05125  bl 0x8250eea8
	ctx.lr = 0x82709D88;
	sub_8250EEA8(ctx, base);
	// 82709D88: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82709D8C: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82709D90: 394BB65C  addi r10, r11, -0x49a4
	ctx.r[10].s64 = ctx.r[11].s64 + -18852;
	// 82709D94: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82709D98: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 82709D9C: 396B2110  addi r11, r11, 0x2110
	ctx.r[11].s64 = ctx.r[11].s64 + 8464;
	// 82709DA0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82709DA4: 3948B258  addi r10, r8, -0x4da8
	ctx.r[10].s64 = ctx.r[8].s64 + -19880;
	// 82709DA8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82709DAC: 3949B270  addi r10, r9, -0x4d90
	ctx.r[10].s64 = ctx.r[9].s64 + -19856;
	// 82709DB0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82709DB4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82709DB8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82709DBC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82709DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82709DD0 size=24
    let mut pc: u32 = 0x82709DD0;
    'dispatch: loop {
        match pc {
            0x82709DD0 => {
    //   block [0x82709DD0..0x82709DE8)
	// 82709DD0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82709DD4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82709DD8: 394AD028  addi r10, r10, -0x2fd8
	ctx.r[10].s64 = ctx.r[10].s64 + -12248;
	// 82709DDC: 816BCE44  lwz r11, -0x31bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12732 as u32) ) } as u64;
	// 82709DE0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82709DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709DE8 size=116
    let mut pc: u32 = 0x82709DE8;
    'dispatch: loop {
        match pc {
            0x82709DE8 => {
    //   block [0x82709DE8..0x82709E5C)
	// 82709DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709DF4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709DF8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709DFC: 392B3DA0  addi r9, r11, 0x3da0
	ctx.r[9].s64 = ctx.r[11].s64 + 15776;
	// 82709E00: 38AA22B4  addi r5, r10, 0x22b4
	ctx.r[5].s64 = ctx.r[10].s64 + 8884;
	// 82709E04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709E08: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 82709E0C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82709E10: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82709E14: 388AB65C  addi r4, r10, -0x49a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18852;
	// 82709E18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709E1C: 396BD028  addi r11, r11, -0x2fd8
	ctx.r[11].s64 = ctx.r[11].s64 + -12248;
	// 82709E20: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82709E24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709E28: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82709E2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709E30: 386A2120  addi r3, r10, 0x2120
	ctx.r[3].s64 = ctx.r[10].s64 + 8480;
	// 82709E34: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82709E38: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82709E3C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82709E40: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82709E44: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709E48: 4BD5CFD9  bl 0x82466e20
	ctx.lr = 0x82709E4C;
	sub_82466E20(ctx, base);
	// 82709E4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709E60 size=112
    let mut pc: u32 = 0x82709E60;
    'dispatch: loop {
        match pc {
            0x82709E60 => {
    //   block [0x82709E60..0x82709ED0)
	// 82709E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709E6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82709E70: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709E74: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82709E78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709E7C: 390B3E28  addi r8, r11, 0x3e28
	ctx.r[8].s64 = ctx.r[11].s64 + 15912;
	// 82709E80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82709E84: 388AB4E4  addi r4, r10, -0x4b1c
	ctx.r[4].s64 = ctx.r[10].s64 + -19228;
	// 82709E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709E8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709E90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709E98: 386A2150  addi r3, r10, 0x2150
	ctx.r[3].s64 = ctx.r[10].s64 + 8528;
	// 82709E9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709EA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709EA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709EA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709EB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709EB4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82709EB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709EBC: 4BD5CF65  bl 0x82466e20
	ctx.lr = 0x82709EC0;
	sub_82466E20(ctx, base);
	// 82709EC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709ED0 size=108
    let mut pc: u32 = 0x82709ED0;
    'dispatch: loop {
        match pc {
            0x82709ED0 => {
    //   block [0x82709ED0..0x82709F3C)
	// 82709ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709EDC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709EE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709EE4: 38EB3E58  addi r7, r11, 0x3e58
	ctx.r[7].s64 = ctx.r[11].s64 + 15960;
	// 82709EE8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82709EEC: 388AB3B4  addi r4, r10, -0x4c4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19532;
	// 82709EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709EF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709EF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82709EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709F00: 386A2180  addi r3, r10, 0x2180
	ctx.r[3].s64 = ctx.r[10].s64 + 8576;
	// 82709F04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82709F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709F14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709F1C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82709F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709F24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709F28: 4BD5CEF9  bl 0x82466e20
	ctx.lr = 0x82709F2C;
	sub_82466E20(ctx, base);
	// 82709F2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709F30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709F34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709F40 size=112
    let mut pc: u32 = 0x82709F40;
    'dispatch: loop {
        match pc {
            0x82709F40 => {
    //   block [0x82709F40..0x82709FB0)
	// 82709F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709F4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709F50: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709F54: 38AA2314  addi r5, r10, 0x2314
	ctx.r[5].s64 = ctx.r[10].s64 + 8980;
	// 82709F58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709F5C: 390B3ED0  addi r8, r11, 0x3ed0
	ctx.r[8].s64 = ctx.r[11].s64 + 16080;
	// 82709F60: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82709F64: 388AB958  addi r4, r10, -0x46a8
	ctx.r[4].s64 = ctx.r[10].s64 + -18088;
	// 82709F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709F6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709F70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709F78: 386A21B0  addi r3, r10, 0x21b0
	ctx.r[3].s64 = ctx.r[10].s64 + 8624;
	// 82709F7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709F94: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 82709F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709F9C: 4BD5CE85  bl 0x82466e20
	ctx.lr = 0x82709FA0;
	sub_82466E20(ctx, base);
	// 82709FA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709FB0 size=96
    let mut pc: u32 = 0x82709FB0;
    'dispatch: loop {
        match pc {
            0x82709FB0 => {
    //   block [0x82709FB0..0x8270A010)
	// 82709FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709FBC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709FC4: 388ABA4C  addi r4, r10, -0x45b4
	ctx.r[4].s64 = ctx.r[10].s64 + -17844;
	// 82709FC8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709FD0: 386A21E0  addi r3, r10, 0x21e0
	ctx.r[3].s64 = ctx.r[10].s64 + 8672;
	// 82709FD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709FDC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82709FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709FEC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82709FF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82709FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709FF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709FFC: 4BD5CE25  bl 0x82466e20
	ctx.lr = 0x8270A000;
	sub_82466E20(ctx, base);
	// 8270A000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A010 size=100
    let mut pc: u32 = 0x8270A010;
    'dispatch: loop {
        match pc {
            0x8270A010 => {
    //   block [0x8270A010..0x8270A074)
	// 8270A010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A01C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A024: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 8270A028: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A02C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A030: 388AB750  addi r4, r10, -0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + -18608;
	// 8270A034: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A044: 386A2210  addi r3, r10, 0x2210
	ctx.r[3].s64 = ctx.r[10].s64 + 8720;
	// 8270A048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A04C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A050: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270A054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A058: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270A05C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8270A060: 4BD5CDC1  bl 0x82466e20
	ctx.lr = 0x8270A064;
	sub_82466E20(ctx, base);
	// 8270A064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A06C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A078 size=104
    let mut pc: u32 = 0x8270A078;
    'dispatch: loop {
        match pc {
            0x8270A078 => {
    //   block [0x8270A078..0x8270A0E0)
	// 8270A078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A084: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8270A088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A08C: 392A3FFC  addi r9, r10, 0x3ffc
	ctx.r[9].s64 = ctx.r[10].s64 + 16380;
	// 8270A090: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270A094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A098: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270A09C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A0AC: 388AB440  addi r4, r10, -0x4bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -19392;
	// 8270A0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A0B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A0B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270A0BC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8270A0C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270A0C4: 386A2254  addi r3, r10, 0x2254
	ctx.r[3].s64 = ctx.r[10].s64 + 8788;
	// 8270A0C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8270A0CC: 4BD5CD55  bl 0x82466e20
	ctx.lr = 0x8270A0D0;
	sub_82466E20(ctx, base);
	// 8270A0D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A0E0 size=112
    let mut pc: u32 = 0x8270A0E0;
    'dispatch: loop {
        match pc {
            0x8270A0E0 => {
    //   block [0x8270A0E0..0x8270A150)
	// 8270A0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A0EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A0F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A0F4: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 8270A0F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A0FC: 390B40FC  addi r8, r11, 0x40fc
	ctx.r[8].s64 = ctx.r[11].s64 + 16636;
	// 8270A100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270A104: 388AB7D4  addi r4, r10, -0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + -18476;
	// 8270A108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A10C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A118: 386A2284  addi r3, r10, 0x2284
	ctx.r[3].s64 = ctx.r[10].s64 + 8836;
	// 8270A11C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A12C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A134: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8270A138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A13C: 4BD5CCE5  bl 0x82466e20
	ctx.lr = 0x8270A140;
	sub_82466E20(ctx, base);
	// 8270A140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A150 size=112
    let mut pc: u32 = 0x8270A150;
    'dispatch: loop {
        match pc {
            0x8270A150 => {
    //   block [0x8270A150..0x8270A1C0)
	// 8270A150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A15C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A160: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A164: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 8270A168: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A16C: 390B4114  addi r8, r11, 0x4114
	ctx.r[8].s64 = ctx.r[11].s64 + 16660;
	// 8270A170: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270A174: 388AB51C  addi r4, r10, -0x4ae4
	ctx.r[4].s64 = ctx.r[10].s64 + -19172;
	// 8270A178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A17C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A180: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A188: 386A22B4  addi r3, r10, 0x22b4
	ctx.r[3].s64 = ctx.r[10].s64 + 8884;
	// 8270A18C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A19C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8270A1A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A1A4: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8270A1A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A1AC: 4BD5CC75  bl 0x82466e20
	ctx.lr = 0x8270A1B0;
	sub_82466E20(ctx, base);
	// 8270A1B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A1B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A1B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A1BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A1C0 size=112
    let mut pc: u32 = 0x8270A1C0;
    'dispatch: loop {
        match pc {
            0x8270A1C0 => {
    //   block [0x8270A1C0..0x8270A230)
	// 8270A1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A1CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A1D0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A1D4: 38AA2210  addi r5, r10, 0x2210
	ctx.r[5].s64 = ctx.r[10].s64 + 8720;
	// 8270A1D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A1DC: 390B4130  addi r8, r11, 0x4130
	ctx.r[8].s64 = ctx.r[11].s64 + 16688;
	// 8270A1E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270A1E4: 388AB97C  addi r4, r10, -0x4684
	ctx.r[4].s64 = ctx.r[10].s64 + -18052;
	// 8270A1E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A1EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A1F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A1F8: 386A22E4  addi r3, r10, 0x22e4
	ctx.r[3].s64 = ctx.r[10].s64 + 8932;
	// 8270A1FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A20C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A214: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8270A218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A21C: 4BD5CC05  bl 0x82466e20
	ctx.lr = 0x8270A220;
	sub_82466E20(ctx, base);
	// 8270A220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A230 size=112
    let mut pc: u32 = 0x8270A230;
    'dispatch: loop {
        match pc {
            0x8270A230 => {
    //   block [0x8270A230..0x8270A2A0)
	// 8270A230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A23C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A240: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A244: 38AA19C0  addi r5, r10, 0x19c0
	ctx.r[5].s64 = ctx.r[10].s64 + 6592;
	// 8270A248: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A24C: 390B41C0  addi r8, r11, 0x41c0
	ctx.r[8].s64 = ctx.r[11].s64 + 16832;
	// 8270A250: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8270A254: 388AB93C  addi r4, r10, -0x46c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18116;
	// 8270A258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A25C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A260: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A268: 386A2314  addi r3, r10, 0x2314
	ctx.r[3].s64 = ctx.r[10].s64 + 8980;
	// 8270A26C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A27C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A284: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8270A288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A28C: 4BD5CB95  bl 0x82466e20
	ctx.lr = 0x8270A290;
	sub_82466E20(ctx, base);
	// 8270A290: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A2A0 size=96
    let mut pc: u32 = 0x8270A2A0;
    'dispatch: loop {
        match pc {
            0x8270A2A0 => {
    //   block [0x8270A2A0..0x8270A300)
	// 8270A2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A2A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A2AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A2B4: 388AB40C  addi r4, r10, -0x4bf4
	ctx.r[4].s64 = ctx.r[10].s64 + -19444;
	// 8270A2B8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A2BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A2C0: 386A2344  addi r3, r10, 0x2344
	ctx.r[3].s64 = ctx.r[10].s64 + 9028;
	// 8270A2C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A2C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A2CC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8270A2D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A2D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A2D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A2DC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8270A2E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270A2E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270A2E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270A2EC: 4BD5CB35  bl 0x82466e20
	ctx.lr = 0x8270A2F0;
	sub_82466E20(ctx, base);
	// 8270A2F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270A300 size=12
    let mut pc: u32 = 0x8270A300;
    'dispatch: loop {
        match pc {
            0x8270A300 => {
    //   block [0x8270A300..0x8270A30C)
	// 8270A300: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 8270A304: 386BCF90  addi r3, r11, -0x3070
	ctx.r[3].s64 = ctx.r[11].s64 + -12400;
	// 8270A308: 4BE28830  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A310 size=108
    let mut pc: u32 = 0x8270A310;
    'dispatch: loop {
        match pc {
            0x8270A310 => {
    //   block [0x8270A310..0x8270A37C)
	// 8270A310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A31C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A320: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A324: 38EB5C88  addi r7, r11, 0x5c88
	ctx.r[7].s64 = ctx.r[11].s64 + 23688;
	// 8270A328: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8270A32C: 388ACB58  addi r4, r10, -0x34a8
	ctx.r[4].s64 = ctx.r[10].s64 + -13480;
	// 8270A330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A334: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A338: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270A33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A340: 386A2684  addi r3, r10, 0x2684
	ctx.r[3].s64 = ctx.r[10].s64 + 9860;
	// 8270A344: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270A348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A34C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A35C: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8270A360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270A368: 4BD5CAB9  bl 0x82466e20
	ctx.lr = 0x8270A36C;
	sub_82466E20(ctx, base);
	// 8270A36C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A380 size=112
    let mut pc: u32 = 0x8270A380;
    'dispatch: loop {
        match pc {
            0x8270A380 => {
    //   block [0x8270A380..0x8270A3F0)
	// 8270A380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A38C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A390: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270A394: 396B5D78  addi r11, r11, 0x5d78
	ctx.r[11].s64 = ctx.r[11].s64 + 23928;
	// 8270A398: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270A39C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A3A0: 390B01C8  addi r8, r11, 0x1c8
	ctx.r[8].s64 = ctx.r[11].s64 + 456;
	// 8270A3A4: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 8270A3A8: 388ACB7C  addi r4, r10, -0x3484
	ctx.r[4].s64 = ctx.r[10].s64 + -13444;
	// 8270A3AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270A3B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A3B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A3B8: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8270A3BC: 38C001A0  li r6, 0x1a0
	ctx.r[6].s64 = 416;
	// 8270A3C0: 386A26B4  addi r3, r10, 0x26b4
	ctx.r[3].s64 = ctx.r[10].s64 + 9908;
	// 8270A3C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8270A3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A3D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8270A3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A3D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8270A3DC: 4BD5CA45  bl 0x82466e20
	ctx.lr = 0x8270A3E0;
	sub_82466E20(ctx, base);
	// 8270A3E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A3E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A3E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A3EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A3F0 size=112
    let mut pc: u32 = 0x8270A3F0;
    'dispatch: loop {
        match pc {
            0x8270A3F0 => {
    //   block [0x8270A3F0..0x8270A460)
	// 8270A3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A3F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A3FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270A400: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A404: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270A408: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A40C: 390B5FA8  addi r8, r11, 0x5fa8
	ctx.r[8].s64 = ctx.r[11].s64 + 24488;
	// 8270A410: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270A414: 388ACF08  addi r4, r10, -0x30f8
	ctx.r[4].s64 = ctx.r[10].s64 + -12536;
	// 8270A418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A41C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A420: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A428: 386A26E4  addi r3, r10, 0x26e4
	ctx.r[3].s64 = ctx.r[10].s64 + 9956;
	// 8270A42C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A43C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8270A440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A444: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8270A448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A44C: 4BD5C9D5  bl 0x82466e20
	ctx.lr = 0x8270A450;
	sub_82466E20(ctx, base);
	// 8270A450: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A45C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A460 size=108
    let mut pc: u32 = 0x8270A460;
    'dispatch: loop {
        match pc {
            0x8270A460 => {
    //   block [0x8270A460..0x8270A4CC)
	// 8270A460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A468: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A46C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A470: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8270A474: 396B602C  addi r11, r11, 0x602c
	ctx.r[11].s64 = ctx.r[11].s64 + 24620;
	// 8270A478: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8270A47C: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 8270A480: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8270A484: 4BE2307D  bl 0x8252d500
	ctx.lr = 0x8270A488;
	sub_8252D500(ctx, base);
	// 8270A488: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8270A48C: 3D008253  lis r8, -0x7dad
	ctx.r[8].s64 = -2108489728;
	// 8270A490: 394BCF24  addi r10, r11, -0x30dc
	ctx.r[10].s64 = ctx.r[11].s64 + -12508;
	// 8270A494: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8270A498: 3D208253  lis r9, -0x7dad
	ctx.r[9].s64 = -2108489728;
	// 8270A49C: 396B2714  addi r11, r11, 0x2714
	ctx.r[11].s64 = ctx.r[11].s64 + 10004;
	// 8270A4A0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8270A4A4: 3948C5B0  addi r10, r8, -0x3a50
	ctx.r[10].s64 = ctx.r[8].s64 + -14928;
	// 8270A4A8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270A4AC: 3949C550  addi r10, r9, -0x3ab0
	ctx.r[10].s64 = ctx.r[9].s64 + -15024;
	// 8270A4B0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8270A4B4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8270A4B8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8270A4BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8270A4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A4C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A4D0 size=112
    let mut pc: u32 = 0x8270A4D0;
    'dispatch: loop {
        match pc {
            0x8270A4D0 => {
    //   block [0x8270A4D0..0x8270A540)
	// 8270A4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A4DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A4E0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A4E4: 38AA2BD4  addi r5, r10, 0x2bd4
	ctx.r[5].s64 = ctx.r[10].s64 + 11220;
	// 8270A4E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A4EC: 390B5FC0  addi r8, r11, 0x5fc0
	ctx.r[8].s64 = ctx.r[11].s64 + 24512;
	// 8270A4F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270A4F4: 388ACF24  addi r4, r10, -0x30dc
	ctx.r[4].s64 = ctx.r[10].s64 + -12508;
	// 8270A4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A4FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A508: 386A2724  addi r3, r10, 0x2724
	ctx.r[3].s64 = ctx.r[10].s64 + 10020;
	// 8270A50C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A524: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8270A528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A52C: 4BD5C8F5  bl 0x82466e20
	ctx.lr = 0x8270A530;
	sub_82466E20(ctx, base);
	// 8270A530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A540 size=100
    let mut pc: u32 = 0x8270A540;
    'dispatch: loop {
        match pc {
            0x8270A540 => {
    //   block [0x8270A540..0x8270A5A4)
	// 8270A540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A54C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270A550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A554: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270A558: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A55C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A560: 388ACC4C  addi r4, r10, -0x33b4
	ctx.r[4].s64 = ctx.r[10].s64 + -13236;
	// 8270A564: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A56C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A574: 386A2754  addi r3, r10, 0x2754
	ctx.r[3].s64 = ctx.r[10].s64 + 10068;
	// 8270A578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A57C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A580: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270A584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A588: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270A58C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8270A590: 4BD5C891  bl 0x82466e20
	ctx.lr = 0x8270A594;
	sub_82466E20(ctx, base);
	// 8270A594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A5A8 size=100
    let mut pc: u32 = 0x8270A5A8;
    'dispatch: loop {
        match pc {
            0x8270A5A8 => {
    //   block [0x8270A5A8..0x8270A60C)
	// 8270A5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A5B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270A5B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A5BC: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270A5C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A5C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A5C8: 388ACC68  addi r4, r10, -0x3398
	ctx.r[4].s64 = ctx.r[10].s64 + -13208;
	// 8270A5CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A5D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A5D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A5DC: 386A2784  addi r3, r10, 0x2784
	ctx.r[3].s64 = ctx.r[10].s64 + 10116;
	// 8270A5E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A5E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A5E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270A5EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A5F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270A5F4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8270A5F8: 4BD5C829  bl 0x82466e20
	ctx.lr = 0x8270A5FC;
	sub_82466E20(ctx, base);
	// 8270A5FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A610 size=108
    let mut pc: u32 = 0x8270A610;
    'dispatch: loop {
        match pc {
            0x8270A610 => {
    //   block [0x8270A610..0x8270A67C)
	// 8270A610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A61C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A620: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A624: 38EB6058  addi r7, r11, 0x6058
	ctx.r[7].s64 = ctx.r[11].s64 + 24664;
	// 8270A628: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8270A62C: 388ACCF4  addi r4, r10, -0x330c
	ctx.r[4].s64 = ctx.r[10].s64 + -13068;
	// 8270A630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A634: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A638: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270A63C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A640: 386A27B4  addi r3, r10, 0x27b4
	ctx.r[3].s64 = ctx.r[10].s64 + 10164;
	// 8270A644: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270A648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A64C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A65C: 38C00064  li r6, 0x64
	ctx.r[6].s64 = 100;
	// 8270A660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270A668: 4BD5C7B9  bl 0x82466e20
	ctx.lr = 0x8270A66C;
	sub_82466E20(ctx, base);
	// 8270A66C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A680 size=108
    let mut pc: u32 = 0x8270A680;
    'dispatch: loop {
        match pc {
            0x8270A680 => {
    //   block [0x8270A680..0x8270A6EC)
	// 8270A680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A68C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A694: 38EB6148  addi r7, r11, 0x6148
	ctx.r[7].s64 = ctx.r[11].s64 + 24904;
	// 8270A698: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8270A69C: 388ACD24  addi r4, r10, -0x32dc
	ctx.r[4].s64 = ctx.r[10].s64 + -13020;
	// 8270A6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A6A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A6A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270A6AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A6B0: 386A27E4  addi r3, r10, 0x27e4
	ctx.r[3].s64 = ctx.r[10].s64 + 10212;
	// 8270A6B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270A6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A6BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A6CC: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 8270A6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A6D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270A6D8: 4BD5C749  bl 0x82466e20
	ctx.lr = 0x8270A6DC;
	sub_82466E20(ctx, base);
	// 8270A6DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A6E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A6E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A6F0 size=108
    let mut pc: u32 = 0x8270A6F0;
    'dispatch: loop {
        match pc {
            0x8270A6F0 => {
    //   block [0x8270A6F0..0x8270A75C)
	// 8270A6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A6FC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A704: 38EB6190  addi r7, r11, 0x6190
	ctx.r[7].s64 = ctx.r[11].s64 + 24976;
	// 8270A708: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8270A70C: 388ACD44  addi r4, r10, -0x32bc
	ctx.r[4].s64 = ctx.r[10].s64 + -12988;
	// 8270A710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A714: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A718: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270A71C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A720: 386A2814  addi r3, r10, 0x2814
	ctx.r[3].s64 = ctx.r[10].s64 + 10260;
	// 8270A724: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270A728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A73C: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8270A740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270A748: 4BD5C6D9  bl 0x82466e20
	ctx.lr = 0x8270A74C;
	sub_82466E20(ctx, base);
	// 8270A74C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A760 size=108
    let mut pc: u32 = 0x8270A760;
    'dispatch: loop {
        match pc {
            0x8270A760 => {
    //   block [0x8270A760..0x8270A7CC)
	// 8270A760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A76C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A770: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A774: 38EB6268  addi r7, r11, 0x6268
	ctx.r[7].s64 = ctx.r[11].s64 + 25192;
	// 8270A778: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8270A77C: 388ACD68  addi r4, r10, -0x3298
	ctx.r[4].s64 = ctx.r[10].s64 + -12952;
	// 8270A780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A784: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A788: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270A78C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A790: 386A2844  addi r3, r10, 0x2844
	ctx.r[3].s64 = ctx.r[10].s64 + 10308;
	// 8270A794: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270A798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A7A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A7AC: 38C00048  li r6, 0x48
	ctx.r[6].s64 = 72;
	// 8270A7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A7B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270A7B8: 4BD5C669  bl 0x82466e20
	ctx.lr = 0x8270A7BC;
	sub_82466E20(ctx, base);
	// 8270A7BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A7C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A7C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A7D0 size=112
    let mut pc: u32 = 0x8270A7D0;
    'dispatch: loop {
        match pc {
            0x8270A7D0 => {
    //   block [0x8270A7D0..0x8270A840)
	// 8270A7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A7DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A7E0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A7E4: 38AA2754  addi r5, r10, 0x2754
	ctx.r[5].s64 = ctx.r[10].s64 + 10068;
	// 8270A7E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A7EC: 390B6280  addi r8, r11, 0x6280
	ctx.r[8].s64 = ctx.r[11].s64 + 25216;
	// 8270A7F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8270A7F4: 388ACC80  addi r4, r10, -0x3380
	ctx.r[4].s64 = ctx.r[10].s64 + -13184;
	// 8270A7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A7FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A808: 386A2874  addi r3, r10, 0x2874
	ctx.r[3].s64 = ctx.r[10].s64 + 10356;
	// 8270A80C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A824: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8270A828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A82C: 4BD5C5F5  bl 0x82466e20
	ctx.lr = 0x8270A830;
	sub_82466E20(ctx, base);
	// 8270A830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A840 size=112
    let mut pc: u32 = 0x8270A840;
    'dispatch: loop {
        match pc {
            0x8270A840 => {
    //   block [0x8270A840..0x8270A8B0)
	// 8270A840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A84C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A850: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A854: 38AA2784  addi r5, r10, 0x2784
	ctx.r[5].s64 = ctx.r[10].s64 + 10116;
	// 8270A858: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A85C: 390B62E0  addi r8, r11, 0x62e0
	ctx.r[8].s64 = ctx.r[11].s64 + 25312;
	// 8270A860: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8270A864: 388ACCA4  addi r4, r10, -0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + -13148;
	// 8270A868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A86C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A878: 386A28A4  addi r3, r10, 0x28a4
	ctx.r[3].s64 = ctx.r[10].s64 + 10404;
	// 8270A87C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A894: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8270A898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A89C: 4BD5C585  bl 0x82466e20
	ctx.lr = 0x8270A8A0;
	sub_82466E20(ctx, base);
	// 8270A8A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A8B0 size=108
    let mut pc: u32 = 0x8270A8B0;
    'dispatch: loop {
        match pc {
            0x8270A8B0 => {
    //   block [0x8270A8B0..0x8270A91C)
	// 8270A8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A8BC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A8C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A8C4: 38EB6378  addi r7, r11, 0x6378
	ctx.r[7].s64 = ctx.r[11].s64 + 25464;
	// 8270A8C8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8270A8CC: 388ACDB4  addi r4, r10, -0x324c
	ctx.r[4].s64 = ctx.r[10].s64 + -12876;
	// 8270A8D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A8D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A8D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270A8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A8E0: 386A28D4  addi r3, r10, 0x28d4
	ctx.r[3].s64 = ctx.r[10].s64 + 10452;
	// 8270A8E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270A8E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A8EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A8F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A8F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A8F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A8FC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8270A900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270A908: 4BD5C519  bl 0x82466e20
	ctx.lr = 0x8270A90C;
	sub_82466E20(ctx, base);
	// 8270A90C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A920 size=112
    let mut pc: u32 = 0x8270A920;
    'dispatch: loop {
        match pc {
            0x8270A920 => {
    //   block [0x8270A920..0x8270A990)
	// 8270A920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A92C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270A930: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A934: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270A938: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A93C: 390B63C0  addi r8, r11, 0x63c0
	ctx.r[8].s64 = ctx.r[11].s64 + 25536;
	// 8270A940: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270A944: 388ACDE4  addi r4, r10, -0x321c
	ctx.r[4].s64 = ctx.r[10].s64 + -12828;
	// 8270A948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A94C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A958: 386A2904  addi r3, r10, 0x2904
	ctx.r[3].s64 = ctx.r[10].s64 + 10500;
	// 8270A95C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A974: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8270A978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A97C: 4BD5C4A5  bl 0x82466e20
	ctx.lr = 0x8270A980;
	sub_82466E20(ctx, base);
	// 8270A980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A990 size=112
    let mut pc: u32 = 0x8270A990;
    'dispatch: loop {
        match pc {
            0x8270A990 => {
    //   block [0x8270A990..0x8270AA00)
	// 8270A990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A99C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A9A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A9A4: 38AA2B44  addi r5, r10, 0x2b44
	ctx.r[5].s64 = ctx.r[10].s64 + 11076;
	// 8270A9A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A9AC: 390B63D8  addi r8, r11, 0x63d8
	ctx.r[8].s64 = ctx.r[11].s64 + 25560;
	// 8270A9B0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8270A9B4: 388ACE6C  addi r4, r10, -0x3194
	ctx.r[4].s64 = ctx.r[10].s64 + -12692;
	// 8270A9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A9BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A9C8: 386A2934  addi r3, r10, 0x2934
	ctx.r[3].s64 = ctx.r[10].s64 + 10548;
	// 8270A9CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A9E4: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 8270A9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A9EC: 4BD5C435  bl 0x82466e20
	ctx.lr = 0x8270A9F0;
	sub_82466E20(ctx, base);
	// 8270A9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


