pub fn sub_825FC758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC758 size=108
    let mut pc: u32 = 0x825FC758;
    'dispatch: loop {
        match pc {
            0x825FC758 => {
    //   block [0x825FC758..0x825FC7C4)
	// 825FC758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC764: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC768: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC76C: 38EB3360  addi r7, r11, 0x3360
	ctx.r[7].s64 = ctx.r[11].s64 + 13152;
	// 825FC770: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FC774: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 825FC778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC77C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC780: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC788: 386A0B6C  addi r3, r10, 0xb6c
	ctx.r[3].s64 = ctx.r[10].s64 + 2924;
	// 825FC78C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC79C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC7AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC7B0: 4BE6A671  bl 0x82466e20
	ctx.lr = 0x825FC7B4;
	sub_82466E20(ctx, base);
	// 825FC7B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC7B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC7BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC7C8 size=108
    let mut pc: u32 = 0x825FC7C8;
    'dispatch: loop {
        match pc {
            0x825FC7C8 => {
    //   block [0x825FC7C8..0x825FC834)
	// 825FC7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC7D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC7D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC7DC: 38EB3378  addi r7, r11, 0x3378
	ctx.r[7].s64 = ctx.r[11].s64 + 13176;
	// 825FC7E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC7E4: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 825FC7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC7EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC7F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC7F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC7F8: 386A0B9C  addi r3, r10, 0xb9c
	ctx.r[3].s64 = ctx.r[10].s64 + 2972;
	// 825FC7FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC80C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC81C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC820: 4BE6A601  bl 0x82466e20
	ctx.lr = 0x825FC824;
	sub_82466E20(ctx, base);
	// 825FC824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC82C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC838 size=112
    let mut pc: u32 = 0x825FC838;
    'dispatch: loop {
        match pc {
            0x825FC838 => {
    //   block [0x825FC838..0x825FC8A8)
	// 825FC838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC844: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FC848: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC84C: 392A8C94  addi r9, r10, -0x736c
	ctx.r[9].s64 = ctx.r[10].s64 + -29548;
	// 825FC850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FC854: 390B33A8  addi r8, r11, 0x33a8
	ctx.r[8].s64 = ctx.r[11].s64 + 13224;
	// 825FC858: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825FC85C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 825FC860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC864: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FC86C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC870: 386A0BCC  addi r3, r10, 0xbcc
	ctx.r[3].s64 = ctx.r[10].s64 + 3020;
	// 825FC874: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FC878: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FC87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC88C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC894: 4BE6A58D  bl 0x82466e20
	ctx.lr = 0x825FC898;
	sub_82466E20(ctx, base);
	// 825FC898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC8A8 size=108
    let mut pc: u32 = 0x825FC8A8;
    'dispatch: loop {
        match pc {
            0x825FC8A8 => {
    //   block [0x825FC8A8..0x825FC914)
	// 825FC8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC8B4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC8B8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC8BC: 38EB3420  addi r7, r11, 0x3420
	ctx.r[7].s64 = ctx.r[11].s64 + 13344;
	// 825FC8C0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825FC8C4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 825FC8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC8CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC8D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC8D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC8D8: 386A0BFC  addi r3, r10, 0xbfc
	ctx.r[3].s64 = ctx.r[10].s64 + 3068;
	// 825FC8DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC8EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC8F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC8FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC900: 4BE6A521  bl 0x82466e20
	ctx.lr = 0x825FC904;
	sub_82466E20(ctx, base);
	// 825FC904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC90C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FC918 size=24
    let mut pc: u32 = 0x825FC918;
    'dispatch: loop {
        match pc {
            0x825FC918 => {
    //   block [0x825FC918..0x825FC930)
	// 825FC918: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC91C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FC920: 394AA848  addi r10, r10, -0x57b8
	ctx.r[10].s64 = ctx.r[10].s64 + -22456;
	// 825FC924: 816B3510  lwz r11, 0x3510(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13584 as u32) ) } as u64;
	// 825FC928: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825FC92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC930 size=108
    let mut pc: u32 = 0x825FC930;
    'dispatch: loop {
        match pc {
            0x825FC930 => {
    //   block [0x825FC930..0x825FC99C)
	// 825FC930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC93C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FC940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FC944: 38EBA848  addi r7, r11, -0x57b8
	ctx.r[7].s64 = ctx.r[11].s64 + -22456;
	// 825FC948: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC94C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 825FC950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC954: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC960: 386A0C2C  addi r3, r10, 0xc2c
	ctx.r[3].s64 = ctx.r[10].s64 + 3116;
	// 825FC964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC988: 4BE6A499  bl 0x82466e20
	ctx.lr = 0x825FC98C;
	sub_82466E20(ctx, base);
	// 825FC98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FC9A0 size=24
    let mut pc: u32 = 0x825FC9A0;
    'dispatch: loop {
        match pc {
            0x825FC9A0 => {
    //   block [0x825FC9A0..0x825FC9B8)
	// 825FC9A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC9A4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FC9A8: 394AA878  addi r10, r10, -0x5788
	ctx.r[10].s64 = ctx.r[10].s64 + -22408;
	// 825FC9AC: 816B3510  lwz r11, 0x3510(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13584 as u32) ) } as u64;
	// 825FC9B0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825FC9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC9B8 size=108
    let mut pc: u32 = 0x825FC9B8;
    'dispatch: loop {
        match pc {
            0x825FC9B8 => {
    //   block [0x825FC9B8..0x825FCA24)
	// 825FC9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC9C4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FC9C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FC9CC: 38EBA878  addi r7, r11, -0x5788
	ctx.r[7].s64 = ctx.r[11].s64 + -22408;
	// 825FC9D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC9D4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 825FC9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC9DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC9E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC9E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC9E8: 386A0C5C  addi r3, r10, 0xc5c
	ctx.r[3].s64 = ctx.r[10].s64 + 3164;
	// 825FC9EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC9F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC9F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC9F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCA00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCA04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCA08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCA0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCA10: 4BE6A411  bl 0x82466e20
	ctx.lr = 0x825FCA14;
	sub_82466E20(ctx, base);
	// 825FCA14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCA28 size=108
    let mut pc: u32 = 0x825FCA28;
    'dispatch: loop {
        match pc {
            0x825FCA28 => {
    //   block [0x825FCA28..0x825FCA94)
	// 825FCA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCA34: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCA38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCA3C: 38EB34F8  addi r7, r11, 0x34f8
	ctx.r[7].s64 = ctx.r[11].s64 + 13560;
	// 825FCA40: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FCA44: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 825FCA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCA4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCA50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCA58: 386A0C8C  addi r3, r10, 0xc8c
	ctx.r[3].s64 = ctx.r[10].s64 + 3212;
	// 825FCA5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCA60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCA64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCA6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCA74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCA7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCA80: 4BE6A3A1  bl 0x82466e20
	ctx.lr = 0x825FCA84;
	sub_82466E20(ctx, base);
	// 825FCA84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCA90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FCA98 size=24
    let mut pc: u32 = 0x825FCA98;
    'dispatch: loop {
        match pc {
            0x825FCA98 => {
    //   block [0x825FCA98..0x825FCAB0)
	// 825FCA98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCA9C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FCAA0: 394AA8A8  addi r10, r10, -0x5758
	ctx.r[10].s64 = ctx.r[10].s64 + -22360;
	// 825FCAA4: 816B3510  lwz r11, 0x3510(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13584 as u32) ) } as u64;
	// 825FCAA8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825FCAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCAB0 size=108
    let mut pc: u32 = 0x825FCAB0;
    'dispatch: loop {
        match pc {
            0x825FCAB0 => {
    //   block [0x825FCAB0..0x825FCB1C)
	// 825FCAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCABC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FCAC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCAC4: 38EBA8A8  addi r7, r11, -0x5758
	ctx.r[7].s64 = ctx.r[11].s64 + -22360;
	// 825FCAC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FCACC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 825FCAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCAD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCAD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCAE0: 386A0CBC  addi r3, r10, 0xcbc
	ctx.r[3].s64 = ctx.r[10].s64 + 3260;
	// 825FCAE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCAF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCB04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCB08: 4BE6A319  bl 0x82466e20
	ctx.lr = 0x825FCB0C;
	sub_82466E20(ctx, base);
	// 825FCB0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCB20 size=112
    let mut pc: u32 = 0x825FCB20;
    'dispatch: loop {
        match pc {
            0x825FCB20 => {
    //   block [0x825FCB20..0x825FCB90)
	// 825FCB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCB2C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FCB30: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCB34: 392A8CD8  addi r9, r10, -0x7328
	ctx.r[9].s64 = ctx.r[10].s64 + -29480;
	// 825FCB38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCB3C: 390B3514  addi r8, r11, 0x3514
	ctx.r[8].s64 = ctx.r[11].s64 + 13588;
	// 825FCB40: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825FCB44: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 825FCB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCB4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCB50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FCB54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCB58: 386A0CEC  addi r3, r10, 0xcec
	ctx.r[3].s64 = ctx.r[10].s64 + 3308;
	// 825FCB5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FCB60: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FCB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCB68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCB6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCB70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCB74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCB78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCB7C: 4BE6A2A5  bl 0x82466e20
	ctx.lr = 0x825FCB80;
	sub_82466E20(ctx, base);
	// 825FCB80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCB84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCB88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCB90 size=108
    let mut pc: u32 = 0x825FCB90;
    'dispatch: loop {
        match pc {
            0x825FCB90 => {
    //   block [0x825FCB90..0x825FCBFC)
	// 825FCB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCB9C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCBA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCBA4: 38EB3544  addi r7, r11, 0x3544
	ctx.r[7].s64 = ctx.r[11].s64 + 13636;
	// 825FCBA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FCBAC: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 825FCBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCBB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCBB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCBBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCBC0: 386A0D1C  addi r3, r10, 0xd1c
	ctx.r[3].s64 = ctx.r[10].s64 + 3356;
	// 825FCBC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCBCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCBE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCBE8: 4BE6A239  bl 0x82466e20
	ctx.lr = 0x825FCBEC;
	sub_82466E20(ctx, base);
	// 825FCBEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCC00 size=108
    let mut pc: u32 = 0x825FCC00;
    'dispatch: loop {
        match pc {
            0x825FCC00 => {
    //   block [0x825FCC00..0x825FCC6C)
	// 825FCC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCC08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCC0C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCC10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCC14: 38EB3574  addi r7, r11, 0x3574
	ctx.r[7].s64 = ctx.r[11].s64 + 13684;
	// 825FCC18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FCC1C: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 825FCC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCC24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCC28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCC30: 386A0D4C  addi r3, r10, 0xd4c
	ctx.r[3].s64 = ctx.r[10].s64 + 3404;
	// 825FCC34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCC54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCC58: 4BE6A1C9  bl 0x82466e20
	ctx.lr = 0x825FCC5C;
	sub_82466E20(ctx, base);
	// 825FCC5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCC70 size=108
    let mut pc: u32 = 0x825FCC70;
    'dispatch: loop {
        match pc {
            0x825FCC70 => {
    //   block [0x825FCC70..0x825FCCDC)
	// 825FCC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCC78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCC7C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCC84: 38EB358C  addi r7, r11, 0x358c
	ctx.r[7].s64 = ctx.r[11].s64 + 13708;
	// 825FCC88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FCC8C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 825FCC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCC94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCC98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCCA0: 386A0D7C  addi r3, r10, 0xd7c
	ctx.r[3].s64 = ctx.r[10].s64 + 3452;
	// 825FCCA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCCA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCCAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCCB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCCB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCCB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCCBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCCC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCCC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCCC8: 4BE6A159  bl 0x82466e20
	ctx.lr = 0x825FCCCC;
	sub_82466E20(ctx, base);
	// 825FCCCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCCE0 size=112
    let mut pc: u32 = 0x825FCCE0;
    'dispatch: loop {
        match pc {
            0x825FCCE0 => {
    //   block [0x825FCCE0..0x825FCD50)
	// 825FCCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCCE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCCEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCCF0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCCF4: 38AA0DDC  addi r5, r10, 0xddc
	ctx.r[5].s64 = ctx.r[10].s64 + 3548;
	// 825FCCF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCCFC: 390B35BC  addi r8, r11, 0x35bc
	ctx.r[8].s64 = ctx.r[11].s64 + 13756;
	// 825FCD00: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FCD04: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 825FCD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCD0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCD10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FCD14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCD18: 386A0DAC  addi r3, r10, 0xdac
	ctx.r[3].s64 = ctx.r[10].s64 + 3500;
	// 825FCD1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FCD20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCD24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCD28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCD2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCD30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCD34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCD38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCD3C: 4BE6A0E5  bl 0x82466e20
	ctx.lr = 0x825FCD40;
	sub_82466E20(ctx, base);
	// 825FCD40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCD44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCD48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCD4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCD50 size=108
    let mut pc: u32 = 0x825FCD50;
    'dispatch: loop {
        match pc {
            0x825FCD50 => {
    //   block [0x825FCD50..0x825FCDBC)
	// 825FCD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCD5C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCD60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCD64: 38EB35D4  addi r7, r11, 0x35d4
	ctx.r[7].s64 = ctx.r[11].s64 + 13780;
	// 825FCD68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FCD6C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 825FCD70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCD74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCD78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCD7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCD80: 386A0DDC  addi r3, r10, 0xddc
	ctx.r[3].s64 = ctx.r[10].s64 + 3548;
	// 825FCD84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCD88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCD8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCD90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCD94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCD98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCDA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCDA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCDA8: 4BE6A079  bl 0x82466e20
	ctx.lr = 0x825FCDAC;
	sub_82466E20(ctx, base);
	// 825FCDAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCDB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCDB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCDB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCDC0 size=108
    let mut pc: u32 = 0x825FCDC0;
    'dispatch: loop {
        match pc {
            0x825FCDC0 => {
    //   block [0x825FCDC0..0x825FCE2C)
	// 825FCDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCDC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCDCC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCDD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCDD4: 38EB3604  addi r7, r11, 0x3604
	ctx.r[7].s64 = ctx.r[11].s64 + 13828;
	// 825FCDD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FCDDC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 825FCDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCDE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCDE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCDEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCDF0: 386A0E0C  addi r3, r10, 0xe0c
	ctx.r[3].s64 = ctx.r[10].s64 + 3596;
	// 825FCDF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCDF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCDFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCE04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCE14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCE18: 4BE6A009  bl 0x82466e20
	ctx.lr = 0x825FCE1C;
	sub_82466E20(ctx, base);
	// 825FCE1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCE28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCE30 size=108
    let mut pc: u32 = 0x825FCE30;
    'dispatch: loop {
        match pc {
            0x825FCE30 => {
    //   block [0x825FCE30..0x825FCE9C)
	// 825FCE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCE38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCE3C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCE44: 38EB361C  addi r7, r11, 0x361c
	ctx.r[7].s64 = ctx.r[11].s64 + 13852;
	// 825FCE48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FCE4C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 825FCE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCE54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCE58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCE5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCE60: 386A0E3C  addi r3, r10, 0xe3c
	ctx.r[3].s64 = ctx.r[10].s64 + 3644;
	// 825FCE64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCE6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCE84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCE88: 4BE69F99  bl 0x82466e20
	ctx.lr = 0x825FCE8C;
	sub_82466E20(ctx, base);
	// 825FCE8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCE90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCE94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCE98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCEA0 size=108
    let mut pc: u32 = 0x825FCEA0;
    'dispatch: loop {
        match pc {
            0x825FCEA0 => {
    //   block [0x825FCEA0..0x825FCF0C)
	// 825FCEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCEAC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCEB4: 38EB3650  addi r7, r11, 0x3650
	ctx.r[7].s64 = ctx.r[11].s64 + 13904;
	// 825FCEB8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825FCEBC: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 825FCEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCEC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCEC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCED0: 386A0E6C  addi r3, r10, 0xe6c
	ctx.r[3].s64 = ctx.r[10].s64 + 3692;
	// 825FCED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCEDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCEF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCEF8: 4BE69F29  bl 0x82466e20
	ctx.lr = 0x825FCEFC;
	sub_82466E20(ctx, base);
	// 825FCEFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCF10 size=108
    let mut pc: u32 = 0x825FCF10;
    'dispatch: loop {
        match pc {
            0x825FCF10 => {
    //   block [0x825FCF10..0x825FCF7C)
	// 825FCF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCF18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCF1C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCF20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCF24: 38EB36F8  addi r7, r11, 0x36f8
	ctx.r[7].s64 = ctx.r[11].s64 + 14072;
	// 825FCF28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FCF2C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 825FCF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCF34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCF38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCF40: 386A0E9C  addi r3, r10, 0xe9c
	ctx.r[3].s64 = ctx.r[10].s64 + 3740;
	// 825FCF44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCF48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCF4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCF64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCF68: 4BE69EB9  bl 0x82466e20
	ctx.lr = 0x825FCF6C;
	sub_82466E20(ctx, base);
	// 825FCF6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCF80 size=108
    let mut pc: u32 = 0x825FCF80;
    'dispatch: loop {
        match pc {
            0x825FCF80 => {
    //   block [0x825FCF80..0x825FCFEC)
	// 825FCF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCF8C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FCF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FCF94: 38EB3728  addi r7, r11, 0x3728
	ctx.r[7].s64 = ctx.r[11].s64 + 14120;
	// 825FCF98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FCF9C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 825FCFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FCFA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FCFA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FCFAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FCFB0: 386A0ECC  addi r3, r10, 0xecc
	ctx.r[3].s64 = ctx.r[10].s64 + 3788;
	// 825FCFB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FCFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FCFBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FCFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FCFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FCFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FCFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FCFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FCFD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FCFD8: 4BE69E49  bl 0x82466e20
	ctx.lr = 0x825FCFDC;
	sub_82466E20(ctx, base);
	// 825FCFDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FCFE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FCFE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FCFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FCFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FCFF0 size=108
    let mut pc: u32 = 0x825FCFF0;
    'dispatch: loop {
        match pc {
            0x825FCFF0 => {
    //   block [0x825FCFF0..0x825FD05C)
	// 825FCFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FCFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FCFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FCFFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD004: 38EB3740  addi r7, r11, 0x3740
	ctx.r[7].s64 = ctx.r[11].s64 + 14144;
	// 825FD008: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FD00C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 825FD010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD014: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD018: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD020: 386A0EFC  addi r3, r10, 0xefc
	ctx.r[3].s64 = ctx.r[10].s64 + 3836;
	// 825FD024: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD044: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD048: 4BE69DD9  bl 0x82466e20
	ctx.lr = 0x825FD04C;
	sub_82466E20(ctx, base);
	// 825FD04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD060 size=112
    let mut pc: u32 = 0x825FD060;
    'dispatch: loop {
        match pc {
            0x825FD060 => {
    //   block [0x825FD060..0x825FD0D0)
	// 825FD060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD06C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD070: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD074: 38AA0D4C  addi r5, r10, 0xd4c
	ctx.r[5].s64 = ctx.r[10].s64 + 3404;
	// 825FD078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD07C: 390B3770  addi r8, r11, 0x3770
	ctx.r[8].s64 = ctx.r[11].s64 + 14192;
	// 825FD080: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825FD084: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 825FD088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD08C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FD094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD098: 386A0F2C  addi r3, r10, 0xf2c
	ctx.r[3].s64 = ctx.r[10].s64 + 3884;
	// 825FD09C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FD0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD0BC: 4BE69D65  bl 0x82466e20
	ctx.lr = 0x825FD0C0;
	sub_82466E20(ctx, base);
	// 825FD0C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FD0D0 size=24
    let mut pc: u32 = 0x825FD0D0;
    'dispatch: loop {
        match pc {
            0x825FD0D0 => {
    //   block [0x825FD0D0..0x825FD0E8)
	// 825FD0D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD0D4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FD0D8: 394AA8D8  addi r10, r10, -0x5728
	ctx.r[10].s64 = ctx.r[10].s64 + -22312;
	// 825FD0DC: 816B364C  lwz r11, 0x364c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13900 as u32) ) } as u64;
	// 825FD0E0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825FD0E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD0E8 size=112
    let mut pc: u32 = 0x825FD0E8;
    'dispatch: loop {
        match pc {
            0x825FD0E8 => {
    //   block [0x825FD0E8..0x825FD158)
	// 825FD0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD0F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD0F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FD0F8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FD0FC: 392A8D04  addi r9, r10, -0x72fc
	ctx.r[9].s64 = ctx.r[10].s64 + -29436;
	// 825FD100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD104: 390BA8D8  addi r8, r11, -0x5728
	ctx.r[8].s64 = ctx.r[11].s64 + -22312;
	// 825FD108: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825FD10C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 825FD110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD114: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FD11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD120: 386A0F5C  addi r3, r10, 0xf5c
	ctx.r[3].s64 = ctx.r[10].s64 + 3932;
	// 825FD124: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FD128: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FD12C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD13C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD144: 4BE69CDD  bl 0x82466e20
	ctx.lr = 0x825FD148;
	sub_82466E20(ctx, base);
	// 825FD148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD158 size=108
    let mut pc: u32 = 0x825FD158;
    'dispatch: loop {
        match pc {
            0x825FD158 => {
    //   block [0x825FD158..0x825FD1C4)
	// 825FD158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD164: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD16C: 38EB381C  addi r7, r11, 0x381c
	ctx.r[7].s64 = ctx.r[11].s64 + 14364;
	// 825FD170: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FD174: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 825FD178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD17C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD180: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD188: 386A0F8C  addi r3, r10, 0xf8c
	ctx.r[3].s64 = ctx.r[10].s64 + 3980;
	// 825FD18C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD19C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD1A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD1A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD1A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD1AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD1B0: 4BE69C71  bl 0x82466e20
	ctx.lr = 0x825FD1B4;
	sub_82466E20(ctx, base);
	// 825FD1B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD1B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD1BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD1C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD1C8 size=116
    let mut pc: u32 = 0x825FD1C8;
    'dispatch: loop {
        match pc {
            0x825FD1C8 => {
    //   block [0x825FD1C8..0x825FD23C)
	// 825FD1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD1D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD1D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD1D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FD1DC: 390B3850  addi r8, r11, 0x3850
	ctx.r[8].s64 = ctx.r[11].s64 + 14416;
	// 825FD1E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD1E4: 392A8D48  addi r9, r10, -0x72b8
	ctx.r[9].s64 = ctx.r[10].s64 + -29368;
	// 825FD1E8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD1EC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825FD1F0: 38AA0D4C  addi r5, r10, 0xd4c
	ctx.r[5].s64 = ctx.r[10].s64 + 3404;
	// 825FD1F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FD1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD1FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD20C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FD210: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 825FD214: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FD218: 386B0FBC  addi r3, r11, 0xfbc
	ctx.r[3].s64 = ctx.r[11].s64 + 4028;
	// 825FD21C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FD220: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD228: 4BE69BF9  bl 0x82466e20
	ctx.lr = 0x825FD22C;
	sub_82466E20(ctx, base);
	// 825FD22C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FD240 size=24
    let mut pc: u32 = 0x825FD240;
    'dispatch: loop {
        match pc {
            0x825FD240 => {
    //   block [0x825FD240..0x825FD258)
	// 825FD240: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD244: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FD248: 394AA950  addi r10, r10, -0x56b0
	ctx.r[10].s64 = ctx.r[10].s64 + -22192;
	// 825FD24C: 816B384C  lwz r11, 0x384c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14412 as u32) ) } as u64;
	// 825FD250: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825FD254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD258 size=112
    let mut pc: u32 = 0x825FD258;
    'dispatch: loop {
        match pc {
            0x825FD258 => {
    //   block [0x825FD258..0x825FD2C8)
	// 825FD258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD264: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FD268: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FD26C: 392A8D84  addi r9, r10, -0x727c
	ctx.r[9].s64 = ctx.r[10].s64 + -29308;
	// 825FD270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD274: 390BA950  addi r8, r11, -0x56b0
	ctx.r[8].s64 = ctx.r[11].s64 + -22192;
	// 825FD278: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825FD27C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 825FD280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD284: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FD28C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD290: 386A0FEC  addi r3, r10, 0xfec
	ctx.r[3].s64 = ctx.r[10].s64 + 4076;
	// 825FD294: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FD298: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FD29C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD2A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD2AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD2B4: 4BE69B6D  bl 0x82466e20
	ctx.lr = 0x825FD2B8;
	sub_82466E20(ctx, base);
	// 825FD2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD2C8 size=108
    let mut pc: u32 = 0x825FD2C8;
    'dispatch: loop {
        match pc {
            0x825FD2C8 => {
    //   block [0x825FD2C8..0x825FD334)
	// 825FD2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD2D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD2D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD2DC: 38EB3910  addi r7, r11, 0x3910
	ctx.r[7].s64 = ctx.r[11].s64 + 14608;
	// 825FD2E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FD2E4: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 825FD2E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD2EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD2F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD2F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD2F8: 386A101C  addi r3, r10, 0x101c
	ctx.r[3].s64 = ctx.r[10].s64 + 4124;
	// 825FD2FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD30C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD31C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD320: 4BE69B01  bl 0x82466e20
	ctx.lr = 0x825FD324;
	sub_82466E20(ctx, base);
	// 825FD324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD32C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD338 size=108
    let mut pc: u32 = 0x825FD338;
    'dispatch: loop {
        match pc {
            0x825FD338 => {
    //   block [0x825FD338..0x825FD3A4)
	// 825FD338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD344: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD34C: 38EB3928  addi r7, r11, 0x3928
	ctx.r[7].s64 = ctx.r[11].s64 + 14632;
	// 825FD350: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FD354: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 825FD358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD35C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD360: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD368: 386A104C  addi r3, r10, 0x104c
	ctx.r[3].s64 = ctx.r[10].s64 + 4172;
	// 825FD36C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD38C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD390: 4BE69A91  bl 0x82466e20
	ctx.lr = 0x825FD394;
	sub_82466E20(ctx, base);
	// 825FD394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FD3A8 size=24
    let mut pc: u32 = 0x825FD3A8;
    'dispatch: loop {
        match pc {
            0x825FD3A8 => {
    //   block [0x825FD3A8..0x825FD3C0)
	// 825FD3A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD3AC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FD3B0: 394AA998  addi r10, r10, -0x5668
	ctx.r[10].s64 = ctx.r[10].s64 + -22120;
	// 825FD3B4: 816B3958  lwz r11, 0x3958(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14680 as u32) ) } as u64;
	// 825FD3B8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825FD3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD3C0 size=112
    let mut pc: u32 = 0x825FD3C0;
    'dispatch: loop {
        match pc {
            0x825FD3C0 => {
    //   block [0x825FD3C0..0x825FD430)
	// 825FD3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD3C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD3CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FD3D0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FD3D4: 392A8DC0  addi r9, r10, -0x7240
	ctx.r[9].s64 = ctx.r[10].s64 + -29248;
	// 825FD3D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD3DC: 390BA998  addi r8, r11, -0x5668
	ctx.r[8].s64 = ctx.r[11].s64 + -22120;
	// 825FD3E0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825FD3E4: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 825FD3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD3EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD3F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FD3F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD3F8: 386A107C  addi r3, r10, 0x107c
	ctx.r[3].s64 = ctx.r[10].s64 + 4220;
	// 825FD3FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FD400: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FD404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD414: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD41C: 4BE69A05  bl 0x82466e20
	ctx.lr = 0x825FD420;
	sub_82466E20(ctx, base);
	// 825FD420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD430 size=112
    let mut pc: u32 = 0x825FD430;
    'dispatch: loop {
        match pc {
            0x825FD430 => {
    //   block [0x825FD430..0x825FD4A0)
	// 825FD430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD43C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD440: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD444: 38AA0D4C  addi r5, r10, 0xd4c
	ctx.r[5].s64 = ctx.r[10].s64 + 3404;
	// 825FD448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD44C: 390B395C  addi r8, r11, 0x395c
	ctx.r[8].s64 = ctx.r[11].s64 + 14684;
	// 825FD450: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FD454: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 825FD458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD45C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD460: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FD464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD468: 386A10AC  addi r3, r10, 0x10ac
	ctx.r[3].s64 = ctx.r[10].s64 + 4268;
	// 825FD46C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FD470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD47C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD48C: 4BE69995  bl 0x82466e20
	ctx.lr = 0x825FD490;
	sub_82466E20(ctx, base);
	// 825FD490: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD4A0 size=108
    let mut pc: u32 = 0x825FD4A0;
    'dispatch: loop {
        match pc {
            0x825FD4A0 => {
    //   block [0x825FD4A0..0x825FD50C)
	// 825FD4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD4AC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD4B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD4B4: 38EB398C  addi r7, r11, 0x398c
	ctx.r[7].s64 = ctx.r[11].s64 + 14732;
	// 825FD4B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FD4BC: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 825FD4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD4C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD4C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD4D0: 386A10DC  addi r3, r10, 0x10dc
	ctx.r[3].s64 = ctx.r[10].s64 + 4316;
	// 825FD4D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD4D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD4EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD4F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD4F8: 4BE69929  bl 0x82466e20
	ctx.lr = 0x825FD4FC;
	sub_82466E20(ctx, base);
	// 825FD4FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD510 size=108
    let mut pc: u32 = 0x825FD510;
    'dispatch: loop {
        match pc {
            0x825FD510 => {
    //   block [0x825FD510..0x825FD57C)
	// 825FD510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD51C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD524: 38EB39C0  addi r7, r11, 0x39c0
	ctx.r[7].s64 = ctx.r[11].s64 + 14784;
	// 825FD528: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FD52C: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 825FD530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD534: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD540: 386A110C  addi r3, r10, 0x110c
	ctx.r[3].s64 = ctx.r[10].s64 + 4364;
	// 825FD544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD55C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD568: 4BE698B9  bl 0x82466e20
	ctx.lr = 0x825FD56C;
	sub_82466E20(ctx, base);
	// 825FD56C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD580 size=108
    let mut pc: u32 = 0x825FD580;
    'dispatch: loop {
        match pc {
            0x825FD580 => {
    //   block [0x825FD580..0x825FD5EC)
	// 825FD580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD58C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD594: 38EB3A20  addi r7, r11, 0x3a20
	ctx.r[7].s64 = ctx.r[11].s64 + 14880;
	// 825FD598: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FD59C: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 825FD5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD5A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD5A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD5B0: 386A113C  addi r3, r10, 0x113c
	ctx.r[3].s64 = ctx.r[10].s64 + 4412;
	// 825FD5B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD5B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD5D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD5D8: 4BE69849  bl 0x82466e20
	ctx.lr = 0x825FD5DC;
	sub_82466E20(ctx, base);
	// 825FD5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD5F0 size=108
    let mut pc: u32 = 0x825FD5F0;
    'dispatch: loop {
        match pc {
            0x825FD5F0 => {
    //   block [0x825FD5F0..0x825FD65C)
	// 825FD5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD5F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD5FC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD604: 38EB3A50  addi r7, r11, 0x3a50
	ctx.r[7].s64 = ctx.r[11].s64 + 14928;
	// 825FD608: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 825FD60C: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 825FD610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD614: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD620: 386A116C  addi r3, r10, 0x116c
	ctx.r[3].s64 = ctx.r[10].s64 + 4460;
	// 825FD624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD63C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD648: 4BE697D9  bl 0x82466e20
	ctx.lr = 0x825FD64C;
	sub_82466E20(ctx, base);
	// 825FD64C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD660 size=108
    let mut pc: u32 = 0x825FD660;
    'dispatch: loop {
        match pc {
            0x825FD660 => {
    //   block [0x825FD660..0x825FD6CC)
	// 825FD660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD66C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD674: 38EB3B70  addi r7, r11, 0x3b70
	ctx.r[7].s64 = ctx.r[11].s64 + 15216;
	// 825FD678: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FD67C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 825FD680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD684: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD688: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD690: 386A119C  addi r3, r10, 0x119c
	ctx.r[3].s64 = ctx.r[10].s64 + 4508;
	// 825FD694: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD6A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD6AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD6B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD6B8: 4BE69769  bl 0x82466e20
	ctx.lr = 0x825FD6BC;
	sub_82466E20(ctx, base);
	// 825FD6BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD6D0 size=108
    let mut pc: u32 = 0x825FD6D0;
    'dispatch: loop {
        match pc {
            0x825FD6D0 => {
    //   block [0x825FD6D0..0x825FD73C)
	// 825FD6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD6D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD6DC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD6E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD6E4: 38EB3B88  addi r7, r11, 0x3b88
	ctx.r[7].s64 = ctx.r[11].s64 + 15240;
	// 825FD6E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FD6EC: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 825FD6F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD6F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD6F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD6FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD700: 386A11CC  addi r3, r10, 0x11cc
	ctx.r[3].s64 = ctx.r[10].s64 + 4556;
	// 825FD704: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD70C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD71C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD728: 4BE696F9  bl 0x82466e20
	ctx.lr = 0x825FD72C;
	sub_82466E20(ctx, base);
	// 825FD72C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD740 size=108
    let mut pc: u32 = 0x825FD740;
    'dispatch: loop {
        match pc {
            0x825FD740 => {
    //   block [0x825FD740..0x825FD7AC)
	// 825FD740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD74C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD754: 38EB3BA0  addi r7, r11, 0x3ba0
	ctx.r[7].s64 = ctx.r[11].s64 + 15264;
	// 825FD758: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FD75C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 825FD760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD764: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD76C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD770: 386A11FC  addi r3, r10, 0x11fc
	ctx.r[3].s64 = ctx.r[10].s64 + 4604;
	// 825FD774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD78C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD798: 4BE69689  bl 0x82466e20
	ctx.lr = 0x825FD79C;
	sub_82466E20(ctx, base);
	// 825FD79C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD7A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD7A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD7B0 size=108
    let mut pc: u32 = 0x825FD7B0;
    'dispatch: loop {
        match pc {
            0x825FD7B0 => {
    //   block [0x825FD7B0..0x825FD81C)
	// 825FD7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD7B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD7BC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD7C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD7C4: 38EB3BB8  addi r7, r11, 0x3bb8
	ctx.r[7].s64 = ctx.r[11].s64 + 15288;
	// 825FD7C8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FD7CC: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 825FD7D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD7D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD7D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD7E0: 386A122C  addi r3, r10, 0x122c
	ctx.r[3].s64 = ctx.r[10].s64 + 4652;
	// 825FD7E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD7E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD7EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD7F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD7F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD7FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD808: 4BE69619  bl 0x82466e20
	ctx.lr = 0x825FD80C;
	sub_82466E20(ctx, base);
	// 825FD80C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD820 size=108
    let mut pc: u32 = 0x825FD820;
    'dispatch: loop {
        match pc {
            0x825FD820 => {
    //   block [0x825FD820..0x825FD88C)
	// 825FD820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD82C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD834: 38EB3BD0  addi r7, r11, 0x3bd0
	ctx.r[7].s64 = ctx.r[11].s64 + 15312;
	// 825FD838: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FD83C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 825FD840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD844: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD84C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD850: 386A125C  addi r3, r10, 0x125c
	ctx.r[3].s64 = ctx.r[10].s64 + 4700;
	// 825FD854: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD85C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD86C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD878: 4BE695A9  bl 0x82466e20
	ctx.lr = 0x825FD87C;
	sub_82466E20(ctx, base);
	// 825FD87C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD890 size=108
    let mut pc: u32 = 0x825FD890;
    'dispatch: loop {
        match pc {
            0x825FD890 => {
    //   block [0x825FD890..0x825FD8FC)
	// 825FD890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD89C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD8A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD8A4: 38EB3BE8  addi r7, r11, 0x3be8
	ctx.r[7].s64 = ctx.r[11].s64 + 15336;
	// 825FD8A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FD8AC: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 825FD8B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD8B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD8B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD8BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD8C0: 386A128C  addi r3, r10, 0x128c
	ctx.r[3].s64 = ctx.r[10].s64 + 4748;
	// 825FD8C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD8C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD8CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD8D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD8D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD8DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD8E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD8E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD8E8: 4BE69539  bl 0x82466e20
	ctx.lr = 0x825FD8EC;
	sub_82466E20(ctx, base);
	// 825FD8EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD8F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD8F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD8F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD900 size=108
    let mut pc: u32 = 0x825FD900;
    'dispatch: loop {
        match pc {
            0x825FD900 => {
    //   block [0x825FD900..0x825FD96C)
	// 825FD900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD90C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD914: 38EB3C00  addi r7, r11, 0x3c00
	ctx.r[7].s64 = ctx.r[11].s64 + 15360;
	// 825FD918: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825FD91C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 825FD920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD924: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD930: 386A12BC  addi r3, r10, 0x12bc
	ctx.r[3].s64 = ctx.r[10].s64 + 4796;
	// 825FD934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD94C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD958: 4BE694C9  bl 0x82466e20
	ctx.lr = 0x825FD95C;
	sub_82466E20(ctx, base);
	// 825FD95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD970 size=108
    let mut pc: u32 = 0x825FD970;
    'dispatch: loop {
        match pc {
            0x825FD970 => {
    //   block [0x825FD970..0x825FD9DC)
	// 825FD970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD97C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD984: 38EB3C90  addi r7, r11, 0x3c90
	ctx.r[7].s64 = ctx.r[11].s64 + 15504;
	// 825FD988: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825FD98C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 825FD990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FD994: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FD998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FD99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FD9A0: 386A12EC  addi r3, r10, 0x12ec
	ctx.r[3].s64 = ctx.r[10].s64 + 4844;
	// 825FD9A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FD9A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FD9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FD9B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FD9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FD9B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FD9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FD9C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FD9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FD9C8: 4BE69459  bl 0x82466e20
	ctx.lr = 0x825FD9CC;
	sub_82466E20(ctx, base);
	// 825FD9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FD9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FD9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FD9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FD9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FD9E0 size=108
    let mut pc: u32 = 0x825FD9E0;
    'dispatch: loop {
        match pc {
            0x825FD9E0 => {
    //   block [0x825FD9E0..0x825FDA4C)
	// 825FD9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FD9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FD9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FD9EC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FD9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FD9F4: 38EB3D50  addi r7, r11, 0x3d50
	ctx.r[7].s64 = ctx.r[11].s64 + 15696;
	// 825FD9F8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825FD9FC: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 825FDA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDA04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDA08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDA0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDA10: 386A131C  addi r3, r10, 0x131c
	ctx.r[3].s64 = ctx.r[10].s64 + 4892;
	// 825FDA14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDA20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDA28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDA30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDA34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDA38: 4BE693E9  bl 0x82466e20
	ctx.lr = 0x825FDA3C;
	sub_82466E20(ctx, base);
	// 825FDA3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDA40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDA44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDA48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDA50 size=108
    let mut pc: u32 = 0x825FDA50;
    'dispatch: loop {
        match pc {
            0x825FDA50 => {
    //   block [0x825FDA50..0x825FDABC)
	// 825FDA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDA58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDA5C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDA64: 38EB3E28  addi r7, r11, 0x3e28
	ctx.r[7].s64 = ctx.r[11].s64 + 15912;
	// 825FDA68: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825FDA6C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 825FDA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDA74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDA78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDA7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDA80: 386A134C  addi r3, r10, 0x134c
	ctx.r[3].s64 = ctx.r[10].s64 + 4940;
	// 825FDA84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDA88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDAA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDAA8: 4BE69379  bl 0x82466e20
	ctx.lr = 0x825FDAAC;
	sub_82466E20(ctx, base);
	// 825FDAAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDAC0 size=108
    let mut pc: u32 = 0x825FDAC0;
    'dispatch: loop {
        match pc {
            0x825FDAC0 => {
    //   block [0x825FDAC0..0x825FDB2C)
	// 825FDAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDAC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDACC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDAD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDAD4: 38EB3EE8  addi r7, r11, 0x3ee8
	ctx.r[7].s64 = ctx.r[11].s64 + 16104;
	// 825FDAD8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825FDADC: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 825FDAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDAE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDAE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDAF0: 386A137C  addi r3, r10, 0x137c
	ctx.r[3].s64 = ctx.r[10].s64 + 4988;
	// 825FDAF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDAF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDAFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDB00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDB04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDB08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDB10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDB14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDB18: 4BE69309  bl 0x82466e20
	ctx.lr = 0x825FDB1C;
	sub_82466E20(ctx, base);
	// 825FDB1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDB20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDB24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDB30 size=112
    let mut pc: u32 = 0x825FDB30;
    'dispatch: loop {
        match pc {
            0x825FDB30 => {
    //   block [0x825FDB30..0x825FDBA0)
	// 825FDB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDB38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDB3C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FDB40: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 825FDB44: 38EA3F90  addi r7, r10, 0x3f90
	ctx.r[7].s64 = ctx.r[10].s64 + 16272;
	// 825FDB48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDB4C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FDB50: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 825FDB54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDB58: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDB5C: 396B8DD8  addi r11, r11, -0x7228
	ctx.r[11].s64 = ctx.r[11].s64 + -29224;
	// 825FDB60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDB64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDB68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDB6C: 386A13AC  addi r3, r10, 0x13ac
	ctx.r[3].s64 = ctx.r[10].s64 + 5036;
	// 825FDB70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDB74: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FDB78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDB7C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FDB80: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDB84: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDB88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDB8C: 4BE69295  bl 0x82466e20
	ctx.lr = 0x825FDB90;
	sub_82466E20(ctx, base);
	// 825FDB90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDB94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDB98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDB9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDBA0 size=112
    let mut pc: u32 = 0x825FDBA0;
    'dispatch: loop {
        match pc {
            0x825FDBA0 => {
    //   block [0x825FDBA0..0x825FDC10)
	// 825FDBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDBA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDBA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDBAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDBB0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDBB4: 38AA0D4C  addi r5, r10, 0xd4c
	ctx.r[5].s64 = ctx.r[10].s64 + 3404;
	// 825FDBB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDBBC: 390B40C8  addi r8, r11, 0x40c8
	ctx.r[8].s64 = ctx.r[11].s64 + 16584;
	// 825FDBC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FDBC4: 388AA3A4  addi r4, r10, -0x5c5c
	ctx.r[4].s64 = ctx.r[10].s64 + -23644;
	// 825FDBC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDBCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDBD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FDBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDBD8: 386A13DC  addi r3, r10, 0x13dc
	ctx.r[3].s64 = ctx.r[10].s64 + 5084;
	// 825FDBDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FDBE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDBE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDBE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDBEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDBF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDBF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDBF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDBFC: 4BE69225  bl 0x82466e20
	ctx.lr = 0x825FDC00;
	sub_82466E20(ctx, base);
	// 825FDC00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDC04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDC08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDC0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDC10 size=108
    let mut pc: u32 = 0x825FDC10;
    'dispatch: loop {
        match pc {
            0x825FDC10 => {
    //   block [0x825FDC10..0x825FDC7C)
	// 825FDC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDC1C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDC24: 38EB40F8  addi r7, r11, 0x40f8
	ctx.r[7].s64 = ctx.r[11].s64 + 16632;
	// 825FDC28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FDC2C: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 825FDC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDC34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDC38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDC40: 386A140C  addi r3, r10, 0x140c
	ctx.r[3].s64 = ctx.r[10].s64 + 5132;
	// 825FDC44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDC68: 4BE691B9  bl 0x82466e20
	ctx.lr = 0x825FDC6C;
	sub_82466E20(ctx, base);
	// 825FDC6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDC70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDC74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDC78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDC80 size=108
    let mut pc: u32 = 0x825FDC80;
    'dispatch: loop {
        match pc {
            0x825FDC80 => {
    //   block [0x825FDC80..0x825FDCEC)
	// 825FDC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDC8C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDC90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDC94: 38EB4158  addi r7, r11, 0x4158
	ctx.r[7].s64 = ctx.r[11].s64 + 16728;
	// 825FDC98: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 825FDC9C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 825FDCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDCA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDCA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDCB0: 386A143C  addi r3, r10, 0x143c
	ctx.r[3].s64 = ctx.r[10].s64 + 5180;
	// 825FDCB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDCB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDCBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDCC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDCC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDCC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDCCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDCD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDCD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDCD8: 4BE69149  bl 0x82466e20
	ctx.lr = 0x825FDCDC;
	sub_82466E20(ctx, base);
	// 825FDCDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDCF0 size=108
    let mut pc: u32 = 0x825FDCF0;
    'dispatch: loop {
        match pc {
            0x825FDCF0 => {
    //   block [0x825FDCF0..0x825FDD5C)
	// 825FDCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDCFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDD04: 38EB4260  addi r7, r11, 0x4260
	ctx.r[7].s64 = ctx.r[11].s64 + 16992;
	// 825FDD08: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825FDD0C: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 825FDD10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDD14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDD18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDD1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDD20: 386A146C  addi r3, r10, 0x146c
	ctx.r[3].s64 = ctx.r[10].s64 + 5228;
	// 825FDD24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDD28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDD2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDD30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDD38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDD3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDD40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDD44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDD48: 4BE690D9  bl 0x82466e20
	ctx.lr = 0x825FDD4C;
	sub_82466E20(ctx, base);
	// 825FDD4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDD50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDD54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDD58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDD60 size=108
    let mut pc: u32 = 0x825FDD60;
    'dispatch: loop {
        match pc {
            0x825FDD60 => {
    //   block [0x825FDD60..0x825FDDCC)
	// 825FDD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDD6C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDD74: 38EB4338  addi r7, r11, 0x4338
	ctx.r[7].s64 = ctx.r[11].s64 + 17208;
	// 825FDD78: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FDD7C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 825FDD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDD84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDD88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDD90: 386A149C  addi r3, r10, 0x149c
	ctx.r[3].s64 = ctx.r[10].s64 + 5276;
	// 825FDD94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDDA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDDA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDDA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDDB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDDB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDDB8: 4BE69069  bl 0x82466e20
	ctx.lr = 0x825FDDBC;
	sub_82466E20(ctx, base);
	// 825FDDBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDDC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDDC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDDD0 size=108
    let mut pc: u32 = 0x825FDDD0;
    'dispatch: loop {
        match pc {
            0x825FDDD0 => {
    //   block [0x825FDDD0..0x825FDE3C)
	// 825FDDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDDDC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDDE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FDDE4: 38EB4380  addi r7, r11, 0x4380
	ctx.r[7].s64 = ctx.r[11].s64 + 17280;
	// 825FDDE8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FDDEC: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 825FDDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDDF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDDF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDE00: 386A14CC  addi r3, r10, 0x14cc
	ctx.r[3].s64 = ctx.r[10].s64 + 5324;
	// 825FDE04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDE0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDE24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDE28: 4BE68FF9  bl 0x82466e20
	ctx.lr = 0x825FDE2C;
	sub_82466E20(ctx, base);
	// 825FDE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDE40 size=108
    let mut pc: u32 = 0x825FDE40;
    'dispatch: loop {
        match pc {
            0x825FDE40 => {
    //   block [0x825FDE40..0x825FDEAC)
	// 825FDE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDE4C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDE50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FDE54: 38EB4398  addi r7, r11, 0x4398
	ctx.r[7].s64 = ctx.r[11].s64 + 17304;
	// 825FDE58: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FDE5C: 388AB3B4  addi r4, r10, -0x4c4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19532;
	// 825FDE60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDE64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDE68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDE70: 386A14FC  addi r3, r10, 0x14fc
	ctx.r[3].s64 = ctx.r[10].s64 + 5372;
	// 825FDE74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDE78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDE7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDE84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDE8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDE94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDE98: 4BE68F89  bl 0x82466e20
	ctx.lr = 0x825FDE9C;
	sub_82466E20(ctx, base);
	// 825FDE9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDEA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDEA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDEA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDEB0 size=108
    let mut pc: u32 = 0x825FDEB0;
    'dispatch: loop {
        match pc {
            0x825FDEB0 => {
    //   block [0x825FDEB0..0x825FDF1C)
	// 825FDEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDEBC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDEC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FDEC4: 38EB43F8  addi r7, r11, 0x43f8
	ctx.r[7].s64 = ctx.r[11].s64 + 17400;
	// 825FDEC8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825FDECC: 388AB3C0  addi r4, r10, -0x4c40
	ctx.r[4].s64 = ctx.r[10].s64 + -19520;
	// 825FDED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDED4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDED8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FDEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDEE0: 386A152C  addi r3, r10, 0x152c
	ctx.r[3].s64 = ctx.r[10].s64 + 5420;
	// 825FDEE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FDEE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDEEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDEF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDEF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDEF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDEFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDF00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDF04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FDF08: 4BE68F19  bl 0x82466e20
	ctx.lr = 0x825FDF0C;
	sub_82466E20(ctx, base);
	// 825FDF0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDF10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDF14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDF18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDF20 size=116
    let mut pc: u32 = 0x825FDF20;
    'dispatch: loop {
        match pc {
            0x825FDF20 => {
    //   block [0x825FDF20..0x825FDF94)
	// 825FDF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDF2C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDF30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FDF34: 390B44B8  addi r8, r11, 0x44b8
	ctx.r[8].s64 = ctx.r[11].s64 + 17592;
	// 825FDF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDF3C: 392A8E54  addi r9, r10, -0x71ac
	ctx.r[9].s64 = ctx.r[10].s64 + -29100;
	// 825FDF40: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDF44: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 825FDF48: 38AA14FC  addi r5, r10, 0x14fc
	ctx.r[5].s64 = ctx.r[10].s64 + 5372;
	// 825FDF4C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FDF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDF54: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FDF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDF64: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FDF68: 388AB3E0  addi r4, r10, -0x4c20
	ctx.r[4].s64 = ctx.r[10].s64 + -19488;
	// 825FDF6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FDF70: 386B155C  addi r3, r11, 0x155c
	ctx.r[3].s64 = ctx.r[11].s64 + 5468;
	// 825FDF74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FDF78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDF7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDF80: 4BE68EA1  bl 0x82466e20
	ctx.lr = 0x825FDF84;
	sub_82466E20(ctx, base);
	// 825FDF84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDF88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FDF8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FDF90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FDF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FDF98 size=112
    let mut pc: u32 = 0x825FDF98;
    'dispatch: loop {
        match pc {
            0x825FDF98 => {
    //   block [0x825FDF98..0x825FE008)
	// 825FDF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FDF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FDFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FDFA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDFA8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FDFAC: 38AA389C  addi r5, r10, 0x389c
	ctx.r[5].s64 = ctx.r[10].s64 + 14492;
	// 825FDFB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FDFB4: 390B4548  addi r8, r11, 0x4548
	ctx.r[8].s64 = ctx.r[11].s64 + 17736;
	// 825FDFB8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FDFBC: 388AB3F0  addi r4, r10, -0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + -19472;
	// 825FDFC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FDFC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FDFC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FDFCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FDFD0: 386A158C  addi r3, r10, 0x158c
	ctx.r[3].s64 = ctx.r[10].s64 + 5516;
	// 825FDFD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FDFD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FDFDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FDFE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FDFE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FDFE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FDFEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FDFF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FDFF4: 4BE68E2D  bl 0x82466e20
	ctx.lr = 0x825FDFF8;
	sub_82466E20(ctx, base);
	// 825FDFF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FDFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE008 size=96
    let mut pc: u32 = 0x825FE008;
    'dispatch: loop {
        match pc {
            0x825FE008 => {
    //   block [0x825FE008..0x825FE068)
	// 825FE008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE014: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE01C: 388AB40C  addi r4, r10, -0x4bf4
	ctx.r[4].s64 = ctx.r[10].s64 + -19444;
	// 825FE020: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE028: 386A15BC  addi r3, r10, 0x15bc
	ctx.r[3].s64 = ctx.r[10].s64 + 5564;
	// 825FE02C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE034: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FE038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE048: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE04C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FE050: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE054: 4BE68DCD  bl 0x82466e20
	ctx.lr = 0x825FE058;
	sub_82466E20(ctx, base);
	// 825FE058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE05C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FE068 size=24
    let mut pc: u32 = 0x825FE068;
    'dispatch: loop {
        match pc {
            0x825FE068 => {
    //   block [0x825FE068..0x825FE080)
	// 825FE068: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE06C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FE070: 394AAA10  addi r10, r10, -0x55f0
	ctx.r[10].s64 = ctx.r[10].s64 + -22000;
	// 825FE074: 816B45A8  lwz r11, 0x45a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17832 as u32) ) } as u64;
	// 825FE078: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825FE07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE080 size=116
    let mut pc: u32 = 0x825FE080;
    'dispatch: loop {
        match pc {
            0x825FE080 => {
    //   block [0x825FE080..0x825FE0F4)
	// 825FE080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE08C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FE090: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE094: 390BAA10  addi r8, r11, -0x55f0
	ctx.r[8].s64 = ctx.r[11].s64 + -22000;
	// 825FE098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE09C: 392A8EA0  addi r9, r10, -0x7160
	ctx.r[9].s64 = ctx.r[10].s64 + -29024;
	// 825FE0A0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE0A4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825FE0A8: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FE0AC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE0B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE0BC: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FE0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE0C4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FE0C8: 388AB42C  addi r4, r10, -0x4bd4
	ctx.r[4].s64 = ctx.r[10].s64 + -19412;
	// 825FE0CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FE0D0: 386B15EC  addi r3, r11, 0x15ec
	ctx.r[3].s64 = ctx.r[11].s64 + 5612;
	// 825FE0D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FE0D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE0DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE0E0: 4BE68D41  bl 0x82466e20
	ctx.lr = 0x825FE0E4;
	sub_82466E20(ctx, base);
	// 825FE0E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE0E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE0EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE0F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE0F8 size=104
    let mut pc: u32 = 0x825FE0F8;
    'dispatch: loop {
        match pc {
            0x825FE0F8 => {
    //   block [0x825FE0F8..0x825FE160)
	// 825FE0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE104: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE10C: 392A8ECC  addi r9, r10, -0x7134
	ctx.r[9].s64 = ctx.r[10].s64 + -28980;
	// 825FE110: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE118: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FE11C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE12C: 388AB440  addi r4, r10, -0x4bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -19392;
	// 825FE130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE134: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE138: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE13C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE140: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE144: 386A161C  addi r3, r10, 0x161c
	ctx.r[3].s64 = ctx.r[10].s64 + 5660;
	// 825FE148: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FE14C: 4BE68CD5  bl 0x82466e20
	ctx.lr = 0x825FE150;
	sub_82466E20(ctx, base);
	// 825FE150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE160 size=96
    let mut pc: u32 = 0x825FE160;
    'dispatch: loop {
        match pc {
            0x825FE160 => {
    //   block [0x825FE160..0x825FE1C0)
	// 825FE160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE16C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE174: 388AB454  addi r4, r10, -0x4bac
	ctx.r[4].s64 = ctx.r[10].s64 + -19372;
	// 825FE178: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE180: 386A164C  addi r3, r10, 0x164c
	ctx.r[3].s64 = ctx.r[10].s64 + 5708;
	// 825FE184: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE18C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FE190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE19C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE1A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE1A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FE1A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE1AC: 4BE68C75  bl 0x82466e20
	ctx.lr = 0x825FE1B0;
	sub_82466E20(ctx, base);
	// 825FE1B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE1B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE1B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE1BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE1C0 size=96
    let mut pc: u32 = 0x825FE1C0;
    'dispatch: loop {
        match pc {
            0x825FE1C0 => {
    //   block [0x825FE1C0..0x825FE220)
	// 825FE1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE1CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE1D4: 388AB46C  addi r4, r10, -0x4b94
	ctx.r[4].s64 = ctx.r[10].s64 + -19348;
	// 825FE1D8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE1E0: 386A167C  addi r3, r10, 0x167c
	ctx.r[3].s64 = ctx.r[10].s64 + 5756;
	// 825FE1E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE1EC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FE1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE200: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FE208: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE20C: 4BE68C15  bl 0x82466e20
	ctx.lr = 0x825FE210;
	sub_82466E20(ctx, base);
	// 825FE210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE220 size=100
    let mut pc: u32 = 0x825FE220;
    'dispatch: loop {
        match pc {
            0x825FE220 => {
    //   block [0x825FE220..0x825FE284)
	// 825FE220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE22C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE234: 38AA161C  addi r5, r10, 0x161c
	ctx.r[5].s64 = ctx.r[10].s64 + 5660;
	// 825FE238: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE23C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE240: 388AB488  addi r4, r10, -0x4b78
	ctx.r[4].s64 = ctx.r[10].s64 + -19320;
	// 825FE244: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE254: 386A16AC  addi r3, r10, 0x16ac
	ctx.r[3].s64 = ctx.r[10].s64 + 5804;
	// 825FE258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE25C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE260: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE268: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE270: 4BE68BB1  bl 0x82466e20
	ctx.lr = 0x825FE274;
	sub_82466E20(ctx, base);
	// 825FE274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE288 size=112
    let mut pc: u32 = 0x825FE288;
    'dispatch: loop {
        match pc {
            0x825FE288 => {
    //   block [0x825FE288..0x825FE2F8)
	// 825FE288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE294: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE298: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE29C: 38AA15EC  addi r5, r10, 0x15ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5612;
	// 825FE2A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE2A4: 390B45B0  addi r8, r11, 0x45b0
	ctx.r[8].s64 = ctx.r[11].s64 + 17840;
	// 825FE2A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FE2AC: 388AB4A4  addi r4, r10, -0x4b5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19292;
	// 825FE2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE2B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE2C0: 386A16DC  addi r3, r10, 0x16dc
	ctx.r[3].s64 = ctx.r[10].s64 + 5852;
	// 825FE2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE2E4: 4BE68B3D  bl 0x82466e20
	ctx.lr = 0x825FE2E8;
	sub_82466E20(ctx, base);
	// 825FE2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE2F8 size=112
    let mut pc: u32 = 0x825FE2F8;
    'dispatch: loop {
        match pc {
            0x825FE2F8 => {
    //   block [0x825FE2F8..0x825FE368)
	// 825FE2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE304: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE308: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE30C: 38AA15EC  addi r5, r10, 0x15ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5612;
	// 825FE310: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE314: 390B45F8  addi r8, r11, 0x45f8
	ctx.r[8].s64 = ctx.r[11].s64 + 17912;
	// 825FE318: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FE31C: 388AB4B4  addi r4, r10, -0x4b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19276;
	// 825FE320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE324: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE330: 386A170C  addi r3, r10, 0x170c
	ctx.r[3].s64 = ctx.r[10].s64 + 5900;
	// 825FE334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE354: 4BE68ACD  bl 0x82466e20
	ctx.lr = 0x825FE358;
	sub_82466E20(ctx, base);
	// 825FE358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE368 size=100
    let mut pc: u32 = 0x825FE368;
    'dispatch: loop {
        match pc {
            0x825FE368 => {
    //   block [0x825FE368..0x825FE3CC)
	// 825FE368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE374: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE37C: 38AA15EC  addi r5, r10, 0x15ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5612;
	// 825FE380: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE388: 388AB4CC  addi r4, r10, -0x4b34
	ctx.r[4].s64 = ctx.r[10].s64 + -19252;
	// 825FE38C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE39C: 386A173C  addi r3, r10, 0x173c
	ctx.r[3].s64 = ctx.r[10].s64 + 5948;
	// 825FE3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE3A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE3A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE3B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE3B8: 4BE68A69  bl 0x82466e20
	ctx.lr = 0x825FE3BC;
	sub_82466E20(ctx, base);
	// 825FE3BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE3C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE3C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE3D0 size=112
    let mut pc: u32 = 0x825FE3D0;
    'dispatch: loop {
        match pc {
            0x825FE3D0 => {
    //   block [0x825FE3D0..0x825FE440)
	// 825FE3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE3DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE3E0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE3E4: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FE3E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE3EC: 390B4610  addi r8, r11, 0x4610
	ctx.r[8].s64 = ctx.r[11].s64 + 17936;
	// 825FE3F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FE3F4: 388AB4E4  addi r4, r10, -0x4b1c
	ctx.r[4].s64 = ctx.r[10].s64 + -19228;
	// 825FE3F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE3FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE408: 386A176C  addi r3, r10, 0x176c
	ctx.r[3].s64 = ctx.r[10].s64 + 5996;
	// 825FE40C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE41C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE42C: 4BE689F5  bl 0x82466e20
	ctx.lr = 0x825FE430;
	sub_82466E20(ctx, base);
	// 825FE430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE440 size=96
    let mut pc: u32 = 0x825FE440;
    'dispatch: loop {
        match pc {
            0x825FE440 => {
    //   block [0x825FE440..0x825FE4A0)
	// 825FE440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE44C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE454: 388AB4F0  addi r4, r10, -0x4b10
	ctx.r[4].s64 = ctx.r[10].s64 + -19216;
	// 825FE458: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE460: 386A179C  addi r3, r10, 0x179c
	ctx.r[3].s64 = ctx.r[10].s64 + 6044;
	// 825FE464: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE46C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FE470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE47C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE480: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FE488: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE48C: 4BE68995  bl 0x82466e20
	ctx.lr = 0x825FE490;
	sub_82466E20(ctx, base);
	// 825FE490: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE4A0 size=112
    let mut pc: u32 = 0x825FE4A0;
    'dispatch: loop {
        match pc {
            0x825FE4A0 => {
    //   block [0x825FE4A0..0x825FE510)
	// 825FE4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE4AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE4B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE4B4: 38AA179C  addi r5, r10, 0x179c
	ctx.r[5].s64 = ctx.r[10].s64 + 6044;
	// 825FE4B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE4BC: 390B4640  addi r8, r11, 0x4640
	ctx.r[8].s64 = ctx.r[11].s64 + 17984;
	// 825FE4C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FE4C4: 388AB504  addi r4, r10, -0x4afc
	ctx.r[4].s64 = ctx.r[10].s64 + -19196;
	// 825FE4C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE4CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE4D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE4D8: 386A17CC  addi r3, r10, 0x17cc
	ctx.r[3].s64 = ctx.r[10].s64 + 6092;
	// 825FE4DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE4E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE4E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE4EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE4F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE4F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE4F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE4FC: 4BE68925  bl 0x82466e20
	ctx.lr = 0x825FE500;
	sub_82466E20(ctx, base);
	// 825FE500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE510 size=112
    let mut pc: u32 = 0x825FE510;
    'dispatch: loop {
        match pc {
            0x825FE510 => {
    //   block [0x825FE510..0x825FE580)
	// 825FE510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE51C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE520: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE524: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FE528: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE52C: 390B4658  addi r8, r11, 0x4658
	ctx.r[8].s64 = ctx.r[11].s64 + 18008;
	// 825FE530: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FE534: 388AB51C  addi r4, r10, -0x4ae4
	ctx.r[4].s64 = ctx.r[10].s64 + -19172;
	// 825FE538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE53C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE540: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE548: 386A17FC  addi r3, r10, 0x17fc
	ctx.r[3].s64 = ctx.r[10].s64 + 6140;
	// 825FE54C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE55C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FE560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE56C: 4BE688B5  bl 0x82466e20
	ctx.lr = 0x825FE570;
	sub_82466E20(ctx, base);
	// 825FE570: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE580 size=112
    let mut pc: u32 = 0x825FE580;
    'dispatch: loop {
        match pc {
            0x825FE580 => {
    //   block [0x825FE580..0x825FE5F0)
	// 825FE580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE58C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE590: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE594: 38AA17FC  addi r5, r10, 0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + 6140;
	// 825FE598: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE59C: 390B4670  addi r8, r11, 0x4670
	ctx.r[8].s64 = ctx.r[11].s64 + 18032;
	// 825FE5A0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FE5A4: 388AB530  addi r4, r10, -0x4ad0
	ctx.r[4].s64 = ctx.r[10].s64 + -19152;
	// 825FE5A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE5AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE5B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE5B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE5B8: 386A182C  addi r3, r10, 0x182c
	ctx.r[3].s64 = ctx.r[10].s64 + 6188;
	// 825FE5BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE5C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE5C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE5C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE5CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE5D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE5D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE5D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE5DC: 4BE68845  bl 0x82466e20
	ctx.lr = 0x825FE5E0;
	sub_82466E20(ctx, base);
	// 825FE5E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FE5F0 size=36
    let mut pc: u32 = 0x825FE5F0;
    'dispatch: loop {
        match pc {
            0x825FE5F0 => {
    //   block [0x825FE5F0..0x825FE614)
	// 825FE5F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE5F4: 814B46C0  lwz r10, 0x46c0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18112 as u32) ) } as u64;
	// 825FE5F8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FE5FC: 396BAA40  addi r11, r11, -0x55c0
	ctx.r[11].s64 = ctx.r[11].s64 + -21952;
	// 825FE600: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825FE604: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FE608: 814A46B8  lwz r10, 0x46b8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(18104 as u32) ) } as u64;
	// 825FE60C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 825FE610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE618 size=108
    let mut pc: u32 = 0x825FE618;
    'dispatch: loop {
        match pc {
            0x825FE618 => {
    //   block [0x825FE618..0x825FE684)
	// 825FE618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE624: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FE628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE62C: 38EBAA40  addi r7, r11, -0x55c0
	ctx.r[7].s64 = ctx.r[11].s64 + -21952;
	// 825FE630: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825FE634: 388AB548  addi r4, r10, -0x4ab8
	ctx.r[4].s64 = ctx.r[10].s64 + -19128;
	// 825FE638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE63C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FE644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE648: 386A185C  addi r3, r10, 0x185c
	ctx.r[3].s64 = ctx.r[10].s64 + 6236;
	// 825FE64C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FE650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FE670: 4BE687B1  bl 0x82466e20
	ctx.lr = 0x825FE674;
	sub_82466E20(ctx, base);
	// 825FE674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FE688 size=24
    let mut pc: u32 = 0x825FE688;
    'dispatch: loop {
        match pc {
            0x825FE688 => {
    //   block [0x825FE688..0x825FE6A0)
	// 825FE688: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE68C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FE690: 394AAAE8  addi r10, r10, -0x5518
	ctx.r[10].s64 = ctx.r[10].s64 + -21784;
	// 825FE694: 816B46B8  lwz r11, 0x46b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18104 as u32) ) } as u64;
	// 825FE698: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 825FE69C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE6A0 size=116
    let mut pc: u32 = 0x825FE6A0;
    'dispatch: loop {
        match pc {
            0x825FE6A0 => {
    //   block [0x825FE6A0..0x825FE714)
	// 825FE6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE6AC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FE6B0: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 825FE6B4: 390AAAE8  addi r8, r10, -0x5518
	ctx.r[8].s64 = ctx.r[10].s64 + -21784;
	// 825FE6B8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE6BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FE6C0: 38AA185C  addi r5, r10, 0x185c
	ctx.r[5].s64 = ctx.r[10].s64 + 6236;
	// 825FE6C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE6C8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FE6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE6D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE6D4: 388AB57C  addi r4, r10, -0x4a84
	ctx.r[4].s64 = ctx.r[10].s64 + -19076;
	// 825FE6D8: 396B8F6C  addi r11, r11, -0x7094
	ctx.r[11].s64 = ctx.r[11].s64 + -28820;
	// 825FE6DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE6E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE6E4: 386A188C  addi r3, r10, 0x188c
	ctx.r[3].s64 = ctx.r[10].s64 + 6284;
	// 825FE6E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FE6EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE6F0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FE6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE6F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE6FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE700: 4BE68721  bl 0x82466e20
	ctx.lr = 0x825FE704;
	sub_82466E20(ctx, base);
	// 825FE704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE70C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE718 size=112
    let mut pc: u32 = 0x825FE718;
    'dispatch: loop {
        match pc {
            0x825FE718 => {
    //   block [0x825FE718..0x825FE788)
	// 825FE718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE724: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE728: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE72C: 38AA185C  addi r5, r10, 0x185c
	ctx.r[5].s64 = ctx.r[10].s64 + 6236;
	// 825FE730: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE734: 390B46C8  addi r8, r11, 0x46c8
	ctx.r[8].s64 = ctx.r[11].s64 + 18120;
	// 825FE738: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825FE73C: 388AB5A4  addi r4, r10, -0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19036;
	// 825FE740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE750: 386A18BC  addi r3, r10, 0x18bc
	ctx.r[3].s64 = ctx.r[10].s64 + 6332;
	// 825FE754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE774: 4BE686AD  bl 0x82466e20
	ctx.lr = 0x825FE778;
	sub_82466E20(ctx, base);
	// 825FE778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FE788 size=24
    let mut pc: u32 = 0x825FE788;
    'dispatch: loop {
        match pc {
            0x825FE788 => {
    //   block [0x825FE788..0x825FE7A0)
	// 825FE788: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE78C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FE790: 394AABD8  addi r10, r10, -0x5428
	ctx.r[10].s64 = ctx.r[10].s64 + -21544;
	// 825FE794: 816B51E8  lwz r11, 0x51e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20968 as u32) ) } as u64;
	// 825FE798: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 825FE79C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE7A0 size=116
    let mut pc: u32 = 0x825FE7A0;
    'dispatch: loop {
        match pc {
            0x825FE7A0 => {
    //   block [0x825FE7A0..0x825FE814)
	// 825FE7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE7AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FE7B0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE7B4: 392B8F30  addi r9, r11, -0x70d0
	ctx.r[9].s64 = ctx.r[11].s64 + -28880;
	// 825FE7B8: 38AA17FC  addi r5, r10, 0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + 6140;
	// 825FE7BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE7C0: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 825FE7C4: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 825FE7C8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FE7CC: 388AB5E4  addi r4, r10, -0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + -18972;
	// 825FE7D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE7D4: 396BABD8  addi r11, r11, -0x5428
	ctx.r[11].s64 = ctx.r[11].s64 + -21544;
	// 825FE7D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825FE7DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE7E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825FE7E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE7E8: 386A18EC  addi r3, r10, 0x18ec
	ctx.r[3].s64 = ctx.r[10].s64 + 6380;
	// 825FE7EC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825FE7F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825FE7F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE7F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825FE7FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE800: 4BE68621  bl 0x82466e20
	ctx.lr = 0x825FE804;
	sub_82466E20(ctx, base);
	// 825FE804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE818 size=100
    let mut pc: u32 = 0x825FE818;
    'dispatch: loop {
        match pc {
            0x825FE818 => {
    //   block [0x825FE818..0x825FE87C)
	// 825FE818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE824: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE82C: 38AA1A0C  addi r5, r10, 0x1a0c
	ctx.r[5].s64 = ctx.r[10].s64 + 6668;
	// 825FE830: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE838: 388AB5FC  addi r4, r10, -0x4a04
	ctx.r[4].s64 = ctx.r[10].s64 + -18948;
	// 825FE83C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE84C: 386A191C  addi r3, r10, 0x191c
	ctx.r[3].s64 = ctx.r[10].s64 + 6428;
	// 825FE850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE854: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE858: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FE85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE860: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FE864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE868: 4BE685B9  bl 0x82466e20
	ctx.lr = 0x825FE86C;
	sub_82466E20(ctx, base);
	// 825FE86C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE880 size=108
    let mut pc: u32 = 0x825FE880;
    'dispatch: loop {
        match pc {
            0x825FE880 => {
    //   block [0x825FE880..0x825FE8EC)
	// 825FE880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE88C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE890: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE894: 38EB4740  addi r7, r11, 0x4740
	ctx.r[7].s64 = ctx.r[11].s64 + 18240;
	// 825FE898: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FE89C: 388AB610  addi r4, r10, -0x49f0
	ctx.r[4].s64 = ctx.r[10].s64 + -18928;
	// 825FE8A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE8A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE8A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FE8AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE8B0: 386A194C  addi r3, r10, 0x194c
	ctx.r[3].s64 = ctx.r[10].s64 + 6476;
	// 825FE8B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FE8B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE8BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE8C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE8C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE8C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE8CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE8D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE8D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FE8D8: 4BE68549  bl 0x82466e20
	ctx.lr = 0x825FE8DC;
	sub_82466E20(ctx, base);
	// 825FE8DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE8E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE8E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE8F0 size=112
    let mut pc: u32 = 0x825FE8F0;
    'dispatch: loop {
        match pc {
            0x825FE8F0 => {
    //   block [0x825FE8F0..0x825FE960)
	// 825FE8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE8FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE900: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE904: 38AA17FC  addi r5, r10, 0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + 6140;
	// 825FE908: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE90C: 390B47A0  addi r8, r11, 0x47a0
	ctx.r[8].s64 = ctx.r[11].s64 + 18336;
	// 825FE910: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FE914: 388AB628  addi r4, r10, -0x49d8
	ctx.r[4].s64 = ctx.r[10].s64 + -18904;
	// 825FE918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE91C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FE924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE928: 386A197C  addi r3, r10, 0x197c
	ctx.r[3].s64 = ctx.r[10].s64 + 6524;
	// 825FE92C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FE930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE94C: 4BE684D5  bl 0x82466e20
	ctx.lr = 0x825FE950;
	sub_82466E20(ctx, base);
	// 825FE950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE960 size=108
    let mut pc: u32 = 0x825FE960;
    'dispatch: loop {
        match pc {
            0x825FE960 => {
    //   block [0x825FE960..0x825FE9CC)
	// 825FE960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE96C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE970: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FE974: 38EB4800  addi r7, r11, 0x4800
	ctx.r[7].s64 = ctx.r[11].s64 + 18432;
	// 825FE978: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FE97C: 388AB638  addi r4, r10, -0x49c8
	ctx.r[4].s64 = ctx.r[10].s64 + -18888;
	// 825FE980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FE984: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FE988: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FE98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FE990: 386A19AC  addi r3, r10, 0x19ac
	ctx.r[3].s64 = ctx.r[10].s64 + 6572;
	// 825FE994: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FE998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FE99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FE9A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FE9A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FE9A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FE9AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FE9B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FE9B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FE9B8: 4BE68469  bl 0x82466e20
	ctx.lr = 0x825FE9BC;
	sub_82466E20(ctx, base);
	// 825FE9BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FE9C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FE9C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FE9C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FE9D0 size=28
    let mut pc: u32 = 0x825FE9D0;
    'dispatch: loop {
        match pc {
            0x825FE9D0 => {
    //   block [0x825FE9D0..0x825FE9EC)
	// 825FE9D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FE9D4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FE9D8: 394AACE0  addi r10, r10, -0x5320
	ctx.r[10].s64 = ctx.r[10].s64 + -21280;
	// 825FE9DC: 816B46C4  lwz r11, 0x46c4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18116 as u32) ) } as u64;
	// 825FE9E0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825FE9E4: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 825FE9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FE9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FE9F0 size=112
    let mut pc: u32 = 0x825FE9F0;
    'dispatch: loop {
        match pc {
            0x825FE9F0 => {
    //   block [0x825FE9F0..0x825FEA60)
	// 825FE9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FE9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FE9F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FE9FC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FEA00: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 825FEA04: 38EAACE0  addi r7, r10, -0x5320
	ctx.r[7].s64 = ctx.r[10].s64 + -21280;
	// 825FEA08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEA0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FEA10: 388AB648  addi r4, r10, -0x49b8
	ctx.r[4].s64 = ctx.r[10].s64 + -18872;
	// 825FEA14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEA18: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FEA1C: 396B9030  addi r11, r11, -0x6fd0
	ctx.r[11].s64 = ctx.r[11].s64 + -28624;
	// 825FEA20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FEA24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEA28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEA2C: 386A19DC  addi r3, r10, 0x19dc
	ctx.r[3].s64 = ctx.r[10].s64 + 6620;
	// 825FEA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEA34: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FEA38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEA3C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FEA40: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEA44: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEA48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FEA4C: 4BE683D5  bl 0x82466e20
	ctx.lr = 0x825FEA50;
	sub_82466E20(ctx, base);
	// 825FEA50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FEA60 size=24
    let mut pc: u32 = 0x825FEA60;
    'dispatch: loop {
        match pc {
            0x825FEA60 => {
    //   block [0x825FEA60..0x825FEA78)
	// 825FEA60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEA64: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FEA68: 394AAE48  addi r10, r10, -0x51b8
	ctx.r[10].s64 = ctx.r[10].s64 + -20920;
	// 825FEA6C: 816B51E8  lwz r11, 0x51e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20968 as u32) ) } as u64;
	// 825FEA70: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825FEA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEA78 size=116
    let mut pc: u32 = 0x825FEA78;
    'dispatch: loop {
        match pc {
            0x825FEA78 => {
    //   block [0x825FEA78..0x825FEAEC)
	// 825FEA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEA84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FEA88: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEA8C: 392B9008  addi r9, r11, -0x6ff8
	ctx.r[9].s64 = ctx.r[11].s64 + -28664;
	// 825FEA90: 38AA17FC  addi r5, r10, 0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + 6140;
	// 825FEA94: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEA98: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 825FEA9C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 825FEAA0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FEAA4: 388AB65C  addi r4, r10, -0x49a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18852;
	// 825FEAA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEAAC: 396BAE48  addi r11, r11, -0x51b8
	ctx.r[11].s64 = ctx.r[11].s64 + -20920;
	// 825FEAB0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825FEAB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEAB8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825FEABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEAC0: 386A1A0C  addi r3, r10, 0x1a0c
	ctx.r[3].s64 = ctx.r[10].s64 + 6668;
	// 825FEAC4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FEAC8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825FEACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEAD0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825FEAD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FEAD8: 4BE68349  bl 0x82466e20
	ctx.lr = 0x825FEADC;
	sub_82466E20(ctx, base);
	// 825FEADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEAF0 size=108
    let mut pc: u32 = 0x825FEAF0;
    'dispatch: loop {
        match pc {
            0x825FEAF0 => {
    //   block [0x825FEAF0..0x825FEB5C)
	// 825FEAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEAFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEB00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEB04: 38EB4820  addi r7, r11, 0x4820
	ctx.r[7].s64 = ctx.r[11].s64 + 18464;
	// 825FEB08: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FEB0C: 388AB66C  addi r4, r10, -0x4994
	ctx.r[4].s64 = ctx.r[10].s64 + -18836;
	// 825FEB10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FEB14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEB18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FEB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEB20: 386A1A3C  addi r3, r10, 0x1a3c
	ctx.r[3].s64 = ctx.r[10].s64 + 6716;
	// 825FEB24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FEB28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEB30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FEB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEB38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FEB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEB40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEB44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FEB48: 4BE682D9  bl 0x82466e20
	ctx.lr = 0x825FEB4C;
	sub_82466E20(ctx, base);
	// 825FEB4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEB50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEB54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FEB60 size=24
    let mut pc: u32 = 0x825FEB60;
    'dispatch: loop {
        match pc {
            0x825FEB60 => {
    //   block [0x825FEB60..0x825FEB78)
	// 825FEB60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEB64: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FEB68: 394AAEF0  addi r10, r10, -0x5110
	ctx.r[10].s64 = ctx.r[10].s64 + -20752;
	// 825FEB6C: 816B51E8  lwz r11, 0x51e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20968 as u32) ) } as u64;
	// 825FEB70: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825FEB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEB78 size=116
    let mut pc: u32 = 0x825FEB78;
    'dispatch: loop {
        match pc {
            0x825FEB78 => {
    //   block [0x825FEB78..0x825FEBEC)
	// 825FEB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEB84: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FEB88: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825FEB8C: 390AAEF0  addi r8, r10, -0x5110
	ctx.r[8].s64 = ctx.r[10].s64 + -20752;
	// 825FEB90: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEB94: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FEB98: 38AA17FC  addi r5, r10, 0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + 6140;
	// 825FEB9C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEBA0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FEBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEBA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FEBAC: 388AB688  addi r4, r10, -0x4978
	ctx.r[4].s64 = ctx.r[10].s64 + -18808;
	// 825FEBB0: 396B9090  addi r11, r11, -0x6f70
	ctx.r[11].s64 = ctx.r[11].s64 + -28528;
	// 825FEBB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEBB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEBBC: 386A1A6C  addi r3, r10, 0x1a6c
	ctx.r[3].s64 = ctx.r[10].s64 + 6764;
	// 825FEBC0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FEBC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEBC8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FEBCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEBD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEBD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEBD8: 4BE68249  bl 0x82466e20
	ctx.lr = 0x825FEBDC;
	sub_82466E20(ctx, base);
	// 825FEBDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEBF0 size=112
    let mut pc: u32 = 0x825FEBF0;
    'dispatch: loop {
        match pc {
            0x825FEBF0 => {
    //   block [0x825FEBF0..0x825FEC60)
	// 825FEBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEBFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEC00: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEC04: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FEC08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEC0C: 390B4880  addi r8, r11, 0x4880
	ctx.r[8].s64 = ctx.r[11].s64 + 18560;
	// 825FEC10: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825FEC14: 388AB69C  addi r4, r10, -0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + -18788;
	// 825FEC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FEC1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FEC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEC28: 386A1A9C  addi r3, r10, 0x1a9c
	ctx.r[3].s64 = ctx.r[10].s64 + 6812;
	// 825FEC2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FEC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FEC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FEC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEC4C: 4BE681D5  bl 0x82466e20
	ctx.lr = 0x825FEC50;
	sub_82466E20(ctx, base);
	// 825FEC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEC60 size=112
    let mut pc: u32 = 0x825FEC60;
    'dispatch: loop {
        match pc {
            0x825FEC60 => {
    //   block [0x825FEC60..0x825FECD0)
	// 825FEC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEC6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEC70: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEC74: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FEC78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEC7C: 390B4910  addi r8, r11, 0x4910
	ctx.r[8].s64 = ctx.r[11].s64 + 18704;
	// 825FEC80: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FEC84: 388AB6CC  addi r4, r10, -0x4934
	ctx.r[4].s64 = ctx.r[10].s64 + -18740;
	// 825FEC88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FEC8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEC90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FEC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEC98: 386A1ACC  addi r3, r10, 0x1acc
	ctx.r[3].s64 = ctx.r[10].s64 + 6860;
	// 825FEC9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FECA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FECA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FECA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FECAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FECB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FECB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FECB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FECBC: 4BE68165  bl 0x82466e20
	ctx.lr = 0x825FECC0;
	sub_82466E20(ctx, base);
	// 825FECC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FECC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FECC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FECCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FECD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FECD0 size=112
    let mut pc: u32 = 0x825FECD0;
    'dispatch: loop {
        match pc {
            0x825FECD0 => {
    //   block [0x825FECD0..0x825FED40)
	// 825FECD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FECD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FECD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FECDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FECE0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FECE4: 38AA18EC  addi r5, r10, 0x18ec
	ctx.r[5].s64 = ctx.r[10].s64 + 6380;
	// 825FECE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FECEC: 390B4970  addi r8, r11, 0x4970
	ctx.r[8].s64 = ctx.r[11].s64 + 18800;
	// 825FECF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FECF4: 388AB6FC  addi r4, r10, -0x4904
	ctx.r[4].s64 = ctx.r[10].s64 + -18692;
	// 825FECF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FECFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FED00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FED04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FED08: 386A1AFC  addi r3, r10, 0x1afc
	ctx.r[3].s64 = ctx.r[10].s64 + 6908;
	// 825FED0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FED10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FED14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FED18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FED1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FED20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FED24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FED28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FED2C: 4BE680F5  bl 0x82466e20
	ctx.lr = 0x825FED30;
	sub_82466E20(ctx, base);
	// 825FED30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FED34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FED38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FED3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FED40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FED40 size=112
    let mut pc: u32 = 0x825FED40;
    'dispatch: loop {
        match pc {
            0x825FED40 => {
    //   block [0x825FED40..0x825FEDB0)
	// 825FED40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FED44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FED48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FED4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FED50: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FED54: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FED58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FED5C: 390B49A0  addi r8, r11, 0x49a0
	ctx.r[8].s64 = ctx.r[11].s64 + 18848;
	// 825FED60: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825FED64: 388AB718  addi r4, r10, -0x48e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18664;
	// 825FED68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FED6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FED70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FED74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FED78: 386A1B2C  addi r3, r10, 0x1b2c
	ctx.r[3].s64 = ctx.r[10].s64 + 6956;
	// 825FED7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FED80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FED84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FED88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FED8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FED90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FED94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FED98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FED9C: 4BE68085  bl 0x82466e20
	ctx.lr = 0x825FEDA0;
	sub_82466E20(ctx, base);
	// 825FEDA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEDA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEDA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEDAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEDB0 size=112
    let mut pc: u32 = 0x825FEDB0;
    'dispatch: loop {
        match pc {
            0x825FEDB0 => {
    //   block [0x825FEDB0..0x825FEE20)
	// 825FEDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEDB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEDBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEDC0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEDC4: 38AA1A0C  addi r5, r10, 0x1a0c
	ctx.r[5].s64 = ctx.r[10].s64 + 6668;
	// 825FEDC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEDCC: 390B4A30  addi r8, r11, 0x4a30
	ctx.r[8].s64 = ctx.r[11].s64 + 18992;
	// 825FEDD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FEDD4: 388AB73C  addi r4, r10, -0x48c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18628;
	// 825FEDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FEDDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEDE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FEDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEDE8: 386A1B5C  addi r3, r10, 0x1b5c
	ctx.r[3].s64 = ctx.r[10].s64 + 7004;
	// 825FEDEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FEDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEDF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FEDFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FEE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEE0C: 4BE68015  bl 0x82466e20
	ctx.lr = 0x825FEE10;
	sub_82466E20(ctx, base);
	// 825FEE10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEE20 size=100
    let mut pc: u32 = 0x825FEE20;
    'dispatch: loop {
        match pc {
            0x825FEE20 => {
    //   block [0x825FEE20..0x825FEE84)
	// 825FEE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEE2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FEE34: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FEE38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEE40: 388AB750  addi r4, r10, -0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + -18608;
	// 825FEE44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FEE4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FEE54: 386A1B8C  addi r3, r10, 0x1b8c
	ctx.r[3].s64 = ctx.r[10].s64 + 7052;
	// 825FEE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEE5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEE60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FEE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEE68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FEE6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEE70: 4BE67FB1  bl 0x82466e20
	ctx.lr = 0x825FEE74;
	sub_82466E20(ctx, base);
	// 825FEE74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEE88 size=112
    let mut pc: u32 = 0x825FEE88;
    'dispatch: loop {
        match pc {
            0x825FEE88 => {
    //   block [0x825FEE88..0x825FEEF8)
	// 825FEE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEE94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEE98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEE9C: 38AA1B8C  addi r5, r10, 0x1b8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7052;
	// 825FEEA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEEA4: 390B4A48  addi r8, r11, 0x4a48
	ctx.r[8].s64 = ctx.r[11].s64 + 19016;
	// 825FEEA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FEEAC: 388AB760  addi r4, r10, -0x48a0
	ctx.r[4].s64 = ctx.r[10].s64 + -18592;
	// 825FEEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FEEB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEEB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FEEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEEC0: 386A1BBC  addi r3, r10, 0x1bbc
	ctx.r[3].s64 = ctx.r[10].s64 + 7100;
	// 825FEEC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FEEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FEED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FEEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEEE4: 4BE67F3D  bl 0x82466e20
	ctx.lr = 0x825FEEE8;
	sub_82466E20(ctx, base);
	// 825FEEE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEEF8 size=112
    let mut pc: u32 = 0x825FEEF8;
    'dispatch: loop {
        match pc {
            0x825FEEF8 => {
    //   block [0x825FEEF8..0x825FEF68)
	// 825FEEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEF04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEF08: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEF0C: 38AA1BBC  addi r5, r10, 0x1bbc
	ctx.r[5].s64 = ctx.r[10].s64 + 7100;
	// 825FEF10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEF14: 390B4AA8  addi r8, r11, 0x4aa8
	ctx.r[8].s64 = ctx.r[11].s64 + 19112;
	// 825FEF18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FEF1C: 388AB778  addi r4, r10, -0x4888
	ctx.r[4].s64 = ctx.r[10].s64 + -18568;
	// 825FEF20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FEF24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEF28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FEF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEF30: 386A1BEC  addi r3, r10, 0x1bec
	ctx.r[3].s64 = ctx.r[10].s64 + 7148;
	// 825FEF34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FEF38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEF40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FEF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEF48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FEF4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEF50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEF54: 4BE67ECD  bl 0x82466e20
	ctx.lr = 0x825FEF58;
	sub_82466E20(ctx, base);
	// 825FEF58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FEF68 size=24
    let mut pc: u32 = 0x825FEF68;
    'dispatch: loop {
        match pc {
            0x825FEF68 => {
    //   block [0x825FEF68..0x825FEF80)
	// 825FEF68: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEF6C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FEF70: 394AAF68  addi r10, r10, -0x5098
	ctx.r[10].s64 = ctx.r[10].s64 + -20632;
	// 825FEF74: 816B51E8  lwz r11, 0x51e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20968 as u32) ) } as u64;
	// 825FEF78: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825FEF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEF80 size=116
    let mut pc: u32 = 0x825FEF80;
    'dispatch: loop {
        match pc {
            0x825FEF80 => {
    //   block [0x825FEF80..0x825FEFF4)
	// 825FEF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEF8C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FEF90: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825FEF94: 390AAF68  addi r8, r10, -0x5098
	ctx.r[8].s64 = ctx.r[10].s64 + -20632;
	// 825FEF98: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEF9C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FEFA0: 38AA1BBC  addi r5, r10, 0x1bbc
	ctx.r[5].s64 = ctx.r[10].s64 + 7100;
	// 825FEFA4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEFA8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FEFAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FEFB4: 388AB78C  addi r4, r10, -0x4874
	ctx.r[4].s64 = ctx.r[10].s64 + -18548;
	// 825FEFB8: 396B90A8  addi r11, r11, -0x6f58
	ctx.r[11].s64 = ctx.r[11].s64 + -28504;
	// 825FEFBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEFC0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FEFC4: 386A1C1C  addi r3, r10, 0x1c1c
	ctx.r[3].s64 = ctx.r[10].s64 + 7196;
	// 825FEFC8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FEFCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEFD0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FEFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEFE0: 4BE67E41  bl 0x82466e20
	ctx.lr = 0x825FEFE4;
	sub_82466E20(ctx, base);
	// 825FEFE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEFF8 size=112
    let mut pc: u32 = 0x825FEFF8;
    'dispatch: loop {
        match pc {
            0x825FEFF8 => {
    //   block [0x825FEFF8..0x825FF068)
	// 825FEFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF004: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF008: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF00C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FF010: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF014: 390B4AD8  addi r8, r11, 0x4ad8
	ctx.r[8].s64 = ctx.r[11].s64 + 19160;
	// 825FF018: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FF01C: 388AB7A4  addi r4, r10, -0x485c
	ctx.r[4].s64 = ctx.r[10].s64 + -18524;
	// 825FF020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF024: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF02C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF030: 386A1C4C  addi r3, r10, 0x1c4c
	ctx.r[3].s64 = ctx.r[10].s64 + 7244;
	// 825FF034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF044: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FF048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF04C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF054: 4BE67DCD  bl 0x82466e20
	ctx.lr = 0x825FF058;
	sub_82466E20(ctx, base);
	// 825FF058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF05C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF068 size=116
    let mut pc: u32 = 0x825FF068;
    'dispatch: loop {
        match pc {
            0x825FF068 => {
    //   block [0x825FF068..0x825FF0DC)
	// 825FF068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF074: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF078: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF07C: 390B4B08  addi r8, r11, 0x4b08
	ctx.r[8].s64 = ctx.r[11].s64 + 19208;
	// 825FF080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF084: 392A90E8  addi r9, r10, -0x6f18
	ctx.r[9].s64 = ctx.r[10].s64 + -28440;
	// 825FF088: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF08C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825FF090: 38AA1F1C  addi r5, r10, 0x1f1c
	ctx.r[5].s64 = ctx.r[10].s64 + 7964;
	// 825FF094: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF09C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF0A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF0A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF0AC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FF0B0: 388AB7C4  addi r4, r10, -0x483c
	ctx.r[4].s64 = ctx.r[10].s64 + -18492;
	// 825FF0B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FF0B8: 386B1C7C  addi r3, r11, 0x1c7c
	ctx.r[3].s64 = ctx.r[11].s64 + 7292;
	// 825FF0BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FF0C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF0C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF0C8: 4BE67D59  bl 0x82466e20
	ctx.lr = 0x825FF0CC;
	sub_82466E20(ctx, base);
	// 825FF0CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF0E0 size=112
    let mut pc: u32 = 0x825FF0E0;
    'dispatch: loop {
        match pc {
            0x825FF0E0 => {
    //   block [0x825FF0E0..0x825FF150)
	// 825FF0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF0EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF0F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF0F4: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF0F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF0FC: 390B4B20  addi r8, r11, 0x4b20
	ctx.r[8].s64 = ctx.r[11].s64 + 19232;
	// 825FF100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FF104: 388AB7D4  addi r4, r10, -0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + -18476;
	// 825FF108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF10C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF118: 386A1CAC  addi r3, r10, 0x1cac
	ctx.r[3].s64 = ctx.r[10].s64 + 7340;
	// 825FF11C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF12C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF13C: 4BE67CE5  bl 0x82466e20
	ctx.lr = 0x825FF140;
	sub_82466E20(ctx, base);
	// 825FF140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF150 size=116
    let mut pc: u32 = 0x825FF150;
    'dispatch: loop {
        match pc {
            0x825FF150 => {
    //   block [0x825FF150..0x825FF1C4)
	// 825FF150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF15C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF160: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF164: 390B4B3C  addi r8, r11, 0x4b3c
	ctx.r[8].s64 = ctx.r[11].s64 + 19260;
	// 825FF168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF16C: 392A9114  addi r9, r10, -0x6eec
	ctx.r[9].s64 = ctx.r[10].s64 + -28396;
	// 825FF170: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF174: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825FF178: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF17C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF184: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF18C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF194: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FF198: 388AB7E0  addi r4, r10, -0x4820
	ctx.r[4].s64 = ctx.r[10].s64 + -18464;
	// 825FF19C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FF1A0: 386B1CDC  addi r3, r11, 0x1cdc
	ctx.r[3].s64 = ctx.r[11].s64 + 7388;
	// 825FF1A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FF1A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF1AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF1B0: 4BE67C71  bl 0x82466e20
	ctx.lr = 0x825FF1B4;
	sub_82466E20(ctx, base);
	// 825FF1B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF1B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF1BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF1C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF1C8 size=112
    let mut pc: u32 = 0x825FF1C8;
    'dispatch: loop {
        match pc {
            0x825FF1C8 => {
    //   block [0x825FF1C8..0x825FF238)
	// 825FF1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF1D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF1D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF1D8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF1DC: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF1E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF1E4: 390B4B70  addi r8, r11, 0x4b70
	ctx.r[8].s64 = ctx.r[11].s64 + 19312;
	// 825FF1E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FF1EC: 388AB7F0  addi r4, r10, -0x4810
	ctx.r[4].s64 = ctx.r[10].s64 + -18448;
	// 825FF1F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF1F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF1F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF1FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF200: 386A1D0C  addi r3, r10, 0x1d0c
	ctx.r[3].s64 = ctx.r[10].s64 + 7436;
	// 825FF204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF20C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF21C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF224: 4BE67BFD  bl 0x82466e20
	ctx.lr = 0x825FF228;
	sub_82466E20(ctx, base);
	// 825FF228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF238 size=112
    let mut pc: u32 = 0x825FF238;
    'dispatch: loop {
        match pc {
            0x825FF238 => {
    //   block [0x825FF238..0x825FF2A8)
	// 825FF238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF244: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF248: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF24C: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF250: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF254: 390B4BB8  addi r8, r11, 0x4bb8
	ctx.r[8].s64 = ctx.r[11].s64 + 19384;
	// 825FF258: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FF25C: 388AB808  addi r4, r10, -0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + -18424;
	// 825FF260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF264: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF26C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF270: 386A1D3C  addi r3, r10, 0x1d3c
	ctx.r[3].s64 = ctx.r[10].s64 + 7484;
	// 825FF274: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF28C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF294: 4BE67B8D  bl 0x82466e20
	ctx.lr = 0x825FF298;
	sub_82466E20(ctx, base);
	// 825FF298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF2A8 size=108
    let mut pc: u32 = 0x825FF2A8;
    'dispatch: loop {
        match pc {
            0x825FF2A8 => {
    //   block [0x825FF2A8..0x825FF314)
	// 825FF2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF2B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF2B4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF2B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF2BC: 38EB4C00  addi r7, r11, 0x4c00
	ctx.r[7].s64 = ctx.r[11].s64 + 19456;
	// 825FF2C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FF2C4: 388AB820  addi r4, r10, -0x47e0
	ctx.r[4].s64 = ctx.r[10].s64 + -18400;
	// 825FF2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF2CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF2D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FF2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF2D8: 386A1D6C  addi r3, r10, 0x1d6c
	ctx.r[3].s64 = ctx.r[10].s64 + 7532;
	// 825FF2DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FF2E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF2E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF2F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF2F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF2F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF2FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FF300: 4BE67B21  bl 0x82466e20
	ctx.lr = 0x825FF304;
	sub_82466E20(ctx, base);
	// 825FF304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF318 size=112
    let mut pc: u32 = 0x825FF318;
    'dispatch: loop {
        match pc {
            0x825FF318 => {
    //   block [0x825FF318..0x825FF388)
	// 825FF318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF324: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF328: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF32C: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF330: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF334: 390B4C48  addi r8, r11, 0x4c48
	ctx.r[8].s64 = ctx.r[11].s64 + 19528;
	// 825FF338: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825FF33C: 388AB844  addi r4, r10, -0x47bc
	ctx.r[4].s64 = ctx.r[10].s64 + -18364;
	// 825FF340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF344: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF34C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF350: 386A1D9C  addi r3, r10, 0x1d9c
	ctx.r[3].s64 = ctx.r[10].s64 + 7580;
	// 825FF354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF35C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF36C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF374: 4BE67AAD  bl 0x82466e20
	ctx.lr = 0x825FF378;
	sub_82466E20(ctx, base);
	// 825FF378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF388 size=116
    let mut pc: u32 = 0x825FF388;
    'dispatch: loop {
        match pc {
            0x825FF388 => {
    //   block [0x825FF388..0x825FF3FC)
	// 825FF388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF394: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FF398: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF39C: 392B9150  addi r9, r11, -0x6eb0
	ctx.r[9].s64 = ctx.r[11].s64 + -28336;
	// 825FF3A0: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF3A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF3A8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825FF3AC: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 825FF3B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF3B4: 388AB85C  addi r4, r10, -0x47a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18340;
	// 825FF3B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF3BC: 396B4CC0  addi r11, r11, 0x4cc0
	ctx.r[11].s64 = ctx.r[11].s64 + 19648;
	// 825FF3C0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825FF3C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF3C8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825FF3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF3D0: 386A1DCC  addi r3, r10, 0x1dcc
	ctx.r[3].s64 = ctx.r[10].s64 + 7628;
	// 825FF3D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FF3D8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825FF3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF3E0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825FF3E4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FF3E8: 4BE67A39  bl 0x82466e20
	ctx.lr = 0x825FF3EC;
	sub_82466E20(ctx, base);
	// 825FF3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF400 size=108
    let mut pc: u32 = 0x825FF400;
    'dispatch: loop {
        match pc {
            0x825FF400 => {
    //   block [0x825FF400..0x825FF46C)
	// 825FF400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF40C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF410: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF414: 38EB4D50  addi r7, r11, 0x4d50
	ctx.r[7].s64 = ctx.r[11].s64 + 19792;
	// 825FF418: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FF41C: 388AB870  addi r4, r10, -0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + -18320;
	// 825FF420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF424: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF428: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FF42C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF430: 386A1DFC  addi r3, r10, 0x1dfc
	ctx.r[3].s64 = ctx.r[10].s64 + 7676;
	// 825FF434: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FF438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF43C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF44C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FF458: 4BE679C9  bl 0x82466e20
	ctx.lr = 0x825FF45C;
	sub_82466E20(ctx, base);
	// 825FF45C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF470 size=108
    let mut pc: u32 = 0x825FF470;
    'dispatch: loop {
        match pc {
            0x825FF470 => {
    //   block [0x825FF470..0x825FF4DC)
	// 825FF470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF47C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF480: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF484: 38EB4D98  addi r7, r11, 0x4d98
	ctx.r[7].s64 = ctx.r[11].s64 + 19864;
	// 825FF488: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FF48C: 388AB898  addi r4, r10, -0x4768
	ctx.r[4].s64 = ctx.r[10].s64 + -18280;
	// 825FF490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF494: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FF49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF4A0: 386A1E2C  addi r3, r10, 0x1e2c
	ctx.r[3].s64 = ctx.r[10].s64 + 7724;
	// 825FF4A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FF4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF4C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FF4C8: 4BE67959  bl 0x82466e20
	ctx.lr = 0x825FF4CC;
	sub_82466E20(ctx, base);
	// 825FF4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF4E0 size=112
    let mut pc: u32 = 0x825FF4E0;
    'dispatch: loop {
        match pc {
            0x825FF4E0 => {
    //   block [0x825FF4E0..0x825FF550)
	// 825FF4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF4EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF4F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF4F4: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF4F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF4FC: 390B4DF8  addi r8, r11, 0x4df8
	ctx.r[8].s64 = ctx.r[11].s64 + 19960;
	// 825FF500: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FF504: 388AB8C0  addi r4, r10, -0x4740
	ctx.r[4].s64 = ctx.r[10].s64 + -18240;
	// 825FF508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF50C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF518: 386A1E5C  addi r3, r10, 0x1e5c
	ctx.r[3].s64 = ctx.r[10].s64 + 7772;
	// 825FF51C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF53C: 4BE678E5  bl 0x82466e20
	ctx.lr = 0x825FF540;
	sub_82466E20(ctx, base);
	// 825FF540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF550 size=112
    let mut pc: u32 = 0x825FF550;
    'dispatch: loop {
        match pc {
            0x825FF550 => {
    //   block [0x825FF550..0x825FF5C0)
	// 825FF550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF55C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF560: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF564: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF568: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF56C: 390B4E58  addi r8, r11, 0x4e58
	ctx.r[8].s64 = ctx.r[11].s64 + 20056;
	// 825FF570: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FF574: 388AB8E0  addi r4, r10, -0x4720
	ctx.r[4].s64 = ctx.r[10].s64 + -18208;
	// 825FF578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF57C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF588: 386A1E8C  addi r3, r10, 0x1e8c
	ctx.r[3].s64 = ctx.r[10].s64 + 7820;
	// 825FF58C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF59C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF5A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF5A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF5A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF5AC: 4BE67875  bl 0x82466e20
	ctx.lr = 0x825FF5B0;
	sub_82466E20(ctx, base);
	// 825FF5B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FF5C0 size=24
    let mut pc: u32 = 0x825FF5C0;
    'dispatch: loop {
        match pc {
            0x825FF5C0 => {
    //   block [0x825FF5C0..0x825FF5D8)
	// 825FF5C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF5C4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FF5C8: 394AB040  addi r10, r10, -0x4fc0
	ctx.r[10].s64 = ctx.r[10].s64 + -20416;
	// 825FF5CC: 816B51E8  lwz r11, 0x51e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20968 as u32) ) } as u64;
	// 825FF5D0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825FF5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF5D8 size=116
    let mut pc: u32 = 0x825FF5D8;
    'dispatch: loop {
        match pc {
            0x825FF5D8 => {
    //   block [0x825FF5D8..0x825FF64C)
	// 825FF5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF5E4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FF5E8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825FF5EC: 390AB040  addi r8, r10, -0x4fc0
	ctx.r[8].s64 = ctx.r[10].s64 + -20416;
	// 825FF5F0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF5F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FF5F8: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF5FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF600: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FF604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF60C: 388AB8F0  addi r4, r10, -0x4710
	ctx.r[4].s64 = ctx.r[10].s64 + -18192;
	// 825FF610: 396B9180  addi r11, r11, -0x6e80
	ctx.r[11].s64 = ctx.r[11].s64 + -28288;
	// 825FF614: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF618: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF61C: 386A1EBC  addi r3, r10, 0x1ebc
	ctx.r[3].s64 = ctx.r[10].s64 + 7868;
	// 825FF620: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FF624: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF628: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FF62C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF638: 4BE677E9  bl 0x82466e20
	ctx.lr = 0x825FF63C;
	sub_82466E20(ctx, base);
	// 825FF63C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF650 size=100
    let mut pc: u32 = 0x825FF650;
    'dispatch: loop {
        match pc {
            0x825FF650 => {
    //   block [0x825FF650..0x825FF6B4)
	// 825FF650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF65C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF664: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FF668: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF670: 388AB904  addi r4, r10, -0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + -18172;
	// 825FF674: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF67C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF684: 386A1EEC  addi r3, r10, 0x1eec
	ctx.r[3].s64 = ctx.r[10].s64 + 7916;
	// 825FF688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF68C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF690: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FF694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF698: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FF69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF6A0: 4BE67781  bl 0x82466e20
	ctx.lr = 0x825FF6A4;
	sub_82466E20(ctx, base);
	// 825FF6A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF6A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF6AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF6B8 size=100
    let mut pc: u32 = 0x825FF6B8;
    'dispatch: loop {
        match pc {
            0x825FF6B8 => {
    //   block [0x825FF6B8..0x825FF71C)
	// 825FF6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF6C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF6C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF6CC: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FF6D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF6D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF6D8: 388AB918  addi r4, r10, -0x46e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18152;
	// 825FF6DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF6EC: 386A1F1C  addi r3, r10, 0x1f1c
	ctx.r[3].s64 = ctx.r[10].s64 + 7964;
	// 825FF6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF6F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF6F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FF6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF700: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FF704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF708: 4BE67719  bl 0x82466e20
	ctx.lr = 0x825FF70C;
	sub_82466E20(ctx, base);
	// 825FF70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF720 size=112
    let mut pc: u32 = 0x825FF720;
    'dispatch: loop {
        match pc {
            0x825FF720 => {
    //   block [0x825FF720..0x825FF790)
	// 825FF720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF72C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF730: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF734: 38AA1EEC  addi r5, r10, 0x1eec
	ctx.r[5].s64 = ctx.r[10].s64 + 7916;
	// 825FF738: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF73C: 390B4E70  addi r8, r11, 0x4e70
	ctx.r[8].s64 = ctx.r[11].s64 + 20080;
	// 825FF740: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FF744: 388AB92C  addi r4, r10, -0x46d4
	ctx.r[4].s64 = ctx.r[10].s64 + -18132;
	// 825FF748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF74C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF758: 386A1F4C  addi r3, r10, 0x1f4c
	ctx.r[3].s64 = ctx.r[10].s64 + 8012;
	// 825FF75C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF76C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF77C: 4BE676A5  bl 0x82466e20
	ctx.lr = 0x825FF780;
	sub_82466E20(ctx, base);
	// 825FF780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF790 size=112
    let mut pc: u32 = 0x825FF790;
    'dispatch: loop {
        match pc {
            0x825FF790 => {
    //   block [0x825FF790..0x825FF800)
	// 825FF790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF79C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF7A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF7A4: 38AA1EEC  addi r5, r10, 0x1eec
	ctx.r[5].s64 = ctx.r[10].s64 + 7916;
	// 825FF7A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF7AC: 390B4EB8  addi r8, r11, 0x4eb8
	ctx.r[8].s64 = ctx.r[11].s64 + 20152;
	// 825FF7B0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 825FF7B4: 388AB93C  addi r4, r10, -0x46c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18116;
	// 825FF7B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF7BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF7C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF7C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF7C8: 386A1F7C  addi r3, r10, 0x1f7c
	ctx.r[3].s64 = ctx.r[10].s64 + 8060;
	// 825FF7CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF7D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF7D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF7D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF7DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF7E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF7E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF7E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF7EC: 4BE67635  bl 0x82466e20
	ctx.lr = 0x825FF7F0;
	sub_82466E20(ctx, base);
	// 825FF7F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF800 size=112
    let mut pc: u32 = 0x825FF800;
    'dispatch: loop {
        match pc {
            0x825FF800 => {
    //   block [0x825FF800..0x825FF870)
	// 825FF800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF80C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF810: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF814: 38AA1F7C  addi r5, r10, 0x1f7c
	ctx.r[5].s64 = ctx.r[10].s64 + 8060;
	// 825FF818: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF81C: 390B4F78  addi r8, r11, 0x4f78
	ctx.r[8].s64 = ctx.r[11].s64 + 20344;
	// 825FF820: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FF824: 388AB958  addi r4, r10, -0x46a8
	ctx.r[4].s64 = ctx.r[10].s64 + -18088;
	// 825FF828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF82C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF838: 386A1FAC  addi r3, r10, 0x1fac
	ctx.r[3].s64 = ctx.r[10].s64 + 8108;
	// 825FF83C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF84C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF85C: 4BE675C5  bl 0x82466e20
	ctx.lr = 0x825FF860;
	sub_82466E20(ctx, base);
	// 825FF860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF870 size=112
    let mut pc: u32 = 0x825FF870;
    'dispatch: loop {
        match pc {
            0x825FF870 => {
    //   block [0x825FF870..0x825FF8E0)
	// 825FF870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF87C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF880: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF884: 38AA1B8C  addi r5, r10, 0x1b8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7052;
	// 825FF888: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF88C: 390B4FA8  addi r8, r11, 0x4fa8
	ctx.r[8].s64 = ctx.r[11].s64 + 20392;
	// 825FF890: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FF894: 388AB97C  addi r4, r10, -0x4684
	ctx.r[4].s64 = ctx.r[10].s64 + -18052;
	// 825FF898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF89C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF8A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF8A8: 386A1FDC  addi r3, r10, 0x1fdc
	ctx.r[3].s64 = ctx.r[10].s64 + 8156;
	// 825FF8AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF8B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF8B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF8BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF8C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF8C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF8C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF8CC: 4BE67555  bl 0x82466e20
	ctx.lr = 0x825FF8D0;
	sub_82466E20(ctx, base);
	// 825FF8D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF8E0 size=112
    let mut pc: u32 = 0x825FF8E0;
    'dispatch: loop {
        match pc {
            0x825FF8E0 => {
    //   block [0x825FF8E0..0x825FF950)
	// 825FF8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF8EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF8F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF8F4: 38AA17FC  addi r5, r10, 0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + 6140;
	// 825FF8F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF8FC: 390B4FD8  addi r8, r11, 0x4fd8
	ctx.r[8].s64 = ctx.r[11].s64 + 20440;
	// 825FF900: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FF904: 388AB9A0  addi r4, r10, -0x4660
	ctx.r[4].s64 = ctx.r[10].s64 + -18016;
	// 825FF908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF90C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF910: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF918: 386A200C  addi r3, r10, 0x200c
	ctx.r[3].s64 = ctx.r[10].s64 + 8204;
	// 825FF91C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF92C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF93C: 4BE674E5  bl 0x82466e20
	ctx.lr = 0x825FF940;
	sub_82466E20(ctx, base);
	// 825FF940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF950 size=112
    let mut pc: u32 = 0x825FF950;
    'dispatch: loop {
        match pc {
            0x825FF950 => {
    //   block [0x825FF950..0x825FF9C0)
	// 825FF950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF95C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF960: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF964: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FF968: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF96C: 390B5008  addi r8, r11, 0x5008
	ctx.r[8].s64 = ctx.r[11].s64 + 20488;
	// 825FF970: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FF974: 388AB9C4  addi r4, r10, -0x463c
	ctx.r[4].s64 = ctx.r[10].s64 + -17980;
	// 825FF978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF97C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF988: 386A203C  addi r3, r10, 0x203c
	ctx.r[3].s64 = ctx.r[10].s64 + 8252;
	// 825FF98C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF9A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF9AC: 4BE67475  bl 0x82466e20
	ctx.lr = 0x825FF9B0;
	sub_82466E20(ctx, base);
	// 825FF9B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF9C0 size=112
    let mut pc: u32 = 0x825FF9C0;
    'dispatch: loop {
        match pc {
            0x825FF9C0 => {
    //   block [0x825FF9C0..0x825FFA30)
	// 825FF9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF9CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF9D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF9D4: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF9D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF9DC: 390B5038  addi r8, r11, 0x5038
	ctx.r[8].s64 = ctx.r[11].s64 + 20536;
	// 825FF9E0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825FF9E4: 388AB9D0  addi r4, r10, -0x4630
	ctx.r[4].s64 = ctx.r[10].s64 + -17968;
	// 825FF9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF9EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF9F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF9F8: 386A206C  addi r3, r10, 0x206c
	ctx.r[3].s64 = ctx.r[10].s64 + 8300;
	// 825FF9FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FFA00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFA04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFA0C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FFA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFA14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFA1C: 4BE67405  bl 0x82466e20
	ctx.lr = 0x825FFA20;
	sub_82466E20(ctx, base);
	// 825FFA20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFA30 size=108
    let mut pc: u32 = 0x825FFA30;
    'dispatch: loop {
        match pc {
            0x825FFA30 => {
    //   block [0x825FFA30..0x825FFA9C)
	// 825FFA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFA3C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFA40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFA44: 38EB50B0  addi r7, r11, 0x50b0
	ctx.r[7].s64 = ctx.r[11].s64 + 20656;
	// 825FFA48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FFA4C: 388AB9E4  addi r4, r10, -0x461c
	ctx.r[4].s64 = ctx.r[10].s64 + -17948;
	// 825FFA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFA54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFA58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FFA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFA60: 386A209C  addi r3, r10, 0x209c
	ctx.r[3].s64 = ctx.r[10].s64 + 8348;
	// 825FFA64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FFA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFA84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FFA88: 4BE67399  bl 0x82466e20
	ctx.lr = 0x825FFA8C;
	sub_82466E20(ctx, base);
	// 825FFA8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFAA0 size=112
    let mut pc: u32 = 0x825FFAA0;
    'dispatch: loop {
        match pc {
            0x825FFAA0 => {
    //   block [0x825FFAA0..0x825FFB10)
	// 825FFAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFAA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFAAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFAB0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFAB4: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FFAB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFABC: 390B50E0  addi r8, r11, 0x50e0
	ctx.r[8].s64 = ctx.r[11].s64 + 20704;
	// 825FFAC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FFAC4: 388AB9F8  addi r4, r10, -0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + -17928;
	// 825FFAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFACC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFAD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FFAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFAD8: 386A20CC  addi r3, r10, 0x20cc
	ctx.r[3].s64 = ctx.r[10].s64 + 8396;
	// 825FFADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FFAE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFAE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFAE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFAF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFAF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFAF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFAFC: 4BE67325  bl 0x82466e20
	ctx.lr = 0x825FFB00;
	sub_82466E20(ctx, base);
	// 825FFB00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFB10 size=112
    let mut pc: u32 = 0x825FFB10;
    'dispatch: loop {
        match pc {
            0x825FFB10 => {
    //   block [0x825FFB10..0x825FFB80)
	// 825FFB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFB1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFB20: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFB24: 38AA1F1C  addi r5, r10, 0x1f1c
	ctx.r[5].s64 = ctx.r[10].s64 + 7964;
	// 825FFB28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFB2C: 390B5110  addi r8, r11, 0x5110
	ctx.r[8].s64 = ctx.r[11].s64 + 20752;
	// 825FFB30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FFB34: 388ABA0C  addi r4, r10, -0x45f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17908;
	// 825FFB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFB3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFB40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FFB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFB48: 386A20FC  addi r3, r10, 0x20fc
	ctx.r[3].s64 = ctx.r[10].s64 + 8444;
	// 825FFB4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FFB50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFB54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFB58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFB60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFB68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFB6C: 4BE672B5  bl 0x82466e20
	ctx.lr = 0x825FFB70;
	sub_82466E20(ctx, base);
	// 825FFB70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFB80 size=100
    let mut pc: u32 = 0x825FFB80;
    'dispatch: loop {
        match pc {
            0x825FFB80 => {
    //   block [0x825FFB80..0x825FFBE4)
	// 825FFB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFB88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFB8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFB94: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FFB98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFBA0: 388ABA20  addi r4, r10, -0x45e0
	ctx.r[4].s64 = ctx.r[10].s64 + -17888;
	// 825FFBA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFBB4: 386A212C  addi r3, r10, 0x212c
	ctx.r[3].s64 = ctx.r[10].s64 + 8492;
	// 825FFBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFBBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFBC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FFBC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFBC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FFBCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFBD0: 4BE67251  bl 0x82466e20
	ctx.lr = 0x825FFBD4;
	sub_82466E20(ctx, base);
	// 825FFBD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFBD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFBDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFBE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFBE8 size=112
    let mut pc: u32 = 0x825FFBE8;
    'dispatch: loop {
        match pc {
            0x825FFBE8 => {
    //   block [0x825FFBE8..0x825FFC58)
	// 825FFBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFBF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFBF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFBF8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFBFC: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FFC00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFC04: 390B5140  addi r8, r11, 0x5140
	ctx.r[8].s64 = ctx.r[11].s64 + 20800;
	// 825FFC08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FFC0C: 388ABA38  addi r4, r10, -0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + -17864;
	// 825FFC10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFC14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFC18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FFC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFC20: 386A215C  addi r3, r10, 0x215c
	ctx.r[3].s64 = ctx.r[10].s64 + 8540;
	// 825FFC24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FFC28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFC30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFC34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFC38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFC3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFC40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFC44: 4BE671DD  bl 0x82466e20
	ctx.lr = 0x825FFC48;
	sub_82466E20(ctx, base);
	// 825FFC48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFC58 size=96
    let mut pc: u32 = 0x825FFC58;
    'dispatch: loop {
        match pc {
            0x825FFC58 => {
    //   block [0x825FFC58..0x825FFCB8)
	// 825FFC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFC60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFC64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFC6C: 388ABA4C  addi r4, r10, -0x45b4
	ctx.r[4].s64 = ctx.r[10].s64 + -17844;
	// 825FFC70: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFC78: 386A218C  addi r3, r10, 0x218c
	ctx.r[3].s64 = ctx.r[10].s64 + 8588;
	// 825FFC7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFC80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFC84: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FFC88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFC8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFC90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFC94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFC98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FFC9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FFCA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FFCA4: 4BE6717D  bl 0x82466e20
	ctx.lr = 0x825FFCA8;
	sub_82466E20(ctx, base);
	// 825FFCA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFCAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFCB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFCB8 size=108
    let mut pc: u32 = 0x825FFCB8;
    'dispatch: loop {
        match pc {
            0x825FFCB8 => {
    //   block [0x825FFCB8..0x825FFD24)
	// 825FFCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFCC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFCC4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFCC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFCCC: 38EB5188  addi r7, r11, 0x5188
	ctx.r[7].s64 = ctx.r[11].s64 + 20872;
	// 825FFCD0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FFCD4: 388ABA68  addi r4, r10, -0x4598
	ctx.r[4].s64 = ctx.r[10].s64 + -17816;
	// 825FFCD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFCDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFCE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FFCE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFCE8: 386A21BC  addi r3, r10, 0x21bc
	ctx.r[3].s64 = ctx.r[10].s64 + 8636;
	// 825FFCEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FFCF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFCF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFCF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFCFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFD00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFD04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFD08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFD0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FFD10: 4BE67111  bl 0x82466e20
	ctx.lr = 0x825FFD14;
	sub_82466E20(ctx, base);
	// 825FFD14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFD18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFD1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFD20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFD28 size=100
    let mut pc: u32 = 0x825FFD28;
    'dispatch: loop {
        match pc {
            0x825FFD28 => {
    //   block [0x825FFD28..0x825FFD8C)
	// 825FFD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFD34: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFD3C: 392A91F0  addi r9, r10, -0x6e10
	ctx.r[9].s64 = ctx.r[10].s64 + -28176;
	// 825FFD40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFD44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFD48: 388ABA80  addi r4, r10, -0x4580
	ctx.r[4].s64 = ctx.r[10].s64 + -17792;
	// 825FFD4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFD54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFD5C: 386A21EC  addi r3, r10, 0x21ec
	ctx.r[3].s64 = ctx.r[10].s64 + 8684;
	// 825FFD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFD64: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825FFD68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FFD6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFD70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FFD74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FFD78: 4BE670A9  bl 0x82466e20
	ctx.lr = 0x825FFD7C;
	sub_82466E20(ctx, base);
	// 825FFD7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFD80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFD84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FFD90 size=24
    let mut pc: u32 = 0x825FFD90;
    'dispatch: loop {
        match pc {
            0x825FFD90 => {
    //   block [0x825FFD90..0x825FFDA8)
	// 825FFD90: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFD94: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FFD98: 394AB0E8  addi r10, r10, -0x4f18
	ctx.r[10].s64 = ctx.r[10].s64 + -20248;
	// 825FFD9C: 816B51F4  lwz r11, 0x51f4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20980 as u32) ) } as u64;
	// 825FFDA0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825FFDA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFDA8 size=112
    let mut pc: u32 = 0x825FFDA8;
    'dispatch: loop {
        match pc {
            0x825FFDA8 => {
    //   block [0x825FFDA8..0x825FFE18)
	// 825FFDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFDB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFDB4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFDB8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FFDBC: 392A9330  addi r9, r10, -0x6cd0
	ctx.r[9].s64 = ctx.r[10].s64 + -27856;
	// 825FFDC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFDC4: 390BB0E8  addi r8, r11, -0x4f18
	ctx.r[8].s64 = ctx.r[11].s64 + -20248;
	// 825FFDC8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825FFDCC: 388ABA94  addi r4, r10, -0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + -17772;
	// 825FFDD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFDD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFDD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FFDDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFDE0: 386A221C  addi r3, r10, 0x221c
	ctx.r[3].s64 = ctx.r[10].s64 + 8732;
	// 825FFDE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FFDE8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FFDEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFDF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFDF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFDF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFDFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FFE00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFE04: 4BE6701D  bl 0x82466e20
	ctx.lr = 0x825FFE08;
	sub_82466E20(ctx, base);
	// 825FFE08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFE18 size=112
    let mut pc: u32 = 0x825FFE18;
    'dispatch: loop {
        match pc {
            0x825FFE18 => {
    //   block [0x825FFE18..0x825FFE88)
	// 825FFE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFE24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFE28: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFE2C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 825FFE30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFE34: 390B51FC  addi r8, r11, 0x51fc
	ctx.r[8].s64 = ctx.r[11].s64 + 20988;
	// 825FFE38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FFE3C: 388ABAA8  addi r4, r10, -0x4558
	ctx.r[4].s64 = ctx.r[10].s64 + -17752;
	// 825FFE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFE44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFE48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FFE4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFE50: 386A224C  addi r3, r10, 0x224c
	ctx.r[3].s64 = ctx.r[10].s64 + 8780;
	// 825FFE54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FFE58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFE5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFE60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFE64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFE68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFE6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFE70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFE74: 4BE66FAD  bl 0x82466e20
	ctx.lr = 0x825FFE78;
	sub_82466E20(ctx, base);
	// 825FFE78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFE88 size=108
    let mut pc: u32 = 0x825FFE88;
    'dispatch: loop {
        match pc {
            0x825FFE88 => {
    //   block [0x825FFE88..0x825FFEF4)
	// 825FFE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFE94: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFE98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFE9C: 38EB522C  addi r7, r11, 0x522c
	ctx.r[7].s64 = ctx.r[11].s64 + 21036;
	// 825FFEA0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FFEA4: 388ABAC0  addi r4, r10, -0x4540
	ctx.r[4].s64 = ctx.r[10].s64 + -17728;
	// 825FFEA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFEAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFEB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FFEB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFEB8: 386A227C  addi r3, r10, 0x227c
	ctx.r[3].s64 = ctx.r[10].s64 + 8828;
	// 825FFEBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FFEC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFEC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFEDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FFEE0: 4BE66F41  bl 0x82466e20
	ctx.lr = 0x825FFEE4;
	sub_82466E20(ctx, base);
	// 825FFEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFEF8 size=112
    let mut pc: u32 = 0x825FFEF8;
    'dispatch: loop {
        match pc {
            0x825FFEF8 => {
    //   block [0x825FFEF8..0x825FFF68)
	// 825FFEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFF04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFF08: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFF0C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 825FFF10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFF14: 390B5248  addi r8, r11, 0x5248
	ctx.r[8].s64 = ctx.r[11].s64 + 21064;
	// 825FFF18: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825FFF1C: 388ABAD0  addi r4, r10, -0x4530
	ctx.r[4].s64 = ctx.r[10].s64 + -17712;
	// 825FFF20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFF24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFF28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FFF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFF30: 386A22AC  addi r3, r10, 0x22ac
	ctx.r[3].s64 = ctx.r[10].s64 + 8876;
	// 825FFF34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FFF38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFF40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFF48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFF4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFF50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFF54: 4BE66ECD  bl 0x82466e20
	ctx.lr = 0x825FFF58;
	sub_82466E20(ctx, base);
	// 825FFF58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFF68 size=100
    let mut pc: u32 = 0x825FFF68;
    'dispatch: loop {
        match pc {
            0x825FFF68 => {
    //   block [0x825FFF68..0x825FFFCC)
	// 825FFF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFF70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFF74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFF7C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 825FFF80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFF84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFF88: 388ABAF0  addi r4, r10, -0x4510
	ctx.r[4].s64 = ctx.r[10].s64 + -17680;
	// 825FFF8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFF9C: 386A22DC  addi r3, r10, 0x22dc
	ctx.r[3].s64 = ctx.r[10].s64 + 8924;
	// 825FFFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFFA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFFA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FFFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFFB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FFFB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFFB8: 4BE66E69  bl 0x82466e20
	ctx.lr = 0x825FFFBC;
	sub_82466E20(ctx, base);
	// 825FFFBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFFC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFFC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFFC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFFD0 size=112
    let mut pc: u32 = 0x825FFFD0;
    'dispatch: loop {
        match pc {
            0x825FFFD0 => {
    //   block [0x825FFFD0..0x82600040)
	// 825FFFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFFD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFFDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFFE0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFFE4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 825FFFE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFFEC: 390B52F0  addi r8, r11, 0x52f0
	ctx.r[8].s64 = ctx.r[11].s64 + 21232;
	// 825FFFF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FFFF4: 388ABB0C  addi r4, r10, -0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17652;
	// 825FFFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFFFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600000: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600008: 386A230C  addi r3, r10, 0x230c
	ctx.r[3].s64 = ctx.r[10].s64 + 8972;
	// 8260000C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260001C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260002C: 4BE66DF5  bl 0x82466e20
	ctx.lr = 0x82600030;
	sub_82466E20(ctx, base);
	// 82600030: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260003C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600040 size=112
    let mut pc: u32 = 0x82600040;
    'dispatch: loop {
        match pc {
            0x82600040 => {
    //   block [0x82600040..0x826000B0)
	// 82600040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260004C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600050: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600054: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600058: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260005C: 390B5308  addi r8, r11, 0x5308
	ctx.r[8].s64 = ctx.r[11].s64 + 21256;
	// 82600060: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82600064: 388ABB2C  addi r4, r10, -0x44d4
	ctx.r[4].s64 = ctx.r[10].s64 + -17620;
	// 82600068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260006C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600070: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600078: 386A233C  addi r3, r10, 0x233c
	ctx.r[3].s64 = ctx.r[10].s64 + 9020;
	// 8260007C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260008C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260009C: 4BE66D85  bl 0x82466e20
	ctx.lr = 0x826000A0;
	sub_82466E20(ctx, base);
	// 826000A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826000A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826000A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826000AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826000B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826000B0 size=112
    let mut pc: u32 = 0x826000B0;
    'dispatch: loop {
        match pc {
            0x826000B0 => {
    //   block [0x826000B0..0x82600120)
	// 826000B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826000B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826000B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826000BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826000C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826000C4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826000C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826000CC: 390B5338  addi r8, r11, 0x5338
	ctx.r[8].s64 = ctx.r[11].s64 + 21304;
	// 826000D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826000D4: 388ABB50  addi r4, r10, -0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + -17584;
	// 826000D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826000DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826000E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826000E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826000E8: 386A236C  addi r3, r10, 0x236c
	ctx.r[3].s64 = ctx.r[10].s64 + 9068;
	// 826000EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826000F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826000F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826000F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826000FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260010C: 4BE66D15  bl 0x82466e20
	ctx.lr = 0x82600110;
	sub_82466E20(ctx, base);
	// 82600110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260011C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600120 size=112
    let mut pc: u32 = 0x82600120;
    'dispatch: loop {
        match pc {
            0x82600120 => {
    //   block [0x82600120..0x82600190)
	// 82600120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260012C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600130: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600134: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600138: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260013C: 390B5368  addi r8, r11, 0x5368
	ctx.r[8].s64 = ctx.r[11].s64 + 21352;
	// 82600140: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82600144: 388ABB78  addi r4, r10, -0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + -17544;
	// 82600148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260014C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600158: 386A239C  addi r3, r10, 0x239c
	ctx.r[3].s64 = ctx.r[10].s64 + 9116;
	// 8260015C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260016C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260017C: 4BE66CA5  bl 0x82466e20
	ctx.lr = 0x82600180;
	sub_82466E20(ctx, base);
	// 82600180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260018C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600190 size=112
    let mut pc: u32 = 0x82600190;
    'dispatch: loop {
        match pc {
            0x82600190 => {
    //   block [0x82600190..0x82600200)
	// 82600190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260019C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826001A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826001A4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826001A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826001AC: 390B5398  addi r8, r11, 0x5398
	ctx.r[8].s64 = ctx.r[11].s64 + 21400;
	// 826001B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826001B4: 388ABB9C  addi r4, r10, -0x4464
	ctx.r[4].s64 = ctx.r[10].s64 + -17508;
	// 826001B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826001BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826001C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826001C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826001C8: 386A23CC  addi r3, r10, 0x23cc
	ctx.r[3].s64 = ctx.r[10].s64 + 9164;
	// 826001CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826001D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826001D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826001D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826001DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826001E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826001E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826001E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826001EC: 4BE66C35  bl 0x82466e20
	ctx.lr = 0x826001F0;
	sub_82466E20(ctx, base);
	// 826001F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826001F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826001F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826001FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600200 size=112
    let mut pc: u32 = 0x82600200;
    'dispatch: loop {
        match pc {
            0x82600200 => {
    //   block [0x82600200..0x82600270)
	// 82600200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260020C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600210: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600214: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600218: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260021C: 390B53B0  addi r8, r11, 0x53b0
	ctx.r[8].s64 = ctx.r[11].s64 + 21424;
	// 82600220: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82600224: 388ABBBC  addi r4, r10, -0x4444
	ctx.r[4].s64 = ctx.r[10].s64 + -17476;
	// 82600228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260022C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600238: 386A23FC  addi r3, r10, 0x23fc
	ctx.r[3].s64 = ctx.r[10].s64 + 9212;
	// 8260023C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260024C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260025C: 4BE66BC5  bl 0x82466e20
	ctx.lr = 0x82600260;
	sub_82466E20(ctx, base);
	// 82600260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260026C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600270 size=112
    let mut pc: u32 = 0x82600270;
    'dispatch: loop {
        match pc {
            0x82600270 => {
    //   block [0x82600270..0x826002E0)
	// 82600270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260027C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600280: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600284: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600288: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260028C: 390B53C8  addi r8, r11, 0x53c8
	ctx.r[8].s64 = ctx.r[11].s64 + 21448;
	// 82600290: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82600294: 388ABBD4  addi r4, r10, -0x442c
	ctx.r[4].s64 = ctx.r[10].s64 + -17452;
	// 82600298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260029C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826002A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826002A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826002A8: 386A242C  addi r3, r10, 0x242c
	ctx.r[3].s64 = ctx.r[10].s64 + 9260;
	// 826002AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826002B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826002B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826002B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826002BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826002C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826002C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826002C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826002CC: 4BE66B55  bl 0x82466e20
	ctx.lr = 0x826002D0;
	sub_82466E20(ctx, base);
	// 826002D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826002D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826002D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826002DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826002E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826002E0 size=112
    let mut pc: u32 = 0x826002E0;
    'dispatch: loop {
        match pc {
            0x826002E0 => {
    //   block [0x826002E0..0x82600350)
	// 826002E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826002E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826002E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826002EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826002F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826002F4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826002F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826002FC: 390B5410  addi r8, r11, 0x5410
	ctx.r[8].s64 = ctx.r[11].s64 + 21520;
	// 82600300: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82600304: 388ABBF0  addi r4, r10, -0x4410
	ctx.r[4].s64 = ctx.r[10].s64 + -17424;
	// 82600308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260030C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600310: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600318: 386A245C  addi r3, r10, 0x245c
	ctx.r[3].s64 = ctx.r[10].s64 + 9308;
	// 8260031C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260032C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260033C: 4BE66AE5  bl 0x82466e20
	ctx.lr = 0x82600340;
	sub_82466E20(ctx, base);
	// 82600340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260034C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600350 size=112
    let mut pc: u32 = 0x82600350;
    'dispatch: loop {
        match pc {
            0x82600350 => {
    //   block [0x82600350..0x826003C0)
	// 82600350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260035C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600360: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600364: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600368: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260036C: 390B5458  addi r8, r11, 0x5458
	ctx.r[8].s64 = ctx.r[11].s64 + 21592;
	// 82600370: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82600374: 388ABC0C  addi r4, r10, -0x43f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17396;
	// 82600378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260037C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600380: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600388: 386A248C  addi r3, r10, 0x248c
	ctx.r[3].s64 = ctx.r[10].s64 + 9356;
	// 8260038C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260039C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826003A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826003A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826003A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826003AC: 4BE66A75  bl 0x82466e20
	ctx.lr = 0x826003B0;
	sub_82466E20(ctx, base);
	// 826003B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826003B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826003B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826003BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826003C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826003C0 size=112
    let mut pc: u32 = 0x826003C0;
    'dispatch: loop {
        match pc {
            0x826003C0 => {
    //   block [0x826003C0..0x82600430)
	// 826003C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826003C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826003C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826003CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826003D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826003D4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826003D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826003DC: 390B5470  addi r8, r11, 0x5470
	ctx.r[8].s64 = ctx.r[11].s64 + 21616;
	// 826003E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826003E4: 388ABC24  addi r4, r10, -0x43dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17372;
	// 826003E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826003EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826003F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826003F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826003F8: 386A24BC  addi r3, r10, 0x24bc
	ctx.r[3].s64 = ctx.r[10].s64 + 9404;
	// 826003FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260040C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260041C: 4BE66A05  bl 0x82466e20
	ctx.lr = 0x82600420;
	sub_82466E20(ctx, base);
	// 82600420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260042C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600430 size=116
    let mut pc: u32 = 0x82600430;
    'dispatch: loop {
        match pc {
            0x82600430 => {
    //   block [0x82600430..0x826004A4)
	// 82600430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260043C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 82600440: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82600444: 390A54A0  addi r8, r10, 0x54a0
	ctx.r[8].s64 = ctx.r[10].s64 + 21664;
	// 82600448: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260044C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82600450: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600454: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600458: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260045C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600460: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600464: 388ABC3C  addi r4, r10, -0x43c4
	ctx.r[4].s64 = ctx.r[10].s64 + -17348;
	// 82600468: 396B9358  addi r11, r11, -0x6ca8
	ctx.r[11].s64 = ctx.r[11].s64 + -27816;
	// 8260046C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600470: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600474: 386A24EC  addi r3, r10, 0x24ec
	ctx.r[3].s64 = ctx.r[10].s64 + 9452;
	// 82600478: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260047C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600480: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82600484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260048C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600490: 4BE66991  bl 0x82466e20
	ctx.lr = 0x82600494;
	sub_82466E20(ctx, base);
	// 82600494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260049C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826004A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826004A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826004A8 size=116
    let mut pc: u32 = 0x826004A8;
    'dispatch: loop {
        match pc {
            0x826004A8 => {
    //   block [0x826004A8..0x8260051C)
	// 826004A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826004AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826004B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826004B4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 826004B8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826004BC: 390A5518  addi r8, r10, 0x5518
	ctx.r[8].s64 = ctx.r[10].s64 + 21784;
	// 826004C0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826004C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826004C8: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826004CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826004D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826004D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826004D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826004DC: 388ABC58  addi r4, r10, -0x43a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17320;
	// 826004E0: 396B9370  addi r11, r11, -0x6c90
	ctx.r[11].s64 = ctx.r[11].s64 + -27792;
	// 826004E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826004E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826004EC: 386A251C  addi r3, r10, 0x251c
	ctx.r[3].s64 = ctx.r[10].s64 + 9500;
	// 826004F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826004F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826004F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826004FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600508: 4BE66919  bl 0x82466e20
	ctx.lr = 0x8260050C;
	sub_82466E20(ctx, base);
	// 8260050C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82600520 size=24
    let mut pc: u32 = 0x82600520;
    'dispatch: loop {
        match pc {
            0x82600520 => {
    //   block [0x82600520..0x82600538)
	// 82600520: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600524: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82600528: 394AB100  addi r10, r10, -0x4f00
	ctx.r[10].s64 = ctx.r[10].s64 + -20224;
	// 8260052C: 816B5244  lwz r11, 0x5244(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21060 as u32) ) } as u64;
	// 82600530: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82600534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600538 size=116
    let mut pc: u32 = 0x82600538;
    'dispatch: loop {
        match pc {
            0x82600538 => {
    //   block [0x82600538..0x826005AC)
	// 82600538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260053C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600544: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82600548: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260054C: 392B939C  addi r9, r11, -0x6c64
	ctx.r[9].s64 = ctx.r[11].s64 + -27748;
	// 82600550: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600554: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600558: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8260055C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82600560: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82600564: 388ABC8C  addi r4, r10, -0x4374
	ctx.r[4].s64 = ctx.r[10].s64 + -17268;
	// 82600568: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260056C: 396BB100  addi r11, r11, -0x4f00
	ctx.r[11].s64 = ctx.r[11].s64 + -20224;
	// 82600570: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82600574: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600578: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260057C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600580: 386A254C  addi r3, r10, 0x254c
	ctx.r[3].s64 = ctx.r[10].s64 + 9548;
	// 82600584: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82600588: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260058C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600590: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82600594: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82600598: 4BE66889  bl 0x82466e20
	ctx.lr = 0x8260059C;
	sub_82466E20(ctx, base);
	// 8260059C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826005A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826005A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826005A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826005B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826005B0 size=112
    let mut pc: u32 = 0x826005B0;
    'dispatch: loop {
        match pc {
            0x826005B0 => {
    //   block [0x826005B0..0x82600620)
	// 826005B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826005B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826005B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826005BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826005C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826005C4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826005C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826005CC: 390B55A8  addi r8, r11, 0x55a8
	ctx.r[8].s64 = ctx.r[11].s64 + 21928;
	// 826005D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826005D4: 388ABCA8  addi r4, r10, -0x4358
	ctx.r[4].s64 = ctx.r[10].s64 + -17240;
	// 826005D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826005DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826005E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826005E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826005E8: 386A257C  addi r3, r10, 0x257c
	ctx.r[3].s64 = ctx.r[10].s64 + 9596;
	// 826005EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826005F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826005F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826005F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826005FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260060C: 4BE66815  bl 0x82466e20
	ctx.lr = 0x82600610;
	sub_82466E20(ctx, base);
	// 82600610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260061C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600620 size=112
    let mut pc: u32 = 0x82600620;
    'dispatch: loop {
        match pc {
            0x82600620 => {
    //   block [0x82600620..0x82600690)
	// 82600620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260062C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600630: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600634: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600638: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260063C: 390B5608  addi r8, r11, 0x5608
	ctx.r[8].s64 = ctx.r[11].s64 + 22024;
	// 82600640: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82600644: 388ABCC8  addi r4, r10, -0x4338
	ctx.r[4].s64 = ctx.r[10].s64 + -17208;
	// 82600648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260064C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600658: 386A25AC  addi r3, r10, 0x25ac
	ctx.r[3].s64 = ctx.r[10].s64 + 9644;
	// 8260065C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260066C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260067C: 4BE667A5  bl 0x82466e20
	ctx.lr = 0x82600680;
	sub_82466E20(ctx, base);
	// 82600680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260068C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600690 size=112
    let mut pc: u32 = 0x82600690;
    'dispatch: loop {
        match pc {
            0x82600690 => {
    //   block [0x82600690..0x82600700)
	// 82600690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260069C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826006A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826006A4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826006A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826006AC: 390B56B0  addi r8, r11, 0x56b0
	ctx.r[8].s64 = ctx.r[11].s64 + 22192;
	// 826006B0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826006B4: 388ABCE4  addi r4, r10, -0x431c
	ctx.r[4].s64 = ctx.r[10].s64 + -17180;
	// 826006B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826006BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826006C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826006C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826006C8: 386A25DC  addi r3, r10, 0x25dc
	ctx.r[3].s64 = ctx.r[10].s64 + 9692;
	// 826006CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826006D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826006D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826006D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826006DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826006E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826006E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826006E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826006EC: 4BE66735  bl 0x82466e20
	ctx.lr = 0x826006F0;
	sub_82466E20(ctx, base);
	// 826006F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826006F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826006F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826006FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600700 size=112
    let mut pc: u32 = 0x82600700;
    'dispatch: loop {
        match pc {
            0x82600700 => {
    //   block [0x82600700..0x82600770)
	// 82600700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260070C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600710: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600714: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600718: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260071C: 390B5728  addi r8, r11, 0x5728
	ctx.r[8].s64 = ctx.r[11].s64 + 22312;
	// 82600720: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82600724: 388ABD04  addi r4, r10, -0x42fc
	ctx.r[4].s64 = ctx.r[10].s64 + -17148;
	// 82600728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260072C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600730: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600738: 386A260C  addi r3, r10, 0x260c
	ctx.r[3].s64 = ctx.r[10].s64 + 9740;
	// 8260073C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260074C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260075C: 4BE666C5  bl 0x82466e20
	ctx.lr = 0x82600760;
	sub_82466E20(ctx, base);
	// 82600760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260076C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600770 size=112
    let mut pc: u32 = 0x82600770;
    'dispatch: loop {
        match pc {
            0x82600770 => {
    //   block [0x82600770..0x826007E0)
	// 82600770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260077C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600780: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600784: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600788: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260078C: 390B5770  addi r8, r11, 0x5770
	ctx.r[8].s64 = ctx.r[11].s64 + 22384;
	// 82600790: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82600794: 388ABD24  addi r4, r10, -0x42dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17116;
	// 82600798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260079C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826007A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826007A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826007A8: 386A263C  addi r3, r10, 0x263c
	ctx.r[3].s64 = ctx.r[10].s64 + 9788;
	// 826007AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826007B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826007B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826007B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826007BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826007C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826007C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826007C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826007CC: 4BE66655  bl 0x82466e20
	ctx.lr = 0x826007D0;
	sub_82466E20(ctx, base);
	// 826007D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826007D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826007D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826007DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826007E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826007E0 size=112
    let mut pc: u32 = 0x826007E0;
    'dispatch: loop {
        match pc {
            0x826007E0 => {
    //   block [0x826007E0..0x82600850)
	// 826007E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826007E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826007E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826007EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826007F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826007F4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826007F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826007FC: 390B5800  addi r8, r11, 0x5800
	ctx.r[8].s64 = ctx.r[11].s64 + 22528;
	// 82600800: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82600804: 388ABD40  addi r4, r10, -0x42c0
	ctx.r[4].s64 = ctx.r[10].s64 + -17088;
	// 82600808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260080C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600810: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600818: 386A266C  addi r3, r10, 0x266c
	ctx.r[3].s64 = ctx.r[10].s64 + 9836;
	// 8260081C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260082C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260083C: 4BE665E5  bl 0x82466e20
	ctx.lr = 0x82600840;
	sub_82466E20(ctx, base);
	// 82600840: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260084C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600850 size=112
    let mut pc: u32 = 0x82600850;
    'dispatch: loop {
        match pc {
            0x82600850 => {
    //   block [0x82600850..0x826008C0)
	// 82600850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260085C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600860: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600864: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600868: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260086C: 390B5860  addi r8, r11, 0x5860
	ctx.r[8].s64 = ctx.r[11].s64 + 22624;
	// 82600870: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82600874: 388ABD58  addi r4, r10, -0x42a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17064;
	// 82600878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260087C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600888: 386A269C  addi r3, r10, 0x269c
	ctx.r[3].s64 = ctx.r[10].s64 + 9884;
	// 8260088C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260089C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826008A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826008A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826008A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826008AC: 4BE66575  bl 0x82466e20
	ctx.lr = 0x826008B0;
	sub_82466E20(ctx, base);
	// 826008B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826008B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826008B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826008BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826008C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826008C0 size=112
    let mut pc: u32 = 0x826008C0;
    'dispatch: loop {
        match pc {
            0x826008C0 => {
    //   block [0x826008C0..0x82600930)
	// 826008C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826008C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826008C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826008CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826008D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826008D4: 38AA269C  addi r5, r10, 0x269c
	ctx.r[5].s64 = ctx.r[10].s64 + 9884;
	// 826008D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826008DC: 390B58C0  addi r8, r11, 0x58c0
	ctx.r[8].s64 = ctx.r[11].s64 + 22720;
	// 826008E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826008E4: 388ABD74  addi r4, r10, -0x428c
	ctx.r[4].s64 = ctx.r[10].s64 + -17036;
	// 826008E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826008EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826008F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826008F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826008F8: 386A26CC  addi r3, r10, 0x26cc
	ctx.r[3].s64 = ctx.r[10].s64 + 9932;
	// 826008FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260090C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260091C: 4BE66505  bl 0x82466e20
	ctx.lr = 0x82600920;
	sub_82466E20(ctx, base);
	// 82600920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260092C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600930 size=112
    let mut pc: u32 = 0x82600930;
    'dispatch: loop {
        match pc {
            0x82600930 => {
    //   block [0x82600930..0x826009A0)
	// 82600930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260093C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600940: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600944: 38AA269C  addi r5, r10, 0x269c
	ctx.r[5].s64 = ctx.r[10].s64 + 9884;
	// 82600948: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260094C: 390B58F0  addi r8, r11, 0x58f0
	ctx.r[8].s64 = ctx.r[11].s64 + 22768;
	// 82600950: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82600954: 388ABD9C  addi r4, r10, -0x4264
	ctx.r[4].s64 = ctx.r[10].s64 + -16996;
	// 82600958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260095C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600968: 386A26FC  addi r3, r10, 0x26fc
	ctx.r[3].s64 = ctx.r[10].s64 + 9980;
	// 8260096C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260097C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260098C: 4BE66495  bl 0x82466e20
	ctx.lr = 0x82600990;
	sub_82466E20(ctx, base);
	// 82600990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260099C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826009A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826009A0 size=100
    let mut pc: u32 = 0x826009A0;
    'dispatch: loop {
        match pc {
            0x826009A0 => {
    //   block [0x826009A0..0x82600A04)
	// 826009A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826009A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826009A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826009AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826009B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826009B4: 38AA269C  addi r5, r10, 0x269c
	ctx.r[5].s64 = ctx.r[10].s64 + 9884;
	// 826009B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826009BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826009C0: 388ABDC4  addi r4, r10, -0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + -16956;
	// 826009C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826009C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826009CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826009D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826009D4: 386A272C  addi r3, r10, 0x272c
	ctx.r[3].s64 = ctx.r[10].s64 + 10028;
	// 826009D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826009DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826009E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826009E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826009E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826009EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826009F0: 4BE66431  bl 0x82466e20
	ctx.lr = 0x826009F4;
	sub_82466E20(ctx, base);
	// 826009F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826009F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826009FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600A08 size=112
    let mut pc: u32 = 0x82600A08;
    'dispatch: loop {
        match pc {
            0x82600A08 => {
    //   block [0x82600A08..0x82600A78)
	// 82600A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600A14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600A18: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600A1C: 38AA269C  addi r5, r10, 0x269c
	ctx.r[5].s64 = ctx.r[10].s64 + 9884;
	// 82600A20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600A24: 390B5938  addi r8, r11, 0x5938
	ctx.r[8].s64 = ctx.r[11].s64 + 22840;
	// 82600A28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82600A2C: 388ABDEC  addi r4, r10, -0x4214
	ctx.r[4].s64 = ctx.r[10].s64 + -16916;
	// 82600A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600A34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600A40: 386A275C  addi r3, r10, 0x275c
	ctx.r[3].s64 = ctx.r[10].s64 + 10076;
	// 82600A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600A64: 4BE663BD  bl 0x82466e20
	ctx.lr = 0x82600A68;
	sub_82466E20(ctx, base);
	// 82600A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600A78 size=108
    let mut pc: u32 = 0x82600A78;
    'dispatch: loop {
        match pc {
            0x82600A78 => {
    //   block [0x82600A78..0x82600AE4)
	// 82600A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600A84: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600A88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600A8C: 38EB5950  addi r7, r11, 0x5950
	ctx.r[7].s64 = ctx.r[11].s64 + 22864;
	// 82600A90: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82600A94: 388ABE14  addi r4, r10, -0x41ec
	ctx.r[4].s64 = ctx.r[10].s64 + -16876;
	// 82600A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600A9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600AA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82600AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600AA8: 386A278C  addi r3, r10, 0x278c
	ctx.r[3].s64 = ctx.r[10].s64 + 10124;
	// 82600AAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82600AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82600AD0: 4BE66351  bl 0x82466e20
	ctx.lr = 0x82600AD4;
	sub_82466E20(ctx, base);
	// 82600AD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600AE8 size=112
    let mut pc: u32 = 0x82600AE8;
    'dispatch: loop {
        match pc {
            0x82600AE8 => {
    //   block [0x82600AE8..0x82600B58)
	// 82600AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600AF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600AF8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600AFC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82600B00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600B04: 390B5998  addi r8, r11, 0x5998
	ctx.r[8].s64 = ctx.r[11].s64 + 22936;
	// 82600B08: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82600B0C: 388ABE38  addi r4, r10, -0x41c8
	ctx.r[4].s64 = ctx.r[10].s64 + -16840;
	// 82600B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600B14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600B20: 386A27BC  addi r3, r10, 0x27bc
	ctx.r[3].s64 = ctx.r[10].s64 + 10172;
	// 82600B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600B44: 4BE662DD  bl 0x82466e20
	ctx.lr = 0x82600B48;
	sub_82466E20(ctx, base);
	// 82600B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600B58 size=112
    let mut pc: u32 = 0x82600B58;
    'dispatch: loop {
        match pc {
            0x82600B58 => {
    //   block [0x82600B58..0x82600BC8)
	// 82600B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600B64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600B68: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600B6C: 38AA27BC  addi r5, r10, 0x27bc
	ctx.r[5].s64 = ctx.r[10].s64 + 10172;
	// 82600B70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600B74: 390B59F8  addi r8, r11, 0x59f8
	ctx.r[8].s64 = ctx.r[11].s64 + 23032;
	// 82600B78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82600B7C: 388ABE44  addi r4, r10, -0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16828;
	// 82600B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600B84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600B90: 386A27EC  addi r3, r10, 0x27ec
	ctx.r[3].s64 = ctx.r[10].s64 + 10220;
	// 82600B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600BB4: 4BE6626D  bl 0x82466e20
	ctx.lr = 0x82600BB8;
	sub_82466E20(ctx, base);
	// 82600BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600BC8 size=112
    let mut pc: u32 = 0x82600BC8;
    'dispatch: loop {
        match pc {
            0x82600BC8 => {
    //   block [0x82600BC8..0x82600C38)
	// 82600BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600BD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600BD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600BDC: 38AA27BC  addi r5, r10, 0x27bc
	ctx.r[5].s64 = ctx.r[10].s64 + 10172;
	// 82600BE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600BE4: 390B5A10  addi r8, r11, 0x5a10
	ctx.r[8].s64 = ctx.r[11].s64 + 23056;
	// 82600BE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82600BEC: 388ABE54  addi r4, r10, -0x41ac
	ctx.r[4].s64 = ctx.r[10].s64 + -16812;
	// 82600BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600BF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600C00: 386A281C  addi r3, r10, 0x281c
	ctx.r[3].s64 = ctx.r[10].s64 + 10268;
	// 82600C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600C24: 4BE661FD  bl 0x82466e20
	ctx.lr = 0x82600C28;
	sub_82466E20(ctx, base);
	// 82600C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600C38 size=112
    let mut pc: u32 = 0x82600C38;
    'dispatch: loop {
        match pc {
            0x82600C38 => {
    //   block [0x82600C38..0x82600CA8)
	// 82600C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600C44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600C48: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600C4C: 38AA27BC  addi r5, r10, 0x27bc
	ctx.r[5].s64 = ctx.r[10].s64 + 10172;
	// 82600C50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600C54: 390B5A40  addi r8, r11, 0x5a40
	ctx.r[8].s64 = ctx.r[11].s64 + 23104;
	// 82600C58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82600C5C: 388ABE64  addi r4, r10, -0x419c
	ctx.r[4].s64 = ctx.r[10].s64 + -16796;
	// 82600C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600C64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600C70: 386A284C  addi r3, r10, 0x284c
	ctx.r[3].s64 = ctx.r[10].s64 + 10316;
	// 82600C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600C94: 4BE6618D  bl 0x82466e20
	ctx.lr = 0x82600C98;
	sub_82466E20(ctx, base);
	// 82600C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82600CA8 size=24
    let mut pc: u32 = 0x82600CA8;
    'dispatch: loop {
        match pc {
            0x82600CA8 => {
    //   block [0x82600CA8..0x82600CC0)
	// 82600CA8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600CAC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82600CB0: 394AB1C0  addi r10, r10, -0x4e40
	ctx.r[10].s64 = ctx.r[10].s64 + -20032;
	// 82600CB4: 816B5A58  lwz r11, 0x5a58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23128 as u32) ) } as u64;
	// 82600CB8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82600CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600CC0 size=112
    let mut pc: u32 = 0x82600CC0;
    'dispatch: loop {
        match pc {
            0x82600CC0 => {
    //   block [0x82600CC0..0x82600D30)
	// 82600CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600CCC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600CD0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82600CD4: 392A9400  addi r9, r10, -0x6c00
	ctx.r[9].s64 = ctx.r[10].s64 + -27648;
	// 82600CD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600CDC: 390BB1C0  addi r8, r11, -0x4e40
	ctx.r[8].s64 = ctx.r[11].s64 + -20032;
	// 82600CE0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82600CE4: 388ABE74  addi r4, r10, -0x418c
	ctx.r[4].s64 = ctx.r[10].s64 + -16780;
	// 82600CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600CEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600CF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600CF8: 386A287C  addi r3, r10, 0x287c
	ctx.r[3].s64 = ctx.r[10].s64 + 10364;
	// 82600CFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82600D00: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82600D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600D14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82600D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600D1C: 4BE66105  bl 0x82466e20
	ctx.lr = 0x82600D20;
	sub_82466E20(ctx, base);
	// 82600D20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600D30 size=108
    let mut pc: u32 = 0x82600D30;
    'dispatch: loop {
        match pc {
            0x82600D30 => {
    //   block [0x82600D30..0x82600D9C)
	// 82600D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600D3C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600D40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600D44: 38EB5A5C  addi r7, r11, 0x5a5c
	ctx.r[7].s64 = ctx.r[11].s64 + 23132;
	// 82600D48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82600D4C: 388ABE80  addi r4, r10, -0x4180
	ctx.r[4].s64 = ctx.r[10].s64 + -16768;
	// 82600D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600D54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600D58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82600D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600D60: 386A28AC  addi r3, r10, 0x28ac
	ctx.r[3].s64 = ctx.r[10].s64 + 10412;
	// 82600D64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82600D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600D6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82600D88: 4BE66099  bl 0x82466e20
	ctx.lr = 0x82600D8C;
	sub_82466E20(ctx, base);
	// 82600D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600DA0 size=108
    let mut pc: u32 = 0x82600DA0;
    'dispatch: loop {
        match pc {
            0x82600DA0 => {
    //   block [0x82600DA0..0x82600E0C)
	// 82600DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600DAC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600DB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600DB4: 38EB5A78  addi r7, r11, 0x5a78
	ctx.r[7].s64 = ctx.r[11].s64 + 23160;
	// 82600DB8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82600DBC: 388ABE94  addi r4, r10, -0x416c
	ctx.r[4].s64 = ctx.r[10].s64 + -16748;
	// 82600DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600DC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600DC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82600DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600DD0: 386A28DC  addi r3, r10, 0x28dc
	ctx.r[3].s64 = ctx.r[10].s64 + 10460;
	// 82600DD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82600DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600DF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82600DF8: 4BE66029  bl 0x82466e20
	ctx.lr = 0x82600DFC;
	sub_82466E20(ctx, base);
	// 82600DFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600E10 size=116
    let mut pc: u32 = 0x82600E10;
    'dispatch: loop {
        match pc {
            0x82600E10 => {
    //   block [0x82600E10..0x82600E84)
	// 82600E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600E1C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600E20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600E24: 390B5AC0  addi r8, r11, 0x5ac0
	ctx.r[8].s64 = ctx.r[11].s64 + 23232;
	// 82600E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600E2C: 392A94C0  addi r9, r10, -0x6b40
	ctx.r[9].s64 = ctx.r[10].s64 + -27456;
	// 82600E30: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600E34: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82600E38: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82600E3C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600E44: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600E54: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82600E58: 388ABEA0  addi r4, r10, -0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + -16736;
	// 82600E5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82600E60: 386B290C  addi r3, r11, 0x290c
	ctx.r[3].s64 = ctx.r[11].s64 + 10508;
	// 82600E64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82600E68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600E70: 4BE65FB1  bl 0x82466e20
	ctx.lr = 0x82600E74;
	sub_82466E20(ctx, base);
	// 82600E74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82600E88 size=24
    let mut pc: u32 = 0x82600E88;
    'dispatch: loop {
        match pc {
            0x82600E88 => {
    //   block [0x82600E88..0x82600EA0)
	// 82600E88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600E8C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82600E90: 394AB208  addi r10, r10, -0x4df8
	ctx.r[10].s64 = ctx.r[10].s64 + -19960;
	// 82600E94: 816B5AD8  lwz r11, 0x5ad8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23256 as u32) ) } as u64;
	// 82600E98: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82600E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600EA0 size=116
    let mut pc: u32 = 0x82600EA0;
    'dispatch: loop {
        match pc {
            0x82600EA0 => {
    //   block [0x82600EA0..0x82600F14)
	// 82600EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600EAC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82600EB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600EB4: 390BB208  addi r8, r11, -0x4df8
	ctx.r[8].s64 = ctx.r[11].s64 + -19960;
	// 82600EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600EBC: 392A9530  addi r9, r10, -0x6ad0
	ctx.r[9].s64 = ctx.r[10].s64 + -27344;
	// 82600EC0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600EC4: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82600EC8: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82600ECC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600ED4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600EE4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82600EE8: 388ABEB4  addi r4, r10, -0x414c
	ctx.r[4].s64 = ctx.r[10].s64 + -16716;
	// 82600EEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82600EF0: 386B293C  addi r3, r11, 0x293c
	ctx.r[3].s64 = ctx.r[11].s64 + 10556;
	// 82600EF4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82600EF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600EFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600F00: 4BE65F21  bl 0x82466e20
	ctx.lr = 0x82600F04;
	sub_82466E20(ctx, base);
	// 82600F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600F18 size=108
    let mut pc: u32 = 0x82600F18;
    'dispatch: loop {
        match pc {
            0x82600F18 => {
    //   block [0x82600F18..0x82600F84)
	// 82600F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600F24: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600F28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600F2C: 38EB5AE8  addi r7, r11, 0x5ae8
	ctx.r[7].s64 = ctx.r[11].s64 + 23272;
	// 82600F30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82600F34: 388ABECC  addi r4, r10, -0x4134
	ctx.r[4].s64 = ctx.r[10].s64 + -16692;
	// 82600F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600F3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600F40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82600F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600F48: 386A296C  addi r3, r10, 0x296c
	ctx.r[3].s64 = ctx.r[10].s64 + 10604;
	// 82600F4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82600F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82600F70: 4BE65EB1  bl 0x82466e20
	ctx.lr = 0x82600F74;
	sub_82466E20(ctx, base);
	// 82600F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600F88 size=112
    let mut pc: u32 = 0x82600F88;
    'dispatch: loop {
        match pc {
            0x82600F88 => {
    //   block [0x82600F88..0x82600FF8)
	// 82600F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600F94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600F98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600F9C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82600FA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600FA4: 390B5B18  addi r8, r11, 0x5b18
	ctx.r[8].s64 = ctx.r[11].s64 + 23320;
	// 82600FA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82600FAC: 388ABEF0  addi r4, r10, -0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + -16656;
	// 82600FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600FB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600FB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600FC0: 386A299C  addi r3, r10, 0x299c
	ctx.r[3].s64 = ctx.r[10].s64 + 10652;
	// 82600FC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600FE4: 4BE65E3D  bl 0x82466e20
	ctx.lr = 0x82600FE8;
	sub_82466E20(ctx, base);
	// 82600FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600FF8 size=112
    let mut pc: u32 = 0x82600FF8;
    'dispatch: loop {
        match pc {
            0x82600FF8 => {
    //   block [0x82600FF8..0x82601068)
	// 82600FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601004: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601008: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260100C: 392A9588  addi r9, r10, -0x6a78
	ctx.r[9].s64 = ctx.r[10].s64 + -27256;
	// 82601010: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601014: 390B5B38  addi r8, r11, 0x5b38
	ctx.r[8].s64 = ctx.r[11].s64 + 23352;
	// 82601018: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8260101C: 388ABF10  addi r4, r10, -0x40f0
	ctx.r[4].s64 = ctx.r[10].s64 + -16624;
	// 82601020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601024: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260102C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601030: 386A29CC  addi r3, r10, 0x29cc
	ctx.r[3].s64 = ctx.r[10].s64 + 10700;
	// 82601034: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82601038: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260103C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260104C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601054: 4BE65DCD  bl 0x82466e20
	ctx.lr = 0x82601058;
	sub_82466E20(ctx, base);
	// 82601058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260105C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601068 size=112
    let mut pc: u32 = 0x82601068;
    'dispatch: loop {
        match pc {
            0x82601068 => {
    //   block [0x82601068..0x826010D8)
	// 82601068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260106C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601074: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601078: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260107C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601084: 390B5B80  addi r8, r11, 0x5b80
	ctx.r[8].s64 = ctx.r[11].s64 + 23424;
	// 82601088: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260108C: 388ABF2C  addi r4, r10, -0x40d4
	ctx.r[4].s64 = ctx.r[10].s64 + -16596;
	// 82601090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601094: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260109C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826010A0: 386A29FC  addi r3, r10, 0x29fc
	ctx.r[3].s64 = ctx.r[10].s64 + 10748;
	// 826010A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826010A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826010AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826010B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826010B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826010B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826010BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826010C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826010C4: 4BE65D5D  bl 0x82466e20
	ctx.lr = 0x826010C8;
	sub_82466E20(ctx, base);
	// 826010C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826010CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826010D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826010D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826010D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826010D8 size=112
    let mut pc: u32 = 0x826010D8;
    'dispatch: loop {
        match pc {
            0x826010D8 => {
    //   block [0x826010D8..0x82601148)
	// 826010D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826010DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826010E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826010E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826010E8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826010EC: 392A95B4  addi r9, r10, -0x6a4c
	ctx.r[9].s64 = ctx.r[10].s64 + -27212;
	// 826010F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826010F4: 390B5B98  addi r8, r11, 0x5b98
	ctx.r[8].s64 = ctx.r[11].s64 + 23448;
	// 826010F8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826010FC: 388ABF44  addi r4, r10, -0x40bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16572;
	// 82601100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601104: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260110C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601110: 386A2A2C  addi r3, r10, 0x2a2c
	ctx.r[3].s64 = ctx.r[10].s64 + 10796;
	// 82601114: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82601118: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260111C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260112C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601134: 4BE65CED  bl 0x82466e20
	ctx.lr = 0x82601138;
	sub_82466E20(ctx, base);
	// 82601138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260113C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601148 size=112
    let mut pc: u32 = 0x82601148;
    'dispatch: loop {
        match pc {
            0x82601148 => {
    //   block [0x82601148..0x826011B8)
	// 82601148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260114C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601154: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601158: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260115C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601160: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601164: 390B5C28  addi r8, r11, 0x5c28
	ctx.r[8].s64 = ctx.r[11].s64 + 23592;
	// 82601168: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260116C: 388ABF68  addi r4, r10, -0x4098
	ctx.r[4].s64 = ctx.r[10].s64 + -16536;
	// 82601170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601174: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601178: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260117C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601180: 386A2A5C  addi r3, r10, 0x2a5c
	ctx.r[3].s64 = ctx.r[10].s64 + 10844;
	// 82601184: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260118C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260119C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826011A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826011A4: 4BE65C7D  bl 0x82466e20
	ctx.lr = 0x826011A8;
	sub_82466E20(ctx, base);
	// 826011A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826011AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826011B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826011B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826011B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826011B8 size=112
    let mut pc: u32 = 0x826011B8;
    'dispatch: loop {
        match pc {
            0x826011B8 => {
    //   block [0x826011B8..0x82601228)
	// 826011B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826011BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826011C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826011C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826011C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826011CC: 38AA2ABC  addi r5, r10, 0x2abc
	ctx.r[5].s64 = ctx.r[10].s64 + 10940;
	// 826011D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826011D4: 390B5C40  addi r8, r11, 0x5c40
	ctx.r[8].s64 = ctx.r[11].s64 + 23616;
	// 826011D8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826011DC: 388ABF88  addi r4, r10, -0x4078
	ctx.r[4].s64 = ctx.r[10].s64 + -16504;
	// 826011E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826011E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826011E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826011EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826011F0: 386A2A8C  addi r3, r10, 0x2a8c
	ctx.r[3].s64 = ctx.r[10].s64 + 10892;
	// 826011F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826011F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826011FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260120C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601214: 4BE65C0D  bl 0x82466e20
	ctx.lr = 0x82601218;
	sub_82466E20(ctx, base);
	// 82601218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260121C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601228 size=100
    let mut pc: u32 = 0x82601228;
    'dispatch: loop {
        match pc {
            0x82601228 => {
    //   block [0x82601228..0x8260128C)
	// 82601228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260122C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260123C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82601240: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601248: 388ABFA4  addi r4, r10, -0x405c
	ctx.r[4].s64 = ctx.r[10].s64 + -16476;
	// 8260124C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260125C: 386A2ABC  addi r3, r10, 0x2abc
	ctx.r[3].s64 = ctx.r[10].s64 + 10940;
	// 82601260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601264: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601268: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260126C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601270: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82601274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601278: 4BE65BA9  bl 0x82466e20
	ctx.lr = 0x8260127C;
	sub_82466E20(ctx, base);
	// 8260127C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82601290 size=24
    let mut pc: u32 = 0x82601290;
    'dispatch: loop {
        match pc {
            0x82601290 => {
    //   block [0x82601290..0x826012A8)
	// 82601290: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601294: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82601298: 394AB2E0  addi r10, r10, -0x4d20
	ctx.r[10].s64 = ctx.r[10].s64 + -19744;
	// 8260129C: 816B5CB8  lwz r11, 0x5cb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23736 as u32) ) } as u64;
	// 826012A0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826012A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826012A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826012A8 size=116
    let mut pc: u32 = 0x826012A8;
    'dispatch: loop {
        match pc {
            0x826012A8 => {
    //   block [0x826012A8..0x8260131C)
	// 826012A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826012AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826012B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826012B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826012B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826012BC: 390BB2E0  addi r8, r11, -0x4d20
	ctx.r[8].s64 = ctx.r[11].s64 + -19744;
	// 826012C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826012C4: 392A95F0  addi r9, r10, -0x6a10
	ctx.r[9].s64 = ctx.r[10].s64 + -27152;
	// 826012C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826012CC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826012D0: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 826012D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826012D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826012DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826012E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826012E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826012E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826012EC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 826012F0: 388ABFB8  addi r4, r10, -0x4048
	ctx.r[4].s64 = ctx.r[10].s64 + -16456;
	// 826012F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826012F8: 386B2AEC  addi r3, r11, 0x2aec
	ctx.r[3].s64 = ctx.r[11].s64 + 10988;
	// 826012FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82601300: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601308: 4BE65B19  bl 0x82466e20
	ctx.lr = 0x8260130C;
	sub_82466E20(ctx, base);
	// 8260130C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601320 size=108
    let mut pc: u32 = 0x82601320;
    'dispatch: loop {
        match pc {
            0x82601320 => {
    //   block [0x82601320..0x8260138C)
	// 82601320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260132C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601330: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601334: 38EB5CBC  addi r7, r11, 0x5cbc
	ctx.r[7].s64 = ctx.r[11].s64 + 23740;
	// 82601338: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260133C: 388ABFD8  addi r4, r10, -0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + -16424;
	// 82601340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601344: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601348: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260134C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601350: 386A2B1C  addi r3, r10, 0x2b1c
	ctx.r[3].s64 = ctx.r[10].s64 + 11036;
	// 82601354: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82601358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260135C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260136C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601378: 4BE65AA9  bl 0x82466e20
	ctx.lr = 0x8260137C;
	sub_82466E20(ctx, base);
	// 8260137C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601390 size=112
    let mut pc: u32 = 0x82601390;
    'dispatch: loop {
        match pc {
            0x82601390 => {
    //   block [0x82601390..0x82601400)
	// 82601390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260139C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826013A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826013A4: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 826013A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826013AC: 390B5CEC  addi r8, r11, 0x5cec
	ctx.r[8].s64 = ctx.r[11].s64 + 23788;
	// 826013B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826013B4: 388ABFFC  addi r4, r10, -0x4004
	ctx.r[4].s64 = ctx.r[10].s64 + -16388;
	// 826013B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826013BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826013C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826013C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826013C8: 386A2B4C  addi r3, r10, 0x2b4c
	ctx.r[3].s64 = ctx.r[10].s64 + 11084;
	// 826013CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826013D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826013D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826013D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826013DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826013E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826013E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826013E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826013EC: 4BE65A35  bl 0x82466e20
	ctx.lr = 0x826013F0;
	sub_82466E20(ctx, base);
	// 826013F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826013F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826013F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826013FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601400 size=112
    let mut pc: u32 = 0x82601400;
    'dispatch: loop {
        match pc {
            0x82601400 => {
    //   block [0x82601400..0x82601470)
	// 82601400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260140C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601410: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601414: 392A9614  addi r9, r10, -0x69ec
	ctx.r[9].s64 = ctx.r[10].s64 + -27116;
	// 82601418: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260141C: 390B5D08  addi r8, r11, 0x5d08
	ctx.r[8].s64 = ctx.r[11].s64 + 23816;
	// 82601420: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82601424: 388AC01C  addi r4, r10, -0x3fe4
	ctx.r[4].s64 = ctx.r[10].s64 + -16356;
	// 82601428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260142C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601438: 386A2B7C  addi r3, r10, 0x2b7c
	ctx.r[3].s64 = ctx.r[10].s64 + 11132;
	// 8260143C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82601440: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82601444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260144C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260145C: 4BE659C5  bl 0x82466e20
	ctx.lr = 0x82601460;
	sub_82466E20(ctx, base);
	// 82601460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260146C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601470 size=112
    let mut pc: u32 = 0x82601470;
    'dispatch: loop {
        match pc {
            0x82601470 => {
    //   block [0x82601470..0x826014E0)
	// 82601470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260147C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601480: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601484: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601488: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260148C: 390B5DB0  addi r8, r11, 0x5db0
	ctx.r[8].s64 = ctx.r[11].s64 + 23984;
	// 82601490: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82601494: 388AC03C  addi r4, r10, -0x3fc4
	ctx.r[4].s64 = ctx.r[10].s64 + -16324;
	// 82601498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260149C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826014A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826014A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826014A8: 386A2BAC  addi r3, r10, 0x2bac
	ctx.r[3].s64 = ctx.r[10].s64 + 11180;
	// 826014AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826014B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826014B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826014B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826014BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826014C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826014C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826014C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826014CC: 4BE65955  bl 0x82466e20
	ctx.lr = 0x826014D0;
	sub_82466E20(ctx, base);
	// 826014D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826014D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826014D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826014DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826014E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826014E0 size=112
    let mut pc: u32 = 0x826014E0;
    'dispatch: loop {
        match pc {
            0x826014E0 => {
    //   block [0x826014E0..0x82601550)
	// 826014E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826014E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826014E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826014EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826014F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826014F4: 392A966C  addi r9, r10, -0x6994
	ctx.r[9].s64 = ctx.r[10].s64 + -27028;
	// 826014F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826014FC: 390B5DD0  addi r8, r11, 0x5dd0
	ctx.r[8].s64 = ctx.r[11].s64 + 24016;
	// 82601500: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82601504: 388AC058  addi r4, r10, -0x3fa8
	ctx.r[4].s64 = ctx.r[10].s64 + -16296;
	// 82601508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260150C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601518: 386A2BDC  addi r3, r10, 0x2bdc
	ctx.r[3].s64 = ctx.r[10].s64 + 11228;
	// 8260151C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82601520: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82601524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260152C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260153C: 4BE658E5  bl 0x82466e20
	ctx.lr = 0x82601540;
	sub_82466E20(ctx, base);
	// 82601540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260154C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601550 size=116
    let mut pc: u32 = 0x82601550;
    'dispatch: loop {
        match pc {
            0x82601550 => {
    //   block [0x82601550..0x826015C4)
	// 82601550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260155C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601560: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601564: 390B5E78  addi r8, r11, 0x5e78
	ctx.r[8].s64 = ctx.r[11].s64 + 24184;
	// 82601568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260156C: 392A9640  addi r9, r10, -0x69c0
	ctx.r[9].s64 = ctx.r[10].s64 + -27072;
	// 82601570: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601574: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82601578: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 8260157C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601584: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260158C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601594: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82601598: 388AC078  addi r4, r10, -0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + -16264;
	// 8260159C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826015A0: 386B2C0C  addi r3, r11, 0x2c0c
	ctx.r[3].s64 = ctx.r[11].s64 + 11276;
	// 826015A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826015A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826015AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826015B0: 4BE65871  bl 0x82466e20
	ctx.lr = 0x826015B4;
	sub_82466E20(ctx, base);
	// 826015B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826015B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826015BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826015C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826015C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826015C8 size=108
    let mut pc: u32 = 0x826015C8;
    'dispatch: loop {
        match pc {
            0x826015C8 => {
    //   block [0x826015C8..0x82601634)
	// 826015C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826015CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826015D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826015D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826015D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826015DC: 38EB5E90  addi r7, r11, 0x5e90
	ctx.r[7].s64 = ctx.r[11].s64 + 24208;
	// 826015E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826015E4: 388AC094  addi r4, r10, -0x3f6c
	ctx.r[4].s64 = ctx.r[10].s64 + -16236;
	// 826015E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826015EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826015F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826015F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826015F8: 386A2C3C  addi r3, r10, 0x2c3c
	ctx.r[3].s64 = ctx.r[10].s64 + 11324;
	// 826015FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82601600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260160C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260161C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601620: 4BE65801  bl 0x82466e20
	ctx.lr = 0x82601624;
	sub_82466E20(ctx, base);
	// 82601624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260162C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601638 size=112
    let mut pc: u32 = 0x82601638;
    'dispatch: loop {
        match pc {
            0x82601638 => {
    //   block [0x82601638..0x826016A8)
	// 82601638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260163C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601644: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601648: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260164C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601650: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601654: 390B5EC0  addi r8, r11, 0x5ec0
	ctx.r[8].s64 = ctx.r[11].s64 + 24256;
	// 82601658: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260165C: 388AC0B8  addi r4, r10, -0x3f48
	ctx.r[4].s64 = ctx.r[10].s64 + -16200;
	// 82601660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601664: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601668: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260166C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601670: 386A2C6C  addi r3, r10, 0x2c6c
	ctx.r[3].s64 = ctx.r[10].s64 + 11372;
	// 82601674: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260167C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260168C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601694: 4BE6578D  bl 0x82466e20
	ctx.lr = 0x82601698;
	sub_82466E20(ctx, base);
	// 82601698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260169C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826016A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826016A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826016A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826016A8 size=112
    let mut pc: u32 = 0x826016A8;
    'dispatch: loop {
        match pc {
            0x826016A8 => {
    //   block [0x826016A8..0x82601718)
	// 826016A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826016AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826016B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826016B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826016B8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826016BC: 392A96A0  addi r9, r10, -0x6960
	ctx.r[9].s64 = ctx.r[10].s64 + -26976;
	// 826016C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826016C4: 390B5EE0  addi r8, r11, 0x5ee0
	ctx.r[8].s64 = ctx.r[11].s64 + 24288;
	// 826016C8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826016CC: 388AC0D8  addi r4, r10, -0x3f28
	ctx.r[4].s64 = ctx.r[10].s64 + -16168;
	// 826016D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826016D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826016D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826016DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826016E0: 386A2C9C  addi r3, r10, 0x2c9c
	ctx.r[3].s64 = ctx.r[10].s64 + 11420;
	// 826016E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826016E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826016EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826016F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826016F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826016F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826016FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601704: 4BE6571D  bl 0x82466e20
	ctx.lr = 0x82601708;
	sub_82466E20(ctx, base);
	// 82601708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260170C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601718 size=112
    let mut pc: u32 = 0x82601718;
    'dispatch: loop {
        match pc {
            0x82601718 => {
    //   block [0x82601718..0x82601788)
	// 82601718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260171C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601724: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601728: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260172C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601730: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601734: 390B5F88  addi r8, r11, 0x5f88
	ctx.r[8].s64 = ctx.r[11].s64 + 24456;
	// 82601738: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260173C: 388AC0F4  addi r4, r10, -0x3f0c
	ctx.r[4].s64 = ctx.r[10].s64 + -16140;
	// 82601740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260174C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601750: 386A2CCC  addi r3, r10, 0x2ccc
	ctx.r[3].s64 = ctx.r[10].s64 + 11468;
	// 82601754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260175C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260176C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601774: 4BE656AD  bl 0x82466e20
	ctx.lr = 0x82601778;
	sub_82466E20(ctx, base);
	// 82601778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260177C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601788 size=108
    let mut pc: u32 = 0x82601788;
    'dispatch: loop {
        match pc {
            0x82601788 => {
    //   block [0x82601788..0x826017F4)
	// 82601788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260178C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601794: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601798: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260179C: 38EB5FD0  addi r7, r11, 0x5fd0
	ctx.r[7].s64 = ctx.r[11].s64 + 24528;
	// 826017A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826017A4: 388AC10C  addi r4, r10, -0x3ef4
	ctx.r[4].s64 = ctx.r[10].s64 + -16116;
	// 826017A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826017AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826017B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826017B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826017B8: 386A2CFC  addi r3, r10, 0x2cfc
	ctx.r[3].s64 = ctx.r[10].s64 + 11516;
	// 826017BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826017C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826017C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826017C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826017CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826017D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826017D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826017D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826017DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826017E0: 4BE65641  bl 0x82466e20
	ctx.lr = 0x826017E4;
	sub_82466E20(ctx, base);
	// 826017E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826017E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826017EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826017F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826017F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826017F8 size=112
    let mut pc: u32 = 0x826017F8;
    'dispatch: loop {
        match pc {
            0x826017F8 => {
    //   block [0x826017F8..0x82601868)
	// 826017F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826017FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601804: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601808: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260180C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601810: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601814: 390B6000  addi r8, r11, 0x6000
	ctx.r[8].s64 = ctx.r[11].s64 + 24576;
	// 82601818: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260181C: 388AC130  addi r4, r10, -0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -16080;
	// 82601820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601824: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601828: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260182C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601830: 386A2D2C  addi r3, r10, 0x2d2c
	ctx.r[3].s64 = ctx.r[10].s64 + 11564;
	// 82601834: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260183C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260184C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601854: 4BE655CD  bl 0x82466e20
	ctx.lr = 0x82601858;
	sub_82466E20(ctx, base);
	// 82601858: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260185C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601868 size=112
    let mut pc: u32 = 0x82601868;
    'dispatch: loop {
        match pc {
            0x82601868 => {
    //   block [0x82601868..0x826018D8)
	// 82601868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260186C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601874: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601878: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260187C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601880: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601884: 390B6018  addi r8, r11, 0x6018
	ctx.r[8].s64 = ctx.r[11].s64 + 24600;
	// 82601888: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8260188C: 388AC14C  addi r4, r10, -0x3eb4
	ctx.r[4].s64 = ctx.r[10].s64 + -16052;
	// 82601890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601894: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260189C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826018A0: 386A2D5C  addi r3, r10, 0x2d5c
	ctx.r[3].s64 = ctx.r[10].s64 + 11612;
	// 826018A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826018A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826018AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826018B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826018B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826018B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826018BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826018C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826018C4: 4BE6555D  bl 0x82466e20
	ctx.lr = 0x826018C8;
	sub_82466E20(ctx, base);
	// 826018C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826018CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826018D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826018D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826018D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826018D8 size=100
    let mut pc: u32 = 0x826018D8;
    'dispatch: loop {
        match pc {
            0x826018D8 => {
    //   block [0x826018D8..0x8260193C)
	// 826018D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826018DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826018E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826018E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826018E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826018EC: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 826018F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826018F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826018F8: 388AC168  addi r4, r10, -0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + -16024;
	// 826018FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260190C: 386A2D8C  addi r3, r10, 0x2d8c
	ctx.r[3].s64 = ctx.r[10].s64 + 11660;
	// 82601910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601918: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260191C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601920: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82601924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601928: 4BE654F9  bl 0x82466e20
	ctx.lr = 0x8260192C;
	sub_82466E20(ctx, base);
	// 8260192C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601940 size=112
    let mut pc: u32 = 0x82601940;
    'dispatch: loop {
        match pc {
            0x82601940 => {
    //   block [0x82601940..0x826019B0)
	// 82601940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260194C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601950: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601954: 38AA293C  addi r5, r10, 0x293c
	ctx.r[5].s64 = ctx.r[10].s64 + 10556;
	// 82601958: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260195C: 390B60D8  addi r8, r11, 0x60d8
	ctx.r[8].s64 = ctx.r[11].s64 + 24792;
	// 82601960: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82601964: 388AC180  addi r4, r10, -0x3e80
	ctx.r[4].s64 = ctx.r[10].s64 + -16000;
	// 82601968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260196C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601978: 386A2DBC  addi r3, r10, 0x2dbc
	ctx.r[3].s64 = ctx.r[10].s64 + 11708;
	// 8260197C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260198C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260199C: 4BE65485  bl 0x82466e20
	ctx.lr = 0x826019A0;
	sub_82466E20(ctx, base);
	// 826019A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826019A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826019A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826019AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826019B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826019B0 size=112
    let mut pc: u32 = 0x826019B0;
    'dispatch: loop {
        match pc {
            0x826019B0 => {
    //   block [0x826019B0..0x82601A20)
	// 826019B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826019B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826019B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826019BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826019C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826019C4: 38AA27BC  addi r5, r10, 0x27bc
	ctx.r[5].s64 = ctx.r[10].s64 + 10172;
	// 826019C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826019CC: 390B6108  addi r8, r11, 0x6108
	ctx.r[8].s64 = ctx.r[11].s64 + 24840;
	// 826019D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826019D4: 388AC19C  addi r4, r10, -0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + -15972;
	// 826019D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826019DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826019E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826019E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826019E8: 386A2DEC  addi r3, r10, 0x2dec
	ctx.r[3].s64 = ctx.r[10].s64 + 11756;
	// 826019EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826019F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826019F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826019F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826019FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601A0C: 4BE65415  bl 0x82466e20
	ctx.lr = 0x82601A10;
	sub_82466E20(ctx, base);
	// 82601A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601A20 size=108
    let mut pc: u32 = 0x82601A20;
    'dispatch: loop {
        match pc {
            0x82601A20 => {
    //   block [0x82601A20..0x82601A8C)
	// 82601A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601A2C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601A30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601A34: 38EB6120  addi r7, r11, 0x6120
	ctx.r[7].s64 = ctx.r[11].s64 + 24864;
	// 82601A38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82601A3C: 388AC1C0  addi r4, r10, -0x3e40
	ctx.r[4].s64 = ctx.r[10].s64 + -15936;
	// 82601A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601A44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601A48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82601A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601A50: 386A2E1C  addi r3, r10, 0x2e1c
	ctx.r[3].s64 = ctx.r[10].s64 + 11804;
	// 82601A54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82601A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601A74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601A78: 4BE653A9  bl 0x82466e20
	ctx.lr = 0x82601A7C;
	sub_82466E20(ctx, base);
	// 82601A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601A90 size=112
    let mut pc: u32 = 0x82601A90;
    'dispatch: loop {
        match pc {
            0x82601A90 => {
    //   block [0x82601A90..0x82601B00)
	// 82601A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601A9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601AA0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601AA4: 38AA2D8C  addi r5, r10, 0x2d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 11660;
	// 82601AA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601AAC: 390B6150  addi r8, r11, 0x6150
	ctx.r[8].s64 = ctx.r[11].s64 + 24912;
	// 82601AB0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82601AB4: 388AC1E8  addi r4, r10, -0x3e18
	ctx.r[4].s64 = ctx.r[10].s64 + -15896;
	// 82601AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601ABC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601AC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601AC8: 386A2E4C  addi r3, r10, 0x2e4c
	ctx.r[3].s64 = ctx.r[10].s64 + 11852;
	// 82601ACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601AD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601AD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601AE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601AE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601AE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601AEC: 4BE65335  bl 0x82466e20
	ctx.lr = 0x82601AF0;
	sub_82466E20(ctx, base);
	// 82601AF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601B00 size=112
    let mut pc: u32 = 0x82601B00;
    'dispatch: loop {
        match pc {
            0x82601B00 => {
    //   block [0x82601B00..0x82601B70)
	// 82601B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601B0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601B10: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601B14: 392A96CC  addi r9, r10, -0x6934
	ctx.r[9].s64 = ctx.r[10].s64 + -26932;
	// 82601B18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601B1C: 390B61E0  addi r8, r11, 0x61e0
	ctx.r[8].s64 = ctx.r[11].s64 + 25056;
	// 82601B20: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82601B24: 388AC200  addi r4, r10, -0x3e00
	ctx.r[4].s64 = ctx.r[10].s64 + -15872;
	// 82601B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601B2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601B30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601B38: 386A2E7C  addi r3, r10, 0x2e7c
	ctx.r[3].s64 = ctx.r[10].s64 + 11900;
	// 82601B3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82601B40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82601B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601B54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601B5C: 4BE652C5  bl 0x82466e20
	ctx.lr = 0x82601B60;
	sub_82466E20(ctx, base);
	// 82601B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601B70 size=112
    let mut pc: u32 = 0x82601B70;
    'dispatch: loop {
        match pc {
            0x82601B70 => {
    //   block [0x82601B70..0x82601BE0)
	// 82601B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601B7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601B80: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601B84: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601B88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601B8C: 390B6228  addi r8, r11, 0x6228
	ctx.r[8].s64 = ctx.r[11].s64 + 25128;
	// 82601B90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82601B94: 388AC218  addi r4, r10, -0x3de8
	ctx.r[4].s64 = ctx.r[10].s64 + -15848;
	// 82601B98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601B9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601BA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601BA8: 386A2EAC  addi r3, r10, 0x2eac
	ctx.r[3].s64 = ctx.r[10].s64 + 11948;
	// 82601BAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601BB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601BC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601BC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601BCC: 4BE65255  bl 0x82466e20
	ctx.lr = 0x82601BD0;
	sub_82466E20(ctx, base);
	// 82601BD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601BE0 size=108
    let mut pc: u32 = 0x82601BE0;
    'dispatch: loop {
        match pc {
            0x82601BE0 => {
    //   block [0x82601BE0..0x82601C4C)
	// 82601BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601BEC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601BF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601BF4: 38EB6240  addi r7, r11, 0x6240
	ctx.r[7].s64 = ctx.r[11].s64 + 25152;
	// 82601BF8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82601BFC: 388AC22C  addi r4, r10, -0x3dd4
	ctx.r[4].s64 = ctx.r[10].s64 + -15828;
	// 82601C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601C04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601C08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82601C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601C10: 386A2EDC  addi r3, r10, 0x2edc
	ctx.r[3].s64 = ctx.r[10].s64 + 11996;
	// 82601C14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82601C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601C34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601C38: 4BE651E9  bl 0x82466e20
	ctx.lr = 0x82601C3C;
	sub_82466E20(ctx, base);
	// 82601C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601C50 size=116
    let mut pc: u32 = 0x82601C50;
    'dispatch: loop {
        match pc {
            0x82601C50 => {
    //   block [0x82601C50..0x82601CC4)
	// 82601C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601C5C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 82601C60: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82601C64: 390A62D0  addi r8, r10, 0x62d0
	ctx.r[8].s64 = ctx.r[10].s64 + 25296;
	// 82601C68: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601C6C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82601C70: 38AA2D8C  addi r5, r10, 0x2d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 11660;
	// 82601C74: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601C78: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82601C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601C84: 388AC250  addi r4, r10, -0x3db0
	ctx.r[4].s64 = ctx.r[10].s64 + -15792;
	// 82601C88: 396B96E0  addi r11, r11, -0x6920
	ctx.r[11].s64 = ctx.r[11].s64 + -26912;
	// 82601C8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601C90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601C94: 386A2F0C  addi r3, r10, 0x2f0c
	ctx.r[3].s64 = ctx.r[10].s64 + 12044;
	// 82601C98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82601C9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601CA0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82601CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601CB0: 4BE65171  bl 0x82466e20
	ctx.lr = 0x82601CB4;
	sub_82466E20(ctx, base);
	// 82601CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601CC8 size=112
    let mut pc: u32 = 0x82601CC8;
    'dispatch: loop {
        match pc {
            0x82601CC8 => {
    //   block [0x82601CC8..0x82601D38)
	// 82601CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601CD4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601CD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601CDC: 392A972C  addi r9, r10, -0x68d4
	ctx.r[9].s64 = ctx.r[10].s64 + -26836;
	// 82601CE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601CE4: 390B63B0  addi r8, r11, 0x63b0
	ctx.r[8].s64 = ctx.r[11].s64 + 25520;
	// 82601CE8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82601CEC: 388AC264  addi r4, r10, -0x3d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15772;
	// 82601CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601CF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601D00: 386A2F3C  addi r3, r10, 0x2f3c
	ctx.r[3].s64 = ctx.r[10].s64 + 12092;
	// 82601D04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82601D08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82601D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601D24: 4BE650FD  bl 0x82466e20
	ctx.lr = 0x82601D28;
	sub_82466E20(ctx, base);
	// 82601D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601D38 size=112
    let mut pc: u32 = 0x82601D38;
    'dispatch: loop {
        match pc {
            0x82601D38 => {
    //   block [0x82601D38..0x82601DA8)
	// 82601D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601D44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601D48: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601D4C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601D50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601D54: 390B6410  addi r8, r11, 0x6410
	ctx.r[8].s64 = ctx.r[11].s64 + 25616;
	// 82601D58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82601D5C: 388AC280  addi r4, r10, -0x3d80
	ctx.r[4].s64 = ctx.r[10].s64 + -15744;
	// 82601D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601D64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601D70: 386A2F6C  addi r3, r10, 0x2f6c
	ctx.r[3].s64 = ctx.r[10].s64 + 12140;
	// 82601D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601D94: 4BE6508D  bl 0x82466e20
	ctx.lr = 0x82601D98;
	sub_82466E20(ctx, base);
	// 82601D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601DA8 size=108
    let mut pc: u32 = 0x82601DA8;
    'dispatch: loop {
        match pc {
            0x82601DA8 => {
    //   block [0x82601DA8..0x82601E14)
	// 82601DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601DB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601DB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601DBC: 38EB6428  addi r7, r11, 0x6428
	ctx.r[7].s64 = ctx.r[11].s64 + 25640;
	// 82601DC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82601DC4: 388AC298  addi r4, r10, -0x3d68
	ctx.r[4].s64 = ctx.r[10].s64 + -15720;
	// 82601DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601DCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601DD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82601DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601DD8: 386A2F9C  addi r3, r10, 0x2f9c
	ctx.r[3].s64 = ctx.r[10].s64 + 12188;
	// 82601DDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82601DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601DFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601E00: 4BE65021  bl 0x82466e20
	ctx.lr = 0x82601E04;
	sub_82466E20(ctx, base);
	// 82601E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601E18 size=112
    let mut pc: u32 = 0x82601E18;
    'dispatch: loop {
        match pc {
            0x82601E18 => {
    //   block [0x82601E18..0x82601E88)
	// 82601E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601E24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601E28: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601E2C: 38AA2D8C  addi r5, r10, 0x2d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 11660;
	// 82601E30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601E34: 390B6470  addi r8, r11, 0x6470
	ctx.r[8].s64 = ctx.r[11].s64 + 25712;
	// 82601E38: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82601E3C: 388AC2C0  addi r4, r10, -0x3d40
	ctx.r[4].s64 = ctx.r[10].s64 + -15680;
	// 82601E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601E44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601E48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601E50: 386A2FCC  addi r3, r10, 0x2fcc
	ctx.r[3].s64 = ctx.r[10].s64 + 12236;
	// 82601E54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601E74: 4BE64FAD  bl 0x82466e20
	ctx.lr = 0x82601E78;
	sub_82466E20(ctx, base);
	// 82601E78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601E88 size=112
    let mut pc: u32 = 0x82601E88;
    'dispatch: loop {
        match pc {
            0x82601E88 => {
    //   block [0x82601E88..0x82601EF8)
	// 82601E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601E94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601E98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601E9C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601EA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601EA4: 390B64E8  addi r8, r11, 0x64e8
	ctx.r[8].s64 = ctx.r[11].s64 + 25832;
	// 82601EA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82601EAC: 388AC2D8  addi r4, r10, -0x3d28
	ctx.r[4].s64 = ctx.r[10].s64 + -15656;
	// 82601EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601EB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601EB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601EC0: 386A2FFC  addi r3, r10, 0x2ffc
	ctx.r[3].s64 = ctx.r[10].s64 + 12284;
	// 82601EC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601EE4: 4BE64F3D  bl 0x82466e20
	ctx.lr = 0x82601EE8;
	sub_82466E20(ctx, base);
	// 82601EE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601EF8 size=108
    let mut pc: u32 = 0x82601EF8;
    'dispatch: loop {
        match pc {
            0x82601EF8 => {
    //   block [0x82601EF8..0x82601F64)
	// 82601EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601F04: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601F08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601F0C: 38EB6518  addi r7, r11, 0x6518
	ctx.r[7].s64 = ctx.r[11].s64 + 25880;
	// 82601F10: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82601F14: 388AC2F4  addi r4, r10, -0x3d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -15628;
	// 82601F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601F1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601F20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82601F24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601F28: 386A302C  addi r3, r10, 0x302c
	ctx.r[3].s64 = ctx.r[10].s64 + 12332;
	// 82601F2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82601F30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601F44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601F4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601F50: 4BE64ED1  bl 0x82466e20
	ctx.lr = 0x82601F54;
	sub_82466E20(ctx, base);
	// 82601F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601F68 size=108
    let mut pc: u32 = 0x82601F68;
    'dispatch: loop {
        match pc {
            0x82601F68 => {
    //   block [0x82601F68..0x82601FD4)
	// 82601F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601F74: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601F78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601F7C: 38EB6578  addi r7, r11, 0x6578
	ctx.r[7].s64 = ctx.r[11].s64 + 25976;
	// 82601F80: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82601F84: 388AC324  addi r4, r10, -0x3cdc
	ctx.r[4].s64 = ctx.r[10].s64 + -15580;
	// 82601F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601F8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601F90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82601F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601F98: 386A305C  addi r3, r10, 0x305c
	ctx.r[3].s64 = ctx.r[10].s64 + 12380;
	// 82601F9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82601FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601FC0: 4BE64E61  bl 0x82466e20
	ctx.lr = 0x82601FC4;
	sub_82466E20(ctx, base);
	// 82601FC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601FD8 size=112
    let mut pc: u32 = 0x82601FD8;
    'dispatch: loop {
        match pc {
            0x82601FD8 => {
    //   block [0x82601FD8..0x82602048)
	// 82601FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601FE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601FE8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601FEC: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601FF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601FF4: 390B65F0  addi r8, r11, 0x65f0
	ctx.r[8].s64 = ctx.r[11].s64 + 26096;
	// 82601FF8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82601FFC: 388AC344  addi r4, r10, -0x3cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -15548;
	// 82602000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602004: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260200C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602010: 386A308C  addi r3, r10, 0x308c
	ctx.r[3].s64 = ctx.r[10].s64 + 12428;
	// 82602014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260201C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260202C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602034: 4BE64DED  bl 0x82466e20
	ctx.lr = 0x82602038;
	sub_82466E20(ctx, base);
	// 82602038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260203C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82602048 size=24
    let mut pc: u32 = 0x82602048;
    'dispatch: loop {
        match pc {
            0x82602048 => {
    //   block [0x82602048..0x82602060)
	// 82602048: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260204C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82602050: 394AB358  addi r10, r10, -0x4ca8
	ctx.r[10].s64 = ctx.r[10].s64 + -19624;
	// 82602054: 816B63AC  lwz r11, 0x63ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25516 as u32) ) } as u64;
	// 82602058: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8260205C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602060 size=116
    let mut pc: u32 = 0x82602060;
    'dispatch: loop {
        match pc {
            0x82602060 => {
    //   block [0x82602060..0x826020D4)
	// 82602060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260206C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82602070: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602074: 390BB358  addi r8, r11, -0x4ca8
	ctx.r[8].s64 = ctx.r[11].s64 + -19624;
	// 82602078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260207C: 392A9770  addi r9, r10, -0x6890
	ctx.r[9].s64 = ctx.r[10].s64 + -26768;
	// 82602080: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602084: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82602088: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 8260208C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602094: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260209C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826020A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826020A4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 826020A8: 388AC360  addi r4, r10, -0x3ca0
	ctx.r[4].s64 = ctx.r[10].s64 + -15520;
	// 826020AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826020B0: 386B30BC  addi r3, r11, 0x30bc
	ctx.r[3].s64 = ctx.r[11].s64 + 12476;
	// 826020B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826020B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826020BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826020C0: 4BE64D61  bl 0x82466e20
	ctx.lr = 0x826020C4;
	sub_82466E20(ctx, base);
	// 826020C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826020C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826020CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826020D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826020D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826020D8 size=112
    let mut pc: u32 = 0x826020D8;
    'dispatch: loop {
        match pc {
            0x826020D8 => {
    //   block [0x826020D8..0x82602148)
	// 826020D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826020DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826020E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826020E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826020E8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826020EC: 38AA30BC  addi r5, r10, 0x30bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12476;
	// 826020F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826020F4: 390B6638  addi r8, r11, 0x6638
	ctx.r[8].s64 = ctx.r[11].s64 + 26168;
	// 826020F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826020FC: 388AC374  addi r4, r10, -0x3c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -15500;
	// 82602100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602104: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260210C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602110: 386A30EC  addi r3, r10, 0x30ec
	ctx.r[3].s64 = ctx.r[10].s64 + 12524;
	// 82602114: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260211C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260212C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602134: 4BE64CED  bl 0x82466e20
	ctx.lr = 0x82602138;
	sub_82466E20(ctx, base);
	// 82602138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260213C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82602148 size=24
    let mut pc: u32 = 0x82602148;
    'dispatch: loop {
        match pc {
            0x82602148 => {
    //   block [0x82602148..0x82602160)
	// 82602148: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260214C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82602150: 394AB370  addi r10, r10, -0x4c90
	ctx.r[10].s64 = ctx.r[10].s64 + -19600;
	// 82602154: 816B6668  lwz r11, 0x6668(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26216 as u32) ) } as u64;
	// 82602158: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8260215C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602160 size=116
    let mut pc: u32 = 0x82602160;
    'dispatch: loop {
        match pc {
            0x82602160 => {
    //   block [0x82602160..0x826021D4)
	// 82602160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260216C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82602170: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602174: 390BB370  addi r8, r11, -0x4c90
	ctx.r[8].s64 = ctx.r[11].s64 + -19600;
	// 82602178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260217C: 392A97AC  addi r9, r10, -0x6854
	ctx.r[9].s64 = ctx.r[10].s64 + -26708;
	// 82602180: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602184: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82602188: 38AA30EC  addi r5, r10, 0x30ec
	ctx.r[5].s64 = ctx.r[10].s64 + 12524;
	// 8260218C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602194: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260219C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826021A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826021A4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 826021A8: 388AC394  addi r4, r10, -0x3c6c
	ctx.r[4].s64 = ctx.r[10].s64 + -15468;
	// 826021AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826021B0: 386B311C  addi r3, r11, 0x311c
	ctx.r[3].s64 = ctx.r[11].s64 + 12572;
	// 826021B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826021B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826021BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826021C0: 4BE64C61  bl 0x82466e20
	ctx.lr = 0x826021C4;
	sub_82466E20(ctx, base);
	// 826021C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826021C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826021CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826021D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826021D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826021D8 size=112
    let mut pc: u32 = 0x826021D8;
    'dispatch: loop {
        match pc {
            0x826021D8 => {
    //   block [0x826021D8..0x82602248)
	// 826021D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826021DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826021E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826021E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826021E8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826021EC: 38AA30EC  addi r5, r10, 0x30ec
	ctx.r[5].s64 = ctx.r[10].s64 + 12524;
	// 826021F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826021F4: 390B6670  addi r8, r11, 0x6670
	ctx.r[8].s64 = ctx.r[11].s64 + 26224;
	// 826021F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826021FC: 388AC3B0  addi r4, r10, -0x3c50
	ctx.r[4].s64 = ctx.r[10].s64 + -15440;
	// 82602200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602204: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602208: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260220C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602210: 386A314C  addi r3, r10, 0x314c
	ctx.r[3].s64 = ctx.r[10].s64 + 12620;
	// 82602214: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260221C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260222C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602234: 4BE64BED  bl 0x82466e20
	ctx.lr = 0x82602238;
	sub_82466E20(ctx, base);
	// 82602238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260223C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602248 size=112
    let mut pc: u32 = 0x82602248;
    'dispatch: loop {
        match pc {
            0x82602248 => {
    //   block [0x82602248..0x826022B8)
	// 82602248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260224C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602254: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602258: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260225C: 38AA30EC  addi r5, r10, 0x30ec
	ctx.r[5].s64 = ctx.r[10].s64 + 12524;
	// 82602260: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602264: 390B66D0  addi r8, r11, 0x66d0
	ctx.r[8].s64 = ctx.r[11].s64 + 26320;
	// 82602268: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260226C: 388AC3CC  addi r4, r10, -0x3c34
	ctx.r[4].s64 = ctx.r[10].s64 + -15412;
	// 82602270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602274: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260227C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602280: 386A317C  addi r3, r10, 0x317c
	ctx.r[3].s64 = ctx.r[10].s64 + 12668;
	// 82602284: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260228C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260229C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826022A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826022A4: 4BE64B7D  bl 0x82466e20
	ctx.lr = 0x826022A8;
	sub_82466E20(ctx, base);
	// 826022A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826022AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826022B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826022B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826022B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826022B8 size=112
    let mut pc: u32 = 0x826022B8;
    'dispatch: loop {
        match pc {
            0x826022B8 => {
    //   block [0x826022B8..0x82602328)
	// 826022B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826022BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826022C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826022C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826022C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826022CC: 38AA30EC  addi r5, r10, 0x30ec
	ctx.r[5].s64 = ctx.r[10].s64 + 12524;
	// 826022D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826022D4: 390B6700  addi r8, r11, 0x6700
	ctx.r[8].s64 = ctx.r[11].s64 + 26368;
	// 826022D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826022DC: 388AC3EC  addi r4, r10, -0x3c14
	ctx.r[4].s64 = ctx.r[10].s64 + -15380;
	// 826022E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826022E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826022E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826022EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826022F0: 386A31AC  addi r3, r10, 0x31ac
	ctx.r[3].s64 = ctx.r[10].s64 + 12716;
	// 826022F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826022F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826022FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260230C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602314: 4BE64B0D  bl 0x82466e20
	ctx.lr = 0x82602318;
	sub_82466E20(ctx, base);
	// 82602318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260231C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602328 size=108
    let mut pc: u32 = 0x82602328;
    'dispatch: loop {
        match pc {
            0x82602328 => {
    //   block [0x82602328..0x82602394)
	// 82602328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260232C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602334: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602338: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260233C: 38EB6748  addi r7, r11, 0x6748
	ctx.r[7].s64 = ctx.r[11].s64 + 26440;
	// 82602340: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82602344: 388AC408  addi r4, r10, -0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + -15352;
	// 82602348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260234C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602350: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82602354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602358: 386A31DC  addi r3, r10, 0x31dc
	ctx.r[3].s64 = ctx.r[10].s64 + 12764;
	// 8260235C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82602360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260236C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260237C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82602380: 4BE64AA1  bl 0x82466e20
	ctx.lr = 0x82602384;
	sub_82466E20(ctx, base);
	// 82602384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260238C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602398 size=112
    let mut pc: u32 = 0x82602398;
    'dispatch: loop {
        match pc {
            0x82602398 => {
    //   block [0x82602398..0x82602408)
	// 82602398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260239C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826023A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826023A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826023A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826023AC: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 826023B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826023B4: 390B6778  addi r8, r11, 0x6778
	ctx.r[8].s64 = ctx.r[11].s64 + 26488;
	// 826023B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826023BC: 388AC428  addi r4, r10, -0x3bd8
	ctx.r[4].s64 = ctx.r[10].s64 + -15320;
	// 826023C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826023C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826023C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826023CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826023D0: 386A320C  addi r3, r10, 0x320c
	ctx.r[3].s64 = ctx.r[10].s64 + 12812;
	// 826023D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826023D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826023DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826023E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826023E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826023E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826023EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826023F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826023F4: 4BE64A2D  bl 0x82466e20
	ctx.lr = 0x826023F8;
	sub_82466E20(ctx, base);
	// 826023F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826023FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602408 size=108
    let mut pc: u32 = 0x82602408;
    'dispatch: loop {
        match pc {
            0x82602408 => {
    //   block [0x82602408..0x82602474)
	// 82602408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260240C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602414: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602418: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260241C: 38EB6790  addi r7, r11, 0x6790
	ctx.r[7].s64 = ctx.r[11].s64 + 26512;
	// 82602420: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82602424: 388AC440  addi r4, r10, -0x3bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -15296;
	// 82602428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260242C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82602434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602438: 386A323C  addi r3, r10, 0x323c
	ctx.r[3].s64 = ctx.r[10].s64 + 12860;
	// 8260243C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82602440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260244C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260245C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82602460: 4BE649C1  bl 0x82466e20
	ctx.lr = 0x82602464;
	sub_82466E20(ctx, base);
	// 82602464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260246C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602478 size=108
    let mut pc: u32 = 0x82602478;
    'dispatch: loop {
        match pc {
            0x82602478 => {
    //   block [0x82602478..0x826024E4)
	// 82602478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260247C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602484: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602488: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260248C: 38EB67D8  addi r7, r11, 0x67d8
	ctx.r[7].s64 = ctx.r[11].s64 + 26584;
	// 82602490: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82602494: 388AC46C  addi r4, r10, -0x3b94
	ctx.r[4].s64 = ctx.r[10].s64 + -15252;
	// 82602498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260249C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826024A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826024A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826024A8: 386A326C  addi r3, r10, 0x326c
	ctx.r[3].s64 = ctx.r[10].s64 + 12908;
	// 826024AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826024B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826024B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826024B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826024BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826024C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826024C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826024C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826024CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826024D0: 4BE64951  bl 0x82466e20
	ctx.lr = 0x826024D4;
	sub_82466E20(ctx, base);
	// 826024D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826024D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826024DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826024E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826024E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826024E8 size=108
    let mut pc: u32 = 0x826024E8;
    'dispatch: loop {
        match pc {
            0x826024E8 => {
    //   block [0x826024E8..0x82602554)
	// 826024E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826024EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826024F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826024F4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826024F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826024FC: 38EB6838  addi r7, r11, 0x6838
	ctx.r[7].s64 = ctx.r[11].s64 + 26680;
	// 82602500: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82602504: 388AC48C  addi r4, r10, -0x3b74
	ctx.r[4].s64 = ctx.r[10].s64 + -15220;
	// 82602508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260250C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602510: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82602514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602518: 386A329C  addi r3, r10, 0x329c
	ctx.r[3].s64 = ctx.r[10].s64 + 12956;
	// 8260251C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82602520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260252C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260253C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82602540: 4BE648E1  bl 0x82466e20
	ctx.lr = 0x82602544;
	sub_82466E20(ctx, base);
	// 82602544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260254C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602558 size=116
    let mut pc: u32 = 0x82602558;
    'dispatch: loop {
        match pc {
            0x82602558 => {
    //   block [0x82602558..0x826025CC)
	// 82602558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260255C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602564: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82602568: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260256C: 392B97E8  addi r9, r11, -0x6818
	ctx.r[9].s64 = ctx.r[11].s64 + -26648;
	// 82602570: 38AA37AC  addi r5, r10, 0x37ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14252;
	// 82602574: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602578: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 8260257C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 82602580: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602584: 388AC4A8  addi r4, r10, -0x3b58
	ctx.r[4].s64 = ctx.r[10].s64 + -15192;
	// 82602588: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260258C: 396B6868  addi r11, r11, 0x6868
	ctx.r[11].s64 = ctx.r[11].s64 + 26728;
	// 82602590: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82602594: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602598: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260259C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826025A0: 386A32CC  addi r3, r10, 0x32cc
	ctx.r[3].s64 = ctx.r[10].s64 + 13004;
	// 826025A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826025A8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826025AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826025B0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826025B4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826025B8: 4BE64869  bl 0x82466e20
	ctx.lr = 0x826025BC;
	sub_82466E20(ctx, base);
	// 826025BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826025C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826025C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826025C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826025D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826025D0 size=100
    let mut pc: u32 = 0x826025D0;
    'dispatch: loop {
        match pc {
            0x826025D0 => {
    //   block [0x826025D0..0x82602634)
	// 826025D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826025D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826025D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826025DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826025E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826025E4: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826025E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826025EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826025F0: 388AC4B4  addi r4, r10, -0x3b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -15180;
	// 826025F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826025F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826025FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602604: 386A32FC  addi r3, r10, 0x32fc
	ctx.r[3].s64 = ctx.r[10].s64 + 13052;
	// 82602608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260260C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602610: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602618: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260261C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602620: 4BE64801  bl 0x82466e20
	ctx.lr = 0x82602624;
	sub_82466E20(ctx, base);
	// 82602624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260262C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602638 size=100
    let mut pc: u32 = 0x82602638;
    'dispatch: loop {
        match pc {
            0x82602638 => {
    //   block [0x82602638..0x8260269C)
	// 82602638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260263C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602644: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260264C: 38AA338C  addi r5, r10, 0x338c
	ctx.r[5].s64 = ctx.r[10].s64 + 13196;
	// 82602650: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602658: 388AC4CC  addi r4, r10, -0x3b34
	ctx.r[4].s64 = ctx.r[10].s64 + -15156;
	// 8260265C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260266C: 386A332C  addi r3, r10, 0x332c
	ctx.r[3].s64 = ctx.r[10].s64 + 13100;
	// 82602670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602674: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602678: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260267C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602680: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602688: 4BE64799  bl 0x82466e20
	ctx.lr = 0x8260268C;
	sub_82466E20(ctx, base);
	// 8260268C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826026A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826026A0 size=100
    let mut pc: u32 = 0x826026A0;
    'dispatch: loop {
        match pc {
            0x826026A0 => {
    //   block [0x826026A0..0x82602704)
	// 826026A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826026A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826026A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826026AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826026B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826026B4: 38AA32CC  addi r5, r10, 0x32cc
	ctx.r[5].s64 = ctx.r[10].s64 + 13004;
	// 826026B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826026BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826026C0: 388AC4E8  addi r4, r10, -0x3b18
	ctx.r[4].s64 = ctx.r[10].s64 + -15128;
	// 826026C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826026C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826026CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826026D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826026D4: 386A335C  addi r3, r10, 0x335c
	ctx.r[3].s64 = ctx.r[10].s64 + 13148;
	// 826026D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826026DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826026E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826026E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826026E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826026EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826026F0: 4BE64731  bl 0x82466e20
	ctx.lr = 0x826026F4;
	sub_82466E20(ctx, base);
	// 826026F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826026F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826026FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602708 size=104
    let mut pc: u32 = 0x82602708;
    'dispatch: loop {
        match pc {
            0x82602708 => {
    //   block [0x82602708..0x82602770)
	// 82602708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260270C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602714: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260271C: 392A9868  addi r9, r10, -0x6798
	ctx.r[9].s64 = ctx.r[10].s64 + -26520;
	// 82602720: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602728: 38AA32FC  addi r5, r10, 0x32fc
	ctx.r[5].s64 = ctx.r[10].s64 + 13052;
	// 8260272C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260273C: 388AC4F8  addi r4, r10, -0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + -15112;
	// 82602740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602748: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260274C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602750: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602754: 386A338C  addi r3, r10, 0x338c
	ctx.r[3].s64 = ctx.r[10].s64 + 13196;
	// 82602758: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260275C: 4BE646C5  bl 0x82466e20
	ctx.lr = 0x82602760;
	sub_82466E20(ctx, base);
	// 82602760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260276C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602770 size=108
    let mut pc: u32 = 0x82602770;
    'dispatch: loop {
        match pc {
            0x82602770 => {
    //   block [0x82602770..0x826027DC)
	// 82602770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260277C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602780: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602784: 38EB6A04  addi r7, r11, 0x6a04
	ctx.r[7].s64 = ctx.r[11].s64 + 27140;
	// 82602788: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260278C: 388AC510  addi r4, r10, -0x3af0
	ctx.r[4].s64 = ctx.r[10].s64 + -15088;
	// 82602790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602794: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602798: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260279C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826027A0: 386A33BC  addi r3, r10, 0x33bc
	ctx.r[3].s64 = ctx.r[10].s64 + 13244;
	// 826027A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826027A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826027AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826027B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826027B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826027B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826027BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826027C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826027C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826027C8: 4BE64659  bl 0x82466e20
	ctx.lr = 0x826027CC;
	sub_82466E20(ctx, base);
	// 826027CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826027D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826027D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826027D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826027E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826027E0 size=112
    let mut pc: u32 = 0x826027E0;
    'dispatch: loop {
        match pc {
            0x826027E0 => {
    //   block [0x826027E0..0x82602850)
	// 826027E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826027E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826027E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826027EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826027F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826027F4: 38AA338C  addi r5, r10, 0x338c
	ctx.r[5].s64 = ctx.r[10].s64 + 13196;
	// 826027F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826027FC: 390B6A38  addi r8, r11, 0x6a38
	ctx.r[8].s64 = ctx.r[11].s64 + 27192;
	// 82602800: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82602804: 388AC538  addi r4, r10, -0x3ac8
	ctx.r[4].s64 = ctx.r[10].s64 + -15048;
	// 82602808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260280C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602810: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602818: 386A33EC  addi r3, r10, 0x33ec
	ctx.r[3].s64 = ctx.r[10].s64 + 13292;
	// 8260281C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260282C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260283C: 4BE645E5  bl 0x82466e20
	ctx.lr = 0x82602840;
	sub_82466E20(ctx, base);
	// 82602840: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260284C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82602850 size=24
    let mut pc: u32 = 0x82602850;
    'dispatch: loop {
        match pc {
            0x82602850 => {
    //   block [0x82602850..0x82602868)
	// 82602850: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602854: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82602858: 394AB3E8  addi r10, r10, -0x4c18
	ctx.r[10].s64 = ctx.r[10].s64 + -19480;
	// 8260285C: 816B6A34  lwz r11, 0x6a34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27188 as u32) ) } as u64;
	// 82602860: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82602864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602868 size=116
    let mut pc: u32 = 0x82602868;
    'dispatch: loop {
        match pc {
            0x82602868 => {
    //   block [0x82602868..0x826028DC)
	// 82602868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260286C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602874: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82602878: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260287C: 390BB3E8  addi r8, r11, -0x4c18
	ctx.r[8].s64 = ctx.r[11].s64 + -19480;
	// 82602880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602884: 392A98D8  addi r9, r10, -0x6728
	ctx.r[9].s64 = ctx.r[10].s64 + -26408;
	// 82602888: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260288C: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82602890: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82602894: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260289C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826028A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826028A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826028A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826028AC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 826028B0: 388AC558  addi r4, r10, -0x3aa8
	ctx.r[4].s64 = ctx.r[10].s64 + -15016;
	// 826028B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826028B8: 386B341C  addi r3, r11, 0x341c
	ctx.r[3].s64 = ctx.r[11].s64 + 13340;
	// 826028BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826028C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826028C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826028C8: 4BE64559  bl 0x82466e20
	ctx.lr = 0x826028CC;
	sub_82466E20(ctx, base);
	// 826028CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826028D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826028D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826028D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826028E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826028E0 size=100
    let mut pc: u32 = 0x826028E0;
    'dispatch: loop {
        match pc {
            0x826028E0 => {
    //   block [0x826028E0..0x82602944)
	// 826028E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826028E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826028E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826028EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826028F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826028F4: 38AA341C  addi r5, r10, 0x341c
	ctx.r[5].s64 = ctx.r[10].s64 + 13340;
	// 826028F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826028FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602900: 388AC564  addi r4, r10, -0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15004;
	// 82602904: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260290C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602914: 386A344C  addi r3, r10, 0x344c
	ctx.r[3].s64 = ctx.r[10].s64 + 13388;
	// 82602918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260291C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602920: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602928: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260292C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602930: 4BE644F1  bl 0x82466e20
	ctx.lr = 0x82602934;
	sub_82466E20(ctx, base);
	// 82602934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260293C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602948 size=100
    let mut pc: u32 = 0x82602948;
    'dispatch: loop {
        match pc {
            0x82602948 => {
    //   block [0x82602948..0x826029AC)
	// 82602948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260294C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602954: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260295C: 38AA341C  addi r5, r10, 0x341c
	ctx.r[5].s64 = ctx.r[10].s64 + 13340;
	// 82602960: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602968: 388AC574  addi r4, r10, -0x3a8c
	ctx.r[4].s64 = ctx.r[10].s64 + -14988;
	// 8260296C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260297C: 386A347C  addi r3, r10, 0x347c
	ctx.r[3].s64 = ctx.r[10].s64 + 13436;
	// 82602980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602984: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602988: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260298C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602990: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602998: 4BE64489  bl 0x82466e20
	ctx.lr = 0x8260299C;
	sub_82466E20(ctx, base);
	// 8260299C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826029A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826029A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826029A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826029B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826029B0 size=100
    let mut pc: u32 = 0x826029B0;
    'dispatch: loop {
        match pc {
            0x826029B0 => {
    //   block [0x826029B0..0x82602A14)
	// 826029B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826029B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826029B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826029BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826029C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826029C4: 38AA34DC  addi r5, r10, 0x34dc
	ctx.r[5].s64 = ctx.r[10].s64 + 13532;
	// 826029C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826029CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826029D0: 388AC588  addi r4, r10, -0x3a78
	ctx.r[4].s64 = ctx.r[10].s64 + -14968;
	// 826029D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826029D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826029DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826029E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826029E4: 386A34AC  addi r3, r10, 0x34ac
	ctx.r[3].s64 = ctx.r[10].s64 + 13484;
	// 826029E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826029EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826029F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826029F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826029F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826029FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602A00: 4BE64421  bl 0x82466e20
	ctx.lr = 0x82602A04;
	sub_82466E20(ctx, base);
	// 82602A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602A18 size=100
    let mut pc: u32 = 0x82602A18;
    'dispatch: loop {
        match pc {
            0x82602A18 => {
    //   block [0x82602A18..0x82602A7C)
	// 82602A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602A24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602A2C: 38AA341C  addi r5, r10, 0x341c
	ctx.r[5].s64 = ctx.r[10].s64 + 13340;
	// 82602A30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602A34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602A38: 388AC59C  addi r4, r10, -0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + -14948;
	// 82602A3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602A4C: 386A34DC  addi r3, r10, 0x34dc
	ctx.r[3].s64 = ctx.r[10].s64 + 13532;
	// 82602A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602A54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602A58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602A60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602A68: 4BE643B9  bl 0x82466e20
	ctx.lr = 0x82602A6C;
	sub_82466E20(ctx, base);
	// 82602A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602A80 size=100
    let mut pc: u32 = 0x82602A80;
    'dispatch: loop {
        match pc {
            0x82602A80 => {
    //   block [0x82602A80..0x82602AE4)
	// 82602A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602A8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602A94: 38AA34DC  addi r5, r10, 0x34dc
	ctx.r[5].s64 = ctx.r[10].s64 + 13532;
	// 82602A98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602AA0: 388AC5B4  addi r4, r10, -0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + -14924;
	// 82602AA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602AB4: 386A350C  addi r3, r10, 0x350c
	ctx.r[3].s64 = ctx.r[10].s64 + 13580;
	// 82602AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602ABC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602AC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602AC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602AD0: 4BE64351  bl 0x82466e20
	ctx.lr = 0x82602AD4;
	sub_82466E20(ctx, base);
	// 82602AD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602AE8 size=100
    let mut pc: u32 = 0x82602AE8;
    'dispatch: loop {
        match pc {
            0x82602AE8 => {
    //   block [0x82602AE8..0x82602B4C)
	// 82602AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602AF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602AFC: 38AA341C  addi r5, r10, 0x341c
	ctx.r[5].s64 = ctx.r[10].s64 + 13340;
	// 82602B00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602B08: 388AC5C8  addi r4, r10, -0x3a38
	ctx.r[4].s64 = ctx.r[10].s64 + -14904;
	// 82602B0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602B10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602B18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602B1C: 386A353C  addi r3, r10, 0x353c
	ctx.r[3].s64 = ctx.r[10].s64 + 13628;
	// 82602B20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602B24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602B28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602B30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602B38: 4BE642E9  bl 0x82466e20
	ctx.lr = 0x82602B3C;
	sub_82466E20(ctx, base);
	// 82602B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602B50 size=100
    let mut pc: u32 = 0x82602B50;
    'dispatch: loop {
        match pc {
            0x82602B50 => {
    //   block [0x82602B50..0x82602BB4)
	// 82602B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602B5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602B64: 38AA344C  addi r5, r10, 0x344c
	ctx.r[5].s64 = ctx.r[10].s64 + 13388;
	// 82602B68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602B6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602B70: 388AC5D8  addi r4, r10, -0x3a28
	ctx.r[4].s64 = ctx.r[10].s64 + -14888;
	// 82602B74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602B7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602B84: 386A356C  addi r3, r10, 0x356c
	ctx.r[3].s64 = ctx.r[10].s64 + 13676;
	// 82602B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602B8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602B90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602B98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602BA0: 4BE64281  bl 0x82466e20
	ctx.lr = 0x82602BA4;
	sub_82466E20(ctx, base);
	// 82602BA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602BB8 size=100
    let mut pc: u32 = 0x82602BB8;
    'dispatch: loop {
        match pc {
            0x82602BB8 => {
    //   block [0x82602BB8..0x82602C1C)
	// 82602BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602BC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602BCC: 38AA353C  addi r5, r10, 0x353c
	ctx.r[5].s64 = ctx.r[10].s64 + 13628;
	// 82602BD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602BD8: 388AC5F0  addi r4, r10, -0x3a10
	ctx.r[4].s64 = ctx.r[10].s64 + -14864;
	// 82602BDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602BEC: 386A359C  addi r3, r10, 0x359c
	ctx.r[3].s64 = ctx.r[10].s64 + 13724;
	// 82602BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602BF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602BF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602C00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602C08: 4BE64219  bl 0x82466e20
	ctx.lr = 0x82602C0C;
	sub_82466E20(ctx, base);
	// 82602C0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602C20 size=100
    let mut pc: u32 = 0x82602C20;
    'dispatch: loop {
        match pc {
            0x82602C20 => {
    //   block [0x82602C20..0x82602C84)
	// 82602C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602C2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602C34: 38AA344C  addi r5, r10, 0x344c
	ctx.r[5].s64 = ctx.r[10].s64 + 13388;
	// 82602C38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602C40: 388AC60C  addi r4, r10, -0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + -14836;
	// 82602C44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602C54: 386A35CC  addi r3, r10, 0x35cc
	ctx.r[3].s64 = ctx.r[10].s64 + 13772;
	// 82602C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602C5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602C60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602C64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602C68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602C70: 4BE641B1  bl 0x82466e20
	ctx.lr = 0x82602C74;
	sub_82466E20(ctx, base);
	// 82602C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


