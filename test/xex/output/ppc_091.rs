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


pub fn sub_82656B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656B00 size=108
    let mut pc: u32 = 0x82656B00;
    'dispatch: loop {
        match pc {
            0x82656B00 => {
    //   block [0x82656B00..0x82656B6C)
	// 82656B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656B0C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656B14: 38EBB79C  addi r7, r11, -0x4864
	ctx.r[7].s64 = ctx.r[11].s64 + -18532;
	// 82656B18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82656B1C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 82656B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656B24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656B28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656B30: 386A6D18  addi r3, r10, 0x6d18
	ctx.r[3].s64 = ctx.r[10].s64 + 27928;
	// 82656B34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656B54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656B58: 4BE102C9  bl 0x82466e20
	ctx.lr = 0x82656B5C;
	sub_82466E20(ctx, base);
	// 82656B5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656B60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656B64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656B70 size=108
    let mut pc: u32 = 0x82656B70;
    'dispatch: loop {
        match pc {
            0x82656B70 => {
    //   block [0x82656B70..0x82656BDC)
	// 82656B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656B7C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656B84: 38EBB7CC  addi r7, r11, -0x4834
	ctx.r[7].s64 = ctx.r[11].s64 + -18484;
	// 82656B88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82656B8C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 82656B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656B94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656B98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656BA0: 386A6D48  addi r3, r10, 0x6d48
	ctx.r[3].s64 = ctx.r[10].s64 + 27976;
	// 82656BA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656BC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656BC8: 4BE10259  bl 0x82466e20
	ctx.lr = 0x82656BCC;
	sub_82466E20(ctx, base);
	// 82656BCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656BE0 size=108
    let mut pc: u32 = 0x82656BE0;
    'dispatch: loop {
        match pc {
            0x82656BE0 => {
    //   block [0x82656BE0..0x82656C4C)
	// 82656BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656BEC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656BF4: 38EBB7E4  addi r7, r11, -0x481c
	ctx.r[7].s64 = ctx.r[11].s64 + -18460;
	// 82656BF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82656BFC: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82656C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656C04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656C08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656C10: 386A6D78  addi r3, r10, 0x6d78
	ctx.r[3].s64 = ctx.r[10].s64 + 28024;
	// 82656C14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656C34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656C38: 4BE101E9  bl 0x82466e20
	ctx.lr = 0x82656C3C;
	sub_82466E20(ctx, base);
	// 82656C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656C50 size=108
    let mut pc: u32 = 0x82656C50;
    'dispatch: loop {
        match pc {
            0x82656C50 => {
    //   block [0x82656C50..0x82656CBC)
	// 82656C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656C5C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656C64: 38EBB818  addi r7, r11, -0x47e8
	ctx.r[7].s64 = ctx.r[11].s64 + -18408;
	// 82656C68: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82656C6C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 82656C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656C74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656C78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656C80: 386A6DA8  addi r3, r10, 0x6da8
	ctx.r[3].s64 = ctx.r[10].s64 + 28072;
	// 82656C84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656C8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656C94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656CA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656CA8: 4BE10179  bl 0x82466e20
	ctx.lr = 0x82656CAC;
	sub_82466E20(ctx, base);
	// 82656CAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656CC0 size=108
    let mut pc: u32 = 0x82656CC0;
    'dispatch: loop {
        match pc {
            0x82656CC0 => {
    //   block [0x82656CC0..0x82656D2C)
	// 82656CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656CCC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656CD4: 38EBB8C0  addi r7, r11, -0x4740
	ctx.r[7].s64 = ctx.r[11].s64 + -18240;
	// 82656CD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82656CDC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 82656CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656CE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656CE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656CF0: 386A6DD8  addi r3, r10, 0x6dd8
	ctx.r[3].s64 = ctx.r[10].s64 + 28120;
	// 82656CF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656D14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656D18: 4BE10109  bl 0x82466e20
	ctx.lr = 0x82656D1C;
	sub_82466E20(ctx, base);
	// 82656D1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656D30 size=108
    let mut pc: u32 = 0x82656D30;
    'dispatch: loop {
        match pc {
            0x82656D30 => {
    //   block [0x82656D30..0x82656D9C)
	// 82656D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656D3C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656D44: 38EBB8F0  addi r7, r11, -0x4710
	ctx.r[7].s64 = ctx.r[11].s64 + -18192;
	// 82656D48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82656D4C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 82656D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656D54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656D58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656D60: 386A6E08  addi r3, r10, 0x6e08
	ctx.r[3].s64 = ctx.r[10].s64 + 28168;
	// 82656D64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656D6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656D88: 4BE10099  bl 0x82466e20
	ctx.lr = 0x82656D8C;
	sub_82466E20(ctx, base);
	// 82656D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656DA0 size=108
    let mut pc: u32 = 0x82656DA0;
    'dispatch: loop {
        match pc {
            0x82656DA0 => {
    //   block [0x82656DA0..0x82656E0C)
	// 82656DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656DAC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656DB4: 38EBB908  addi r7, r11, -0x46f8
	ctx.r[7].s64 = ctx.r[11].s64 + -18168;
	// 82656DB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82656DBC: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82656DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656DC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656DC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656DD0: 386A6E38  addi r3, r10, 0x6e38
	ctx.r[3].s64 = ctx.r[10].s64 + 28216;
	// 82656DD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656DF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656DF8: 4BE10029  bl 0x82466e20
	ctx.lr = 0x82656DFC;
	sub_82466E20(ctx, base);
	// 82656DFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656E10 size=112
    let mut pc: u32 = 0x82656E10;
    'dispatch: loop {
        match pc {
            0x82656E10 => {
    //   block [0x82656E10..0x82656E80)
	// 82656E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656E1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656E20: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656E24: 38AA6C88  addi r5, r10, 0x6c88
	ctx.r[5].s64 = ctx.r[10].s64 + 27784;
	// 82656E28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656E2C: 390BB938  addi r8, r11, -0x46c8
	ctx.r[8].s64 = ctx.r[11].s64 + -18120;
	// 82656E30: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82656E34: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82656E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656E3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656E40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82656E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656E48: 386A6E68  addi r3, r10, 0x6e68
	ctx.r[3].s64 = ctx.r[10].s64 + 28264;
	// 82656E4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82656E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656E6C: 4BE0FFB5  bl 0x82466e20
	ctx.lr = 0x82656E70;
	sub_82466E20(ctx, base);
	// 82656E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82656E80 size=24
    let mut pc: u32 = 0x82656E80;
    'dispatch: loop {
        match pc {
            0x82656E80 => {
    //   block [0x82656E80..0x82656E98)
	// 82656E80: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656E84: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82656E88: 394A1A40  addi r10, r10, 0x1a40
	ctx.r[10].s64 = ctx.r[10].s64 + 6720;
	// 82656E8C: 816BB814  lwz r11, -0x47ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18412 as u32) ) } as u64;
	// 82656E90: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82656E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656E98 size=112
    let mut pc: u32 = 0x82656E98;
    'dispatch: loop {
        match pc {
            0x82656E98 => {
    //   block [0x82656E98..0x82656F08)
	// 82656E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656EA4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82656EA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656EAC: 392AD110  addi r9, r10, -0x2ef0
	ctx.r[9].s64 = ctx.r[10].s64 + -12016;
	// 82656EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656EB4: 390B1A40  addi r8, r11, 0x1a40
	ctx.r[8].s64 = ctx.r[11].s64 + 6720;
	// 82656EB8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82656EBC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82656EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656EC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656EC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82656ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656ED0: 386A6E98  addi r3, r10, 0x6e98
	ctx.r[3].s64 = ctx.r[10].s64 + 28312;
	// 82656ED4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82656ED8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82656EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656EEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656EF4: 4BE0FF2D  bl 0x82466e20
	ctx.lr = 0x82656EF8;
	sub_82466E20(ctx, base);
	// 82656EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656F08 size=108
    let mut pc: u32 = 0x82656F08;
    'dispatch: loop {
        match pc {
            0x82656F08 => {
    //   block [0x82656F08..0x82656F74)
	// 82656F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656F14: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656F1C: 38EBB9E4  addi r7, r11, -0x461c
	ctx.r[7].s64 = ctx.r[11].s64 + -17948;
	// 82656F20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82656F24: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82656F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656F2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82656F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82656F38: 386A6EC8  addi r3, r10, 0x6ec8
	ctx.r[3].s64 = ctx.r[10].s64 + 28360;
	// 82656F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82656F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82656F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82656F60: 4BE0FEC1  bl 0x82466e20
	ctx.lr = 0x82656F64;
	sub_82466E20(ctx, base);
	// 82656F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82656F78 size=116
    let mut pc: u32 = 0x82656F78;
    'dispatch: loop {
        match pc {
            0x82656F78 => {
    //   block [0x82656F78..0x82656FEC)
	// 82656F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82656F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82656F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82656F84: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656F88: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82656F8C: 390BBA18  addi r8, r11, -0x45e8
	ctx.r[8].s64 = ctx.r[11].s64 + -17896;
	// 82656F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82656F94: 392AD154  addi r9, r10, -0x2eac
	ctx.r[9].s64 = ctx.r[10].s64 + -11948;
	// 82656F98: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82656F9C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82656FA0: 38AA6C88  addi r5, r10, 0x6c88
	ctx.r[5].s64 = ctx.r[10].s64 + 27784;
	// 82656FA4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82656FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82656FAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82656FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82656FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82656FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82656FBC: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82656FC0: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82656FC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82656FC8: 386B6EF8  addi r3, r11, 0x6ef8
	ctx.r[3].s64 = ctx.r[11].s64 + 28408;
	// 82656FCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82656FD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82656FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82656FD8: 4BE0FE49  bl 0x82466e20
	ctx.lr = 0x82656FDC;
	sub_82466E20(ctx, base);
	// 82656FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82656FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82656FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82656FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82656FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82656FF0 size=24
    let mut pc: u32 = 0x82656FF0;
    'dispatch: loop {
        match pc {
            0x82656FF0 => {
    //   block [0x82656FF0..0x82657008)
	// 82656FF0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82656FF4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82656FF8: 394A1AB8  addi r10, r10, 0x1ab8
	ctx.r[10].s64 = ctx.r[10].s64 + 6840;
	// 82656FFC: 816BBA14  lwz r11, -0x45ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17900 as u32) ) } as u64;
	// 82657000: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82657004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657008 size=112
    let mut pc: u32 = 0x82657008;
    'dispatch: loop {
        match pc {
            0x82657008 => {
    //   block [0x82657008..0x82657078)
	// 82657008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265700C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657014: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82657018: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265701C: 392AD190  addi r9, r10, -0x2e70
	ctx.r[9].s64 = ctx.r[10].s64 + -11888;
	// 82657020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657024: 390B1AB8  addi r8, r11, 0x1ab8
	ctx.r[8].s64 = ctx.r[11].s64 + 6840;
	// 82657028: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8265702C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 82657030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657034: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265703C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657040: 386A6F28  addi r3, r10, 0x6f28
	ctx.r[3].s64 = ctx.r[10].s64 + 28456;
	// 82657044: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82657048: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265704C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265705C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657064: 4BE0FDBD  bl 0x82466e20
	ctx.lr = 0x82657068;
	sub_82466E20(ctx, base);
	// 82657068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265706C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657078 size=108
    let mut pc: u32 = 0x82657078;
    'dispatch: loop {
        match pc {
            0x82657078 => {
    //   block [0x82657078..0x826570E4)
	// 82657078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265707C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657084: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265708C: 38EBBAD8  addi r7, r11, -0x4528
	ctx.r[7].s64 = ctx.r[11].s64 + -17704;
	// 82657090: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82657094: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 82657098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265709C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826570A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826570A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826570A8: 386A6F58  addi r3, r10, 0x6f58
	ctx.r[3].s64 = ctx.r[10].s64 + 28504;
	// 826570AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826570B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826570B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826570B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826570BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826570C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826570C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826570C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826570CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826570D0: 4BE0FD51  bl 0x82466e20
	ctx.lr = 0x826570D4;
	sub_82466E20(ctx, base);
	// 826570D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826570D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826570DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826570E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826570E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826570E8 size=108
    let mut pc: u32 = 0x826570E8;
    'dispatch: loop {
        match pc {
            0x826570E8 => {
    //   block [0x826570E8..0x82657154)
	// 826570E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826570EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826570F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826570F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826570F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826570FC: 38EBBAF0  addi r7, r11, -0x4510
	ctx.r[7].s64 = ctx.r[11].s64 + -17680;
	// 82657100: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82657104: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82657108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265710C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657110: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657118: 386A6F88  addi r3, r10, 0x6f88
	ctx.r[3].s64 = ctx.r[10].s64 + 28552;
	// 8265711C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265712C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265713C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657140: 4BE0FCE1  bl 0x82466e20
	ctx.lr = 0x82657144;
	sub_82466E20(ctx, base);
	// 82657144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265714C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82657158 size=24
    let mut pc: u32 = 0x82657158;
    'dispatch: loop {
        match pc {
            0x82657158 => {
    //   block [0x82657158..0x82657170)
	// 82657158: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265715C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82657160: 394A1B00  addi r10, r10, 0x1b00
	ctx.r[10].s64 = ctx.r[10].s64 + 6912;
	// 82657164: 816BBB20  lwz r11, -0x44e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17632 as u32) ) } as u64;
	// 82657168: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265716C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657170 size=112
    let mut pc: u32 = 0x82657170;
    'dispatch: loop {
        match pc {
            0x82657170 => {
    //   block [0x82657170..0x826571E0)
	// 82657170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265717C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82657180: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657184: 392AD1CC  addi r9, r10, -0x2e34
	ctx.r[9].s64 = ctx.r[10].s64 + -11828;
	// 82657188: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265718C: 390B1B00  addi r8, r11, 0x1b00
	ctx.r[8].s64 = ctx.r[11].s64 + 6912;
	// 82657190: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82657194: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 82657198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265719C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826571A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826571A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826571A8: 386A6FB8  addi r3, r10, 0x6fb8
	ctx.r[3].s64 = ctx.r[10].s64 + 28600;
	// 826571AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826571B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826571B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826571B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826571BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826571C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826571C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826571C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826571CC: 4BE0FC55  bl 0x82466e20
	ctx.lr = 0x826571D0;
	sub_82466E20(ctx, base);
	// 826571D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826571D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826571D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826571DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826571E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826571E0 size=112
    let mut pc: u32 = 0x826571E0;
    'dispatch: loop {
        match pc {
            0x826571E0 => {
    //   block [0x826571E0..0x82657250)
	// 826571E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826571E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826571E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826571EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826571F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826571F4: 38AA6C88  addi r5, r10, 0x6c88
	ctx.r[5].s64 = ctx.r[10].s64 + 27784;
	// 826571F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826571FC: 390BBB24  addi r8, r11, -0x44dc
	ctx.r[8].s64 = ctx.r[11].s64 + -17628;
	// 82657200: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82657204: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 82657208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265720C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657210: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82657214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657218: 386A6FE8  addi r3, r10, 0x6fe8
	ctx.r[3].s64 = ctx.r[10].s64 + 28648;
	// 8265721C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82657220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265722C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265723C: 4BE0FBE5  bl 0x82466e20
	ctx.lr = 0x82657240;
	sub_82466E20(ctx, base);
	// 82657240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265724C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657250 size=108
    let mut pc: u32 = 0x82657250;
    'dispatch: loop {
        match pc {
            0x82657250 => {
    //   block [0x82657250..0x826572BC)
	// 82657250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265725C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657264: 38EBBB54  addi r7, r11, -0x44ac
	ctx.r[7].s64 = ctx.r[11].s64 + -17580;
	// 82657268: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265726C: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 82657270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657274: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265727C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657280: 386A7018  addi r3, r10, 0x7018
	ctx.r[3].s64 = ctx.r[10].s64 + 28696;
	// 82657284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265728C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265729C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826572A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826572A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826572A8: 4BE0FB79  bl 0x82466e20
	ctx.lr = 0x826572AC;
	sub_82466E20(ctx, base);
	// 826572AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826572B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826572B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826572B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826572C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826572C0 size=108
    let mut pc: u32 = 0x826572C0;
    'dispatch: loop {
        match pc {
            0x826572C0 => {
    //   block [0x826572C0..0x8265732C)
	// 826572C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826572C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826572C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826572CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826572D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826572D4: 38EBBB88  addi r7, r11, -0x4478
	ctx.r[7].s64 = ctx.r[11].s64 + -17528;
	// 826572D8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826572DC: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826572E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826572E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826572E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826572EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826572F0: 386A7048  addi r3, r10, 0x7048
	ctx.r[3].s64 = ctx.r[10].s64 + 28744;
	// 826572F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826572F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826572FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265730C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657318: 4BE0FB09  bl 0x82466e20
	ctx.lr = 0x8265731C;
	sub_82466E20(ctx, base);
	// 8265731C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657330 size=108
    let mut pc: u32 = 0x82657330;
    'dispatch: loop {
        match pc {
            0x82657330 => {
    //   block [0x82657330..0x8265739C)
	// 82657330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265733C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657344: 38EBBBE8  addi r7, r11, -0x4418
	ctx.r[7].s64 = ctx.r[11].s64 + -17432;
	// 82657348: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265734C: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 82657350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657354: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657358: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265735C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657360: 386A7078  addi r3, r10, 0x7078
	ctx.r[3].s64 = ctx.r[10].s64 + 28792;
	// 82657364: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265736C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265737C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657384: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657388: 4BE0FA99  bl 0x82466e20
	ctx.lr = 0x8265738C;
	sub_82466E20(ctx, base);
	// 8265738C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826573A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826573A0 size=108
    let mut pc: u32 = 0x826573A0;
    'dispatch: loop {
        match pc {
            0x826573A0 => {
    //   block [0x826573A0..0x8265740C)
	// 826573A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826573A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826573A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826573AC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826573B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826573B4: 38EBBC18  addi r7, r11, -0x43e8
	ctx.r[7].s64 = ctx.r[11].s64 + -17384;
	// 826573B8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826573BC: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826573C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826573C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826573C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826573CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826573D0: 386A70A8  addi r3, r10, 0x70a8
	ctx.r[3].s64 = ctx.r[10].s64 + 28840;
	// 826573D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826573D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826573DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826573E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826573E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826573E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826573EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826573F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826573F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826573F8: 4BE0FA29  bl 0x82466e20
	ctx.lr = 0x826573FC;
	sub_82466E20(ctx, base);
	// 826573FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657410 size=108
    let mut pc: u32 = 0x82657410;
    'dispatch: loop {
        match pc {
            0x82657410 => {
    //   block [0x82657410..0x8265747C)
	// 82657410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265741C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657424: 38EBBD38  addi r7, r11, -0x42c8
	ctx.r[7].s64 = ctx.r[11].s64 + -17096;
	// 82657428: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265742C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 82657430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657434: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657438: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265743C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657440: 386A70D8  addi r3, r10, 0x70d8
	ctx.r[3].s64 = ctx.r[10].s64 + 28888;
	// 82657444: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265744C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265745C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657468: 4BE0F9B9  bl 0x82466e20
	ctx.lr = 0x8265746C;
	sub_82466E20(ctx, base);
	// 8265746C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657480 size=108
    let mut pc: u32 = 0x82657480;
    'dispatch: loop {
        match pc {
            0x82657480 => {
    //   block [0x82657480..0x826574EC)
	// 82657480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265748C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657494: 38EBBD50  addi r7, r11, -0x42b0
	ctx.r[7].s64 = ctx.r[11].s64 + -17072;
	// 82657498: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265749C: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 826574A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826574A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826574A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826574AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826574B0: 386A7108  addi r3, r10, 0x7108
	ctx.r[3].s64 = ctx.r[10].s64 + 28936;
	// 826574B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826574B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826574BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826574C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826574C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826574C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826574CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826574D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826574D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826574D8: 4BE0F949  bl 0x82466e20
	ctx.lr = 0x826574DC;
	sub_82466E20(ctx, base);
	// 826574DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826574E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826574E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826574E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826574F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826574F0 size=108
    let mut pc: u32 = 0x826574F0;
    'dispatch: loop {
        match pc {
            0x826574F0 => {
    //   block [0x826574F0..0x8265755C)
	// 826574F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826574F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826574F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826574FC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657504: 38EBBD68  addi r7, r11, -0x4298
	ctx.r[7].s64 = ctx.r[11].s64 + -17048;
	// 82657508: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265750C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 82657510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657514: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265751C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657520: 386A7138  addi r3, r10, 0x7138
	ctx.r[3].s64 = ctx.r[10].s64 + 28984;
	// 82657524: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265752C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265753C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657548: 4BE0F8D9  bl 0x82466e20
	ctx.lr = 0x8265754C;
	sub_82466E20(ctx, base);
	// 8265754C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657560 size=108
    let mut pc: u32 = 0x82657560;
    'dispatch: loop {
        match pc {
            0x82657560 => {
    //   block [0x82657560..0x826575CC)
	// 82657560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265756C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657574: 38EBBD80  addi r7, r11, -0x4280
	ctx.r[7].s64 = ctx.r[11].s64 + -17024;
	// 82657578: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265757C: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 82657580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657584: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265758C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657590: 386A7168  addi r3, r10, 0x7168
	ctx.r[3].s64 = ctx.r[10].s64 + 29032;
	// 82657594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265759C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826575A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826575A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826575A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826575AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826575B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826575B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826575B8: 4BE0F869  bl 0x82466e20
	ctx.lr = 0x826575BC;
	sub_82466E20(ctx, base);
	// 826575BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826575C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826575C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826575C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826575D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826575D0 size=108
    let mut pc: u32 = 0x826575D0;
    'dispatch: loop {
        match pc {
            0x826575D0 => {
    //   block [0x826575D0..0x8265763C)
	// 826575D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826575D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826575D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826575DC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826575E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826575E4: 38EBBD98  addi r7, r11, -0x4268
	ctx.r[7].s64 = ctx.r[11].s64 + -17000;
	// 826575E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826575EC: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826575F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826575F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826575F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826575FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657600: 386A7198  addi r3, r10, 0x7198
	ctx.r[3].s64 = ctx.r[10].s64 + 29080;
	// 82657604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265760C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265761C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657628: 4BE0F7F9  bl 0x82466e20
	ctx.lr = 0x8265762C;
	sub_82466E20(ctx, base);
	// 8265762C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657640 size=108
    let mut pc: u32 = 0x82657640;
    'dispatch: loop {
        match pc {
            0x82657640 => {
    //   block [0x82657640..0x826576AC)
	// 82657640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265764C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657654: 38EBBDB0  addi r7, r11, -0x4250
	ctx.r[7].s64 = ctx.r[11].s64 + -16976;
	// 82657658: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265765C: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 82657660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657664: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265766C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657670: 386A71C8  addi r3, r10, 0x71c8
	ctx.r[3].s64 = ctx.r[10].s64 + 29128;
	// 82657674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265767C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265768C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657698: 4BE0F789  bl 0x82466e20
	ctx.lr = 0x8265769C;
	sub_82466E20(ctx, base);
	// 8265769C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826576A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826576A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826576A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826576B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826576B0 size=108
    let mut pc: u32 = 0x826576B0;
    'dispatch: loop {
        match pc {
            0x826576B0 => {
    //   block [0x826576B0..0x8265771C)
	// 826576B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826576B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826576B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826576BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826576C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826576C4: 38EBBDC8  addi r7, r11, -0x4238
	ctx.r[7].s64 = ctx.r[11].s64 + -16952;
	// 826576C8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826576CC: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826576D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826576D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826576D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826576DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826576E0: 386A71F8  addi r3, r10, 0x71f8
	ctx.r[3].s64 = ctx.r[10].s64 + 29176;
	// 826576E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826576E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826576EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826576F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826576F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826576F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826576FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657708: 4BE0F719  bl 0x82466e20
	ctx.lr = 0x8265770C;
	sub_82466E20(ctx, base);
	// 8265770C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657720 size=108
    let mut pc: u32 = 0x82657720;
    'dispatch: loop {
        match pc {
            0x82657720 => {
    //   block [0x82657720..0x8265778C)
	// 82657720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265772C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657734: 38EBBE58  addi r7, r11, -0x41a8
	ctx.r[7].s64 = ctx.r[11].s64 + -16808;
	// 82657738: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8265773C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 82657740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657744: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265774C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657750: 386A7228  addi r3, r10, 0x7228
	ctx.r[3].s64 = ctx.r[10].s64 + 29224;
	// 82657754: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265775C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265776C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657778: 4BE0F6A9  bl 0x82466e20
	ctx.lr = 0x8265777C;
	sub_82466E20(ctx, base);
	// 8265777C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657790 size=108
    let mut pc: u32 = 0x82657790;
    'dispatch: loop {
        match pc {
            0x82657790 => {
    //   block [0x82657790..0x826577FC)
	// 82657790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265779C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826577A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826577A4: 38EBBF18  addi r7, r11, -0x40e8
	ctx.r[7].s64 = ctx.r[11].s64 + -16616;
	// 826577A8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826577AC: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826577B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826577B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826577B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826577BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826577C0: 386A7258  addi r3, r10, 0x7258
	ctx.r[3].s64 = ctx.r[10].s64 + 29272;
	// 826577C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826577C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826577CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826577D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826577D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826577D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826577DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826577E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826577E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826577E8: 4BE0F639  bl 0x82466e20
	ctx.lr = 0x826577EC;
	sub_82466E20(ctx, base);
	// 826577EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826577F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826577F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826577F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657800 size=108
    let mut pc: u32 = 0x82657800;
    'dispatch: loop {
        match pc {
            0x82657800 => {
    //   block [0x82657800..0x8265786C)
	// 82657800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265780C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657810: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657814: 38EBBFF0  addi r7, r11, -0x4010
	ctx.r[7].s64 = ctx.r[11].s64 + -16400;
	// 82657818: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8265781C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 82657820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657824: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657828: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265782C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657830: 386A7288  addi r3, r10, 0x7288
	ctx.r[3].s64 = ctx.r[10].s64 + 29320;
	// 82657834: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265783C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265784C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657854: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657858: 4BE0F5C9  bl 0x82466e20
	ctx.lr = 0x8265785C;
	sub_82466E20(ctx, base);
	// 8265785C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657870 size=108
    let mut pc: u32 = 0x82657870;
    'dispatch: loop {
        match pc {
            0x82657870 => {
    //   block [0x82657870..0x826578DC)
	// 82657870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265787C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657884: 38EBC0B0  addi r7, r11, -0x3f50
	ctx.r[7].s64 = ctx.r[11].s64 + -16208;
	// 82657888: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8265788C: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 82657890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657894: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657898: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265789C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826578A0: 386A72B8  addi r3, r10, 0x72b8
	ctx.r[3].s64 = ctx.r[10].s64 + 29368;
	// 826578A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826578A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826578AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826578B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826578B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826578B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826578BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826578C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826578C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826578C8: 4BE0F559  bl 0x82466e20
	ctx.lr = 0x826578CC;
	sub_82466E20(ctx, base);
	// 826578CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826578D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826578D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826578D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826578E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826578E0 size=112
    let mut pc: u32 = 0x826578E0;
    'dispatch: loop {
        match pc {
            0x826578E0 => {
    //   block [0x826578E0..0x82657950)
	// 826578E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826578E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826578E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826578EC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826578F0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826578F4: 38EAC158  addi r7, r10, -0x3ea8
	ctx.r[7].s64 = ctx.r[10].s64 + -16040;
	// 826578F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826578FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82657900: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 82657904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657908: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265790C: 396BD1E0  addi r11, r11, -0x2e20
	ctx.r[11].s64 = ctx.r[11].s64 + -11808;
	// 82657910: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657914: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657918: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265791C: 386A72E8  addi r3, r10, 0x72e8
	ctx.r[3].s64 = ctx.r[10].s64 + 29416;
	// 82657920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657924: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82657928: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265792C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82657930: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657934: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657938: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265793C: 4BE0F4E5  bl 0x82466e20
	ctx.lr = 0x82657940;
	sub_82466E20(ctx, base);
	// 82657940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265794C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657950 size=108
    let mut pc: u32 = 0x82657950;
    'dispatch: loop {
        match pc {
            0x82657950 => {
    //   block [0x82657950..0x826579BC)
	// 82657950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265795C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657960: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657964: 38EBC278  addi r7, r11, -0x3d88
	ctx.r[7].s64 = ctx.r[11].s64 + -15752;
	// 82657968: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8265796C: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 82657970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657974: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657978: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265797C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657980: 386A7318  addi r3, r10, 0x7318
	ctx.r[3].s64 = ctx.r[10].s64 + 29464;
	// 82657984: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265798C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265799C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826579A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826579A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826579A8: 4BE0F479  bl 0x82466e20
	ctx.lr = 0x826579AC;
	sub_82466E20(ctx, base);
	// 826579AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826579B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826579B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826579B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826579C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826579C0 size=108
    let mut pc: u32 = 0x826579C0;
    'dispatch: loop {
        match pc {
            0x826579C0 => {
    //   block [0x826579C0..0x82657A2C)
	// 826579C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826579C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826579C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826579CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826579D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826579D4: 38EBC2D8  addi r7, r11, -0x3d28
	ctx.r[7].s64 = ctx.r[11].s64 + -15656;
	// 826579D8: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826579DC: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826579E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826579E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826579E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826579EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826579F0: 386A7348  addi r3, r10, 0x7348
	ctx.r[3].s64 = ctx.r[10].s64 + 29512;
	// 826579F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826579F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826579FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657A14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657A18: 4BE0F409  bl 0x82466e20
	ctx.lr = 0x82657A1C;
	sub_82466E20(ctx, base);
	// 82657A1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657A30 size=108
    let mut pc: u32 = 0x82657A30;
    'dispatch: loop {
        match pc {
            0x82657A30 => {
    //   block [0x82657A30..0x82657A9C)
	// 82657A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657A3C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657A40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657A44: 38EBC3E0  addi r7, r11, -0x3c20
	ctx.r[7].s64 = ctx.r[11].s64 + -15392;
	// 82657A48: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82657A4C: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 82657A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657A54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657A58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657A60: 386A7378  addi r3, r10, 0x7378
	ctx.r[3].s64 = ctx.r[10].s64 + 29560;
	// 82657A64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657A68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657A84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657A88: 4BE0F399  bl 0x82466e20
	ctx.lr = 0x82657A8C;
	sub_82466E20(ctx, base);
	// 82657A8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657AA0 size=108
    let mut pc: u32 = 0x82657AA0;
    'dispatch: loop {
        match pc {
            0x82657AA0 => {
    //   block [0x82657AA0..0x82657B0C)
	// 82657AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657AAC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657AB4: 38EBC4B8  addi r7, r11, -0x3b48
	ctx.r[7].s64 = ctx.r[11].s64 + -15176;
	// 82657AB8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82657ABC: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 82657AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657AC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657AC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657AD0: 386A73A8  addi r3, r10, 0x73a8
	ctx.r[3].s64 = ctx.r[10].s64 + 29608;
	// 82657AD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657AF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657AF8: 4BE0F329  bl 0x82466e20
	ctx.lr = 0x82657AFC;
	sub_82466E20(ctx, base);
	// 82657AFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657B10 size=108
    let mut pc: u32 = 0x82657B10;
    'dispatch: loop {
        match pc {
            0x82657B10 => {
    //   block [0x82657B10..0x82657B7C)
	// 82657B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657B1C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657B24: 38EBC500  addi r7, r11, -0x3b00
	ctx.r[7].s64 = ctx.r[11].s64 + -15104;
	// 82657B28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82657B2C: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 82657B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657B34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657B38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657B40: 386A73D8  addi r3, r10, 0x73d8
	ctx.r[3].s64 = ctx.r[10].s64 + 29656;
	// 82657B44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657B68: 4BE0F2B9  bl 0x82466e20
	ctx.lr = 0x82657B6C;
	sub_82466E20(ctx, base);
	// 82657B6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657B80 size=112
    let mut pc: u32 = 0x82657B80;
    'dispatch: loop {
        match pc {
            0x82657B80 => {
    //   block [0x82657B80..0x82657BF0)
	// 82657B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657B8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657B90: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657B94: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82657B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657B9C: 390BC518  addi r8, r11, -0x3ae8
	ctx.r[8].s64 = ctx.r[11].s64 + -15080;
	// 82657BA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82657BA4: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 82657BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657BAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657BB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82657BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657BB8: 386A7408  addi r3, r10, 0x7408
	ctx.r[3].s64 = ctx.r[10].s64 + 29704;
	// 82657BBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82657BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657BDC: 4BE0F245  bl 0x82466e20
	ctx.lr = 0x82657BE0;
	sub_82466E20(ctx, base);
	// 82657BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657BF0 size=112
    let mut pc: u32 = 0x82657BF0;
    'dispatch: loop {
        match pc {
            0x82657BF0 => {
    //   block [0x82657BF0..0x82657C60)
	// 82657BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657BFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657C00: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657C04: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82657C08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657C0C: 390BC578  addi r8, r11, -0x3a88
	ctx.r[8].s64 = ctx.r[11].s64 + -14984;
	// 82657C10: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82657C14: 388A3824  addi r4, r10, 0x3824
	ctx.r[4].s64 = ctx.r[10].s64 + 14372;
	// 82657C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657C1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657C20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82657C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657C28: 386A7438  addi r3, r10, 0x7438
	ctx.r[3].s64 = ctx.r[10].s64 + 29752;
	// 82657C2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82657C30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657C44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657C4C: 4BE0F1D5  bl 0x82466e20
	ctx.lr = 0x82657C50;
	sub_82466E20(ctx, base);
	// 82657C50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657C60 size=108
    let mut pc: u32 = 0x82657C60;
    'dispatch: loop {
        match pc {
            0x82657C60 => {
    //   block [0x82657C60..0x82657CCC)
	// 82657C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657C6C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657C70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657C74: 38EBC5C0  addi r7, r11, -0x3a40
	ctx.r[7].s64 = ctx.r[11].s64 + -14912;
	// 82657C78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82657C7C: 388A3838  addi r4, r10, 0x3838
	ctx.r[4].s64 = ctx.r[10].s64 + 14392;
	// 82657C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657C84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657C88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657C90: 386A7468  addi r3, r10, 0x7468
	ctx.r[3].s64 = ctx.r[10].s64 + 29800;
	// 82657C94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657C98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657CB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657CB8: 4BE0F169  bl 0x82466e20
	ctx.lr = 0x82657CBC;
	sub_82466E20(ctx, base);
	// 82657CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82657CD0 size=24
    let mut pc: u32 = 0x82657CD0;
    'dispatch: loop {
        match pc {
            0x82657CD0 => {
    //   block [0x82657CD0..0x82657CE8)
	// 82657CD0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657CD4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82657CD8: 394A1B78  addi r10, r10, 0x1b78
	ctx.r[10].s64 = ctx.r[10].s64 + 7032;
	// 82657CDC: 816BBB84  lwz r11, -0x447c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17532 as u32) ) } as u64;
	// 82657CE0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82657CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657CE8 size=112
    let mut pc: u32 = 0x82657CE8;
    'dispatch: loop {
        match pc {
            0x82657CE8 => {
    //   block [0x82657CE8..0x82657D58)
	// 82657CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657CF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657CF8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657CFC: 38AA7678  addi r5, r10, 0x7678
	ctx.r[5].s64 = ctx.r[10].s64 + 30328;
	// 82657D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657D04: 390B1B78  addi r8, r11, 0x1b78
	ctx.r[8].s64 = ctx.r[11].s64 + 7032;
	// 82657D08: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82657D0C: 388A3888  addi r4, r10, 0x3888
	ctx.r[4].s64 = ctx.r[10].s64 + 14472;
	// 82657D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657D14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82657D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657D20: 386A7498  addi r3, r10, 0x7498
	ctx.r[3].s64 = ctx.r[10].s64 + 29848;
	// 82657D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82657D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657D44: 4BE0F0DD  bl 0x82466e20
	ctx.lr = 0x82657D48;
	sub_82466E20(ctx, base);
	// 82657D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657D58 size=108
    let mut pc: u32 = 0x82657D58;
    'dispatch: loop {
        match pc {
            0x82657D58 => {
    //   block [0x82657D58..0x82657DC4)
	// 82657D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657D64: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657D6C: 38EBC5D8  addi r7, r11, -0x3a28
	ctx.r[7].s64 = ctx.r[11].s64 + -14888;
	// 82657D70: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82657D74: 388A38A0  addi r4, r10, 0x38a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14496;
	// 82657D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657D7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657D88: 386A74C8  addi r3, r10, 0x74c8
	ctx.r[3].s64 = ctx.r[10].s64 + 29896;
	// 82657D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657DB0: 4BE0F071  bl 0x82466e20
	ctx.lr = 0x82657DB4;
	sub_82466E20(ctx, base);
	// 82657DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657DC8 size=112
    let mut pc: u32 = 0x82657DC8;
    'dispatch: loop {
        match pc {
            0x82657DC8 => {
    //   block [0x82657DC8..0x82657E38)
	// 82657DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657DD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657DD8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657DDC: 38AA7678  addi r5, r10, 0x7678
	ctx.r[5].s64 = ctx.r[10].s64 + 30328;
	// 82657DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657DE4: 390BC638  addi r8, r11, -0x39c8
	ctx.r[8].s64 = ctx.r[11].s64 + -14792;
	// 82657DE8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82657DEC: 388A38BC  addi r4, r10, 0x38bc
	ctx.r[4].s64 = ctx.r[10].s64 + 14524;
	// 82657DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657DF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82657DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657E00: 386A74F8  addi r3, r10, 0x74f8
	ctx.r[3].s64 = ctx.r[10].s64 + 29944;
	// 82657E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82657E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657E24: 4BE0EFFD  bl 0x82466e20
	ctx.lr = 0x82657E28;
	sub_82466E20(ctx, base);
	// 82657E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657E38 size=108
    let mut pc: u32 = 0x82657E38;
    'dispatch: loop {
        match pc {
            0x82657E38 => {
    //   block [0x82657E38..0x82657EA4)
	// 82657E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657E44: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657E4C: 38EBC6F8  addi r7, r11, -0x3908
	ctx.r[7].s64 = ctx.r[11].s64 + -14600;
	// 82657E50: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82657E54: 388A38D0  addi r4, r10, 0x38d0
	ctx.r[4].s64 = ctx.r[10].s64 + 14544;
	// 82657E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657E5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657E60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657E68: 386A7528  addi r3, r10, 0x7528
	ctx.r[3].s64 = ctx.r[10].s64 + 29992;
	// 82657E6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657E90: 4BE0EF91  bl 0x82466e20
	ctx.lr = 0x82657E94;
	sub_82466E20(ctx, base);
	// 82657E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657EA8 size=108
    let mut pc: u32 = 0x82657EA8;
    'dispatch: loop {
        match pc {
            0x82657EA8 => {
    //   block [0x82657EA8..0x82657F14)
	// 82657EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657EB4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657EBC: 38EBC770  addi r7, r11, -0x3890
	ctx.r[7].s64 = ctx.r[11].s64 + -14480;
	// 82657EC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82657EC4: 388A38E0  addi r4, r10, 0x38e0
	ctx.r[4].s64 = ctx.r[10].s64 + 14560;
	// 82657EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657ECC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657ED8: 386A7558  addi r3, r10, 0x7558
	ctx.r[3].s64 = ctx.r[10].s64 + 30040;
	// 82657EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657F00: 4BE0EF21  bl 0x82466e20
	ctx.lr = 0x82657F04;
	sub_82466E20(ctx, base);
	// 82657F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657F18 size=108
    let mut pc: u32 = 0x82657F18;
    'dispatch: loop {
        match pc {
            0x82657F18 => {
    //   block [0x82657F18..0x82657F84)
	// 82657F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657F24: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657F2C: 38EBC7B8  addi r7, r11, -0x3848
	ctx.r[7].s64 = ctx.r[11].s64 + -14408;
	// 82657F30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82657F34: 388A3904  addi r4, r10, 0x3904
	ctx.r[4].s64 = ctx.r[10].s64 + 14596;
	// 82657F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657F3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657F40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82657F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657F48: 386A7588  addi r3, r10, 0x7588
	ctx.r[3].s64 = ctx.r[10].s64 + 30088;
	// 82657F4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82657F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82657F70: 4BE0EEB1  bl 0x82466e20
	ctx.lr = 0x82657F74;
	sub_82466E20(ctx, base);
	// 82657F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657F88 size=112
    let mut pc: u32 = 0x82657F88;
    'dispatch: loop {
        match pc {
            0x82657F88 => {
    //   block [0x82657F88..0x82657FF8)
	// 82657F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82657F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82657F94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657F98: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82657F9C: 38AA7588  addi r5, r10, 0x7588
	ctx.r[5].s64 = ctx.r[10].s64 + 30088;
	// 82657FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82657FA4: 390BC800  addi r8, r11, -0x3800
	ctx.r[8].s64 = ctx.r[11].s64 + -14336;
	// 82657FA8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82657FAC: 388A3928  addi r4, r10, 0x3928
	ctx.r[4].s64 = ctx.r[10].s64 + 14632;
	// 82657FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82657FB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82657FB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82657FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82657FC0: 386A75B8  addi r3, r10, 0x75b8
	ctx.r[3].s64 = ctx.r[10].s64 + 30136;
	// 82657FC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82657FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82657FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82657FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82657FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82657FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82657FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82657FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82657FE4: 4BE0EE3D  bl 0x82466e20
	ctx.lr = 0x82657FE8;
	sub_82466E20(ctx, base);
	// 82657FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82657FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82657FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82657FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82657FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82657FF8 size=108
    let mut pc: u32 = 0x82657FF8;
    'dispatch: loop {
        match pc {
            0x82657FF8 => {
    //   block [0x82657FF8..0x82658064)
	// 82657FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82657FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658004: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265800C: 38EBC878  addi r7, r11, -0x3788
	ctx.r[7].s64 = ctx.r[11].s64 + -14216;
	// 82658010: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82658014: 388A394C  addi r4, r10, 0x394c
	ctx.r[4].s64 = ctx.r[10].s64 + 14668;
	// 82658018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265801C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82658024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658028: 386A75E8  addi r3, r10, 0x75e8
	ctx.r[3].s64 = ctx.r[10].s64 + 30184;
	// 8265802C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82658030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265803C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265804C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658050: 4BE0EDD1  bl 0x82466e20
	ctx.lr = 0x82658054;
	sub_82466E20(ctx, base);
	// 82658054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265805C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658068 size=108
    let mut pc: u32 = 0x82658068;
    'dispatch: loop {
        match pc {
            0x82658068 => {
    //   block [0x82658068..0x826580D4)
	// 82658068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265806C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658074: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265807C: 38EBC8C0  addi r7, r11, -0x3740
	ctx.r[7].s64 = ctx.r[11].s64 + -14144;
	// 82658080: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82658084: 388A3968  addi r4, r10, 0x3968
	ctx.r[4].s64 = ctx.r[10].s64 + 14696;
	// 82658088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265808C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82658094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658098: 386A7618  addi r3, r10, 0x7618
	ctx.r[3].s64 = ctx.r[10].s64 + 30232;
	// 8265809C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826580A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826580A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826580A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826580AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826580B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826580B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826580B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826580BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826580C0: 4BE0ED61  bl 0x82466e20
	ctx.lr = 0x826580C4;
	sub_82466E20(ctx, base);
	// 826580C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826580C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826580CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826580D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826580D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826580D8 size=108
    let mut pc: u32 = 0x826580D8;
    'dispatch: loop {
        match pc {
            0x826580D8 => {
    //   block [0x826580D8..0x82658144)
	// 826580D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826580DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826580E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826580E4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826580E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826580EC: 38EBC980  addi r7, r11, -0x3680
	ctx.r[7].s64 = ctx.r[11].s64 + -13952;
	// 826580F0: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 826580F4: 388A3980  addi r4, r10, 0x3980
	ctx.r[4].s64 = ctx.r[10].s64 + 14720;
	// 826580F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826580FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82658104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658108: 386A7648  addi r3, r10, 0x7648
	ctx.r[3].s64 = ctx.r[10].s64 + 30280;
	// 8265810C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82658110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265811C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265812C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658130: 4BE0ECF1  bl 0x82466e20
	ctx.lr = 0x82658134;
	sub_82466E20(ctx, base);
	// 82658134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265813C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658148 size=112
    let mut pc: u32 = 0x82658148;
    'dispatch: loop {
        match pc {
            0x82658148 => {
    //   block [0x82658148..0x826581B8)
	// 82658148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265814C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658154: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658158: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265815C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82658160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658164: 390BCB00  addi r8, r11, -0x3500
	ctx.r[8].s64 = ctx.r[11].s64 + -13568;
	// 82658168: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265816C: 388A3990  addi r4, r10, 0x3990
	ctx.r[4].s64 = ctx.r[10].s64 + 14736;
	// 82658170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658174: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658178: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265817C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658180: 386A7678  addi r3, r10, 0x7678
	ctx.r[3].s64 = ctx.r[10].s64 + 30328;
	// 82658184: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265818C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265819C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826581A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826581A4: 4BE0EC7D  bl 0x82466e20
	ctx.lr = 0x826581A8;
	sub_82466E20(ctx, base);
	// 826581A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826581AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826581B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826581B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826581B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826581B8 size=108
    let mut pc: u32 = 0x826581B8;
    'dispatch: loop {
        match pc {
            0x826581B8 => {
    //   block [0x826581B8..0x82658224)
	// 826581B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826581BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826581C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826581C4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826581C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826581CC: 38EBCB60  addi r7, r11, -0x34a0
	ctx.r[7].s64 = ctx.r[11].s64 + -13472;
	// 826581D0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826581D4: 388A39A0  addi r4, r10, 0x39a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14752;
	// 826581D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826581DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826581E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826581E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826581E8: 386A76A8  addi r3, r10, 0x76a8
	ctx.r[3].s64 = ctx.r[10].s64 + 30376;
	// 826581EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826581F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826581F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826581F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826581FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265820C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658210: 4BE0EC11  bl 0x82466e20
	ctx.lr = 0x82658214;
	sub_82466E20(ctx, base);
	// 82658214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265821C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658228 size=112
    let mut pc: u32 = 0x82658228;
    'dispatch: loop {
        match pc {
            0x82658228 => {
    //   block [0x82658228..0x82658298)
	// 82658228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265822C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658234: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658238: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265823C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82658240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658244: 390BCBD8  addi r8, r11, -0x3428
	ctx.r[8].s64 = ctx.r[11].s64 + -13352;
	// 82658248: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265824C: 388A39B4  addi r4, r10, 0x39b4
	ctx.r[4].s64 = ctx.r[10].s64 + 14772;
	// 82658250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658254: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265825C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658260: 386A76D8  addi r3, r10, 0x76d8
	ctx.r[3].s64 = ctx.r[10].s64 + 30424;
	// 82658264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265826C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265827C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658284: 4BE0EB9D  bl 0x82466e20
	ctx.lr = 0x82658288;
	sub_82466E20(ctx, base);
	// 82658288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265828C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658298 size=108
    let mut pc: u32 = 0x82658298;
    'dispatch: loop {
        match pc {
            0x82658298 => {
    //   block [0x82658298..0x82658304)
	// 82658298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265829C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826582A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826582A4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826582A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826582AC: 38EBCC20  addi r7, r11, -0x33e0
	ctx.r[7].s64 = ctx.r[11].s64 + -13280;
	// 826582B0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826582B4: 388A39C0  addi r4, r10, 0x39c0
	ctx.r[4].s64 = ctx.r[10].s64 + 14784;
	// 826582B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826582BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826582C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826582C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826582C8: 386A7708  addi r3, r10, 0x7708
	ctx.r[3].s64 = ctx.r[10].s64 + 30472;
	// 826582CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826582D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826582D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826582D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826582DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826582E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826582E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826582E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826582EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826582F0: 4BE0EB31  bl 0x82466e20
	ctx.lr = 0x826582F4;
	sub_82466E20(ctx, base);
	// 826582F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826582F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826582FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658308 size=108
    let mut pc: u32 = 0x82658308;
    'dispatch: loop {
        match pc {
            0x82658308 => {
    //   block [0x82658308..0x82658374)
	// 82658308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265830C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658314: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658318: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265831C: 38EBCC80  addi r7, r11, -0x3380
	ctx.r[7].s64 = ctx.r[11].s64 + -13184;
	// 82658320: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82658324: 388A0DBC  addi r4, r10, 0xdbc
	ctx.r[4].s64 = ctx.r[10].s64 + 3516;
	// 82658328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265832C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82658334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658338: 386A7738  addi r3, r10, 0x7738
	ctx.r[3].s64 = ctx.r[10].s64 + 30520;
	// 8265833C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82658340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265834C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265835C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658360: 4BE0EAC1  bl 0x82466e20
	ctx.lr = 0x82658364;
	sub_82466E20(ctx, base);
	// 82658364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265836C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658378 size=108
    let mut pc: u32 = 0x82658378;
    'dispatch: loop {
        match pc {
            0x82658378 => {
    //   block [0x82658378..0x826583E4)
	// 82658378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265837C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658384: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265838C: 38EBCCC8  addi r7, r11, -0x3338
	ctx.r[7].s64 = ctx.r[11].s64 + -13112;
	// 82658390: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82658394: 388A39D8  addi r4, r10, 0x39d8
	ctx.r[4].s64 = ctx.r[10].s64 + 14808;
	// 82658398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265839C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826583A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826583A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826583A8: 386A7768  addi r3, r10, 0x7768
	ctx.r[3].s64 = ctx.r[10].s64 + 30568;
	// 826583AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826583B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826583B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826583B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826583BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826583C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826583C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826583C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826583CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826583D0: 4BE0EA51  bl 0x82466e20
	ctx.lr = 0x826583D4;
	sub_82466E20(ctx, base);
	// 826583D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826583D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826583DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826583E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826583E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826583E8 size=108
    let mut pc: u32 = 0x826583E8;
    'dispatch: loop {
        match pc {
            0x826583E8 => {
    //   block [0x826583E8..0x82658454)
	// 826583E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826583EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826583F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826583F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826583F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826583FC: 38EBCD88  addi r7, r11, -0x3278
	ctx.r[7].s64 = ctx.r[11].s64 + -12920;
	// 82658400: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82658404: 388A39F4  addi r4, r10, 0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14836;
	// 82658408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265840C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658410: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82658414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658418: 386A7798  addi r3, r10, 0x7798
	ctx.r[3].s64 = ctx.r[10].s64 + 30616;
	// 8265841C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82658420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265842C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265843C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658440: 4BE0E9E1  bl 0x82466e20
	ctx.lr = 0x82658444;
	sub_82466E20(ctx, base);
	// 82658444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265844C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658458 size=108
    let mut pc: u32 = 0x82658458;
    'dispatch: loop {
        match pc {
            0x82658458 => {
    //   block [0x82658458..0x826584C4)
	// 82658458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265845C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658464: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265846C: 38EBCE18  addi r7, r11, -0x31e8
	ctx.r[7].s64 = ctx.r[11].s64 + -12776;
	// 82658470: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 82658474: 388A3A14  addi r4, r10, 0x3a14
	ctx.r[4].s64 = ctx.r[10].s64 + 14868;
	// 82658478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265847C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658480: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82658484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658488: 386A77C8  addi r3, r10, 0x77c8
	ctx.r[3].s64 = ctx.r[10].s64 + 30664;
	// 8265848C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82658490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265849C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826584A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826584A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826584A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826584AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826584B0: 4BE0E971  bl 0x82466e20
	ctx.lr = 0x826584B4;
	sub_82466E20(ctx, base);
	// 826584B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826584B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826584BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826584C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826584C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826584C8 size=108
    let mut pc: u32 = 0x826584C8;
    'dispatch: loop {
        match pc {
            0x826584C8 => {
    //   block [0x826584C8..0x82658534)
	// 826584C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826584CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826584D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826584D4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826584D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826584DC: 38EBCF50  addi r7, r11, -0x30b0
	ctx.r[7].s64 = ctx.r[11].s64 + -12464;
	// 826584E0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826584E4: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 826584E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826584EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826584F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826584F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826584F8: 386A77F8  addi r3, r10, 0x77f8
	ctx.r[3].s64 = ctx.r[10].s64 + 30712;
	// 826584FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82658500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265850C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265851C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658520: 4BE0E901  bl 0x82466e20
	ctx.lr = 0x82658524;
	sub_82466E20(ctx, base);
	// 82658524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265852C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658538 size=116
    let mut pc: u32 = 0x82658538;
    'dispatch: loop {
        match pc {
            0x82658538 => {
    //   block [0x82658538..0x826585AC)
	// 82658538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265853C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658544: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658548: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265854C: 390BCFB8  addi r8, r11, -0x3048
	ctx.r[8].s64 = ctx.r[11].s64 + -12360;
	// 82658550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658554: 392AD294  addi r9, r10, -0x2d6c
	ctx.r[9].s64 = ctx.r[10].s64 + -11628;
	// 82658558: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265855C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82658560: 38AA77F8  addi r5, r10, 0x77f8
	ctx.r[5].s64 = ctx.r[10].s64 + 30712;
	// 82658564: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265856C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265857C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82658580: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 82658584: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82658588: 386B7828  addi r3, r11, 0x7828
	ctx.r[3].s64 = ctx.r[11].s64 + 30760;
	// 8265858C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82658590: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658598: 4BE0E889  bl 0x82466e20
	ctx.lr = 0x8265859C;
	sub_82466E20(ctx, base);
	// 8265859C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826585A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826585A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826585A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826585B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826585B0 size=96
    let mut pc: u32 = 0x826585B0;
    'dispatch: loop {
        match pc {
            0x826585B0 => {
    //   block [0x826585B0..0x82658610)
	// 826585B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826585B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826585B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826585BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826585C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826585C4: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 826585C8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826585CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826585D0: 386A7858  addi r3, r10, 0x7858
	ctx.r[3].s64 = ctx.r[10].s64 + 30808;
	// 826585D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826585D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826585DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826585E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826585E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826585E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826585EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826585F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826585F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826585F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826585FC: 4BE0E825  bl 0x82466e20
	ctx.lr = 0x82658600;
	sub_82466E20(ctx, base);
	// 82658600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265860C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658610 size=112
    let mut pc: u32 = 0x82658610;
    'dispatch: loop {
        match pc {
            0x82658610 => {
    //   block [0x82658610..0x82658680)
	// 82658610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265861C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82658620: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658624: 38AA9928  addi r5, r10, -0x66d8
	ctx.r[5].s64 = ctx.r[10].s64 + -26328;
	// 82658628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265862C: 390BD030  addi r8, r11, -0x2fd0
	ctx.r[8].s64 = ctx.r[11].s64 + -12240;
	// 82658630: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82658634: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 82658638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265863C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658648: 386A7888  addi r3, r10, 0x7888
	ctx.r[3].s64 = ctx.r[10].s64 + 30856;
	// 8265864C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265865C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265866C: 4BE0E7B5  bl 0x82466e20
	ctx.lr = 0x82658670;
	sub_82466E20(ctx, base);
	// 82658670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265867C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658680 size=96
    let mut pc: u32 = 0x82658680;
    'dispatch: loop {
        match pc {
            0x82658680 => {
    //   block [0x82658680..0x826586E0)
	// 82658680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265868C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658694: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82658698: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265869C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826586A0: 386A78B8  addi r3, r10, 0x78b8
	ctx.r[3].s64 = ctx.r[10].s64 + 30904;
	// 826586A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826586A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826586AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826586B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826586B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826586B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826586BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826586C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826586C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826586C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826586CC: 4BE0E755  bl 0x82466e20
	ctx.lr = 0x826586D0;
	sub_82466E20(ctx, base);
	// 826586D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826586D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826586D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826586DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826586E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826586E0 size=24
    let mut pc: u32 = 0x826586E0;
    'dispatch: loop {
        match pc {
            0x826586E0 => {
    //   block [0x826586E0..0x826586F8)
	// 826586E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826586E4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826586E8: 394A1C38  addi r10, r10, 0x1c38
	ctx.r[10].s64 = ctx.r[10].s64 + 7224;
	// 826586EC: 816BCFB4  lwz r11, -0x304c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12364 as u32) ) } as u64;
	// 826586F0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826586F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826586F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826586F8 size=116
    let mut pc: u32 = 0x826586F8;
    'dispatch: loop {
        match pc {
            0x826586F8 => {
    //   block [0x826586F8..0x8265876C)
	// 826586F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826586FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658704: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658708: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265870C: 390B1C38  addi r8, r11, 0x1c38
	ctx.r[8].s64 = ctx.r[11].s64 + 7224;
	// 82658710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658714: 392AD2D0  addi r9, r10, -0x2d30
	ctx.r[9].s64 = ctx.r[10].s64 + -11568;
	// 82658718: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265871C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82658720: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82658724: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265872C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658734: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82658738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265873C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82658740: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82658744: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82658748: 386B78E8  addi r3, r11, 0x78e8
	ctx.r[3].s64 = ctx.r[11].s64 + 30952;
	// 8265874C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82658750: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658758: 4BE0E6C9  bl 0x82466e20
	ctx.lr = 0x8265875C;
	sub_82466E20(ctx, base);
	// 8265875C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658770 size=104
    let mut pc: u32 = 0x82658770;
    'dispatch: loop {
        match pc {
            0x82658770 => {
    //   block [0x82658770..0x826587D8)
	// 82658770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265877C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82658780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658784: 392AD2FC  addi r9, r10, -0x2d04
	ctx.r[9].s64 = ctx.r[10].s64 + -11524;
	// 82658788: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265878C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658790: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82658794: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265879C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826587A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826587A4: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 826587A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826587AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826587B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826587B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826587B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826587BC: 386A7918  addi r3, r10, 0x7918
	ctx.r[3].s64 = ctx.r[10].s64 + 31000;
	// 826587C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826587C4: 4BE0E65D  bl 0x82466e20
	ctx.lr = 0x826587C8;
	sub_82466E20(ctx, base);
	// 826587C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826587CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826587D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826587D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826587D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826587D8 size=96
    let mut pc: u32 = 0x826587D8;
    'dispatch: loop {
        match pc {
            0x826587D8 => {
    //   block [0x826587D8..0x82658838)
	// 826587D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826587DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826587E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826587E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826587E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826587EC: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826587F0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826587F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826587F8: 386A7948  addi r3, r10, 0x7948
	ctx.r[3].s64 = ctx.r[10].s64 + 31048;
	// 826587FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658804: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82658808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265880C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658818: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265881C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658820: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82658824: 4BE0E5FD  bl 0x82466e20
	ctx.lr = 0x82658828;
	sub_82466E20(ctx, base);
	// 82658828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265882C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658838 size=100
    let mut pc: u32 = 0x82658838;
    'dispatch: loop {
        match pc {
            0x82658838 => {
    //   block [0x82658838..0x8265889C)
	// 82658838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265883C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658844: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265884C: 38AA7918  addi r5, r10, 0x7918
	ctx.r[5].s64 = ctx.r[10].s64 + 31000;
	// 82658850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658858: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 8265885C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265886C: 386A7978  addi r3, r10, 0x7978
	ctx.r[3].s64 = ctx.r[10].s64 + 31096;
	// 82658870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658874: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658878: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265887C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658880: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82658884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658888: 4BE0E599  bl 0x82466e20
	ctx.lr = 0x8265888C;
	sub_82466E20(ctx, base);
	// 8265888C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826588A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826588A0 size=112
    let mut pc: u32 = 0x826588A0;
    'dispatch: loop {
        match pc {
            0x826588A0 => {
    //   block [0x826588A0..0x82658910)
	// 826588A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826588A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826588A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826588AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826588B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826588B4: 38AA78E8  addi r5, r10, 0x78e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30952;
	// 826588B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826588BC: 390BD098  addi r8, r11, -0x2f68
	ctx.r[8].s64 = ctx.r[11].s64 + -12136;
	// 826588C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826588C4: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 826588C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826588CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826588D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826588D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826588D8: 386A79A8  addi r3, r10, 0x79a8
	ctx.r[3].s64 = ctx.r[10].s64 + 31144;
	// 826588DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826588E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826588E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826588E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826588EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826588F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826588F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826588F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826588FC: 4BE0E525  bl 0x82466e20
	ctx.lr = 0x82658900;
	sub_82466E20(ctx, base);
	// 82658900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265890C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658910 size=112
    let mut pc: u32 = 0x82658910;
    'dispatch: loop {
        match pc {
            0x82658910 => {
    //   block [0x82658910..0x82658980)
	// 82658910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265891C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658920: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658924: 38AA78E8  addi r5, r10, 0x78e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30952;
	// 82658928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265892C: 390BD0E0  addi r8, r11, -0x2f20
	ctx.r[8].s64 = ctx.r[11].s64 + -12064;
	// 82658930: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82658934: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 82658938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265893C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658940: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658948: 386A79D8  addi r3, r10, 0x79d8
	ctx.r[3].s64 = ctx.r[10].s64 + 31192;
	// 8265894C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265895C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265896C: 4BE0E4B5  bl 0x82466e20
	ctx.lr = 0x82658970;
	sub_82466E20(ctx, base);
	// 82658970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265897C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658980 size=100
    let mut pc: u32 = 0x82658980;
    'dispatch: loop {
        match pc {
            0x82658980 => {
    //   block [0x82658980..0x826589E4)
	// 82658980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265898C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658994: 38AA78E8  addi r5, r10, 0x78e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30952;
	// 82658998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265899C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826589A0: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 826589A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826589A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826589AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826589B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826589B4: 386A7A08  addi r3, r10, 0x7a08
	ctx.r[3].s64 = ctx.r[10].s64 + 31240;
	// 826589B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826589BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826589C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826589C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826589C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826589CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826589D0: 4BE0E451  bl 0x82466e20
	ctx.lr = 0x826589D4;
	sub_82466E20(ctx, base);
	// 826589D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826589D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826589DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826589E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826589E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826589E8 size=96
    let mut pc: u32 = 0x826589E8;
    'dispatch: loop {
        match pc {
            0x826589E8 => {
    //   block [0x826589E8..0x82658A48)
	// 826589E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826589EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826589F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826589F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826589F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826589FC: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82658A00: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658A08: 386A7A38  addi r3, r10, 0x7a38
	ctx.r[3].s64 = ctx.r[10].s64 + 31288;
	// 82658A0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658A10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658A14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82658A18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658A20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658A28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82658A2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658A30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82658A34: 4BE0E3ED  bl 0x82466e20
	ctx.lr = 0x82658A38;
	sub_82466E20(ctx, base);
	// 82658A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658A48 size=112
    let mut pc: u32 = 0x82658A48;
    'dispatch: loop {
        match pc {
            0x82658A48 => {
    //   block [0x82658A48..0x82658AB8)
	// 82658A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658A54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658A58: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658A5C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82658A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658A64: 390BD0F8  addi r8, r11, -0x2f08
	ctx.r[8].s64 = ctx.r[11].s64 + -12040;
	// 82658A68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82658A6C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 82658A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658A74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658A78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658A80: 386A7A68  addi r3, r10, 0x7a68
	ctx.r[3].s64 = ctx.r[10].s64 + 31336;
	// 82658A84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658AA4: 4BE0E37D  bl 0x82466e20
	ctx.lr = 0x82658AA8;
	sub_82466E20(ctx, base);
	// 82658AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658AB8 size=96
    let mut pc: u32 = 0x82658AB8;
    'dispatch: loop {
        match pc {
            0x82658AB8 => {
    //   block [0x82658AB8..0x82658B18)
	// 82658AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658AC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658ACC: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 82658AD0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658AD8: 386A7A98  addi r3, r10, 0x7a98
	ctx.r[3].s64 = ctx.r[10].s64 + 31384;
	// 82658ADC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658AE4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82658AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658AF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82658AFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658B00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82658B04: 4BE0E31D  bl 0x82466e20
	ctx.lr = 0x82658B08;
	sub_82466E20(ctx, base);
	// 82658B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658B18 size=112
    let mut pc: u32 = 0x82658B18;
    'dispatch: loop {
        match pc {
            0x82658B18 => {
    //   block [0x82658B18..0x82658B88)
	// 82658B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658B24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658B28: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658B2C: 38AA7A98  addi r5, r10, 0x7a98
	ctx.r[5].s64 = ctx.r[10].s64 + 31384;
	// 82658B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658B34: 390BD128  addi r8, r11, -0x2ed8
	ctx.r[8].s64 = ctx.r[11].s64 + -11992;
	// 82658B38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82658B3C: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 82658B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658B44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658B50: 386A7AC8  addi r3, r10, 0x7ac8
	ctx.r[3].s64 = ctx.r[10].s64 + 31432;
	// 82658B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658B74: 4BE0E2AD  bl 0x82466e20
	ctx.lr = 0x82658B78;
	sub_82466E20(ctx, base);
	// 82658B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658B88 size=108
    let mut pc: u32 = 0x82658B88;
    'dispatch: loop {
        match pc {
            0x82658B88 => {
    //   block [0x82658B88..0x82658BF4)
	// 82658B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658B94: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658B9C: 38EBD140  addi r7, r11, -0x2ec0
	ctx.r[7].s64 = ctx.r[11].s64 + -11968;
	// 82658BA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82658BA4: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 82658BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658BAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658BB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82658BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658BB8: 386A7AF8  addi r3, r10, 0x7af8
	ctx.r[3].s64 = ctx.r[10].s64 + 31480;
	// 82658BBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82658BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658BDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82658BE0: 4BE0E241  bl 0x82466e20
	ctx.lr = 0x82658BE4;
	sub_82466E20(ctx, base);
	// 82658BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658BF8 size=112
    let mut pc: u32 = 0x82658BF8;
    'dispatch: loop {
        match pc {
            0x82658BF8 => {
    //   block [0x82658BF8..0x82658C68)
	// 82658BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658C04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658C08: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658C0C: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 82658C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658C14: 390BD1A0  addi r8, r11, -0x2e60
	ctx.r[8].s64 = ctx.r[11].s64 + -11872;
	// 82658C18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82658C1C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 82658C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658C24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658C30: 386A7B28  addi r3, r10, 0x7b28
	ctx.r[3].s64 = ctx.r[10].s64 + 31528;
	// 82658C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658C54: 4BE0E1CD  bl 0x82466e20
	ctx.lr = 0x82658C58;
	sub_82466E20(ctx, base);
	// 82658C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658C68 size=112
    let mut pc: u32 = 0x82658C68;
    'dispatch: loop {
        match pc {
            0x82658C68 => {
    //   block [0x82658C68..0x82658CD8)
	// 82658C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658C74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658C78: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658C7C: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 82658C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658C84: 390BD1B8  addi r8, r11, -0x2e48
	ctx.r[8].s64 = ctx.r[11].s64 + -11848;
	// 82658C88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82658C8C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 82658C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658C94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658CA0: 386A7B58  addi r3, r10, 0x7b58
	ctx.r[3].s64 = ctx.r[10].s64 + 31576;
	// 82658CA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658CC4: 4BE0E15D  bl 0x82466e20
	ctx.lr = 0x82658CC8;
	sub_82466E20(ctx, base);
	// 82658CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658CD8 size=100
    let mut pc: u32 = 0x82658CD8;
    'dispatch: loop {
        match pc {
            0x82658CD8 => {
    //   block [0x82658CD8..0x82658D3C)
	// 82658CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658CE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658CEC: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 82658CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658CF8: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 82658CFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658D0C: 386A7B88  addi r3, r10, 0x7b88
	ctx.r[3].s64 = ctx.r[10].s64 + 31624;
	// 82658D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658D14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658D18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82658D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658D20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82658D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658D28: 4BE0E0F9  bl 0x82466e20
	ctx.lr = 0x82658D2C;
	sub_82466E20(ctx, base);
	// 82658D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658D40 size=116
    let mut pc: u32 = 0x82658D40;
    'dispatch: loop {
        match pc {
            0x82658D40 => {
    //   block [0x82658D40..0x82658DB4)
	// 82658D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658D4C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658D50: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82658D54: 390BD1E8  addi r8, r11, -0x2e18
	ctx.r[8].s64 = ctx.r[11].s64 + -11800;
	// 82658D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658D5C: 392AD328  addi r9, r10, -0x2cd8
	ctx.r[9].s64 = ctx.r[10].s64 + -11480;
	// 82658D60: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658D64: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82658D68: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 82658D6C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658D74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658D84: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82658D88: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 82658D8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82658D90: 386B7BB8  addi r3, r11, 0x7bb8
	ctx.r[3].s64 = ctx.r[11].s64 + 31672;
	// 82658D94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82658D98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658DA0: 4BE0E081  bl 0x82466e20
	ctx.lr = 0x82658DA4;
	sub_82466E20(ctx, base);
	// 82658DA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658DB8 size=112
    let mut pc: u32 = 0x82658DB8;
    'dispatch: loop {
        match pc {
            0x82658DB8 => {
    //   block [0x82658DB8..0x82658E28)
	// 82658DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658DC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658DC8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658DCC: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 82658DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658DD4: 390BD218  addi r8, r11, -0x2de8
	ctx.r[8].s64 = ctx.r[11].s64 + -11752;
	// 82658DD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82658DDC: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 82658DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658DE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658DF0: 386A7BE8  addi r3, r10, 0x7be8
	ctx.r[3].s64 = ctx.r[10].s64 + 31720;
	// 82658DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658E04: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82658E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658E14: 4BE0E00D  bl 0x82466e20
	ctx.lr = 0x82658E18;
	sub_82466E20(ctx, base);
	// 82658E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658E28 size=116
    let mut pc: u32 = 0x82658E28;
    'dispatch: loop {
        match pc {
            0x82658E28 => {
    //   block [0x82658E28..0x82658E9C)
	// 82658E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658E34: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658E38: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82658E3C: 390BD234  addi r8, r11, -0x2dcc
	ctx.r[8].s64 = ctx.r[11].s64 + -11724;
	// 82658E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658E44: 392AD354  addi r9, r10, -0x2cac
	ctx.r[9].s64 = ctx.r[10].s64 + -11436;
	// 82658E48: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82658E4C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82658E50: 38AA8278  addi r5, r10, -0x7d88
	ctx.r[5].s64 = ctx.r[10].s64 + -32136;
	// 82658E54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658E5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658E6C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82658E70: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 82658E74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82658E78: 386B7C18  addi r3, r11, 0x7c18
	ctx.r[3].s64 = ctx.r[11].s64 + 31768;
	// 82658E7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82658E80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658E88: 4BE0DF99  bl 0x82466e20
	ctx.lr = 0x82658E8C;
	sub_82466E20(ctx, base);
	// 82658E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658EA0 size=112
    let mut pc: u32 = 0x82658EA0;
    'dispatch: loop {
        match pc {
            0x82658EA0 => {
    //   block [0x82658EA0..0x82658F10)
	// 82658EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658EAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658EB0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658EB4: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 82658EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658EBC: 390BD250  addi r8, r11, -0x2db0
	ctx.r[8].s64 = ctx.r[11].s64 + -11696;
	// 82658EC0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82658EC4: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 82658EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658ECC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658ED8: 386A7C48  addi r3, r10, 0x7c48
	ctx.r[3].s64 = ctx.r[10].s64 + 31816;
	// 82658EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658EEC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82658EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658EFC: 4BE0DF25  bl 0x82466e20
	ctx.lr = 0x82658F00;
	sub_82466E20(ctx, base);
	// 82658F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658F10 size=112
    let mut pc: u32 = 0x82658F10;
    'dispatch: loop {
        match pc {
            0x82658F10 => {
    //   block [0x82658F10..0x82658F80)
	// 82658F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658F1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658F20: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658F24: 38AA7BE8  addi r5, r10, 0x7be8
	ctx.r[5].s64 = ctx.r[10].s64 + 31720;
	// 82658F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658F2C: 390BD2C8  addi r8, r11, -0x2d38
	ctx.r[8].s64 = ctx.r[11].s64 + -11576;
	// 82658F30: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82658F34: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 82658F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658F3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658F40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658F48: 386A7C78  addi r3, r10, 0x7c78
	ctx.r[3].s64 = ctx.r[10].s64 + 31864;
	// 82658F4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658F6C: 4BE0DEB5  bl 0x82466e20
	ctx.lr = 0x82658F70;
	sub_82466E20(ctx, base);
	// 82658F70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658F80 size=112
    let mut pc: u32 = 0x82658F80;
    'dispatch: loop {
        match pc {
            0x82658F80 => {
    //   block [0x82658F80..0x82658FF0)
	// 82658F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658F8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658F90: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82658F94: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 82658F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82658F9C: 390BD310  addi r8, r11, -0x2cf0
	ctx.r[8].s64 = ctx.r[11].s64 + -11504;
	// 82658FA0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82658FA4: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 82658FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82658FAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82658FB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82658FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82658FB8: 386A7CA8  addi r3, r10, 0x7ca8
	ctx.r[3].s64 = ctx.r[10].s64 + 31912;
	// 82658FBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82658FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82658FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82658FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82658FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82658FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82658FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82658FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82658FDC: 4BE0DE45  bl 0x82466e20
	ctx.lr = 0x82658FE0;
	sub_82466E20(ctx, base);
	// 82658FE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82658FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82658FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82658FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82658FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82658FF0 size=112
    let mut pc: u32 = 0x82658FF0;
    'dispatch: loop {
        match pc {
            0x82658FF0 => {
    //   block [0x82658FF0..0x82659060)
	// 82658FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82658FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82658FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82658FFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659000: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659004: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 82659008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265900C: 390BD358  addi r8, r11, -0x2ca8
	ctx.r[8].s64 = ctx.r[11].s64 + -11432;
	// 82659010: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82659014: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 82659018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265901C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659028: 386A7CD8  addi r3, r10, 0x7cd8
	ctx.r[3].s64 = ctx.r[10].s64 + 31960;
	// 8265902C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265903C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265904C: 4BE0DDD5  bl 0x82466e20
	ctx.lr = 0x82659050;
	sub_82466E20(ctx, base);
	// 82659050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265905C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659060 size=108
    let mut pc: u32 = 0x82659060;
    'dispatch: loop {
        match pc {
            0x82659060 => {
    //   block [0x82659060..0x826590CC)
	// 82659060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265906C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659074: 38EBD3A0  addi r7, r11, -0x2c60
	ctx.r[7].s64 = ctx.r[11].s64 + -11360;
	// 82659078: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265907C: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 82659080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659084: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659088: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265908C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659090: 386A7D08  addi r3, r10, 0x7d08
	ctx.r[3].s64 = ctx.r[10].s64 + 32008;
	// 82659094: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82659098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265909C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826590A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826590A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826590A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826590AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826590B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826590B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826590B8: 4BE0DD69  bl 0x82466e20
	ctx.lr = 0x826590BC;
	sub_82466E20(ctx, base);
	// 826590BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826590C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826590C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826590C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826590D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826590D0 size=112
    let mut pc: u32 = 0x826590D0;
    'dispatch: loop {
        match pc {
            0x826590D0 => {
    //   block [0x826590D0..0x82659140)
	// 826590D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826590D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826590D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826590DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826590E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826590E4: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 826590E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826590EC: 390BD3E8  addi r8, r11, -0x2c18
	ctx.r[8].s64 = ctx.r[11].s64 + -11288;
	// 826590F0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826590F4: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 826590F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826590FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659108: 386A7D38  addi r3, r10, 0x7d38
	ctx.r[3].s64 = ctx.r[10].s64 + 32056;
	// 8265910C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265911C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265912C: 4BE0DCF5  bl 0x82466e20
	ctx.lr = 0x82659130;
	sub_82466E20(ctx, base);
	// 82659130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265913C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659140 size=116
    let mut pc: u32 = 0x82659140;
    'dispatch: loop {
        match pc {
            0x82659140 => {
    //   block [0x82659140..0x826591B4)
	// 82659140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265914C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82659150: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659154: 392BD390  addi r9, r11, -0x2c70
	ctx.r[9].s64 = ctx.r[11].s64 + -11376;
	// 82659158: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 8265915C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659160: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82659164: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82659168: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265916C: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 82659170: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659174: 396BD460  addi r11, r11, -0x2ba0
	ctx.r[11].s64 = ctx.r[11].s64 + -11168;
	// 82659178: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8265917C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659180: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82659184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659188: 386A7D68  addi r3, r10, 0x7d68
	ctx.r[3].s64 = ctx.r[10].s64 + 32104;
	// 8265918C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82659190: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82659194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659198: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8265919C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826591A0: 4BE0DC81  bl 0x82466e20
	ctx.lr = 0x826591A4;
	sub_82466E20(ctx, base);
	// 826591A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826591A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826591AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826591B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826591B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826591B8 size=36
    let mut pc: u32 = 0x826591B8;
    'dispatch: loop {
        match pc {
            0x826591B8 => {
    //   block [0x826591B8..0x826591DC)
	// 826591B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826591BC: 814BD4F8  lwz r10, -0x2b08(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11016 as u32) ) } as u64;
	// 826591C0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826591C4: 396B1C68  addi r11, r11, 0x1c68
	ctx.r[11].s64 = ctx.r[11].s64 + 7272;
	// 826591C8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826591CC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826591D0: 814AD4F0  lwz r10, -0x2b10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-11024 as u32) ) } as u64;
	// 826591D4: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826591D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826591E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826591E0 size=108
    let mut pc: u32 = 0x826591E0;
    'dispatch: loop {
        match pc {
            0x826591E0 => {
    //   block [0x826591E0..0x8265924C)
	// 826591E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826591E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826591E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826591EC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826591F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826591F4: 38EB1C68  addi r7, r11, 0x1c68
	ctx.r[7].s64 = ctx.r[11].s64 + 7272;
	// 826591F8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826591FC: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 82659200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659204: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659208: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265920C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659210: 386A7D98  addi r3, r10, 0x7d98
	ctx.r[3].s64 = ctx.r[10].s64 + 32152;
	// 82659214: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82659218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265921C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265922C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82659238: 4BE0DBE9  bl 0x82466e20
	ctx.lr = 0x8265923C;
	sub_82466E20(ctx, base);
	// 8265923C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82659250 size=24
    let mut pc: u32 = 0x82659250;
    'dispatch: loop {
        match pc {
            0x82659250 => {
    //   block [0x82659250..0x82659268)
	// 82659250: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659254: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659258: 394A1D10  addi r10, r10, 0x1d10
	ctx.r[10].s64 = ctx.r[10].s64 + 7440;
	// 8265925C: 816BD4F0  lwz r11, -0x2b10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11024 as u32) ) } as u64;
	// 82659260: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82659264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659268 size=116
    let mut pc: u32 = 0x82659268;
    'dispatch: loop {
        match pc {
            0x82659268 => {
    //   block [0x82659268..0x826592DC)
	// 82659268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265926C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659274: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659278: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8265927C: 390A1D10  addi r8, r10, 0x1d10
	ctx.r[8].s64 = ctx.r[10].s64 + 7440;
	// 82659280: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659284: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82659288: 38AA7D98  addi r5, r10, 0x7d98
	ctx.r[5].s64 = ctx.r[10].s64 + 32152;
	// 8265928C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659290: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82659294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265929C: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 826592A0: 396BD44C  addi r11, r11, -0x2bb4
	ctx.r[11].s64 = ctx.r[11].s64 + -11188;
	// 826592A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826592A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826592AC: 386A7DC8  addi r3, r10, 0x7dc8
	ctx.r[3].s64 = ctx.r[10].s64 + 32200;
	// 826592B0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826592B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826592B8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826592BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826592C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826592C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826592C8: 4BE0DB59  bl 0x82466e20
	ctx.lr = 0x826592CC;
	sub_82466E20(ctx, base);
	// 826592CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826592D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826592D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826592D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826592E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826592E0 size=112
    let mut pc: u32 = 0x826592E0;
    'dispatch: loop {
        match pc {
            0x826592E0 => {
    //   block [0x826592E0..0x82659350)
	// 826592E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826592E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826592E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826592EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826592F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826592F4: 38AA7D98  addi r5, r10, 0x7d98
	ctx.r[5].s64 = ctx.r[10].s64 + 32152;
	// 826592F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826592FC: 390BD500  addi r8, r11, -0x2b00
	ctx.r[8].s64 = ctx.r[11].s64 + -11008;
	// 82659300: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82659304: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 82659308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265930C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659310: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659318: 386A7DF8  addi r3, r10, 0x7df8
	ctx.r[3].s64 = ctx.r[10].s64 + 32248;
	// 8265931C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265932C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265933C: 4BE0DAE5  bl 0x82466e20
	ctx.lr = 0x82659340;
	sub_82466E20(ctx, base);
	// 82659340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265934C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82659350 size=24
    let mut pc: u32 = 0x82659350;
    'dispatch: loop {
        match pc {
            0x82659350 => {
    //   block [0x82659350..0x82659368)
	// 82659350: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659354: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659358: 394A1E00  addi r10, r10, 0x1e00
	ctx.r[10].s64 = ctx.r[10].s64 + 7680;
	// 8265935C: 816BD654  lwz r11, -0x29ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10668 as u32) ) } as u64;
	// 82659360: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82659364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


