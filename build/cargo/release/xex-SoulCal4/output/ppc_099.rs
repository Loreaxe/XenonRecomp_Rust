pub fn sub_8262FDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FDB0 size=112
    let mut pc: u32 = 0x8262FDB0;
    'dispatch: loop {
        match pc {
            0x8262FDB0 => {
    //   block [0x8262FDB0..0x8262FE20)
	// 8262FDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FDB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FDBC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262FDC0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FDC4: 392A4C2C  addi r9, r10, 0x4c2c
	ctx.r[9].s64 = ctx.r[10].s64 + 19500;
	// 8262FDC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FDCC: 390B2020  addi r8, r11, 0x2020
	ctx.r[8].s64 = ctx.r[11].s64 + 8224;
	// 8262FDD0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8262FDD4: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 8262FDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FDDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FDE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262FDE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FDE8: 386A6634  addi r3, r10, 0x6634
	ctx.r[3].s64 = ctx.r[10].s64 + 26164;
	// 8262FDEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262FDF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262FDF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FE04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FE0C: 4BE37015  bl 0x82466e20
	ctx.lr = 0x8262FE10;
	sub_82466E20(ctx, base);
	// 8262FE10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FE20 size=108
    let mut pc: u32 = 0x8262FE20;
    'dispatch: loop {
        match pc {
            0x8262FE20 => {
    //   block [0x8262FE20..0x8262FE8C)
	// 8262FE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FE2C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FE30: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FE34: 38EBD1A0  addi r7, r11, -0x2e60
	ctx.r[7].s64 = ctx.r[11].s64 + -11872;
	// 8262FE38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262FE3C: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 8262FE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FE44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FE48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262FE4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FE50: 386A6664  addi r3, r10, 0x6664
	ctx.r[3].s64 = ctx.r[10].s64 + 26212;
	// 8262FE54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262FE58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FE60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FE68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FE6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FE70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FE74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FE78: 4BE36FA9  bl 0x82466e20
	ctx.lr = 0x8262FE7C;
	sub_82466E20(ctx, base);
	// 8262FE7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FE80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FE84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FE90 size=108
    let mut pc: u32 = 0x8262FE90;
    'dispatch: loop {
        match pc {
            0x8262FE90 => {
    //   block [0x8262FE90..0x8262FEFC)
	// 8262FE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FE98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FE9C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FEA0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FEA4: 38EBD1D0  addi r7, r11, -0x2e30
	ctx.r[7].s64 = ctx.r[11].s64 + -11824;
	// 8262FEA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262FEAC: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 8262FEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FEB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FEB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262FEBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FEC0: 386A6694  addi r3, r10, 0x6694
	ctx.r[3].s64 = ctx.r[10].s64 + 26260;
	// 8262FEC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262FEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FEE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FEE8: 4BE36F39  bl 0x82466e20
	ctx.lr = 0x8262FEEC;
	sub_82466E20(ctx, base);
	// 8262FEEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FEF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FEF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FEF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FF00 size=112
    let mut pc: u32 = 0x8262FF00;
    'dispatch: loop {
        match pc {
            0x8262FF00 => {
    //   block [0x8262FF00..0x8262FF70)
	// 8262FF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FF08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FF0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262FF10: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FF14: 392A4C50  addi r9, r10, 0x4c50
	ctx.r[9].s64 = ctx.r[10].s64 + 19536;
	// 8262FF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262FF1C: 390BD208  addi r8, r11, -0x2df8
	ctx.r[8].s64 = ctx.r[11].s64 + -11768;
	// 8262FF20: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8262FF24: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 8262FF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FF2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FF30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262FF34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FF38: 386A66C4  addi r3, r10, 0x66c4
	ctx.r[3].s64 = ctx.r[10].s64 + 26308;
	// 8262FF3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262FF40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262FF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FF54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FF5C: 4BE36EC5  bl 0x82466e20
	ctx.lr = 0x8262FF60;
	sub_82466E20(ctx, base);
	// 8262FF60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FF64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FF68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FF70 size=108
    let mut pc: u32 = 0x8262FF70;
    'dispatch: loop {
        match pc {
            0x8262FF70 => {
    //   block [0x8262FF70..0x8262FFDC)
	// 8262FF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FF78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FF7C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FF80: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FF84: 38EBD268  addi r7, r11, -0x2d98
	ctx.r[7].s64 = ctx.r[11].s64 + -11672;
	// 8262FF88: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8262FF8C: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 8262FF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FF94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FF98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262FF9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FFA0: 386A66F4  addi r3, r10, 0x66f4
	ctx.r[3].s64 = ctx.r[10].s64 + 26356;
	// 8262FFA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262FFA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FFAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FFB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FFB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FFC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FFC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FFC8: 4BE36E59  bl 0x82466e20
	ctx.lr = 0x8262FFCC;
	sub_82466E20(ctx, base);
	// 8262FFCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FFD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FFD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FFD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FFE0 size=108
    let mut pc: u32 = 0x8262FFE0;
    'dispatch: loop {
        match pc {
            0x8262FFE0 => {
    //   block [0x8262FFE0..0x8263004C)
	// 8262FFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FFE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FFEC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FFF0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FFF4: 38EBD328  addi r7, r11, -0x2cd8
	ctx.r[7].s64 = ctx.r[11].s64 + -11480;
	// 8262FFF8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262FFFC: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 82630000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630004: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630008: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263000C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630010: 386A6724  addi r3, r10, 0x6724
	ctx.r[3].s64 = ctx.r[10].s64 + 26404;
	// 82630014: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263001C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263002C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630034: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630038: 4BE36DE9  bl 0x82466e20
	ctx.lr = 0x8263003C;
	sub_82466E20(ctx, base);
	// 8263003C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630050 size=108
    let mut pc: u32 = 0x82630050;
    'dispatch: loop {
        match pc {
            0x82630050 => {
    //   block [0x82630050..0x826300BC)
	// 82630050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263005C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630064: 38EBD340  addi r7, r11, -0x2cc0
	ctx.r[7].s64 = ctx.r[11].s64 + -11456;
	// 82630068: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8263006C: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82630070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630074: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630078: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263007C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630080: 386A6754  addi r3, r10, 0x6754
	ctx.r[3].s64 = ctx.r[10].s64 + 26452;
	// 82630084: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263008C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263009C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826300A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826300A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826300A8: 4BE36D79  bl 0x82466e20
	ctx.lr = 0x826300AC;
	sub_82466E20(ctx, base);
	// 826300AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826300B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826300B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826300B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826300C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826300C0 size=24
    let mut pc: u32 = 0x826300C0;
    'dispatch: loop {
        match pc {
            0x826300C0 => {
    //   block [0x826300C0..0x826300D8)
	// 826300C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826300C4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 826300C8: 394A20B0  addi r10, r10, 0x20b0
	ctx.r[10].s64 = ctx.r[10].s64 + 8368;
	// 826300CC: 816BD204  lwz r11, -0x2dfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11772 as u32) ) } as u64;
	// 826300D0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826300D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826300D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826300D8 size=108
    let mut pc: u32 = 0x826300D8;
    'dispatch: loop {
        match pc {
            0x826300D8 => {
    //   block [0x826300D8..0x82630144)
	// 826300D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826300DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826300E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826300E4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826300E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826300EC: 38EB20B0  addi r7, r11, 0x20b0
	ctx.r[7].s64 = ctx.r[11].s64 + 8368;
	// 826300F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826300F4: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826300F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826300FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630108: 386A6784  addi r3, r10, 0x6784
	ctx.r[3].s64 = ctx.r[10].s64 + 26500;
	// 8263010C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263011C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263012C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630130: 4BE36CF1  bl 0x82466e20
	ctx.lr = 0x82630134;
	sub_82466E20(ctx, base);
	// 82630134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263013C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82630148 size=24
    let mut pc: u32 = 0x82630148;
    'dispatch: loop {
        match pc {
            0x82630148 => {
    //   block [0x82630148..0x82630160)
	// 82630148: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263014C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82630150: 394A20E0  addi r10, r10, 0x20e0
	ctx.r[10].s64 = ctx.r[10].s64 + 8416;
	// 82630154: 816BD204  lwz r11, -0x2dfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11772 as u32) ) } as u64;
	// 82630158: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263015C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630160 size=108
    let mut pc: u32 = 0x82630160;
    'dispatch: loop {
        match pc {
            0x82630160 => {
    //   block [0x82630160..0x826301CC)
	// 82630160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263016C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630174: 38EB20E0  addi r7, r11, 0x20e0
	ctx.r[7].s64 = ctx.r[11].s64 + 8416;
	// 82630178: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263017C: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 82630180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630184: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263018C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630190: 386A67B4  addi r3, r10, 0x67b4
	ctx.r[3].s64 = ctx.r[10].s64 + 26548;
	// 82630194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263019C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826301A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826301A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826301A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826301AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826301B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826301B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826301B8: 4BE36C69  bl 0x82466e20
	ctx.lr = 0x826301BC;
	sub_82466E20(ctx, base);
	// 826301BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826301C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826301C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826301C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826301D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826301D0 size=108
    let mut pc: u32 = 0x826301D0;
    'dispatch: loop {
        match pc {
            0x826301D0 => {
    //   block [0x826301D0..0x8263023C)
	// 826301D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826301D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826301D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826301DC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826301E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826301E4: 38EBD3B8  addi r7, r11, -0x2c48
	ctx.r[7].s64 = ctx.r[11].s64 + -11336;
	// 826301E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826301EC: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826301F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826301F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826301F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826301FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630200: 386A67E4  addi r3, r10, 0x67e4
	ctx.r[3].s64 = ctx.r[10].s64 + 26596;
	// 82630204: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263020C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263021C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630224: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630228: 4BE36BF9  bl 0x82466e20
	ctx.lr = 0x8263022C;
	sub_82466E20(ctx, base);
	// 8263022C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82630240 size=24
    let mut pc: u32 = 0x82630240;
    'dispatch: loop {
        match pc {
            0x82630240 => {
    //   block [0x82630240..0x82630258)
	// 82630240: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630244: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82630248: 394A2110  addi r10, r10, 0x2110
	ctx.r[10].s64 = ctx.r[10].s64 + 8464;
	// 8263024C: 816BD204  lwz r11, -0x2dfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11772 as u32) ) } as u64;
	// 82630250: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82630254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630258 size=108
    let mut pc: u32 = 0x82630258;
    'dispatch: loop {
        match pc {
            0x82630258 => {
    //   block [0x82630258..0x826302C4)
	// 82630258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263025C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630264: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263026C: 38EB2110  addi r7, r11, 0x2110
	ctx.r[7].s64 = ctx.r[11].s64 + 8464;
	// 82630270: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82630274: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 82630278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263027C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630280: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630288: 386A6814  addi r3, r10, 0x6814
	ctx.r[3].s64 = ctx.r[10].s64 + 26644;
	// 8263028C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263029C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826302A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826302A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826302A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826302AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826302B0: 4BE36B71  bl 0x82466e20
	ctx.lr = 0x826302B4;
	sub_82466E20(ctx, base);
	// 826302B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826302B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826302BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826302C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826302C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826302C8 size=112
    let mut pc: u32 = 0x826302C8;
    'dispatch: loop {
        match pc {
            0x826302C8 => {
    //   block [0x826302C8..0x82630338)
	// 826302C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826302CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826302D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826302D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826302D8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826302DC: 392A4C94  addi r9, r10, 0x4c94
	ctx.r[9].s64 = ctx.r[10].s64 + 19604;
	// 826302E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826302E4: 390BD3D0  addi r8, r11, -0x2c30
	ctx.r[8].s64 = ctx.r[11].s64 + -11312;
	// 826302E8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826302EC: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826302F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826302F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826302F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826302FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630300: 386A6844  addi r3, r10, 0x6844
	ctx.r[3].s64 = ctx.r[10].s64 + 26692;
	// 82630304: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82630308: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263030C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263031C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630324: 4BE36AFD  bl 0x82466e20
	ctx.lr = 0x82630328;
	sub_82466E20(ctx, base);
	// 82630328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263032C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630338 size=108
    let mut pc: u32 = 0x82630338;
    'dispatch: loop {
        match pc {
            0x82630338 => {
    //   block [0x82630338..0x826303A4)
	// 82630338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263033C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630344: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263034C: 38EBD400  addi r7, r11, -0x2c00
	ctx.r[7].s64 = ctx.r[11].s64 + -11264;
	// 82630350: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82630354: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 82630358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263035C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630360: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630368: 386A6874  addi r3, r10, 0x6874
	ctx.r[3].s64 = ctx.r[10].s64 + 26740;
	// 8263036C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263037C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263038C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630390: 4BE36A91  bl 0x82466e20
	ctx.lr = 0x82630394;
	sub_82466E20(ctx, base);
	// 82630394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263039C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826303A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826303A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826303A8 size=108
    let mut pc: u32 = 0x826303A8;
    'dispatch: loop {
        match pc {
            0x826303A8 => {
    //   block [0x826303A8..0x82630414)
	// 826303A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826303AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826303B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826303B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826303B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826303BC: 38EBD430  addi r7, r11, -0x2bd0
	ctx.r[7].s64 = ctx.r[11].s64 + -11216;
	// 826303C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826303C4: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826303C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826303CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826303D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826303D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826303D8: 386A68A4  addi r3, r10, 0x68a4
	ctx.r[3].s64 = ctx.r[10].s64 + 26788;
	// 826303DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826303E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826303E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826303E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826303EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826303F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826303F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826303F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826303FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630400: 4BE36A21  bl 0x82466e20
	ctx.lr = 0x82630404;
	sub_82466E20(ctx, base);
	// 82630404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263040C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630418 size=112
    let mut pc: u32 = 0x82630418;
    'dispatch: loop {
        match pc {
            0x82630418 => {
    //   block [0x82630418..0x82630488)
	// 82630418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263041C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630428: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263042C: 38AA6904  addi r5, r10, 0x6904
	ctx.r[5].s64 = ctx.r[10].s64 + 26884;
	// 82630430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630434: 390BD460  addi r8, r11, -0x2ba0
	ctx.r[8].s64 = ctx.r[11].s64 + -11168;
	// 82630438: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263043C: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82630440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630444: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263044C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630450: 386A68D4  addi r3, r10, 0x68d4
	ctx.r[3].s64 = ctx.r[10].s64 + 26836;
	// 82630454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82630458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263045C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263046C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630474: 4BE369AD  bl 0x82466e20
	ctx.lr = 0x82630478;
	sub_82466E20(ctx, base);
	// 82630478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263047C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630488 size=108
    let mut pc: u32 = 0x82630488;
    'dispatch: loop {
        match pc {
            0x82630488 => {
    //   block [0x82630488..0x826304F4)
	// 82630488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630494: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263049C: 38EBD478  addi r7, r11, -0x2b88
	ctx.r[7].s64 = ctx.r[11].s64 + -11144;
	// 826304A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826304A4: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826304A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826304AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826304B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826304B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826304B8: 386A6904  addi r3, r10, 0x6904
	ctx.r[3].s64 = ctx.r[10].s64 + 26884;
	// 826304BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826304C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826304C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826304C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826304CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826304D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826304D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826304D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826304DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826304E0: 4BE36941  bl 0x82466e20
	ctx.lr = 0x826304E4;
	sub_82466E20(ctx, base);
	// 826304E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826304E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826304EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826304F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826304F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826304F8 size=108
    let mut pc: u32 = 0x826304F8;
    'dispatch: loop {
        match pc {
            0x826304F8 => {
    //   block [0x826304F8..0x82630564)
	// 826304F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826304FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630504: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263050C: 38EBD4A8  addi r7, r11, -0x2b58
	ctx.r[7].s64 = ctx.r[11].s64 + -11096;
	// 82630510: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82630514: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 82630518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263051C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630520: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630528: 386A6934  addi r3, r10, 0x6934
	ctx.r[3].s64 = ctx.r[10].s64 + 26932;
	// 8263052C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263053C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263054C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630550: 4BE368D1  bl 0x82466e20
	ctx.lr = 0x82630554;
	sub_82466E20(ctx, base);
	// 82630554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263055C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630568 size=108
    let mut pc: u32 = 0x82630568;
    'dispatch: loop {
        match pc {
            0x82630568 => {
    //   block [0x82630568..0x826305D4)
	// 82630568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263056C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630574: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263057C: 38EBD4C0  addi r7, r11, -0x2b40
	ctx.r[7].s64 = ctx.r[11].s64 + -11072;
	// 82630580: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82630584: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82630588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263058C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630598: 386A6964  addi r3, r10, 0x6964
	ctx.r[3].s64 = ctx.r[10].s64 + 26980;
	// 8263059C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826305A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826305A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826305A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826305AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826305B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826305B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826305B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826305BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826305C0: 4BE36861  bl 0x82466e20
	ctx.lr = 0x826305C4;
	sub_82466E20(ctx, base);
	// 826305C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826305C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826305CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826305D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826305D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826305D8 size=108
    let mut pc: u32 = 0x826305D8;
    'dispatch: loop {
        match pc {
            0x826305D8 => {
    //   block [0x826305D8..0x82630644)
	// 826305D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826305DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826305E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826305E4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826305E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826305EC: 38EBD4F0  addi r7, r11, -0x2b10
	ctx.r[7].s64 = ctx.r[11].s64 + -11024;
	// 826305F0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826305F4: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826305F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826305FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630608: 386A6994  addi r3, r10, 0x6994
	ctx.r[3].s64 = ctx.r[10].s64 + 27028;
	// 8263060C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263061C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263062C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630630: 4BE367F1  bl 0x82466e20
	ctx.lr = 0x82630634;
	sub_82466E20(ctx, base);
	// 82630634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263063C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630648 size=108
    let mut pc: u32 = 0x82630648;
    'dispatch: loop {
        match pc {
            0x82630648 => {
    //   block [0x82630648..0x826306B4)
	// 82630648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630654: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263065C: 38EBD598  addi r7, r11, -0x2a68
	ctx.r[7].s64 = ctx.r[11].s64 + -10856;
	// 82630660: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82630664: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 82630668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263066C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630670: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630678: 386A69C4  addi r3, r10, 0x69c4
	ctx.r[3].s64 = ctx.r[10].s64 + 27076;
	// 8263067C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263068C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263069C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826306A0: 4BE36781  bl 0x82466e20
	ctx.lr = 0x826306A4;
	sub_82466E20(ctx, base);
	// 826306A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826306A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826306AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826306B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826306B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826306B8 size=108
    let mut pc: u32 = 0x826306B8;
    'dispatch: loop {
        match pc {
            0x826306B8 => {
    //   block [0x826306B8..0x82630724)
	// 826306B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826306BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826306C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826306C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826306C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826306CC: 38EBD5C8  addi r7, r11, -0x2a38
	ctx.r[7].s64 = ctx.r[11].s64 + -10808;
	// 826306D0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826306D4: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826306D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826306DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826306E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826306E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826306E8: 386A69F4  addi r3, r10, 0x69f4
	ctx.r[3].s64 = ctx.r[10].s64 + 27124;
	// 826306EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826306F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826306F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826306F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826306FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263070C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630710: 4BE36711  bl 0x82466e20
	ctx.lr = 0x82630714;
	sub_82466E20(ctx, base);
	// 82630714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263071C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630728 size=108
    let mut pc: u32 = 0x82630728;
    'dispatch: loop {
        match pc {
            0x82630728 => {
    //   block [0x82630728..0x82630794)
	// 82630728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263072C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630734: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263073C: 38EBD5E0  addi r7, r11, -0x2a20
	ctx.r[7].s64 = ctx.r[11].s64 + -10784;
	// 82630740: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82630744: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82630748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263074C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630750: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630758: 386A6A24  addi r3, r10, 0x6a24
	ctx.r[3].s64 = ctx.r[10].s64 + 27172;
	// 8263075C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263076C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263077C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630780: 4BE366A1  bl 0x82466e20
	ctx.lr = 0x82630784;
	sub_82466E20(ctx, base);
	// 82630784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263078C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630798 size=108
    let mut pc: u32 = 0x82630798;
    'dispatch: loop {
        match pc {
            0x82630798 => {
    //   block [0x82630798..0x82630804)
	// 82630798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263079C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826307A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826307A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826307A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826307AC: 38EBD610  addi r7, r11, -0x29f0
	ctx.r[7].s64 = ctx.r[11].s64 + -10736;
	// 826307B0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826307B4: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826307B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826307BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826307C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826307C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826307C8: 386A6A54  addi r3, r10, 0x6a54
	ctx.r[3].s64 = ctx.r[10].s64 + 27220;
	// 826307CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826307D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826307D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826307D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826307DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826307E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826307E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826307E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826307EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826307F0: 4BE36631  bl 0x82466e20
	ctx.lr = 0x826307F4;
	sub_82466E20(ctx, base);
	// 826307F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826307F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826307FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82630808 size=24
    let mut pc: u32 = 0x82630808;
    'dispatch: loop {
        match pc {
            0x82630808 => {
    //   block [0x82630808..0x82630820)
	// 82630808: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263080C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82630810: 394A2140  addi r10, r10, 0x2140
	ctx.r[10].s64 = ctx.r[10].s64 + 8512;
	// 82630814: 816BD6D0  lwz r11, -0x2930(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10544 as u32) ) } as u64;
	// 82630818: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263081C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630820 size=112
    let mut pc: u32 = 0x82630820;
    'dispatch: loop {
        match pc {
            0x82630820 => {
    //   block [0x82630820..0x82630890)
	// 82630820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263082C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82630830: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630834: 392A4CC0  addi r9, r10, 0x4cc0
	ctx.r[9].s64 = ctx.r[10].s64 + 19648;
	// 82630838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263083C: 390B2140  addi r8, r11, 0x2140
	ctx.r[8].s64 = ctx.r[11].s64 + 8512;
	// 82630840: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82630844: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82630848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263084C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82630854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630858: 386A6A84  addi r3, r10, 0x6a84
	ctx.r[3].s64 = ctx.r[10].s64 + 27268;
	// 8263085C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82630860: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82630864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263086C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263087C: 4BE365A5  bl 0x82466e20
	ctx.lr = 0x82630880;
	sub_82466E20(ctx, base);
	// 82630880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263088C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630890 size=108
    let mut pc: u32 = 0x82630890;
    'dispatch: loop {
        match pc {
            0x82630890 => {
    //   block [0x82630890..0x826308FC)
	// 82630890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263089C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826308A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826308A4: 38EBD6D8  addi r7, r11, -0x2928
	ctx.r[7].s64 = ctx.r[11].s64 + -10536;
	// 826308A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826308AC: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826308B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826308B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826308B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826308BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826308C0: 386A6AB4  addi r3, r10, 0x6ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 27316;
	// 826308C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826308C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826308CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826308D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826308D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826308D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826308DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826308E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826308E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826308E8: 4BE36539  bl 0x82466e20
	ctx.lr = 0x826308EC;
	sub_82466E20(ctx, base);
	// 826308EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826308F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826308F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826308F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630900 size=112
    let mut pc: u32 = 0x82630900;
    'dispatch: loop {
        match pc {
            0x82630900 => {
    //   block [0x82630900..0x82630970)
	// 82630900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263090C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82630910: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630914: 392A4D04  addi r9, r10, 0x4d04
	ctx.r[9].s64 = ctx.r[10].s64 + 19716;
	// 82630918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263091C: 390BD708  addi r8, r11, -0x28f8
	ctx.r[8].s64 = ctx.r[11].s64 + -10488;
	// 82630920: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82630924: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82630928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263092C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630930: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82630934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630938: 386A6AE4  addi r3, r10, 0x6ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 27364;
	// 8263093C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82630940: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82630944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263094C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263095C: 4BE364C5  bl 0x82466e20
	ctx.lr = 0x82630960;
	sub_82466E20(ctx, base);
	// 82630960: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263096C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82630970 size=24
    let mut pc: u32 = 0x82630970;
    'dispatch: loop {
        match pc {
            0x82630970 => {
    //   block [0x82630970..0x82630988)
	// 82630970: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630974: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82630978: 394A21B8  addi r10, r10, 0x21b8
	ctx.r[10].s64 = ctx.r[10].s64 + 8632;
	// 8263097C: 816BD7C8  lwz r11, -0x2838(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10296 as u32) ) } as u64;
	// 82630980: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82630984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630988 size=112
    let mut pc: u32 = 0x82630988;
    'dispatch: loop {
        match pc {
            0x82630988 => {
    //   block [0x82630988..0x826309F8)
	// 82630988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263098C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630994: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82630998: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263099C: 392A4D40  addi r9, r10, 0x4d40
	ctx.r[9].s64 = ctx.r[10].s64 + 19776;
	// 826309A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826309A4: 390B21B8  addi r8, r11, 0x21b8
	ctx.r[8].s64 = ctx.r[11].s64 + 8632;
	// 826309A8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826309AC: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826309B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826309B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826309B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826309BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826309C0: 386A6B14  addi r3, r10, 0x6b14
	ctx.r[3].s64 = ctx.r[10].s64 + 27412;
	// 826309C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826309C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826309CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826309D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826309D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826309D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826309DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826309E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826309E4: 4BE3643D  bl 0x82466e20
	ctx.lr = 0x826309E8;
	sub_82466E20(ctx, base);
	// 826309E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826309EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826309F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826309F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826309F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826309F8 size=108
    let mut pc: u32 = 0x826309F8;
    'dispatch: loop {
        match pc {
            0x826309F8 => {
    //   block [0x826309F8..0x82630A64)
	// 826309F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826309FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630A04: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630A0C: 38EBD7CC  addi r7, r11, -0x2834
	ctx.r[7].s64 = ctx.r[11].s64 + -10292;
	// 82630A10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82630A14: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 82630A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630A1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630A20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630A28: 386A6B44  addi r3, r10, 0x6b44
	ctx.r[3].s64 = ctx.r[10].s64 + 27460;
	// 82630A2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630A50: 4BE363D1  bl 0x82466e20
	ctx.lr = 0x82630A54;
	sub_82466E20(ctx, base);
	// 82630A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630A68 size=108
    let mut pc: u32 = 0x82630A68;
    'dispatch: loop {
        match pc {
            0x82630A68 => {
    //   block [0x82630A68..0x82630AD4)
	// 82630A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630A74: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630A7C: 38EBD7E4  addi r7, r11, -0x281c
	ctx.r[7].s64 = ctx.r[11].s64 + -10268;
	// 82630A80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82630A84: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82630A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630A8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630A90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630A98: 386A6B74  addi r3, r10, 0x6b74
	ctx.r[3].s64 = ctx.r[10].s64 + 27508;
	// 82630A9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630AC0: 4BE36361  bl 0x82466e20
	ctx.lr = 0x82630AC4;
	sub_82466E20(ctx, base);
	// 82630AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82630AD8 size=24
    let mut pc: u32 = 0x82630AD8;
    'dispatch: loop {
        match pc {
            0x82630AD8 => {
    //   block [0x82630AD8..0x82630AF0)
	// 82630AD8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630ADC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82630AE0: 394A2200  addi r10, r10, 0x2200
	ctx.r[10].s64 = ctx.r[10].s64 + 8704;
	// 82630AE4: 816BD814  lwz r11, -0x27ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10220 as u32) ) } as u64;
	// 82630AE8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82630AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630AF0 size=112
    let mut pc: u32 = 0x82630AF0;
    'dispatch: loop {
        match pc {
            0x82630AF0 => {
    //   block [0x82630AF0..0x82630B60)
	// 82630AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630AFC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82630B00: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630B04: 392A4D7C  addi r9, r10, 0x4d7c
	ctx.r[9].s64 = ctx.r[10].s64 + 19836;
	// 82630B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630B0C: 390B2200  addi r8, r11, 0x2200
	ctx.r[8].s64 = ctx.r[11].s64 + 8704;
	// 82630B10: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82630B14: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 82630B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630B1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630B20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82630B24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630B28: 386A6BA4  addi r3, r10, 0x6ba4
	ctx.r[3].s64 = ctx.r[10].s64 + 27556;
	// 82630B2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82630B30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82630B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630B4C: 4BE362D5  bl 0x82466e20
	ctx.lr = 0x82630B50;
	sub_82466E20(ctx, base);
	// 82630B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630B60 size=108
    let mut pc: u32 = 0x82630B60;
    'dispatch: loop {
        match pc {
            0x82630B60 => {
    //   block [0x82630B60..0x82630BCC)
	// 82630B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630B6C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630B74: 38EBD818  addi r7, r11, -0x27e8
	ctx.r[7].s64 = ctx.r[11].s64 + -10216;
	// 82630B78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82630B7C: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 82630B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630B84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630B88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630B90: 386A6BD4  addi r3, r10, 0x6bd4
	ctx.r[3].s64 = ctx.r[10].s64 + 27604;
	// 82630B94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630BB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630BB8: 4BE36269  bl 0x82466e20
	ctx.lr = 0x82630BBC;
	sub_82466E20(ctx, base);
	// 82630BBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630BD0 size=108
    let mut pc: u32 = 0x82630BD0;
    'dispatch: loop {
        match pc {
            0x82630BD0 => {
    //   block [0x82630BD0..0x82630C3C)
	// 82630BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630BDC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630BE4: 38EBD830  addi r7, r11, -0x27d0
	ctx.r[7].s64 = ctx.r[11].s64 + -10192;
	// 82630BE8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82630BEC: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82630BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630BF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630BF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630C00: 386A6C04  addi r3, r10, 0x6c04
	ctx.r[3].s64 = ctx.r[10].s64 + 27652;
	// 82630C04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630C28: 4BE361F9  bl 0x82466e20
	ctx.lr = 0x82630C2C;
	sub_82466E20(ctx, base);
	// 82630C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630C40 size=108
    let mut pc: u32 = 0x82630C40;
    'dispatch: loop {
        match pc {
            0x82630C40 => {
    //   block [0x82630C40..0x82630CAC)
	// 82630C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630C4C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630C54: 38EBD878  addi r7, r11, -0x2788
	ctx.r[7].s64 = ctx.r[11].s64 + -10120;
	// 82630C58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82630C5C: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 82630C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630C64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630C70: 386A6C34  addi r3, r10, 0x6c34
	ctx.r[3].s64 = ctx.r[10].s64 + 27700;
	// 82630C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630C98: 4BE36189  bl 0x82466e20
	ctx.lr = 0x82630C9C;
	sub_82466E20(ctx, base);
	// 82630C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630CB0 size=108
    let mut pc: u32 = 0x82630CB0;
    'dispatch: loop {
        match pc {
            0x82630CB0 => {
    //   block [0x82630CB0..0x82630D1C)
	// 82630CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630CBC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630CC4: 38EBD8A8  addi r7, r11, -0x2758
	ctx.r[7].s64 = ctx.r[11].s64 + -10072;
	// 82630CC8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82630CCC: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82630CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630CD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630CE0: 386A6C64  addi r3, r10, 0x6c64
	ctx.r[3].s64 = ctx.r[10].s64 + 27748;
	// 82630CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630D08: 4BE36119  bl 0x82466e20
	ctx.lr = 0x82630D0C;
	sub_82466E20(ctx, base);
	// 82630D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630D20 size=108
    let mut pc: u32 = 0x82630D20;
    'dispatch: loop {
        match pc {
            0x82630D20 => {
    //   block [0x82630D20..0x82630D8C)
	// 82630D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630D2C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630D34: 38EBD9C8  addi r7, r11, -0x2638
	ctx.r[7].s64 = ctx.r[11].s64 + -9784;
	// 82630D38: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82630D3C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 82630D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630D44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630D48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630D50: 386A6C94  addi r3, r10, 0x6c94
	ctx.r[3].s64 = ctx.r[10].s64 + 27796;
	// 82630D54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630D78: 4BE360A9  bl 0x82466e20
	ctx.lr = 0x82630D7C;
	sub_82466E20(ctx, base);
	// 82630D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630D90 size=108
    let mut pc: u32 = 0x82630D90;
    'dispatch: loop {
        match pc {
            0x82630D90 => {
    //   block [0x82630D90..0x82630DFC)
	// 82630D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630D9C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630DA4: 38EBDA58  addi r7, r11, -0x25a8
	ctx.r[7].s64 = ctx.r[11].s64 + -9640;
	// 82630DA8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82630DAC: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 82630DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630DB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630DB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630DC0: 386A6CC4  addi r3, r10, 0x6cc4
	ctx.r[3].s64 = ctx.r[10].s64 + 27844;
	// 82630DC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630DE8: 4BE36039  bl 0x82466e20
	ctx.lr = 0x82630DEC;
	sub_82466E20(ctx, base);
	// 82630DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630E00 size=108
    let mut pc: u32 = 0x82630E00;
    'dispatch: loop {
        match pc {
            0x82630E00 => {
    //   block [0x82630E00..0x82630E6C)
	// 82630E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630E0C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630E14: 38EBDB18  addi r7, r11, -0x24e8
	ctx.r[7].s64 = ctx.r[11].s64 + -9448;
	// 82630E18: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82630E1C: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 82630E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630E24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630E28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630E30: 386A6CF4  addi r3, r10, 0x6cf4
	ctx.r[3].s64 = ctx.r[10].s64 + 27892;
	// 82630E34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630E54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630E58: 4BE35FC9  bl 0x82466e20
	ctx.lr = 0x82630E5C;
	sub_82466E20(ctx, base);
	// 82630E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630E70 size=108
    let mut pc: u32 = 0x82630E70;
    'dispatch: loop {
        match pc {
            0x82630E70 => {
    //   block [0x82630E70..0x82630EDC)
	// 82630E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630E7C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630E84: 38EBDBF0  addi r7, r11, -0x2410
	ctx.r[7].s64 = ctx.r[11].s64 + -9232;
	// 82630E88: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82630E8C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 82630E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630E94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630E98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630EA0: 386A6D24  addi r3, r10, 0x6d24
	ctx.r[3].s64 = ctx.r[10].s64 + 27940;
	// 82630EA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630EC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630EC8: 4BE35F59  bl 0x82466e20
	ctx.lr = 0x82630ECC;
	sub_82466E20(ctx, base);
	// 82630ECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630EE0 size=108
    let mut pc: u32 = 0x82630EE0;
    'dispatch: loop {
        match pc {
            0x82630EE0 => {
    //   block [0x82630EE0..0x82630F4C)
	// 82630EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630EEC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630EF4: 38EBDCB0  addi r7, r11, -0x2350
	ctx.r[7].s64 = ctx.r[11].s64 + -9040;
	// 82630EF8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82630EFC: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 82630F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630F04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630F08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630F10: 386A6D54  addi r3, r10, 0x6d54
	ctx.r[3].s64 = ctx.r[10].s64 + 27988;
	// 82630F14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630F34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630F38: 4BE35EE9  bl 0x82466e20
	ctx.lr = 0x82630F3C;
	sub_82466E20(ctx, base);
	// 82630F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630F50 size=112
    let mut pc: u32 = 0x82630F50;
    'dispatch: loop {
        match pc {
            0x82630F50 => {
    //   block [0x82630F50..0x82630FC0)
	// 82630F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630F5C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82630F60: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82630F64: 38EADD58  addi r7, r10, -0x22a8
	ctx.r[7].s64 = ctx.r[10].s64 + -8872;
	// 82630F68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630F6C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82630F70: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 82630F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630F78: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630F7C: 396B4D90  addi r11, r11, 0x4d90
	ctx.r[11].s64 = ctx.r[11].s64 + 19856;
	// 82630F80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630F84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630F88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630F8C: 386A6D84  addi r3, r10, 0x6d84
	ctx.r[3].s64 = ctx.r[10].s64 + 28036;
	// 82630F90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630F94: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82630F98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630F9C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82630FA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630FA4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630FA8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630FAC: 4BE35E75  bl 0x82466e20
	ctx.lr = 0x82630FB0;
	sub_82466E20(ctx, base);
	// 82630FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630FC0 size=108
    let mut pc: u32 = 0x82630FC0;
    'dispatch: loop {
        match pc {
            0x82630FC0 => {
    //   block [0x82630FC0..0x8263102C)
	// 82630FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630FCC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630FD4: 38EBDE78  addi r7, r11, -0x2188
	ctx.r[7].s64 = ctx.r[11].s64 + -8584;
	// 82630FD8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82630FDC: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 82630FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630FE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630FE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630FF0: 386A6DB4  addi r3, r10, 0x6db4
	ctx.r[3].s64 = ctx.r[10].s64 + 28084;
	// 82630FF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263100C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631018: 4BE35E09  bl 0x82466e20
	ctx.lr = 0x8263101C;
	sub_82466E20(ctx, base);
	// 8263101C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631030 size=108
    let mut pc: u32 = 0x82631030;
    'dispatch: loop {
        match pc {
            0x82631030 => {
    //   block [0x82631030..0x8263109C)
	// 82631030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263103C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631044: 38EBDED8  addi r7, r11, -0x2128
	ctx.r[7].s64 = ctx.r[11].s64 + -8488;
	// 82631048: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8263104C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 82631050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631054: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263105C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631060: 386A6DE4  addi r3, r10, 0x6de4
	ctx.r[3].s64 = ctx.r[10].s64 + 28132;
	// 82631064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82631068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263106C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263107C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631088: 4BE35D99  bl 0x82466e20
	ctx.lr = 0x8263108C;
	sub_82466E20(ctx, base);
	// 8263108C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826310A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826310A0 size=108
    let mut pc: u32 = 0x826310A0;
    'dispatch: loop {
        match pc {
            0x826310A0 => {
    //   block [0x826310A0..0x8263110C)
	// 826310A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826310A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826310A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826310AC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826310B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826310B4: 38EBDFE0  addi r7, r11, -0x2020
	ctx.r[7].s64 = ctx.r[11].s64 + -8224;
	// 826310B8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826310BC: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826310C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826310C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826310C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826310CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826310D0: 386A6E14  addi r3, r10, 0x6e14
	ctx.r[3].s64 = ctx.r[10].s64 + 28180;
	// 826310D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826310D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826310DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826310E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826310E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826310E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826310EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826310F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826310F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826310F8: 4BE35D29  bl 0x82466e20
	ctx.lr = 0x826310FC;
	sub_82466E20(ctx, base);
	// 826310FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631110 size=108
    let mut pc: u32 = 0x82631110;
    'dispatch: loop {
        match pc {
            0x82631110 => {
    //   block [0x82631110..0x8263117C)
	// 82631110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263111C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631124: 38EBE0B8  addi r7, r11, -0x1f48
	ctx.r[7].s64 = ctx.r[11].s64 + -8008;
	// 82631128: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263112C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 82631130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631134: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263113C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631140: 386A6E44  addi r3, r10, 0x6e44
	ctx.r[3].s64 = ctx.r[10].s64 + 28228;
	// 82631144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82631148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263114C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263115C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631168: 4BE35CB9  bl 0x82466e20
	ctx.lr = 0x8263116C;
	sub_82466E20(ctx, base);
	// 8263116C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631180 size=108
    let mut pc: u32 = 0x82631180;
    'dispatch: loop {
        match pc {
            0x82631180 => {
    //   block [0x82631180..0x826311EC)
	// 82631180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263118C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631194: 38EBE100  addi r7, r11, -0x1f00
	ctx.r[7].s64 = ctx.r[11].s64 + -7936;
	// 82631198: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263119C: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826311A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826311A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826311A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826311AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826311B0: 386A6E74  addi r3, r10, 0x6e74
	ctx.r[3].s64 = ctx.r[10].s64 + 28276;
	// 826311B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826311B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826311BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826311C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826311C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826311C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826311CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826311D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826311D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826311D8: 4BE35C49  bl 0x82466e20
	ctx.lr = 0x826311DC;
	sub_82466E20(ctx, base);
	// 826311DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826311E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826311E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826311E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826311F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826311F0 size=108
    let mut pc: u32 = 0x826311F0;
    'dispatch: loop {
        match pc {
            0x826311F0 => {
    //   block [0x826311F0..0x8263125C)
	// 826311F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826311F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826311F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826311FC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631204: 38EBE118  addi r7, r11, -0x1ee8
	ctx.r[7].s64 = ctx.r[11].s64 + -7912;
	// 82631208: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263120C: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 82631210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631214: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631218: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263121C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631220: 386A6EA4  addi r3, r10, 0x6ea4
	ctx.r[3].s64 = ctx.r[10].s64 + 28324;
	// 82631224: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82631228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263122C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263123C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631248: 4BE35BD9  bl 0x82466e20
	ctx.lr = 0x8263124C;
	sub_82466E20(ctx, base);
	// 8263124C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631260 size=108
    let mut pc: u32 = 0x82631260;
    'dispatch: loop {
        match pc {
            0x82631260 => {
    //   block [0x82631260..0x826312CC)
	// 82631260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263126C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631274: 38EBE160  addi r7, r11, -0x1ea0
	ctx.r[7].s64 = ctx.r[11].s64 + -7840;
	// 82631278: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263127C: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 82631280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631284: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263128C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631290: 386A6ED4  addi r3, r10, 0x6ed4
	ctx.r[3].s64 = ctx.r[10].s64 + 28372;
	// 82631294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82631298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263129C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826312A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826312A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826312A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826312AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826312B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826312B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826312B8: 4BE35B69  bl 0x82466e20
	ctx.lr = 0x826312BC;
	sub_82466E20(ctx, base);
	// 826312BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826312C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826312C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826312C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826312D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826312D0 size=112
    let mut pc: u32 = 0x826312D0;
    'dispatch: loop {
        match pc {
            0x826312D0 => {
    //   block [0x826312D0..0x82631340)
	// 826312D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826312D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826312D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826312DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826312E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826312E4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826312E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826312EC: 390BE178  addi r8, r11, -0x1e88
	ctx.r[8].s64 = ctx.r[11].s64 + -7816;
	// 826312F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826312F4: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 826312F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826312FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631308: 386A6F04  addi r3, r10, 0x6f04
	ctx.r[3].s64 = ctx.r[10].s64 + 28420;
	// 8263130C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263131C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263132C: 4BE35AF5  bl 0x82466e20
	ctx.lr = 0x82631330;
	sub_82466E20(ctx, base);
	// 82631330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263133C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631340 size=108
    let mut pc: u32 = 0x82631340;
    'dispatch: loop {
        match pc {
            0x82631340 => {
    //   block [0x82631340..0x826313AC)
	// 82631340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263134C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631354: 38EBE1C0  addi r7, r11, -0x1e40
	ctx.r[7].s64 = ctx.r[11].s64 + -7744;
	// 82631358: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263135C: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 82631360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631364: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263136C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631370: 386A6F34  addi r3, r10, 0x6f34
	ctx.r[3].s64 = ctx.r[10].s64 + 28468;
	// 82631374: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82631378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263137C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263138C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631398: 4BE35A89  bl 0x82466e20
	ctx.lr = 0x8263139C;
	sub_82466E20(ctx, base);
	// 8263139C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826313A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826313A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826313A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826313B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826313B0 size=112
    let mut pc: u32 = 0x826313B0;
    'dispatch: loop {
        match pc {
            0x826313B0 => {
    //   block [0x826313B0..0x82631420)
	// 826313B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826313B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826313B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826313BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826313C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826313C4: 38AA6F34  addi r5, r10, 0x6f34
	ctx.r[5].s64 = ctx.r[10].s64 + 28468;
	// 826313C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826313CC: 390BE220  addi r8, r11, -0x1de0
	ctx.r[8].s64 = ctx.r[11].s64 + -7648;
	// 826313D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826313D4: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 826313D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826313DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826313E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826313E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826313E8: 386A6F64  addi r3, r10, 0x6f64
	ctx.r[3].s64 = ctx.r[10].s64 + 28516;
	// 826313EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826313F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826313F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826313F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826313FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263140C: 4BE35A15  bl 0x82466e20
	ctx.lr = 0x82631410;
	sub_82466E20(ctx, base);
	// 82631410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263141C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631420 size=96
    let mut pc: u32 = 0x82631420;
    'dispatch: loop {
        match pc {
            0x82631420 => {
    //   block [0x82631420..0x82631480)
	// 82631420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263142C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631434: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 82631438: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8263143C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631440: 386A6F94  addi r3, r10, 0x6f94
	ctx.r[3].s64 = ctx.r[10].s64 + 28564;
	// 82631444: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263144C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82631450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263145C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631460: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82631464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631468: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263146C: 4BE359B5  bl 0x82466e20
	ctx.lr = 0x82631470;
	sub_82466E20(ctx, base);
	// 82631470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263147C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631480 size=112
    let mut pc: u32 = 0x82631480;
    'dispatch: loop {
        match pc {
            0x82631480 => {
    //   block [0x82631480..0x826314F0)
	// 82631480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263148C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82631490: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631494: 38AA87F4  addi r5, r10, -0x780c
	ctx.r[5].s64 = ctx.r[10].s64 + -30732;
	// 82631498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263149C: 390BE268  addi r8, r11, -0x1d98
	ctx.r[8].s64 = ctx.r[11].s64 + -7576;
	// 826314A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826314A4: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 826314A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826314AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826314B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826314B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826314B8: 386A6FC4  addi r3, r10, 0x6fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 28612;
	// 826314BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826314C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826314C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826314C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826314CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826314D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826314D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826314D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826314DC: 4BE35945  bl 0x82466e20
	ctx.lr = 0x826314E0;
	sub_82466E20(ctx, base);
	// 826314E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826314E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826314E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826314EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826314F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826314F0 size=96
    let mut pc: u32 = 0x826314F0;
    'dispatch: loop {
        match pc {
            0x826314F0 => {
    //   block [0x826314F0..0x82631550)
	// 826314F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826314F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826314F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826314FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631504: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82631508: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8263150C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631510: 386A6FF4  addi r3, r10, 0x6ff4
	ctx.r[3].s64 = ctx.r[10].s64 + 28660;
	// 82631514: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263151C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82631520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263152C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631530: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82631534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631538: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263153C: 4BE358E5  bl 0x82466e20
	ctx.lr = 0x82631540;
	sub_82466E20(ctx, base);
	// 82631540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263154C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631550 size=100
    let mut pc: u32 = 0x82631550;
    'dispatch: loop {
        match pc {
            0x82631550 => {
    //   block [0x82631550..0x826315B4)
	// 82631550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263155C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631564: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82631568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263156C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631570: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82631574: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263157C: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82631580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631584: 386A7024  addi r3, r10, 0x7024
	ctx.r[3].s64 = ctx.r[10].s64 + 28708;
	// 82631588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263158C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631590: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82631594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631598: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263159C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826315A0: 4BE35881  bl 0x82466e20
	ctx.lr = 0x826315A4;
	sub_82466E20(ctx, base);
	// 826315A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826315A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826315AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826315B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826315B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826315B8 size=96
    let mut pc: u32 = 0x826315B8;
    'dispatch: loop {
        match pc {
            0x826315B8 => {
    //   block [0x826315B8..0x82631618)
	// 826315B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826315BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826315C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826315C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826315C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826315CC: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826315D0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826315D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826315D8: 386A7054  addi r3, r10, 0x7054
	ctx.r[3].s64 = ctx.r[10].s64 + 28756;
	// 826315DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826315E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826315E4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826315E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826315EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826315F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826315F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826315F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826315FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631600: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82631604: 4BE3581D  bl 0x82466e20
	ctx.lr = 0x82631608;
	sub_82466E20(ctx, base);
	// 82631608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263160C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631618 size=112
    let mut pc: u32 = 0x82631618;
    'dispatch: loop {
        match pc {
            0x82631618 => {
    //   block [0x82631618..0x82631688)
	// 82631618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263161C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631624: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631628: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263162C: 38AA7024  addi r5, r10, 0x7024
	ctx.r[5].s64 = ctx.r[10].s64 + 28708;
	// 82631630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631634: 390BE2C8  addi r8, r11, -0x1d38
	ctx.r[8].s64 = ctx.r[11].s64 + -7480;
	// 82631638: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263163C: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 82631640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631644: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263164C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631650: 386A7084  addi r3, r10, 0x7084
	ctx.r[3].s64 = ctx.r[10].s64 + 28804;
	// 82631654: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263165C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263166C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631674: 4BE357AD  bl 0x82466e20
	ctx.lr = 0x82631678;
	sub_82466E20(ctx, base);
	// 82631678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263167C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631688 size=112
    let mut pc: u32 = 0x82631688;
    'dispatch: loop {
        match pc {
            0x82631688 => {
    //   block [0x82631688..0x826316F8)
	// 82631688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263168C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631694: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631698: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263169C: 38AA7024  addi r5, r10, 0x7024
	ctx.r[5].s64 = ctx.r[10].s64 + 28708;
	// 826316A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826316A4: 390BE2F8  addi r8, r11, -0x1d08
	ctx.r[8].s64 = ctx.r[11].s64 + -7432;
	// 826316A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826316AC: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 826316B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826316B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826316B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826316BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826316C0: 386A70B4  addi r3, r10, 0x70b4
	ctx.r[3].s64 = ctx.r[10].s64 + 28852;
	// 826316C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826316C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826316CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826316D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826316D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826316D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826316DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826316E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826316E4: 4BE3573D  bl 0x82466e20
	ctx.lr = 0x826316E8;
	sub_82466E20(ctx, base);
	// 826316E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826316EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826316F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826316F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826316F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826316F8 size=100
    let mut pc: u32 = 0x826316F8;
    'dispatch: loop {
        match pc {
            0x826316F8 => {
    //   block [0x826316F8..0x8263175C)
	// 826316F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826316FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631704: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263170C: 38AA7024  addi r5, r10, 0x7024
	ctx.r[5].s64 = ctx.r[10].s64 + 28708;
	// 82631710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631718: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 8263171C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263172C: 386A70E4  addi r3, r10, 0x70e4
	ctx.r[3].s64 = ctx.r[10].s64 + 28900;
	// 82631730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631738: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263173C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631740: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82631744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631748: 4BE356D9  bl 0x82466e20
	ctx.lr = 0x8263174C;
	sub_82466E20(ctx, base);
	// 8263174C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631760 size=96
    let mut pc: u32 = 0x82631760;
    'dispatch: loop {
        match pc {
            0x82631760 => {
    //   block [0x82631760..0x826317C0)
	// 82631760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263176C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631774: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82631778: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8263177C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631780: 386A7114  addi r3, r10, 0x7114
	ctx.r[3].s64 = ctx.r[10].s64 + 28948;
	// 82631784: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263178C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82631790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263179C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826317A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826317A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826317A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826317AC: 4BE35675  bl 0x82466e20
	ctx.lr = 0x826317B0;
	sub_82466E20(ctx, base);
	// 826317B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826317B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826317B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826317BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826317C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826317C0 size=112
    let mut pc: u32 = 0x826317C0;
    'dispatch: loop {
        match pc {
            0x826317C0 => {
    //   block [0x826317C0..0x82631830)
	// 826317C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826317C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826317C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826317CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826317D0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826317D4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826317D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826317DC: 390BE310  addi r8, r11, -0x1cf0
	ctx.r[8].s64 = ctx.r[11].s64 + -7408;
	// 826317E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826317E4: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826317E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826317EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826317F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826317F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826317F8: 386A7144  addi r3, r10, 0x7144
	ctx.r[3].s64 = ctx.r[10].s64 + 28996;
	// 826317FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263180C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263181C: 4BE35605  bl 0x82466e20
	ctx.lr = 0x82631820;
	sub_82466E20(ctx, base);
	// 82631820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263182C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631830 size=96
    let mut pc: u32 = 0x82631830;
    'dispatch: loop {
        match pc {
            0x82631830 => {
    //   block [0x82631830..0x82631890)
	// 82631830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263183C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631844: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 82631848: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8263184C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631850: 386A7174  addi r3, r10, 0x7174
	ctx.r[3].s64 = ctx.r[10].s64 + 29044;
	// 82631854: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263185C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82631860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263186C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631870: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82631874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631878: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263187C: 4BE355A5  bl 0x82466e20
	ctx.lr = 0x82631880;
	sub_82466E20(ctx, base);
	// 82631880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263188C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631890 size=112
    let mut pc: u32 = 0x82631890;
    'dispatch: loop {
        match pc {
            0x82631890 => {
    //   block [0x82631890..0x82631900)
	// 82631890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263189C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826318A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826318A4: 38AA7174  addi r5, r10, 0x7174
	ctx.r[5].s64 = ctx.r[10].s64 + 29044;
	// 826318A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826318AC: 390BE328  addi r8, r11, -0x1cd8
	ctx.r[8].s64 = ctx.r[11].s64 + -7384;
	// 826318B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826318B4: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826318B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826318BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826318C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826318C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826318C8: 386A71A4  addi r3, r10, 0x71a4
	ctx.r[3].s64 = ctx.r[10].s64 + 29092;
	// 826318CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826318D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826318D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826318D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826318DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826318E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826318E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826318E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826318EC: 4BE35535  bl 0x82466e20
	ctx.lr = 0x826318F0;
	sub_82466E20(ctx, base);
	// 826318F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826318F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826318F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826318FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631900 size=108
    let mut pc: u32 = 0x82631900;
    'dispatch: loop {
        match pc {
            0x82631900 => {
    //   block [0x82631900..0x8263196C)
	// 82631900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263190C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631914: 38EBE340  addi r7, r11, -0x1cc0
	ctx.r[7].s64 = ctx.r[11].s64 + -7360;
	// 82631918: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263191C: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 82631920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631924: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263192C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631930: 386A71D4  addi r3, r10, 0x71d4
	ctx.r[3].s64 = ctx.r[10].s64 + 29140;
	// 82631934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82631938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263193C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263194C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631958: 4BE354C9  bl 0x82466e20
	ctx.lr = 0x8263195C;
	sub_82466E20(ctx, base);
	// 8263195C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631970 size=112
    let mut pc: u32 = 0x82631970;
    'dispatch: loop {
        match pc {
            0x82631970 => {
    //   block [0x82631970..0x826319E0)
	// 82631970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263197C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631980: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631984: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 82631988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263198C: 390BE3A0  addi r8, r11, -0x1c60
	ctx.r[8].s64 = ctx.r[11].s64 + -7264;
	// 82631990: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82631994: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 82631998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263199C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826319A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826319A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826319A8: 386A7204  addi r3, r10, 0x7204
	ctx.r[3].s64 = ctx.r[10].s64 + 29188;
	// 826319AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826319B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826319B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826319B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826319BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826319C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826319C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826319C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826319CC: 4BE35455  bl 0x82466e20
	ctx.lr = 0x826319D0;
	sub_82466E20(ctx, base);
	// 826319D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826319D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826319D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826319DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826319E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826319E0 size=112
    let mut pc: u32 = 0x826319E0;
    'dispatch: loop {
        match pc {
            0x826319E0 => {
    //   block [0x826319E0..0x82631A50)
	// 826319E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826319E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826319E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826319EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826319F0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826319F4: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 826319F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826319FC: 390BE3B8  addi r8, r11, -0x1c48
	ctx.r[8].s64 = ctx.r[11].s64 + -7240;
	// 82631A00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82631A04: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 82631A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631A0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631A18: 386A7234  addi r3, r10, 0x7234
	ctx.r[3].s64 = ctx.r[10].s64 + 29236;
	// 82631A1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631A3C: 4BE353E5  bl 0x82466e20
	ctx.lr = 0x82631A40;
	sub_82466E20(ctx, base);
	// 82631A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631A50 size=112
    let mut pc: u32 = 0x82631A50;
    'dispatch: loop {
        match pc {
            0x82631A50 => {
    //   block [0x82631A50..0x82631AC0)
	// 82631A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631A5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631A60: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631A64: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 82631A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631A6C: 390BE3E8  addi r8, r11, -0x1c18
	ctx.r[8].s64 = ctx.r[11].s64 + -7192;
	// 82631A70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82631A74: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 82631A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631A7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631A80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631A88: 386A7264  addi r3, r10, 0x7264
	ctx.r[3].s64 = ctx.r[10].s64 + 29284;
	// 82631A8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631AAC: 4BE35375  bl 0x82466e20
	ctx.lr = 0x82631AB0;
	sub_82466E20(ctx, base);
	// 82631AB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631AC0 size=116
    let mut pc: u32 = 0x82631AC0;
    'dispatch: loop {
        match pc {
            0x82631AC0 => {
    //   block [0x82631AC0..0x82631B34)
	// 82631AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631ACC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631AD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82631AD4: 390BE404  addi r8, r11, -0x1bfc
	ctx.r[8].s64 = ctx.r[11].s64 + -7164;
	// 82631AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631ADC: 392A4E08  addi r9, r10, 0x4e08
	ctx.r[9].s64 = ctx.r[10].s64 + 19976;
	// 82631AE0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631AE4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82631AE8: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 82631AEC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631AF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631B04: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82631B08: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 82631B0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82631B10: 386B7294  addi r3, r11, 0x7294
	ctx.r[3].s64 = ctx.r[11].s64 + 29332;
	// 82631B14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82631B18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631B20: 4BE35301  bl 0x82466e20
	ctx.lr = 0x82631B24;
	sub_82466E20(ctx, base);
	// 82631B24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631B38 size=112
    let mut pc: u32 = 0x82631B38;
    'dispatch: loop {
        match pc {
            0x82631B38 => {
    //   block [0x82631B38..0x82631BA8)
	// 82631B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631B44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631B48: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631B4C: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 82631B50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631B54: 390BE434  addi r8, r11, -0x1bcc
	ctx.r[8].s64 = ctx.r[11].s64 + -7116;
	// 82631B58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82631B5C: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 82631B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631B64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631B68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631B70: 386A72C4  addi r3, r10, 0x72c4
	ctx.r[3].s64 = ctx.r[10].s64 + 29380;
	// 82631B74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631B78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631B80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631B84: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82631B88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631B90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631B94: 4BE3528D  bl 0x82466e20
	ctx.lr = 0x82631B98;
	sub_82466E20(ctx, base);
	// 82631B98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631BA8 size=112
    let mut pc: u32 = 0x82631BA8;
    'dispatch: loop {
        match pc {
            0x82631BA8 => {
    //   block [0x82631BA8..0x82631C18)
	// 82631BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631BB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631BB8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631BBC: 38AA77A4  addi r5, r10, 0x77a4
	ctx.r[5].s64 = ctx.r[10].s64 + 30628;
	// 82631BC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631BC4: 390BE44C  addi r8, r11, -0x1bb4
	ctx.r[8].s64 = ctx.r[11].s64 + -7092;
	// 82631BC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82631BCC: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 82631BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631BD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631BD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631BE0: 386A72F4  addi r3, r10, 0x72f4
	ctx.r[3].s64 = ctx.r[10].s64 + 29428;
	// 82631BE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631BE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631C04: 4BE3521D  bl 0x82466e20
	ctx.lr = 0x82631C08;
	sub_82466E20(ctx, base);
	// 82631C08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631C18 size=112
    let mut pc: u32 = 0x82631C18;
    'dispatch: loop {
        match pc {
            0x82631C18 => {
    //   block [0x82631C18..0x82631C88)
	// 82631C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631C24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631C28: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631C2C: 38AA7504  addi r5, r10, 0x7504
	ctx.r[5].s64 = ctx.r[10].s64 + 29956;
	// 82631C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631C34: 390BE464  addi r8, r11, -0x1b9c
	ctx.r[8].s64 = ctx.r[11].s64 + -7068;
	// 82631C38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82631C3C: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 82631C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631C44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631C48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631C50: 386A7324  addi r3, r10, 0x7324
	ctx.r[3].s64 = ctx.r[10].s64 + 29476;
	// 82631C54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631C74: 4BE351AD  bl 0x82466e20
	ctx.lr = 0x82631C78;
	sub_82466E20(ctx, base);
	// 82631C78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631C88 size=112
    let mut pc: u32 = 0x82631C88;
    'dispatch: loop {
        match pc {
            0x82631C88 => {
    //   block [0x82631C88..0x82631CF8)
	// 82631C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631C94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631C98: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631C9C: 38AA72C4  addi r5, r10, 0x72c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29380;
	// 82631CA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631CA4: 390BE480  addi r8, r11, -0x1b80
	ctx.r[8].s64 = ctx.r[11].s64 + -7040;
	// 82631CA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82631CAC: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 82631CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631CB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631CB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631CC0: 386A7354  addi r3, r10, 0x7354
	ctx.r[3].s64 = ctx.r[10].s64 + 29524;
	// 82631CC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631CCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631CD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631CD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631CDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631CE4: 4BE3513D  bl 0x82466e20
	ctx.lr = 0x82631CE8;
	sub_82466E20(ctx, base);
	// 82631CE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631CF8 size=112
    let mut pc: u32 = 0x82631CF8;
    'dispatch: loop {
        match pc {
            0x82631CF8 => {
    //   block [0x82631CF8..0x82631D68)
	// 82631CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631D04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631D08: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631D0C: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 82631D10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631D14: 390BE4C8  addi r8, r11, -0x1b38
	ctx.r[8].s64 = ctx.r[11].s64 + -6968;
	// 82631D18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82631D1C: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 82631D20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631D24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631D28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631D30: 386A7384  addi r3, r10, 0x7384
	ctx.r[3].s64 = ctx.r[10].s64 + 29572;
	// 82631D34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631D38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631D3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631D40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631D48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631D4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631D50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631D54: 4BE350CD  bl 0x82466e20
	ctx.lr = 0x82631D58;
	sub_82466E20(ctx, base);
	// 82631D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631D68 size=112
    let mut pc: u32 = 0x82631D68;
    'dispatch: loop {
        match pc {
            0x82631D68 => {
    //   block [0x82631D68..0x82631DD8)
	// 82631D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631D74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631D78: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631D7C: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 82631D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631D84: 390BE4F8  addi r8, r11, -0x1b08
	ctx.r[8].s64 = ctx.r[11].s64 + -6920;
	// 82631D88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82631D8C: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 82631D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631D94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631D98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631DA0: 386A73B4  addi r3, r10, 0x73b4
	ctx.r[3].s64 = ctx.r[10].s64 + 29620;
	// 82631DA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631DC4: 4BE3505D  bl 0x82466e20
	ctx.lr = 0x82631DC8;
	sub_82466E20(ctx, base);
	// 82631DC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631DD8 size=108
    let mut pc: u32 = 0x82631DD8;
    'dispatch: loop {
        match pc {
            0x82631DD8 => {
    //   block [0x82631DD8..0x82631E44)
	// 82631DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631DE4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631DEC: 38EBE528  addi r7, r11, -0x1ad8
	ctx.r[7].s64 = ctx.r[11].s64 + -6872;
	// 82631DF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82631DF4: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 82631DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631DFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631E00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82631E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631E08: 386A73E4  addi r3, r10, 0x73e4
	ctx.r[3].s64 = ctx.r[10].s64 + 29668;
	// 82631E0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82631E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631E2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631E30: 4BE34FF1  bl 0x82466e20
	ctx.lr = 0x82631E34;
	sub_82466E20(ctx, base);
	// 82631E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631E48 size=112
    let mut pc: u32 = 0x82631E48;
    'dispatch: loop {
        match pc {
            0x82631E48 => {
    //   block [0x82631E48..0x82631EB8)
	// 82631E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631E54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631E58: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631E5C: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 82631E60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631E64: 390BE570  addi r8, r11, -0x1a90
	ctx.r[8].s64 = ctx.r[11].s64 + -6800;
	// 82631E68: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82631E6C: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 82631E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631E74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631E78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631E80: 386A7414  addi r3, r10, 0x7414
	ctx.r[3].s64 = ctx.r[10].s64 + 29716;
	// 82631E84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631E9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631EA4: 4BE34F7D  bl 0x82466e20
	ctx.lr = 0x82631EA8;
	sub_82466E20(ctx, base);
	// 82631EA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631EB8 size=116
    let mut pc: u32 = 0x82631EB8;
    'dispatch: loop {
        match pc {
            0x82631EB8 => {
    //   block [0x82631EB8..0x82631F2C)
	// 82631EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631EC4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631EC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82631ECC: 390BE5E8  addi r8, r11, -0x1a18
	ctx.r[8].s64 = ctx.r[11].s64 + -6680;
	// 82631ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631ED4: 392A4E44  addi r9, r10, 0x4e44
	ctx.r[9].s64 = ctx.r[10].s64 + 20036;
	// 82631ED8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631EDC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82631EE0: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 82631EE4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631EEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631EFC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82631F00: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 82631F04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82631F08: 386B7444  addi r3, r11, 0x7444
	ctx.r[3].s64 = ctx.r[11].s64 + 29764;
	// 82631F0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82631F10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631F18: 4BE34F09  bl 0x82466e20
	ctx.lr = 0x82631F1C;
	sub_82466E20(ctx, base);
	// 82631F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631F30 size=100
    let mut pc: u32 = 0x82631F30;
    'dispatch: loop {
        match pc {
            0x82631F30 => {
    //   block [0x82631F30..0x82631F94)
	// 82631F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631F3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631F44: 38AA7594  addi r5, r10, 0x7594
	ctx.r[5].s64 = ctx.r[10].s64 + 30100;
	// 82631F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631F50: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 82631F54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631F64: 386A7474  addi r3, r10, 0x7474
	ctx.r[3].s64 = ctx.r[10].s64 + 29812;
	// 82631F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631F6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631F70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82631F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631F78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82631F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631F80: 4BE34EA1  bl 0x82466e20
	ctx.lr = 0x82631F84;
	sub_82466E20(ctx, base);
	// 82631F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631F98 size=100
    let mut pc: u32 = 0x82631F98;
    'dispatch: loop {
        match pc {
            0x82631F98 => {
    //   block [0x82631F98..0x82631FFC)
	// 82631F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631FA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631FAC: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 82631FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631FB8: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 82631FBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631FCC: 386A74A4  addi r3, r10, 0x74a4
	ctx.r[3].s64 = ctx.r[10].s64 + 29860;
	// 82631FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631FD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631FD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82631FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631FE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82631FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631FE8: 4BE34E39  bl 0x82466e20
	ctx.lr = 0x82631FEC;
	sub_82466E20(ctx, base);
	// 82631FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632000 size=108
    let mut pc: u32 = 0x82632000;
    'dispatch: loop {
        match pc {
            0x82632000 => {
    //   block [0x82632000..0x8263206C)
	// 82632000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263200C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632014: 38EBE660  addi r7, r11, -0x19a0
	ctx.r[7].s64 = ctx.r[11].s64 + -6560;
	// 82632018: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263201C: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 82632020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632024: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263202C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632030: 386A74D4  addi r3, r10, 0x74d4
	ctx.r[3].s64 = ctx.r[10].s64 + 29908;
	// 82632034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263203C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263204C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632058: 4BE34DC9  bl 0x82466e20
	ctx.lr = 0x8263205C;
	sub_82466E20(ctx, base);
	// 8263205C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632070 size=112
    let mut pc: u32 = 0x82632070;
    'dispatch: loop {
        match pc {
            0x82632070 => {
    //   block [0x82632070..0x826320E0)
	// 82632070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263207C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632080: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632084: 38AA72C4  addi r5, r10, 0x72c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29380;
	// 82632088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263208C: 390BE690  addi r8, r11, -0x1970
	ctx.r[8].s64 = ctx.r[11].s64 + -6512;
	// 82632090: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82632094: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 82632098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263209C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826320A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826320A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826320A8: 386A7504  addi r3, r10, 0x7504
	ctx.r[3].s64 = ctx.r[10].s64 + 29956;
	// 826320AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826320B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826320B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826320B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826320BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826320C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826320C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826320C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826320CC: 4BE34D55  bl 0x82466e20
	ctx.lr = 0x826320D0;
	sub_82466E20(ctx, base);
	// 826320D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826320D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826320D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826320DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826320E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826320E0 size=108
    let mut pc: u32 = 0x826320E0;
    'dispatch: loop {
        match pc {
            0x826320E0 => {
    //   block [0x826320E0..0x8263214C)
	// 826320E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826320E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826320E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826320EC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826320F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826320F4: 38EBE6A8  addi r7, r11, -0x1958
	ctx.r[7].s64 = ctx.r[11].s64 + -6488;
	// 826320F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826320FC: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 82632100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632104: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632108: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263210C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632110: 386A7534  addi r3, r10, 0x7534
	ctx.r[3].s64 = ctx.r[10].s64 + 30004;
	// 82632114: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263211C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263212C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632134: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632138: 4BE34CE9  bl 0x82466e20
	ctx.lr = 0x8263213C;
	sub_82466E20(ctx, base);
	// 8263213C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82632150 size=28
    let mut pc: u32 = 0x82632150;
    'dispatch: loop {
        match pc {
            0x82632150 => {
    //   block [0x82632150..0x8263216C)
	// 82632150: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632154: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82632158: 394A2278  addi r10, r10, 0x2278
	ctx.r[10].s64 = ctx.r[10].s64 + 8824;
	// 8263215C: 816BE6C0  lwz r11, -0x1940(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6464 as u32) ) } as u64;
	// 82632160: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82632164: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82632168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632170 size=108
    let mut pc: u32 = 0x82632170;
    'dispatch: loop {
        match pc {
            0x82632170 => {
    //   block [0x82632170..0x826321DC)
	// 82632170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263217C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632184: 38EB2278  addi r7, r11, 0x2278
	ctx.r[7].s64 = ctx.r[11].s64 + 8824;
	// 82632188: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8263218C: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 82632190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632194: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632198: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263219C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826321A0: 386A7564  addi r3, r10, 0x7564
	ctx.r[3].s64 = ctx.r[10].s64 + 30052;
	// 826321A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826321A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826321AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826321B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826321B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826321B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826321BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826321C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826321C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826321C8: 4BE34C59  bl 0x82466e20
	ctx.lr = 0x826321CC;
	sub_82466E20(ctx, base);
	// 826321CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826321D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826321D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826321D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826321E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826321E0 size=116
    let mut pc: u32 = 0x826321E0;
    'dispatch: loop {
        match pc {
            0x826321E0 => {
    //   block [0x826321E0..0x82632254)
	// 826321E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826321E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826321E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826321EC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826321F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826321F4: 390BE6C8  addi r8, r11, -0x1938
	ctx.r[8].s64 = ctx.r[11].s64 + -6456;
	// 826321F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826321FC: 392A4E98  addi r9, r10, 0x4e98
	ctx.r[9].s64 = ctx.r[10].s64 + 20120;
	// 82632200: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632204: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82632208: 38AA72C4  addi r5, r10, 0x72c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29380;
	// 8263220C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632214: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263221C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632224: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82632228: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 8263222C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82632230: 386B7594  addi r3, r11, 0x7594
	ctx.r[3].s64 = ctx.r[11].s64 + 30100;
	// 82632234: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82632238: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263223C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632240: 4BE34BE1  bl 0x82466e20
	ctx.lr = 0x82632244;
	sub_82466E20(ctx, base);
	// 82632244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263224C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632258 size=112
    let mut pc: u32 = 0x82632258;
    'dispatch: loop {
        match pc {
            0x82632258 => {
    //   block [0x82632258..0x826322C8)
	// 82632258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263225C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632264: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632268: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263226C: 38AA7264  addi r5, r10, 0x7264
	ctx.r[5].s64 = ctx.r[10].s64 + 29284;
	// 82632270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632274: 390BE740  addi r8, r11, -0x18c0
	ctx.r[8].s64 = ctx.r[11].s64 + -6336;
	// 82632278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263227C: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 82632280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632284: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263228C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632290: 386A75C4  addi r3, r10, 0x75c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30148;
	// 82632294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263229C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826322A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826322A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826322A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826322AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826322B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826322B4: 4BE34B6D  bl 0x82466e20
	ctx.lr = 0x826322B8;
	sub_82466E20(ctx, base);
	// 826322B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826322BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826322C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826322C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826322C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826322C8 size=108
    let mut pc: u32 = 0x826322C8;
    'dispatch: loop {
        match pc {
            0x826322C8 => {
    //   block [0x826322C8..0x82632334)
	// 826322C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826322CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826322D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826322D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826322D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826322DC: 38EBE758  addi r7, r11, -0x18a8
	ctx.r[7].s64 = ctx.r[11].s64 + -6312;
	// 826322E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826322E4: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826322E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826322EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826322F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826322F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826322F8: 386A75F4  addi r3, r10, 0x75f4
	ctx.r[3].s64 = ctx.r[10].s64 + 30196;
	// 826322FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263230C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263231C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632320: 4BE34B01  bl 0x82466e20
	ctx.lr = 0x82632324;
	sub_82466E20(ctx, base);
	// 82632324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263232C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632338 size=112
    let mut pc: u32 = 0x82632338;
    'dispatch: loop {
        match pc {
            0x82632338 => {
    //   block [0x82632338..0x826323A8)
	// 82632338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263233C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632344: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632348: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263234C: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 82632350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632354: 390BE788  addi r8, r11, -0x1878
	ctx.r[8].s64 = ctx.r[11].s64 + -6264;
	// 82632358: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263235C: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 82632360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632364: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263236C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632370: 386A7624  addi r3, r10, 0x7624
	ctx.r[3].s64 = ctx.r[10].s64 + 30244;
	// 82632374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263237C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263238C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632394: 4BE34A8D  bl 0x82466e20
	ctx.lr = 0x82632398;
	sub_82466E20(ctx, base);
	// 82632398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263239C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826323A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826323A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826323A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826323A8 size=112
    let mut pc: u32 = 0x826323A8;
    'dispatch: loop {
        match pc {
            0x826323A8 => {
    //   block [0x826323A8..0x82632418)
	// 826323A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826323AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826323B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826323B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826323B8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826323BC: 38AA77A4  addi r5, r10, 0x77a4
	ctx.r[5].s64 = ctx.r[10].s64 + 30628;
	// 826323C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826323C4: 390BE7B8  addi r8, r11, -0x1848
	ctx.r[8].s64 = ctx.r[11].s64 + -6216;
	// 826323C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826323CC: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 826323D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826323D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826323D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826323DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826323E0: 386A7654  addi r3, r10, 0x7654
	ctx.r[3].s64 = ctx.r[10].s64 + 30292;
	// 826323E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826323E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826323EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826323F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826323F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826323F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826323FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632404: 4BE34A1D  bl 0x82466e20
	ctx.lr = 0x82632408;
	sub_82466E20(ctx, base);
	// 82632408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263240C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632418 size=100
    let mut pc: u32 = 0x82632418;
    'dispatch: loop {
        match pc {
            0x82632418 => {
    //   block [0x82632418..0x8263247C)
	// 82632418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263241C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263242C: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 82632430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632438: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 8263243C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263244C: 386A7684  addi r3, r10, 0x7684
	ctx.r[3].s64 = ctx.r[10].s64 + 30340;
	// 82632450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632454: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632458: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263245C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632460: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82632464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632468: 4BE349B9  bl 0x82466e20
	ctx.lr = 0x8263246C;
	sub_82466E20(ctx, base);
	// 8263246C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632480 size=112
    let mut pc: u32 = 0x82632480;
    'dispatch: loop {
        match pc {
            0x82632480 => {
    //   block [0x82632480..0x826324F0)
	// 82632480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263248C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632490: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632494: 38AA74A4  addi r5, r10, 0x74a4
	ctx.r[5].s64 = ctx.r[10].s64 + 29860;
	// 82632498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263249C: 390BE7E8  addi r8, r11, -0x1818
	ctx.r[8].s64 = ctx.r[11].s64 + -6168;
	// 826324A0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826324A4: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 826324A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826324AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826324B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826324B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826324B8: 386A76B4  addi r3, r10, 0x76b4
	ctx.r[3].s64 = ctx.r[10].s64 + 30388;
	// 826324BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826324C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826324C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826324C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826324CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826324D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826324D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826324D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826324DC: 4BE34945  bl 0x82466e20
	ctx.lr = 0x826324E0;
	sub_82466E20(ctx, base);
	// 826324E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826324E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826324E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826324EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826324F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826324F0 size=112
    let mut pc: u32 = 0x826324F0;
    'dispatch: loop {
        match pc {
            0x826324F0 => {
    //   block [0x826324F0..0x82632560)
	// 826324F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826324F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826324F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826324FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632500: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632504: 38AA74A4  addi r5, r10, 0x74a4
	ctx.r[5].s64 = ctx.r[10].s64 + 29860;
	// 82632508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263250C: 390BE830  addi r8, r11, -0x17d0
	ctx.r[8].s64 = ctx.r[11].s64 + -6096;
	// 82632510: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82632514: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 82632518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263251C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632528: 386A76E4  addi r3, r10, 0x76e4
	ctx.r[3].s64 = ctx.r[10].s64 + 30436;
	// 8263252C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263253C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263254C: 4BE348D5  bl 0x82466e20
	ctx.lr = 0x82632550;
	sub_82466E20(ctx, base);
	// 82632550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263255C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632560 size=108
    let mut pc: u32 = 0x82632560;
    'dispatch: loop {
        match pc {
            0x82632560 => {
    //   block [0x82632560..0x826325CC)
	// 82632560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263256C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632574: 38EBE8D8  addi r7, r11, -0x1728
	ctx.r[7].s64 = ctx.r[11].s64 + -5928;
	// 82632578: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263257C: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 82632580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632584: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263258C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632590: 386A7714  addi r3, r10, 0x7714
	ctx.r[3].s64 = ctx.r[10].s64 + 30484;
	// 82632594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263259C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826325A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826325A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826325A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826325AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826325B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826325B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826325B8: 4BE34869  bl 0x82466e20
	ctx.lr = 0x826325BC;
	sub_82466E20(ctx, base);
	// 826325BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826325C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826325C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826325C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826325D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826325D0 size=112
    let mut pc: u32 = 0x826325D0;
    'dispatch: loop {
        match pc {
            0x826325D0 => {
    //   block [0x826325D0..0x82632640)
	// 826325D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826325D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826325D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826325DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826325E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826325E4: 38AA72C4  addi r5, r10, 0x72c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29380;
	// 826325E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826325EC: 390BE920  addi r8, r11, -0x16e0
	ctx.r[8].s64 = ctx.r[11].s64 + -5856;
	// 826325F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826325F4: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 826325F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826325FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632608: 386A7744  addi r3, r10, 0x7744
	ctx.r[3].s64 = ctx.r[10].s64 + 30532;
	// 8263260C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263261C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263262C: 4BE347F5  bl 0x82466e20
	ctx.lr = 0x82632630;
	sub_82466E20(ctx, base);
	// 82632630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263263C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632640 size=100
    let mut pc: u32 = 0x82632640;
    'dispatch: loop {
        match pc {
            0x82632640 => {
    //   block [0x82632640..0x826326A4)
	// 82632640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263264C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632654: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 82632658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263265C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632660: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 82632664: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263266C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632674: 386A7774  addi r3, r10, 0x7774
	ctx.r[3].s64 = ctx.r[10].s64 + 30580;
	// 82632678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263267C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632680: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82632684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632688: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263268C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632690: 4BE34791  bl 0x82466e20
	ctx.lr = 0x82632694;
	sub_82466E20(ctx, base);
	// 82632694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263269C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826326A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826326A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826326A8 size=100
    let mut pc: u32 = 0x826326A8;
    'dispatch: loop {
        match pc {
            0x826326A8 => {
    //   block [0x826326A8..0x8263270C)
	// 826326A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826326AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826326B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826326B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826326B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826326BC: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 826326C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826326C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826326C8: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826326CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826326D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826326D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826326D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826326DC: 386A77A4  addi r3, r10, 0x77a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30628;
	// 826326E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826326E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826326E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826326EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826326F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826326F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826326F8: 4BE34729  bl 0x82466e20
	ctx.lr = 0x826326FC;
	sub_82466E20(ctx, base);
	// 826326FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632710 size=112
    let mut pc: u32 = 0x82632710;
    'dispatch: loop {
        match pc {
            0x82632710 => {
    //   block [0x82632710..0x82632780)
	// 82632710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263271C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632720: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632724: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82632728: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263272C: 390BE980  addi r8, r11, -0x1680
	ctx.r[8].s64 = ctx.r[11].s64 + -5760;
	// 82632730: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82632734: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 82632738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263273C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632740: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632748: 386A77D4  addi r3, r10, 0x77d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30676;
	// 8263274C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263275C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263276C: 4BE346B5  bl 0x82466e20
	ctx.lr = 0x82632770;
	sub_82466E20(ctx, base);
	// 82632770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263277C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632780 size=112
    let mut pc: u32 = 0x82632780;
    'dispatch: loop {
        match pc {
            0x82632780 => {
    //   block [0x82632780..0x826327F0)
	// 82632780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263278C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632790: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632794: 38AA7594  addi r5, r10, 0x7594
	ctx.r[5].s64 = ctx.r[10].s64 + 30100;
	// 82632798: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263279C: 390BEA10  addi r8, r11, -0x15f0
	ctx.r[8].s64 = ctx.r[11].s64 + -5616;
	// 826327A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826327A4: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826327A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826327AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826327B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826327B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826327B8: 386A7804  addi r3, r10, 0x7804
	ctx.r[3].s64 = ctx.r[10].s64 + 30724;
	// 826327BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826327C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826327C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826327C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826327CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826327D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826327D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826327D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826327DC: 4BE34645  bl 0x82466e20
	ctx.lr = 0x826327E0;
	sub_82466E20(ctx, base);
	// 826327E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826327E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826327E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826327EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826327F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826327F0 size=112
    let mut pc: u32 = 0x826327F0;
    'dispatch: loop {
        match pc {
            0x826327F0 => {
    //   block [0x826327F0..0x82632860)
	// 826327F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826327F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826327F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826327FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632800: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632804: 38AA76E4  addi r5, r10, 0x76e4
	ctx.r[5].s64 = ctx.r[10].s64 + 30436;
	// 82632808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263280C: 390BEA28  addi r8, r11, -0x15d8
	ctx.r[8].s64 = ctx.r[11].s64 + -5592;
	// 82632810: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82632814: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 82632818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263281C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632828: 386A7834  addi r3, r10, 0x7834
	ctx.r[3].s64 = ctx.r[10].s64 + 30772;
	// 8263282C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263283C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263284C: 4BE345D5  bl 0x82466e20
	ctx.lr = 0x82632850;
	sub_82466E20(ctx, base);
	// 82632850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263285C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632860 size=112
    let mut pc: u32 = 0x82632860;
    'dispatch: loop {
        match pc {
            0x82632860 => {
    //   block [0x82632860..0x826328D0)
	// 82632860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263286C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632870: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632874: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 82632878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263287C: 390BEA58  addi r8, r11, -0x15a8
	ctx.r[8].s64 = ctx.r[11].s64 + -5544;
	// 82632880: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82632884: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 82632888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263288C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632898: 386A7864  addi r3, r10, 0x7864
	ctx.r[3].s64 = ctx.r[10].s64 + 30820;
	// 8263289C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826328A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826328A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826328A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826328AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826328B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826328B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826328B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826328BC: 4BE34565  bl 0x82466e20
	ctx.lr = 0x826328C0;
	sub_82466E20(ctx, base);
	// 826328C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826328C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826328C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826328CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826328D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826328D0 size=112
    let mut pc: u32 = 0x826328D0;
    'dispatch: loop {
        match pc {
            0x826328D0 => {
    //   block [0x826328D0..0x82632940)
	// 826328D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826328D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826328D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826328DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826328E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826328E4: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 826328E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826328EC: 390BEAA0  addi r8, r11, -0x1560
	ctx.r[8].s64 = ctx.r[11].s64 + -5472;
	// 826328F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826328F4: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 826328F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826328FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632900: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632908: 386A7894  addi r3, r10, 0x7894
	ctx.r[3].s64 = ctx.r[10].s64 + 30868;
	// 8263290C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263291C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263292C: 4BE344F5  bl 0x82466e20
	ctx.lr = 0x82632930;
	sub_82466E20(ctx, base);
	// 82632930: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263293C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632940 size=112
    let mut pc: u32 = 0x82632940;
    'dispatch: loop {
        match pc {
            0x82632940 => {
    //   block [0x82632940..0x826329B0)
	// 82632940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263294C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632950: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632954: 38AA7264  addi r5, r10, 0x7264
	ctx.r[5].s64 = ctx.r[10].s64 + 29284;
	// 82632958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263295C: 390BEAE8  addi r8, r11, -0x1518
	ctx.r[8].s64 = ctx.r[11].s64 + -5400;
	// 82632960: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82632964: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 82632968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263296C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632978: 386A78C4  addi r3, r10, 0x78c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30916;
	// 8263297C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263298C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263299C: 4BE34485  bl 0x82466e20
	ctx.lr = 0x826329A0;
	sub_82466E20(ctx, base);
	// 826329A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826329A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826329A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826329AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826329B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826329B0 size=112
    let mut pc: u32 = 0x826329B0;
    'dispatch: loop {
        match pc {
            0x826329B0 => {
    //   block [0x826329B0..0x82632A20)
	// 826329B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826329B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826329B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826329BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826329C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826329C4: 38AA72C4  addi r5, r10, 0x72c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29380;
	// 826329C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826329CC: 390BEB00  addi r8, r11, -0x1500
	ctx.r[8].s64 = ctx.r[11].s64 + -5376;
	// 826329D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826329D4: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826329D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826329DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826329E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826329E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826329E8: 386A78F4  addi r3, r10, 0x78f4
	ctx.r[3].s64 = ctx.r[10].s64 + 30964;
	// 826329EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826329F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826329F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826329F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826329FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632A0C: 4BE34415  bl 0x82466e20
	ctx.lr = 0x82632A10;
	sub_82466E20(ctx, base);
	// 82632A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632A20 size=112
    let mut pc: u32 = 0x82632A20;
    'dispatch: loop {
        match pc {
            0x82632A20 => {
    //   block [0x82632A20..0x82632A90)
	// 82632A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632A2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632A30: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632A34: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82632A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632A3C: 390BEB30  addi r8, r11, -0x14d0
	ctx.r[8].s64 = ctx.r[11].s64 + -5328;
	// 82632A40: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82632A44: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 82632A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632A4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632A50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632A58: 386A7924  addi r3, r10, 0x7924
	ctx.r[3].s64 = ctx.r[10].s64 + 31012;
	// 82632A5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632A7C: 4BE343A5  bl 0x82466e20
	ctx.lr = 0x82632A80;
	sub_82466E20(ctx, base);
	// 82632A80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632A90 size=112
    let mut pc: u32 = 0x82632A90;
    'dispatch: loop {
        match pc {
            0x82632A90 => {
    //   block [0x82632A90..0x82632B00)
	// 82632A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632A9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632AA0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632AA4: 38AA7924  addi r5, r10, 0x7924
	ctx.r[5].s64 = ctx.r[10].s64 + 31012;
	// 82632AA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632AAC: 390BEB90  addi r8, r11, -0x1470
	ctx.r[8].s64 = ctx.r[11].s64 + -5232;
	// 82632AB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82632AB4: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 82632AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632ABC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632AC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632AC8: 386A7954  addi r3, r10, 0x7954
	ctx.r[3].s64 = ctx.r[10].s64 + 31060;
	// 82632ACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632AD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632AD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632AE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632AE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632AE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632AEC: 4BE34335  bl 0x82466e20
	ctx.lr = 0x82632AF0;
	sub_82466E20(ctx, base);
	// 82632AF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632B00 size=112
    let mut pc: u32 = 0x82632B00;
    'dispatch: loop {
        match pc {
            0x82632B00 => {
    //   block [0x82632B00..0x82632B70)
	// 82632B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632B0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632B10: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632B14: 38AA7924  addi r5, r10, 0x7924
	ctx.r[5].s64 = ctx.r[10].s64 + 31012;
	// 82632B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632B1C: 390BEBA8  addi r8, r11, -0x1458
	ctx.r[8].s64 = ctx.r[11].s64 + -5208;
	// 82632B20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82632B24: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 82632B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632B2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632B30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632B38: 386A7984  addi r3, r10, 0x7984
	ctx.r[3].s64 = ctx.r[10].s64 + 31108;
	// 82632B3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632B5C: 4BE342C5  bl 0x82466e20
	ctx.lr = 0x82632B60;
	sub_82466E20(ctx, base);
	// 82632B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632B70 size=112
    let mut pc: u32 = 0x82632B70;
    'dispatch: loop {
        match pc {
            0x82632B70 => {
    //   block [0x82632B70..0x82632BE0)
	// 82632B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632B7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632B80: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632B84: 38AA7924  addi r5, r10, 0x7924
	ctx.r[5].s64 = ctx.r[10].s64 + 31012;
	// 82632B88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632B8C: 390BEBD8  addi r8, r11, -0x1428
	ctx.r[8].s64 = ctx.r[11].s64 + -5160;
	// 82632B90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82632B94: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 82632B98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632B9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632BA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632BA8: 386A79B4  addi r3, r10, 0x79b4
	ctx.r[3].s64 = ctx.r[10].s64 + 31156;
	// 82632BAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632BB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632BC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632BC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632BCC: 4BE34255  bl 0x82466e20
	ctx.lr = 0x82632BD0;
	sub_82466E20(ctx, base);
	// 82632BD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82632BE0 size=24
    let mut pc: u32 = 0x82632BE0;
    'dispatch: loop {
        match pc {
            0x82632BE0 => {
    //   block [0x82632BE0..0x82632BF8)
	// 82632BE0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632BE4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82632BE8: 394A23B0  addi r10, r10, 0x23b0
	ctx.r[10].s64 = ctx.r[10].s64 + 9136;
	// 82632BEC: 816BEBF0  lwz r11, -0x1410(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5136 as u32) ) } as u64;
	// 82632BF0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82632BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632BF8 size=112
    let mut pc: u32 = 0x82632BF8;
    'dispatch: loop {
        match pc {
            0x82632BF8 => {
    //   block [0x82632BF8..0x82632C68)
	// 82632BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632C04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82632C08: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632C0C: 392A4EE8  addi r9, r10, 0x4ee8
	ctx.r[9].s64 = ctx.r[10].s64 + 20200;
	// 82632C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632C14: 390B23B0  addi r8, r11, 0x23b0
	ctx.r[8].s64 = ctx.r[11].s64 + 9136;
	// 82632C18: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82632C1C: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 82632C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632C24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632C30: 386A79E4  addi r3, r10, 0x79e4
	ctx.r[3].s64 = ctx.r[10].s64 + 31204;
	// 82632C34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82632C38: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82632C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632C4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632C54: 4BE341CD  bl 0x82466e20
	ctx.lr = 0x82632C58;
	sub_82466E20(ctx, base);
	// 82632C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632C68 size=108
    let mut pc: u32 = 0x82632C68;
    'dispatch: loop {
        match pc {
            0x82632C68 => {
    //   block [0x82632C68..0x82632CD4)
	// 82632C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632C74: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632C78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632C7C: 38EBEBF4  addi r7, r11, -0x140c
	ctx.r[7].s64 = ctx.r[11].s64 + -5132;
	// 82632C80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82632C84: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 82632C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632C8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632C90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82632C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632C98: 386A7A14  addi r3, r10, 0x7a14
	ctx.r[3].s64 = ctx.r[10].s64 + 31252;
	// 82632C9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632CA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632CA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632CA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632CB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632CBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632CC0: 4BE34161  bl 0x82466e20
	ctx.lr = 0x82632CC4;
	sub_82466E20(ctx, base);
	// 82632CC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632CD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632CD8 size=108
    let mut pc: u32 = 0x82632CD8;
    'dispatch: loop {
        match pc {
            0x82632CD8 => {
    //   block [0x82632CD8..0x82632D44)
	// 82632CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632CE4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632CE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632CEC: 38EBEC10  addi r7, r11, -0x13f0
	ctx.r[7].s64 = ctx.r[11].s64 + -5104;
	// 82632CF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82632CF4: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 82632CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632CFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632D00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82632D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632D08: 386A7A44  addi r3, r10, 0x7a44
	ctx.r[3].s64 = ctx.r[10].s64 + 31300;
	// 82632D0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632D2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632D30: 4BE340F1  bl 0x82466e20
	ctx.lr = 0x82632D34;
	sub_82466E20(ctx, base);
	// 82632D34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632D48 size=116
    let mut pc: u32 = 0x82632D48;
    'dispatch: loop {
        match pc {
            0x82632D48 => {
    //   block [0x82632D48..0x82632DBC)
	// 82632D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632D54: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632D58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82632D5C: 390BEC58  addi r8, r11, -0x13a8
	ctx.r[8].s64 = ctx.r[11].s64 + -5032;
	// 82632D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632D64: 392A4FB8  addi r9, r10, 0x4fb8
	ctx.r[9].s64 = ctx.r[10].s64 + 20408;
	// 82632D68: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632D6C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82632D70: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82632D74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632D7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632D8C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82632D90: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 82632D94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82632D98: 386B7A74  addi r3, r11, 0x7a74
	ctx.r[3].s64 = ctx.r[11].s64 + 31348;
	// 82632D9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82632DA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632DA8: 4BE34079  bl 0x82466e20
	ctx.lr = 0x82632DAC;
	sub_82466E20(ctx, base);
	// 82632DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632DC0 size=108
    let mut pc: u32 = 0x82632DC0;
    'dispatch: loop {
        match pc {
            0x82632DC0 => {
    //   block [0x82632DC0..0x82632E2C)
	// 82632DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632DCC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632DD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82632DD4: 38EBEC70  addi r7, r11, -0x1390
	ctx.r[7].s64 = ctx.r[11].s64 + -5008;
	// 82632DD8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82632DDC: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 82632DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632DE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82632DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632DF0: 386A7AA4  addi r3, r10, 0x7aa4
	ctx.r[3].s64 = ctx.r[10].s64 + 31396;
	// 82632DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632E18: 4BE34009  bl 0x82466e20
	ctx.lr = 0x82632E1C;
	sub_82466E20(ctx, base);
	// 82632E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82632E30 size=24
    let mut pc: u32 = 0x82632E30;
    'dispatch: loop {
        match pc {
            0x82632E30 => {
    //   block [0x82632E30..0x82632E48)
	// 82632E30: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632E34: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82632E38: 394A23F8  addi r10, r10, 0x23f8
	ctx.r[10].s64 = ctx.r[10].s64 + 9208;
	// 82632E3C: 816BECD0  lwz r11, -0x1330(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4912 as u32) ) } as u64;
	// 82632E40: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82632E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632E48 size=116
    let mut pc: u32 = 0x82632E48;
    'dispatch: loop {
        match pc {
            0x82632E48 => {
    //   block [0x82632E48..0x82632EBC)
	// 82632E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632E54: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632E58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82632E5C: 390B23F8  addi r8, r11, 0x23f8
	ctx.r[8].s64 = ctx.r[11].s64 + 9208;
	// 82632E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632E64: 392A5014  addi r9, r10, 0x5014
	ctx.r[9].s64 = ctx.r[10].s64 + 20500;
	// 82632E68: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632E6C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82632E70: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82632E74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632E7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632E8C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82632E90: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 82632E94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82632E98: 386B7AD4  addi r3, r11, 0x7ad4
	ctx.r[3].s64 = ctx.r[11].s64 + 31444;
	// 82632E9C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82632EA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632EA8: 4BE33F79  bl 0x82466e20
	ctx.lr = 0x82632EAC;
	sub_82466E20(ctx, base);
	// 82632EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632EC0 size=112
    let mut pc: u32 = 0x82632EC0;
    'dispatch: loop {
        match pc {
            0x82632EC0 => {
    //   block [0x82632EC0..0x82632F30)
	// 82632EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632ECC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632ED0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632ED4: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82632ED8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82632EDC: 390BECDC  addi r8, r11, -0x1324
	ctx.r[8].s64 = ctx.r[11].s64 + -4900;
	// 82632EE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82632EE4: 388A2200  addi r4, r10, 0x2200
	ctx.r[4].s64 = ctx.r[10].s64 + 8704;
	// 82632EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632EEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632EF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632EF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632EF8: 386A7B04  addi r3, r10, 0x7b04
	ctx.r[3].s64 = ctx.r[10].s64 + 31492;
	// 82632EFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632F1C: 4BE33F05  bl 0x82466e20
	ctx.lr = 0x82632F20;
	sub_82466E20(ctx, base);
	// 82632F20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632F30 size=112
    let mut pc: u32 = 0x82632F30;
    'dispatch: loop {
        match pc {
            0x82632F30 => {
    //   block [0x82632F30..0x82632FA0)
	// 82632F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632F3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632F40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632F44: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82632F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632F4C: 390BECF4  addi r8, r11, -0x130c
	ctx.r[8].s64 = ctx.r[11].s64 + -4876;
	// 82632F50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82632F54: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 82632F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632F5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632F68: 386A7B34  addi r3, r10, 0x7b34
	ctx.r[3].s64 = ctx.r[10].s64 + 31540;
	// 82632F6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632F8C: 4BE33E95  bl 0x82466e20
	ctx.lr = 0x82632F90;
	sub_82466E20(ctx, base);
	// 82632F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632FA0 size=108
    let mut pc: u32 = 0x82632FA0;
    'dispatch: loop {
        match pc {
            0x82632FA0 => {
    //   block [0x82632FA0..0x8263300C)
	// 82632FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632FAC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632FB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82632FB4: 38EBED28  addi r7, r11, -0x12d8
	ctx.r[7].s64 = ctx.r[11].s64 + -4824;
	// 82632FB8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82632FBC: 388A2224  addi r4, r10, 0x2224
	ctx.r[4].s64 = ctx.r[10].s64 + 8740;
	// 82632FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632FC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632FC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82632FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632FD0: 386A7B64  addi r3, r10, 0x7b64
	ctx.r[3].s64 = ctx.r[10].s64 + 31588;
	// 82632FD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632FF8: 4BE33E29  bl 0x82466e20
	ctx.lr = 0x82632FFC;
	sub_82466E20(ctx, base);
	// 82632FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633010 size=108
    let mut pc: u32 = 0x82633010;
    'dispatch: loop {
        match pc {
            0x82633010 => {
    //   block [0x82633010..0x8263307C)
	// 82633010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263301C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633020: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633024: 38EBED70  addi r7, r11, -0x1290
	ctx.r[7].s64 = ctx.r[11].s64 + -4752;
	// 82633028: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263302C: 388A224C  addi r4, r10, 0x224c
	ctx.r[4].s64 = ctx.r[10].s64 + 8780;
	// 82633030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633034: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263303C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633040: 386A7B94  addi r3, r10, 0x7b94
	ctx.r[3].s64 = ctx.r[10].s64 + 31636;
	// 82633044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263304C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263305C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633068: 4BE33DB9  bl 0x82466e20
	ctx.lr = 0x8263306C;
	sub_82466E20(ctx, base);
	// 8263306C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633080 size=112
    let mut pc: u32 = 0x82633080;
    'dispatch: loop {
        match pc {
            0x82633080 => {
    //   block [0x82633080..0x826330F0)
	// 82633080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263308C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633090: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633094: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633098: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263309C: 390BEDA0  addi r8, r11, -0x1260
	ctx.r[8].s64 = ctx.r[11].s64 + -4704;
	// 826330A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826330A4: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826330A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826330AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826330B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826330B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826330B8: 386A7BC4  addi r3, r10, 0x7bc4
	ctx.r[3].s64 = ctx.r[10].s64 + 31684;
	// 826330BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826330C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826330C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826330C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826330CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826330D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826330D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826330D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826330DC: 4BE33D45  bl 0x82466e20
	ctx.lr = 0x826330E0;
	sub_82466E20(ctx, base);
	// 826330E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826330E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826330E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826330EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826330F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826330F0 size=108
    let mut pc: u32 = 0x826330F0;
    'dispatch: loop {
        match pc {
            0x826330F0 => {
    //   block [0x826330F0..0x8263315C)
	// 826330F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826330F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826330F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826330FC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633100: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633104: 38EBEDD0  addi r7, r11, -0x1230
	ctx.r[7].s64 = ctx.r[11].s64 + -4656;
	// 82633108: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263310C: 388A2274  addi r4, r10, 0x2274
	ctx.r[4].s64 = ctx.r[10].s64 + 8820;
	// 82633110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633114: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633118: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263311C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633120: 386A7BF4  addi r3, r10, 0x7bf4
	ctx.r[3].s64 = ctx.r[10].s64 + 31732;
	// 82633124: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263312C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263313C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633144: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633148: 4BE33CD9  bl 0x82466e20
	ctx.lr = 0x8263314C;
	sub_82466E20(ctx, base);
	// 8263314C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633160 size=108
    let mut pc: u32 = 0x82633160;
    'dispatch: loop {
        match pc {
            0x82633160 => {
    //   block [0x82633160..0x826331CC)
	// 82633160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263316C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633170: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633174: 38EBEE30  addi r7, r11, -0x11d0
	ctx.r[7].s64 = ctx.r[11].s64 + -4560;
	// 82633178: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263317C: 388A22A4  addi r4, r10, 0x22a4
	ctx.r[4].s64 = ctx.r[10].s64 + 8868;
	// 82633180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633184: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263318C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633190: 386A7C24  addi r3, r10, 0x7c24
	ctx.r[3].s64 = ctx.r[10].s64 + 31780;
	// 82633194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263319C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826331A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826331A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826331A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826331AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826331B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826331B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826331B8: 4BE33C69  bl 0x82466e20
	ctx.lr = 0x826331BC;
	sub_82466E20(ctx, base);
	// 826331BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826331C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826331C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826331C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826331D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826331D0 size=116
    let mut pc: u32 = 0x826331D0;
    'dispatch: loop {
        match pc {
            0x826331D0 => {
    //   block [0x826331D0..0x82633244)
	// 826331D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826331D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826331D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826331DC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 826331E0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826331E4: 390AEE78  addi r8, r10, -0x1188
	ctx.r[8].s64 = ctx.r[10].s64 + -4488;
	// 826331E8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826331EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826331F0: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 826331F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826331F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826331FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633204: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 82633208: 396B5050  addi r11, r11, 0x5050
	ctx.r[11].s64 = ctx.r[11].s64 + 20560;
	// 8263320C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633210: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633214: 386A7C54  addi r3, r10, 0x7c54
	ctx.r[3].s64 = ctx.r[10].s64 + 31828;
	// 82633218: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263321C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633220: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82633224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263322C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633230: 4BE33BF1  bl 0x82466e20
	ctx.lr = 0x82633234;
	sub_82466E20(ctx, base);
	// 82633234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263323C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633248 size=112
    let mut pc: u32 = 0x82633248;
    'dispatch: loop {
        match pc {
            0x82633248 => {
    //   block [0x82633248..0x826332B8)
	// 82633248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263324C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633254: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633258: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263325C: 38AA7CB4  addi r5, r10, 0x7cb4
	ctx.r[5].s64 = ctx.r[10].s64 + 31924;
	// 82633260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633264: 390BEF08  addi r8, r11, -0x10f8
	ctx.r[8].s64 = ctx.r[11].s64 + -4344;
	// 82633268: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8263326C: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 82633270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633274: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263327C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633280: 386A7C84  addi r3, r10, 0x7c84
	ctx.r[3].s64 = ctx.r[10].s64 + 31876;
	// 82633284: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263328C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263329C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826332A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826332A4: 4BE33B7D  bl 0x82466e20
	ctx.lr = 0x826332A8;
	sub_82466E20(ctx, base);
	// 826332A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826332AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826332B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826332B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826332B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826332B8 size=100
    let mut pc: u32 = 0x826332B8;
    'dispatch: loop {
        match pc {
            0x826332B8 => {
    //   block [0x826332B8..0x8263331C)
	// 826332B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826332BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826332C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826332C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826332C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826332CC: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826332D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826332D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826332D8: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 826332DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826332E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826332E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826332E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826332EC: 386A7CB4  addi r3, r10, 0x7cb4
	ctx.r[3].s64 = ctx.r[10].s64 + 31924;
	// 826332F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826332F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826332F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826332FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633300: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82633304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633308: 4BE33B19  bl 0x82466e20
	ctx.lr = 0x8263330C;
	sub_82466E20(ctx, base);
	// 8263330C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82633320 size=24
    let mut pc: u32 = 0x82633320;
    'dispatch: loop {
        match pc {
            0x82633320 => {
    //   block [0x82633320..0x82633338)
	// 82633320: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633324: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82633328: 394A24B8  addi r10, r10, 0x24b8
	ctx.r[10].s64 = ctx.r[10].s64 + 9400;
	// 8263332C: 816BED24  lwz r11, -0x12dc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4828 as u32) ) } as u64;
	// 82633330: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82633334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633338 size=116
    let mut pc: u32 = 0x82633338;
    'dispatch: loop {
        match pc {
            0x82633338 => {
    //   block [0x82633338..0x826333AC)
	// 82633338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263333C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633344: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633348: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263334C: 390B24B8  addi r8, r11, 0x24b8
	ctx.r[8].s64 = ctx.r[11].s64 + 9400;
	// 82633350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633354: 392A5094  addi r9, r10, 0x5094
	ctx.r[9].s64 = ctx.r[10].s64 + 20628;
	// 82633358: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8263335C: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82633360: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633364: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263336C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263337C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82633380: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 82633384: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82633388: 386B7CE4  addi r3, r11, 0x7ce4
	ctx.r[3].s64 = ctx.r[11].s64 + 31972;
	// 8263338C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82633390: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633398: 4BE33A89  bl 0x82466e20
	ctx.lr = 0x8263339C;
	sub_82466E20(ctx, base);
	// 8263339C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826333A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826333A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826333A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826333B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826333B0 size=112
    let mut pc: u32 = 0x826333B0;
    'dispatch: loop {
        match pc {
            0x826333B0 => {
    //   block [0x826333B0..0x82633420)
	// 826333B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826333B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826333B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826333BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826333C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826333C4: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 826333C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826333CC: 390BEF80  addi r8, r11, -0x1080
	ctx.r[8].s64 = ctx.r[11].s64 + -4224;
	// 826333D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826333D4: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826333D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826333DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826333E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826333E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826333E8: 386A7D14  addi r3, r10, 0x7d14
	ctx.r[3].s64 = ctx.r[10].s64 + 32020;
	// 826333EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826333F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826333F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826333F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826333FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263340C: 4BE33A15  bl 0x82466e20
	ctx.lr = 0x82633410;
	sub_82466E20(ctx, base);
	// 82633410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263341C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633420 size=112
    let mut pc: u32 = 0x82633420;
    'dispatch: loop {
        match pc {
            0x82633420 => {
    //   block [0x82633420..0x82633490)
	// 82633420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263342C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633430: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633434: 38AA7C54  addi r5, r10, 0x7c54
	ctx.r[5].s64 = ctx.r[10].s64 + 31828;
	// 82633438: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263343C: 390BEFC8  addi r8, r11, -0x1038
	ctx.r[8].s64 = ctx.r[11].s64 + -4152;
	// 82633440: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82633444: 388A22D4  addi r4, r10, 0x22d4
	ctx.r[4].s64 = ctx.r[10].s64 + 8916;
	// 82633448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263344C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633458: 386A7D44  addi r3, r10, 0x7d44
	ctx.r[3].s64 = ctx.r[10].s64 + 32068;
	// 8263345C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263346C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263347C: 4BE339A5  bl 0x82466e20
	ctx.lr = 0x82633480;
	sub_82466E20(ctx, base);
	// 82633480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263348C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633490 size=108
    let mut pc: u32 = 0x82633490;
    'dispatch: loop {
        match pc {
            0x82633490 => {
    //   block [0x82633490..0x826334FC)
	// 82633490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263349C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826334A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826334A4: 38EBF028  addi r7, r11, -0xfd8
	ctx.r[7].s64 = ctx.r[11].s64 + -4056;
	// 826334A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826334AC: 388A22F4  addi r4, r10, 0x22f4
	ctx.r[4].s64 = ctx.r[10].s64 + 8948;
	// 826334B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826334B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826334B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826334BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826334C0: 386A7D74  addi r3, r10, 0x7d74
	ctx.r[3].s64 = ctx.r[10].s64 + 32116;
	// 826334C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826334C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826334CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826334D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826334D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826334D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826334DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826334E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826334E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826334E8: 4BE33939  bl 0x82466e20
	ctx.lr = 0x826334EC;
	sub_82466E20(ctx, base);
	// 826334EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826334F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826334F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826334F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633500 size=108
    let mut pc: u32 = 0x82633500;
    'dispatch: loop {
        match pc {
            0x82633500 => {
    //   block [0x82633500..0x8263356C)
	// 82633500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263350C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633510: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633514: 38EBF070  addi r7, r11, -0xf90
	ctx.r[7].s64 = ctx.r[11].s64 + -3984;
	// 82633518: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263351C: 388A2320  addi r4, r10, 0x2320
	ctx.r[4].s64 = ctx.r[10].s64 + 8992;
	// 82633520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633524: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263352C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633530: 386A7DA4  addi r3, r10, 0x7da4
	ctx.r[3].s64 = ctx.r[10].s64 + 32164;
	// 82633534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263353C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263354C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633558: 4BE338C9  bl 0x82466e20
	ctx.lr = 0x8263355C;
	sub_82466E20(ctx, base);
	// 8263355C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633570 size=112
    let mut pc: u32 = 0x82633570;
    'dispatch: loop {
        match pc {
            0x82633570 => {
    //   block [0x82633570..0x826335E0)
	// 82633570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263357C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633580: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633584: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263358C: 390BF0B8  addi r8, r11, -0xf48
	ctx.r[8].s64 = ctx.r[11].s64 + -3912;
	// 82633590: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82633594: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 82633598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263359C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826335A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826335A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826335A8: 386A7DD4  addi r3, r10, 0x7dd4
	ctx.r[3].s64 = ctx.r[10].s64 + 32212;
	// 826335AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826335B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826335B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826335B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826335BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826335C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826335C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826335C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826335CC: 4BE33855  bl 0x82466e20
	ctx.lr = 0x826335D0;
	sub_82466E20(ctx, base);
	// 826335D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826335D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826335D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826335DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826335E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826335E0 size=112
    let mut pc: u32 = 0x826335E0;
    'dispatch: loop {
        match pc {
            0x826335E0 => {
    //   block [0x826335E0..0x82633650)
	// 826335E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826335E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826335E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826335EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826335F0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826335F4: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 826335F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826335FC: 390BF160  addi r8, r11, -0xea0
	ctx.r[8].s64 = ctx.r[11].s64 + -3744;
	// 82633600: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82633604: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 82633608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263360C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633618: 386A7E04  addi r3, r10, 0x7e04
	ctx.r[3].s64 = ctx.r[10].s64 + 32260;
	// 8263361C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263362C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263363C: 4BE337E5  bl 0x82466e20
	ctx.lr = 0x82633640;
	sub_82466E20(ctx, base);
	// 82633640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263364C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633650 size=108
    let mut pc: u32 = 0x82633650;
    'dispatch: loop {
        match pc {
            0x82633650 => {
    //   block [0x82633650..0x826336BC)
	// 82633650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263365C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633660: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633664: 38EBF1A8  addi r7, r11, -0xe58
	ctx.r[7].s64 = ctx.r[11].s64 + -3672;
	// 82633668: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263366C: 388A234C  addi r4, r10, 0x234c
	ctx.r[4].s64 = ctx.r[10].s64 + 9036;
	// 82633670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633674: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263367C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633680: 386A7E34  addi r3, r10, 0x7e34
	ctx.r[3].s64 = ctx.r[10].s64 + 32308;
	// 82633684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263368C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263369C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826336A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826336A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826336A8: 4BE33779  bl 0x82466e20
	ctx.lr = 0x826336AC;
	sub_82466E20(ctx, base);
	// 826336AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826336B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826336B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826336B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826336C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826336C0 size=108
    let mut pc: u32 = 0x826336C0;
    'dispatch: loop {
        match pc {
            0x826336C0 => {
    //   block [0x826336C0..0x8263372C)
	// 826336C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826336C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826336C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826336CC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826336D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826336D4: 38EBF1D8  addi r7, r11, -0xe28
	ctx.r[7].s64 = ctx.r[11].s64 + -3624;
	// 826336D8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826336DC: 388A2374  addi r4, r10, 0x2374
	ctx.r[4].s64 = ctx.r[10].s64 + 9076;
	// 826336E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826336E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826336E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826336EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826336F0: 386A7E64  addi r3, r10, 0x7e64
	ctx.r[3].s64 = ctx.r[10].s64 + 32356;
	// 826336F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826336F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826336FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263370C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633718: 4BE33709  bl 0x82466e20
	ctx.lr = 0x8263371C;
	sub_82466E20(ctx, base);
	// 8263371C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633730 size=112
    let mut pc: u32 = 0x82633730;
    'dispatch: loop {
        match pc {
            0x82633730 => {
    //   block [0x82633730..0x826337A0)
	// 82633730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263373C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633740: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633744: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263374C: 390BF268  addi r8, r11, -0xd98
	ctx.r[8].s64 = ctx.r[11].s64 + -3480;
	// 82633750: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82633754: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 82633758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263375C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633768: 386A7E94  addi r3, r10, 0x7e94
	ctx.r[3].s64 = ctx.r[10].s64 + 32404;
	// 8263376C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263377C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263378C: 4BE33695  bl 0x82466e20
	ctx.lr = 0x82633790;
	sub_82466E20(ctx, base);
	// 82633790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263379C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826337A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826337A0 size=112
    let mut pc: u32 = 0x826337A0;
    'dispatch: loop {
        match pc {
            0x826337A0 => {
    //   block [0x826337A0..0x82633810)
	// 826337A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826337A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826337A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826337AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826337B0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826337B4: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 826337B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826337BC: 390BF2F8  addi r8, r11, -0xd08
	ctx.r[8].s64 = ctx.r[11].s64 + -3336;
	// 826337C0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826337C4: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826337C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826337CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826337D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826337D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826337D8: 386A7EC4  addi r3, r10, 0x7ec4
	ctx.r[3].s64 = ctx.r[10].s64 + 32452;
	// 826337DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826337E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826337E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826337E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826337EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826337F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826337F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826337F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826337FC: 4BE33625  bl 0x82466e20
	ctx.lr = 0x82633800;
	sub_82466E20(ctx, base);
	// 82633800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263380C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633810 size=100
    let mut pc: u32 = 0x82633810;
    'dispatch: loop {
        match pc {
            0x82633810 => {
    //   block [0x82633810..0x82633874)
	// 82633810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263381C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633824: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263382C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633830: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 82633834: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263383C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633844: 386A7EF4  addi r3, r10, 0x7ef4
	ctx.r[3].s64 = ctx.r[10].s64 + 32500;
	// 82633848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263384C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633850: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82633854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633858: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263385C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633860: 4BE335C1  bl 0x82466e20
	ctx.lr = 0x82633864;
	sub_82466E20(ctx, base);
	// 82633864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263386C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633878 size=112
    let mut pc: u32 = 0x82633878;
    'dispatch: loop {
        match pc {
            0x82633878 => {
    //   block [0x82633878..0x826338E8)
	// 82633878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263387C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633884: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633888: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263388C: 38AA7AD4  addi r5, r10, 0x7ad4
	ctx.r[5].s64 = ctx.r[10].s64 + 31444;
	// 82633890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633894: 390BF3B8  addi r8, r11, -0xc48
	ctx.r[8].s64 = ctx.r[11].s64 + -3144;
	// 82633898: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263389C: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 826338A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826338A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826338A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826338AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826338B0: 386A7F24  addi r3, r10, 0x7f24
	ctx.r[3].s64 = ctx.r[10].s64 + 32548;
	// 826338B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826338B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826338BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826338C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826338C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826338C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826338CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826338D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826338D4: 4BE3354D  bl 0x82466e20
	ctx.lr = 0x826338D8;
	sub_82466E20(ctx, base);
	// 826338D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826338DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826338E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826338E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826338E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826338E8 size=112
    let mut pc: u32 = 0x826338E8;
    'dispatch: loop {
        match pc {
            0x826338E8 => {
    //   block [0x826338E8..0x82633958)
	// 826338E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826338EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826338F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826338F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826338F8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826338FC: 38AA7924  addi r5, r10, 0x7924
	ctx.r[5].s64 = ctx.r[10].s64 + 31012;
	// 82633900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633904: 390BF3E8  addi r8, r11, -0xc18
	ctx.r[8].s64 = ctx.r[11].s64 + -3096;
	// 82633908: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263390C: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 82633910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633914: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633918: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263391C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633920: 386A7F54  addi r3, r10, 0x7f54
	ctx.r[3].s64 = ctx.r[10].s64 + 32596;
	// 82633924: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633928: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263392C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263393C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633944: 4BE334DD  bl 0x82466e20
	ctx.lr = 0x82633948;
	sub_82466E20(ctx, base);
	// 82633948: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263394C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633958 size=108
    let mut pc: u32 = 0x82633958;
    'dispatch: loop {
        match pc {
            0x82633958 => {
    //   block [0x82633958..0x826339C4)
	// 82633958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263395C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633964: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263396C: 38EBF400  addi r7, r11, -0xc00
	ctx.r[7].s64 = ctx.r[11].s64 + -3072;
	// 82633970: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82633974: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 82633978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263397C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633980: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82633984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633988: 386A7F84  addi r3, r10, 0x7f84
	ctx.r[3].s64 = ctx.r[10].s64 + 32644;
	// 8263398C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263399C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826339A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826339A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826339A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826339AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826339B0: 4BE33471  bl 0x82466e20
	ctx.lr = 0x826339B4;
	sub_82466E20(ctx, base);
	// 826339B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826339B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826339BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826339C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826339C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826339C8 size=112
    let mut pc: u32 = 0x826339C8;
    'dispatch: loop {
        match pc {
            0x826339C8 => {
    //   block [0x826339C8..0x82633A38)
	// 826339C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826339CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826339D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826339D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826339D8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826339DC: 38AA7EF4  addi r5, r10, 0x7ef4
	ctx.r[5].s64 = ctx.r[10].s64 + 32500;
	// 826339E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826339E4: 390BF430  addi r8, r11, -0xbd0
	ctx.r[8].s64 = ctx.r[11].s64 + -3024;
	// 826339E8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826339EC: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826339F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826339F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826339F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826339FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633A00: 386A7FB4  addi r3, r10, 0x7fb4
	ctx.r[3].s64 = ctx.r[10].s64 + 32692;
	// 82633A04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633A08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633A10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633A18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633A1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633A20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633A24: 4BE333FD  bl 0x82466e20
	ctx.lr = 0x82633A28;
	sub_82466E20(ctx, base);
	// 82633A28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633A38 size=108
    let mut pc: u32 = 0x82633A38;
    'dispatch: loop {
        match pc {
            0x82633A38 => {
    //   block [0x82633A38..0x82633AA4)
	// 82633A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633A44: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633A48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633A4C: 38EBF4A8  addi r7, r11, -0xb58
	ctx.r[7].s64 = ctx.r[11].s64 + -2904;
	// 82633A50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82633A54: 388A23DC  addi r4, r10, 0x23dc
	ctx.r[4].s64 = ctx.r[10].s64 + 9180;
	// 82633A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633A5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633A60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82633A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633A68: 386A7FE4  addi r3, r10, 0x7fe4
	ctx.r[3].s64 = ctx.r[10].s64 + 32740;
	// 82633A6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633A70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633A8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633A90: 4BE33391  bl 0x82466e20
	ctx.lr = 0x82633A94;
	sub_82466E20(ctx, base);
	// 82633A94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633AA8 size=108
    let mut pc: u32 = 0x82633AA8;
    'dispatch: loop {
        match pc {
            0x82633AA8 => {
    //   block [0x82633AA8..0x82633B14)
	// 82633AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633AB4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633AB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633ABC: 38EBF4F0  addi r7, r11, -0xb10
	ctx.r[7].s64 = ctx.r[11].s64 + -2832;
	// 82633AC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82633AC4: 388A2400  addi r4, r10, 0x2400
	ctx.r[4].s64 = ctx.r[10].s64 + 9216;
	// 82633AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633ACC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633AD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82633AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633AD8: 386A8014  addi r3, r10, -0x7fec
	ctx.r[3].s64 = ctx.r[10].s64 + -32748;
	// 82633ADC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633AE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633AF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633AFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633B00: 4BE33321  bl 0x82466e20
	ctx.lr = 0x82633B04;
	sub_82466E20(ctx, base);
	// 82633B04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633B18 size=112
    let mut pc: u32 = 0x82633B18;
    'dispatch: loop {
        match pc {
            0x82633B18 => {
    //   block [0x82633B18..0x82633B88)
	// 82633B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633B24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633B28: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633B2C: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633B34: 390BF520  addi r8, r11, -0xae0
	ctx.r[8].s64 = ctx.r[11].s64 + -2784;
	// 82633B38: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82633B3C: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 82633B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633B44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633B50: 386A8044  addi r3, r10, -0x7fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -32700;
	// 82633B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633B74: 4BE332AD  bl 0x82466e20
	ctx.lr = 0x82633B78;
	sub_82466E20(ctx, base);
	// 82633B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633B88 size=108
    let mut pc: u32 = 0x82633B88;
    'dispatch: loop {
        match pc {
            0x82633B88 => {
    //   block [0x82633B88..0x82633BF4)
	// 82633B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633B94: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633B9C: 38EBF5B0  addi r7, r11, -0xa50
	ctx.r[7].s64 = ctx.r[11].s64 + -2640;
	// 82633BA0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82633BA4: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 82633BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633BAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633BB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82633BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633BB8: 386A8074  addi r3, r10, -0x7f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -32652;
	// 82633BBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633BDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633BE0: 4BE33241  bl 0x82466e20
	ctx.lr = 0x82633BE4;
	sub_82466E20(ctx, base);
	// 82633BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633BF8 size=116
    let mut pc: u32 = 0x82633BF8;
    'dispatch: loop {
        match pc {
            0x82633BF8 => {
    //   block [0x82633BF8..0x82633C6C)
	// 82633BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633C04: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82633C08: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82633C0C: 390AF640  addi r8, r10, -0x9c0
	ctx.r[8].s64 = ctx.r[10].s64 + -2496;
	// 82633C10: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633C14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82633C18: 38AA7EF4  addi r5, r10, 0x7ef4
	ctx.r[5].s64 = ctx.r[10].s64 + 32500;
	// 82633C1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633C20: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82633C24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633C2C: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 82633C30: 396B50A8  addi r11, r11, 0x50a8
	ctx.r[11].s64 = ctx.r[11].s64 + 20648;
	// 82633C34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633C38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633C3C: 386A80A4  addi r3, r10, -0x7f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -32604;
	// 82633C40: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82633C44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633C48: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82633C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633C58: 4BE331C9  bl 0x82466e20
	ctx.lr = 0x82633C5C;
	sub_82466E20(ctx, base);
	// 82633C5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633C70 size=108
    let mut pc: u32 = 0x82633C70;
    'dispatch: loop {
        match pc {
            0x82633C70 => {
    //   block [0x82633C70..0x82633CDC)
	// 82633C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633C7C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633C84: 38EBF700  addi r7, r11, -0x900
	ctx.r[7].s64 = ctx.r[11].s64 + -2304;
	// 82633C88: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82633C8C: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 82633C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633C94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633C98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82633C9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633CA0: 386A80D4  addi r3, r10, -0x7f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -32556;
	// 82633CA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633CC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633CC8: 4BE33159  bl 0x82466e20
	ctx.lr = 0x82633CCC;
	sub_82466E20(ctx, base);
	// 82633CCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633CE0 size=112
    let mut pc: u32 = 0x82633CE0;
    'dispatch: loop {
        match pc {
            0x82633CE0 => {
    //   block [0x82633CE0..0x82633D50)
	// 82633CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633CEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633CF0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633CF4: 38AA7EF4  addi r5, r10, 0x7ef4
	ctx.r[5].s64 = ctx.r[10].s64 + 32500;
	// 82633CF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633CFC: 390BF748  addi r8, r11, -0x8b8
	ctx.r[8].s64 = ctx.r[11].s64 + -2232;
	// 82633D00: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82633D04: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 82633D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633D0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633D10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633D18: 386A8104  addi r3, r10, -0x7efc
	ctx.r[3].s64 = ctx.r[10].s64 + -32508;
	// 82633D1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633D20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633D3C: 4BE330E5  bl 0x82466e20
	ctx.lr = 0x82633D40;
	sub_82466E20(ctx, base);
	// 82633D40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633D50 size=112
    let mut pc: u32 = 0x82633D50;
    'dispatch: loop {
        match pc {
            0x82633D50 => {
    //   block [0x82633D50..0x82633DC0)
	// 82633D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633D5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633D60: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633D64: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633D6C: 390BF7A8  addi r8, r11, -0x858
	ctx.r[8].s64 = ctx.r[11].s64 + -2136;
	// 82633D70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82633D74: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 82633D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633D7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633D80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633D88: 386A8134  addi r3, r10, -0x7ecc
	ctx.r[3].s64 = ctx.r[10].s64 + -32460;
	// 82633D8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633DAC: 4BE33075  bl 0x82466e20
	ctx.lr = 0x82633DB0;
	sub_82466E20(ctx, base);
	// 82633DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633DC0 size=108
    let mut pc: u32 = 0x82633DC0;
    'dispatch: loop {
        match pc {
            0x82633DC0 => {
    //   block [0x82633DC0..0x82633E2C)
	// 82633DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633DCC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633DD4: 38EBF7C0  addi r7, r11, -0x840
	ctx.r[7].s64 = ctx.r[11].s64 + -2112;
	// 82633DD8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82633DDC: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 82633DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633DE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82633DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633DF0: 386A8164  addi r3, r10, -0x7e9c
	ctx.r[3].s64 = ctx.r[10].s64 + -32412;
	// 82633DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633E18: 4BE33009  bl 0x82466e20
	ctx.lr = 0x82633E1C;
	sub_82466E20(ctx, base);
	// 82633E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633E30 size=112
    let mut pc: u32 = 0x82633E30;
    'dispatch: loop {
        match pc {
            0x82633E30 => {
    //   block [0x82633E30..0x82633EA0)
	// 82633E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633E3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633E40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633E44: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633E4C: 390BF838  addi r8, r11, -0x7c8
	ctx.r[8].s64 = ctx.r[11].s64 + -1992;
	// 82633E50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82633E54: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82633E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633E5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633E60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633E68: 386A8194  addi r3, r10, -0x7e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -32364;
	// 82633E6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633E8C: 4BE32F95  bl 0x82466e20
	ctx.lr = 0x82633E90;
	sub_82466E20(ctx, base);
	// 82633E90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633EA0 size=104
    let mut pc: u32 = 0x82633EA0;
    'dispatch: loop {
        match pc {
            0x82633EA0 => {
    //   block [0x82633EA0..0x82633F08)
	// 82633EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633EAC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633EB4: 392A5100  addi r9, r10, 0x5100
	ctx.r[9].s64 = ctx.r[10].s64 + 20736;
	// 82633EB8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633EC0: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82633EC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633ED4: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 82633ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633EDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633EE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82633EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633EE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82633EEC: 386A81C4  addi r3, r10, -0x7e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -32316;
	// 82633EF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82633EF4: 4BE32F2D  bl 0x82466e20
	ctx.lr = 0x82633EF8;
	sub_82466E20(ctx, base);
	// 82633EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633F08 size=112
    let mut pc: u32 = 0x82633F08;
    'dispatch: loop {
        match pc {
            0x82633F08 => {
    //   block [0x82633F08..0x82633F78)
	// 82633F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633F14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633F18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633F1C: 38AA81C4  addi r5, r10, -0x7e3c
	ctx.r[5].s64 = ctx.r[10].s64 + -32316;
	// 82633F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633F24: 390BF86C  addi r8, r11, -0x794
	ctx.r[8].s64 = ctx.r[11].s64 + -1940;
	// 82633F28: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82633F2C: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 82633F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633F34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633F40: 386A81F4  addi r3, r10, -0x7e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -32268;
	// 82633F44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633F64: 4BE32EBD  bl 0x82466e20
	ctx.lr = 0x82633F68;
	sub_82466E20(ctx, base);
	// 82633F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633F78 size=112
    let mut pc: u32 = 0x82633F78;
    'dispatch: loop {
        match pc {
            0x82633F78 => {
    //   block [0x82633F78..0x82633FE8)
	// 82633F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633F84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633F88: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633F8C: 38AA81F4  addi r5, r10, -0x7e0c
	ctx.r[5].s64 = ctx.r[10].s64 + -32268;
	// 82633F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633F94: 390BF8A0  addi r8, r11, -0x760
	ctx.r[8].s64 = ctx.r[11].s64 + -1888;
	// 82633F98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82633F9C: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 82633FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633FA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633FB0: 386A8224  addi r3, r10, -0x7ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -32220;
	// 82633FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633FD4: 4BE32E4D  bl 0x82466e20
	ctx.lr = 0x82633FD8;
	sub_82466E20(ctx, base);
	// 82633FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633FE8 size=112
    let mut pc: u32 = 0x82633FE8;
    'dispatch: loop {
        match pc {
            0x82633FE8 => {
    //   block [0x82633FE8..0x82634058)
	// 82633FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633FF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633FF8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633FFC: 38AA81F4  addi r5, r10, -0x7e0c
	ctx.r[5].s64 = ctx.r[10].s64 + -32268;
	// 82634000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634004: 390BF900  addi r8, r11, -0x700
	ctx.r[8].s64 = ctx.r[11].s64 + -1792;
	// 82634008: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263400C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82634010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634014: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263401C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634020: 386A8254  addi r3, r10, -0x7dac
	ctx.r[3].s64 = ctx.r[10].s64 + -32172;
	// 82634024: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263402C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263403C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634044: 4BE32DDD  bl 0x82466e20
	ctx.lr = 0x82634048;
	sub_82466E20(ctx, base);
	// 82634048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263404C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634058 size=112
    let mut pc: u32 = 0x82634058;
    'dispatch: loop {
        match pc {
            0x82634058 => {
    //   block [0x82634058..0x826340C8)
	// 82634058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263405C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634064: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634068: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263406C: 38AA81F4  addi r5, r10, -0x7e0c
	ctx.r[5].s64 = ctx.r[10].s64 + -32268;
	// 82634070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634074: 390BF930  addi r8, r11, -0x6d0
	ctx.r[8].s64 = ctx.r[11].s64 + -1744;
	// 82634078: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263407C: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 82634080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634084: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263408C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634090: 386A8284  addi r3, r10, -0x7d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -32124;
	// 82634094: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263409C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826340A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826340A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826340A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826340AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826340B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826340B4: 4BE32D6D  bl 0x82466e20
	ctx.lr = 0x826340B8;
	sub_82466E20(ctx, base);
	// 826340B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826340BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826340C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826340C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826340C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826340C8 size=112
    let mut pc: u32 = 0x826340C8;
    'dispatch: loop {
        match pc {
            0x826340C8 => {
    //   block [0x826340C8..0x82634138)
	// 826340C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826340CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826340D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826340D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826340D8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826340DC: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 826340E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826340E4: 390BF978  addi r8, r11, -0x688
	ctx.r[8].s64 = ctx.r[11].s64 + -1672;
	// 826340E8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826340EC: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 826340F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826340F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826340F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826340FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634100: 386A82B4  addi r3, r10, -0x7d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -32076;
	// 82634104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263410C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263411C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634124: 4BE32CFD  bl 0x82466e20
	ctx.lr = 0x82634128;
	sub_82466E20(ctx, base);
	// 82634128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263412C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634138 size=116
    let mut pc: u32 = 0x82634138;
    'dispatch: loop {
        match pc {
            0x82634138 => {
    //   block [0x82634138..0x826341AC)
	// 82634138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263413C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634144: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82634148: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 8263414C: 390AFA08  addi r8, r10, -0x5f8
	ctx.r[8].s64 = ctx.r[10].s64 + -1528;
	// 82634150: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634154: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82634158: 38AA8764  addi r5, r10, -0x789c
	ctx.r[5].s64 = ctx.r[10].s64 + -30876;
	// 8263415C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634160: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82634164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263416C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 82634170: 396B5118  addi r11, r11, 0x5118
	ctx.r[11].s64 = ctx.r[11].s64 + 20760;
	// 82634174: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634178: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263417C: 386A82E4  addi r3, r10, -0x7d1c
	ctx.r[3].s64 = ctx.r[10].s64 + -32028;
	// 82634180: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82634184: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634188: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263418C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634198: 4BE32C89  bl 0x82466e20
	ctx.lr = 0x8263419C;
	sub_82466E20(ctx, base);
	// 8263419C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826341A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826341A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826341A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826341B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826341B0 size=100
    let mut pc: u32 = 0x826341B0;
    'dispatch: loop {
        match pc {
            0x826341B0 => {
    //   block [0x826341B0..0x82634214)
	// 826341B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826341B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826341B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826341BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826341C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826341C4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826341C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826341CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826341D0: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826341D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826341D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826341DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826341E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826341E4: 386A8314  addi r3, r10, -0x7cec
	ctx.r[3].s64 = ctx.r[10].s64 + -31980;
	// 826341E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826341EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826341F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826341F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826341F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826341FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634200: 4BE32C21  bl 0x82466e20
	ctx.lr = 0x82634204;
	sub_82466E20(ctx, base);
	// 82634204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263420C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634218 size=100
    let mut pc: u32 = 0x82634218;
    'dispatch: loop {
        match pc {
            0x82634218 => {
    //   block [0x82634218..0x8263427C)
	// 82634218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263421C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634224: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263422C: 38AA83A4  addi r5, r10, -0x7c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -31836;
	// 82634230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634238: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 8263423C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263424C: 386A8344  addi r3, r10, -0x7cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -31932;
	// 82634250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634254: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634258: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263425C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634260: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82634264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634268: 4BE32BB9  bl 0x82466e20
	ctx.lr = 0x8263426C;
	sub_82466E20(ctx, base);
	// 8263426C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634280 size=100
    let mut pc: u32 = 0x82634280;
    'dispatch: loop {
        match pc {
            0x82634280 => {
    //   block [0x82634280..0x826342E4)
	// 82634280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263428C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634294: 38AA82E4  addi r5, r10, -0x7d1c
	ctx.r[5].s64 = ctx.r[10].s64 + -32028;
	// 82634298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263429C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826342A0: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 826342A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826342A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826342AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826342B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826342B4: 386A8374  addi r3, r10, -0x7c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -31884;
	// 826342B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826342BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826342C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826342C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826342C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826342CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826342D0: 4BE32B51  bl 0x82466e20
	ctx.lr = 0x826342D4;
	sub_82466E20(ctx, base);
	// 826342D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826342D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826342DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826342E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826342E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826342E8 size=104
    let mut pc: u32 = 0x826342E8;
    'dispatch: loop {
        match pc {
            0x826342E8 => {
    //   block [0x826342E8..0x82634350)
	// 826342E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826342EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826342F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826342F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826342F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826342FC: 392A5184  addi r9, r10, 0x5184
	ctx.r[9].s64 = ctx.r[10].s64 + 20868;
	// 82634300: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634308: 38AA8314  addi r5, r10, -0x7cec
	ctx.r[5].s64 = ctx.r[10].s64 + -31980;
	// 8263430C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263431C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 82634320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634324: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634328: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263432C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634330: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82634334: 386A83A4  addi r3, r10, -0x7c5c
	ctx.r[3].s64 = ctx.r[10].s64 + -31836;
	// 82634338: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263433C: 4BE32AE5  bl 0x82466e20
	ctx.lr = 0x82634340;
	sub_82466E20(ctx, base);
	// 82634340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263434C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634350 size=108
    let mut pc: u32 = 0x82634350;
    'dispatch: loop {
        match pc {
            0x82634350 => {
    //   block [0x82634350..0x826343BC)
	// 82634350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263435C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634364: 38EBFBB8  addi r7, r11, -0x448
	ctx.r[7].s64 = ctx.r[11].s64 + -1096;
	// 82634368: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263436C: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82634370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634374: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634378: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263437C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634380: 386A83D4  addi r3, r10, -0x7c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -31788;
	// 82634384: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82634388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263438C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263439C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826343A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826343A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826343A8: 4BE32A79  bl 0x82466e20
	ctx.lr = 0x826343AC;
	sub_82466E20(ctx, base);
	// 826343AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826343B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826343B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826343B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826343C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826343C0 size=112
    let mut pc: u32 = 0x826343C0;
    'dispatch: loop {
        match pc {
            0x826343C0 => {
    //   block [0x826343C0..0x82634430)
	// 826343C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826343C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826343C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826343CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826343D0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826343D4: 38AA83A4  addi r5, r10, -0x7c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -31836;
	// 826343D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826343DC: 390BFBE8  addi r8, r11, -0x418
	ctx.r[8].s64 = ctx.r[11].s64 + -1048;
	// 826343E0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826343E4: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 826343E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826343EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826343F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826343F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826343F8: 386A8404  addi r3, r10, -0x7bfc
	ctx.r[3].s64 = ctx.r[10].s64 + -31740;
	// 826343FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263440C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263441C: 4BE32A05  bl 0x82466e20
	ctx.lr = 0x82634420;
	sub_82466E20(ctx, base);
	// 82634420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263442C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634430 size=116
    let mut pc: u32 = 0x82634430;
    'dispatch: loop {
        match pc {
            0x82634430 => {
    //   block [0x82634430..0x826344A4)
	// 82634430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263443C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634440: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82634444: 390BFC94  addi r8, r11, -0x36c
	ctx.r[8].s64 = ctx.r[11].s64 + -876;
	// 82634448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263444C: 392A51E8  addi r9, r10, 0x51e8
	ctx.r[9].s64 = ctx.r[10].s64 + 20968;
	// 82634450: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634454: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82634458: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 8263445C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634464: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263446C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634474: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82634478: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8263447C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82634480: 386B8434  addi r3, r11, -0x7bcc
	ctx.r[3].s64 = ctx.r[11].s64 + -31692;
	// 82634484: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82634488: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263448C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634490: 4BE32991  bl 0x82466e20
	ctx.lr = 0x82634494;
	sub_82466E20(ctx, base);
	// 82634494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263449C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826344A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826344A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826344A8 size=112
    let mut pc: u32 = 0x826344A8;
    'dispatch: loop {
        match pc {
            0x826344A8 => {
    //   block [0x826344A8..0x82634518)
	// 826344A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826344AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826344B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826344B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826344B8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826344BC: 38AA84F4  addi r5, r10, -0x7b0c
	ctx.r[5].s64 = ctx.r[10].s64 + -31500;
	// 826344C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826344C4: 390BFCAC  addi r8, r11, -0x354
	ctx.r[8].s64 = ctx.r[11].s64 + -852;
	// 826344C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826344CC: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826344D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826344D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826344D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826344DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826344E0: 386A8464  addi r3, r10, -0x7b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -31644;
	// 826344E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826344E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826344EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826344F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826344F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826344F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826344FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634504: 4BE3291D  bl 0x82466e20
	ctx.lr = 0x82634508;
	sub_82466E20(ctx, base);
	// 82634508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263450C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634518 size=100
    let mut pc: u32 = 0x82634518;
    'dispatch: loop {
        match pc {
            0x82634518 => {
    //   block [0x82634518..0x8263457C)
	// 82634518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263451C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634524: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263452C: 38AA84C4  addi r5, r10, -0x7b3c
	ctx.r[5].s64 = ctx.r[10].s64 + -31548;
	// 82634530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634538: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 8263453C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263454C: 386A8494  addi r3, r10, -0x7b6c
	ctx.r[3].s64 = ctx.r[10].s64 + -31596;
	// 82634550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634554: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634558: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263455C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634560: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82634564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634568: 4BE328B9  bl 0x82466e20
	ctx.lr = 0x8263456C;
	sub_82466E20(ctx, base);
	// 8263456C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634580 size=112
    let mut pc: u32 = 0x82634580;
    'dispatch: loop {
        match pc {
            0x82634580 => {
    //   block [0x82634580..0x826345F0)
	// 82634580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263458C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634590: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634594: 38AA84F4  addi r5, r10, -0x7b0c
	ctx.r[5].s64 = ctx.r[10].s64 + -31500;
	// 82634598: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263459C: 390BFCC4  addi r8, r11, -0x33c
	ctx.r[8].s64 = ctx.r[11].s64 + -828;
	// 826345A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826345A4: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 826345A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826345AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826345B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826345B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826345B8: 386A84C4  addi r3, r10, -0x7b3c
	ctx.r[3].s64 = ctx.r[10].s64 + -31548;
	// 826345BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826345C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826345C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826345C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826345CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826345D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826345D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826345D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826345DC: 4BE32845  bl 0x82466e20
	ctx.lr = 0x826345E0;
	sub_82466E20(ctx, base);
	// 826345E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826345E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826345E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826345EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826345F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826345F0 size=112
    let mut pc: u32 = 0x826345F0;
    'dispatch: loop {
        match pc {
            0x826345F0 => {
    //   block [0x826345F0..0x82634660)
	// 826345F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826345F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826345F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826345FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634600: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634604: 38AA8434  addi r5, r10, -0x7bcc
	ctx.r[5].s64 = ctx.r[10].s64 + -31692;
	// 82634608: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263460C: 390BFCF8  addi r8, r11, -0x308
	ctx.r[8].s64 = ctx.r[11].s64 + -776;
	// 82634610: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82634614: 388A2424  addi r4, r10, 0x2424
	ctx.r[4].s64 = ctx.r[10].s64 + 9252;
	// 82634618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263461C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634620: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634628: 386A84F4  addi r3, r10, -0x7b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -31500;
	// 8263462C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263463C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263464C: 4BE327D5  bl 0x82466e20
	ctx.lr = 0x82634650;
	sub_82466E20(ctx, base);
	// 82634650: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263465C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634660 size=100
    let mut pc: u32 = 0x82634660;
    'dispatch: loop {
        match pc {
            0x82634660 => {
    //   block [0x82634660..0x826346C4)
	// 82634660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263466C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634674: 38AA84F4  addi r5, r10, -0x7b0c
	ctx.r[5].s64 = ctx.r[10].s64 + -31500;
	// 82634678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263467C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634680: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 82634684: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263468C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634694: 386A8524  addi r3, r10, -0x7adc
	ctx.r[3].s64 = ctx.r[10].s64 + -31452;
	// 82634698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263469C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826346A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826346A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826346A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826346AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826346B0: 4BE32771  bl 0x82466e20
	ctx.lr = 0x826346B4;
	sub_82466E20(ctx, base);
	// 826346B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826346B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826346BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826346C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826346C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826346C8 size=100
    let mut pc: u32 = 0x826346C8;
    'dispatch: loop {
        match pc {
            0x826346C8 => {
    //   block [0x826346C8..0x8263472C)
	// 826346C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826346CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826346D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826346D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826346D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826346DC: 38AA8464  addi r5, r10, -0x7b9c
	ctx.r[5].s64 = ctx.r[10].s64 + -31644;
	// 826346E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826346E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826346E8: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 826346EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826346F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826346F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826346F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826346FC: 386A8554  addi r3, r10, -0x7aac
	ctx.r[3].s64 = ctx.r[10].s64 + -31404;
	// 82634700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634704: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634708: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263470C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634710: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82634714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634718: 4BE32709  bl 0x82466e20
	ctx.lr = 0x8263471C;
	sub_82466E20(ctx, base);
	// 8263471C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634730 size=100
    let mut pc: u32 = 0x82634730;
    'dispatch: loop {
        match pc {
            0x82634730 => {
    //   block [0x82634730..0x82634794)
	// 82634730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263473C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634744: 38AA8524  addi r5, r10, -0x7adc
	ctx.r[5].s64 = ctx.r[10].s64 + -31452;
	// 82634748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263474C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634750: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82634754: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263475C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634764: 386A8584  addi r3, r10, -0x7a7c
	ctx.r[3].s64 = ctx.r[10].s64 + -31356;
	// 82634768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263476C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634770: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82634774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634778: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263477C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634780: 4BE326A1  bl 0x82466e20
	ctx.lr = 0x82634784;
	sub_82466E20(ctx, base);
	// 82634784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263478C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634798 size=100
    let mut pc: u32 = 0x82634798;
    'dispatch: loop {
        match pc {
            0x82634798 => {
    //   block [0x82634798..0x826347FC)
	// 82634798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263479C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826347A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826347A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826347A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826347AC: 38AA8464  addi r5, r10, -0x7b9c
	ctx.r[5].s64 = ctx.r[10].s64 + -31644;
	// 826347B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826347B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826347B8: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 826347BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826347C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826347C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826347C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826347CC: 386A85B4  addi r3, r10, -0x7a4c
	ctx.r[3].s64 = ctx.r[10].s64 + -31308;
	// 826347D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826347D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826347D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826347DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826347E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826347E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826347E8: 4BE32639  bl 0x82466e20
	ctx.lr = 0x826347EC;
	sub_82466E20(ctx, base);
	// 826347EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826347F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826347F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826347F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634800 size=112
    let mut pc: u32 = 0x82634800;
    'dispatch: loop {
        match pc {
            0x82634800 => {
    //   block [0x82634800..0x82634870)
	// 82634800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263480C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634810: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634814: 38AA8644  addi r5, r10, -0x79bc
	ctx.r[5].s64 = ctx.r[10].s64 + -31164;
	// 82634818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263481C: 390BFDA0  addi r8, r11, -0x260
	ctx.r[8].s64 = ctx.r[11].s64 + -608;
	// 82634820: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82634824: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82634828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263482C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634838: 386A85E4  addi r3, r10, -0x7a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -31260;
	// 8263483C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263484C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263485C: 4BE325C5  bl 0x82466e20
	ctx.lr = 0x82634860;
	sub_82466E20(ctx, base);
	// 82634860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263486C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634870 size=112
    let mut pc: u32 = 0x82634870;
    'dispatch: loop {
        match pc {
            0x82634870 => {
    //   block [0x82634870..0x826348E0)
	// 82634870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263487C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634880: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634884: 38AA8674  addi r5, r10, -0x798c
	ctx.r[5].s64 = ctx.r[10].s64 + -31116;
	// 82634888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263488C: 390BFDD0  addi r8, r11, -0x230
	ctx.r[8].s64 = ctx.r[11].s64 + -560;
	// 82634890: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82634894: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 82634898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263489C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826348A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826348A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826348A8: 386A8614  addi r3, r10, -0x79ec
	ctx.r[3].s64 = ctx.r[10].s64 + -31212;
	// 826348AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826348B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826348B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826348B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826348BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826348C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826348C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826348C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826348CC: 4BE32555  bl 0x82466e20
	ctx.lr = 0x826348D0;
	sub_82466E20(ctx, base);
	// 826348D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826348D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826348D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826348DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826348E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826348E0 size=112
    let mut pc: u32 = 0x826348E0;
    'dispatch: loop {
        match pc {
            0x826348E0 => {
    //   block [0x826348E0..0x82634950)
	// 826348E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826348E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826348E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826348EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826348F0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826348F4: 38AA8764  addi r5, r10, -0x789c
	ctx.r[5].s64 = ctx.r[10].s64 + -30876;
	// 826348F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826348FC: 390BFDE8  addi r8, r11, -0x218
	ctx.r[8].s64 = ctx.r[11].s64 + -536;
	// 82634900: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82634904: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82634908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263490C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634910: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634918: 386A8644  addi r3, r10, -0x79bc
	ctx.r[3].s64 = ctx.r[10].s64 + -31164;
	// 8263491C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263492C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263493C: 4BE324E5  bl 0x82466e20
	ctx.lr = 0x82634940;
	sub_82466E20(ctx, base);
	// 82634940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263494C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634950 size=112
    let mut pc: u32 = 0x82634950;
    'dispatch: loop {
        match pc {
            0x82634950 => {
    //   block [0x82634950..0x826349C0)
	// 82634950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263495C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634960: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634964: 38AA8644  addi r5, r10, -0x79bc
	ctx.r[5].s64 = ctx.r[10].s64 + -31164;
	// 82634968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263496C: 390BFE18  addi r8, r11, -0x1e8
	ctx.r[8].s64 = ctx.r[11].s64 + -488;
	// 82634970: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82634974: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 82634978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263497C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634988: 386A8674  addi r3, r10, -0x798c
	ctx.r[3].s64 = ctx.r[10].s64 + -31116;
	// 8263498C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263499C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826349A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826349A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826349A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826349AC: 4BE32475  bl 0x82466e20
	ctx.lr = 0x826349B0;
	sub_82466E20(ctx, base);
	// 826349B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826349B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826349B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826349BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826349C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826349C0 size=112
    let mut pc: u32 = 0x826349C0;
    'dispatch: loop {
        match pc {
            0x826349C0 => {
    //   block [0x826349C0..0x82634A30)
	// 826349C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826349C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826349C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826349CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826349D0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826349D4: 38AA8674  addi r5, r10, -0x798c
	ctx.r[5].s64 = ctx.r[10].s64 + -31116;
	// 826349D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826349DC: 390BFE30  addi r8, r11, -0x1d0
	ctx.r[8].s64 = ctx.r[11].s64 + -464;
	// 826349E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826349E4: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826349E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826349EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826349F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826349F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826349F8: 386A86A4  addi r3, r10, -0x795c
	ctx.r[3].s64 = ctx.r[10].s64 + -31068;
	// 826349FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634A1C: 4BE32405  bl 0x82466e20
	ctx.lr = 0x82634A20;
	sub_82466E20(ctx, base);
	// 82634A20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634A30 size=112
    let mut pc: u32 = 0x82634A30;
    'dispatch: loop {
        match pc {
            0x82634A30 => {
    //   block [0x82634A30..0x82634AA0)
	// 82634A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634A3C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82634A40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634A44: 392A5214  addi r9, r10, 0x5214
	ctx.r[9].s64 = ctx.r[10].s64 + 21012;
	// 82634A48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634A4C: 390BFE48  addi r8, r11, -0x1b8
	ctx.r[8].s64 = ctx.r[11].s64 + -440;
	// 82634A50: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82634A54: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 82634A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634A5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634A60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634A68: 386A86D4  addi r3, r10, -0x792c
	ctx.r[3].s64 = ctx.r[10].s64 + -31020;
	// 82634A6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82634A70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82634A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634A84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82634A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634A8C: 4BE32395  bl 0x82466e20
	ctx.lr = 0x82634A90;
	sub_82466E20(ctx, base);
	// 82634A90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634AA0 size=116
    let mut pc: u32 = 0x82634AA0;
    'dispatch: loop {
        match pc {
            0x82634AA0 => {
    //   block [0x82634AA0..0x82634B14)
	// 82634AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634AAC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82634AB0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82634AB4: 390AFE78  addi r8, r10, -0x188
	ctx.r[8].s64 = ctx.r[10].s64 + -392;
	// 82634AB8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634ABC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82634AC0: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82634AC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634AC8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82634ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634AD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634AD4: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82634AD8: 396B5228  addi r11, r11, 0x5228
	ctx.r[11].s64 = ctx.r[11].s64 + 21032;
	// 82634ADC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634AE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634AE4: 386A8704  addi r3, r10, -0x78fc
	ctx.r[3].s64 = ctx.r[10].s64 + -30972;
	// 82634AE8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82634AEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634AF0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82634AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634B00: 4BE32321  bl 0x82466e20
	ctx.lr = 0x82634B04;
	sub_82466E20(ctx, base);
	// 82634B04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82634B18 size=48
    let mut pc: u32 = 0x82634B18;
    'dispatch: loop {
        match pc {
            0x82634B18 => {
    //   block [0x82634B18..0x82634B48)
	// 82634B18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634B1C: 814BFF2C  lwz r10, -0xd4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-212 as u32) ) } as u64;
	// 82634B20: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634B24: 396B2518  addi r11, r11, 0x2518
	ctx.r[11].s64 = ctx.r[11].s64 + 9496;
	// 82634B28: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82634B2C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82634B30: 814AFF28  lwz r10, -0xd8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-216 as u32) ) } as u64;
	// 82634B34: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82634B38: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82634B3C: 814AFF24  lwz r10, -0xdc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-220 as u32) ) } as u64;
	// 82634B40: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 82634B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634B48 size=116
    let mut pc: u32 = 0x82634B48;
    'dispatch: loop {
        match pc {
            0x82634B48 => {
    //   block [0x82634B48..0x82634BBC)
	// 82634B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634B54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82634B58: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634B5C: 392B5328  addi r9, r11, 0x5328
	ctx.r[9].s64 = ctx.r[11].s64 + 21288;
	// 82634B60: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82634B64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634B68: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 82634B6C: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 82634B70: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634B74: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 82634B78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634B7C: 396B2518  addi r11, r11, 0x2518
	ctx.r[11].s64 = ctx.r[11].s64 + 9496;
	// 82634B80: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82634B84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634B88: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82634B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634B90: 386A8734  addi r3, r10, -0x78cc
	ctx.r[3].s64 = ctx.r[10].s64 + -30924;
	// 82634B94: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82634B98: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82634B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634BA0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82634BA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82634BA8: 4BE32279  bl 0x82466e20
	ctx.lr = 0x82634BAC;
	sub_82466E20(ctx, base);
	// 82634BAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634BC0 size=116
    let mut pc: u32 = 0x82634BC0;
    'dispatch: loop {
        match pc {
            0x82634BC0 => {
    //   block [0x82634BC0..0x82634C34)
	// 82634BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634BCC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634BD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82634BD4: 390BFF38  addi r8, r11, -0xc8
	ctx.r[8].s64 = ctx.r[11].s64 + -200;
	// 82634BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634BDC: 392A5410  addi r9, r10, 0x5410
	ctx.r[9].s64 = ctx.r[10].s64 + 21520;
	// 82634BE0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634BE4: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82634BE8: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82634BEC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634BF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634C04: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82634C08: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 82634C0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82634C10: 386B8764  addi r3, r11, -0x789c
	ctx.r[3].s64 = ctx.r[11].s64 + -30876;
	// 82634C14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82634C18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634C20: 4BE32201  bl 0x82466e20
	ctx.lr = 0x82634C24;
	sub_82466E20(ctx, base);
	// 82634C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634C38 size=112
    let mut pc: u32 = 0x82634C38;
    'dispatch: loop {
        match pc {
            0x82634C38 => {
    //   block [0x82634C38..0x82634CA8)
	// 82634C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634C44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634C48: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634C4C: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82634C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634C54: 390BFFC8  addi r8, r11, -0x38
	ctx.r[8].s64 = ctx.r[11].s64 + -56;
	// 82634C58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82634C5C: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82634C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634C64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634C70: 386A8794  addi r3, r10, -0x786c
	ctx.r[3].s64 = ctx.r[10].s64 + -30828;
	// 82634C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634C94: 4BE3218D  bl 0x82466e20
	ctx.lr = 0x82634C98;
	sub_82466E20(ctx, base);
	// 82634C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634CA8 size=112
    let mut pc: u32 = 0x82634CA8;
    'dispatch: loop {
        match pc {
            0x82634CA8 => {
    //   block [0x82634CA8..0x82634D18)
	// 82634CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634CB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634CB8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634CBC: 38AA6F64  addi r5, r10, 0x6f64
	ctx.r[5].s64 = ctx.r[10].s64 + 28516;
	// 82634CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634CC4: 390BFFE0  addi r8, r11, -0x20
	ctx.r[8].s64 = ctx.r[11].s64 + -32;
	// 82634CC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82634CCC: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82634CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634CD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634CD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634CE0: 386A87C4  addi r3, r10, -0x783c
	ctx.r[3].s64 = ctx.r[10].s64 + -30780;
	// 82634CE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634D04: 4BE3211D  bl 0x82466e20
	ctx.lr = 0x82634D08;
	sub_82466E20(ctx, base);
	// 82634D08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634D18 size=108
    let mut pc: u32 = 0x82634D18;
    'dispatch: loop {
        match pc {
            0x82634D18 => {
    //   block [0x82634D18..0x82634D84)
	// 82634D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634D24: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634D2C: 38EBFFF8  addi r7, r11, -8
	ctx.r[7].s64 = ctx.r[11].s64 + -8;
	// 82634D30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82634D34: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82634D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634D3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82634D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634D48: 386A87F4  addi r3, r10, -0x780c
	ctx.r[3].s64 = ctx.r[10].s64 + -30732;
	// 82634D4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82634D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82634D70: 4BE320B1  bl 0x82466e20
	ctx.lr = 0x82634D74;
	sub_82466E20(ctx, base);
	// 82634D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634D88 size=112
    let mut pc: u32 = 0x82634D88;
    'dispatch: loop {
        match pc {
            0x82634D88 => {
    //   block [0x82634D88..0x82634DF8)
	// 82634D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634D94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634D98: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634D9C: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82634DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634DA4: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82634DA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82634DAC: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82634DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634DB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634DC0: 386A8824  addi r3, r10, -0x77dc
	ctx.r[3].s64 = ctx.r[10].s64 + -30684;
	// 82634DC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634DE4: 4BE3203D  bl 0x82466e20
	ctx.lr = 0x82634DE8;
	sub_82466E20(ctx, base);
	// 82634DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634DF8 size=108
    let mut pc: u32 = 0x82634DF8;
    'dispatch: loop {
        match pc {
            0x82634DF8 => {
    //   block [0x82634DF8..0x82634E64)
	// 82634DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634E04: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634E0C: 38EB0058  addi r7, r11, 0x58
	ctx.r[7].s64 = ctx.r[11].s64 + 88;
	// 82634E10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82634E14: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82634E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634E1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634E20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82634E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634E28: 386A8854  addi r3, r10, -0x77ac
	ctx.r[3].s64 = ctx.r[10].s64 + -30636;
	// 82634E2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82634E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82634E50: 4BE31FD1  bl 0x82466e20
	ctx.lr = 0x82634E54;
	sub_82466E20(ctx, base);
	// 82634E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634E68 size=112
    let mut pc: u32 = 0x82634E68;
    'dispatch: loop {
        match pc {
            0x82634E68 => {
    //   block [0x82634E68..0x82634ED8)
	// 82634E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634E74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634E78: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634E7C: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82634E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634E84: 390B0070  addi r8, r11, 0x70
	ctx.r[8].s64 = ctx.r[11].s64 + 112;
	// 82634E88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82634E8C: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 82634E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634E94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634E98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634EA0: 386A8884  addi r3, r10, -0x777c
	ctx.r[3].s64 = ctx.r[10].s64 + -30588;
	// 82634EA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634EC4: 4BE31F5D  bl 0x82466e20
	ctx.lr = 0x82634EC8;
	sub_82466E20(ctx, base);
	// 82634EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634ED8 size=112
    let mut pc: u32 = 0x82634ED8;
    'dispatch: loop {
        match pc {
            0x82634ED8 => {
    //   block [0x82634ED8..0x82634F48)
	// 82634ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634EE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634EE8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634EEC: 38AA8944  addi r5, r10, -0x76bc
	ctx.r[5].s64 = ctx.r[10].s64 + -30396;
	// 82634EF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82634EF4: 390B00A0  addi r8, r11, 0xa0
	ctx.r[8].s64 = ctx.r[11].s64 + 160;
	// 82634EF8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82634EFC: 388A2464  addi r4, r10, 0x2464
	ctx.r[4].s64 = ctx.r[10].s64 + 9316;
	// 82634F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634F04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634F10: 386A88B4  addi r3, r10, -0x774c
	ctx.r[3].s64 = ctx.r[10].s64 + -30540;
	// 82634F14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634F34: 4BE31EED  bl 0x82466e20
	ctx.lr = 0x82634F38;
	sub_82466E20(ctx, base);
	// 82634F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634F48 size=108
    let mut pc: u32 = 0x82634F48;
    'dispatch: loop {
        match pc {
            0x82634F48 => {
    //   block [0x82634F48..0x82634FB4)
	// 82634F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634F54: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634F58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82634F5C: 38EB0118  addi r7, r11, 0x118
	ctx.r[7].s64 = ctx.r[11].s64 + 280;
	// 82634F60: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82634F64: 388A2484  addi r4, r10, 0x2484
	ctx.r[4].s64 = ctx.r[10].s64 + 9348;
	// 82634F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634F6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634F70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82634F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634F78: 386A88E4  addi r3, r10, -0x771c
	ctx.r[3].s64 = ctx.r[10].s64 + -30492;
	// 82634F7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82634F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634F9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82634FA0: 4BE31E81  bl 0x82466e20
	ctx.lr = 0x82634FA4;
	sub_82466E20(ctx, base);
	// 82634FA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634FB8 size=108
    let mut pc: u32 = 0x82634FB8;
    'dispatch: loop {
        match pc {
            0x82634FB8 => {
    //   block [0x82634FB8..0x82635024)
	// 82634FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634FC4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634FC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82634FCC: 38EB0160  addi r7, r11, 0x160
	ctx.r[7].s64 = ctx.r[11].s64 + 352;
	// 82634FD0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82634FD4: 388A24AC  addi r4, r10, 0x24ac
	ctx.r[4].s64 = ctx.r[10].s64 + 9388;
	// 82634FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634FDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634FE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82634FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634FE8: 386A8914  addi r3, r10, -0x76ec
	ctx.r[3].s64 = ctx.r[10].s64 + -30444;
	// 82634FEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82634FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263500C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635010: 4BE31E11  bl 0x82466e20
	ctx.lr = 0x82635014;
	sub_82466E20(ctx, base);
	// 82635014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263501C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635028 size=116
    let mut pc: u32 = 0x82635028;
    'dispatch: loop {
        match pc {
            0x82635028 => {
    //   block [0x82635028..0x8263509C)
	// 82635028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263502C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635034: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82635038: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8263503C: 390A01A8  addi r8, r10, 0x1a8
	ctx.r[8].s64 = ctx.r[10].s64 + 424;
	// 82635040: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635044: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82635048: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 8263504C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635050: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82635054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263505C: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 82635060: 396B5424  addi r11, r11, 0x5424
	ctx.r[11].s64 = ctx.r[11].s64 + 21540;
	// 82635064: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635068: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263506C: 386A8944  addi r3, r10, -0x76bc
	ctx.r[3].s64 = ctx.r[10].s64 + -30396;
	// 82635070: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82635074: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635078: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263507C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635088: 4BE31D99  bl 0x82466e20
	ctx.lr = 0x8263508C;
	sub_82466E20(ctx, base);
	// 8263508C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826350A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826350A0 size=108
    let mut pc: u32 = 0x826350A0;
    'dispatch: loop {
        match pc {
            0x826350A0 => {
    //   block [0x826350A0..0x8263510C)
	// 826350A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826350A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826350A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826350AC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826350B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826350B4: 38EB0280  addi r7, r11, 0x280
	ctx.r[7].s64 = ctx.r[11].s64 + 640;
	// 826350B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826350BC: 388A24D4  addi r4, r10, 0x24d4
	ctx.r[4].s64 = ctx.r[10].s64 + 9428;
	// 826350C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826350C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826350C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826350CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826350D0: 386A8974  addi r3, r10, -0x768c
	ctx.r[3].s64 = ctx.r[10].s64 + -30348;
	// 826350D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826350D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826350DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826350E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826350E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826350E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826350EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826350F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826350F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826350F8: 4BE31D29  bl 0x82466e20
	ctx.lr = 0x826350FC;
	sub_82466E20(ctx, base);
	// 826350FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635110 size=108
    let mut pc: u32 = 0x82635110;
    'dispatch: loop {
        match pc {
            0x82635110 => {
    //   block [0x82635110..0x8263517C)
	// 82635110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263511C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635120: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82635124: 38EB02B0  addi r7, r11, 0x2b0
	ctx.r[7].s64 = ctx.r[11].s64 + 688;
	// 82635128: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263512C: 388A24F8  addi r4, r10, 0x24f8
	ctx.r[4].s64 = ctx.r[10].s64 + 9464;
	// 82635130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635134: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263513C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635140: 386A89A4  addi r3, r10, -0x765c
	ctx.r[3].s64 = ctx.r[10].s64 + -30300;
	// 82635144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263514C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263515C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635168: 4BE31CB9  bl 0x82466e20
	ctx.lr = 0x8263516C;
	sub_82466E20(ctx, base);
	// 8263516C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635180 size=112
    let mut pc: u32 = 0x82635180;
    'dispatch: loop {
        match pc {
            0x82635180 => {
    //   block [0x82635180..0x826351F0)
	// 82635180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263518C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635190: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635194: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82635198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263519C: 390B02E0  addi r8, r11, 0x2e0
	ctx.r[8].s64 = ctx.r[11].s64 + 736;
	// 826351A0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826351A4: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826351A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826351AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826351B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826351B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826351B8: 386A89D4  addi r3, r10, -0x762c
	ctx.r[3].s64 = ctx.r[10].s64 + -30252;
	// 826351BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826351C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826351C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826351C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826351CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826351D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826351D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826351D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826351DC: 4BE31C45  bl 0x82466e20
	ctx.lr = 0x826351E0;
	sub_82466E20(ctx, base);
	// 826351E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826351E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826351E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826351EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826351F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826351F0 size=112
    let mut pc: u32 = 0x826351F0;
    'dispatch: loop {
        match pc {
            0x826351F0 => {
    //   block [0x826351F0..0x82635260)
	// 826351F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826351F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826351F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826351FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635200: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635204: 38AA7984  addi r5, r10, 0x7984
	ctx.r[5].s64 = ctx.r[10].s64 + 31108;
	// 82635208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263520C: 390B03B8  addi r8, r11, 0x3b8
	ctx.r[8].s64 = ctx.r[11].s64 + 952;
	// 82635210: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82635214: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82635218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263521C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635228: 386A8A04  addi r3, r10, -0x75fc
	ctx.r[3].s64 = ctx.r[10].s64 + -30204;
	// 8263522C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263523C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263524C: 4BE31BD5  bl 0x82466e20
	ctx.lr = 0x82635250;
	sub_82466E20(ctx, base);
	// 82635250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263525C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635260 size=112
    let mut pc: u32 = 0x82635260;
    'dispatch: loop {
        match pc {
            0x82635260 => {
    //   block [0x82635260..0x826352D0)
	// 82635260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263526C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635270: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635274: 38AA7984  addi r5, r10, 0x7984
	ctx.r[5].s64 = ctx.r[10].s64 + 31108;
	// 82635278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263527C: 390B0400  addi r8, r11, 0x400
	ctx.r[8].s64 = ctx.r[11].s64 + 1024;
	// 82635280: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82635284: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 82635288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263528C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635298: 386A8A34  addi r3, r10, -0x75cc
	ctx.r[3].s64 = ctx.r[10].s64 + -30156;
	// 8263529C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826352A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826352A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826352A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826352AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826352B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826352B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826352B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826352BC: 4BE31B65  bl 0x82466e20
	ctx.lr = 0x826352C0;
	sub_82466E20(ctx, base);
	// 826352C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826352C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826352C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826352CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826352D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826352D0 size=112
    let mut pc: u32 = 0x826352D0;
    'dispatch: loop {
        match pc {
            0x826352D0 => {
    //   block [0x826352D0..0x82635340)
	// 826352D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826352D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826352D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826352DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826352E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826352E4: 38AA79B4  addi r5, r10, 0x79b4
	ctx.r[5].s64 = ctx.r[10].s64 + 31156;
	// 826352E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826352EC: 390B0460  addi r8, r11, 0x460
	ctx.r[8].s64 = ctx.r[11].s64 + 1120;
	// 826352F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826352F4: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826352F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826352FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635308: 386A8A64  addi r3, r10, -0x759c
	ctx.r[3].s64 = ctx.r[10].s64 + -30108;
	// 8263530C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263531C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263532C: 4BE31AF5  bl 0x82466e20
	ctx.lr = 0x82635330;
	sub_82466E20(ctx, base);
	// 82635330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263533C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635340 size=112
    let mut pc: u32 = 0x82635340;
    'dispatch: loop {
        match pc {
            0x82635340 => {
    //   block [0x82635340..0x826353B0)
	// 82635340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263534C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635350: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635354: 38AA79B4  addi r5, r10, 0x79b4
	ctx.r[5].s64 = ctx.r[10].s64 + 31156;
	// 82635358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263535C: 390B04C0  addi r8, r11, 0x4c0
	ctx.r[8].s64 = ctx.r[11].s64 + 1216;
	// 82635360: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82635364: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82635368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263536C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635378: 386A8A94  addi r3, r10, -0x756c
	ctx.r[3].s64 = ctx.r[10].s64 + -30060;
	// 8263537C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263538C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263539C: 4BE31A85  bl 0x82466e20
	ctx.lr = 0x826353A0;
	sub_82466E20(ctx, base);
	// 826353A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826353A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826353A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826353AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826353B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826353B0 size=112
    let mut pc: u32 = 0x826353B0;
    'dispatch: loop {
        match pc {
            0x826353B0 => {
    //   block [0x826353B0..0x82635420)
	// 826353B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826353B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826353B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826353BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826353C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826353C4: 38AA7984  addi r5, r10, 0x7984
	ctx.r[5].s64 = ctx.r[10].s64 + 31108;
	// 826353C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826353CC: 390B0520  addi r8, r11, 0x520
	ctx.r[8].s64 = ctx.r[11].s64 + 1312;
	// 826353D0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826353D4: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826353D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826353DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826353E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826353E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826353E8: 386A8AC4  addi r3, r10, -0x753c
	ctx.r[3].s64 = ctx.r[10].s64 + -30012;
	// 826353EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826353F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826353F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826353F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826353FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263540C: 4BE31A15  bl 0x82466e20
	ctx.lr = 0x82635410;
	sub_82466E20(ctx, base);
	// 82635410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263541C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635420 size=108
    let mut pc: u32 = 0x82635420;
    'dispatch: loop {
        match pc {
            0x82635420 => {
    //   block [0x82635420..0x8263548C)
	// 82635420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263542C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635434: 38EB05E0  addi r7, r11, 0x5e0
	ctx.r[7].s64 = ctx.r[11].s64 + 1504;
	// 82635438: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8263543C: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82635440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635444: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263544C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635450: 386A8AF4  addi r3, r10, -0x750c
	ctx.r[3].s64 = ctx.r[10].s64 + -29964;
	// 82635454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263545C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263546C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635478: 4BE319A9  bl 0x82466e20
	ctx.lr = 0x8263547C;
	sub_82466E20(ctx, base);
	// 8263547C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635490 size=112
    let mut pc: u32 = 0x82635490;
    'dispatch: loop {
        match pc {
            0x82635490 => {
    //   block [0x82635490..0x82635500)
	// 82635490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263549C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826354A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826354A4: 38AA7024  addi r5, r10, 0x7024
	ctx.r[5].s64 = ctx.r[10].s64 + 28708;
	// 826354A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826354AC: 390B0778  addi r8, r11, 0x778
	ctx.r[8].s64 = ctx.r[11].s64 + 1912;
	// 826354B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826354B4: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826354B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826354BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826354C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826354C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826354C8: 386A8B24  addi r3, r10, -0x74dc
	ctx.r[3].s64 = ctx.r[10].s64 + -29916;
	// 826354CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826354D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826354D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826354D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826354DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826354E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826354E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826354E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826354EC: 4BE31935  bl 0x82466e20
	ctx.lr = 0x826354F0;
	sub_82466E20(ctx, base);
	// 826354F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826354F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826354F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826354FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635500 size=112
    let mut pc: u32 = 0x82635500;
    'dispatch: loop {
        match pc {
            0x82635500 => {
    //   block [0x82635500..0x82635570)
	// 82635500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263550C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635510: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635514: 38AA7024  addi r5, r10, 0x7024
	ctx.r[5].s64 = ctx.r[10].s64 + 28708;
	// 82635518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263551C: 390B0790  addi r8, r11, 0x790
	ctx.r[8].s64 = ctx.r[11].s64 + 1936;
	// 82635520: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82635524: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82635528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263552C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635538: 386A8B54  addi r3, r10, -0x74ac
	ctx.r[3].s64 = ctx.r[10].s64 + -29868;
	// 8263553C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263554C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82635550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263555C: 4BE318C5  bl 0x82466e20
	ctx.lr = 0x82635560;
	sub_82466E20(ctx, base);
	// 82635560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263556C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635570 size=112
    let mut pc: u32 = 0x82635570;
    'dispatch: loop {
        match pc {
            0x82635570 => {
    //   block [0x82635570..0x826355E0)
	// 82635570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263557C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635580: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635584: 38AA7024  addi r5, r10, 0x7024
	ctx.r[5].s64 = ctx.r[10].s64 + 28708;
	// 82635588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263558C: 390B07A8  addi r8, r11, 0x7a8
	ctx.r[8].s64 = ctx.r[11].s64 + 1960;
	// 82635590: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82635594: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 82635598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263559C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826355A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826355A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826355A8: 386A8B84  addi r3, r10, -0x747c
	ctx.r[3].s64 = ctx.r[10].s64 + -29820;
	// 826355AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826355B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826355B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826355B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826355BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826355C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826355C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826355C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826355CC: 4BE31855  bl 0x82466e20
	ctx.lr = 0x826355D0;
	sub_82466E20(ctx, base);
	// 826355D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826355D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826355D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826355DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826355E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826355E0 size=108
    let mut pc: u32 = 0x826355E0;
    'dispatch: loop {
        match pc {
            0x826355E0 => {
    //   block [0x826355E0..0x8263564C)
	// 826355E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826355E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826355E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826355EC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826355F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826355F4: 38EB07D8  addi r7, r11, 0x7d8
	ctx.r[7].s64 = ctx.r[11].s64 + 2008;
	// 826355F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826355FC: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82635600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635604: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263560C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635610: 386A8BB4  addi r3, r10, -0x744c
	ctx.r[3].s64 = ctx.r[10].s64 + -29772;
	// 82635614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263561C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263562C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635638: 4BE317E9  bl 0x82466e20
	ctx.lr = 0x8263563C;
	sub_82466E20(ctx, base);
	// 8263563C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635650 size=112
    let mut pc: u32 = 0x82635650;
    'dispatch: loop {
        match pc {
            0x82635650 => {
    //   block [0x82635650..0x826356C0)
	// 82635650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263565C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635660: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635664: 38AA7024  addi r5, r10, 0x7024
	ctx.r[5].s64 = ctx.r[10].s64 + 28708;
	// 82635668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263566C: 390B0808  addi r8, r11, 0x808
	ctx.r[8].s64 = ctx.r[11].s64 + 2056;
	// 82635670: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82635674: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 82635678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263567C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635688: 386A8BE4  addi r3, r10, -0x741c
	ctx.r[3].s64 = ctx.r[10].s64 + -29724;
	// 8263568C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263569C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826356A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826356A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826356A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826356AC: 4BE31775  bl 0x82466e20
	ctx.lr = 0x826356B0;
	sub_82466E20(ctx, base);
	// 826356B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826356B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826356B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826356BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826356C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826356C0 size=108
    let mut pc: u32 = 0x826356C0;
    'dispatch: loop {
        match pc {
            0x826356C0 => {
    //   block [0x826356C0..0x8263572C)
	// 826356C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826356C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826356C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826356CC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826356D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826356D4: 38EB0820  addi r7, r11, 0x820
	ctx.r[7].s64 = ctx.r[11].s64 + 2080;
	// 826356D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826356DC: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 826356E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826356E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826356E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826356EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826356F0: 386A8C14  addi r3, r10, -0x73ec
	ctx.r[3].s64 = ctx.r[10].s64 + -29676;
	// 826356F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826356F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826356FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263570C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635718: 4BE31709  bl 0x82466e20
	ctx.lr = 0x8263571C;
	sub_82466E20(ctx, base);
	// 8263571C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635730 size=108
    let mut pc: u32 = 0x82635730;
    'dispatch: loop {
        match pc {
            0x82635730 => {
    //   block [0x82635730..0x8263579C)
	// 82635730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263573C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635744: 38EB0850  addi r7, r11, 0x850
	ctx.r[7].s64 = ctx.r[11].s64 + 2128;
	// 82635748: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263574C: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 82635750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635754: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635758: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263575C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635760: 386A8C44  addi r3, r10, -0x73bc
	ctx.r[3].s64 = ctx.r[10].s64 + -29628;
	// 82635764: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263576C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263577C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635784: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635788: 4BE31699  bl 0x82466e20
	ctx.lr = 0x8263578C;
	sub_82466E20(ctx, base);
	// 8263578C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826357A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826357A0 size=112
    let mut pc: u32 = 0x826357A0;
    'dispatch: loop {
        match pc {
            0x826357A0 => {
    //   block [0x826357A0..0x82635810)
	// 826357A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826357A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826357A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826357AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826357B0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826357B4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826357B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826357BC: 390B0898  addi r8, r11, 0x898
	ctx.r[8].s64 = ctx.r[11].s64 + 2200;
	// 826357C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826357C4: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 826357C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826357CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826357D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826357D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826357D8: 386A8C74  addi r3, r10, -0x738c
	ctx.r[3].s64 = ctx.r[10].s64 + -29580;
	// 826357DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826357E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826357E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826357E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826357EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826357F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826357F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826357F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826357FC: 4BE31625  bl 0x82466e20
	ctx.lr = 0x82635800;
	sub_82466E20(ctx, base);
	// 82635800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263580C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635810 size=112
    let mut pc: u32 = 0x82635810;
    'dispatch: loop {
        match pc {
            0x82635810 => {
    //   block [0x82635810..0x82635880)
	// 82635810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263581C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635820: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635824: 38AA79B4  addi r5, r10, 0x79b4
	ctx.r[5].s64 = ctx.r[10].s64 + 31156;
	// 82635828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263582C: 390B08E0  addi r8, r11, 0x8e0
	ctx.r[8].s64 = ctx.r[11].s64 + 2272;
	// 82635830: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82635834: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82635838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263583C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635848: 386A8CA4  addi r3, r10, -0x735c
	ctx.r[3].s64 = ctx.r[10].s64 + -29532;
	// 8263584C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263585C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263586C: 4BE315B5  bl 0x82466e20
	ctx.lr = 0x82635870;
	sub_82466E20(ctx, base);
	// 82635870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263587C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635880 size=108
    let mut pc: u32 = 0x82635880;
    'dispatch: loop {
        match pc {
            0x82635880 => {
    //   block [0x82635880..0x826358EC)
	// 82635880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263588C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635894: 38EB0970  addi r7, r11, 0x970
	ctx.r[7].s64 = ctx.r[11].s64 + 2416;
	// 82635898: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263589C: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826358A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826358A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826358A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826358AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826358B0: 386A8CD4  addi r3, r10, -0x732c
	ctx.r[3].s64 = ctx.r[10].s64 + -29484;
	// 826358B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826358B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826358BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826358C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826358C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826358C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826358CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826358D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826358D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826358D8: 4BE31549  bl 0x82466e20
	ctx.lr = 0x826358DC;
	sub_82466E20(ctx, base);
	// 826358DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826358E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826358E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826358E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826358F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826358F0 size=108
    let mut pc: u32 = 0x826358F0;
    'dispatch: loop {
        match pc {
            0x826358F0 => {
    //   block [0x826358F0..0x8263595C)
	// 826358F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826358F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826358F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826358FC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635904: 38EB09B8  addi r7, r11, 0x9b8
	ctx.r[7].s64 = ctx.r[11].s64 + 2488;
	// 82635908: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263590C: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 82635910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635914: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635918: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263591C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635920: 386A8D04  addi r3, r10, -0x72fc
	ctx.r[3].s64 = ctx.r[10].s64 + -29436;
	// 82635924: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635928: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263592C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263593C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635944: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635948: 4BE314D9  bl 0x82466e20
	ctx.lr = 0x8263594C;
	sub_82466E20(ctx, base);
	// 8263594C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635960 size=108
    let mut pc: u32 = 0x82635960;
    'dispatch: loop {
        match pc {
            0x82635960 => {
    //   block [0x82635960..0x826359CC)
	// 82635960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263596C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635974: 38EB09E8  addi r7, r11, 0x9e8
	ctx.r[7].s64 = ctx.r[11].s64 + 2536;
	// 82635978: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263597C: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 82635980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635984: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635988: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263598C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635990: 386A8D34  addi r3, r10, -0x72cc
	ctx.r[3].s64 = ctx.r[10].s64 + -29388;
	// 82635994: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263599C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826359A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826359A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826359A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826359AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826359B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826359B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826359B8: 4BE31469  bl 0x82466e20
	ctx.lr = 0x826359BC;
	sub_82466E20(ctx, base);
	// 826359BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826359C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826359C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826359C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826359D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826359D0 size=112
    let mut pc: u32 = 0x826359D0;
    'dispatch: loop {
        match pc {
            0x826359D0 => {
    //   block [0x826359D0..0x82635A40)
	// 826359D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826359D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826359D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826359DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826359E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826359E4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826359E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826359EC: 390B0A18  addi r8, r11, 0xa18
	ctx.r[8].s64 = ctx.r[11].s64 + 2584;
	// 826359F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826359F4: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826359F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826359FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635A00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635A08: 386A8D64  addi r3, r10, -0x729c
	ctx.r[3].s64 = ctx.r[10].s64 + -29340;
	// 82635A0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635A2C: 4BE313F5  bl 0x82466e20
	ctx.lr = 0x82635A30;
	sub_82466E20(ctx, base);
	// 82635A30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635A40 size=112
    let mut pc: u32 = 0x82635A40;
    'dispatch: loop {
        match pc {
            0x82635A40 => {
    //   block [0x82635A40..0x82635AB0)
	// 82635A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635A4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635A50: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635A54: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82635A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635A5C: 390B0A48  addi r8, r11, 0xa48
	ctx.r[8].s64 = ctx.r[11].s64 + 2632;
	// 82635A60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82635A64: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82635A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635A6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635A70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635A78: 386A8D94  addi r3, r10, -0x726c
	ctx.r[3].s64 = ctx.r[10].s64 + -29292;
	// 82635A7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635A9C: 4BE31385  bl 0x82466e20
	ctx.lr = 0x82635AA0;
	sub_82466E20(ctx, base);
	// 82635AA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635AB0 size=112
    let mut pc: u32 = 0x82635AB0;
    'dispatch: loop {
        match pc {
            0x82635AB0 => {
    //   block [0x82635AB0..0x82635B20)
	// 82635AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635ABC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635AC0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635AC4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82635AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635ACC: 390B0A60  addi r8, r11, 0xa60
	ctx.r[8].s64 = ctx.r[11].s64 + 2656;
	// 82635AD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82635AD4: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 82635AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635ADC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635AE8: 386A8DC4  addi r3, r10, -0x723c
	ctx.r[3].s64 = ctx.r[10].s64 + -29244;
	// 82635AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635B0C: 4BE31315  bl 0x82466e20
	ctx.lr = 0x82635B10;
	sub_82466E20(ctx, base);
	// 82635B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635B20 size=108
    let mut pc: u32 = 0x82635B20;
    'dispatch: loop {
        match pc {
            0x82635B20 => {
    //   block [0x82635B20..0x82635B8C)
	// 82635B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635B2C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635B34: 38EB0A78  addi r7, r11, 0xa78
	ctx.r[7].s64 = ctx.r[11].s64 + 2680;
	// 82635B38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82635B3C: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82635B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635B44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635B48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82635B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635B50: 386A8DF4  addi r3, r10, -0x720c
	ctx.r[3].s64 = ctx.r[10].s64 + -29196;
	// 82635B54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635B74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635B78: 4BE312A9  bl 0x82466e20
	ctx.lr = 0x82635B7C;
	sub_82466E20(ctx, base);
	// 82635B7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635B90 size=112
    let mut pc: u32 = 0x82635B90;
    'dispatch: loop {
        match pc {
            0x82635B90 => {
    //   block [0x82635B90..0x82635C00)
	// 82635B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635B9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635BA0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635BA4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82635BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635BAC: 390B0AA8  addi r8, r11, 0xaa8
	ctx.r[8].s64 = ctx.r[11].s64 + 2728;
	// 82635BB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82635BB4: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 82635BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635BBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635BC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635BC8: 386A8E24  addi r3, r10, -0x71dc
	ctx.r[3].s64 = ctx.r[10].s64 + -29148;
	// 82635BCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635BEC: 4BE31235  bl 0x82466e20
	ctx.lr = 0x82635BF0;
	sub_82466E20(ctx, base);
	// 82635BF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635C00 size=108
    let mut pc: u32 = 0x82635C00;
    'dispatch: loop {
        match pc {
            0x82635C00 => {
    //   block [0x82635C00..0x82635C6C)
	// 82635C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635C0C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635C14: 38EB0AC0  addi r7, r11, 0xac0
	ctx.r[7].s64 = ctx.r[11].s64 + 2752;
	// 82635C18: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82635C1C: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 82635C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635C24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635C28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82635C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635C30: 386A8E54  addi r3, r10, -0x71ac
	ctx.r[3].s64 = ctx.r[10].s64 + -29100;
	// 82635C34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635C58: 4BE311C9  bl 0x82466e20
	ctx.lr = 0x82635C5C;
	sub_82466E20(ctx, base);
	// 82635C5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635C70 size=112
    let mut pc: u32 = 0x82635C70;
    'dispatch: loop {
        match pc {
            0x82635C70 => {
    //   block [0x82635C70..0x82635CE0)
	// 82635C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635C7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635C80: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635C84: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82635C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635C8C: 390B0B98  addi r8, r11, 0xb98
	ctx.r[8].s64 = ctx.r[11].s64 + 2968;
	// 82635C90: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 82635C94: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 82635C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635C9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635CA8: 386A8E84  addi r3, r10, -0x717c
	ctx.r[3].s64 = ctx.r[10].s64 + -29052;
	// 82635CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635CCC: 4BE31155  bl 0x82466e20
	ctx.lr = 0x82635CD0;
	sub_82466E20(ctx, base);
	// 82635CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635CE0 size=108
    let mut pc: u32 = 0x82635CE0;
    'dispatch: loop {
        match pc {
            0x82635CE0 => {
    //   block [0x82635CE0..0x82635D4C)
	// 82635CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635CEC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635CF4: 38EB0D48  addi r7, r11, 0xd48
	ctx.r[7].s64 = ctx.r[11].s64 + 3400;
	// 82635CF8: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82635CFC: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 82635D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635D04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635D08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82635D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635D10: 386A8EB4  addi r3, r10, -0x714c
	ctx.r[3].s64 = ctx.r[10].s64 + -29004;
	// 82635D14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635D24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635D34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635D38: 4BE310E9  bl 0x82466e20
	ctx.lr = 0x82635D3C;
	sub_82466E20(ctx, base);
	// 82635D3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635D50 size=112
    let mut pc: u32 = 0x82635D50;
    'dispatch: loop {
        match pc {
            0x82635D50 => {
    //   block [0x82635D50..0x82635DC0)
	// 82635D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635D5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635D60: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635D64: 38AA79B4  addi r5, r10, 0x79b4
	ctx.r[5].s64 = ctx.r[10].s64 + 31156;
	// 82635D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635D6C: 390B0EE0  addi r8, r11, 0xee0
	ctx.r[8].s64 = ctx.r[11].s64 + 3808;
	// 82635D70: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 82635D74: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 82635D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635D7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635D80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635D88: 386A8EE4  addi r3, r10, -0x711c
	ctx.r[3].s64 = ctx.r[10].s64 + -28956;
	// 82635D8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635DAC: 4BE31075  bl 0x82466e20
	ctx.lr = 0x82635DB0;
	sub_82466E20(ctx, base);
	// 82635DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635DC0 size=100
    let mut pc: u32 = 0x82635DC0;
    'dispatch: loop {
        match pc {
            0x82635DC0 => {
    //   block [0x82635DC0..0x82635E24)
	// 82635DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635DCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635DD4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82635DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635DDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635DE0: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 82635DE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635DF4: 386A8F14  addi r3, r10, -0x70ec
	ctx.r[3].s64 = ctx.r[10].s64 + -28908;
	// 82635DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635DFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635E00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82635E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635E08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82635E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635E10: 4BE31011  bl 0x82466e20
	ctx.lr = 0x82635E14;
	sub_82466E20(ctx, base);
	// 82635E14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635E28 size=112
    let mut pc: u32 = 0x82635E28;
    'dispatch: loop {
        match pc {
            0x82635E28 => {
    //   block [0x82635E28..0x82635E98)
	// 82635E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635E34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635E38: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635E3C: 38AA8F14  addi r5, r10, -0x70ec
	ctx.r[5].s64 = ctx.r[10].s64 + -28908;
	// 82635E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635E44: 390B1138  addi r8, r11, 0x1138
	ctx.r[8].s64 = ctx.r[11].s64 + 4408;
	// 82635E48: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82635E4C: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82635E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635E54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635E58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635E60: 386A8F44  addi r3, r10, -0x70bc
	ctx.r[3].s64 = ctx.r[10].s64 + -28860;
	// 82635E64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635E84: 4BE30F9D  bl 0x82466e20
	ctx.lr = 0x82635E88;
	sub_82466E20(ctx, base);
	// 82635E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635E98 size=100
    let mut pc: u32 = 0x82635E98;
    'dispatch: loop {
        match pc {
            0x82635E98 => {
    //   block [0x82635E98..0x82635EFC)
	// 82635E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635EA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635EAC: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82635EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635EB8: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 82635EBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635ECC: 386A8F74  addi r3, r10, -0x708c
	ctx.r[3].s64 = ctx.r[10].s64 + -28812;
	// 82635ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635ED4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635ED8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82635EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635EE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82635EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635EE8: 4BE30F39  bl 0x82466e20
	ctx.lr = 0x82635EEC;
	sub_82466E20(ctx, base);
	// 82635EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635F00 size=108
    let mut pc: u32 = 0x82635F00;
    'dispatch: loop {
        match pc {
            0x82635F00 => {
    //   block [0x82635F00..0x82635F6C)
	// 82635F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635F0C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635F14: 38EB11B0  addi r7, r11, 0x11b0
	ctx.r[7].s64 = ctx.r[11].s64 + 4528;
	// 82635F18: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82635F1C: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 82635F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635F24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635F28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82635F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635F30: 386A8FA4  addi r3, r10, -0x705c
	ctx.r[3].s64 = ctx.r[10].s64 + -28764;
	// 82635F34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635F58: 4BE30EC9  bl 0x82466e20
	ctx.lr = 0x82635F5C;
	sub_82466E20(ctx, base);
	// 82635F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635F70 size=112
    let mut pc: u32 = 0x82635F70;
    'dispatch: loop {
        match pc {
            0x82635F70 => {
    //   block [0x82635F70..0x82635FE0)
	// 82635F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635F7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635F80: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635F84: 38AA8F74  addi r5, r10, -0x708c
	ctx.r[5].s64 = ctx.r[10].s64 + -28812;
	// 82635F88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635F8C: 390B11F8  addi r8, r11, 0x11f8
	ctx.r[8].s64 = ctx.r[11].s64 + 4600;
	// 82635F90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82635F94: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 82635F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635F9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635FA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635FA8: 386A8FD4  addi r3, r10, -0x702c
	ctx.r[3].s64 = ctx.r[10].s64 + -28716;
	// 82635FAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635FB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635FB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635FC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635FC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635FC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635FCC: 4BE30E55  bl 0x82466e20
	ctx.lr = 0x82635FD0;
	sub_82466E20(ctx, base);
	// 82635FD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635FE0 size=100
    let mut pc: u32 = 0x82635FE0;
    'dispatch: loop {
        match pc {
            0x82635FE0 => {
    //   block [0x82635FE0..0x82636044)
	// 82635FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635FEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635FF4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82635FF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636000: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 82636004: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263600C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636014: 386A9004  addi r3, r10, -0x6ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -28668;
	// 82636018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263601C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636020: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82636024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636028: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263602C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636030: 4BE30DF1  bl 0x82466e20
	ctx.lr = 0x82636034;
	sub_82466E20(ctx, base);
	// 82636034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263603C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636048 size=100
    let mut pc: u32 = 0x82636048;
    'dispatch: loop {
        match pc {
            0x82636048 => {
    //   block [0x82636048..0x826360AC)
	// 82636048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263604C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636054: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82636058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263605C: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82636060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636068: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 8263606C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263607C: 386A9034  addi r3, r10, -0x6fcc
	ctx.r[3].s64 = ctx.r[10].s64 + -28620;
	// 82636080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636084: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636088: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263608C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636090: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82636094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636098: 4BE30D89  bl 0x82466e20
	ctx.lr = 0x8263609C;
	sub_82466E20(ctx, base);
	// 8263609C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826360A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826360A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826360A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826360B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826360B0 size=112
    let mut pc: u32 = 0x826360B0;
    'dispatch: loop {
        match pc {
            0x826360B0 => {
    //   block [0x826360B0..0x82636120)
	// 826360B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826360B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826360B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826360BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826360C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826360C4: 38AA9004  addi r5, r10, -0x6ffc
	ctx.r[5].s64 = ctx.r[10].s64 + -28668;
	// 826360C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826360CC: 390B1228  addi r8, r11, 0x1228
	ctx.r[8].s64 = ctx.r[11].s64 + 4648;
	// 826360D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826360D4: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826360D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826360DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826360E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826360E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826360E8: 386A9064  addi r3, r10, -0x6f9c
	ctx.r[3].s64 = ctx.r[10].s64 + -28572;
	// 826360EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826360F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826360F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826360F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826360FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263610C: 4BE30D15  bl 0x82466e20
	ctx.lr = 0x82636110;
	sub_82466E20(ctx, base);
	// 82636110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263611C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636120 size=112
    let mut pc: u32 = 0x82636120;
    'dispatch: loop {
        match pc {
            0x82636120 => {
    //   block [0x82636120..0x82636190)
	// 82636120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263612C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636130: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636134: 38AA9034  addi r5, r10, -0x6fcc
	ctx.r[5].s64 = ctx.r[10].s64 + -28620;
	// 82636138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263613C: 390B1288  addi r8, r11, 0x1288
	ctx.r[8].s64 = ctx.r[11].s64 + 4744;
	// 82636140: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82636144: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82636148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263614C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82636154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636158: 386A9094  addi r3, r10, -0x6f6c
	ctx.r[3].s64 = ctx.r[10].s64 + -28524;
	// 8263615C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263616C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263617C: 4BE30CA5  bl 0x82466e20
	ctx.lr = 0x82636180;
	sub_82466E20(ctx, base);
	// 82636180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263618C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636190 size=100
    let mut pc: u32 = 0x82636190;
    'dispatch: loop {
        match pc {
            0x82636190 => {
    //   block [0x82636190..0x826361F4)
	// 82636190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263619C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826361A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826361A4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826361A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826361AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826361B0: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826361B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826361B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826361BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826361C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826361C4: 386A90C4  addi r3, r10, -0x6f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -28476;
	// 826361C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826361CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826361D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826361D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826361D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826361DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826361E0: 4BE30C41  bl 0x82466e20
	ctx.lr = 0x826361E4;
	sub_82466E20(ctx, base);
	// 826361E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826361E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826361EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826361F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826361F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826361F8 size=112
    let mut pc: u32 = 0x826361F8;
    'dispatch: loop {
        match pc {
            0x826361F8 => {
    //   block [0x826361F8..0x82636268)
	// 826361F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826361FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636204: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636208: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263620C: 38AA90C4  addi r5, r10, -0x6f3c
	ctx.r[5].s64 = ctx.r[10].s64 + -28476;
	// 82636210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636214: 390B12E8  addi r8, r11, 0x12e8
	ctx.r[8].s64 = ctx.r[11].s64 + 4840;
	// 82636218: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8263621C: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 82636220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636224: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263622C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636230: 386A90F4  addi r3, r10, -0x6f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -28428;
	// 82636234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263623C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263624C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636254: 4BE30BCD  bl 0x82466e20
	ctx.lr = 0x82636258;
	sub_82466E20(ctx, base);
	// 82636258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263625C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636268 size=108
    let mut pc: u32 = 0x82636268;
    'dispatch: loop {
        match pc {
            0x82636268 => {
    //   block [0x82636268..0x826362D4)
	// 82636268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263626C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636274: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263627C: 38EB13D8  addi r7, r11, 0x13d8
	ctx.r[7].s64 = ctx.r[11].s64 + 5080;
	// 82636280: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82636284: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 82636288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263628C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82636294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636298: 386A9124  addi r3, r10, -0x6edc
	ctx.r[3].s64 = ctx.r[10].s64 + -28380;
	// 8263629C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826362A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826362A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826362A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826362AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826362B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826362B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826362B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826362BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826362C0: 4BE30B61  bl 0x82466e20
	ctx.lr = 0x826362C4;
	sub_82466E20(ctx, base);
	// 826362C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826362C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826362CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826362D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826362D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826362D8 size=108
    let mut pc: u32 = 0x826362D8;
    'dispatch: loop {
        match pc {
            0x826362D8 => {
    //   block [0x826362D8..0x82636344)
	// 826362D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826362DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826362E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826362E4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826362E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826362EC: 38EB14C8  addi r7, r11, 0x14c8
	ctx.r[7].s64 = ctx.r[11].s64 + 5320;
	// 826362F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826362F4: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826362F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826362FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82636304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636308: 386A9154  addi r3, r10, -0x6eac
	ctx.r[3].s64 = ctx.r[10].s64 + -28332;
	// 8263630C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82636310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263631C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263632C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82636330: 4BE30AF1  bl 0x82466e20
	ctx.lr = 0x82636334;
	sub_82466E20(ctx, base);
	// 82636334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263633C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636348 size=108
    let mut pc: u32 = 0x82636348;
    'dispatch: loop {
        match pc {
            0x82636348 => {
    //   block [0x82636348..0x826363B4)
	// 82636348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263634C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636354: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263635C: 38EB1510  addi r7, r11, 0x1510
	ctx.r[7].s64 = ctx.r[11].s64 + 5392;
	// 82636360: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82636364: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82636368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263636C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636370: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82636374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636378: 386A9184  addi r3, r10, -0x6e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -28284;
	// 8263637C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82636380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263638C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263639C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826363A0: 4BE30A81  bl 0x82466e20
	ctx.lr = 0x826363A4;
	sub_82466E20(ctx, base);
	// 826363A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826363A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826363AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826363B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826363B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826363B8 size=108
    let mut pc: u32 = 0x826363B8;
    'dispatch: loop {
        match pc {
            0x826363B8 => {
    //   block [0x826363B8..0x82636424)
	// 826363B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826363BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826363C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826363C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826363C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826363CC: 38EB15E8  addi r7, r11, 0x15e8
	ctx.r[7].s64 = ctx.r[11].s64 + 5608;
	// 826363D0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826363D4: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826363D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826363DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826363E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826363E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826363E8: 386A91B4  addi r3, r10, -0x6e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -28236;
	// 826363EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826363F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826363F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826363F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826363FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263640C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82636410: 4BE30A11  bl 0x82466e20
	ctx.lr = 0x82636414;
	sub_82466E20(ctx, base);
	// 82636414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263641C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636428 size=100
    let mut pc: u32 = 0x82636428;
    'dispatch: loop {
        match pc {
            0x82636428 => {
    //   block [0x82636428..0x8263648C)
	// 82636428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263642C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636434: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82636438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263643C: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82636440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636448: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8263644C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263645C: 386A91E4  addi r3, r10, -0x6e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -28188;
	// 82636460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636464: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636468: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263646C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636470: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82636474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636478: 4BE309A9  bl 0x82466e20
	ctx.lr = 0x8263647C;
	sub_82466E20(ctx, base);
	// 8263647C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636490 size=112
    let mut pc: u32 = 0x82636490;
    'dispatch: loop {
        match pc {
            0x82636490 => {
    //   block [0x82636490..0x82636500)
	// 82636490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263649C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826364A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826364A4: 38AA91E4  addi r5, r10, -0x6e1c
	ctx.r[5].s64 = ctx.r[10].s64 + -28188;
	// 826364A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826364AC: 390B1600  addi r8, r11, 0x1600
	ctx.r[8].s64 = ctx.r[11].s64 + 5632;
	// 826364B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826364B4: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826364B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826364BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826364C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826364C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826364C8: 386A9214  addi r3, r10, -0x6dec
	ctx.r[3].s64 = ctx.r[10].s64 + -28140;
	// 826364CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826364D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826364D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826364D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826364DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826364E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826364E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826364E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826364EC: 4BE30935  bl 0x82466e20
	ctx.lr = 0x826364F0;
	sub_82466E20(ctx, base);
	// 826364F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826364F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826364F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826364FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636500 size=108
    let mut pc: u32 = 0x82636500;
    'dispatch: loop {
        match pc {
            0x82636500 => {
    //   block [0x82636500..0x8263656C)
	// 82636500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263650C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636514: 38EB1648  addi r7, r11, 0x1648
	ctx.r[7].s64 = ctx.r[11].s64 + 5704;
	// 82636518: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263651C: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82636520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636524: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263652C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636530: 386A9244  addi r3, r10, -0x6dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -28092;
	// 82636534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82636538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263653C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263654C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82636558: 4BE308C9  bl 0x82466e20
	ctx.lr = 0x8263655C;
	sub_82466E20(ctx, base);
	// 8263655C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636570 size=112
    let mut pc: u32 = 0x82636570;
    'dispatch: loop {
        match pc {
            0x82636570 => {
    //   block [0x82636570..0x826365E0)
	// 82636570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263657C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82636580: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636584: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82636588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263658C: 390B1690  addi r8, r11, 0x1690
	ctx.r[8].s64 = ctx.r[11].s64 + 5776;
	// 82636590: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82636594: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 82636598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263659C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826365A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826365A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826365A8: 386A9274  addi r3, r10, -0x6d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -28044;
	// 826365AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826365B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826365B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826365B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826365BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826365C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826365C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826365C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826365CC: 4BE30855  bl 0x82466e20
	ctx.lr = 0x826365D0;
	sub_82466E20(ctx, base);
	// 826365D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826365D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826365D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826365DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826365E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826365E0 size=108
    let mut pc: u32 = 0x826365E0;
    'dispatch: loop {
        match pc {
            0x826365E0 => {
    //   block [0x826365E0..0x8263664C)
	// 826365E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826365E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826365E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826365EC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826365F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826365F4: 38EB16A8  addi r7, r11, 0x16a8
	ctx.r[7].s64 = ctx.r[11].s64 + 5800;
	// 826365F8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826365FC: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82636600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636604: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263660C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636610: 386A92A4  addi r3, r10, -0x6d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -27996;
	// 82636614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82636618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263661C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263662C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82636638: 4BE307E9  bl 0x82466e20
	ctx.lr = 0x8263663C;
	sub_82466E20(ctx, base);
	// 8263663C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636650 size=112
    let mut pc: u32 = 0x82636650;
    'dispatch: loop {
        match pc {
            0x82636650 => {
    //   block [0x82636650..0x826366C0)
	// 82636650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263665C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636660: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636664: 38AA9274  addi r5, r10, -0x6d8c
	ctx.r[5].s64 = ctx.r[10].s64 + -28044;
	// 82636668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263666C: 390B16F0  addi r8, r11, 0x16f0
	ctx.r[8].s64 = ctx.r[11].s64 + 5872;
	// 82636670: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82636674: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82636678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263667C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82636684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636688: 386A92D4  addi r3, r10, -0x6d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -27948;
	// 8263668C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263669C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826366A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826366A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826366A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826366AC: 4BE30775  bl 0x82466e20
	ctx.lr = 0x826366B0;
	sub_82466E20(ctx, base);
	// 826366B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826366B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826366B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826366BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


