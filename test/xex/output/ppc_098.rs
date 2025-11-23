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


pub fn sub_82683CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683CE0 size=100
    let mut pc: u32 = 0x82683CE0;
    'dispatch: loop {
        match pc {
            0x82683CE0 => {
    //   block [0x82683CE0..0x82683D44)
	// 82683CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683CEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82683CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683CF4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82683CF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683D00: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 82683D04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683D14: 386A9E28  addi r3, r10, -0x61d8
	ctx.r[3].s64 = ctx.r[10].s64 + -25048;
	// 82683D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683D1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683D20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82683D24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683D28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82683D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683D30: 4BDE30F1  bl 0x82466e20
	ctx.lr = 0x82683D34;
	sub_82466E20(ctx, base);
	// 82683D34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683D48 size=112
    let mut pc: u32 = 0x82683D48;
    'dispatch: loop {
        match pc {
            0x82683D48 => {
    //   block [0x82683D48..0x82683DB8)
	// 82683D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683D54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683D58: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683D5C: 38AA9E28  addi r5, r10, -0x61d8
	ctx.r[5].s64 = ctx.r[10].s64 + -25048;
	// 82683D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683D64: 390BDBE0  addi r8, r11, -0x2420
	ctx.r[8].s64 = ctx.r[11].s64 + -9248;
	// 82683D68: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82683D6C: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82683D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683D74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683D78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683D80: 386A9E58  addi r3, r10, -0x61a8
	ctx.r[3].s64 = ctx.r[10].s64 + -25000;
	// 82683D84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683DA4: 4BDE307D  bl 0x82466e20
	ctx.lr = 0x82683DA8;
	sub_82466E20(ctx, base);
	// 82683DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683DB8 size=100
    let mut pc: u32 = 0x82683DB8;
    'dispatch: loop {
        match pc {
            0x82683DB8 => {
    //   block [0x82683DB8..0x82683E1C)
	// 82683DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683DC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82683DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683DCC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82683DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683DD8: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 82683DDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683DEC: 386A9E88  addi r3, r10, -0x6178
	ctx.r[3].s64 = ctx.r[10].s64 + -24952;
	// 82683DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683DF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683DF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82683DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683E00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82683E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683E08: 4BDE3019  bl 0x82466e20
	ctx.lr = 0x82683E0C;
	sub_82466E20(ctx, base);
	// 82683E0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683E20 size=108
    let mut pc: u32 = 0x82683E20;
    'dispatch: loop {
        match pc {
            0x82683E20 => {
    //   block [0x82683E20..0x82683E8C)
	// 82683E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683E2C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683E30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683E34: 38EBDC58  addi r7, r11, -0x23a8
	ctx.r[7].s64 = ctx.r[11].s64 + -9128;
	// 82683E38: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82683E3C: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 82683E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683E44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683E48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82683E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683E50: 386A9EB8  addi r3, r10, -0x6148
	ctx.r[3].s64 = ctx.r[10].s64 + -24904;
	// 82683E54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82683E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683E74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82683E78: 4BDE2FA9  bl 0x82466e20
	ctx.lr = 0x82683E7C;
	sub_82466E20(ctx, base);
	// 82683E7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683E90 size=112
    let mut pc: u32 = 0x82683E90;
    'dispatch: loop {
        match pc {
            0x82683E90 => {
    //   block [0x82683E90..0x82683F00)
	// 82683E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683E9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683EA0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683EA4: 38AA9E88  addi r5, r10, -0x6178
	ctx.r[5].s64 = ctx.r[10].s64 + -24952;
	// 82683EA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683EAC: 390BDCA0  addi r8, r11, -0x2360
	ctx.r[8].s64 = ctx.r[11].s64 + -9056;
	// 82683EB0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82683EB4: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 82683EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683EBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683EC8: 386A9EE8  addi r3, r10, -0x6118
	ctx.r[3].s64 = ctx.r[10].s64 + -24856;
	// 82683ECC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683EEC: 4BDE2F35  bl 0x82466e20
	ctx.lr = 0x82683EF0;
	sub_82466E20(ctx, base);
	// 82683EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683F00 size=100
    let mut pc: u32 = 0x82683F00;
    'dispatch: loop {
        match pc {
            0x82683F00 => {
    //   block [0x82683F00..0x82683F64)
	// 82683F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683F0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82683F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683F14: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82683F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683F20: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 82683F24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683F34: 386A9F18  addi r3, r10, -0x60e8
	ctx.r[3].s64 = ctx.r[10].s64 + -24808;
	// 82683F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683F3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683F40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82683F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683F48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82683F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683F50: 4BDE2ED1  bl 0x82466e20
	ctx.lr = 0x82683F54;
	sub_82466E20(ctx, base);
	// 82683F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683F68 size=100
    let mut pc: u32 = 0x82683F68;
    'dispatch: loop {
        match pc {
            0x82683F68 => {
    //   block [0x82683F68..0x82683FCC)
	// 82683F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683F74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82683F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683F7C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82683F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683F88: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 82683F8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683F9C: 386A9F48  addi r3, r10, -0x60b8
	ctx.r[3].s64 = ctx.r[10].s64 + -24760;
	// 82683FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683FA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683FA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82683FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683FB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82683FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683FB8: 4BDE2E69  bl 0x82466e20
	ctx.lr = 0x82683FBC;
	sub_82466E20(ctx, base);
	// 82683FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683FD0 size=112
    let mut pc: u32 = 0x82683FD0;
    'dispatch: loop {
        match pc {
            0x82683FD0 => {
    //   block [0x82683FD0..0x82684040)
	// 82683FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683FDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683FE0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683FE4: 38AA9F18  addi r5, r10, -0x60e8
	ctx.r[5].s64 = ctx.r[10].s64 + -24808;
	// 82683FE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683FEC: 390BDCD0  addi r8, r11, -0x2330
	ctx.r[8].s64 = ctx.r[11].s64 + -9008;
	// 82683FF0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82683FF4: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 82683FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683FFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684000: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684008: 386A9F78  addi r3, r10, -0x6088
	ctx.r[3].s64 = ctx.r[10].s64 + -24712;
	// 8268400C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268401C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268402C: 4BDE2DF5  bl 0x82466e20
	ctx.lr = 0x82684030;
	sub_82466E20(ctx, base);
	// 82684030: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268403C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684040 size=112
    let mut pc: u32 = 0x82684040;
    'dispatch: loop {
        match pc {
            0x82684040 => {
    //   block [0x82684040..0x826840B0)
	// 82684040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268404C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684050: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684054: 38AA9F48  addi r5, r10, -0x60b8
	ctx.r[5].s64 = ctx.r[10].s64 + -24760;
	// 82684058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268405C: 390BDD30  addi r8, r11, -0x22d0
	ctx.r[8].s64 = ctx.r[11].s64 + -8912;
	// 82684060: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82684064: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82684068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268406C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684070: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684078: 386A9FA8  addi r3, r10, -0x6058
	ctx.r[3].s64 = ctx.r[10].s64 + -24664;
	// 8268407C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268408C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268409C: 4BDE2D85  bl 0x82466e20
	ctx.lr = 0x826840A0;
	sub_82466E20(ctx, base);
	// 826840A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826840A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826840A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826840AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826840B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826840B0 size=100
    let mut pc: u32 = 0x826840B0;
    'dispatch: loop {
        match pc {
            0x826840B0 => {
    //   block [0x826840B0..0x82684114)
	// 826840B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826840B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826840B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826840BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826840C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826840C4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826840C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826840CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826840D0: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826840D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826840D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826840DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826840E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826840E4: 386A9FD8  addi r3, r10, -0x6028
	ctx.r[3].s64 = ctx.r[10].s64 + -24616;
	// 826840E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826840EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826840F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826840F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826840F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826840FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684100: 4BDE2D21  bl 0x82466e20
	ctx.lr = 0x82684104;
	sub_82466E20(ctx, base);
	// 82684104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268410C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684118 size=112
    let mut pc: u32 = 0x82684118;
    'dispatch: loop {
        match pc {
            0x82684118 => {
    //   block [0x82684118..0x82684188)
	// 82684118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268411C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684124: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684128: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268412C: 38AA9FD8  addi r5, r10, -0x6028
	ctx.r[5].s64 = ctx.r[10].s64 + -24616;
	// 82684130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684134: 390BDD90  addi r8, r11, -0x2270
	ctx.r[8].s64 = ctx.r[11].s64 + -8816;
	// 82684138: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8268413C: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 82684140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684144: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268414C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684150: 386AA008  addi r3, r10, -0x5ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -24568;
	// 82684154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268415C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268416C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684174: 4BDE2CAD  bl 0x82466e20
	ctx.lr = 0x82684178;
	sub_82466E20(ctx, base);
	// 82684178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268417C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684188 size=108
    let mut pc: u32 = 0x82684188;
    'dispatch: loop {
        match pc {
            0x82684188 => {
    //   block [0x82684188..0x826841F4)
	// 82684188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268418C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684194: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268419C: 38EBDE80  addi r7, r11, -0x2180
	ctx.r[7].s64 = ctx.r[11].s64 + -8576;
	// 826841A0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826841A4: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 826841A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826841AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826841B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826841B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826841B8: 386AA038  addi r3, r10, -0x5fc8
	ctx.r[3].s64 = ctx.r[10].s64 + -24520;
	// 826841BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826841C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826841C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826841C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826841CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826841D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826841D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826841D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826841DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826841E0: 4BDE2C41  bl 0x82466e20
	ctx.lr = 0x826841E4;
	sub_82466E20(ctx, base);
	// 826841E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826841E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826841EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826841F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826841F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826841F8 size=108
    let mut pc: u32 = 0x826841F8;
    'dispatch: loop {
        match pc {
            0x826841F8 => {
    //   block [0x826841F8..0x82684264)
	// 826841F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826841FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684204: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268420C: 38EBDF70  addi r7, r11, -0x2090
	ctx.r[7].s64 = ctx.r[11].s64 + -8336;
	// 82684210: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82684214: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 82684218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268421C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684220: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684228: 386AA068  addi r3, r10, -0x5f98
	ctx.r[3].s64 = ctx.r[10].s64 + -24472;
	// 8268422C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268423C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268424C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684250: 4BDE2BD1  bl 0x82466e20
	ctx.lr = 0x82684254;
	sub_82466E20(ctx, base);
	// 82684254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268425C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684268 size=108
    let mut pc: u32 = 0x82684268;
    'dispatch: loop {
        match pc {
            0x82684268 => {
    //   block [0x82684268..0x826842D4)
	// 82684268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268426C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684274: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268427C: 38EBDFB8  addi r7, r11, -0x2048
	ctx.r[7].s64 = ctx.r[11].s64 + -8264;
	// 82684280: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82684284: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82684288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268428C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684298: 386AA098  addi r3, r10, -0x5f68
	ctx.r[3].s64 = ctx.r[10].s64 + -24424;
	// 8268429C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826842A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826842A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826842A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826842AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826842B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826842B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826842B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826842BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826842C0: 4BDE2B61  bl 0x82466e20
	ctx.lr = 0x826842C4;
	sub_82466E20(ctx, base);
	// 826842C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826842C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826842CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826842D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826842D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826842D8 size=108
    let mut pc: u32 = 0x826842D8;
    'dispatch: loop {
        match pc {
            0x826842D8 => {
    //   block [0x826842D8..0x82684344)
	// 826842D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826842DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826842E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826842E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826842E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826842EC: 38EBE090  addi r7, r11, -0x1f70
	ctx.r[7].s64 = ctx.r[11].s64 + -8048;
	// 826842F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826842F4: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826842F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826842FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684308: 386AA0C8  addi r3, r10, -0x5f38
	ctx.r[3].s64 = ctx.r[10].s64 + -24376;
	// 8268430C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268431C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268432C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684330: 4BDE2AF1  bl 0x82466e20
	ctx.lr = 0x82684334;
	sub_82466E20(ctx, base);
	// 82684334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268433C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684348 size=100
    let mut pc: u32 = 0x82684348;
    'dispatch: loop {
        match pc {
            0x82684348 => {
    //   block [0x82684348..0x826843AC)
	// 82684348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268434C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684354: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82684358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268435C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82684360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684368: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8268436C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268437C: 386AA0F8  addi r3, r10, -0x5f08
	ctx.r[3].s64 = ctx.r[10].s64 + -24328;
	// 82684380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684384: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684388: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268438C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684390: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82684394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684398: 4BDE2A89  bl 0x82466e20
	ctx.lr = 0x8268439C;
	sub_82466E20(ctx, base);
	// 8268439C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826843A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826843A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826843A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826843B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826843B0 size=112
    let mut pc: u32 = 0x826843B0;
    'dispatch: loop {
        match pc {
            0x826843B0 => {
    //   block [0x826843B0..0x82684420)
	// 826843B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826843B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826843B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826843BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826843C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826843C4: 38AAA0F8  addi r5, r10, -0x5f08
	ctx.r[5].s64 = ctx.r[10].s64 + -24328;
	// 826843C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826843CC: 390BE0A8  addi r8, r11, -0x1f58
	ctx.r[8].s64 = ctx.r[11].s64 + -8024;
	// 826843D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826843D4: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826843D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826843DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826843E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826843E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826843E8: 386AA128  addi r3, r10, -0x5ed8
	ctx.r[3].s64 = ctx.r[10].s64 + -24280;
	// 826843EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826843F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826843F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826843F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826843FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268440C: 4BDE2A15  bl 0x82466e20
	ctx.lr = 0x82684410;
	sub_82466E20(ctx, base);
	// 82684410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268441C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684420 size=108
    let mut pc: u32 = 0x82684420;
    'dispatch: loop {
        match pc {
            0x82684420 => {
    //   block [0x82684420..0x8268448C)
	// 82684420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268442C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684434: 38EBE0F0  addi r7, r11, -0x1f10
	ctx.r[7].s64 = ctx.r[11].s64 + -7952;
	// 82684438: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268443C: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82684440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684444: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268444C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684450: 386AA158  addi r3, r10, -0x5ea8
	ctx.r[3].s64 = ctx.r[10].s64 + -24232;
	// 82684454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268445C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268446C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684478: 4BDE29A9  bl 0x82466e20
	ctx.lr = 0x8268447C;
	sub_82466E20(ctx, base);
	// 8268447C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684490 size=112
    let mut pc: u32 = 0x82684490;
    'dispatch: loop {
        match pc {
            0x82684490 => {
    //   block [0x82684490..0x82684500)
	// 82684490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268449C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826844A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826844A4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826844A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826844AC: 390BE138  addi r8, r11, -0x1ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -7880;
	// 826844B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826844B4: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826844B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826844BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826844C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826844C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826844C8: 386AA188  addi r3, r10, -0x5e78
	ctx.r[3].s64 = ctx.r[10].s64 + -24184;
	// 826844CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826844D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826844D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826844D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826844DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826844E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826844E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826844E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826844EC: 4BDE2935  bl 0x82466e20
	ctx.lr = 0x826844F0;
	sub_82466E20(ctx, base);
	// 826844F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826844F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826844F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826844FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684500 size=108
    let mut pc: u32 = 0x82684500;
    'dispatch: loop {
        match pc {
            0x82684500 => {
    //   block [0x82684500..0x8268456C)
	// 82684500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268450C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684514: 38EBE150  addi r7, r11, -0x1eb0
	ctx.r[7].s64 = ctx.r[11].s64 + -7856;
	// 82684518: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268451C: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82684520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684524: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268452C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684530: 386AA1B8  addi r3, r10, -0x5e48
	ctx.r[3].s64 = ctx.r[10].s64 + -24136;
	// 82684534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268453C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268454C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684558: 4BDE28C9  bl 0x82466e20
	ctx.lr = 0x8268455C;
	sub_82466E20(ctx, base);
	// 8268455C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684570 size=112
    let mut pc: u32 = 0x82684570;
    'dispatch: loop {
        match pc {
            0x82684570 => {
    //   block [0x82684570..0x826845E0)
	// 82684570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268457C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684580: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684584: 38AAA188  addi r5, r10, -0x5e78
	ctx.r[5].s64 = ctx.r[10].s64 + -24184;
	// 82684588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268458C: 390BE198  addi r8, r11, -0x1e68
	ctx.r[8].s64 = ctx.r[11].s64 + -7784;
	// 82684590: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82684594: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82684598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268459C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826845A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826845A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826845A8: 386AA1E8  addi r3, r10, -0x5e18
	ctx.r[3].s64 = ctx.r[10].s64 + -24088;
	// 826845AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826845B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826845B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826845B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826845BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826845C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826845C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826845C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826845CC: 4BDE2855  bl 0x82466e20
	ctx.lr = 0x826845D0;
	sub_82466E20(ctx, base);
	// 826845D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826845D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826845D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826845DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826845E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826845E0 size=100
    let mut pc: u32 = 0x826845E0;
    'dispatch: loop {
        match pc {
            0x826845E0 => {
    //   block [0x826845E0..0x82684644)
	// 826845E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826845E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826845E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826845EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826845F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826845F4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826845F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826845FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684600: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 82684604: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268460C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684614: 386AA218  addi r3, r10, -0x5de8
	ctx.r[3].s64 = ctx.r[10].s64 + -24040;
	// 82684618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268461C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684620: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82684624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684628: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268462C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684630: 4BDE27F1  bl 0x82466e20
	ctx.lr = 0x82684634;
	sub_82466E20(ctx, base);
	// 82684634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268463C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684648 size=112
    let mut pc: u32 = 0x82684648;
    'dispatch: loop {
        match pc {
            0x82684648 => {
    //   block [0x82684648..0x826846B8)
	// 82684648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268464C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684654: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684658: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268465C: 38AAA218  addi r5, r10, -0x5de8
	ctx.r[5].s64 = ctx.r[10].s64 + -24040;
	// 82684660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684664: 390BE1B0  addi r8, r11, -0x1e50
	ctx.r[8].s64 = ctx.r[11].s64 + -7760;
	// 82684668: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8268466C: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82684670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684674: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268467C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684680: 386AA248  addi r3, r10, -0x5db8
	ctx.r[3].s64 = ctx.r[10].s64 + -23992;
	// 82684684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268468C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268469C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826846A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826846A4: 4BDE277D  bl 0x82466e20
	ctx.lr = 0x826846A8;
	sub_82466E20(ctx, base);
	// 826846A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826846AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826846B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826846B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826846B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826846B8 size=108
    let mut pc: u32 = 0x826846B8;
    'dispatch: loop {
        match pc {
            0x826846B8 => {
    //   block [0x826846B8..0x82684724)
	// 826846B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826846BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826846C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826846C4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826846C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826846CC: 38EBE258  addi r7, r11, -0x1da8
	ctx.r[7].s64 = ctx.r[11].s64 + -7592;
	// 826846D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826846D4: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826846D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826846DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826846E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826846E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826846E8: 386AA278  addi r3, r10, -0x5d88
	ctx.r[3].s64 = ctx.r[10].s64 + -23944;
	// 826846EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826846F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826846F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826846F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826846FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268470C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684710: 4BDE2711  bl 0x82466e20
	ctx.lr = 0x82684714;
	sub_82466E20(ctx, base);
	// 82684714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268471C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684728 size=112
    let mut pc: u32 = 0x82684728;
    'dispatch: loop {
        match pc {
            0x82684728 => {
    //   block [0x82684728..0x82684798)
	// 82684728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268472C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684734: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82684738: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268473C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82684740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684744: 390BE288  addi r8, r11, -0x1d78
	ctx.r[8].s64 = ctx.r[11].s64 + -7544;
	// 82684748: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268474C: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82684750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684754: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268475C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684760: 386AA2A8  addi r3, r10, -0x5d58
	ctx.r[3].s64 = ctx.r[10].s64 + -23896;
	// 82684764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268476C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268477C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684784: 4BDE269D  bl 0x82466e20
	ctx.lr = 0x82684788;
	sub_82466E20(ctx, base);
	// 82684788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268478C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684798 size=112
    let mut pc: u32 = 0x82684798;
    'dispatch: loop {
        match pc {
            0x82684798 => {
    //   block [0x82684798..0x82684808)
	// 82684798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268479C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826847A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826847A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826847A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826847AC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826847B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826847B4: 390BE2D0  addi r8, r11, -0x1d30
	ctx.r[8].s64 = ctx.r[11].s64 + -7472;
	// 826847B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826847BC: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826847C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826847C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826847C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826847CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826847D0: 386AA2D8  addi r3, r10, -0x5d28
	ctx.r[3].s64 = ctx.r[10].s64 + -23848;
	// 826847D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826847D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826847DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826847E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826847E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826847E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826847EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826847F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826847F4: 4BDE262D  bl 0x82466e20
	ctx.lr = 0x826847F8;
	sub_82466E20(ctx, base);
	// 826847F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826847FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684808 size=100
    let mut pc: u32 = 0x82684808;
    'dispatch: loop {
        match pc {
            0x82684808 => {
    //   block [0x82684808..0x8268486C)
	// 82684808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268480C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684814: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82684818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268481C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82684820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684828: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 8268482C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268483C: 386AA308  addi r3, r10, -0x5cf8
	ctx.r[3].s64 = ctx.r[10].s64 + -23800;
	// 82684840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684844: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684848: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268484C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684850: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82684854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684858: 4BDE25C9  bl 0x82466e20
	ctx.lr = 0x8268485C;
	sub_82466E20(ctx, base);
	// 8268485C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684870 size=112
    let mut pc: u32 = 0x82684870;
    'dispatch: loop {
        match pc {
            0x82684870 => {
    //   block [0x82684870..0x826848E0)
	// 82684870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268487C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684880: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684884: 38AAA308  addi r5, r10, -0x5cf8
	ctx.r[5].s64 = ctx.r[10].s64 + -23800;
	// 82684888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268488C: 390BE318  addi r8, r11, -0x1ce8
	ctx.r[8].s64 = ctx.r[11].s64 + -7400;
	// 82684890: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82684894: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 82684898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268489C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826848A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826848A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826848A8: 386AA338  addi r3, r10, -0x5cc8
	ctx.r[3].s64 = ctx.r[10].s64 + -23752;
	// 826848AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826848B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826848B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826848B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826848BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826848C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826848C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826848C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826848CC: 4BDE2555  bl 0x82466e20
	ctx.lr = 0x826848D0;
	sub_82466E20(ctx, base);
	// 826848D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826848D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826848D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826848DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826848E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826848E0 size=112
    let mut pc: u32 = 0x826848E0;
    'dispatch: loop {
        match pc {
            0x826848E0 => {
    //   block [0x826848E0..0x82684950)
	// 826848E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826848E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826848E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826848EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826848F0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826848F4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826848F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826848FC: 390BE360  addi r8, r11, -0x1ca0
	ctx.r[8].s64 = ctx.r[11].s64 + -7328;
	// 82684900: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82684904: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82684908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268490C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684910: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684918: 386AA368  addi r3, r10, -0x5c98
	ctx.r[3].s64 = ctx.r[10].s64 + -23704;
	// 8268491C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268492C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268493C: 4BDE24E5  bl 0x82466e20
	ctx.lr = 0x82684940;
	sub_82466E20(ctx, base);
	// 82684940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268494C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684950 size=112
    let mut pc: u32 = 0x82684950;
    'dispatch: loop {
        match pc {
            0x82684950 => {
    //   block [0x82684950..0x826849C0)
	// 82684950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268495C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82684960: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684964: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82684968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268496C: 390BE378  addi r8, r11, -0x1c88
	ctx.r[8].s64 = ctx.r[11].s64 + -7304;
	// 82684970: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82684974: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82684978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268497C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684988: 386AA398  addi r3, r10, -0x5c68
	ctx.r[3].s64 = ctx.r[10].s64 + -23656;
	// 8268498C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268499C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826849A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826849A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826849A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826849AC: 4BDE2475  bl 0x82466e20
	ctx.lr = 0x826849B0;
	sub_82466E20(ctx, base);
	// 826849B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826849B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826849B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826849BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826849C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826849C0 size=112
    let mut pc: u32 = 0x826849C0;
    'dispatch: loop {
        match pc {
            0x826849C0 => {
    //   block [0x826849C0..0x82684A30)
	// 826849C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826849C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826849C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826849CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826849D0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826849D4: 38AAA368  addi r5, r10, -0x5c98
	ctx.r[5].s64 = ctx.r[10].s64 + -23704;
	// 826849D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826849DC: 390BE390  addi r8, r11, -0x1c70
	ctx.r[8].s64 = ctx.r[11].s64 + -7280;
	// 826849E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826849E4: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826849E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826849EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826849F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826849F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826849F8: 386AA3C8  addi r3, r10, -0x5c38
	ctx.r[3].s64 = ctx.r[10].s64 + -23608;
	// 826849FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684A1C: 4BDE2405  bl 0x82466e20
	ctx.lr = 0x82684A20;
	sub_82466E20(ctx, base);
	// 82684A20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684A30 size=72
    let mut pc: u32 = 0x82684A30;
    'dispatch: loop {
        match pc {
            0x82684A30 => {
    //   block [0x82684A30..0x82684A78)
	// 82684A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684A3C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82684A40: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82684A44: 38CB3BC0  addi r6, r11, 0x3bc0
	ctx.r[6].s64 = ctx.r[11].s64 + 15296;
	// 82684A48: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82684A4C: 388B5198  addi r4, r11, 0x5198
	ctx.r[4].s64 = ctx.r[11].s64 + 20888;
	// 82684A50: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82684A54: 386BA3F8  addi r3, r11, -0x5c08
	ctx.r[3].s64 = ctx.r[11].s64 + -23560;
	// 82684A58: 4BDF7031  bl 0x8247ba88
	ctx.lr = 0x82684A5C;
	sub_8247BA88(ctx, base);
	// 82684A5C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82684A60: 386BCE40  addi r3, r11, -0x31c0
	ctx.r[3].s64 = ctx.r[11].s64 + -12736;
	// 82684A64: 4BEAE0D5  bl 0x82532b38
	ctx.lr = 0x82684A68;
	sub_82532B38(ctx, base);
	// 82684A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82684A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684A78 size=108
    let mut pc: u32 = 0x82684A78;
    'dispatch: loop {
        match pc {
            0x82684A78 => {
    //   block [0x82684A78..0x82684AE4)
	// 82684A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684A84: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684A88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684A8C: 38EBFC88  addi r7, r11, -0x378
	ctx.r[7].s64 = ctx.r[11].s64 + -888;
	// 82684A90: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82684A94: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 82684A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684A9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684AA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684AA8: 386AA410  addi r3, r10, -0x5bf0
	ctx.r[3].s64 = ctx.r[10].s64 + -23536;
	// 82684AAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684AD0: 4BDE2351  bl 0x82466e20
	ctx.lr = 0x82684AD4;
	sub_82466E20(ctx, base);
	// 82684AD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82684AE8 size=24
    let mut pc: u32 = 0x82684AE8;
    'dispatch: loop {
        match pc {
            0x82684AE8 => {
    //   block [0x82684AE8..0x82684B00)
	// 82684AE8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684AEC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82684AF0: 394A4BF0  addi r10, r10, 0x4bf0
	ctx.r[10].s64 = ctx.r[10].s64 + 19440;
	// 82684AF4: 816BFD00  lwz r11, -0x300(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-768 as u32) ) } as u64;
	// 82684AF8: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82684AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684B00 size=112
    let mut pc: u32 = 0x82684B00;
    'dispatch: loop {
        match pc {
            0x82684B00 => {
    //   block [0x82684B00..0x82684B70)
	// 82684B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684B0C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82684B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684B14: 392B5A6C  addi r9, r11, 0x5a6c
	ctx.r[9].s64 = ctx.r[11].s64 + 23148;
	// 82684B18: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82684B1C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82684B20: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684B24: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 82684B28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684B2C: 396B4BF0  addi r11, r11, 0x4bf0
	ctx.r[11].s64 = ctx.r[11].s64 + 19440;
	// 82684B30: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82684B34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684B38: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82684B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684B40: 386AA440  addi r3, r10, -0x5bc0
	ctx.r[3].s64 = ctx.r[10].s64 + -23488;
	// 82684B44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82684B48: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82684B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684B50: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82684B54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684B58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82684B5C: 4BDE22C5  bl 0x82466e20
	ctx.lr = 0x82684B60;
	sub_82466E20(ctx, base);
	// 82684B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684B70 size=108
    let mut pc: u32 = 0x82684B70;
    'dispatch: loop {
        match pc {
            0x82684B70 => {
    //   block [0x82684B70..0x82684BDC)
	// 82684B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684B7C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684B84: 38EBFD04  addi r7, r11, -0x2fc
	ctx.r[7].s64 = ctx.r[11].s64 + -764;
	// 82684B88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82684B8C: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 82684B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684B94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684B98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684BA0: 386AA470  addi r3, r10, -0x5b90
	ctx.r[3].s64 = ctx.r[10].s64 + -23440;
	// 82684BA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684BC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684BC8: 4BDE2259  bl 0x82466e20
	ctx.lr = 0x82684BCC;
	sub_82466E20(ctx, base);
	// 82684BCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684BE0 size=108
    let mut pc: u32 = 0x82684BE0;
    'dispatch: loop {
        match pc {
            0x82684BE0 => {
    //   block [0x82684BE0..0x82684C4C)
	// 82684BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684BEC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684BF4: 38EBFD34  addi r7, r11, -0x2cc
	ctx.r[7].s64 = ctx.r[11].s64 + -716;
	// 82684BF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82684BFC: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 82684C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684C04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684C08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684C10: 386AA4A0  addi r3, r10, -0x5b60
	ctx.r[3].s64 = ctx.r[10].s64 + -23392;
	// 82684C14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684C34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684C38: 4BDE21E9  bl 0x82466e20
	ctx.lr = 0x82684C3C;
	sub_82466E20(ctx, base);
	// 82684C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684C50 size=112
    let mut pc: u32 = 0x82684C50;
    'dispatch: loop {
        match pc {
            0x82684C50 => {
    //   block [0x82684C50..0x82684CC0)
	// 82684C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684C5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684C60: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684C64: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 82684C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684C6C: 390BFD68  addi r8, r11, -0x298
	ctx.r[8].s64 = ctx.r[11].s64 + -664;
	// 82684C70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82684C74: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 82684C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684C7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684C88: 386AA4D0  addi r3, r10, -0x5b30
	ctx.r[3].s64 = ctx.r[10].s64 + -23344;
	// 82684C8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684CAC: 4BDE2175  bl 0x82466e20
	ctx.lr = 0x82684CB0;
	sub_82466E20(ctx, base);
	// 82684CB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684CC0 size=108
    let mut pc: u32 = 0x82684CC0;
    'dispatch: loop {
        match pc {
            0x82684CC0 => {
    //   block [0x82684CC0..0x82684D2C)
	// 82684CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684CCC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684CD4: 38EBFDC8  addi r7, r11, -0x238
	ctx.r[7].s64 = ctx.r[11].s64 + -568;
	// 82684CD8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82684CDC: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 82684CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684CE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684CE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684CF0: 386AA500  addi r3, r10, -0x5b00
	ctx.r[3].s64 = ctx.r[10].s64 + -23296;
	// 82684CF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684D14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684D18: 4BDE2109  bl 0x82466e20
	ctx.lr = 0x82684D1C;
	sub_82466E20(ctx, base);
	// 82684D1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684D30 size=112
    let mut pc: u32 = 0x82684D30;
    'dispatch: loop {
        match pc {
            0x82684D30 => {
    //   block [0x82684D30..0x82684DA0)
	// 82684D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684D3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684D40: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684D44: 38AAA4D0  addi r5, r10, -0x5b30
	ctx.r[5].s64 = ctx.r[10].s64 + -23344;
	// 82684D48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684D4C: 390BFE28  addi r8, r11, -0x1d8
	ctx.r[8].s64 = ctx.r[11].s64 + -472;
	// 82684D50: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82684D54: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 82684D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684D5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684D60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684D68: 386AA530  addi r3, r10, -0x5ad0
	ctx.r[3].s64 = ctx.r[10].s64 + -23248;
	// 82684D6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684D70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684D74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684D8C: 4BDE2095  bl 0x82466e20
	ctx.lr = 0x82684D90;
	sub_82466E20(ctx, base);
	// 82684D90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684DA0 size=112
    let mut pc: u32 = 0x82684DA0;
    'dispatch: loop {
        match pc {
            0x82684DA0 => {
    //   block [0x82684DA0..0x82684E10)
	// 82684DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684DAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684DB0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684DB4: 38AAA4D0  addi r5, r10, -0x5b30
	ctx.r[5].s64 = ctx.r[10].s64 + -23344;
	// 82684DB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684DBC: 390BFEB8  addi r8, r11, -0x148
	ctx.r[8].s64 = ctx.r[11].s64 + -328;
	// 82684DC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82684DC4: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 82684DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684DCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684DD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684DD8: 386AA560  addi r3, r10, -0x5aa0
	ctx.r[3].s64 = ctx.r[10].s64 + -23200;
	// 82684DDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684DFC: 4BDE2025  bl 0x82466e20
	ctx.lr = 0x82684E00;
	sub_82466E20(ctx, base);
	// 82684E00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684E10 size=108
    let mut pc: u32 = 0x82684E10;
    'dispatch: loop {
        match pc {
            0x82684E10 => {
    //   block [0x82684E10..0x82684E7C)
	// 82684E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684E1C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684E24: 38EBFED0  addi r7, r11, -0x130
	ctx.r[7].s64 = ctx.r[11].s64 + -304;
	// 82684E28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82684E2C: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82684E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684E34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684E38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684E40: 386AA590  addi r3, r10, -0x5a70
	ctx.r[3].s64 = ctx.r[10].s64 + -23152;
	// 82684E44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684E48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684E64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684E68: 4BDE1FB9  bl 0x82466e20
	ctx.lr = 0x82684E6C;
	sub_82466E20(ctx, base);
	// 82684E6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684E80 size=112
    let mut pc: u32 = 0x82684E80;
    'dispatch: loop {
        match pc {
            0x82684E80 => {
    //   block [0x82684E80..0x82684EF0)
	// 82684E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684E8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684E90: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684E94: 38AAA4D0  addi r5, r10, -0x5b30
	ctx.r[5].s64 = ctx.r[10].s64 + -23344;
	// 82684E98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684E9C: 390BFF30  addi r8, r11, -0xd0
	ctx.r[8].s64 = ctx.r[11].s64 + -208;
	// 82684EA0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82684EA4: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 82684EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684EAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684EB8: 386AA5C0  addi r3, r10, -0x5a40
	ctx.r[3].s64 = ctx.r[10].s64 + -23104;
	// 82684EBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684EDC: 4BDE1F45  bl 0x82466e20
	ctx.lr = 0x82684EE0;
	sub_82466E20(ctx, base);
	// 82684EE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684EF0 size=108
    let mut pc: u32 = 0x82684EF0;
    'dispatch: loop {
        match pc {
            0x82684EF0 => {
    //   block [0x82684EF0..0x82684F5C)
	// 82684EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684EFC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684F04: 38EBFFD8  addi r7, r11, -0x28
	ctx.r[7].s64 = ctx.r[11].s64 + -40;
	// 82684F08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82684F0C: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 82684F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684F14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684F18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684F20: 386AA5F0  addi r3, r10, -0x5a10
	ctx.r[3].s64 = ctx.r[10].s64 + -23056;
	// 82684F24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684F48: 4BDE1ED9  bl 0x82466e20
	ctx.lr = 0x82684F4C;
	sub_82466E20(ctx, base);
	// 82684F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684F60 size=108
    let mut pc: u32 = 0x82684F60;
    'dispatch: loop {
        match pc {
            0x82684F60 => {
    //   block [0x82684F60..0x82684FCC)
	// 82684F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684F6C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684F74: 38EBFFF0  addi r7, r11, -0x10
	ctx.r[7].s64 = ctx.r[11].s64 + -16;
	// 82684F78: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82684F7C: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 82684F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684F84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684F90: 386AA620  addi r3, r10, -0x59e0
	ctx.r[3].s64 = ctx.r[10].s64 + -23008;
	// 82684F94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684FB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684FB8: 4BDE1E69  bl 0x82466e20
	ctx.lr = 0x82684FBC;
	sub_82466E20(ctx, base);
	// 82684FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684FD0 size=112
    let mut pc: u32 = 0x82684FD0;
    'dispatch: loop {
        match pc {
            0x82684FD0 => {
    //   block [0x82684FD0..0x82685040)
	// 82684FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684FDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684FE0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684FE4: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 82684FE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684FEC: 390B0050  addi r8, r11, 0x50
	ctx.r[8].s64 = ctx.r[11].s64 + 80;
	// 82684FF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82684FF4: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 82684FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684FFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685000: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82685004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685008: 386AA650  addi r3, r10, -0x59b0
	ctx.r[3].s64 = ctx.r[10].s64 + -22960;
	// 8268500C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82685010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268501C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268502C: 4BDE1DF5  bl 0x82466e20
	ctx.lr = 0x82685030;
	sub_82466E20(ctx, base);
	// 82685030: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268503C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685040 size=108
    let mut pc: u32 = 0x82685040;
    'dispatch: loop {
        match pc {
            0x82685040 => {
    //   block [0x82685040..0x826850AC)
	// 82685040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268504C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685054: 38EB0068  addi r7, r11, 0x68
	ctx.r[7].s64 = ctx.r[11].s64 + 104;
	// 82685058: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268505C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 82685060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685064: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685068: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268506C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685070: 386AA680  addi r3, r10, -0x5980
	ctx.r[3].s64 = ctx.r[10].s64 + -22912;
	// 82685074: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268507C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268508C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685094: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685098: 4BDE1D89  bl 0x82466e20
	ctx.lr = 0x8268509C;
	sub_82466E20(ctx, base);
	// 8268509C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826850A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826850A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826850A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826850B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826850B0 size=108
    let mut pc: u32 = 0x826850B0;
    'dispatch: loop {
        match pc {
            0x826850B0 => {
    //   block [0x826850B0..0x8268511C)
	// 826850B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826850B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826850B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826850BC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826850C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826850C4: 38EB00B0  addi r7, r11, 0xb0
	ctx.r[7].s64 = ctx.r[11].s64 + 176;
	// 826850C8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826850CC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826850D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826850D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826850D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826850DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826850E0: 386AA6B0  addi r3, r10, -0x5950
	ctx.r[3].s64 = ctx.r[10].s64 + -22864;
	// 826850E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826850E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826850EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826850F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826850F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826850F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826850FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685104: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685108: 4BDE1D19  bl 0x82466e20
	ctx.lr = 0x8268510C;
	sub_82466E20(ctx, base);
	// 8268510C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685120 size=108
    let mut pc: u32 = 0x82685120;
    'dispatch: loop {
        match pc {
            0x82685120 => {
    //   block [0x82685120..0x8268518C)
	// 82685120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268512C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685134: 38EB0140  addi r7, r11, 0x140
	ctx.r[7].s64 = ctx.r[11].s64 + 320;
	// 82685138: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8268513C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82685140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685144: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685148: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268514C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685150: 386AA6E0  addi r3, r10, -0x5920
	ctx.r[3].s64 = ctx.r[10].s64 + -22816;
	// 82685154: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268515C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268516C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685174: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685178: 4BDE1CA9  bl 0x82466e20
	ctx.lr = 0x8268517C;
	sub_82466E20(ctx, base);
	// 8268517C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685190 size=100
    let mut pc: u32 = 0x82685190;
    'dispatch: loop {
        match pc {
            0x82685190 => {
    //   block [0x82685190..0x826851F4)
	// 82685190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268519C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826851A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826851A4: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 826851A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826851AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826851B0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826851B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826851B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826851BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826851C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826851C4: 386AA710  addi r3, r10, -0x58f0
	ctx.r[3].s64 = ctx.r[10].s64 + -22768;
	// 826851C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826851CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826851D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826851D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826851D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826851DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826851E0: 4BDE1C41  bl 0x82466e20
	ctx.lr = 0x826851E4;
	sub_82466E20(ctx, base);
	// 826851E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826851E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826851EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826851F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826851F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826851F8 size=112
    let mut pc: u32 = 0x826851F8;
    'dispatch: loop {
        match pc {
            0x826851F8 => {
    //   block [0x826851F8..0x82685268)
	// 826851F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826851FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685204: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685208: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268520C: 38AAA710  addi r5, r10, -0x58f0
	ctx.r[5].s64 = ctx.r[10].s64 + -22768;
	// 82685210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685214: 390B01D0  addi r8, r11, 0x1d0
	ctx.r[8].s64 = ctx.r[11].s64 + 464;
	// 82685218: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8268521C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82685220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685224: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268522C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685230: 386AA740  addi r3, r10, -0x58c0
	ctx.r[3].s64 = ctx.r[10].s64 + -22720;
	// 82685234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82685238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268523C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268524C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685254: 4BDE1BCD  bl 0x82466e20
	ctx.lr = 0x82685258;
	sub_82466E20(ctx, base);
	// 82685258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268525C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685268 size=108
    let mut pc: u32 = 0x82685268;
    'dispatch: loop {
        match pc {
            0x82685268 => {
    //   block [0x82685268..0x826852D4)
	// 82685268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268526C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685274: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268527C: 38EB0230  addi r7, r11, 0x230
	ctx.r[7].s64 = ctx.r[11].s64 + 560;
	// 82685280: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685284: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 82685288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268528C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685298: 386AA770  addi r3, r10, -0x5890
	ctx.r[3].s64 = ctx.r[10].s64 + -22672;
	// 8268529C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826852A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826852A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826852A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826852AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826852B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826852B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826852B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826852BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826852C0: 4BDE1B61  bl 0x82466e20
	ctx.lr = 0x826852C4;
	sub_82466E20(ctx, base);
	// 826852C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826852C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826852CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826852D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826852D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826852D8 size=108
    let mut pc: u32 = 0x826852D8;
    'dispatch: loop {
        match pc {
            0x826852D8 => {
    //   block [0x826852D8..0x82685344)
	// 826852D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826852DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826852E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826852E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826852E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826852EC: 38EB0260  addi r7, r11, 0x260
	ctx.r[7].s64 = ctx.r[11].s64 + 608;
	// 826852F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826852F4: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 826852F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826852FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685308: 386AA7A0  addi r3, r10, -0x5860
	ctx.r[3].s64 = ctx.r[10].s64 + -22624;
	// 8268530C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268531C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268532C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685330: 4BDE1AF1  bl 0x82466e20
	ctx.lr = 0x82685334;
	sub_82466E20(ctx, base);
	// 82685334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268533C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685348 size=108
    let mut pc: u32 = 0x82685348;
    'dispatch: loop {
        match pc {
            0x82685348 => {
    //   block [0x82685348..0x826853B4)
	// 82685348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268534C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685354: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268535C: 38EB02A8  addi r7, r11, 0x2a8
	ctx.r[7].s64 = ctx.r[11].s64 + 680;
	// 82685360: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82685364: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 82685368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268536C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685370: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685378: 386AA7D0  addi r3, r10, -0x5830
	ctx.r[3].s64 = ctx.r[10].s64 + -22576;
	// 8268537C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268538C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268539C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826853A0: 4BDE1A81  bl 0x82466e20
	ctx.lr = 0x826853A4;
	sub_82466E20(ctx, base);
	// 826853A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826853A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826853AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826853B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826853B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826853B8 size=96
    let mut pc: u32 = 0x826853B8;
    'dispatch: loop {
        match pc {
            0x826853B8 => {
    //   block [0x826853B8..0x82685418)
	// 826853B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826853BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826853C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826853C4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826853C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826853CC: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826853D0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826853D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826853D8: 386AA800  addi r3, r10, -0x5800
	ctx.r[3].s64 = ctx.r[10].s64 + -22528;
	// 826853DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826853E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826853E4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826853E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826853EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826853F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826853F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826853F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826853FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685400: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82685404: 4BDE1A1D  bl 0x82466e20
	ctx.lr = 0x82685408;
	sub_82466E20(ctx, base);
	// 82685408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268540C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685418 size=112
    let mut pc: u32 = 0x82685418;
    'dispatch: loop {
        match pc {
            0x82685418 => {
    //   block [0x82685418..0x82685488)
	// 82685418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268541C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685424: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685428: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268542C: 38AAA800  addi r5, r10, -0x5800
	ctx.r[5].s64 = ctx.r[10].s64 + -22528;
	// 82685430: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82685434: 390B0308  addi r8, r11, 0x308
	ctx.r[8].s64 = ctx.r[11].s64 + 776;
	// 82685438: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268543C: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82685440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685444: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268544C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685450: 386AA830  addi r3, r10, -0x57d0
	ctx.r[3].s64 = ctx.r[10].s64 + -22480;
	// 82685454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82685458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268545C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268546C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685474: 4BDE19AD  bl 0x82466e20
	ctx.lr = 0x82685478;
	sub_82466E20(ctx, base);
	// 82685478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268547C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685488 size=112
    let mut pc: u32 = 0x82685488;
    'dispatch: loop {
        match pc {
            0x82685488 => {
    //   block [0x82685488..0x826854F8)
	// 82685488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268548C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685494: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82685498: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268549C: 392A5A98  addi r9, r10, 0x5a98
	ctx.r[9].s64 = ctx.r[10].s64 + 23192;
	// 826854A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826854A4: 390B0338  addi r8, r11, 0x338
	ctx.r[8].s64 = ctx.r[11].s64 + 824;
	// 826854A8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826854AC: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826854B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826854B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826854B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826854BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826854C0: 386AA860  addi r3, r10, -0x57a0
	ctx.r[3].s64 = ctx.r[10].s64 + -22432;
	// 826854C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826854C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826854CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826854D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826854D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826854D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826854DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826854E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826854E4: 4BDE193D  bl 0x82466e20
	ctx.lr = 0x826854E8;
	sub_82466E20(ctx, base);
	// 826854E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826854EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826854F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826854F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826854F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826854F8 size=108
    let mut pc: u32 = 0x826854F8;
    'dispatch: loop {
        match pc {
            0x826854F8 => {
    //   block [0x826854F8..0x82685564)
	// 826854F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826854FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685504: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685508: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268550C: 38EB03E0  addi r7, r11, 0x3e0
	ctx.r[7].s64 = ctx.r[11].s64 + 992;
	// 82685510: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685514: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 82685518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268551C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685520: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685528: 386AA890  addi r3, r10, -0x5770
	ctx.r[3].s64 = ctx.r[10].s64 + -22384;
	// 8268552C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268553C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268554C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685550: 4BDE18D1  bl 0x82466e20
	ctx.lr = 0x82685554;
	sub_82466E20(ctx, base);
	// 82685554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268555C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685568 size=108
    let mut pc: u32 = 0x82685568;
    'dispatch: loop {
        match pc {
            0x82685568 => {
    //   block [0x82685568..0x826855D4)
	// 82685568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268556C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685574: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685578: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268557C: 38EB0410  addi r7, r11, 0x410
	ctx.r[7].s64 = ctx.r[11].s64 + 1040;
	// 82685580: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685584: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82685588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268558C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685598: 386AA8C0  addi r3, r10, -0x5740
	ctx.r[3].s64 = ctx.r[10].s64 + -22336;
	// 8268559C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826855A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826855A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826855A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826855AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826855B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826855B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826855B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826855BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826855C0: 4BDE1861  bl 0x82466e20
	ctx.lr = 0x826855C4;
	sub_82466E20(ctx, base);
	// 826855C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826855C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826855CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826855D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826855D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826855D8 size=28
    let mut pc: u32 = 0x826855D8;
    'dispatch: loop {
        match pc {
            0x826855D8 => {
    //   block [0x826855D8..0x826855F4)
	// 826855D8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826855DC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 826855E0: 394A4C38  addi r10, r10, 0x4c38
	ctx.r[10].s64 = ctx.r[10].s64 + 19512;
	// 826855E4: 816B0440  lwz r11, 0x440(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1088 as u32) ) } as u64;
	// 826855E8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826855EC: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826855F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826855F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826855F8 size=112
    let mut pc: u32 = 0x826855F8;
    'dispatch: loop {
        match pc {
            0x826855F8 => {
    //   block [0x826855F8..0x82685668)
	// 826855F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826855FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685604: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82685608: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268560C: 392A5C08  addi r9, r10, 0x5c08
	ctx.r[9].s64 = ctx.r[10].s64 + 23560;
	// 82685610: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82685614: 390B4C38  addi r8, r11, 0x4c38
	ctx.r[8].s64 = ctx.r[11].s64 + 19512;
	// 82685618: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8268561C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 82685620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685624: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685628: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268562C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685630: 386AA8F0  addi r3, r10, -0x5710
	ctx.r[3].s64 = ctx.r[10].s64 + -22288;
	// 82685634: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82685638: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8268563C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268564C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685654: 4BDE17CD  bl 0x82466e20
	ctx.lr = 0x82685658;
	sub_82466E20(ctx, base);
	// 82685658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268565C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685668 size=108
    let mut pc: u32 = 0x82685668;
    'dispatch: loop {
        match pc {
            0x82685668 => {
    //   block [0x82685668..0x826856D4)
	// 82685668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268566C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685674: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685678: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268567C: 38EB044C  addi r7, r11, 0x44c
	ctx.r[7].s64 = ctx.r[11].s64 + 1100;
	// 82685680: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685684: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 82685688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268568C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685698: 386AA920  addi r3, r10, -0x56e0
	ctx.r[3].s64 = ctx.r[10].s64 + -22240;
	// 8268569C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826856A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826856A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826856A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826856AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826856B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826856B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826856B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826856BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826856C0: 4BDE1761  bl 0x82466e20
	ctx.lr = 0x826856C4;
	sub_82466E20(ctx, base);
	// 826856C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826856C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826856CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826856D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826856D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826856D8 size=108
    let mut pc: u32 = 0x826856D8;
    'dispatch: loop {
        match pc {
            0x826856D8 => {
    //   block [0x826856D8..0x82685744)
	// 826856D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826856DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826856E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826856E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826856E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826856EC: 38EB047C  addi r7, r11, 0x47c
	ctx.r[7].s64 = ctx.r[11].s64 + 1148;
	// 826856F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826856F4: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826856F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826856FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685700: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685708: 386AA950  addi r3, r10, -0x56b0
	ctx.r[3].s64 = ctx.r[10].s64 + -22192;
	// 8268570C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268571C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268572C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685730: 4BDE16F1  bl 0x82466e20
	ctx.lr = 0x82685734;
	sub_82466E20(ctx, base);
	// 82685734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268573C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82685748 size=24
    let mut pc: u32 = 0x82685748;
    'dispatch: loop {
        match pc {
            0x82685748 => {
    //   block [0x82685748..0x82685760)
	// 82685748: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268574C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82685750: 394A4CF8  addi r10, r10, 0x4cf8
	ctx.r[10].s64 = ctx.r[10].s64 + 19704;
	// 82685754: 816B0494  lwz r11, 0x494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82685758: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8268575C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685760 size=112
    let mut pc: u32 = 0x82685760;
    'dispatch: loop {
        match pc {
            0x82685760 => {
    //   block [0x82685760..0x826857D0)
	// 82685760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268576C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82685770: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685774: 392A5C5C  addi r9, r10, 0x5c5c
	ctx.r[9].s64 = ctx.r[10].s64 + 23644;
	// 82685778: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268577C: 390B4CF8  addi r8, r11, 0x4cf8
	ctx.r[8].s64 = ctx.r[11].s64 + 19704;
	// 82685780: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82685784: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 82685788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268578C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82685794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685798: 386AA980  addi r3, r10, -0x5680
	ctx.r[3].s64 = ctx.r[10].s64 + -22144;
	// 8268579C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826857A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826857A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826857A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826857AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826857B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826857B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826857B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826857BC: 4BDE1665  bl 0x82466e20
	ctx.lr = 0x826857C0;
	sub_82466E20(ctx, base);
	// 826857C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826857C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826857C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826857CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826857D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826857D0 size=108
    let mut pc: u32 = 0x826857D0;
    'dispatch: loop {
        match pc {
            0x826857D0 => {
    //   block [0x826857D0..0x8268583C)
	// 826857D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826857D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826857D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826857DC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826857E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826857E4: 38EB0498  addi r7, r11, 0x498
	ctx.r[7].s64 = ctx.r[11].s64 + 1176;
	// 826857E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826857EC: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826857F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826857F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826857F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826857FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685800: 386AA9B0  addi r3, r10, -0x5650
	ctx.r[3].s64 = ctx.r[10].s64 + -22096;
	// 82685804: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268580C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268581C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685828: 4BDE15F9  bl 0x82466e20
	ctx.lr = 0x8268582C;
	sub_82466E20(ctx, base);
	// 8268582C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685840 size=108
    let mut pc: u32 = 0x82685840;
    'dispatch: loop {
        match pc {
            0x82685840 => {
    //   block [0x82685840..0x826858AC)
	// 82685840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268584C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685850: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82685854: 38EB04C8  addi r7, r11, 0x4c8
	ctx.r[7].s64 = ctx.r[11].s64 + 1224;
	// 82685858: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268585C: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 82685860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685864: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268586C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685870: 386AA9E0  addi r3, r10, -0x5620
	ctx.r[3].s64 = ctx.r[10].s64 + -22048;
	// 82685874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268587C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268588C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685898: 4BDE1589  bl 0x82466e20
	ctx.lr = 0x8268589C;
	sub_82466E20(ctx, base);
	// 8268589C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826858A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826858A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826858A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826858B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826858B0 size=112
    let mut pc: u32 = 0x826858B0;
    'dispatch: loop {
        match pc {
            0x826858B0 => {
    //   block [0x826858B0..0x82685920)
	// 826858B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826858B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826858B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826858BC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826858C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826858C4: 392A5C80  addi r9, r10, 0x5c80
	ctx.r[9].s64 = ctx.r[10].s64 + 23680;
	// 826858C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826858CC: 390B0500  addi r8, r11, 0x500
	ctx.r[8].s64 = ctx.r[11].s64 + 1280;
	// 826858D0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826858D4: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826858D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826858DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826858E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826858E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826858E8: 386AAA10  addi r3, r10, -0x55f0
	ctx.r[3].s64 = ctx.r[10].s64 + -22000;
	// 826858EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826858F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826858F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826858F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826858FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268590C: 4BDE1515  bl 0x82466e20
	ctx.lr = 0x82685910;
	sub_82466E20(ctx, base);
	// 82685910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268591C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685920 size=108
    let mut pc: u32 = 0x82685920;
    'dispatch: loop {
        match pc {
            0x82685920 => {
    //   block [0x82685920..0x8268598C)
	// 82685920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268592C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685930: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82685934: 38EB0560  addi r7, r11, 0x560
	ctx.r[7].s64 = ctx.r[11].s64 + 1376;
	// 82685938: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8268593C: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 82685940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685944: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685948: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268594C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685950: 386AAA40  addi r3, r10, -0x55c0
	ctx.r[3].s64 = ctx.r[10].s64 + -21952;
	// 82685954: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268595C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268596C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685978: 4BDE14A9  bl 0x82466e20
	ctx.lr = 0x8268597C;
	sub_82466E20(ctx, base);
	// 8268597C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685990 size=108
    let mut pc: u32 = 0x82685990;
    'dispatch: loop {
        match pc {
            0x82685990 => {
    //   block [0x82685990..0x826859FC)
	// 82685990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268599C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826859A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826859A4: 38EB0620  addi r7, r11, 0x620
	ctx.r[7].s64 = ctx.r[11].s64 + 1568;
	// 826859A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826859AC: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826859B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826859B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826859B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826859BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826859C0: 386AAA70  addi r3, r10, -0x5590
	ctx.r[3].s64 = ctx.r[10].s64 + -21904;
	// 826859C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826859C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826859CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826859D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826859D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826859D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826859DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826859E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826859E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826859E8: 4BDE1439  bl 0x82466e20
	ctx.lr = 0x826859EC;
	sub_82466E20(ctx, base);
	// 826859EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826859F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826859F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826859F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685A00 size=108
    let mut pc: u32 = 0x82685A00;
    'dispatch: loop {
        match pc {
            0x82685A00 => {
    //   block [0x82685A00..0x82685A6C)
	// 82685A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685A0C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685A14: 38EB0638  addi r7, r11, 0x638
	ctx.r[7].s64 = ctx.r[11].s64 + 1592;
	// 82685A18: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82685A1C: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82685A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685A24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685A28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685A30: 386AAAA0  addi r3, r10, -0x5560
	ctx.r[3].s64 = ctx.r[10].s64 + -21856;
	// 82685A34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685A54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685A58: 4BDE13C9  bl 0x82466e20
	ctx.lr = 0x82685A5C;
	sub_82466E20(ctx, base);
	// 82685A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82685A70 size=24
    let mut pc: u32 = 0x82685A70;
    'dispatch: loop {
        match pc {
            0x82685A70 => {
    //   block [0x82685A70..0x82685A88)
	// 82685A70: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685A74: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82685A78: 394A4D88  addi r10, r10, 0x4d88
	ctx.r[10].s64 = ctx.r[10].s64 + 19848;
	// 82685A7C: 816B04FC  lwz r11, 0x4fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1276 as u32) ) } as u64;
	// 82685A80: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82685A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685A88 size=108
    let mut pc: u32 = 0x82685A88;
    'dispatch: loop {
        match pc {
            0x82685A88 => {
    //   block [0x82685A88..0x82685AF4)
	// 82685A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685A94: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685A9C: 38EB4D88  addi r7, r11, 0x4d88
	ctx.r[7].s64 = ctx.r[11].s64 + 19848;
	// 82685AA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685AA4: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 82685AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685AAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685AB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685AB8: 386AAAD0  addi r3, r10, -0x5530
	ctx.r[3].s64 = ctx.r[10].s64 + -21808;
	// 82685ABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685AE0: 4BDE1341  bl 0x82466e20
	ctx.lr = 0x82685AE4;
	sub_82466E20(ctx, base);
	// 82685AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82685AF8 size=24
    let mut pc: u32 = 0x82685AF8;
    'dispatch: loop {
        match pc {
            0x82685AF8 => {
    //   block [0x82685AF8..0x82685B10)
	// 82685AF8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685AFC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82685B00: 394A4DB8  addi r10, r10, 0x4db8
	ctx.r[10].s64 = ctx.r[10].s64 + 19896;
	// 82685B04: 816B04FC  lwz r11, 0x4fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1276 as u32) ) } as u64;
	// 82685B08: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82685B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685B10 size=108
    let mut pc: u32 = 0x82685B10;
    'dispatch: loop {
        match pc {
            0x82685B10 => {
    //   block [0x82685B10..0x82685B7C)
	// 82685B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685B1C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685B24: 38EB4DB8  addi r7, r11, 0x4db8
	ctx.r[7].s64 = ctx.r[11].s64 + 19896;
	// 82685B28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685B2C: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 82685B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685B34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685B38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685B40: 386AAB00  addi r3, r10, -0x5500
	ctx.r[3].s64 = ctx.r[10].s64 + -21760;
	// 82685B44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685B68: 4BDE12B9  bl 0x82466e20
	ctx.lr = 0x82685B6C;
	sub_82466E20(ctx, base);
	// 82685B6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685B80 size=108
    let mut pc: u32 = 0x82685B80;
    'dispatch: loop {
        match pc {
            0x82685B80 => {
    //   block [0x82685B80..0x82685BEC)
	// 82685B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685B8C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685B94: 38EB06B0  addi r7, r11, 0x6b0
	ctx.r[7].s64 = ctx.r[11].s64 + 1712;
	// 82685B98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82685B9C: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 82685BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685BA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685BA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685BB0: 386AAB30  addi r3, r10, -0x54d0
	ctx.r[3].s64 = ctx.r[10].s64 + -21712;
	// 82685BB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685BD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685BD8: 4BDE1249  bl 0x82466e20
	ctx.lr = 0x82685BDC;
	sub_82466E20(ctx, base);
	// 82685BDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82685BF0 size=24
    let mut pc: u32 = 0x82685BF0;
    'dispatch: loop {
        match pc {
            0x82685BF0 => {
    //   block [0x82685BF0..0x82685C08)
	// 82685BF0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685BF4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82685BF8: 394A4DE8  addi r10, r10, 0x4de8
	ctx.r[10].s64 = ctx.r[10].s64 + 19944;
	// 82685BFC: 816B04FC  lwz r11, 0x4fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1276 as u32) ) } as u64;
	// 82685C00: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82685C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685C08 size=108
    let mut pc: u32 = 0x82685C08;
    'dispatch: loop {
        match pc {
            0x82685C08 => {
    //   block [0x82685C08..0x82685C74)
	// 82685C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685C14: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685C1C: 38EB4DE8  addi r7, r11, 0x4de8
	ctx.r[7].s64 = ctx.r[11].s64 + 19944;
	// 82685C20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685C24: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 82685C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685C2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685C30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685C38: 386AAB60  addi r3, r10, -0x54a0
	ctx.r[3].s64 = ctx.r[10].s64 + -21664;
	// 82685C3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685C5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685C60: 4BDE11C1  bl 0x82466e20
	ctx.lr = 0x82685C64;
	sub_82466E20(ctx, base);
	// 82685C64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685C78 size=112
    let mut pc: u32 = 0x82685C78;
    'dispatch: loop {
        match pc {
            0x82685C78 => {
    //   block [0x82685C78..0x82685CE8)
	// 82685C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685C84: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82685C88: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685C8C: 392A5CC4  addi r9, r10, 0x5cc4
	ctx.r[9].s64 = ctx.r[10].s64 + 23748;
	// 82685C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685C94: 390B06C8  addi r8, r11, 0x6c8
	ctx.r[8].s64 = ctx.r[11].s64 + 1736;
	// 82685C98: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82685C9C: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 82685CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685CA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82685CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685CB0: 386AAB90  addi r3, r10, -0x5470
	ctx.r[3].s64 = ctx.r[10].s64 + -21616;
	// 82685CB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82685CB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82685CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685CCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685CD4: 4BDE114D  bl 0x82466e20
	ctx.lr = 0x82685CD8;
	sub_82466E20(ctx, base);
	// 82685CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685CE8 size=108
    let mut pc: u32 = 0x82685CE8;
    'dispatch: loop {
        match pc {
            0x82685CE8 => {
    //   block [0x82685CE8..0x82685D54)
	// 82685CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685CF4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685CF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685CFC: 38EB06F8  addi r7, r11, 0x6f8
	ctx.r[7].s64 = ctx.r[11].s64 + 1784;
	// 82685D00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685D04: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 82685D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685D0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685D10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685D18: 386AABC0  addi r3, r10, -0x5440
	ctx.r[3].s64 = ctx.r[10].s64 + -21568;
	// 82685D1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685D20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685D3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685D40: 4BDE10E1  bl 0x82466e20
	ctx.lr = 0x82685D44;
	sub_82466E20(ctx, base);
	// 82685D44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685D58 size=108
    let mut pc: u32 = 0x82685D58;
    'dispatch: loop {
        match pc {
            0x82685D58 => {
    //   block [0x82685D58..0x82685DC4)
	// 82685D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685D64: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685D6C: 38EB0728  addi r7, r11, 0x728
	ctx.r[7].s64 = ctx.r[11].s64 + 1832;
	// 82685D70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685D74: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 82685D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685D7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685D88: 386AABF0  addi r3, r10, -0x5410
	ctx.r[3].s64 = ctx.r[10].s64 + -21520;
	// 82685D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685DB0: 4BDE1071  bl 0x82466e20
	ctx.lr = 0x82685DB4;
	sub_82466E20(ctx, base);
	// 82685DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685DC8 size=112
    let mut pc: u32 = 0x82685DC8;
    'dispatch: loop {
        match pc {
            0x82685DC8 => {
    //   block [0x82685DC8..0x82685E38)
	// 82685DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685DD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685DD8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685DDC: 38AAAC50  addi r5, r10, -0x53b0
	ctx.r[5].s64 = ctx.r[10].s64 + -21424;
	// 82685DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685DE4: 390B0758  addi r8, r11, 0x758
	ctx.r[8].s64 = ctx.r[11].s64 + 1880;
	// 82685DE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82685DEC: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82685DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685DF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82685DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685E00: 386AAC20  addi r3, r10, -0x53e0
	ctx.r[3].s64 = ctx.r[10].s64 + -21472;
	// 82685E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82685E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685E24: 4BDE0FFD  bl 0x82466e20
	ctx.lr = 0x82685E28;
	sub_82466E20(ctx, base);
	// 82685E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685E38 size=108
    let mut pc: u32 = 0x82685E38;
    'dispatch: loop {
        match pc {
            0x82685E38 => {
    //   block [0x82685E38..0x82685EA4)
	// 82685E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685E44: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685E4C: 38EB0770  addi r7, r11, 0x770
	ctx.r[7].s64 = ctx.r[11].s64 + 1904;
	// 82685E50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685E54: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 82685E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685E5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685E60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685E68: 386AAC50  addi r3, r10, -0x53b0
	ctx.r[3].s64 = ctx.r[10].s64 + -21424;
	// 82685E6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685E90: 4BDE0F91  bl 0x82466e20
	ctx.lr = 0x82685E94;
	sub_82466E20(ctx, base);
	// 82685E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685EA8 size=108
    let mut pc: u32 = 0x82685EA8;
    'dispatch: loop {
        match pc {
            0x82685EA8 => {
    //   block [0x82685EA8..0x82685F14)
	// 82685EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685EB4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685EBC: 38EB07A0  addi r7, r11, 0x7a0
	ctx.r[7].s64 = ctx.r[11].s64 + 1952;
	// 82685EC0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82685EC4: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 82685EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685ECC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685ED8: 386AAC80  addi r3, r10, -0x5380
	ctx.r[3].s64 = ctx.r[10].s64 + -21376;
	// 82685EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685F00: 4BDE0F21  bl 0x82466e20
	ctx.lr = 0x82685F04;
	sub_82466E20(ctx, base);
	// 82685F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685F18 size=108
    let mut pc: u32 = 0x82685F18;
    'dispatch: loop {
        match pc {
            0x82685F18 => {
    //   block [0x82685F18..0x82685F84)
	// 82685F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685F24: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685F2C: 38EB07B8  addi r7, r11, 0x7b8
	ctx.r[7].s64 = ctx.r[11].s64 + 1976;
	// 82685F30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685F34: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82685F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685F3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685F40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685F48: 386AACB0  addi r3, r10, -0x5350
	ctx.r[3].s64 = ctx.r[10].s64 + -21328;
	// 82685F4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685F70: 4BDE0EB1  bl 0x82466e20
	ctx.lr = 0x82685F74;
	sub_82466E20(ctx, base);
	// 82685F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685F88 size=108
    let mut pc: u32 = 0x82685F88;
    'dispatch: loop {
        match pc {
            0x82685F88 => {
    //   block [0x82685F88..0x82685FF4)
	// 82685F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685F94: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685F9C: 38EB07E8  addi r7, r11, 0x7e8
	ctx.r[7].s64 = ctx.r[11].s64 + 2024;
	// 82685FA0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82685FA4: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 82685FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685FAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685FB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685FB8: 386AACE0  addi r3, r10, -0x5320
	ctx.r[3].s64 = ctx.r[10].s64 + -21280;
	// 82685FBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685FE0: 4BDE0E41  bl 0x82466e20
	ctx.lr = 0x82685FE4;
	sub_82466E20(ctx, base);
	// 82685FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685FF8 size=108
    let mut pc: u32 = 0x82685FF8;
    'dispatch: loop {
        match pc {
            0x82685FF8 => {
    //   block [0x82685FF8..0x82686064)
	// 82685FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686004: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268600C: 38EB0890  addi r7, r11, 0x890
	ctx.r[7].s64 = ctx.r[11].s64 + 2192;
	// 82686010: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82686014: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82686018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268601C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82686024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686028: 386AAD10  addi r3, r10, -0x52f0
	ctx.r[3].s64 = ctx.r[10].s64 + -21232;
	// 8268602C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268603C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268604C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686050: 4BDE0DD1  bl 0x82466e20
	ctx.lr = 0x82686054;
	sub_82466E20(ctx, base);
	// 82686054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268605C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686068 size=108
    let mut pc: u32 = 0x82686068;
    'dispatch: loop {
        match pc {
            0x82686068 => {
    //   block [0x82686068..0x826860D4)
	// 82686068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268606C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686074: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268607C: 38EB08C0  addi r7, r11, 0x8c0
	ctx.r[7].s64 = ctx.r[11].s64 + 2240;
	// 82686080: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82686084: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82686088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268608C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82686094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686098: 386AAD40  addi r3, r10, -0x52c0
	ctx.r[3].s64 = ctx.r[10].s64 + -21184;
	// 8268609C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826860A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826860A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826860A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826860AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826860B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826860B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826860B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826860BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826860C0: 4BDE0D61  bl 0x82466e20
	ctx.lr = 0x826860C4;
	sub_82466E20(ctx, base);
	// 826860C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826860C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826860CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826860D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826860D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826860D8 size=24
    let mut pc: u32 = 0x826860D8;
    'dispatch: loop {
        match pc {
            0x826860D8 => {
    //   block [0x826860D8..0x826860F0)
	// 826860D8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826860DC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 826860E0: 394A4E18  addi r10, r10, 0x4e18
	ctx.r[10].s64 = ctx.r[10].s64 + 19992;
	// 826860E4: 816B0980  lwz r11, 0x980(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2432 as u32) ) } as u64;
	// 826860E8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826860EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826860F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826860F0 size=112
    let mut pc: u32 = 0x826860F0;
    'dispatch: loop {
        match pc {
            0x826860F0 => {
    //   block [0x826860F0..0x82686160)
	// 826860F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826860F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826860F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826860FC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82686100: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686104: 392A5CF0  addi r9, r10, 0x5cf0
	ctx.r[9].s64 = ctx.r[10].s64 + 23792;
	// 82686108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268610C: 390B4E18  addi r8, r11, 0x4e18
	ctx.r[8].s64 = ctx.r[11].s64 + 19992;
	// 82686110: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82686114: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82686118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268611C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82686124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686128: 386AAD70  addi r3, r10, -0x5290
	ctx.r[3].s64 = ctx.r[10].s64 + -21136;
	// 8268612C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82686130: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82686134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268613C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686144: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268614C: 4BDE0CD5  bl 0x82466e20
	ctx.lr = 0x82686150;
	sub_82466E20(ctx, base);
	// 82686150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268615C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686160 size=108
    let mut pc: u32 = 0x82686160;
    'dispatch: loop {
        match pc {
            0x82686160 => {
    //   block [0x82686160..0x826861CC)
	// 82686160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268616C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686174: 38EB0988  addi r7, r11, 0x988
	ctx.r[7].s64 = ctx.r[11].s64 + 2440;
	// 82686178: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268617C: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82686180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686184: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268618C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686190: 386AADA0  addi r3, r10, -0x5260
	ctx.r[3].s64 = ctx.r[10].s64 + -21088;
	// 82686194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268619C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826861A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826861A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826861A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826861AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826861B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826861B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826861B8: 4BDE0C69  bl 0x82466e20
	ctx.lr = 0x826861BC;
	sub_82466E20(ctx, base);
	// 826861BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826861C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826861C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826861C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826861D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826861D0 size=112
    let mut pc: u32 = 0x826861D0;
    'dispatch: loop {
        match pc {
            0x826861D0 => {
    //   block [0x826861D0..0x82686240)
	// 826861D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826861D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826861D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826861DC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826861E0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826861E4: 392A5D34  addi r9, r10, 0x5d34
	ctx.r[9].s64 = ctx.r[10].s64 + 23860;
	// 826861E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826861EC: 390B09B8  addi r8, r11, 0x9b8
	ctx.r[8].s64 = ctx.r[11].s64 + 2488;
	// 826861F0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826861F4: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826861F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826861FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82686204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686208: 386AADD0  addi r3, r10, -0x5230
	ctx.r[3].s64 = ctx.r[10].s64 + -21040;
	// 8268620C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82686210: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82686214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268621C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686224: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268622C: 4BDE0BF5  bl 0x82466e20
	ctx.lr = 0x82686230;
	sub_82466E20(ctx, base);
	// 82686230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268623C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82686240 size=24
    let mut pc: u32 = 0x82686240;
    'dispatch: loop {
        match pc {
            0x82686240 => {
    //   block [0x82686240..0x82686258)
	// 82686240: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686244: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82686248: 394A4E90  addi r10, r10, 0x4e90
	ctx.r[10].s64 = ctx.r[10].s64 + 20112;
	// 8268624C: 816B0A78  lwz r11, 0xa78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2680 as u32) ) } as u64;
	// 82686250: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82686254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686258 size=112
    let mut pc: u32 = 0x82686258;
    'dispatch: loop {
        match pc {
            0x82686258 => {
    //   block [0x82686258..0x826862C8)
	// 82686258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268625C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686264: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82686268: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268626C: 392A5D70  addi r9, r10, 0x5d70
	ctx.r[9].s64 = ctx.r[10].s64 + 23920;
	// 82686270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686274: 390B4E90  addi r8, r11, 0x4e90
	ctx.r[8].s64 = ctx.r[11].s64 + 20112;
	// 82686278: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8268627C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 82686280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686284: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268628C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686290: 386AAE00  addi r3, r10, -0x5200
	ctx.r[3].s64 = ctx.r[10].s64 + -20992;
	// 82686294: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82686298: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268629C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826862A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826862A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826862A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826862AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826862B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826862B4: 4BDE0B6D  bl 0x82466e20
	ctx.lr = 0x826862B8;
	sub_82466E20(ctx, base);
	// 826862B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826862BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826862C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826862C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826862C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826862C8 size=108
    let mut pc: u32 = 0x826862C8;
    'dispatch: loop {
        match pc {
            0x826862C8 => {
    //   block [0x826862C8..0x82686334)
	// 826862C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826862CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826862D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826862D4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826862D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826862DC: 38EB0A7C  addi r7, r11, 0xa7c
	ctx.r[7].s64 = ctx.r[11].s64 + 2684;
	// 826862E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826862E4: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826862E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826862EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826862F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826862F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826862F8: 386AAE30  addi r3, r10, -0x51d0
	ctx.r[3].s64 = ctx.r[10].s64 + -20944;
	// 826862FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268630C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268631C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686320: 4BDE0B01  bl 0x82466e20
	ctx.lr = 0x82686324;
	sub_82466E20(ctx, base);
	// 82686324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268632C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686338 size=108
    let mut pc: u32 = 0x82686338;
    'dispatch: loop {
        match pc {
            0x82686338 => {
    //   block [0x82686338..0x826863A4)
	// 82686338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268633C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686344: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268634C: 38EB0A94  addi r7, r11, 0xa94
	ctx.r[7].s64 = ctx.r[11].s64 + 2708;
	// 82686350: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82686354: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82686358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268635C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686360: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82686364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686368: 386AAE60  addi r3, r10, -0x51a0
	ctx.r[3].s64 = ctx.r[10].s64 + -20896;
	// 8268636C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268637C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268638C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686390: 4BDE0A91  bl 0x82466e20
	ctx.lr = 0x82686394;
	sub_82466E20(ctx, base);
	// 82686394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268639C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826863A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826863A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826863A8 size=24
    let mut pc: u32 = 0x826863A8;
    'dispatch: loop {
        match pc {
            0x826863A8 => {
    //   block [0x826863A8..0x826863C0)
	// 826863A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826863AC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 826863B0: 394A4ED8  addi r10, r10, 0x4ed8
	ctx.r[10].s64 = ctx.r[10].s64 + 20184;
	// 826863B4: 816B0AC4  lwz r11, 0xac4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2756 as u32) ) } as u64;
	// 826863B8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826863BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826863C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826863C0 size=112
    let mut pc: u32 = 0x826863C0;
    'dispatch: loop {
        match pc {
            0x826863C0 => {
    //   block [0x826863C0..0x82686430)
	// 826863C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826863C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826863C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826863CC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826863D0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826863D4: 392A5DAC  addi r9, r10, 0x5dac
	ctx.r[9].s64 = ctx.r[10].s64 + 23980;
	// 826863D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826863DC: 390B4ED8  addi r8, r11, 0x4ed8
	ctx.r[8].s64 = ctx.r[11].s64 + 20184;
	// 826863E0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826863E4: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826863E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826863EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826863F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826863F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826863F8: 386AAE90  addi r3, r10, -0x5170
	ctx.r[3].s64 = ctx.r[10].s64 + -20848;
	// 826863FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82686400: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82686404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268640C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686414: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268641C: 4BDE0A05  bl 0x82466e20
	ctx.lr = 0x82686420;
	sub_82466E20(ctx, base);
	// 82686420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268642C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686430 size=108
    let mut pc: u32 = 0x82686430;
    'dispatch: loop {
        match pc {
            0x82686430 => {
    //   block [0x82686430..0x8268649C)
	// 82686430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268643C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686444: 38EB0AC8  addi r7, r11, 0xac8
	ctx.r[7].s64 = ctx.r[11].s64 + 2760;
	// 82686448: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268644C: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 82686450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686454: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686458: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268645C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686460: 386AAEC0  addi r3, r10, -0x5140
	ctx.r[3].s64 = ctx.r[10].s64 + -20800;
	// 82686464: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268646C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268647C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686488: 4BDE0999  bl 0x82466e20
	ctx.lr = 0x8268648C;
	sub_82466E20(ctx, base);
	// 8268648C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


