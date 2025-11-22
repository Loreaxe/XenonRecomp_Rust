pub fn sub_826FDC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FDC90 size=112
    let mut pc: u32 = 0x826FDC90;
    'dispatch: loop {
        match pc {
            0x826FDC90 => {
    //   block [0x826FDC90..0x826FDD00)
	// 826FDC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FDC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FDC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FDC9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDCA0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FDCA4: 38AAD18C  addi r5, r10, -0x2e74
	ctx.r[5].s64 = ctx.r[10].s64 + -11892;
	// 826FDCA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FDCAC: 390B5728  addi r8, r11, 0x5728
	ctx.r[8].s64 = ctx.r[11].s64 + 22312;
	// 826FDCB0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FDCB4: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 826FDCB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FDCBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDCC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FDCC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FDCC8: 386AD24C  addi r3, r10, -0x2db4
	ctx.r[3].s64 = ctx.r[10].s64 + -11700;
	// 826FDCCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FDCD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FDCD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FDCD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FDCDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FDCE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FDCE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FDCE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FDCEC: 4BD69135  bl 0x82466e20
	ctx.lr = 0x826FDCF0;
	sub_82466E20(ctx, base);
	// 826FDCF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FDCF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FDCF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FDCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FDD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FDD00 size=108
    let mut pc: u32 = 0x826FDD00;
    'dispatch: loop {
        match pc {
            0x826FDD00 => {
    //   block [0x826FDD00..0x826FDD6C)
	// 826FDD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FDD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FDD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FDD0C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FDD10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FDD14: 38EB5758  addi r7, r11, 0x5758
	ctx.r[7].s64 = ctx.r[11].s64 + 22360;
	// 826FDD18: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826FDD1C: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826FDD20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FDD24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDD28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FDD2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FDD30: 386AD27C  addi r3, r10, -0x2d84
	ctx.r[3].s64 = ctx.r[10].s64 + -11652;
	// 826FDD34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FDD38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FDD3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FDD40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FDD44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FDD48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FDD4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FDD50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FDD54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FDD58: 4BD690C9  bl 0x82466e20
	ctx.lr = 0x826FDD5C;
	sub_82466E20(ctx, base);
	// 826FDD5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FDD60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FDD64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FDD68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FDD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FDD70 size=112
    let mut pc: u32 = 0x826FDD70;
    'dispatch: loop {
        match pc {
            0x826FDD70 => {
    //   block [0x826FDD70..0x826FDDE0)
	// 826FDD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FDD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FDD78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FDD7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDD80: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FDD84: 38AAD18C  addi r5, r10, -0x2e74
	ctx.r[5].s64 = ctx.r[10].s64 + -11892;
	// 826FDD88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FDD8C: 390B57A0  addi r8, r11, 0x57a0
	ctx.r[8].s64 = ctx.r[11].s64 + 22432;
	// 826FDD90: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826FDD94: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 826FDD98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FDD9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDDA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FDDA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FDDA8: 386AD2AC  addi r3, r10, -0x2d54
	ctx.r[3].s64 = ctx.r[10].s64 + -11604;
	// 826FDDAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FDDB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FDDB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FDDB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FDDBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FDDC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FDDC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FDDC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FDDCC: 4BD69055  bl 0x82466e20
	ctx.lr = 0x826FDDD0;
	sub_82466E20(ctx, base);
	// 826FDDD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FDDD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FDDD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FDDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FDDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FDDE0 size=116
    let mut pc: u32 = 0x826FDDE0;
    'dispatch: loop {
        match pc {
            0x826FDDE0 => {
    //   block [0x826FDDE0..0x826FDE54)
	// 826FDDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FDDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FDDE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FDDEC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FDDF0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FDDF4: 390B5820  addi r8, r11, 0x5820
	ctx.r[8].s64 = ctx.r[11].s64 + 22560;
	// 826FDDF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FDDFC: 392ABA84  addi r9, r10, -0x457c
	ctx.r[9].s64 = ctx.r[10].s64 + -17788;
	// 826FDE00: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDE04: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826FDE08: 38AAD18C  addi r5, r10, -0x2e74
	ctx.r[5].s64 = ctx.r[10].s64 + -11892;
	// 826FDE0C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FDE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FDE14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FDE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FDE1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FDE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FDE24: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826FDE28: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826FDE2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FDE30: 386BD2DC  addi r3, r11, -0x2d24
	ctx.r[3].s64 = ctx.r[11].s64 + -11556;
	// 826FDE34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FDE38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FDE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FDE40: 4BD68FE1  bl 0x82466e20
	ctx.lr = 0x826FDE44;
	sub_82466E20(ctx, base);
	// 826FDE44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FDE48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FDE4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FDE50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FDE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FDE58 size=100
    let mut pc: u32 = 0x826FDE58;
    'dispatch: loop {
        match pc {
            0x826FDE58 => {
    //   block [0x826FDE58..0x826FDEBC)
	// 826FDE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FDE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FDE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FDE64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FDE6C: 38AAD42C  addi r5, r10, -0x2bd4
	ctx.r[5].s64 = ctx.r[10].s64 + -11220;
	// 826FDE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FDE74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FDE78: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 826FDE7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FDE84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FDE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FDE8C: 386AD30C  addi r3, r10, -0x2cf4
	ctx.r[3].s64 = ctx.r[10].s64 + -11508;
	// 826FDE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FDE94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FDE98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FDE9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FDEA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FDEA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FDEA8: 4BD68F79  bl 0x82466e20
	ctx.lr = 0x826FDEAC;
	sub_82466E20(ctx, base);
	// 826FDEAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FDEB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FDEB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FDEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FDEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FDEC0 size=100
    let mut pc: u32 = 0x826FDEC0;
    'dispatch: loop {
        match pc {
            0x826FDEC0 => {
    //   block [0x826FDEC0..0x826FDF24)
	// 826FDEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FDEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FDEC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FDECC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FDED4: 38AACFDC  addi r5, r10, -0x3024
	ctx.r[5].s64 = ctx.r[10].s64 + -12324;
	// 826FDED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FDEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FDEE0: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826FDEE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDEE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FDEEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FDEF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FDEF4: 386AD33C  addi r3, r10, -0x2cc4
	ctx.r[3].s64 = ctx.r[10].s64 + -11460;
	// 826FDEF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FDEFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FDF00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FDF04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FDF08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FDF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FDF10: 4BD68F11  bl 0x82466e20
	ctx.lr = 0x826FDF14;
	sub_82466E20(ctx, base);
	// 826FDF14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FDF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FDF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FDF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FDF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FDF28 size=108
    let mut pc: u32 = 0x826FDF28;
    'dispatch: loop {
        match pc {
            0x826FDF28 => {
    //   block [0x826FDF28..0x826FDF94)
	// 826FDF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FDF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FDF30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FDF34: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FDF38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FDF3C: 38EB5898  addi r7, r11, 0x5898
	ctx.r[7].s64 = ctx.r[11].s64 + 22680;
	// 826FDF40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FDF44: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826FDF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FDF4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDF50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FDF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FDF58: 386AD36C  addi r3, r10, -0x2c94
	ctx.r[3].s64 = ctx.r[10].s64 + -11412;
	// 826FDF5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FDF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FDF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FDF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FDF6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FDF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FDF74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FDF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FDF7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FDF80: 4BD68EA1  bl 0x82466e20
	ctx.lr = 0x826FDF84;
	sub_82466E20(ctx, base);
	// 826FDF84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FDF88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FDF8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FDF90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FDF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FDF98 size=112
    let mut pc: u32 = 0x826FDF98;
    'dispatch: loop {
        match pc {
            0x826FDF98 => {
    //   block [0x826FDF98..0x826FE008)
	// 826FDF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FDF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FDFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FDFA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDFA8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FDFAC: 38AAD15C  addi r5, r10, -0x2ea4
	ctx.r[5].s64 = ctx.r[10].s64 + -11940;
	// 826FDFB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FDFB4: 390B58C8  addi r8, r11, 0x58c8
	ctx.r[8].s64 = ctx.r[11].s64 + 22728;
	// 826FDFB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FDFBC: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 826FDFC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FDFC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDFC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FDFCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FDFD0: 386AD39C  addi r3, r10, -0x2c64
	ctx.r[3].s64 = ctx.r[10].s64 + -11364;
	// 826FDFD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FDFD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FDFDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FDFE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FDFE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FDFE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FDFEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FDFF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FDFF4: 4BD68E2D  bl 0x82466e20
	ctx.lr = 0x826FDFF8;
	sub_82466E20(ctx, base);
	// 826FDFF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FDFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE008 size=108
    let mut pc: u32 = 0x826FE008;
    'dispatch: loop {
        match pc {
            0x826FE008 => {
    //   block [0x826FE008..0x826FE074)
	// 826FE008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE014: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE01C: 38EB58E0  addi r7, r11, 0x58e0
	ctx.r[7].s64 = ctx.r[11].s64 + 22752;
	// 826FE020: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FE024: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 826FE028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE02C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE030: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FE034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE038: 386AD3CC  addi r3, r10, -0x2c34
	ctx.r[3].s64 = ctx.r[10].s64 + -11316;
	// 826FE03C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FE040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE04C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE05C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FE060: 4BD68DC1  bl 0x82466e20
	ctx.lr = 0x826FE064;
	sub_82466E20(ctx, base);
	// 826FE064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE06C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FE078 size=28
    let mut pc: u32 = 0x826FE078;
    'dispatch: loop {
        match pc {
            0x826FE078 => {
    //   block [0x826FE078..0x826FE094)
	// 826FE078: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE07C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FE080: 394A9490  addi r10, r10, -0x6b70
	ctx.r[10].s64 = ctx.r[10].s64 + -27504;
	// 826FE084: 816B581C  lwz r11, 0x581c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22556 as u32) ) } as u64;
	// 826FE088: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826FE08C: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826FE090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE098 size=108
    let mut pc: u32 = 0x826FE098;
    'dispatch: loop {
        match pc {
            0x826FE098 => {
    //   block [0x826FE098..0x826FE104)
	// 826FE098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE0A4: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FE0A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE0AC: 38EB9490  addi r7, r11, -0x6b70
	ctx.r[7].s64 = ctx.r[11].s64 + -27504;
	// 826FE0B0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826FE0B4: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 826FE0B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE0BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE0C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FE0C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE0C8: 386AD3FC  addi r3, r10, -0x2c04
	ctx.r[3].s64 = ctx.r[10].s64 + -11268;
	// 826FE0CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FE0D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE0D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE0D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE0DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE0E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE0E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE0E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE0EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FE0F0: 4BD68D31  bl 0x82466e20
	ctx.lr = 0x826FE0F4;
	sub_82466E20(ctx, base);
	// 826FE0F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE0F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE0FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE108 size=116
    let mut pc: u32 = 0x826FE108;
    'dispatch: loop {
        match pc {
            0x826FE108 => {
    //   block [0x826FE108..0x826FE17C)
	// 826FE108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE114: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE118: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FE11C: 390B5900  addi r8, r11, 0x5900
	ctx.r[8].s64 = ctx.r[11].s64 + 22784;
	// 826FE120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE124: 392ABAD8  addi r9, r10, -0x4528
	ctx.r[9].s64 = ctx.r[10].s64 + -17704;
	// 826FE128: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE12C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826FE130: 38AAD15C  addi r5, r10, -0x2ea4
	ctx.r[5].s64 = ctx.r[10].s64 + -11940;
	// 826FE134: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE13C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE14C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826FE150: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826FE154: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FE158: 386BD42C  addi r3, r11, -0x2bd4
	ctx.r[3].s64 = ctx.r[11].s64 + -11220;
	// 826FE15C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826FE160: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE168: 4BD68CB9  bl 0x82466e20
	ctx.lr = 0x826FE16C;
	sub_82466E20(ctx, base);
	// 826FE16C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE180 size=112
    let mut pc: u32 = 0x826FE180;
    'dispatch: loop {
        match pc {
            0x826FE180 => {
    //   block [0x826FE180..0x826FE1F0)
	// 826FE180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE18C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE190: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE194: 38AAD0FC  addi r5, r10, -0x2f04
	ctx.r[5].s64 = ctx.r[10].s64 + -12036;
	// 826FE198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE19C: 390B5978  addi r8, r11, 0x5978
	ctx.r[8].s64 = ctx.r[11].s64 + 22904;
	// 826FE1A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FE1A4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 826FE1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE1AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE1B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE1B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE1B8: 386AD45C  addi r3, r10, -0x2ba4
	ctx.r[3].s64 = ctx.r[10].s64 + -11172;
	// 826FE1BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FE1C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE1C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE1C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE1CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE1D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE1D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE1D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE1DC: 4BD68C45  bl 0x82466e20
	ctx.lr = 0x826FE1E0;
	sub_82466E20(ctx, base);
	// 826FE1E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE1F0 size=108
    let mut pc: u32 = 0x826FE1F0;
    'dispatch: loop {
        match pc {
            0x826FE1F0 => {
    //   block [0x826FE1F0..0x826FE25C)
	// 826FE1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE1FC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE204: 38EB5990  addi r7, r11, 0x5990
	ctx.r[7].s64 = ctx.r[11].s64 + 22928;
	// 826FE208: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FE20C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826FE210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE214: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE218: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FE21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE220: 386AD48C  addi r3, r10, -0x2b74
	ctx.r[3].s64 = ctx.r[10].s64 + -11124;
	// 826FE224: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FE228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FE248: 4BD68BD9  bl 0x82466e20
	ctx.lr = 0x826FE24C;
	sub_82466E20(ctx, base);
	// 826FE24C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE260 size=112
    let mut pc: u32 = 0x826FE260;
    'dispatch: loop {
        match pc {
            0x826FE260 => {
    //   block [0x826FE260..0x826FE2D0)
	// 826FE260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE26C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE270: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE274: 38AACFDC  addi r5, r10, -0x3024
	ctx.r[5].s64 = ctx.r[10].s64 + -12324;
	// 826FE278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE27C: 390B59C0  addi r8, r11, 0x59c0
	ctx.r[8].s64 = ctx.r[11].s64 + 22976;
	// 826FE280: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FE284: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 826FE288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE28C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE298: 386AD4BC  addi r3, r10, -0x2b44
	ctx.r[3].s64 = ctx.r[10].s64 + -11076;
	// 826FE29C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FE2A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE2A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE2A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE2AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE2B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE2B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE2BC: 4BD68B65  bl 0x82466e20
	ctx.lr = 0x826FE2C0;
	sub_82466E20(ctx, base);
	// 826FE2C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE2D0 size=112
    let mut pc: u32 = 0x826FE2D0;
    'dispatch: loop {
        match pc {
            0x826FE2D0 => {
    //   block [0x826FE2D0..0x826FE340)
	// 826FE2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE2DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE2E0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE2E4: 38AAD63C  addi r5, r10, -0x29c4
	ctx.r[5].s64 = ctx.r[10].s64 + -10692;
	// 826FE2E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE2EC: 390B59F0  addi r8, r11, 0x59f0
	ctx.r[8].s64 = ctx.r[11].s64 + 23024;
	// 826FE2F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FE2F4: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 826FE2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE2FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE308: 386AD4EC  addi r3, r10, -0x2b14
	ctx.r[3].s64 = ctx.r[10].s64 + -11028;
	// 826FE30C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FE310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE32C: 4BD68AF5  bl 0x82466e20
	ctx.lr = 0x826FE330;
	sub_82466E20(ctx, base);
	// 826FE330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE340 size=100
    let mut pc: u32 = 0x826FE340;
    'dispatch: loop {
        match pc {
            0x826FE340 => {
    //   block [0x826FE340..0x826FE3A4)
	// 826FE340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE34C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE354: 38AACFDC  addi r5, r10, -0x3024
	ctx.r[5].s64 = ctx.r[10].s64 + -12324;
	// 826FE358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE35C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE360: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826FE364: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE36C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE374: 386AD51C  addi r3, r10, -0x2ae4
	ctx.r[3].s64 = ctx.r[10].s64 + -10980;
	// 826FE378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE37C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE380: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FE384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE388: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FE38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE390: 4BD68A91  bl 0x82466e20
	ctx.lr = 0x826FE394;
	sub_82466E20(ctx, base);
	// 826FE394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE3A8 size=112
    let mut pc: u32 = 0x826FE3A8;
    'dispatch: loop {
        match pc {
            0x826FE3A8 => {
    //   block [0x826FE3A8..0x826FE418)
	// 826FE3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE3B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE3B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE3BC: 38AAD33C  addi r5, r10, -0x2cc4
	ctx.r[5].s64 = ctx.r[10].s64 + -11460;
	// 826FE3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE3C4: 390B5A20  addi r8, r11, 0x5a20
	ctx.r[8].s64 = ctx.r[11].s64 + 23072;
	// 826FE3C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826FE3CC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 826FE3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE3D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE3D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE3E0: 386AD54C  addi r3, r10, -0x2ab4
	ctx.r[3].s64 = ctx.r[10].s64 + -10932;
	// 826FE3E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FE3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE3EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE3F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE404: 4BD68A1D  bl 0x82466e20
	ctx.lr = 0x826FE408;
	sub_82466E20(ctx, base);
	// 826FE408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE418 size=112
    let mut pc: u32 = 0x826FE418;
    'dispatch: loop {
        match pc {
            0x826FE418 => {
    //   block [0x826FE418..0x826FE488)
	// 826FE418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE424: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE428: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE42C: 38AAD33C  addi r5, r10, -0x2cc4
	ctx.r[5].s64 = ctx.r[10].s64 + -11460;
	// 826FE430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE434: 390B5A68  addi r8, r11, 0x5a68
	ctx.r[8].s64 = ctx.r[11].s64 + 23144;
	// 826FE438: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826FE43C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 826FE440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE444: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE450: 386AD57C  addi r3, r10, -0x2a84
	ctx.r[3].s64 = ctx.r[10].s64 + -10884;
	// 826FE454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FE458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE474: 4BD689AD  bl 0x82466e20
	ctx.lr = 0x826FE478;
	sub_82466E20(ctx, base);
	// 826FE478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE488 size=108
    let mut pc: u32 = 0x826FE488;
    'dispatch: loop {
        match pc {
            0x826FE488 => {
    //   block [0x826FE488..0x826FE4F4)
	// 826FE488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE494: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE49C: 38EB5B10  addi r7, r11, 0x5b10
	ctx.r[7].s64 = ctx.r[11].s64 + 23312;
	// 826FE4A0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826FE4A4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826FE4A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE4AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE4B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FE4B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE4B8: 386AD5AC  addi r3, r10, -0x2a54
	ctx.r[3].s64 = ctx.r[10].s64 + -10836;
	// 826FE4BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FE4C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE4C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE4C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE4CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE4D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE4D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE4D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE4DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FE4E0: 4BD68941  bl 0x82466e20
	ctx.lr = 0x826FE4E4;
	sub_82466E20(ctx, base);
	// 826FE4E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE4E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE4EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE4F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE4F8 size=112
    let mut pc: u32 = 0x826FE4F8;
    'dispatch: loop {
        match pc {
            0x826FE4F8 => {
    //   block [0x826FE4F8..0x826FE568)
	// 826FE4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE504: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE508: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE50C: 38AAD15C  addi r5, r10, -0x2ea4
	ctx.r[5].s64 = ctx.r[10].s64 + -11940;
	// 826FE510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE514: 390B5B58  addi r8, r11, 0x5b58
	ctx.r[8].s64 = ctx.r[11].s64 + 23384;
	// 826FE518: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826FE51C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 826FE520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE524: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE530: 386AD5DC  addi r3, r10, -0x2a24
	ctx.r[3].s64 = ctx.r[10].s64 + -10788;
	// 826FE534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FE538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE554: 4BD688CD  bl 0x82466e20
	ctx.lr = 0x826FE558;
	sub_82466E20(ctx, base);
	// 826FE558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE568 size=100
    let mut pc: u32 = 0x826FE568;
    'dispatch: loop {
        match pc {
            0x826FE568 => {
    //   block [0x826FE568..0x826FE5CC)
	// 826FE568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE574: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE57C: 38AAD18C  addi r5, r10, -0x2e74
	ctx.r[5].s64 = ctx.r[10].s64 + -11892;
	// 826FE580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE588: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 826FE58C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE59C: 386AD60C  addi r3, r10, -0x29f4
	ctx.r[3].s64 = ctx.r[10].s64 + -10740;
	// 826FE5A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE5A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE5A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FE5AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE5B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FE5B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE5B8: 4BD68869  bl 0x82466e20
	ctx.lr = 0x826FE5BC;
	sub_82466E20(ctx, base);
	// 826FE5BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE5D0 size=100
    let mut pc: u32 = 0x826FE5D0;
    'dispatch: loop {
        match pc {
            0x826FE5D0 => {
    //   block [0x826FE5D0..0x826FE634)
	// 826FE5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE5DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE5E4: 38AACFDC  addi r5, r10, -0x3024
	ctx.r[5].s64 = ctx.r[10].s64 + -12324;
	// 826FE5E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE5F0: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826FE5F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE604: 386AD63C  addi r3, r10, -0x29c4
	ctx.r[3].s64 = ctx.r[10].s64 + -10692;
	// 826FE608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE60C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE610: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FE614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE618: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FE61C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE620: 4BD68801  bl 0x82466e20
	ctx.lr = 0x826FE624;
	sub_82466E20(ctx, base);
	// 826FE624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE62C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE638 size=112
    let mut pc: u32 = 0x826FE638;
    'dispatch: loop {
        match pc {
            0x826FE638 => {
    //   block [0x826FE638..0x826FE6A8)
	// 826FE638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE644: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE648: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE64C: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 826FE650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE654: 390B5BB8  addi r8, r11, 0x5bb8
	ctx.r[8].s64 = ctx.r[11].s64 + 23480;
	// 826FE658: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826FE65C: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 826FE660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE664: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE668: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE66C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE670: 386AD66C  addi r3, r10, -0x2994
	ctx.r[3].s64 = ctx.r[10].s64 + -10644;
	// 826FE674: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FE678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE68C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE694: 4BD6878D  bl 0x82466e20
	ctx.lr = 0x826FE698;
	sub_82466E20(ctx, base);
	// 826FE698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE6A8 size=112
    let mut pc: u32 = 0x826FE6A8;
    'dispatch: loop {
        match pc {
            0x826FE6A8 => {
    //   block [0x826FE6A8..0x826FE718)
	// 826FE6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE6B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE6B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE6BC: 38AAD42C  addi r5, r10, -0x2bd4
	ctx.r[5].s64 = ctx.r[10].s64 + -11220;
	// 826FE6C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE6C4: 390B5C48  addi r8, r11, 0x5c48
	ctx.r[8].s64 = ctx.r[11].s64 + 23624;
	// 826FE6C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FE6CC: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826FE6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE6D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE6D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE6DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE6E0: 386AD69C  addi r3, r10, -0x2964
	ctx.r[3].s64 = ctx.r[10].s64 + -10596;
	// 826FE6E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FE6E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE6EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE6F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE6F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE6FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE704: 4BD6871D  bl 0x82466e20
	ctx.lr = 0x826FE708;
	sub_82466E20(ctx, base);
	// 826FE708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE718 size=112
    let mut pc: u32 = 0x826FE718;
    'dispatch: loop {
        match pc {
            0x826FE718 => {
    //   block [0x826FE718..0x826FE788)
	// 826FE718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE724: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE728: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE72C: 38AAD57C  addi r5, r10, -0x2a84
	ctx.r[5].s64 = ctx.r[10].s64 + -10884;
	// 826FE730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE734: 390B5C60  addi r8, r11, 0x5c60
	ctx.r[8].s64 = ctx.r[11].s64 + 23648;
	// 826FE738: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FE73C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 826FE740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE744: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE750: 386AD6CC  addi r3, r10, -0x2934
	ctx.r[3].s64 = ctx.r[10].s64 + -10548;
	// 826FE754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FE758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE774: 4BD686AD  bl 0x82466e20
	ctx.lr = 0x826FE778;
	sub_82466E20(ctx, base);
	// 826FE778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE788 size=112
    let mut pc: u32 = 0x826FE788;
    'dispatch: loop {
        match pc {
            0x826FE788 => {
    //   block [0x826FE788..0x826FE7F8)
	// 826FE788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE794: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE798: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE79C: 38AACFDC  addi r5, r10, -0x3024
	ctx.r[5].s64 = ctx.r[10].s64 + -12324;
	// 826FE7A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE7A4: 390B5C90  addi r8, r11, 0x5c90
	ctx.r[8].s64 = ctx.r[11].s64 + 23696;
	// 826FE7A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826FE7AC: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 826FE7B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE7B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE7B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE7BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE7C0: 386AD6FC  addi r3, r10, -0x2904
	ctx.r[3].s64 = ctx.r[10].s64 + -10500;
	// 826FE7C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FE7C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE7CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE7D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE7D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE7D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE7DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE7E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE7E4: 4BD6863D  bl 0x82466e20
	ctx.lr = 0x826FE7E8;
	sub_82466E20(ctx, base);
	// 826FE7E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE7F8 size=112
    let mut pc: u32 = 0x826FE7F8;
    'dispatch: loop {
        match pc {
            0x826FE7F8 => {
    //   block [0x826FE7F8..0x826FE868)
	// 826FE7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE804: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE808: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE80C: 38AAD18C  addi r5, r10, -0x2e74
	ctx.r[5].s64 = ctx.r[10].s64 + -11892;
	// 826FE810: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE814: 390B5CD8  addi r8, r11, 0x5cd8
	ctx.r[8].s64 = ctx.r[11].s64 + 23768;
	// 826FE818: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826FE81C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 826FE820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE824: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE828: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE830: 386AD72C  addi r3, r10, -0x28d4
	ctx.r[3].s64 = ctx.r[10].s64 + -10452;
	// 826FE834: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FE838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE83C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE84C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE854: 4BD685CD  bl 0x82466e20
	ctx.lr = 0x826FE858;
	sub_82466E20(ctx, base);
	// 826FE858: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE868 size=112
    let mut pc: u32 = 0x826FE868;
    'dispatch: loop {
        match pc {
            0x826FE868 => {
    //   block [0x826FE868..0x826FE8D8)
	// 826FE868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE874: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE878: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE87C: 38AAD0FC  addi r5, r10, -0x2f04
	ctx.r[5].s64 = ctx.r[10].s64 + -12036;
	// 826FE880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE884: 390B5D20  addi r8, r11, 0x5d20
	ctx.r[8].s64 = ctx.r[11].s64 + 23840;
	// 826FE888: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FE88C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 826FE890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE894: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE8A0: 386AD75C  addi r3, r10, -0x28a4
	ctx.r[3].s64 = ctx.r[10].s64 + -10404;
	// 826FE8A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FE8A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE8AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE8B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE8B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE8B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE8BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE8C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE8C4: 4BD6855D  bl 0x82466e20
	ctx.lr = 0x826FE8C8;
	sub_82466E20(ctx, base);
	// 826FE8C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE8D8 size=112
    let mut pc: u32 = 0x826FE8D8;
    'dispatch: loop {
        match pc {
            0x826FE8D8 => {
    //   block [0x826FE8D8..0x826FE948)
	// 826FE8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE8E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE8E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE8E8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE8EC: 38AAD15C  addi r5, r10, -0x2ea4
	ctx.r[5].s64 = ctx.r[10].s64 + -11940;
	// 826FE8F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE8F4: 390B5D38  addi r8, r11, 0x5d38
	ctx.r[8].s64 = ctx.r[11].s64 + 23864;
	// 826FE8F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FE8FC: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826FE900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE904: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE908: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE90C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE910: 386AD78C  addi r3, r10, -0x2874
	ctx.r[3].s64 = ctx.r[10].s64 + -10356;
	// 826FE914: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FE918: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FE91C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FE920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE92C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE934: 4BD684ED  bl 0x82466e20
	ctx.lr = 0x826FE938;
	sub_82466E20(ctx, base);
	// 826FE938: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FE948 size=24
    let mut pc: u32 = 0x826FE948;
    'dispatch: loop {
        match pc {
            0x826FE948 => {
    //   block [0x826FE948..0x826FE960)
	// 826FE948: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE94C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FE950: 394A95C8  addi r10, r10, -0x6a38
	ctx.r[10].s64 = ctx.r[10].s64 + -27192;
	// 826FE954: 816B58FC  lwz r11, 0x58fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22780 as u32) ) } as u64;
	// 826FE958: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826FE95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE960 size=112
    let mut pc: u32 = 0x826FE960;
    'dispatch: loop {
        match pc {
            0x826FE960 => {
    //   block [0x826FE960..0x826FE9D0)
	// 826FE960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE96C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FE970: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FE974: 392ABBF8  addi r9, r10, -0x4408
	ctx.r[9].s64 = ctx.r[10].s64 + -17416;
	// 826FE978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE97C: 390B95C8  addi r8, r11, -0x6a38
	ctx.r[8].s64 = ctx.r[11].s64 + -27192;
	// 826FE980: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826FE984: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 826FE988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE98C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FE994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FE998: 386AD7BC  addi r3, r10, -0x2844
	ctx.r[3].s64 = ctx.r[10].s64 + -10308;
	// 826FE99C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FE9A0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826FE9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FE9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FE9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FE9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FE9B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FE9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FE9BC: 4BD68465  bl 0x82466e20
	ctx.lr = 0x826FE9C0;
	sub_82466E20(ctx, base);
	// 826FE9C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FE9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FE9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FE9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FE9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FE9D0 size=112
    let mut pc: u32 = 0x826FE9D0;
    'dispatch: loop {
        match pc {
            0x826FE9D0 => {
    //   block [0x826FE9D0..0x826FEA40)
	// 826FE9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FE9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FE9D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FE9DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FE9E0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FE9E4: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FE9E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FE9EC: 390B5D6C  addi r8, r11, 0x5d6c
	ctx.r[8].s64 = ctx.r[11].s64 + 23916;
	// 826FE9F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FE9F4: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 826FE9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FE9FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEA00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FEA04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FEA08: 386AD7EC  addi r3, r10, -0x2814
	ctx.r[3].s64 = ctx.r[10].s64 + -10260;
	// 826FEA0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FEA10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FEA14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FEA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FEA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FEA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FEA24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FEA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FEA2C: 4BD683F5  bl 0x82466e20
	ctx.lr = 0x826FEA30;
	sub_82466E20(ctx, base);
	// 826FEA30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FEA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FEA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FEA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FEA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FEA40 size=108
    let mut pc: u32 = 0x826FEA40;
    'dispatch: loop {
        match pc {
            0x826FEA40 => {
    //   block [0x826FEA40..0x826FEAAC)
	// 826FEA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FEA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FEA48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FEA4C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FEA50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FEA54: 38EB5D9C  addi r7, r11, 0x5d9c
	ctx.r[7].s64 = ctx.r[11].s64 + 23964;
	// 826FEA58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FEA5C: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 826FEA60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FEA64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEA68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FEA6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FEA70: 386AD81C  addi r3, r10, -0x27e4
	ctx.r[3].s64 = ctx.r[10].s64 + -10212;
	// 826FEA74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FEA78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FEA7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FEA80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FEA84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FEA88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FEA8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FEA90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FEA94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FEA98: 4BD68389  bl 0x82466e20
	ctx.lr = 0x826FEA9C;
	sub_82466E20(ctx, base);
	// 826FEA9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FEAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FEAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FEAA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FEAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FEAB0 size=100
    let mut pc: u32 = 0x826FEAB0;
    'dispatch: loop {
        match pc {
            0x826FEAB0 => {
    //   block [0x826FEAB0..0x826FEB14)
	// 826FEAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FEAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FEAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FEABC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FEAC4: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FEAC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FEACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FEAD0: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 826FEAD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FEADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FEAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FEAE4: 386AD84C  addi r3, r10, -0x27b4
	ctx.r[3].s64 = ctx.r[10].s64 + -10164;
	// 826FEAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FEAEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FEAF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FEAF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FEAF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FEAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FEB00: 4BD68321  bl 0x82466e20
	ctx.lr = 0x826FEB04;
	sub_82466E20(ctx, base);
	// 826FEB04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FEB08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FEB0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FEB10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FEB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FEB18 size=112
    let mut pc: u32 = 0x826FEB18;
    'dispatch: loop {
        match pc {
            0x826FEB18 => {
    //   block [0x826FEB18..0x826FEB88)
	// 826FEB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FEB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FEB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FEB24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEB28: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FEB2C: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FEB30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FEB34: 390B5DB4  addi r8, r11, 0x5db4
	ctx.r[8].s64 = ctx.r[11].s64 + 23988;
	// 826FEB38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FEB3C: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 826FEB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FEB44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEB48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FEB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FEB50: 386AD87C  addi r3, r10, -0x2784
	ctx.r[3].s64 = ctx.r[10].s64 + -10116;
	// 826FEB54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FEB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FEB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FEB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FEB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FEB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FEB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FEB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FEB74: 4BD682AD  bl 0x82466e20
	ctx.lr = 0x826FEB78;
	sub_82466E20(ctx, base);
	// 826FEB78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FEB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FEB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FEB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FEB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FEB88 size=112
    let mut pc: u32 = 0x826FEB88;
    'dispatch: loop {
        match pc {
            0x826FEB88 => {
    //   block [0x826FEB88..0x826FEBF8)
	// 826FEB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FEB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FEB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FEB94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEB98: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FEB9C: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FEBA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FEBA4: 390B5DCC  addi r8, r11, 0x5dcc
	ctx.r[8].s64 = ctx.r[11].s64 + 24012;
	// 826FEBA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FEBAC: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 826FEBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FEBB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEBB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FEBBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FEBC0: 386AD8AC  addi r3, r10, -0x2754
	ctx.r[3].s64 = ctx.r[10].s64 + -10068;
	// 826FEBC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FEBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FEBCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FEBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FEBD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FEBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FEBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FEBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FEBE4: 4BD6823D  bl 0x82466e20
	ctx.lr = 0x826FEBE8;
	sub_82466E20(ctx, base);
	// 826FEBE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FEBEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FEBF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FEBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FEBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FEBF8 size=112
    let mut pc: u32 = 0x826FEBF8;
    'dispatch: loop {
        match pc {
            0x826FEBF8 => {
    //   block [0x826FEBF8..0x826FEC68)
	// 826FEBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FEBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FEC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FEC04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEC08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FEC0C: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FEC10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FEC14: 390B5DFC  addi r8, r11, 0x5dfc
	ctx.r[8].s64 = ctx.r[11].s64 + 24060;
	// 826FEC18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FEC1C: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 826FEC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FEC24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEC28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FEC2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FEC30: 386AD8DC  addi r3, r10, -0x2724
	ctx.r[3].s64 = ctx.r[10].s64 + -10020;
	// 826FEC34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FEC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FEC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FEC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FEC44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FEC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FEC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FEC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FEC54: 4BD681CD  bl 0x82466e20
	ctx.lr = 0x826FEC58;
	sub_82466E20(ctx, base);
	// 826FEC58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FEC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FEC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FEC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FEC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FEC68 size=112
    let mut pc: u32 = 0x826FEC68;
    'dispatch: loop {
        match pc {
            0x826FEC68 => {
    //   block [0x826FEC68..0x826FECD8)
	// 826FEC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FEC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FEC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FEC74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEC78: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FEC7C: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FEC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FEC84: 390B5E2C  addi r8, r11, 0x5e2c
	ctx.r[8].s64 = ctx.r[11].s64 + 24108;
	// 826FEC88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FEC8C: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 826FEC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FEC94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEC98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FEC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FECA0: 386AD90C  addi r3, r10, -0x26f4
	ctx.r[3].s64 = ctx.r[10].s64 + -9972;
	// 826FECA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FECA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FECAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FECB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FECB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FECB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FECBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FECC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FECC4: 4BD6815D  bl 0x82466e20
	ctx.lr = 0x826FECC8;
	sub_82466E20(ctx, base);
	// 826FECC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FECCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FECD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FECD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FECD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FECD8 size=112
    let mut pc: u32 = 0x826FECD8;
    'dispatch: loop {
        match pc {
            0x826FECD8 => {
    //   block [0x826FECD8..0x826FED48)
	// 826FECD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FECDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FECE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FECE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FECE8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FECEC: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FECF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FECF4: 390B5E5C  addi r8, r11, 0x5e5c
	ctx.r[8].s64 = ctx.r[11].s64 + 24156;
	// 826FECF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FECFC: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 826FED00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FED04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FED08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FED0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FED10: 386AD93C  addi r3, r10, -0x26c4
	ctx.r[3].s64 = ctx.r[10].s64 + -9924;
	// 826FED14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FED18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FED1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FED20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FED24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FED28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FED2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FED30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FED34: 4BD680ED  bl 0x82466e20
	ctx.lr = 0x826FED38;
	sub_82466E20(ctx, base);
	// 826FED38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FED3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FED40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FED44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FED48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FED48 size=112
    let mut pc: u32 = 0x826FED48;
    'dispatch: loop {
        match pc {
            0x826FED48 => {
    //   block [0x826FED48..0x826FEDB8)
	// 826FED48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FED4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FED50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FED54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FED58: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FED5C: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FED60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FED64: 390B5E74  addi r8, r11, 0x5e74
	ctx.r[8].s64 = ctx.r[11].s64 + 24180;
	// 826FED68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FED6C: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 826FED70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FED74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FED78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FED7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FED80: 386AD96C  addi r3, r10, -0x2694
	ctx.r[3].s64 = ctx.r[10].s64 + -9876;
	// 826FED84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FED88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FED8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FED90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FED94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FED98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FED9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FEDA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FEDA4: 4BD6807D  bl 0x82466e20
	ctx.lr = 0x826FEDA8;
	sub_82466E20(ctx, base);
	// 826FEDA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FEDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FEDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FEDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FEDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FEDB8 size=112
    let mut pc: u32 = 0x826FEDB8;
    'dispatch: loop {
        match pc {
            0x826FEDB8 => {
    //   block [0x826FEDB8..0x826FEE28)
	// 826FEDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FEDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FEDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FEDC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEDC8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FEDCC: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FEDD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FEDD4: 390B5E90  addi r8, r11, 0x5e90
	ctx.r[8].s64 = ctx.r[11].s64 + 24208;
	// 826FEDD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826FEDDC: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 826FEDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FEDE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEDE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FEDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FEDF0: 386AD99C  addi r3, r10, -0x2664
	ctx.r[3].s64 = ctx.r[10].s64 + -9828;
	// 826FEDF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FEDF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FEDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FEE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FEE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FEE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FEE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FEE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FEE14: 4BD6800D  bl 0x82466e20
	ctx.lr = 0x826FEE18;
	sub_82466E20(ctx, base);
	// 826FEE18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FEE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FEE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FEE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FEE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FEE28 size=112
    let mut pc: u32 = 0x826FEE28;
    'dispatch: loop {
        match pc {
            0x826FEE28 => {
    //   block [0x826FEE28..0x826FEE98)
	// 826FEE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FEE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FEE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FEE34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEE38: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FEE3C: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FEE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FEE44: 390B5ED8  addi r8, r11, 0x5ed8
	ctx.r[8].s64 = ctx.r[11].s64 + 24280;
	// 826FEE48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826FEE4C: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 826FEE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FEE54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEE58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FEE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FEE60: 386AD9CC  addi r3, r10, -0x2634
	ctx.r[3].s64 = ctx.r[10].s64 + -9780;
	// 826FEE64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FEE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FEE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FEE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FEE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FEE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FEE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FEE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FEE84: 4BD67F9D  bl 0x82466e20
	ctx.lr = 0x826FEE88;
	sub_82466E20(ctx, base);
	// 826FEE88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FEE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FEE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FEE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FEE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FEE98 size=112
    let mut pc: u32 = 0x826FEE98;
    'dispatch: loop {
        match pc {
            0x826FEE98 => {
    //   block [0x826FEE98..0x826FEF08)
	// 826FEE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FEE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FEEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FEEA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEEA8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FEEAC: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FEEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FEEB4: 390B5F20  addi r8, r11, 0x5f20
	ctx.r[8].s64 = ctx.r[11].s64 + 24352;
	// 826FEEB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FEEBC: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 826FEEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FEEC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FEECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FEED0: 386AD9FC  addi r3, r10, -0x2604
	ctx.r[3].s64 = ctx.r[10].s64 + -9732;
	// 826FEED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FEED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FEEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FEEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FEEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FEEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FEEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FEEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FEEF4: 4BD67F2D  bl 0x82466e20
	ctx.lr = 0x826FEEF8;
	sub_82466E20(ctx, base);
	// 826FEEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FEEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FEF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FEF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FEF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FEF08 size=112
    let mut pc: u32 = 0x826FEF08;
    'dispatch: loop {
        match pc {
            0x826FEF08 => {
    //   block [0x826FEF08..0x826FEF78)
	// 826FEF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FEF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FEF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FEF14: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEF18: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FEF1C: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FEF20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FEF24: 390B5F38  addi r8, r11, 0x5f38
	ctx.r[8].s64 = ctx.r[11].s64 + 24376;
	// 826FEF28: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FEF2C: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 826FEF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FEF34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEF38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FEF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FEF40: 386ADA2C  addi r3, r10, -0x25d4
	ctx.r[3].s64 = ctx.r[10].s64 + -9684;
	// 826FEF44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FEF48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FEF4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FEF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FEF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FEF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FEF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FEF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FEF64: 4BD67EBD  bl 0x82466e20
	ctx.lr = 0x826FEF68;
	sub_82466E20(ctx, base);
	// 826FEF68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FEF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FEF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FEF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FEF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FEF78 size=116
    let mut pc: u32 = 0x826FEF78;
    'dispatch: loop {
        match pc {
            0x826FEF78 => {
    //   block [0x826FEF78..0x826FEFEC)
	// 826FEF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FEF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FEF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FEF84: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826FEF88: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826FEF8C: 390A5F68  addi r8, r10, 0x5f68
	ctx.r[8].s64 = ctx.r[10].s64 + 24424;
	// 826FEF90: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEF94: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826FEF98: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FEF9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FEFA0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FEFA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FEFA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FEFAC: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 826FEFB0: 396BBC20  addi r11, r11, -0x43e0
	ctx.r[11].s64 = ctx.r[11].s64 + -17376;
	// 826FEFB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FEFB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FEFBC: 386ADA5C  addi r3, r10, -0x25a4
	ctx.r[3].s64 = ctx.r[10].s64 + -9636;
	// 826FEFC0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826FEFC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FEFC8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826FEFCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FEFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FEFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FEFD8: 4BD67E49  bl 0x82466e20
	ctx.lr = 0x826FEFDC;
	sub_82466E20(ctx, base);
	// 826FEFDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FEFE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FEFE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FEFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FEFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FEFF0 size=116
    let mut pc: u32 = 0x826FEFF0;
    'dispatch: loop {
        match pc {
            0x826FEFF0 => {
    //   block [0x826FEFF0..0x826FF064)
	// 826FEFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FEFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FEFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FEFFC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826FF000: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826FF004: 390A5FE0  addi r8, r10, 0x5fe0
	ctx.r[8].s64 = ctx.r[10].s64 + 24544;
	// 826FF008: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF00C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826FF010: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FF014: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF018: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FF01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF024: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 826FF028: 396BBC38  addi r11, r11, -0x43c8
	ctx.r[11].s64 = ctx.r[11].s64 + -17352;
	// 826FF02C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF030: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF034: 386ADA8C  addi r3, r10, -0x2574
	ctx.r[3].s64 = ctx.r[10].s64 + -9588;
	// 826FF038: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826FF03C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF040: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826FF044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF04C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF050: 4BD67DD1  bl 0x82466e20
	ctx.lr = 0x826FF054;
	sub_82466E20(ctx, base);
	// 826FF054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FF068 size=24
    let mut pc: u32 = 0x826FF068;
    'dispatch: loop {
        match pc {
            0x826FF068 => {
    //   block [0x826FF068..0x826FF080)
	// 826FF068: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF06C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FF070: 394A95E0  addi r10, r10, -0x6a20
	ctx.r[10].s64 = ctx.r[10].s64 + -27168;
	// 826FF074: 816B5E8C  lwz r11, 0x5e8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24204 as u32) ) } as u64;
	// 826FF078: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826FF07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF080 size=116
    let mut pc: u32 = 0x826FF080;
    'dispatch: loop {
        match pc {
            0x826FF080 => {
    //   block [0x826FF080..0x826FF0F4)
	// 826FF080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF08C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826FF090: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF094: 392BBC64  addi r9, r11, -0x439c
	ctx.r[9].s64 = ctx.r[11].s64 + -17308;
	// 826FF098: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FF09C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF0A0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826FF0A4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826FF0A8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FF0AC: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 826FF0B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF0B4: 396B95E0  addi r11, r11, -0x6a20
	ctx.r[11].s64 = ctx.r[11].s64 + -27168;
	// 826FF0B8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826FF0BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF0C0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826FF0C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF0C8: 386ADABC  addi r3, r10, -0x2544
	ctx.r[3].s64 = ctx.r[10].s64 + -9540;
	// 826FF0CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FF0D0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826FF0D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF0D8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826FF0DC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FF0E0: 4BD67D41  bl 0x82466e20
	ctx.lr = 0x826FF0E4;
	sub_82466E20(ctx, base);
	// 826FF0E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF0E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF0EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF0F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF0F8 size=112
    let mut pc: u32 = 0x826FF0F8;
    'dispatch: loop {
        match pc {
            0x826FF0F8 => {
    //   block [0x826FF0F8..0x826FF168)
	// 826FF0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF104: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF108: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF10C: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FF110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF114: 390B6070  addi r8, r11, 0x6070
	ctx.r[8].s64 = ctx.r[11].s64 + 24688;
	// 826FF118: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826FF11C: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 826FF120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF124: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF128: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF12C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF130: 386ADAEC  addi r3, r10, -0x2514
	ctx.r[3].s64 = ctx.r[10].s64 + -9492;
	// 826FF134: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FF138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF13C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF14C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF154: 4BD67CCD  bl 0x82466e20
	ctx.lr = 0x826FF158;
	sub_82466E20(ctx, base);
	// 826FF158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF168 size=112
    let mut pc: u32 = 0x826FF168;
    'dispatch: loop {
        match pc {
            0x826FF168 => {
    //   block [0x826FF168..0x826FF1D8)
	// 826FF168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF174: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF178: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF17C: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FF180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF184: 390B60D0  addi r8, r11, 0x60d0
	ctx.r[8].s64 = ctx.r[11].s64 + 24784;
	// 826FF188: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826FF18C: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 826FF190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF194: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF19C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF1A0: 386ADB1C  addi r3, r10, -0x24e4
	ctx.r[3].s64 = ctx.r[10].s64 + -9444;
	// 826FF1A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FF1A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF1B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF1B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF1B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF1BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF1C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF1C4: 4BD67C5D  bl 0x82466e20
	ctx.lr = 0x826FF1C8;
	sub_82466E20(ctx, base);
	// 826FF1C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF1D8 size=112
    let mut pc: u32 = 0x826FF1D8;
    'dispatch: loop {
        match pc {
            0x826FF1D8 => {
    //   block [0x826FF1D8..0x826FF248)
	// 826FF1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF1E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF1E8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF1EC: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FF1F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF1F4: 390B6178  addi r8, r11, 0x6178
	ctx.r[8].s64 = ctx.r[11].s64 + 24952;
	// 826FF1F8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826FF1FC: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 826FF200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF204: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF208: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF20C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF210: 386ADB4C  addi r3, r10, -0x24b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9396;
	// 826FF214: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FF218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF22C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF234: 4BD67BED  bl 0x82466e20
	ctx.lr = 0x826FF238;
	sub_82466E20(ctx, base);
	// 826FF238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF248 size=112
    let mut pc: u32 = 0x826FF248;
    'dispatch: loop {
        match pc {
            0x826FF248 => {
    //   block [0x826FF248..0x826FF2B8)
	// 826FF248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF254: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF258: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF25C: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FF260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF264: 390B61F0  addi r8, r11, 0x61f0
	ctx.r[8].s64 = ctx.r[11].s64 + 25072;
	// 826FF268: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826FF26C: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 826FF270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF274: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF27C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF280: 386ADB7C  addi r3, r10, -0x2484
	ctx.r[3].s64 = ctx.r[10].s64 + -9348;
	// 826FF284: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FF288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF2A4: 4BD67B7D  bl 0x82466e20
	ctx.lr = 0x826FF2A8;
	sub_82466E20(ctx, base);
	// 826FF2A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF2B8 size=112
    let mut pc: u32 = 0x826FF2B8;
    'dispatch: loop {
        match pc {
            0x826FF2B8 => {
    //   block [0x826FF2B8..0x826FF328)
	// 826FF2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF2C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF2C8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF2CC: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FF2D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF2D4: 390B6238  addi r8, r11, 0x6238
	ctx.r[8].s64 = ctx.r[11].s64 + 25144;
	// 826FF2D8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826FF2DC: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 826FF2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF2E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF2E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF2F0: 386ADBAC  addi r3, r10, -0x2454
	ctx.r[3].s64 = ctx.r[10].s64 + -9300;
	// 826FF2F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FF2F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF2FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF30C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF314: 4BD67B0D  bl 0x82466e20
	ctx.lr = 0x826FF318;
	sub_82466E20(ctx, base);
	// 826FF318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF31C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF328 size=112
    let mut pc: u32 = 0x826FF328;
    'dispatch: loop {
        match pc {
            0x826FF328 => {
    //   block [0x826FF328..0x826FF398)
	// 826FF328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF334: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF338: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF33C: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FF340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF344: 390B62C8  addi r8, r11, 0x62c8
	ctx.r[8].s64 = ctx.r[11].s64 + 25288;
	// 826FF348: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826FF34C: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 826FF350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF354: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF35C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF360: 386ADBDC  addi r3, r10, -0x2424
	ctx.r[3].s64 = ctx.r[10].s64 + -9252;
	// 826FF364: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FF368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF384: 4BD67A9D  bl 0x82466e20
	ctx.lr = 0x826FF388;
	sub_82466E20(ctx, base);
	// 826FF388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF398 size=112
    let mut pc: u32 = 0x826FF398;
    'dispatch: loop {
        match pc {
            0x826FF398 => {
    //   block [0x826FF398..0x826FF408)
	// 826FF398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF3A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF3A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF3AC: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826FF3B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF3B4: 390B6328  addi r8, r11, 0x6328
	ctx.r[8].s64 = ctx.r[11].s64 + 25384;
	// 826FF3B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826FF3BC: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 826FF3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF3C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF3C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF3D0: 386ADC0C  addi r3, r10, -0x23f4
	ctx.r[3].s64 = ctx.r[10].s64 + -9204;
	// 826FF3D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FF3D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF3DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF3E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF3E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF3E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF3F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF3F4: 4BD67A2D  bl 0x82466e20
	ctx.lr = 0x826FF3F8;
	sub_82466E20(ctx, base);
	// 826FF3F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF408 size=112
    let mut pc: u32 = 0x826FF408;
    'dispatch: loop {
        match pc {
            0x826FF408 => {
    //   block [0x826FF408..0x826FF478)
	// 826FF408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF414: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF418: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF41C: 38AADC0C  addi r5, r10, -0x23f4
	ctx.r[5].s64 = ctx.r[10].s64 + -9204;
	// 826FF420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF424: 390B6370  addi r8, r11, 0x6370
	ctx.r[8].s64 = ctx.r[11].s64 + 25456;
	// 826FF428: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FF42C: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 826FF430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF434: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF440: 386ADC3C  addi r3, r10, -0x23c4
	ctx.r[3].s64 = ctx.r[10].s64 + -9156;
	// 826FF444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FF448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF464: 4BD679BD  bl 0x82466e20
	ctx.lr = 0x826FF468;
	sub_82466E20(ctx, base);
	// 826FF468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF478 size=112
    let mut pc: u32 = 0x826FF478;
    'dispatch: loop {
        match pc {
            0x826FF478 => {
    //   block [0x826FF478..0x826FF4E8)
	// 826FF478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF484: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF488: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF48C: 38AADC0C  addi r5, r10, -0x23f4
	ctx.r[5].s64 = ctx.r[10].s64 + -9204;
	// 826FF490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF494: 390B63A0  addi r8, r11, 0x63a0
	ctx.r[8].s64 = ctx.r[11].s64 + 25504;
	// 826FF498: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FF49C: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 826FF4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF4A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF4A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF4AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF4B0: 386ADC6C  addi r3, r10, -0x2394
	ctx.r[3].s64 = ctx.r[10].s64 + -9108;
	// 826FF4B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FF4B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF4C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF4C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF4C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF4CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF4D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF4D4: 4BD6794D  bl 0x82466e20
	ctx.lr = 0x826FF4D8;
	sub_82466E20(ctx, base);
	// 826FF4D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF4DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF4E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF4E8 size=100
    let mut pc: u32 = 0x826FF4E8;
    'dispatch: loop {
        match pc {
            0x826FF4E8 => {
    //   block [0x826FF4E8..0x826FF54C)
	// 826FF4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF4F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF4F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF4FC: 38AADC0C  addi r5, r10, -0x23f4
	ctx.r[5].s64 = ctx.r[10].s64 + -9204;
	// 826FF500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF508: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 826FF50C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF51C: 386ADC9C  addi r3, r10, -0x2364
	ctx.r[3].s64 = ctx.r[10].s64 + -9060;
	// 826FF520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF524: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF528: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FF52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF530: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FF534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF538: 4BD678E9  bl 0x82466e20
	ctx.lr = 0x826FF53C;
	sub_82466E20(ctx, base);
	// 826FF53C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF550 size=112
    let mut pc: u32 = 0x826FF550;
    'dispatch: loop {
        match pc {
            0x826FF550 => {
    //   block [0x826FF550..0x826FF5C0)
	// 826FF550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF55C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF560: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF564: 38AADC0C  addi r5, r10, -0x23f4
	ctx.r[5].s64 = ctx.r[10].s64 + -9204;
	// 826FF568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF56C: 390B63D0  addi r8, r11, 0x63d0
	ctx.r[8].s64 = ctx.r[11].s64 + 25552;
	// 826FF570: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FF574: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 826FF578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF57C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF588: 386ADCCC  addi r3, r10, -0x2334
	ctx.r[3].s64 = ctx.r[10].s64 + -9012;
	// 826FF58C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FF590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF59C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF5A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF5A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF5A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF5AC: 4BD67875  bl 0x82466e20
	ctx.lr = 0x826FF5B0;
	sub_82466E20(ctx, base);
	// 826FF5B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF5C0 size=112
    let mut pc: u32 = 0x826FF5C0;
    'dispatch: loop {
        match pc {
            0x826FF5C0 => {
    //   block [0x826FF5C0..0x826FF630)
	// 826FF5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF5CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF5D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF5D4: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 826FF5D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF5DC: 390B63E8  addi r8, r11, 0x63e8
	ctx.r[8].s64 = ctx.r[11].s64 + 25576;
	// 826FF5E0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826FF5E4: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 826FF5E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF5EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF5F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF5F8: 386ADCFC  addi r3, r10, -0x2304
	ctx.r[3].s64 = ctx.r[10].s64 + -8964;
	// 826FF5FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FF600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF60C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF61C: 4BD67805  bl 0x82466e20
	ctx.lr = 0x826FF620;
	sub_82466E20(ctx, base);
	// 826FF620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF630 size=112
    let mut pc: u32 = 0x826FF630;
    'dispatch: loop {
        match pc {
            0x826FF630 => {
    //   block [0x826FF630..0x826FF6A0)
	// 826FF630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF63C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF640: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF644: 38AADCFC  addi r5, r10, -0x2304
	ctx.r[5].s64 = ctx.r[10].s64 + -8964;
	// 826FF648: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF64C: 390B6448  addi r8, r11, 0x6448
	ctx.r[8].s64 = ctx.r[11].s64 + 25672;
	// 826FF650: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FF654: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 826FF658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF65C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF660: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF668: 386ADD2C  addi r3, r10, -0x22d4
	ctx.r[3].s64 = ctx.r[10].s64 + -8916;
	// 826FF66C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FF670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF67C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF68C: 4BD67795  bl 0x82466e20
	ctx.lr = 0x826FF690;
	sub_82466E20(ctx, base);
	// 826FF690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF69C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF6A0 size=112
    let mut pc: u32 = 0x826FF6A0;
    'dispatch: loop {
        match pc {
            0x826FF6A0 => {
    //   block [0x826FF6A0..0x826FF710)
	// 826FF6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF6AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF6B0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF6B4: 38AADCFC  addi r5, r10, -0x2304
	ctx.r[5].s64 = ctx.r[10].s64 + -8964;
	// 826FF6B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF6BC: 390B6460  addi r8, r11, 0x6460
	ctx.r[8].s64 = ctx.r[11].s64 + 25696;
	// 826FF6C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FF6C4: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826FF6C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF6CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF6D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF6D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF6D8: 386ADD5C  addi r3, r10, -0x22a4
	ctx.r[3].s64 = ctx.r[10].s64 + -8868;
	// 826FF6DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FF6E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF6E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF6F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF6F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF6F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF6FC: 4BD67725  bl 0x82466e20
	ctx.lr = 0x826FF700;
	sub_82466E20(ctx, base);
	// 826FF700: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF70C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF710 size=112
    let mut pc: u32 = 0x826FF710;
    'dispatch: loop {
        match pc {
            0x826FF710 => {
    //   block [0x826FF710..0x826FF780)
	// 826FF710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF71C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF720: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF724: 38AADCFC  addi r5, r10, -0x2304
	ctx.r[5].s64 = ctx.r[10].s64 + -8964;
	// 826FF728: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF72C: 390B6490  addi r8, r11, 0x6490
	ctx.r[8].s64 = ctx.r[11].s64 + 25744;
	// 826FF730: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FF734: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 826FF738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF73C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF740: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF748: 386ADD8C  addi r3, r10, -0x2274
	ctx.r[3].s64 = ctx.r[10].s64 + -8820;
	// 826FF74C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FF750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF75C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF76C: 4BD676B5  bl 0x82466e20
	ctx.lr = 0x826FF770;
	sub_82466E20(ctx, base);
	// 826FF770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FF780 size=24
    let mut pc: u32 = 0x826FF780;
    'dispatch: loop {
        match pc {
            0x826FF780 => {
    //   block [0x826FF780..0x826FF798)
	// 826FF780: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF784: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FF788: 394A9688  addi r10, r10, -0x6978
	ctx.r[10].s64 = ctx.r[10].s64 + -27000;
	// 826FF78C: 816B64A8  lwz r11, 0x64a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25768 as u32) ) } as u64;
	// 826FF790: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826FF794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF798 size=112
    let mut pc: u32 = 0x826FF798;
    'dispatch: loop {
        match pc {
            0x826FF798 => {
    //   block [0x826FF798..0x826FF808)
	// 826FF798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF7A4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FF7A8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FF7AC: 392ABCC0  addi r9, r10, -0x4340
	ctx.r[9].s64 = ctx.r[10].s64 + -17216;
	// 826FF7B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF7B4: 390B9688  addi r8, r11, -0x6978
	ctx.r[8].s64 = ctx.r[11].s64 + -27000;
	// 826FF7B8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826FF7BC: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826FF7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF7C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF7C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF7D0: 386ADDBC  addi r3, r10, -0x2244
	ctx.r[3].s64 = ctx.r[10].s64 + -8772;
	// 826FF7D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FF7D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FF7DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF7E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF7E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF7E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF7EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FF7F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF7F4: 4BD6762D  bl 0x82466e20
	ctx.lr = 0x826FF7F8;
	sub_82466E20(ctx, base);
	// 826FF7F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF7FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF808 size=108
    let mut pc: u32 = 0x826FF808;
    'dispatch: loop {
        match pc {
            0x826FF808 => {
    //   block [0x826FF808..0x826FF874)
	// 826FF808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF814: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF81C: 38EB64AC  addi r7, r11, 0x64ac
	ctx.r[7].s64 = ctx.r[11].s64 + 25772;
	// 826FF820: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FF824: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 826FF828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF82C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF830: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FF834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF838: 386ADDEC  addi r3, r10, -0x2214
	ctx.r[3].s64 = ctx.r[10].s64 + -8724;
	// 826FF83C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FF840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF85C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FF860: 4BD675C1  bl 0x82466e20
	ctx.lr = 0x826FF864;
	sub_82466E20(ctx, base);
	// 826FF864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF86C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF878 size=108
    let mut pc: u32 = 0x826FF878;
    'dispatch: loop {
        match pc {
            0x826FF878 => {
    //   block [0x826FF878..0x826FF8E4)
	// 826FF878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF884: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF88C: 38EB64C8  addi r7, r11, 0x64c8
	ctx.r[7].s64 = ctx.r[11].s64 + 25800;
	// 826FF890: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826FF894: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826FF898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF89C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF8A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FF8A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF8A8: 386ADE1C  addi r3, r10, -0x21e4
	ctx.r[3].s64 = ctx.r[10].s64 + -8676;
	// 826FF8AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FF8B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF8B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF8B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF8BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF8C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF8C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF8C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF8CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FF8D0: 4BD67551  bl 0x82466e20
	ctx.lr = 0x826FF8D4;
	sub_82466E20(ctx, base);
	// 826FF8D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF8E8 size=116
    let mut pc: u32 = 0x826FF8E8;
    'dispatch: loop {
        match pc {
            0x826FF8E8 => {
    //   block [0x826FF8E8..0x826FF95C)
	// 826FF8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF8F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF8F8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FF8FC: 390B6510  addi r8, r11, 0x6510
	ctx.r[8].s64 = ctx.r[11].s64 + 25872;
	// 826FF900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF904: 392ABD78  addi r9, r10, -0x4288
	ctx.r[9].s64 = ctx.r[10].s64 + -17032;
	// 826FF908: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF90C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826FF910: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 826FF914: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FF918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF91C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FF920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF92C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826FF930: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 826FF934: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FF938: 386BDE4C  addi r3, r11, -0x21b4
	ctx.r[3].s64 = ctx.r[11].s64 + -8628;
	// 826FF93C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FF940: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF948: 4BD674D9  bl 0x82466e20
	ctx.lr = 0x826FF94C;
	sub_82466E20(ctx, base);
	// 826FF94C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF960 size=108
    let mut pc: u32 = 0x826FF960;
    'dispatch: loop {
        match pc {
            0x826FF960 => {
    //   block [0x826FF960..0x826FF9CC)
	// 826FF960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF96C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF970: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826FF974: 38EB6528  addi r7, r11, 0x6528
	ctx.r[7].s64 = ctx.r[11].s64 + 25896;
	// 826FF978: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826FF97C: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 826FF980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FF984: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FF988: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FF98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FF990: 386ADE7C  addi r3, r10, -0x2184
	ctx.r[3].s64 = ctx.r[10].s64 + -8580;
	// 826FF994: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FF998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FF99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FF9A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FF9A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FF9A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FF9AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FF9B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FF9B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FF9B8: 4BD67469  bl 0x82466e20
	ctx.lr = 0x826FF9BC;
	sub_82466E20(ctx, base);
	// 826FF9BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FF9C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FF9C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FF9C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FF9D0 size=24
    let mut pc: u32 = 0x826FF9D0;
    'dispatch: loop {
        match pc {
            0x826FF9D0 => {
    //   block [0x826FF9D0..0x826FF9E8)
	// 826FF9D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FF9D4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FF9D8: 394A96D0  addi r10, r10, -0x6930
	ctx.r[10].s64 = ctx.r[10].s64 + -26928;
	// 826FF9DC: 816B6588  lwz r11, 0x6588(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25992 as u32) ) } as u64;
	// 826FF9E0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826FF9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FF9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FF9E8 size=116
    let mut pc: u32 = 0x826FF9E8;
    'dispatch: loop {
        match pc {
            0x826FF9E8 => {
    //   block [0x826FF9E8..0x826FFA5C)
	// 826FF9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FF9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FF9F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FF9F4: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FF9F8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FF9FC: 390B96D0  addi r8, r11, -0x6930
	ctx.r[8].s64 = ctx.r[11].s64 + -26928;
	// 826FFA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FFA04: 392ABDD4  addi r9, r10, -0x422c
	ctx.r[9].s64 = ctx.r[10].s64 + -16940;
	// 826FFA08: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFA0C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826FFA10: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 826FFA14: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FFA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FFA1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FFA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FFA24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FFA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FFA2C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826FFA30: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826FFA34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FFA38: 386BDEAC  addi r3, r11, -0x2154
	ctx.r[3].s64 = ctx.r[11].s64 + -8532;
	// 826FFA3C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826FFA40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FFA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FFA48: 4BD673D9  bl 0x82466e20
	ctx.lr = 0x826FFA4C;
	sub_82466E20(ctx, base);
	// 826FFA4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FFA50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FFA54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FFA58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FFA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FFA60 size=108
    let mut pc: u32 = 0x826FFA60;
    'dispatch: loop {
        match pc {
            0x826FFA60 => {
    //   block [0x826FFA60..0x826FFACC)
	// 826FFA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FFA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FFA68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FFA6C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FFA70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FFA74: 38EB6594  addi r7, r11, 0x6594
	ctx.r[7].s64 = ctx.r[11].s64 + 26004;
	// 826FFA78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FFA7C: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 826FFA80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FFA84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFA88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FFA8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FFA90: 386ADEDC  addi r3, r10, -0x2124
	ctx.r[3].s64 = ctx.r[10].s64 + -8484;
	// 826FFA94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FFA98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FFA9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FFAA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FFAA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FFAA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FFAAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FFAB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FFAB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FFAB8: 4BD67369  bl 0x82466e20
	ctx.lr = 0x826FFABC;
	sub_82466E20(ctx, base);
	// 826FFABC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FFAC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FFAC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FFAC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FFAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FFAD0 size=112
    let mut pc: u32 = 0x826FFAD0;
    'dispatch: loop {
        match pc {
            0x826FFAD0 => {
    //   block [0x826FFAD0..0x826FFB40)
	// 826FFAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FFAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FFAD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FFADC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFAE0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FFAE4: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 826FFAE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FFAEC: 390B65C4  addi r8, r11, 0x65c4
	ctx.r[8].s64 = ctx.r[11].s64 + 26052;
	// 826FFAF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FFAF4: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 826FFAF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FFAFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFB00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FFB04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FFB08: 386ADF0C  addi r3, r10, -0x20f4
	ctx.r[3].s64 = ctx.r[10].s64 + -8436;
	// 826FFB0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FFB10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FFB14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FFB18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FFB1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FFB20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FFB24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FFB28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FFB2C: 4BD672F5  bl 0x82466e20
	ctx.lr = 0x826FFB30;
	sub_82466E20(ctx, base);
	// 826FFB30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FFB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FFB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FFB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FFB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FFB40 size=112
    let mut pc: u32 = 0x826FFB40;
    'dispatch: loop {
        match pc {
            0x826FFB40 => {
    //   block [0x826FFB40..0x826FFBB0)
	// 826FFB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FFB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FFB48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FFB4C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FFB50: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FFB54: 392ABE18  addi r9, r10, -0x41e8
	ctx.r[9].s64 = ctx.r[10].s64 + -16872;
	// 826FFB58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FFB5C: 390B65E0  addi r8, r11, 0x65e0
	ctx.r[8].s64 = ctx.r[11].s64 + 26080;
	// 826FFB60: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826FFB64: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 826FFB68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FFB6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFB70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FFB74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FFB78: 386ADF3C  addi r3, r10, -0x20c4
	ctx.r[3].s64 = ctx.r[10].s64 + -8388;
	// 826FFB7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FFB80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FFB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FFB88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FFB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FFB90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FFB94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FFB98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FFB9C: 4BD67285  bl 0x82466e20
	ctx.lr = 0x826FFBA0;
	sub_82466E20(ctx, base);
	// 826FFBA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FFBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FFBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FFBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FFBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FFBB0 size=112
    let mut pc: u32 = 0x826FFBB0;
    'dispatch: loop {
        match pc {
            0x826FFBB0 => {
    //   block [0x826FFBB0..0x826FFC20)
	// 826FFBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FFBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FFBB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FFBBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFBC0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FFBC4: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 826FFBC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FFBCC: 390B6628  addi r8, r11, 0x6628
	ctx.r[8].s64 = ctx.r[11].s64 + 26152;
	// 826FFBD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FFBD4: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826FFBD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FFBDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFBE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FFBE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FFBE8: 386ADF6C  addi r3, r10, -0x2094
	ctx.r[3].s64 = ctx.r[10].s64 + -8340;
	// 826FFBEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FFBF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FFBF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FFBF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FFBFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FFC00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FFC04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FFC08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FFC0C: 4BD67215  bl 0x82466e20
	ctx.lr = 0x826FFC10;
	sub_82466E20(ctx, base);
	// 826FFC10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FFC14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FFC18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FFC1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FFC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FFC20 size=112
    let mut pc: u32 = 0x826FFC20;
    'dispatch: loop {
        match pc {
            0x826FFC20 => {
    //   block [0x826FFC20..0x826FFC90)
	// 826FFC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FFC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FFC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FFC2C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FFC30: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FFC34: 392ABE44  addi r9, r10, -0x41bc
	ctx.r[9].s64 = ctx.r[10].s64 + -16828;
	// 826FFC38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FFC3C: 390B6648  addi r8, r11, 0x6648
	ctx.r[8].s64 = ctx.r[11].s64 + 26184;
	// 826FFC40: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826FFC44: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 826FFC48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FFC4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFC50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FFC54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FFC58: 386ADF9C  addi r3, r10, -0x2064
	ctx.r[3].s64 = ctx.r[10].s64 + -8292;
	// 826FFC5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FFC60: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FFC64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FFC68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FFC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FFC70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FFC74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FFC78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FFC7C: 4BD671A5  bl 0x82466e20
	ctx.lr = 0x826FFC80;
	sub_82466E20(ctx, base);
	// 826FFC80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FFC84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FFC88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FFC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FFC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FFC90 size=112
    let mut pc: u32 = 0x826FFC90;
    'dispatch: loop {
        match pc {
            0x826FFC90 => {
    //   block [0x826FFC90..0x826FFD00)
	// 826FFC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FFC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FFC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FFC9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFCA0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FFCA4: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 826FFCA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FFCAC: 390B66D8  addi r8, r11, 0x66d8
	ctx.r[8].s64 = ctx.r[11].s64 + 26328;
	// 826FFCB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FFCB4: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 826FFCB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FFCBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFCC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FFCC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FFCC8: 386ADFCC  addi r3, r10, -0x2034
	ctx.r[3].s64 = ctx.r[10].s64 + -8244;
	// 826FFCCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FFCD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FFCD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FFCD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FFCDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FFCE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FFCE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FFCE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FFCEC: 4BD67135  bl 0x82466e20
	ctx.lr = 0x826FFCF0;
	sub_82466E20(ctx, base);
	// 826FFCF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FFCF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FFCF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FFCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FFD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FFD00 size=112
    let mut pc: u32 = 0x826FFD00;
    'dispatch: loop {
        match pc {
            0x826FFD00 => {
    //   block [0x826FFD00..0x826FFD70)
	// 826FFD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FFD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FFD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FFD0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFD10: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FFD14: 38AAE02C  addi r5, r10, -0x1fd4
	ctx.r[5].s64 = ctx.r[10].s64 + -8148;
	// 826FFD18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FFD1C: 390B66F0  addi r8, r11, 0x66f0
	ctx.r[8].s64 = ctx.r[11].s64 + 26352;
	// 826FFD20: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826FFD24: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 826FFD28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FFD2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFD30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FFD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FFD38: 386ADFFC  addi r3, r10, -0x2004
	ctx.r[3].s64 = ctx.r[10].s64 + -8196;
	// 826FFD3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FFD40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FFD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FFD48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FFD4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FFD50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FFD54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FFD58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FFD5C: 4BD670C5  bl 0x82466e20
	ctx.lr = 0x826FFD60;
	sub_82466E20(ctx, base);
	// 826FFD60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FFD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FFD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FFD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FFD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FFD70 size=100
    let mut pc: u32 = 0x826FFD70;
    'dispatch: loop {
        match pc {
            0x826FFD70 => {
    //   block [0x826FFD70..0x826FFDD4)
	// 826FFD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FFD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FFD78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FFD7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FFD84: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 826FFD88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FFD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FFD90: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 826FFD94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FFD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FFDA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FFDA4: 386AE02C  addi r3, r10, -0x1fd4
	ctx.r[3].s64 = ctx.r[10].s64 + -8148;
	// 826FFDA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FFDAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FFDB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FFDB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FFDB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FFDBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FFDC0: 4BD67061  bl 0x82466e20
	ctx.lr = 0x826FFDC4;
	sub_82466E20(ctx, base);
	// 826FFDC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FFDC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FFDCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FFDD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FFDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FFDD8 size=24
    let mut pc: u32 = 0x826FFDD8;
    'dispatch: loop {
        match pc {
            0x826FFDD8 => {
    //   block [0x826FFDD8..0x826FFDF0)
	// 826FFDD8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FFDDC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FFDE0: 394A97A8  addi r10, r10, -0x6858
	ctx.r[10].s64 = ctx.r[10].s64 + -26712;
	// 826FFDE4: 816B6644  lwz r11, 0x6644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26180 as u32) ) } as u64;
	// 826FFDE8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826FFDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FFDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FFDF0 size=116
    let mut pc: u32 = 0x826FFDF0;
    'dispatch: loop {
        match pc {
            0x826FFDF0 => {
    //   block [0x826FFDF0..0x826FFE64)
	// 826FFDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FFDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FFDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FFDFC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FFE00: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FFE04: 390B97A8  addi r8, r11, -0x6858
	ctx.r[8].s64 = ctx.r[11].s64 + -26712;
	// 826FFE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FFE0C: 392ABE80  addi r9, r10, -0x4180
	ctx.r[9].s64 = ctx.r[10].s64 + -16768;
	// 826FFE10: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFE14: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826FFE18: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 826FFE1C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FFE20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FFE24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FFE28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FFE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FFE30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FFE34: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826FFE38: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 826FFE3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FFE40: 386BE05C  addi r3, r11, -0x1fa4
	ctx.r[3].s64 = ctx.r[11].s64 + -8100;
	// 826FFE44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FFE48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FFE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FFE50: 4BD66FD1  bl 0x82466e20
	ctx.lr = 0x826FFE54;
	sub_82466E20(ctx, base);
	// 826FFE54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FFE58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FFE5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FFE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FFE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FFE68 size=108
    let mut pc: u32 = 0x826FFE68;
    'dispatch: loop {
        match pc {
            0x826FFE68 => {
    //   block [0x826FFE68..0x826FFED4)
	// 826FFE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FFE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FFE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FFE74: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FFE78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FFE7C: 38EB6768  addi r7, r11, 0x6768
	ctx.r[7].s64 = ctx.r[11].s64 + 26472;
	// 826FFE80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FFE84: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 826FFE88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FFE8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFE90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FFE94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FFE98: 386AE08C  addi r3, r10, -0x1f74
	ctx.r[3].s64 = ctx.r[10].s64 + -8052;
	// 826FFE9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FFEA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FFEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FFEA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FFEAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FFEB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FFEB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FFEB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FFEBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FFEC0: 4BD66F61  bl 0x82466e20
	ctx.lr = 0x826FFEC4;
	sub_82466E20(ctx, base);
	// 826FFEC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FFEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FFECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FFED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FFED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FFED8 size=112
    let mut pc: u32 = 0x826FFED8;
    'dispatch: loop {
        match pc {
            0x826FFED8 => {
    //   block [0x826FFED8..0x826FFF48)
	// 826FFED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FFEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FFEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FFEE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFEE8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FFEEC: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 826FFEF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FFEF4: 390B6798  addi r8, r11, 0x6798
	ctx.r[8].s64 = ctx.r[11].s64 + 26520;
	// 826FFEF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FFEFC: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826FFF00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FFF04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFF08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FFF0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FFF10: 386AE0BC  addi r3, r10, -0x1f44
	ctx.r[3].s64 = ctx.r[10].s64 + -8004;
	// 826FFF14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FFF18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FFF1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FFF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FFF24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FFF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FFF2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FFF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FFF34: 4BD66EED  bl 0x82466e20
	ctx.lr = 0x826FFF38;
	sub_82466E20(ctx, base);
	// 826FFF38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FFF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FFF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FFF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FFF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FFF48 size=112
    let mut pc: u32 = 0x826FFF48;
    'dispatch: loop {
        match pc {
            0x826FFF48 => {
    //   block [0x826FFF48..0x826FFFB8)
	// 826FFF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FFF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FFF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FFF54: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FFF58: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FFF5C: 392ABEA4  addi r9, r10, -0x415c
	ctx.r[9].s64 = ctx.r[10].s64 + -16732;
	// 826FFF60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FFF64: 390B67B8  addi r8, r11, 0x67b8
	ctx.r[8].s64 = ctx.r[11].s64 + 26552;
	// 826FFF68: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826FFF6C: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 826FFF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FFF74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFF78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FFF7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FFF80: 386AE0EC  addi r3, r10, -0x1f14
	ctx.r[3].s64 = ctx.r[10].s64 + -7956;
	// 826FFF84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FFF88: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FFF8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FFF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FFF94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FFF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FFF9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FFFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FFFA4: 4BD66E7D  bl 0x82466e20
	ctx.lr = 0x826FFFA8;
	sub_82466E20(ctx, base);
	// 826FFFA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FFFAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FFFB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FFFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FFFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FFFB8 size=112
    let mut pc: u32 = 0x826FFFB8;
    'dispatch: loop {
        match pc {
            0x826FFFB8 => {
    //   block [0x826FFFB8..0x82700028)
	// 826FFFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FFFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FFFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FFFC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFFC8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FFFCC: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 826FFFD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FFFD4: 390B6860  addi r8, r11, 0x6860
	ctx.r[8].s64 = ctx.r[11].s64 + 26720;
	// 826FFFD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FFFDC: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 826FFFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FFFE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FFFE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FFFEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FFFF0: 386AE11C  addi r3, r10, -0x1ee4
	ctx.r[3].s64 = ctx.r[10].s64 + -7908;
	// 826FFFF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FFFF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FFFFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270000C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700014: 4BD66E0D  bl 0x82466e20
	ctx.lr = 0x82700018;
	sub_82466E20(ctx, base);
	// 82700018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270001C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700028 size=108
    let mut pc: u32 = 0x82700028;
    'dispatch: loop {
        match pc {
            0x82700028 => {
    //   block [0x82700028..0x82700094)
	// 82700028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270002C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700034: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270003C: 38EB6878  addi r7, r11, 0x6878
	ctx.r[7].s64 = ctx.r[11].s64 + 26744;
	// 82700040: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82700044: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 82700048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270004C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700050: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82700054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700058: 386AE14C  addi r3, r10, -0x1eb4
	ctx.r[3].s64 = ctx.r[10].s64 + -7860;
	// 8270005C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82700060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270006C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270007C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82700080: 4BD66DA1  bl 0x82466e20
	ctx.lr = 0x82700084;
	sub_82466E20(ctx, base);
	// 82700084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270008C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700098 size=112
    let mut pc: u32 = 0x82700098;
    'dispatch: loop {
        match pc {
            0x82700098 => {
    //   block [0x82700098..0x82700108)
	// 82700098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270009C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827000A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827000A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827000A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827000AC: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 827000B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827000B4: 390B68A8  addi r8, r11, 0x68a8
	ctx.r[8].s64 = ctx.r[11].s64 + 26792;
	// 827000B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827000BC: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 827000C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827000C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827000C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827000CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827000D0: 386AE17C  addi r3, r10, -0x1e84
	ctx.r[3].s64 = ctx.r[10].s64 + -7812;
	// 827000D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827000D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827000DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827000E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827000E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827000E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827000EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827000F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827000F4: 4BD66D2D  bl 0x82466e20
	ctx.lr = 0x827000F8;
	sub_82466E20(ctx, base);
	// 827000F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827000FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700108 size=112
    let mut pc: u32 = 0x82700108;
    'dispatch: loop {
        match pc {
            0x82700108 => {
    //   block [0x82700108..0x82700178)
	// 82700108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270010C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700114: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82700118: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270011C: 392ABED8  addi r9, r10, -0x4128
	ctx.r[9].s64 = ctx.r[10].s64 + -16680;
	// 82700120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700124: 390B68C0  addi r8, r11, 0x68c0
	ctx.r[8].s64 = ctx.r[11].s64 + 26816;
	// 82700128: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8270012C: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 82700130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700134: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270013C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700140: 386AE1AC  addi r3, r10, -0x1e54
	ctx.r[3].s64 = ctx.r[10].s64 + -7764;
	// 82700144: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82700148: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8270014C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270015C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82700160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700164: 4BD66CBD  bl 0x82466e20
	ctx.lr = 0x82700168;
	sub_82466E20(ctx, base);
	// 82700168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270016C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700178 size=112
    let mut pc: u32 = 0x82700178;
    'dispatch: loop {
        match pc {
            0x82700178 => {
    //   block [0x82700178..0x827001E8)
	// 82700178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270017C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700184: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700188: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270018C: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 82700190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700194: 390B6968  addi r8, r11, 0x6968
	ctx.r[8].s64 = ctx.r[11].s64 + 26984;
	// 82700198: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270019C: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 827001A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827001A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827001A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827001AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827001B0: 386AE1DC  addi r3, r10, -0x1e24
	ctx.r[3].s64 = ctx.r[10].s64 + -7716;
	// 827001B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827001B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827001BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827001C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827001C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827001C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827001CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827001D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827001D4: 4BD66C4D  bl 0x82466e20
	ctx.lr = 0x827001D8;
	sub_82466E20(ctx, base);
	// 827001D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827001DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827001E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827001E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827001E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827001E8 size=112
    let mut pc: u32 = 0x827001E8;
    'dispatch: loop {
        match pc {
            0x827001E8 => {
    //   block [0x827001E8..0x82700258)
	// 827001E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827001EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827001F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827001F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827001F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827001FC: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 82700200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700204: 390B69B0  addi r8, r11, 0x69b0
	ctx.r[8].s64 = ctx.r[11].s64 + 27056;
	// 82700208: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8270020C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 82700210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700214: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270021C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700220: 386AE20C  addi r3, r10, -0x1df4
	ctx.r[3].s64 = ctx.r[10].s64 + -7668;
	// 82700224: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270022C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270023C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700244: 4BD66BDD  bl 0x82466e20
	ctx.lr = 0x82700248;
	sub_82466E20(ctx, base);
	// 82700248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270024C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700258 size=100
    let mut pc: u32 = 0x82700258;
    'dispatch: loop {
        match pc {
            0x82700258 => {
    //   block [0x82700258..0x827002BC)
	// 82700258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270025C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700264: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270026C: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 82700270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700278: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8270027C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270028C: 386AE23C  addi r3, r10, -0x1dc4
	ctx.r[3].s64 = ctx.r[10].s64 + -7620;
	// 82700290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700294: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700298: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270029C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827002A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827002A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827002A8: 4BD66B79  bl 0x82466e20
	ctx.lr = 0x827002AC;
	sub_82466E20(ctx, base);
	// 827002AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827002B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827002B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827002B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827002C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827002C0 size=112
    let mut pc: u32 = 0x827002C0;
    'dispatch: loop {
        match pc {
            0x827002C0 => {
    //   block [0x827002C0..0x82700330)
	// 827002C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827002C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827002C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827002CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827002D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827002D4: 38AADEAC  addi r5, r10, -0x2154
	ctx.r[5].s64 = ctx.r[10].s64 + -8532;
	// 827002D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827002DC: 390B6A88  addi r8, r11, 0x6a88
	ctx.r[8].s64 = ctx.r[11].s64 + 27272;
	// 827002E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827002E4: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 827002E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827002EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827002F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827002F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827002F8: 386AE26C  addi r3, r10, -0x1d94
	ctx.r[3].s64 = ctx.r[10].s64 + -7572;
	// 827002FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270030C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270031C: 4BD66B05  bl 0x82466e20
	ctx.lr = 0x82700320;
	sub_82466E20(ctx, base);
	// 82700320: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270032C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700330 size=112
    let mut pc: u32 = 0x82700330;
    'dispatch: loop {
        match pc {
            0x82700330 => {
    //   block [0x82700330..0x827003A0)
	// 82700330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270033C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700340: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700344: 38AADCFC  addi r5, r10, -0x2304
	ctx.r[5].s64 = ctx.r[10].s64 + -8964;
	// 82700348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270034C: 390B6AB8  addi r8, r11, 0x6ab8
	ctx.r[8].s64 = ctx.r[11].s64 + 27320;
	// 82700350: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82700354: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 82700358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270035C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700360: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700368: 386AE29C  addi r3, r10, -0x1d64
	ctx.r[3].s64 = ctx.r[10].s64 + -7524;
	// 8270036C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270037C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270038C: 4BD66A95  bl 0x82466e20
	ctx.lr = 0x82700390;
	sub_82466E20(ctx, base);
	// 82700390: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270039C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827003A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827003A0 size=108
    let mut pc: u32 = 0x827003A0;
    'dispatch: loop {
        match pc {
            0x827003A0 => {
    //   block [0x827003A0..0x8270040C)
	// 827003A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827003A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827003A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827003AC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827003B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827003B4: 38EB6AD0  addi r7, r11, 0x6ad0
	ctx.r[7].s64 = ctx.r[11].s64 + 27344;
	// 827003B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 827003BC: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 827003C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827003C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827003C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827003CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827003D0: 386AE2CC  addi r3, r10, -0x1d34
	ctx.r[3].s64 = ctx.r[10].s64 + -7476;
	// 827003D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827003D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827003DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827003E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827003E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827003E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827003EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827003F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827003F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827003F8: 4BD66A29  bl 0x82466e20
	ctx.lr = 0x827003FC;
	sub_82466E20(ctx, base);
	// 827003FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700410 size=112
    let mut pc: u32 = 0x82700410;
    'dispatch: loop {
        match pc {
            0x82700410 => {
    //   block [0x82700410..0x82700480)
	// 82700410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270041C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700420: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700424: 38AAE23C  addi r5, r10, -0x1dc4
	ctx.r[5].s64 = ctx.r[10].s64 + -7620;
	// 82700428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270042C: 390B6B00  addi r8, r11, 0x6b00
	ctx.r[8].s64 = ctx.r[11].s64 + 27392;
	// 82700430: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82700434: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 82700438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270043C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700448: 386AE2FC  addi r3, r10, -0x1d04
	ctx.r[3].s64 = ctx.r[10].s64 + -7428;
	// 8270044C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270045C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270046C: 4BD669B5  bl 0x82466e20
	ctx.lr = 0x82700470;
	sub_82466E20(ctx, base);
	// 82700470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270047C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700480 size=112
    let mut pc: u32 = 0x82700480;
    'dispatch: loop {
        match pc {
            0x82700480 => {
    //   block [0x82700480..0x827004F0)
	// 82700480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270048C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82700490: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700494: 392ABF04  addi r9, r10, -0x40fc
	ctx.r[9].s64 = ctx.r[10].s64 + -16636;
	// 82700498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270049C: 390B6B98  addi r8, r11, 0x6b98
	ctx.r[8].s64 = ctx.r[11].s64 + 27544;
	// 827004A0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 827004A4: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 827004A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827004AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827004B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827004B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827004B8: 386AE32C  addi r3, r10, -0x1cd4
	ctx.r[3].s64 = ctx.r[10].s64 + -7380;
	// 827004BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827004C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827004C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827004C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827004CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827004D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827004D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827004D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827004DC: 4BD66945  bl 0x82466e20
	ctx.lr = 0x827004E0;
	sub_82466E20(ctx, base);
	// 827004E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827004E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827004E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827004EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827004F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827004F0 size=112
    let mut pc: u32 = 0x827004F0;
    'dispatch: loop {
        match pc {
            0x827004F0 => {
    //   block [0x827004F0..0x82700560)
	// 827004F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827004F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827004F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827004FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700500: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700504: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 82700508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270050C: 390B6BE0  addi r8, r11, 0x6be0
	ctx.r[8].s64 = ctx.r[11].s64 + 27616;
	// 82700510: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82700514: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 82700518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270051C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700528: 386AE35C  addi r3, r10, -0x1ca4
	ctx.r[3].s64 = ctx.r[10].s64 + -7332;
	// 8270052C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270053C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270054C: 4BD668D5  bl 0x82466e20
	ctx.lr = 0x82700550;
	sub_82466E20(ctx, base);
	// 82700550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270055C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700560 size=108
    let mut pc: u32 = 0x82700560;
    'dispatch: loop {
        match pc {
            0x82700560 => {
    //   block [0x82700560..0x827005CC)
	// 82700560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270056C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700574: 38EB6BF8  addi r7, r11, 0x6bf8
	ctx.r[7].s64 = ctx.r[11].s64 + 27640;
	// 82700578: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8270057C: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 82700580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700584: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270058C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700590: 386AE38C  addi r3, r10, -0x1c74
	ctx.r[3].s64 = ctx.r[10].s64 + -7284;
	// 82700594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82700598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270059C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827005A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827005A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827005A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827005AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827005B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827005B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827005B8: 4BD66869  bl 0x82466e20
	ctx.lr = 0x827005BC;
	sub_82466E20(ctx, base);
	// 827005BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827005C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827005C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827005C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827005D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827005D0 size=116
    let mut pc: u32 = 0x827005D0;
    'dispatch: loop {
        match pc {
            0x827005D0 => {
    //   block [0x827005D0..0x82700644)
	// 827005D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827005D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827005D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827005DC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 827005E0: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 827005E4: 390A6C88  addi r8, r10, 0x6c88
	ctx.r[8].s64 = ctx.r[10].s64 + 27784;
	// 827005E8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827005EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827005F0: 38AAE23C  addi r5, r10, -0x1dc4
	ctx.r[5].s64 = ctx.r[10].s64 + -7620;
	// 827005F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827005F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827005FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700604: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 82700608: 396BBF18  addi r11, r11, -0x40e8
	ctx.r[11].s64 = ctx.r[11].s64 + -16616;
	// 8270060C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700610: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700614: 386AE3BC  addi r3, r10, -0x1c44
	ctx.r[3].s64 = ctx.r[10].s64 + -7236;
	// 82700618: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8270061C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700620: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82700624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270062C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700630: 4BD667F1  bl 0x82466e20
	ctx.lr = 0x82700634;
	sub_82466E20(ctx, base);
	// 82700634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270063C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700648 size=108
    let mut pc: u32 = 0x82700648;
    'dispatch: loop {
        match pc {
            0x82700648 => {
    //   block [0x82700648..0x827006B4)
	// 82700648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700654: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270065C: 38EB6D60  addi r7, r11, 0x6d60
	ctx.r[7].s64 = ctx.r[11].s64 + 28000;
	// 82700660: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82700664: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 82700668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270066C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700670: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82700674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700678: 386AE3EC  addi r3, r10, -0x1c14
	ctx.r[3].s64 = ctx.r[10].s64 + -7188;
	// 8270067C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82700680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270068C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270069C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827006A0: 4BD66781  bl 0x82466e20
	ctx.lr = 0x827006A4;
	sub_82466E20(ctx, base);
	// 827006A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827006A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827006AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827006B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827006B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827006B8 size=112
    let mut pc: u32 = 0x827006B8;
    'dispatch: loop {
        match pc {
            0x827006B8 => {
    //   block [0x827006B8..0x82700728)
	// 827006B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827006BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827006C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827006C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827006C8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827006CC: 38AAE23C  addi r5, r10, -0x1dc4
	ctx.r[5].s64 = ctx.r[10].s64 + -7620;
	// 827006D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827006D4: 390B6DA8  addi r8, r11, 0x6da8
	ctx.r[8].s64 = ctx.r[11].s64 + 28072;
	// 827006D8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 827006DC: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 827006E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827006E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827006E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827006EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827006F0: 386AE41C  addi r3, r10, -0x1be4
	ctx.r[3].s64 = ctx.r[10].s64 + -7140;
	// 827006F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827006F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827006FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270070C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700714: 4BD6670D  bl 0x82466e20
	ctx.lr = 0x82700718;
	sub_82466E20(ctx, base);
	// 82700718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270071C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700728 size=112
    let mut pc: u32 = 0x82700728;
    'dispatch: loop {
        match pc {
            0x82700728 => {
    //   block [0x82700728..0x82700798)
	// 82700728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270072C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700734: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700738: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270073C: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 82700740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700744: 390B6E20  addi r8, r11, 0x6e20
	ctx.r[8].s64 = ctx.r[11].s64 + 28192;
	// 82700748: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270074C: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 82700750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700754: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270075C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700760: 386AE44C  addi r3, r10, -0x1bb4
	ctx.r[3].s64 = ctx.r[10].s64 + -7092;
	// 82700764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270076C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270077C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700784: 4BD6669D  bl 0x82466e20
	ctx.lr = 0x82700788;
	sub_82466E20(ctx, base);
	// 82700788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270078C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700798 size=108
    let mut pc: u32 = 0x82700798;
    'dispatch: loop {
        match pc {
            0x82700798 => {
    //   block [0x82700798..0x82700804)
	// 82700798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270079C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827007A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827007A4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827007A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827007AC: 38EB6E50  addi r7, r11, 0x6e50
	ctx.r[7].s64 = ctx.r[11].s64 + 28240;
	// 827007B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 827007B4: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 827007B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827007BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827007C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827007C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827007C8: 386AE47C  addi r3, r10, -0x1b84
	ctx.r[3].s64 = ctx.r[10].s64 + -7044;
	// 827007CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827007D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827007D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827007D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827007DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827007E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827007E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827007E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827007EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827007F0: 4BD66631  bl 0x82466e20
	ctx.lr = 0x827007F4;
	sub_82466E20(ctx, base);
	// 827007F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827007F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827007FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700808 size=112
    let mut pc: u32 = 0x82700808;
    'dispatch: loop {
        match pc {
            0x82700808 => {
    //   block [0x82700808..0x82700878)
	// 82700808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270080C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700814: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700818: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270081C: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 82700820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700824: 390B6EC8  addi r8, r11, 0x6ec8
	ctx.r[8].s64 = ctx.r[11].s64 + 28360;
	// 82700828: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270082C: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82700830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700834: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700838: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270083C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700840: 386AE4AC  addi r3, r10, -0x1b54
	ctx.r[3].s64 = ctx.r[10].s64 + -6996;
	// 82700844: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270084C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270085C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700864: 4BD665BD  bl 0x82466e20
	ctx.lr = 0x82700868;
	sub_82466E20(ctx, base);
	// 82700868: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270086C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82700878 size=24
    let mut pc: u32 = 0x82700878;
    'dispatch: loop {
        match pc {
            0x82700878 => {
    //   block [0x82700878..0x82700890)
	// 82700878: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270087C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82700880: 394A9820  addi r10, r10, -0x67e0
	ctx.r[10].s64 = ctx.r[10].s64 + -26592;
	// 82700884: 816B6B94  lwz r11, 0x6b94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27540 as u32) ) } as u64;
	// 82700888: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8270088C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700890 size=116
    let mut pc: u32 = 0x82700890;
    'dispatch: loop {
        match pc {
            0x82700890 => {
    //   block [0x82700890..0x82700904)
	// 82700890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270089C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827008A0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827008A4: 390B9820  addi r8, r11, -0x67e0
	ctx.r[8].s64 = ctx.r[11].s64 + -26592;
	// 827008A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827008AC: 392ABF74  addi r9, r10, -0x408c
	ctx.r[9].s64 = ctx.r[10].s64 + -16524;
	// 827008B0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827008B4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827008B8: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827008BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827008C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827008C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827008C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827008CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827008D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827008D4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827008D8: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 827008DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827008E0: 386BE4DC  addi r3, r11, -0x1b24
	ctx.r[3].s64 = ctx.r[11].s64 + -6948;
	// 827008E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827008E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827008EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827008F0: 4BD66531  bl 0x82466e20
	ctx.lr = 0x827008F4;
	sub_82466E20(ctx, base);
	// 827008F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827008F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827008FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700908 size=112
    let mut pc: u32 = 0x82700908;
    'dispatch: loop {
        match pc {
            0x82700908 => {
    //   block [0x82700908..0x82700978)
	// 82700908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270090C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700914: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700918: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270091C: 38AAE4DC  addi r5, r10, -0x1b24
	ctx.r[5].s64 = ctx.r[10].s64 + -6948;
	// 82700920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700924: 390B6F10  addi r8, r11, 0x6f10
	ctx.r[8].s64 = ctx.r[11].s64 + 28432;
	// 82700928: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270092C: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 82700930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700934: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270093C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700940: 386AE50C  addi r3, r10, -0x1af4
	ctx.r[3].s64 = ctx.r[10].s64 + -6900;
	// 82700944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270094C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270095C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700964: 4BD664BD  bl 0x82466e20
	ctx.lr = 0x82700968;
	sub_82466E20(ctx, base);
	// 82700968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270096C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700978 size=112
    let mut pc: u32 = 0x82700978;
    'dispatch: loop {
        match pc {
            0x82700978 => {
    //   block [0x82700978..0x827009E8)
	// 82700978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270097C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700984: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700988: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270098C: 38AAE50C  addi r5, r10, -0x1af4
	ctx.r[5].s64 = ctx.r[10].s64 + -6900;
	// 82700990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700994: 390B6F40  addi r8, r11, 0x6f40
	ctx.r[8].s64 = ctx.r[11].s64 + 28480;
	// 82700998: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8270099C: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 827009A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827009A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827009A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827009AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827009B0: 386AE53C  addi r3, r10, -0x1ac4
	ctx.r[3].s64 = ctx.r[10].s64 + -6852;
	// 827009B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827009B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827009BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827009C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827009C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827009C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827009CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827009D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827009D4: 4BD6644D  bl 0x82466e20
	ctx.lr = 0x827009D8;
	sub_82466E20(ctx, base);
	// 827009D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827009DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827009E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827009E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827009E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827009E8 size=112
    let mut pc: u32 = 0x827009E8;
    'dispatch: loop {
        match pc {
            0x827009E8 => {
    //   block [0x827009E8..0x82700A58)
	// 827009E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827009EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827009F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827009F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827009F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827009FC: 38AAE50C  addi r5, r10, -0x1af4
	ctx.r[5].s64 = ctx.r[10].s64 + -6900;
	// 82700A00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700A04: 390B6FA0  addi r8, r11, 0x6fa0
	ctx.r[8].s64 = ctx.r[11].s64 + 28576;
	// 82700A08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82700A0C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82700A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700A14: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700A18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700A20: 386AE56C  addi r3, r10, -0x1a94
	ctx.r[3].s64 = ctx.r[10].s64 + -6804;
	// 82700A24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700A44: 4BD663DD  bl 0x82466e20
	ctx.lr = 0x82700A48;
	sub_82466E20(ctx, base);
	// 82700A48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700A58 size=112
    let mut pc: u32 = 0x82700A58;
    'dispatch: loop {
        match pc {
            0x82700A58 => {
    //   block [0x82700A58..0x82700AC8)
	// 82700A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700A64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700A68: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700A6C: 38AAE50C  addi r5, r10, -0x1af4
	ctx.r[5].s64 = ctx.r[10].s64 + -6900;
	// 82700A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700A74: 390B6FD0  addi r8, r11, 0x6fd0
	ctx.r[8].s64 = ctx.r[11].s64 + 28624;
	// 82700A78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82700A7C: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 82700A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700A84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700A90: 386AE59C  addi r3, r10, -0x1a64
	ctx.r[3].s64 = ctx.r[10].s64 + -6756;
	// 82700A94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700AB4: 4BD6636D  bl 0x82466e20
	ctx.lr = 0x82700AB8;
	sub_82466E20(ctx, base);
	// 82700AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700AC8 size=108
    let mut pc: u32 = 0x82700AC8;
    'dispatch: loop {
        match pc {
            0x82700AC8 => {
    //   block [0x82700AC8..0x82700B34)
	// 82700AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700AD4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700ADC: 38EB7018  addi r7, r11, 0x7018
	ctx.r[7].s64 = ctx.r[11].s64 + 28696;
	// 82700AE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82700AE4: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 82700AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700AEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700AF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82700AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700AF8: 386AE5CC  addi r3, r10, -0x1a34
	ctx.r[3].s64 = ctx.r[10].s64 + -6708;
	// 82700AFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82700B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700B1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82700B20: 4BD66301  bl 0x82466e20
	ctx.lr = 0x82700B24;
	sub_82466E20(ctx, base);
	// 82700B24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700B38 size=112
    let mut pc: u32 = 0x82700B38;
    'dispatch: loop {
        match pc {
            0x82700B38 => {
    //   block [0x82700B38..0x82700BA8)
	// 82700B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700B44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700B48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700B4C: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 82700B50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700B54: 390B7048  addi r8, r11, 0x7048
	ctx.r[8].s64 = ctx.r[11].s64 + 28744;
	// 82700B58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82700B5C: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 82700B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700B64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700B68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700B70: 386AE5FC  addi r3, r10, -0x1a04
	ctx.r[3].s64 = ctx.r[10].s64 + -6660;
	// 82700B74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700B78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700B80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700B84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700B88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700B90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700B94: 4BD6628D  bl 0x82466e20
	ctx.lr = 0x82700B98;
	sub_82466E20(ctx, base);
	// 82700B98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700BA8 size=116
    let mut pc: u32 = 0x82700BA8;
    'dispatch: loop {
        match pc {
            0x82700BA8 => {
    //   block [0x82700BA8..0x82700C1C)
	// 82700BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700BB4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 82700BB8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82700BBC: 390A7060  addi r8, r10, 0x7060
	ctx.r[8].s64 = ctx.r[10].s64 + 28768;
	// 82700BC0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700BC4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82700BC8: 38AAEAAC  addi r5, r10, -0x1554
	ctx.r[5].s64 = ctx.r[10].s64 + -5460;
	// 82700BCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700BD0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82700BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700BD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700BDC: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 82700BE0: 396BBF88  addi r11, r11, -0x4078
	ctx.r[11].s64 = ctx.r[11].s64 + -16504;
	// 82700BE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700BE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700BEC: 386AE62C  addi r3, r10, -0x19d4
	ctx.r[3].s64 = ctx.r[10].s64 + -6612;
	// 82700BF0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82700BF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700BF8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82700BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700C08: 4BD66219  bl 0x82466e20
	ctx.lr = 0x82700C0C;
	sub_82466E20(ctx, base);
	// 82700C0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700C20 size=100
    let mut pc: u32 = 0x82700C20;
    'dispatch: loop {
        match pc {
            0x82700C20 => {
    //   block [0x82700C20..0x82700C84)
	// 82700C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700C2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700C34: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82700C38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700C40: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 82700C44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700C54: 386AE65C  addi r3, r10, -0x19a4
	ctx.r[3].s64 = ctx.r[10].s64 + -6564;
	// 82700C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700C5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700C60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82700C64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700C68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82700C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700C70: 4BD661B1  bl 0x82466e20
	ctx.lr = 0x82700C74;
	sub_82466E20(ctx, base);
	// 82700C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700C88 size=100
    let mut pc: u32 = 0x82700C88;
    'dispatch: loop {
        match pc {
            0x82700C88 => {
    //   block [0x82700C88..0x82700CEC)
	// 82700C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700C94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700C9C: 38AAE6EC  addi r5, r10, -0x1914
	ctx.r[5].s64 = ctx.r[10].s64 + -6420;
	// 82700CA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700CA8: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 82700CAC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700CBC: 386AE68C  addi r3, r10, -0x1974
	ctx.r[3].s64 = ctx.r[10].s64 + -6516;
	// 82700CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700CC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700CC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82700CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700CD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82700CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700CD8: 4BD66149  bl 0x82466e20
	ctx.lr = 0x82700CDC;
	sub_82466E20(ctx, base);
	// 82700CDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700CF0 size=100
    let mut pc: u32 = 0x82700CF0;
    'dispatch: loop {
        match pc {
            0x82700CF0 => {
    //   block [0x82700CF0..0x82700D54)
	// 82700CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700CFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700D04: 38AAE62C  addi r5, r10, -0x19d4
	ctx.r[5].s64 = ctx.r[10].s64 + -6612;
	// 82700D08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700D10: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 82700D14: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700D24: 386AE6BC  addi r3, r10, -0x1944
	ctx.r[3].s64 = ctx.r[10].s64 + -6468;
	// 82700D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700D2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700D30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82700D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700D38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82700D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700D40: 4BD660E1  bl 0x82466e20
	ctx.lr = 0x82700D44;
	sub_82466E20(ctx, base);
	// 82700D44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700D58 size=104
    let mut pc: u32 = 0x82700D58;
    'dispatch: loop {
        match pc {
            0x82700D58 => {
    //   block [0x82700D58..0x82700DC0)
	// 82700D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700D64: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82700D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700D6C: 392ABFEC  addi r9, r10, -0x4014
	ctx.r[9].s64 = ctx.r[10].s64 + -16404;
	// 82700D70: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700D78: 38AAE65C  addi r5, r10, -0x19a4
	ctx.r[5].s64 = ctx.r[10].s64 + -6564;
	// 82700D7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700D8C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 82700D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700D94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700D98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82700D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700DA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82700DA4: 386AE6EC  addi r3, r10, -0x1914
	ctx.r[3].s64 = ctx.r[10].s64 + -6420;
	// 82700DA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82700DAC: 4BD66075  bl 0x82466e20
	ctx.lr = 0x82700DB0;
	sub_82466E20(ctx, base);
	// 82700DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700DC0 size=108
    let mut pc: u32 = 0x82700DC0;
    'dispatch: loop {
        match pc {
            0x82700DC0 => {
    //   block [0x82700DC0..0x82700E2C)
	// 82700DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700DCC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700DD4: 38EB71E4  addi r7, r11, 0x71e4
	ctx.r[7].s64 = ctx.r[11].s64 + 29156;
	// 82700DD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82700DDC: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82700DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700DE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82700DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700DF0: 386AE71C  addi r3, r10, -0x18e4
	ctx.r[3].s64 = ctx.r[10].s64 + -6372;
	// 82700DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82700DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82700E18: 4BD66009  bl 0x82466e20
	ctx.lr = 0x82700E1C;
	sub_82466E20(ctx, base);
	// 82700E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700E30 size=112
    let mut pc: u32 = 0x82700E30;
    'dispatch: loop {
        match pc {
            0x82700E30 => {
    //   block [0x82700E30..0x82700EA0)
	// 82700E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700E3C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700E40: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700E44: 38AAE6EC  addi r5, r10, -0x1914
	ctx.r[5].s64 = ctx.r[10].s64 + -6420;
	// 82700E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700E4C: 390B7218  addi r8, r11, 0x7218
	ctx.r[8].s64 = ctx.r[11].s64 + 29208;
	// 82700E50: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82700E54: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82700E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700E5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700E60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700E68: 386AE74C  addi r3, r10, -0x18b4
	ctx.r[3].s64 = ctx.r[10].s64 + -6324;
	// 82700E6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700E8C: 4BD65F95  bl 0x82466e20
	ctx.lr = 0x82700E90;
	sub_82466E20(ctx, base);
	// 82700E90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82700EA0 size=24
    let mut pc: u32 = 0x82700EA0;
    'dispatch: loop {
        match pc {
            0x82700EA0 => {
    //   block [0x82700EA0..0x82700EB8)
	// 82700EA0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700EA4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82700EA8: 394A9838  addi r10, r10, -0x67c8
	ctx.r[10].s64 = ctx.r[10].s64 + -26568;
	// 82700EAC: 816B7214  lwz r11, 0x7214(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29204 as u32) ) } as u64;
	// 82700EB0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82700EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700EB8 size=116
    let mut pc: u32 = 0x82700EB8;
    'dispatch: loop {
        match pc {
            0x82700EB8 => {
    //   block [0x82700EB8..0x82700F2C)
	// 82700EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700EC4: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82700EC8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82700ECC: 390B9838  addi r8, r11, -0x67c8
	ctx.r[8].s64 = ctx.r[11].s64 + -26568;
	// 82700ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700ED4: 392AC050  addi r9, r10, -0x3fb0
	ctx.r[9].s64 = ctx.r[10].s64 + -16304;
	// 82700ED8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700EDC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82700EE0: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82700EE4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700EEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700EFC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82700F00: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 82700F04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82700F08: 386BE77C  addi r3, r11, -0x1884
	ctx.r[3].s64 = ctx.r[11].s64 + -6276;
	// 82700F0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82700F10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700F18: 4BD65F09  bl 0x82466e20
	ctx.lr = 0x82700F1C;
	sub_82466E20(ctx, base);
	// 82700F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700F30 size=100
    let mut pc: u32 = 0x82700F30;
    'dispatch: loop {
        match pc {
            0x82700F30 => {
    //   block [0x82700F30..0x82700F94)
	// 82700F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700F3C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700F44: 38AAE77C  addi r5, r10, -0x1884
	ctx.r[5].s64 = ctx.r[10].s64 + -6276;
	// 82700F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700F50: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 82700F54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700F64: 386AE7AC  addi r3, r10, -0x1854
	ctx.r[3].s64 = ctx.r[10].s64 + -6228;
	// 82700F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700F6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700F70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82700F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700F78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82700F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700F80: 4BD65EA1  bl 0x82466e20
	ctx.lr = 0x82700F84;
	sub_82466E20(ctx, base);
	// 82700F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700F98 size=100
    let mut pc: u32 = 0x82700F98;
    'dispatch: loop {
        match pc {
            0x82700F98 => {
    //   block [0x82700F98..0x82700FFC)
	// 82700F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700FA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700FAC: 38AAE80C  addi r5, r10, -0x17f4
	ctx.r[5].s64 = ctx.r[10].s64 + -6132;
	// 82700FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700FB8: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 82700FBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700FCC: 386AE7DC  addi r3, r10, -0x1824
	ctx.r[3].s64 = ctx.r[10].s64 + -6180;
	// 82700FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700FD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700FD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82700FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700FE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82700FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700FE8: 4BD65E39  bl 0x82466e20
	ctx.lr = 0x82700FEC;
	sub_82466E20(ctx, base);
	// 82700FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701000 size=112
    let mut pc: u32 = 0x82701000;
    'dispatch: loop {
        match pc {
            0x82701000 => {
    //   block [0x82701000..0x82701070)
	// 82701000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270100C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701010: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701014: 38AAE77C  addi r5, r10, -0x1884
	ctx.r[5].s64 = ctx.r[10].s64 + -6276;
	// 82701018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270101C: 390B72C0  addi r8, r11, 0x72c0
	ctx.r[8].s64 = ctx.r[11].s64 + 29376;
	// 82701020: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82701024: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82701028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270102C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701038: 386AE80C  addi r3, r10, -0x17f4
	ctx.r[3].s64 = ctx.r[10].s64 + -6132;
	// 8270103C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270104C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270105C: 4BD65DC5  bl 0x82466e20
	ctx.lr = 0x82701060;
	sub_82466E20(ctx, base);
	// 82701060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270106C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701070 size=100
    let mut pc: u32 = 0x82701070;
    'dispatch: loop {
        match pc {
            0x82701070 => {
    //   block [0x82701070..0x827010D4)
	// 82701070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270107C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701084: 38AAE80C  addi r5, r10, -0x17f4
	ctx.r[5].s64 = ctx.r[10].s64 + -6132;
	// 82701088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270108C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701090: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 82701094: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270109C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827010A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827010A4: 386AE83C  addi r3, r10, -0x17c4
	ctx.r[3].s64 = ctx.r[10].s64 + -6084;
	// 827010A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827010AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827010B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827010B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827010B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827010BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827010C0: 4BD65D61  bl 0x82466e20
	ctx.lr = 0x827010C4;
	sub_82466E20(ctx, base);
	// 827010C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827010C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827010CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827010D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827010D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827010D8 size=100
    let mut pc: u32 = 0x827010D8;
    'dispatch: loop {
        match pc {
            0x827010D8 => {
    //   block [0x827010D8..0x8270113C)
	// 827010D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827010DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827010E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827010E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827010E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827010EC: 38AAE77C  addi r5, r10, -0x1884
	ctx.r[5].s64 = ctx.r[10].s64 + -6276;
	// 827010F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827010F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827010F8: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 827010FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270110C: 386AE86C  addi r3, r10, -0x1794
	ctx.r[3].s64 = ctx.r[10].s64 + -6036;
	// 82701110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701114: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701118: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270111C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701120: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82701124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701128: 4BD65CF9  bl 0x82466e20
	ctx.lr = 0x8270112C;
	sub_82466E20(ctx, base);
	// 8270112C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701140 size=100
    let mut pc: u32 = 0x82701140;
    'dispatch: loop {
        match pc {
            0x82701140 => {
    //   block [0x82701140..0x827011A4)
	// 82701140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270114C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701154: 38AAE7AC  addi r5, r10, -0x1854
	ctx.r[5].s64 = ctx.r[10].s64 + -6228;
	// 82701158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270115C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701160: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 82701164: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270116C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701174: 386AE89C  addi r3, r10, -0x1764
	ctx.r[3].s64 = ctx.r[10].s64 + -5988;
	// 82701178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270117C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701180: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82701184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701188: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270118C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701190: 4BD65C91  bl 0x82466e20
	ctx.lr = 0x82701194;
	sub_82466E20(ctx, base);
	// 82701194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270119C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827011A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827011A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827011A8 size=100
    let mut pc: u32 = 0x827011A8;
    'dispatch: loop {
        match pc {
            0x827011A8 => {
    //   block [0x827011A8..0x8270120C)
	// 827011A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827011AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827011B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827011B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827011B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827011BC: 38AAE86C  addi r5, r10, -0x1794
	ctx.r[5].s64 = ctx.r[10].s64 + -6036;
	// 827011C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827011C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827011C8: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 827011CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827011D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827011D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827011D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827011DC: 386AE8CC  addi r3, r10, -0x1734
	ctx.r[3].s64 = ctx.r[10].s64 + -5940;
	// 827011E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827011E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827011E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827011EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827011F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827011F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827011F8: 4BD65C29  bl 0x82466e20
	ctx.lr = 0x827011FC;
	sub_82466E20(ctx, base);
	// 827011FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701210 size=100
    let mut pc: u32 = 0x82701210;
    'dispatch: loop {
        match pc {
            0x82701210 => {
    //   block [0x82701210..0x82701274)
	// 82701210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270121C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701224: 38AAE7AC  addi r5, r10, -0x1854
	ctx.r[5].s64 = ctx.r[10].s64 + -6228;
	// 82701228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270122C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701230: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 82701234: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270123C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701244: 386AE8FC  addi r3, r10, -0x1704
	ctx.r[3].s64 = ctx.r[10].s64 + -5892;
	// 82701248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270124C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701250: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82701254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701258: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270125C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701260: 4BD65BC1  bl 0x82466e20
	ctx.lr = 0x82701264;
	sub_82466E20(ctx, base);
	// 82701264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270126C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701278 size=112
    let mut pc: u32 = 0x82701278;
    'dispatch: loop {
        match pc {
            0x82701278 => {
    //   block [0x82701278..0x827012E8)
	// 82701278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270127C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701284: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701288: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270128C: 38AAE98C  addi r5, r10, -0x1674
	ctx.r[5].s64 = ctx.r[10].s64 + -5748;
	// 82701290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701294: 390B72F0  addi r8, r11, 0x72f0
	ctx.r[8].s64 = ctx.r[11].s64 + 29424;
	// 82701298: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270129C: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 827012A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827012A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827012A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827012AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827012B0: 386AE92C  addi r3, r10, -0x16d4
	ctx.r[3].s64 = ctx.r[10].s64 + -5844;
	// 827012B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827012B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827012BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827012C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827012C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827012C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827012CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827012D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827012D4: 4BD65B4D  bl 0x82466e20
	ctx.lr = 0x827012D8;
	sub_82466E20(ctx, base);
	// 827012D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827012DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827012E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827012E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827012E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827012E8 size=112
    let mut pc: u32 = 0x827012E8;
    'dispatch: loop {
        match pc {
            0x827012E8 => {
    //   block [0x827012E8..0x82701358)
	// 827012E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827012EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827012F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827012F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827012F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827012FC: 38AAE9BC  addi r5, r10, -0x1644
	ctx.r[5].s64 = ctx.r[10].s64 + -5700;
	// 82701300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701304: 390B7320  addi r8, r11, 0x7320
	ctx.r[8].s64 = ctx.r[11].s64 + 29472;
	// 82701308: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270130C: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 82701310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701314: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270131C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701320: 386AE95C  addi r3, r10, -0x16a4
	ctx.r[3].s64 = ctx.r[10].s64 + -5796;
	// 82701324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270132C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270133C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701344: 4BD65ADD  bl 0x82466e20
	ctx.lr = 0x82701348;
	sub_82466E20(ctx, base);
	// 82701348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270134C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701358 size=112
    let mut pc: u32 = 0x82701358;
    'dispatch: loop {
        match pc {
            0x82701358 => {
    //   block [0x82701358..0x827013C8)
	// 82701358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270135C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701364: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701368: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270136C: 38AAEAAC  addi r5, r10, -0x1554
	ctx.r[5].s64 = ctx.r[10].s64 + -5460;
	// 82701370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701374: 390B7338  addi r8, r11, 0x7338
	ctx.r[8].s64 = ctx.r[11].s64 + 29496;
	// 82701378: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270137C: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82701380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701384: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270138C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701390: 386AE98C  addi r3, r10, -0x1674
	ctx.r[3].s64 = ctx.r[10].s64 + -5748;
	// 82701394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270139C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827013A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827013A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827013A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827013AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827013B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827013B4: 4BD65A6D  bl 0x82466e20
	ctx.lr = 0x827013B8;
	sub_82466E20(ctx, base);
	// 827013B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827013BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827013C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827013C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827013C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827013C8 size=112
    let mut pc: u32 = 0x827013C8;
    'dispatch: loop {
        match pc {
            0x827013C8 => {
    //   block [0x827013C8..0x82701438)
	// 827013C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827013CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827013D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827013D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827013D8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827013DC: 38AAE98C  addi r5, r10, -0x1674
	ctx.r[5].s64 = ctx.r[10].s64 + -5748;
	// 827013E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827013E4: 390B7368  addi r8, r11, 0x7368
	ctx.r[8].s64 = ctx.r[11].s64 + 29544;
	// 827013E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827013EC: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 827013F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827013F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827013F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827013FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701400: 386AE9BC  addi r3, r10, -0x1644
	ctx.r[3].s64 = ctx.r[10].s64 + -5700;
	// 82701404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270140C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270141C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701424: 4BD659FD  bl 0x82466e20
	ctx.lr = 0x82701428;
	sub_82466E20(ctx, base);
	// 82701428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270142C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701438 size=112
    let mut pc: u32 = 0x82701438;
    'dispatch: loop {
        match pc {
            0x82701438 => {
    //   block [0x82701438..0x827014A8)
	// 82701438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270143C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701444: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701448: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270144C: 38AAE9BC  addi r5, r10, -0x1644
	ctx.r[5].s64 = ctx.r[10].s64 + -5700;
	// 82701450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701454: 390B7380  addi r8, r11, 0x7380
	ctx.r[8].s64 = ctx.r[11].s64 + 29568;
	// 82701458: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270145C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 82701460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701464: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270146C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701470: 386AE9EC  addi r3, r10, -0x1614
	ctx.r[3].s64 = ctx.r[10].s64 + -5652;
	// 82701474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270147C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270148C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701494: 4BD6598D  bl 0x82466e20
	ctx.lr = 0x82701498;
	sub_82466E20(ctx, base);
	// 82701498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270149C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827014A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827014A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827014A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827014A8 size=112
    let mut pc: u32 = 0x827014A8;
    'dispatch: loop {
        match pc {
            0x827014A8 => {
    //   block [0x827014A8..0x82701518)
	// 827014A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827014AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827014B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827014B4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827014B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827014BC: 392AC07C  addi r9, r10, -0x3f84
	ctx.r[9].s64 = ctx.r[10].s64 + -16260;
	// 827014C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827014C4: 390B739C  addi r8, r11, 0x739c
	ctx.r[8].s64 = ctx.r[11].s64 + 29596;
	// 827014C8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 827014CC: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 827014D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827014D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827014D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827014DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827014E0: 386AEA1C  addi r3, r10, -0x15e4
	ctx.r[3].s64 = ctx.r[10].s64 + -5604;
	// 827014E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827014E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827014EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827014F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827014F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827014F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827014FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82701500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701504: 4BD6591D  bl 0x82466e20
	ctx.lr = 0x82701508;
	sub_82466E20(ctx, base);
	// 82701508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270150C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701518 size=116
    let mut pc: u32 = 0x82701518;
    'dispatch: loop {
        match pc {
            0x82701518 => {
    //   block [0x82701518..0x8270158C)
	// 82701518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270151C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701524: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 82701528: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8270152C: 390A73D0  addi r8, r10, 0x73d0
	ctx.r[8].s64 = ctx.r[10].s64 + 29648;
	// 82701530: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701534: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82701538: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 8270153C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701540: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82701544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701548: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270154C: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82701550: 396BC090  addi r11, r11, -0x3f70
	ctx.r[11].s64 = ctx.r[11].s64 + -16240;
	// 82701554: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701558: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270155C: 386AEA4C  addi r3, r10, -0x15b4
	ctx.r[3].s64 = ctx.r[10].s64 + -5556;
	// 82701560: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82701564: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701568: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8270156C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701578: 4BD658A9  bl 0x82466e20
	ctx.lr = 0x8270157C;
	sub_82466E20(ctx, base);
	// 8270157C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82701590 size=48
    let mut pc: u32 = 0x82701590;
    'dispatch: loop {
        match pc {
            0x82701590 => {
    //   block [0x82701590..0x827015C0)
	// 82701590: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701594: 814B7480  lwz r10, 0x7480(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29824 as u32) ) } as u64;
	// 82701598: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270159C: 396B98B0  addi r11, r11, -0x6750
	ctx.r[11].s64 = ctx.r[11].s64 + -26448;
	// 827015A0: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827015A4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 827015A8: 814A747C  lwz r10, 0x747c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29820 as u32) ) } as u64;
	// 827015AC: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 827015B0: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 827015B4: 814A7478  lwz r10, 0x7478(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29816 as u32) ) } as u64;
	// 827015B8: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 827015BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827015C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827015C0 size=116
    let mut pc: u32 = 0x827015C0;
    'dispatch: loop {
        match pc {
            0x827015C0 => {
    //   block [0x827015C0..0x82701634)
	// 827015C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827015C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827015C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827015CC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827015D0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827015D4: 392BC160  addi r9, r11, -0x3ea0
	ctx.r[9].s64 = ctx.r[11].s64 + -16032;
	// 827015D8: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827015DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827015E0: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 827015E4: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 827015E8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827015EC: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 827015F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827015F4: 396B98B0  addi r11, r11, -0x6750
	ctx.r[11].s64 = ctx.r[11].s64 + -26448;
	// 827015F8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 827015FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701600: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82701604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701608: 386AEA7C  addi r3, r10, -0x1584
	ctx.r[3].s64 = ctx.r[10].s64 + -5508;
	// 8270160C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82701610: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82701614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701618: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8270161C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82701620: 4BD65801  bl 0x82466e20
	ctx.lr = 0x82701624;
	sub_82466E20(ctx, base);
	// 82701624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270162C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701638 size=116
    let mut pc: u32 = 0x82701638;
    'dispatch: loop {
        match pc {
            0x82701638 => {
    //   block [0x82701638..0x827016AC)
	// 82701638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270163C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701644: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701648: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8270164C: 390B7488  addi r8, r11, 0x7488
	ctx.r[8].s64 = ctx.r[11].s64 + 29832;
	// 82701650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701654: 392AC288  addi r9, r10, -0x3d78
	ctx.r[9].s64 = ctx.r[10].s64 + -15736;
	// 82701658: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270165C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82701660: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82701664: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270166C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270167C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82701680: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 82701684: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82701688: 386BEAAC  addi r3, r11, -0x1554
	ctx.r[3].s64 = ctx.r[11].s64 + -5460;
	// 8270168C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82701690: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701698: 4BD65789  bl 0x82466e20
	ctx.lr = 0x8270169C;
	sub_82466E20(ctx, base);
	// 8270169C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827016A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827016A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827016A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827016B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827016B0 size=112
    let mut pc: u32 = 0x827016B0;
    'dispatch: loop {
        match pc {
            0x827016B0 => {
    //   block [0x827016B0..0x82701720)
	// 827016B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827016B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827016B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827016BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827016C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827016C4: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827016C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827016CC: 390B7518  addi r8, r11, 0x7518
	ctx.r[8].s64 = ctx.r[11].s64 + 29976;
	// 827016D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827016D4: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 827016D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827016DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827016E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827016E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827016E8: 386AEADC  addi r3, r10, -0x1524
	ctx.r[3].s64 = ctx.r[10].s64 + -5412;
	// 827016EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827016F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827016F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827016F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827016FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270170C: 4BD65715  bl 0x82466e20
	ctx.lr = 0x82701710;
	sub_82466E20(ctx, base);
	// 82701710: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270171C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701720 size=112
    let mut pc: u32 = 0x82701720;
    'dispatch: loop {
        match pc {
            0x82701720 => {
    //   block [0x82701720..0x82701790)
	// 82701720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270172C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701730: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701734: 38AACDFC  addi r5, r10, -0x3204
	ctx.r[5].s64 = ctx.r[10].s64 + -12804;
	// 82701738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270173C: 390B7530  addi r8, r11, 0x7530
	ctx.r[8].s64 = ctx.r[11].s64 + 30000;
	// 82701740: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82701744: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82701748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270174C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701758: 386AEB0C  addi r3, r10, -0x14f4
	ctx.r[3].s64 = ctx.r[10].s64 + -5364;
	// 8270175C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270176C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270177C: 4BD656A5  bl 0x82466e20
	ctx.lr = 0x82701780;
	sub_82466E20(ctx, base);
	// 82701780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270178C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701790 size=108
    let mut pc: u32 = 0x82701790;
    'dispatch: loop {
        match pc {
            0x82701790 => {
    //   block [0x82701790..0x827017FC)
	// 82701790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270179C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827017A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827017A4: 38EB7548  addi r7, r11, 0x7548
	ctx.r[7].s64 = ctx.r[11].s64 + 30024;
	// 827017A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827017AC: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 827017B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827017B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827017B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827017BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827017C0: 386AEB3C  addi r3, r10, -0x14c4
	ctx.r[3].s64 = ctx.r[10].s64 + -5316;
	// 827017C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827017C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827017CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827017D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827017D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827017D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827017DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827017E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827017E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827017E8: 4BD65639  bl 0x82466e20
	ctx.lr = 0x827017EC;
	sub_82466E20(ctx, base);
	// 827017EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827017F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827017F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827017F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701800 size=112
    let mut pc: u32 = 0x82701800;
    'dispatch: loop {
        match pc {
            0x82701800 => {
    //   block [0x82701800..0x82701870)
	// 82701800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270180C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701810: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701814: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82701818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270181C: 390B7560  addi r8, r11, 0x7560
	ctx.r[8].s64 = ctx.r[11].s64 + 30048;
	// 82701820: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82701824: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82701828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270182C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701838: 386AEB6C  addi r3, r10, -0x1494
	ctx.r[3].s64 = ctx.r[10].s64 + -5268;
	// 8270183C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270184C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270185C: 4BD655C5  bl 0x82466e20
	ctx.lr = 0x82701860;
	sub_82466E20(ctx, base);
	// 82701860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270186C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701870 size=108
    let mut pc: u32 = 0x82701870;
    'dispatch: loop {
        match pc {
            0x82701870 => {
    //   block [0x82701870..0x827018DC)
	// 82701870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270187C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701884: 38EB75A8  addi r7, r11, 0x75a8
	ctx.r[7].s64 = ctx.r[11].s64 + 30120;
	// 82701888: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8270188C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82701890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701894: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701898: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270189C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827018A0: 386AEB9C  addi r3, r10, -0x1464
	ctx.r[3].s64 = ctx.r[10].s64 + -5220;
	// 827018A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827018A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827018AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827018B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827018B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827018B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827018BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827018C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827018C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827018C8: 4BD65559  bl 0x82466e20
	ctx.lr = 0x827018CC;
	sub_82466E20(ctx, base);
	// 827018CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827018D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827018D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827018D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827018E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827018E0 size=112
    let mut pc: u32 = 0x827018E0;
    'dispatch: loop {
        match pc {
            0x827018E0 => {
    //   block [0x827018E0..0x82701950)
	// 827018E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827018E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827018E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827018EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827018F0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827018F4: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827018F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827018FC: 390B75C0  addi r8, r11, 0x75c0
	ctx.r[8].s64 = ctx.r[11].s64 + 30144;
	// 82701900: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82701904: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 82701908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270190C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701910: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701918: 386AEBCC  addi r3, r10, -0x1434
	ctx.r[3].s64 = ctx.r[10].s64 + -5172;
	// 8270191C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270192C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270193C: 4BD654E5  bl 0x82466e20
	ctx.lr = 0x82701940;
	sub_82466E20(ctx, base);
	// 82701940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270194C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701950 size=112
    let mut pc: u32 = 0x82701950;
    'dispatch: loop {
        match pc {
            0x82701950 => {
    //   block [0x82701950..0x827019C0)
	// 82701950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270195C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82701960: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701964: 392AC2E0  addi r9, r10, -0x3d20
	ctx.r[9].s64 = ctx.r[10].s64 + -15648;
	// 82701968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270196C: 390B75F8  addi r8, r11, 0x75f8
	ctx.r[8].s64 = ctx.r[11].s64 + 30200;
	// 82701970: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82701974: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 82701978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270197C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701988: 386AEBFC  addi r3, r10, -0x1404
	ctx.r[3].s64 = ctx.r[10].s64 + -5124;
	// 8270198C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82701990: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82701994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270199C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827019A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827019A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827019A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827019AC: 4BD65475  bl 0x82466e20
	ctx.lr = 0x827019B0;
	sub_82466E20(ctx, base);
	// 827019B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827019B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827019B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827019BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827019C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827019C0 size=116
    let mut pc: u32 = 0x827019C0;
    'dispatch: loop {
        match pc {
            0x827019C0 => {
    //   block [0x827019C0..0x82701A34)
	// 827019C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827019C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827019C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827019CC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827019D0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827019D4: 390B76A0  addi r8, r11, 0x76a0
	ctx.r[8].s64 = ctx.r[11].s64 + 30368;
	// 827019D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827019DC: 392AC2B4  addi r9, r10, -0x3d4c
	ctx.r[9].s64 = ctx.r[10].s64 + -15692;
	// 827019E0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827019E4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827019E8: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 827019EC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827019F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827019F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827019F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827019FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701A04: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82701A08: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 82701A0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82701A10: 386BEC2C  addi r3, r11, -0x13d4
	ctx.r[3].s64 = ctx.r[11].s64 + -5076;
	// 82701A14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82701A18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701A1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701A20: 4BD65401  bl 0x82466e20
	ctx.lr = 0x82701A24;
	sub_82466E20(ctx, base);
	// 82701A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701A38 size=112
    let mut pc: u32 = 0x82701A38;
    'dispatch: loop {
        match pc {
            0x82701A38 => {
    //   block [0x82701A38..0x82701AA8)
	// 82701A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701A44: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82701A48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701A4C: 392AC30C  addi r9, r10, -0x3cf4
	ctx.r[9].s64 = ctx.r[10].s64 + -15604;
	// 82701A50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701A54: 390B76C0  addi r8, r11, 0x76c0
	ctx.r[8].s64 = ctx.r[11].s64 + 30400;
	// 82701A58: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82701A5C: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 82701A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701A64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701A68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701A70: 386AEC5C  addi r3, r10, -0x13a4
	ctx.r[3].s64 = ctx.r[10].s64 + -5028;
	// 82701A74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82701A78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82701A7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701A8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82701A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701A94: 4BD6538D  bl 0x82466e20
	ctx.lr = 0x82701A98;
	sub_82466E20(ctx, base);
	// 82701A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701AA8 size=112
    let mut pc: u32 = 0x82701AA8;
    'dispatch: loop {
        match pc {
            0x82701AA8 => {
    //   block [0x82701AA8..0x82701B18)
	// 82701AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701AB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701AB8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701ABC: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 82701AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701AC4: 390B7720  addi r8, r11, 0x7720
	ctx.r[8].s64 = ctx.r[11].s64 + 30496;
	// 82701AC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82701ACC: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 82701AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701AD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701AD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701ADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701AE0: 386AEC8C  addi r3, r10, -0x1374
	ctx.r[3].s64 = ctx.r[10].s64 + -4980;
	// 82701AE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701B04: 4BD6531D  bl 0x82466e20
	ctx.lr = 0x82701B08;
	sub_82466E20(ctx, base);
	// 82701B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701B18 size=112
    let mut pc: u32 = 0x82701B18;
    'dispatch: loop {
        match pc {
            0x82701B18 => {
    //   block [0x82701B18..0x82701B88)
	// 82701B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701B24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701B28: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701B2C: 38AADD5C  addi r5, r10, -0x22a4
	ctx.r[5].s64 = ctx.r[10].s64 + -8868;
	// 82701B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701B34: 390B7738  addi r8, r11, 0x7738
	ctx.r[8].s64 = ctx.r[11].s64 + 30520;
	// 82701B38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82701B3C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82701B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701B44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701B50: 386AECBC  addi r3, r10, -0x1344
	ctx.r[3].s64 = ctx.r[10].s64 + -4932;
	// 82701B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701B74: 4BD652AD  bl 0x82466e20
	ctx.lr = 0x82701B78;
	sub_82466E20(ctx, base);
	// 82701B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701B88 size=112
    let mut pc: u32 = 0x82701B88;
    'dispatch: loop {
        match pc {
            0x82701B88 => {
    //   block [0x82701B88..0x82701BF8)
	// 82701B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701B94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701B98: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701B9C: 38AADD5C  addi r5, r10, -0x22a4
	ctx.r[5].s64 = ctx.r[10].s64 + -8868;
	// 82701BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701BA4: 390B7780  addi r8, r11, 0x7780
	ctx.r[8].s64 = ctx.r[11].s64 + 30592;
	// 82701BA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82701BAC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 82701BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701BB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701BB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701BC0: 386AECEC  addi r3, r10, -0x1314
	ctx.r[3].s64 = ctx.r[10].s64 + -4884;
	// 82701BC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701BE4: 4BD6523D  bl 0x82466e20
	ctx.lr = 0x82701BE8;
	sub_82466E20(ctx, base);
	// 82701BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701BF8 size=112
    let mut pc: u32 = 0x82701BF8;
    'dispatch: loop {
        match pc {
            0x82701BF8 => {
    //   block [0x82701BF8..0x82701C68)
	// 82701BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701C04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701C08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701C0C: 38AADD8C  addi r5, r10, -0x2274
	ctx.r[5].s64 = ctx.r[10].s64 + -8820;
	// 82701C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701C14: 390B77E0  addi r8, r11, 0x77e0
	ctx.r[8].s64 = ctx.r[11].s64 + 30688;
	// 82701C18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82701C1C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 82701C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701C24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701C30: 386AED1C  addi r3, r10, -0x12e4
	ctx.r[3].s64 = ctx.r[10].s64 + -4836;
	// 82701C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701C54: 4BD651CD  bl 0x82466e20
	ctx.lr = 0x82701C58;
	sub_82466E20(ctx, base);
	// 82701C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701C68 size=112
    let mut pc: u32 = 0x82701C68;
    'dispatch: loop {
        match pc {
            0x82701C68 => {
    //   block [0x82701C68..0x82701CD8)
	// 82701C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701C74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701C78: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701C7C: 38AADD8C  addi r5, r10, -0x2274
	ctx.r[5].s64 = ctx.r[10].s64 + -8820;
	// 82701C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701C84: 390B7840  addi r8, r11, 0x7840
	ctx.r[8].s64 = ctx.r[11].s64 + 30784;
	// 82701C88: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82701C8C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82701C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701C94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701CA0: 386AED4C  addi r3, r10, -0x12b4
	ctx.r[3].s64 = ctx.r[10].s64 + -4788;
	// 82701CA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701CC4: 4BD6515D  bl 0x82466e20
	ctx.lr = 0x82701CC8;
	sub_82466E20(ctx, base);
	// 82701CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701CD8 size=112
    let mut pc: u32 = 0x82701CD8;
    'dispatch: loop {
        match pc {
            0x82701CD8 => {
    //   block [0x82701CD8..0x82701D48)
	// 82701CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701CE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701CE8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701CEC: 38AADD5C  addi r5, r10, -0x22a4
	ctx.r[5].s64 = ctx.r[10].s64 + -8868;
	// 82701CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701CF4: 390B78A0  addi r8, r11, 0x78a0
	ctx.r[8].s64 = ctx.r[11].s64 + 30880;
	// 82701CF8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82701CFC: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 82701D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701D04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701D08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701D10: 386AED7C  addi r3, r10, -0x1284
	ctx.r[3].s64 = ctx.r[10].s64 + -4740;
	// 82701D14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701D34: 4BD650ED  bl 0x82466e20
	ctx.lr = 0x82701D38;
	sub_82466E20(ctx, base);
	// 82701D38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701D48 size=112
    let mut pc: u32 = 0x82701D48;
    'dispatch: loop {
        match pc {
            0x82701D48 => {
    //   block [0x82701D48..0x82701DB8)
	// 82701D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701D54: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 82701D58: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 82701D5C: 38EA7960  addi r7, r10, 0x7960
	ctx.r[7].s64 = ctx.r[10].s64 + 31072;
	// 82701D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701D64: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82701D68: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82701D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701D70: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82701D74: 396BC320  addi r11, r11, -0x3ce0
	ctx.r[11].s64 = ctx.r[11].s64 + -15584;
	// 82701D78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82701D7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701D80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701D84: 386AEDAC  addi r3, r10, -0x1254
	ctx.r[3].s64 = ctx.r[10].s64 + -4692;
	// 82701D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701D8C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82701D90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701D94: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82701D98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701D9C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701DA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82701DA4: 4BD6507D  bl 0x82466e20
	ctx.lr = 0x82701DA8;
	sub_82466E20(ctx, base);
	// 82701DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701DB8 size=112
    let mut pc: u32 = 0x82701DB8;
    'dispatch: loop {
        match pc {
            0x82701DB8 => {
    //   block [0x82701DB8..0x82701E28)
	// 82701DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701DC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701DC8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701DCC: 38AACEBC  addi r5, r10, -0x3144
	ctx.r[5].s64 = ctx.r[10].s64 + -12612;
	// 82701DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701DD4: 390B7B28  addi r8, r11, 0x7b28
	ctx.r[8].s64 = ctx.r[11].s64 + 31528;
	// 82701DD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82701DDC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 82701DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701DE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701DF0: 386AEDDC  addi r3, r10, -0x1224
	ctx.r[3].s64 = ctx.r[10].s64 + -4644;
	// 82701DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701E04: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82701E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701E14: 4BD6500D  bl 0x82466e20
	ctx.lr = 0x82701E18;
	sub_82466E20(ctx, base);
	// 82701E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701E28 size=112
    let mut pc: u32 = 0x82701E28;
    'dispatch: loop {
        match pc {
            0x82701E28 => {
    //   block [0x82701E28..0x82701E98)
	// 82701E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701E34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701E38: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701E3C: 38AACEBC  addi r5, r10, -0x3144
	ctx.r[5].s64 = ctx.r[10].s64 + -12612;
	// 82701E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701E44: 390B7B40  addi r8, r11, 0x7b40
	ctx.r[8].s64 = ctx.r[11].s64 + 31552;
	// 82701E48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82701E4C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82701E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701E54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701E58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701E60: 386AEE0C  addi r3, r10, -0x11f4
	ctx.r[3].s64 = ctx.r[10].s64 + -4596;
	// 82701E64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701E74: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82701E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701E84: 4BD64F9D  bl 0x82466e20
	ctx.lr = 0x82701E88;
	sub_82466E20(ctx, base);
	// 82701E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701E98 size=112
    let mut pc: u32 = 0x82701E98;
    'dispatch: loop {
        match pc {
            0x82701E98 => {
    //   block [0x82701E98..0x82701F08)
	// 82701E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701EA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701EA8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701EAC: 38AACEBC  addi r5, r10, -0x3144
	ctx.r[5].s64 = ctx.r[10].s64 + -12612;
	// 82701EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701EB4: 390B7B58  addi r8, r11, 0x7b58
	ctx.r[8].s64 = ctx.r[11].s64 + 31576;
	// 82701EB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82701EBC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 82701EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701EC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701EC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701ED0: 386AEE3C  addi r3, r10, -0x11c4
	ctx.r[3].s64 = ctx.r[10].s64 + -4548;
	// 82701ED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701EF4: 4BD64F2D  bl 0x82466e20
	ctx.lr = 0x82701EF8;
	sub_82466E20(ctx, base);
	// 82701EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701F08 size=108
    let mut pc: u32 = 0x82701F08;
    'dispatch: loop {
        match pc {
            0x82701F08 => {
    //   block [0x82701F08..0x82701F74)
	// 82701F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701F14: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701F1C: 38EB7B88  addi r7, r11, 0x7b88
	ctx.r[7].s64 = ctx.r[11].s64 + 31624;
	// 82701F20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82701F24: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82701F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701F2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82701F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701F38: 386AEE6C  addi r3, r10, -0x1194
	ctx.r[3].s64 = ctx.r[10].s64 + -4500;
	// 82701F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82701F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82701F60: 4BD64EC1  bl 0x82466e20
	ctx.lr = 0x82701F64;
	sub_82466E20(ctx, base);
	// 82701F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701F78 size=112
    let mut pc: u32 = 0x82701F78;
    'dispatch: loop {
        match pc {
            0x82701F78 => {
    //   block [0x82701F78..0x82701FE8)
	// 82701F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701F84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701F88: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701F8C: 38AACEBC  addi r5, r10, -0x3144
	ctx.r[5].s64 = ctx.r[10].s64 + -12612;
	// 82701F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701F94: 390B7BB8  addi r8, r11, 0x7bb8
	ctx.r[8].s64 = ctx.r[11].s64 + 31672;
	// 82701F98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82701F9C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 82701FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701FA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701FB0: 386AEE9C  addi r3, r10, -0x1164
	ctx.r[3].s64 = ctx.r[10].s64 + -4452;
	// 82701FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701FC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82701FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701FD4: 4BD64E4D  bl 0x82466e20
	ctx.lr = 0x82701FD8;
	sub_82466E20(ctx, base);
	// 82701FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701FE8 size=108
    let mut pc: u32 = 0x82701FE8;
    'dispatch: loop {
        match pc {
            0x82701FE8 => {
    //   block [0x82701FE8..0x82702054)
	// 82701FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701FF4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701FF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701FFC: 38EB7BD0  addi r7, r11, 0x7bd0
	ctx.r[7].s64 = ctx.r[11].s64 + 31696;
	// 82702000: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82702004: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 82702008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270200C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702010: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702018: 386AEECC  addi r3, r10, -0x1134
	ctx.r[3].s64 = ctx.r[10].s64 + -4404;
	// 8270201C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270202C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270203C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702040: 4BD64DE1  bl 0x82466e20
	ctx.lr = 0x82702044;
	sub_82466E20(ctx, base);
	// 82702044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270204C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702058 size=108
    let mut pc: u32 = 0x82702058;
    'dispatch: loop {
        match pc {
            0x82702058 => {
    //   block [0x82702058..0x827020C4)
	// 82702058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270205C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702064: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82702068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270206C: 38EB7C00  addi r7, r11, 0x7c00
	ctx.r[7].s64 = ctx.r[11].s64 + 31744;
	// 82702070: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82702074: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 82702078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270207C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702088: 386AEEFC  addi r3, r10, -0x1104
	ctx.r[3].s64 = ctx.r[10].s64 + -4356;
	// 8270208C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270209C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827020A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827020A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827020A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827020AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827020B0: 4BD64D71  bl 0x82466e20
	ctx.lr = 0x827020B4;
	sub_82466E20(ctx, base);
	// 827020B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827020B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827020BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827020C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827020C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827020C8 size=112
    let mut pc: u32 = 0x827020C8;
    'dispatch: loop {
        match pc {
            0x827020C8 => {
    //   block [0x827020C8..0x82702138)
	// 827020C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827020CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827020D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827020D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827020D8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827020DC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827020E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827020E4: 390B7C48  addi r8, r11, 0x7c48
	ctx.r[8].s64 = ctx.r[11].s64 + 31816;
	// 827020E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 827020EC: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 827020F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827020F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827020F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827020FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702100: 386AEF2C  addi r3, r10, -0x10d4
	ctx.r[3].s64 = ctx.r[10].s64 + -4308;
	// 82702104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270210C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270211C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702124: 4BD64CFD  bl 0x82466e20
	ctx.lr = 0x82702128;
	sub_82466E20(ctx, base);
	// 82702128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270212C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702138 size=112
    let mut pc: u32 = 0x82702138;
    'dispatch: loop {
        match pc {
            0x82702138 => {
    //   block [0x82702138..0x827021A8)
	// 82702138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270213C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702144: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702148: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270214C: 38AADD8C  addi r5, r10, -0x2274
	ctx.r[5].s64 = ctx.r[10].s64 + -8820;
	// 82702150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702154: 390B7C90  addi r8, r11, 0x7c90
	ctx.r[8].s64 = ctx.r[11].s64 + 31888;
	// 82702158: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8270215C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82702160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702164: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270216C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702170: 386AEF5C  addi r3, r10, -0x10a4
	ctx.r[3].s64 = ctx.r[10].s64 + -4260;
	// 82702174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270217C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270218C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702194: 4BD64C8D  bl 0x82466e20
	ctx.lr = 0x82702198;
	sub_82466E20(ctx, base);
	// 82702198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270219C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827021A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827021A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827021A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827021A8 size=108
    let mut pc: u32 = 0x827021A8;
    'dispatch: loop {
        match pc {
            0x827021A8 => {
    //   block [0x827021A8..0x82702214)
	// 827021A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827021AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827021B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827021B4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827021B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827021BC: 38EB7D20  addi r7, r11, 0x7d20
	ctx.r[7].s64 = ctx.r[11].s64 + 32032;
	// 827021C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 827021C4: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 827021C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827021CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827021D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827021D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827021D8: 386AEF8C  addi r3, r10, -0x1074
	ctx.r[3].s64 = ctx.r[10].s64 + -4212;
	// 827021DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827021E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827021E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827021E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827021EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827021F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827021F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827021F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827021FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702200: 4BD64C21  bl 0x82466e20
	ctx.lr = 0x82702204;
	sub_82466E20(ctx, base);
	// 82702204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270220C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702218 size=108
    let mut pc: u32 = 0x82702218;
    'dispatch: loop {
        match pc {
            0x82702218 => {
    //   block [0x82702218..0x82702284)
	// 82702218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270221C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702224: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82702228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270222C: 38EB7D68  addi r7, r11, 0x7d68
	ctx.r[7].s64 = ctx.r[11].s64 + 32104;
	// 82702230: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82702234: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 82702238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270223C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702240: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702248: 386AEFBC  addi r3, r10, -0x1044
	ctx.r[3].s64 = ctx.r[10].s64 + -4164;
	// 8270224C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270225C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270226C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702270: 4BD64BB1  bl 0x82466e20
	ctx.lr = 0x82702274;
	sub_82466E20(ctx, base);
	// 82702274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270227C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702288 size=108
    let mut pc: u32 = 0x82702288;
    'dispatch: loop {
        match pc {
            0x82702288 => {
    //   block [0x82702288..0x827022F4)
	// 82702288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270228C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702294: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82702298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270229C: 38EB7D98  addi r7, r11, 0x7d98
	ctx.r[7].s64 = ctx.r[11].s64 + 32152;
	// 827022A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 827022A4: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 827022A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827022AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827022B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827022B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827022B8: 386AEFEC  addi r3, r10, -0x1014
	ctx.r[3].s64 = ctx.r[10].s64 + -4116;
	// 827022BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827022C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827022C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827022C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827022CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827022D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827022D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827022D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827022DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827022E0: 4BD64B41  bl 0x82466e20
	ctx.lr = 0x827022E4;
	sub_82466E20(ctx, base);
	// 827022E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827022E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827022EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827022F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827022F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827022F8 size=112
    let mut pc: u32 = 0x827022F8;
    'dispatch: loop {
        match pc {
            0x827022F8 => {
    //   block [0x827022F8..0x82702368)
	// 827022F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827022FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702304: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702308: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270230C: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702314: 390B7DC8  addi r8, r11, 0x7dc8
	ctx.r[8].s64 = ctx.r[11].s64 + 32200;
	// 82702318: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270231C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 82702320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702324: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270232C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702330: 386AF01C  addi r3, r10, -0xfe4
	ctx.r[3].s64 = ctx.r[10].s64 + -4068;
	// 82702334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270233C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270234C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702354: 4BD64ACD  bl 0x82466e20
	ctx.lr = 0x82702358;
	sub_82466E20(ctx, base);
	// 82702358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270235C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702368 size=112
    let mut pc: u32 = 0x82702368;
    'dispatch: loop {
        match pc {
            0x82702368 => {
    //   block [0x82702368..0x827023D8)
	// 82702368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270236C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702374: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702378: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270237C: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702384: 390B7DF8  addi r8, r11, 0x7df8
	ctx.r[8].s64 = ctx.r[11].s64 + 32248;
	// 82702388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270238C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82702390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702394: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270239C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827023A0: 386AF04C  addi r3, r10, -0xfb4
	ctx.r[3].s64 = ctx.r[10].s64 + -4020;
	// 827023A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827023A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827023AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827023B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827023B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827023B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827023BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827023C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827023C4: 4BD64A5D  bl 0x82466e20
	ctx.lr = 0x827023C8;
	sub_82466E20(ctx, base);
	// 827023C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827023CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827023D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827023D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827023D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827023D8 size=112
    let mut pc: u32 = 0x827023D8;
    'dispatch: loop {
        match pc {
            0x827023D8 => {
    //   block [0x827023D8..0x82702448)
	// 827023D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827023DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827023E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827023E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827023E8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827023EC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827023F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827023F4: 390B7E10  addi r8, r11, 0x7e10
	ctx.r[8].s64 = ctx.r[11].s64 + 32272;
	// 827023F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827023FC: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 82702400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702404: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270240C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702410: 386AF07C  addi r3, r10, -0xf84
	ctx.r[3].s64 = ctx.r[10].s64 + -3972;
	// 82702414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270241C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270242C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702434: 4BD649ED  bl 0x82466e20
	ctx.lr = 0x82702438;
	sub_82466E20(ctx, base);
	// 82702438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270243C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702448 size=108
    let mut pc: u32 = 0x82702448;
    'dispatch: loop {
        match pc {
            0x82702448 => {
    //   block [0x82702448..0x827024B4)
	// 82702448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270244C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702454: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82702458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270245C: 38EB7E28  addi r7, r11, 0x7e28
	ctx.r[7].s64 = ctx.r[11].s64 + 32296;
	// 82702460: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82702464: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82702468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270246C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702478: 386AF0AC  addi r3, r10, -0xf54
	ctx.r[3].s64 = ctx.r[10].s64 + -3924;
	// 8270247C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270248C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270249C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827024A0: 4BD64981  bl 0x82466e20
	ctx.lr = 0x827024A4;
	sub_82466E20(ctx, base);
	// 827024A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827024A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827024AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827024B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827024B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827024B8 size=112
    let mut pc: u32 = 0x827024B8;
    'dispatch: loop {
        match pc {
            0x827024B8 => {
    //   block [0x827024B8..0x82702528)
	// 827024B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827024BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827024C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827024C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827024C8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827024CC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827024D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827024D4: 390B7E58  addi r8, r11, 0x7e58
	ctx.r[8].s64 = ctx.r[11].s64 + 32344;
	// 827024D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827024DC: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 827024E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827024E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827024E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827024EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827024F0: 386AF0DC  addi r3, r10, -0xf24
	ctx.r[3].s64 = ctx.r[10].s64 + -3876;
	// 827024F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827024F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827024FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270250C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702514: 4BD6490D  bl 0x82466e20
	ctx.lr = 0x82702518;
	sub_82466E20(ctx, base);
	// 82702518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270251C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702528 size=108
    let mut pc: u32 = 0x82702528;
    'dispatch: loop {
        match pc {
            0x82702528 => {
    //   block [0x82702528..0x82702594)
	// 82702528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270252C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702534: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82702538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270253C: 38EB7E70  addi r7, r11, 0x7e70
	ctx.r[7].s64 = ctx.r[11].s64 + 32368;
	// 82702540: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82702544: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 82702548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270254C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702558: 386AF10C  addi r3, r10, -0xef4
	ctx.r[3].s64 = ctx.r[10].s64 + -3828;
	// 8270255C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270256C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270257C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702580: 4BD648A1  bl 0x82466e20
	ctx.lr = 0x82702584;
	sub_82466E20(ctx, base);
	// 82702584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270258C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702598 size=112
    let mut pc: u32 = 0x82702598;
    'dispatch: loop {
        match pc {
            0x82702598 => {
    //   block [0x82702598..0x82702608)
	// 82702598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270259C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827025A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827025A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827025A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827025AC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827025B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827025B4: 390B7F60  addi r8, r11, 0x7f60
	ctx.r[8].s64 = ctx.r[11].s64 + 32608;
	// 827025B8: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 827025BC: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 827025C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827025C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827025C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827025CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827025D0: 386AF13C  addi r3, r10, -0xec4
	ctx.r[3].s64 = ctx.r[10].s64 + -3780;
	// 827025D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827025D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827025DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827025E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827025E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827025E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827025EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827025F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827025F4: 4BD6482D  bl 0x82466e20
	ctx.lr = 0x827025F8;
	sub_82466E20(ctx, base);
	// 827025F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827025FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702608 size=108
    let mut pc: u32 = 0x82702608;
    'dispatch: loop {
        match pc {
            0x82702608 => {
    //   block [0x82702608..0x82702674)
	// 82702608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270260C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702614: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270261C: 38EB8110  addi r7, r11, -0x7ef0
	ctx.r[7].s64 = ctx.r[11].s64 + -32496;
	// 82702620: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82702624: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 82702628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270262C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702638: 386AF16C  addi r3, r10, -0xe94
	ctx.r[3].s64 = ctx.r[10].s64 + -3732;
	// 8270263C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270264C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270265C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702660: 4BD647C1  bl 0x82466e20
	ctx.lr = 0x82702664;
	sub_82466E20(ctx, base);
	// 82702664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270266C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702678 size=112
    let mut pc: u32 = 0x82702678;
    'dispatch: loop {
        match pc {
            0x82702678 => {
    //   block [0x82702678..0x827026E8)
	// 82702678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270267C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702684: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702688: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270268C: 38AADD8C  addi r5, r10, -0x2274
	ctx.r[5].s64 = ctx.r[10].s64 + -8820;
	// 82702690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702694: 390B82A8  addi r8, r11, -0x7d58
	ctx.r[8].s64 = ctx.r[11].s64 + -32088;
	// 82702698: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8270269C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 827026A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827026A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827026A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827026AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827026B0: 386AF19C  addi r3, r10, -0xe64
	ctx.r[3].s64 = ctx.r[10].s64 + -3684;
	// 827026B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827026B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827026BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827026C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827026C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827026C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827026CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827026D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827026D4: 4BD6474D  bl 0x82466e20
	ctx.lr = 0x827026D8;
	sub_82466E20(ctx, base);
	// 827026D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827026DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827026E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827026E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827026E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827026E8 size=100
    let mut pc: u32 = 0x827026E8;
    'dispatch: loop {
        match pc {
            0x827026E8 => {
    //   block [0x827026E8..0x8270274C)
	// 827026E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827026EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827026F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827026F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827026F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827026FC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702708: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 8270270C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270271C: 386AF1CC  addi r3, r10, -0xe34
	ctx.r[3].s64 = ctx.r[10].s64 + -3636;
	// 82702720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702724: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702728: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270272C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702730: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82702734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702738: 4BD646E9  bl 0x82466e20
	ctx.lr = 0x8270273C;
	sub_82466E20(ctx, base);
	// 8270273C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702750 size=112
    let mut pc: u32 = 0x82702750;
    'dispatch: loop {
        match pc {
            0x82702750 => {
    //   block [0x82702750..0x827027C0)
	// 82702750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270275C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702760: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702764: 38AAF1CC  addi r5, r10, -0xe34
	ctx.r[5].s64 = ctx.r[10].s64 + -3636;
	// 82702768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270276C: 390B8500  addi r8, r11, -0x7b00
	ctx.r[8].s64 = ctx.r[11].s64 + -31488;
	// 82702770: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82702774: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82702778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270277C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82702784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702788: 386AF1FC  addi r3, r10, -0xe04
	ctx.r[3].s64 = ctx.r[10].s64 + -3588;
	// 8270278C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270279C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827027A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827027A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827027A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827027AC: 4BD64675  bl 0x82466e20
	ctx.lr = 0x827027B0;
	sub_82466E20(ctx, base);
	// 827027B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827027B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827027B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827027BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827027C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827027C0 size=100
    let mut pc: u32 = 0x827027C0;
    'dispatch: loop {
        match pc {
            0x827027C0 => {
    //   block [0x827027C0..0x82702824)
	// 827027C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827027C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827027C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827027CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827027D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827027D4: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827027D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827027DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827027E0: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 827027E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827027E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827027EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827027F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827027F4: 386AF22C  addi r3, r10, -0xdd4
	ctx.r[3].s64 = ctx.r[10].s64 + -3540;
	// 827027F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827027FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702800: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82702804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702808: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270280C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702810: 4BD64611  bl 0x82466e20
	ctx.lr = 0x82702814;
	sub_82466E20(ctx, base);
	// 82702814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270281C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702828 size=108
    let mut pc: u32 = 0x82702828;
    'dispatch: loop {
        match pc {
            0x82702828 => {
    //   block [0x82702828..0x82702894)
	// 82702828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270282C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702834: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270283C: 38EB8578  addi r7, r11, -0x7a88
	ctx.r[7].s64 = ctx.r[11].s64 + -31368;
	// 82702840: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82702844: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 82702848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270284C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702858: 386AF25C  addi r3, r10, -0xda4
	ctx.r[3].s64 = ctx.r[10].s64 + -3492;
	// 8270285C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270286C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270287C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702880: 4BD645A1  bl 0x82466e20
	ctx.lr = 0x82702884;
	sub_82466E20(ctx, base);
	// 82702884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270288C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702898 size=112
    let mut pc: u32 = 0x82702898;
    'dispatch: loop {
        match pc {
            0x82702898 => {
    //   block [0x82702898..0x82702908)
	// 82702898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270289C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827028A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827028A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827028A8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827028AC: 38AAF22C  addi r5, r10, -0xdd4
	ctx.r[5].s64 = ctx.r[10].s64 + -3540;
	// 827028B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827028B4: 390B85C0  addi r8, r11, -0x7a40
	ctx.r[8].s64 = ctx.r[11].s64 + -31296;
	// 827028B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827028BC: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 827028C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827028C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827028C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827028CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827028D0: 386AF28C  addi r3, r10, -0xd74
	ctx.r[3].s64 = ctx.r[10].s64 + -3444;
	// 827028D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827028D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827028DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827028E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827028E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827028E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827028EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827028F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827028F4: 4BD6452D  bl 0x82466e20
	ctx.lr = 0x827028F8;
	sub_82466E20(ctx, base);
	// 827028F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827028FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702908 size=100
    let mut pc: u32 = 0x82702908;
    'dispatch: loop {
        match pc {
            0x82702908 => {
    //   block [0x82702908..0x8270296C)
	// 82702908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270290C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702914: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270291C: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702928: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8270292C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270293C: 386AF2BC  addi r3, r10, -0xd44
	ctx.r[3].s64 = ctx.r[10].s64 + -3396;
	// 82702940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702944: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702948: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270294C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702950: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82702954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702958: 4BD644C9  bl 0x82466e20
	ctx.lr = 0x8270295C;
	sub_82466E20(ctx, base);
	// 8270295C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702970 size=100
    let mut pc: u32 = 0x82702970;
    'dispatch: loop {
        match pc {
            0x82702970 => {
    //   block [0x82702970..0x827029D4)
	// 82702970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270297C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702984: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270298C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702990: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 82702994: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270299C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827029A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827029A4: 386AF2EC  addi r3, r10, -0xd14
	ctx.r[3].s64 = ctx.r[10].s64 + -3348;
	// 827029A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827029AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827029B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827029B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827029B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827029BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827029C0: 4BD64461  bl 0x82466e20
	ctx.lr = 0x827029C4;
	sub_82466E20(ctx, base);
	// 827029C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827029C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827029CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827029D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827029D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827029D8 size=112
    let mut pc: u32 = 0x827029D8;
    'dispatch: loop {
        match pc {
            0x827029D8 => {
    //   block [0x827029D8..0x82702A48)
	// 827029D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827029DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827029E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827029E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827029E8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827029EC: 38AAF2BC  addi r5, r10, -0xd44
	ctx.r[5].s64 = ctx.r[10].s64 + -3396;
	// 827029F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827029F4: 390B85F0  addi r8, r11, -0x7a10
	ctx.r[8].s64 = ctx.r[11].s64 + -31248;
	// 827029F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 827029FC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 82702A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702A04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702A08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82702A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702A10: 386AF31C  addi r3, r10, -0xce4
	ctx.r[3].s64 = ctx.r[10].s64 + -3300;
	// 82702A14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702A34: 4BD643ED  bl 0x82466e20
	ctx.lr = 0x82702A38;
	sub_82466E20(ctx, base);
	// 82702A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702A48 size=112
    let mut pc: u32 = 0x82702A48;
    'dispatch: loop {
        match pc {
            0x82702A48 => {
    //   block [0x82702A48..0x82702AB8)
	// 82702A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702A54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702A58: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702A5C: 38AAF2EC  addi r5, r10, -0xd14
	ctx.r[5].s64 = ctx.r[10].s64 + -3348;
	// 82702A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702A64: 390B8650  addi r8, r11, -0x79b0
	ctx.r[8].s64 = ctx.r[11].s64 + -31152;
	// 82702A68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82702A6C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82702A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702A74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702A78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82702A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702A80: 386AF34C  addi r3, r10, -0xcb4
	ctx.r[3].s64 = ctx.r[10].s64 + -3252;
	// 82702A84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702AA4: 4BD6437D  bl 0x82466e20
	ctx.lr = 0x82702AA8;
	sub_82466E20(ctx, base);
	// 82702AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702AB8 size=100
    let mut pc: u32 = 0x82702AB8;
    'dispatch: loop {
        match pc {
            0x82702AB8 => {
    //   block [0x82702AB8..0x82702B1C)
	// 82702AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702AC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702ACC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702AD8: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 82702ADC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702AEC: 386AF37C  addi r3, r10, -0xc84
	ctx.r[3].s64 = ctx.r[10].s64 + -3204;
	// 82702AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702AF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702AF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82702AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702B00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82702B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702B08: 4BD64319  bl 0x82466e20
	ctx.lr = 0x82702B0C;
	sub_82466E20(ctx, base);
	// 82702B0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702B20 size=112
    let mut pc: u32 = 0x82702B20;
    'dispatch: loop {
        match pc {
            0x82702B20 => {
    //   block [0x82702B20..0x82702B90)
	// 82702B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702B2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702B30: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702B34: 38AAF37C  addi r5, r10, -0xc84
	ctx.r[5].s64 = ctx.r[10].s64 + -3204;
	// 82702B38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702B3C: 390B86B0  addi r8, r11, -0x7950
	ctx.r[8].s64 = ctx.r[11].s64 + -31056;
	// 82702B40: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82702B44: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 82702B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702B4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702B50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82702B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702B58: 386AF3AC  addi r3, r10, -0xc54
	ctx.r[3].s64 = ctx.r[10].s64 + -3156;
	// 82702B5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702B7C: 4BD642A5  bl 0x82466e20
	ctx.lr = 0x82702B80;
	sub_82466E20(ctx, base);
	// 82702B80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702B90 size=108
    let mut pc: u32 = 0x82702B90;
    'dispatch: loop {
        match pc {
            0x82702B90 => {
    //   block [0x82702B90..0x82702BFC)
	// 82702B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702B9C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702BA4: 38EB87A0  addi r7, r11, -0x7860
	ctx.r[7].s64 = ctx.r[11].s64 + -30816;
	// 82702BA8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82702BAC: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 82702BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702BB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702BB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702BC0: 386AF3DC  addi r3, r10, -0xc24
	ctx.r[3].s64 = ctx.r[10].s64 + -3108;
	// 82702BC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702BE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702BE8: 4BD64239  bl 0x82466e20
	ctx.lr = 0x82702BEC;
	sub_82466E20(ctx, base);
	// 82702BEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702C00 size=108
    let mut pc: u32 = 0x82702C00;
    'dispatch: loop {
        match pc {
            0x82702C00 => {
    //   block [0x82702C00..0x82702C6C)
	// 82702C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702C0C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702C14: 38EB8890  addi r7, r11, -0x7770
	ctx.r[7].s64 = ctx.r[11].s64 + -30576;
	// 82702C18: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82702C1C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 82702C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702C24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702C28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702C30: 386AF40C  addi r3, r10, -0xbf4
	ctx.r[3].s64 = ctx.r[10].s64 + -3060;
	// 82702C34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702C58: 4BD641C9  bl 0x82466e20
	ctx.lr = 0x82702C5C;
	sub_82466E20(ctx, base);
	// 82702C5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702C70 size=108
    let mut pc: u32 = 0x82702C70;
    'dispatch: loop {
        match pc {
            0x82702C70 => {
    //   block [0x82702C70..0x82702CDC)
	// 82702C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702C7C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702C84: 38EB88D8  addi r7, r11, -0x7728
	ctx.r[7].s64 = ctx.r[11].s64 + -30504;
	// 82702C88: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82702C8C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82702C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702C94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702C98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702C9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702CA0: 386AF43C  addi r3, r10, -0xbc4
	ctx.r[3].s64 = ctx.r[10].s64 + -3012;
	// 82702CA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702CC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702CC8: 4BD64159  bl 0x82466e20
	ctx.lr = 0x82702CCC;
	sub_82466E20(ctx, base);
	// 82702CCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702CE0 size=108
    let mut pc: u32 = 0x82702CE0;
    'dispatch: loop {
        match pc {
            0x82702CE0 => {
    //   block [0x82702CE0..0x82702D4C)
	// 82702CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702CEC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702CF4: 38EB89B0  addi r7, r11, -0x7650
	ctx.r[7].s64 = ctx.r[11].s64 + -30288;
	// 82702CF8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82702CFC: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 82702D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702D04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702D08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702D10: 386AF46C  addi r3, r10, -0xb94
	ctx.r[3].s64 = ctx.r[10].s64 + -2964;
	// 82702D14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702D24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702D34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702D38: 4BD640E9  bl 0x82466e20
	ctx.lr = 0x82702D3C;
	sub_82466E20(ctx, base);
	// 82702D3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702D50 size=100
    let mut pc: u32 = 0x82702D50;
    'dispatch: loop {
        match pc {
            0x82702D50 => {
    //   block [0x82702D50..0x82702DB4)
	// 82702D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702D5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702D64: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702D70: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 82702D74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702D84: 386AF49C  addi r3, r10, -0xb64
	ctx.r[3].s64 = ctx.r[10].s64 + -2916;
	// 82702D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702D8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702D90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82702D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702D98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82702D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702DA0: 4BD64081  bl 0x82466e20
	ctx.lr = 0x82702DA4;
	sub_82466E20(ctx, base);
	// 82702DA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702DB8 size=112
    let mut pc: u32 = 0x82702DB8;
    'dispatch: loop {
        match pc {
            0x82702DB8 => {
    //   block [0x82702DB8..0x82702E28)
	// 82702DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702DC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702DC8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702DCC: 38AAF49C  addi r5, r10, -0xb64
	ctx.r[5].s64 = ctx.r[10].s64 + -2916;
	// 82702DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702DD4: 390B89C8  addi r8, r11, -0x7638
	ctx.r[8].s64 = ctx.r[11].s64 + -30264;
	// 82702DD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82702DDC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 82702DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702DE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82702DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702DF0: 386AF4CC  addi r3, r10, -0xb34
	ctx.r[3].s64 = ctx.r[10].s64 + -2868;
	// 82702DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702E14: 4BD6400D  bl 0x82466e20
	ctx.lr = 0x82702E18;
	sub_82466E20(ctx, base);
	// 82702E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702E28 size=108
    let mut pc: u32 = 0x82702E28;
    'dispatch: loop {
        match pc {
            0x82702E28 => {
    //   block [0x82702E28..0x82702E94)
	// 82702E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702E34: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702E3C: 38EB8A10  addi r7, r11, -0x75f0
	ctx.r[7].s64 = ctx.r[11].s64 + -30192;
	// 82702E40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82702E44: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82702E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702E4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702E50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702E58: 386AF4FC  addi r3, r10, -0xb04
	ctx.r[3].s64 = ctx.r[10].s64 + -2820;
	// 82702E5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702E80: 4BD63FA1  bl 0x82466e20
	ctx.lr = 0x82702E84;
	sub_82466E20(ctx, base);
	// 82702E84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702E98 size=112
    let mut pc: u32 = 0x82702E98;
    'dispatch: loop {
        match pc {
            0x82702E98 => {
    //   block [0x82702E98..0x82702F08)
	// 82702E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702EA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702EA8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702EAC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702EB4: 390B8A58  addi r8, r11, -0x75a8
	ctx.r[8].s64 = ctx.r[11].s64 + -30120;
	// 82702EB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82702EBC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 82702EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702EC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702EC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82702ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702ED0: 386AF52C  addi r3, r10, -0xad4
	ctx.r[3].s64 = ctx.r[10].s64 + -2772;
	// 82702ED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702EF4: 4BD63F2D  bl 0x82466e20
	ctx.lr = 0x82702EF8;
	sub_82466E20(ctx, base);
	// 82702EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702F08 size=108
    let mut pc: u32 = 0x82702F08;
    'dispatch: loop {
        match pc {
            0x82702F08 => {
    //   block [0x82702F08..0x82702F74)
	// 82702F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702F14: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702F1C: 38EB8A70  addi r7, r11, -0x7590
	ctx.r[7].s64 = ctx.r[11].s64 + -30096;
	// 82702F20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82702F24: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82702F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702F2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702F38: 386AF55C  addi r3, r10, -0xaa4
	ctx.r[3].s64 = ctx.r[10].s64 + -2724;
	// 82702F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702F60: 4BD63EC1  bl 0x82466e20
	ctx.lr = 0x82702F64;
	sub_82466E20(ctx, base);
	// 82702F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702F78 size=112
    let mut pc: u32 = 0x82702F78;
    'dispatch: loop {
        match pc {
            0x82702F78 => {
    //   block [0x82702F78..0x82702FE8)
	// 82702F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702F84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702F88: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702F8C: 38AAF52C  addi r5, r10, -0xad4
	ctx.r[5].s64 = ctx.r[10].s64 + -2772;
	// 82702F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702F94: 390B8AB8  addi r8, r11, -0x7548
	ctx.r[8].s64 = ctx.r[11].s64 + -30024;
	// 82702F98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82702F9C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82702FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702FA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82702FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702FB0: 386AF58C  addi r3, r10, -0xa74
	ctx.r[3].s64 = ctx.r[10].s64 + -2676;
	// 82702FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702FD4: 4BD63E4D  bl 0x82466e20
	ctx.lr = 0x82702FD8;
	sub_82466E20(ctx, base);
	// 82702FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702FE8 size=100
    let mut pc: u32 = 0x82702FE8;
    'dispatch: loop {
        match pc {
            0x82702FE8 => {
    //   block [0x82702FE8..0x8270304C)
	// 82702FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702FF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702FFC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82703000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82703004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703008: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8270300C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270301C: 386AF5BC  addi r3, r10, -0xa44
	ctx.r[3].s64 = ctx.r[10].s64 + -2628;
	// 82703020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703024: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703028: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270302C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703030: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82703034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82703038: 4BD63DE9  bl 0x82466e20
	ctx.lr = 0x8270303C;
	sub_82466E20(ctx, base);
	// 8270303C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703050 size=112
    let mut pc: u32 = 0x82703050;
    'dispatch: loop {
        match pc {
            0x82703050 => {
    //   block [0x82703050..0x827030C0)
	// 82703050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270305C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703060: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82703064: 38AAF5BC  addi r5, r10, -0xa44
	ctx.r[5].s64 = ctx.r[10].s64 + -2628;
	// 82703068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270306C: 390B8AD0  addi r8, r11, -0x7530
	ctx.r[8].s64 = ctx.r[11].s64 + -30000;
	// 82703070: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82703074: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82703078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270307C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703088: 386AF5EC  addi r3, r10, -0xa14
	ctx.r[3].s64 = ctx.r[10].s64 + -2580;
	// 8270308C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270309C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827030A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827030A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827030A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827030AC: 4BD63D75  bl 0x82466e20
	ctx.lr = 0x827030B0;
	sub_82466E20(ctx, base);
	// 827030B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827030B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827030B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827030BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827030C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827030C0 size=108
    let mut pc: u32 = 0x827030C0;
    'dispatch: loop {
        match pc {
            0x827030C0 => {
    //   block [0x827030C0..0x8270312C)
	// 827030C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827030C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827030C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827030CC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827030D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827030D4: 38EB8B78  addi r7, r11, -0x7488
	ctx.r[7].s64 = ctx.r[11].s64 + -29832;
	// 827030D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 827030DC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 827030E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827030E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827030E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827030EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827030F0: 386AF61C  addi r3, r10, -0x9e4
	ctx.r[3].s64 = ctx.r[10].s64 + -2532;
	// 827030F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827030F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827030FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270310C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82703110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703114: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82703118: 4BD63D09  bl 0x82466e20
	ctx.lr = 0x8270311C;
	sub_82466E20(ctx, base);
	// 8270311C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703130 size=112
    let mut pc: u32 = 0x82703130;
    'dispatch: loop {
        match pc {
            0x82703130 => {
    //   block [0x82703130..0x827031A0)
	// 82703130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270313C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703140: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82703144: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82703148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270314C: 390B8BA8  addi r8, r11, -0x7458
	ctx.r[8].s64 = ctx.r[11].s64 + -29784;
	// 82703150: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82703154: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82703158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270315C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703168: 386AF64C  addi r3, r10, -0x9b4
	ctx.r[3].s64 = ctx.r[10].s64 + -2484;
	// 8270316C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270317C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82703188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270318C: 4BD63C95  bl 0x82466e20
	ctx.lr = 0x82703190;
	sub_82466E20(ctx, base);
	// 82703190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270319C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827031A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827031A0 size=112
    let mut pc: u32 = 0x827031A0;
    'dispatch: loop {
        match pc {
            0x827031A0 => {
    //   block [0x827031A0..0x82703210)
	// 827031A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827031A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827031A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827031AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827031B0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827031B4: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827031B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827031BC: 390B8BF0  addi r8, r11, -0x7410
	ctx.r[8].s64 = ctx.r[11].s64 + -29712;
	// 827031C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 827031C4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 827031C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827031CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827031D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827031D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827031D8: 386AF67C  addi r3, r10, -0x984
	ctx.r[3].s64 = ctx.r[10].s64 + -2436;
	// 827031DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827031E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827031E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827031E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827031EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827031F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827031F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827031F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827031FC: 4BD63C25  bl 0x82466e20
	ctx.lr = 0x82703200;
	sub_82466E20(ctx, base);
	// 82703200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270320C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703210 size=100
    let mut pc: u32 = 0x82703210;
    'dispatch: loop {
        match pc {
            0x82703210 => {
    //   block [0x82703210..0x82703274)
	// 82703210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270321C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703224: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82703228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270322C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703230: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82703234: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270323C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703244: 386AF6AC  addi r3, r10, -0x954
	ctx.r[3].s64 = ctx.r[10].s64 + -2388;
	// 82703248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270324C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703250: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82703254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703258: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270325C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82703260: 4BD63BC1  bl 0x82466e20
	ctx.lr = 0x82703264;
	sub_82466E20(ctx, base);
	// 82703264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270326C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703278 size=112
    let mut pc: u32 = 0x82703278;
    'dispatch: loop {
        match pc {
            0x82703278 => {
    //   block [0x82703278..0x827032E8)
	// 82703278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270327C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703284: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703288: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270328C: 38AAF6AC  addi r5, r10, -0x954
	ctx.r[5].s64 = ctx.r[10].s64 + -2388;
	// 82703290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82703294: 390B8C38  addi r8, r11, -0x73c8
	ctx.r[8].s64 = ctx.r[11].s64 + -29640;
	// 82703298: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270329C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 827032A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827032A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827032A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827032AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827032B0: 386AF6DC  addi r3, r10, -0x924
	ctx.r[3].s64 = ctx.r[10].s64 + -2340;
	// 827032B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827032B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827032BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827032C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827032C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827032C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827032CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827032D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827032D4: 4BD63B4D  bl 0x82466e20
	ctx.lr = 0x827032D8;
	sub_82466E20(ctx, base);
	// 827032D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827032DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827032E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827032E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827032E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827032E8 size=112
    let mut pc: u32 = 0x827032E8;
    'dispatch: loop {
        match pc {
            0x827032E8 => {
    //   block [0x827032E8..0x82703358)
	// 827032E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827032EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827032F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827032F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827032F8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827032FC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82703300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82703304: 390B8C80  addi r8, r11, -0x7380
	ctx.r[8].s64 = ctx.r[11].s64 + -29568;
	// 82703308: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270330C: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82703310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703314: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270331C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703320: 386AF70C  addi r3, r10, -0x8f4
	ctx.r[3].s64 = ctx.r[10].s64 + -2292;
	// 82703324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270332C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270333C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82703340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703344: 4BD63ADD  bl 0x82466e20
	ctx.lr = 0x82703348;
	sub_82466E20(ctx, base);
	// 82703348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270334C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703358 size=112
    let mut pc: u32 = 0x82703358;
    'dispatch: loop {
        match pc {
            0x82703358 => {
    //   block [0x82703358..0x827033C8)
	// 82703358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270335C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703364: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703368: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270336C: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82703370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82703374: 390B8C98  addi r8, r11, -0x7368
	ctx.r[8].s64 = ctx.r[11].s64 + -29544;
	// 82703378: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270337C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82703380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703384: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270338C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703390: 386AF73C  addi r3, r10, -0x8c4
	ctx.r[3].s64 = ctx.r[10].s64 + -2244;
	// 82703394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270339C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827033A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827033A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827033A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827033AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827033B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827033B4: 4BD63A6D  bl 0x82466e20
	ctx.lr = 0x827033B8;
	sub_82466E20(ctx, base);
	// 827033B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827033BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827033C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827033C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827033C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827033C8 size=112
    let mut pc: u32 = 0x827033C8;
    'dispatch: loop {
        match pc {
            0x827033C8 => {
    //   block [0x827033C8..0x82703438)
	// 827033C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827033CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827033D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827033D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827033D8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827033DC: 38AAF70C  addi r5, r10, -0x8f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2292;
	// 827033E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827033E4: 390B8CB0  addi r8, r11, -0x7350
	ctx.r[8].s64 = ctx.r[11].s64 + -29520;
	// 827033E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 827033EC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 827033F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827033F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827033F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827033FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703400: 386AF76C  addi r3, r10, -0x894
	ctx.r[3].s64 = ctx.r[10].s64 + -2196;
	// 82703404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270340C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270341C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82703420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703424: 4BD639FD  bl 0x82466e20
	ctx.lr = 0x82703428;
	sub_82466E20(ctx, base);
	// 82703428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270342C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703438 size=72
    let mut pc: u32 = 0x82703438;
    'dispatch: loop {
        match pc {
            0x82703438 => {
    //   block [0x82703438..0x82703480)
	// 82703438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270343C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703444: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703448: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8270344C: 38CBB360  addi r6, r11, -0x4ca0
	ctx.r[6].s64 = ctx.r[11].s64 + -19616;
	// 82703450: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703454: 388BC378  addi r4, r11, -0x3c88
	ctx.r[4].s64 = ctx.r[11].s64 + -15496;
	// 82703458: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8270345C: 386BF79C  addi r3, r11, -0x864
	ctx.r[3].s64 = ctx.r[11].s64 + -2148;
	// 82703460: 4BD78629  bl 0x8247ba88
	ctx.lr = 0x82703464;
	sub_8247BA88(ctx, base);
	// 82703464: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82703468: 386BCF60  addi r3, r11, -0x30a0
	ctx.r[3].s64 = ctx.r[11].s64 + -12448;
	// 8270346C: 4BE2F6CD  bl 0x82532b38
	ctx.lr = 0x82703470;
	sub_82532B38(ctx, base);
	// 82703470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82703474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270347C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703480 size=108
    let mut pc: u32 = 0x82703480;
    'dispatch: loop {
        match pc {
            0x82703480 => {
    //   block [0x82703480..0x827034EC)
	// 82703480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270348C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703490: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703494: 38EBCD94  addi r7, r11, -0x326c
	ctx.r[7].s64 = ctx.r[11].s64 + -12908;
	// 82703498: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8270349C: 388AC408  addi r4, r10, -0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + -15352;
	// 827034A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827034A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827034A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827034AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827034B0: 386AF7B4  addi r3, r10, -0x84c
	ctx.r[3].s64 = ctx.r[10].s64 + -2124;
	// 827034B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827034B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827034BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827034C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827034C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827034C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827034CC: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 827034D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827034D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827034D8: 4BD63949  bl 0x82466e20
	ctx.lr = 0x827034DC;
	sub_82466E20(ctx, base);
	// 827034DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827034E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827034E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827034E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827034F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827034F0 size=112
    let mut pc: u32 = 0x827034F0;
    'dispatch: loop {
        match pc {
            0x827034F0 => {
    //   block [0x827034F0..0x82703560)
	// 827034F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827034F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827034F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827034FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703500: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703504: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82703508: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270350C: 390BCDC4  addi r8, r11, -0x323c
	ctx.r[8].s64 = ctx.r[11].s64 + -12860;
	// 82703510: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82703514: 388AC428  addi r4, r10, -0x3bd8
	ctx.r[4].s64 = ctx.r[10].s64 + -15320;
	// 82703518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270351C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703528: 386AF7E4  addi r3, r10, -0x81c
	ctx.r[3].s64 = ctx.r[10].s64 + -2076;
	// 8270352C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270353C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703544: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 82703548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270354C: 4BD638D5  bl 0x82466e20
	ctx.lr = 0x82703550;
	sub_82466E20(ctx, base);
	// 82703550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270355C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82703560 size=24
    let mut pc: u32 = 0x82703560;
    'dispatch: loop {
        match pc {
            0x82703560 => {
    //   block [0x82703560..0x82703578)
	// 82703560: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82703564: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82703568: 394A9F1C  addi r10, r10, -0x60e4
	ctx.r[10].s64 = ctx.r[10].s64 + -24804;
	// 8270356C: 816B9D30  lwz r11, -0x62d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25296 as u32) ) } as u64;
	// 82703570: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82703574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703578 size=112
    let mut pc: u32 = 0x82703578;
    'dispatch: loop {
        match pc {
            0x82703578 => {
    //   block [0x82703578..0x827035E8)
	// 82703578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270357C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703584: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82703588: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270358C: 392ACF18  addi r9, r10, -0x30e8
	ctx.r[9].s64 = ctx.r[10].s64 + -12520;
	// 82703590: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703594: 390B9F1C  addi r8, r11, -0x60e4
	ctx.r[8].s64 = ctx.r[11].s64 + -24804;
	// 82703598: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8270359C: 388ABA94  addi r4, r10, -0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + -17772;
	// 827035A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827035A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827035A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827035AC: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 827035B0: 386AF814  addi r3, r10, -0x7ec
	ctx.r[3].s64 = ctx.r[10].s64 + -2028;
	// 827035B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827035B8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 827035BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827035C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827035C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827035C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827035CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827035D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827035D4: 4BD6384D  bl 0x82466e20
	ctx.lr = 0x827035D8;
	sub_82466E20(ctx, base);
	// 827035D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827035DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827035E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827035E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827035E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827035E8 size=112
    let mut pc: u32 = 0x827035E8;
    'dispatch: loop {
        match pc {
            0x827035E8 => {
    //   block [0x827035E8..0x82703658)
	// 827035E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827035EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827035F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827035F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827035F8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827035FC: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703600: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703604: 390BCF40  addi r8, r11, -0x30c0
	ctx.r[8].s64 = ctx.r[11].s64 + -12480;
	// 82703608: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270360C: 388ABAA8  addi r4, r10, -0x4558
	ctx.r[4].s64 = ctx.r[10].s64 + -17752;
	// 82703610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703614: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270361C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703620: 386AF844  addi r3, r10, -0x7bc
	ctx.r[3].s64 = ctx.r[10].s64 + -1980;
	// 82703624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270362C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270363C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82703640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703644: 4BD637DD  bl 0x82466e20
	ctx.lr = 0x82703648;
	sub_82466E20(ctx, base);
	// 82703648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270364C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703658 size=108
    let mut pc: u32 = 0x82703658;
    'dispatch: loop {
        match pc {
            0x82703658 => {
    //   block [0x82703658..0x827036C4)
	// 82703658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270365C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703664: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703668: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270366C: 38EBCF70  addi r7, r11, -0x3090
	ctx.r[7].s64 = ctx.r[11].s64 + -12432;
	// 82703670: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82703674: 388ABAC0  addi r4, r10, -0x4540
	ctx.r[4].s64 = ctx.r[10].s64 + -17728;
	// 82703678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270367C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82703684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703688: 386AF874  addi r3, r10, -0x78c
	ctx.r[3].s64 = ctx.r[10].s64 + -1932;
	// 8270368C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82703690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270369C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827036A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827036A4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 827036A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827036AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827036B0: 4BD63771  bl 0x82466e20
	ctx.lr = 0x827036B4;
	sub_82466E20(ctx, base);
	// 827036B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827036B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827036BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827036C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827036C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827036C8 size=112
    let mut pc: u32 = 0x827036C8;
    'dispatch: loop {
        match pc {
            0x827036C8 => {
    //   block [0x827036C8..0x82703738)
	// 827036C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827036CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827036D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827036D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827036D8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827036DC: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 827036E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827036E4: 390BCF88  addi r8, r11, -0x3078
	ctx.r[8].s64 = ctx.r[11].s64 + -12408;
	// 827036E8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 827036EC: 388ABAD0  addi r4, r10, -0x4530
	ctx.r[4].s64 = ctx.r[10].s64 + -17712;
	// 827036F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827036F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827036F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827036FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703700: 386AF8A4  addi r3, r10, -0x75c
	ctx.r[3].s64 = ctx.r[10].s64 + -1884;
	// 82703704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270370C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270371C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82703720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703724: 4BD636FD  bl 0x82466e20
	ctx.lr = 0x82703728;
	sub_82466E20(ctx, base);
	// 82703728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270372C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703738 size=100
    let mut pc: u32 = 0x82703738;
    'dispatch: loop {
        match pc {
            0x82703738 => {
    //   block [0x82703738..0x8270379C)
	// 82703738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270373C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703744: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270374C: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703750: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703758: 388ABAF0  addi r4, r10, -0x4510
	ctx.r[4].s64 = ctx.r[10].s64 + -17680;
	// 8270375C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270376C: 386AF8D4  addi r3, r10, -0x72c
	ctx.r[3].s64 = ctx.r[10].s64 + -1836;
	// 82703770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703774: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703778: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270377C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703780: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82703784: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82703788: 4BD63699  bl 0x82466e20
	ctx.lr = 0x8270378C;
	sub_82466E20(ctx, base);
	// 8270378C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827037A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827037A0 size=112
    let mut pc: u32 = 0x827037A0;
    'dispatch: loop {
        match pc {
            0x827037A0 => {
    //   block [0x827037A0..0x82703810)
	// 827037A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827037A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827037A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827037AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827037B0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827037B4: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 827037B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827037BC: 390BD048  addi r8, r11, -0x2fb8
	ctx.r[8].s64 = ctx.r[11].s64 + -12216;
	// 827037C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827037C4: 388ABB0C  addi r4, r10, -0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17652;
	// 827037C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827037CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827037D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827037D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827037D8: 386AF904  addi r3, r10, -0x6fc
	ctx.r[3].s64 = ctx.r[10].s64 + -1788;
	// 827037DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827037E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827037E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827037E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827037EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827037F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827037F4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 827037F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827037FC: 4BD63625  bl 0x82466e20
	ctx.lr = 0x82703800;
	sub_82466E20(ctx, base);
	// 82703800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270380C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703810 size=112
    let mut pc: u32 = 0x82703810;
    'dispatch: loop {
        match pc {
            0x82703810 => {
    //   block [0x82703810..0x82703880)
	// 82703810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270381C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703820: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703824: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703828: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270382C: 390BD060  addi r8, r11, -0x2fa0
	ctx.r[8].s64 = ctx.r[11].s64 + -12192;
	// 82703830: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82703834: 388ABB2C  addi r4, r10, -0x44d4
	ctx.r[4].s64 = ctx.r[10].s64 + -17620;
	// 82703838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270383C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703848: 386AF934  addi r3, r10, -0x6cc
	ctx.r[3].s64 = ctx.r[10].s64 + -1740;
	// 8270384C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270385C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703864: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 82703868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270386C: 4BD635B5  bl 0x82466e20
	ctx.lr = 0x82703870;
	sub_82466E20(ctx, base);
	// 82703870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270387C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703880 size=112
    let mut pc: u32 = 0x82703880;
    'dispatch: loop {
        match pc {
            0x82703880 => {
    //   block [0x82703880..0x827038F0)
	// 82703880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270388C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703890: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703894: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703898: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270389C: 390BD090  addi r8, r11, -0x2f70
	ctx.r[8].s64 = ctx.r[11].s64 + -12144;
	// 827038A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827038A4: 388ABB50  addi r4, r10, -0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + -17584;
	// 827038A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827038AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827038B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827038B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827038B8: 386AF964  addi r3, r10, -0x69c
	ctx.r[3].s64 = ctx.r[10].s64 + -1692;
	// 827038BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827038C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827038C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827038C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827038CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827038D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827038D4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 827038D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827038DC: 4BD63545  bl 0x82466e20
	ctx.lr = 0x827038E0;
	sub_82466E20(ctx, base);
	// 827038E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827038E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827038E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827038EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827038F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827038F0 size=112
    let mut pc: u32 = 0x827038F0;
    'dispatch: loop {
        match pc {
            0x827038F0 => {
    //   block [0x827038F0..0x82703960)
	// 827038F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827038F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827038F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827038FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703900: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703904: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703908: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270390C: 390BD0C0  addi r8, r11, -0x2f40
	ctx.r[8].s64 = ctx.r[11].s64 + -12096;
	// 82703910: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82703914: 388ABB78  addi r4, r10, -0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + -17544;
	// 82703918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270391C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703928: 386AF994  addi r3, r10, -0x66c
	ctx.r[3].s64 = ctx.r[10].s64 + -1644;
	// 8270392C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270393C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703944: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 82703948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270394C: 4BD634D5  bl 0x82466e20
	ctx.lr = 0x82703950;
	sub_82466E20(ctx, base);
	// 82703950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270395C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703960 size=112
    let mut pc: u32 = 0x82703960;
    'dispatch: loop {
        match pc {
            0x82703960 => {
    //   block [0x82703960..0x827039D0)
	// 82703960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270396C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703970: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703974: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270397C: 390BD0F0  addi r8, r11, -0x2f10
	ctx.r[8].s64 = ctx.r[11].s64 + -12048;
	// 82703980: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82703984: 388ABB9C  addi r4, r10, -0x4464
	ctx.r[4].s64 = ctx.r[10].s64 + -17508;
	// 82703988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270398C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703998: 386AF9C4  addi r3, r10, -0x63c
	ctx.r[3].s64 = ctx.r[10].s64 + -1596;
	// 8270399C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827039A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827039A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827039A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827039AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827039B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827039B4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 827039B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827039BC: 4BD63465  bl 0x82466e20
	ctx.lr = 0x827039C0;
	sub_82466E20(ctx, base);
	// 827039C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827039C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827039C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827039CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827039D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827039D0 size=112
    let mut pc: u32 = 0x827039D0;
    'dispatch: loop {
        match pc {
            0x827039D0 => {
    //   block [0x827039D0..0x82703A40)
	// 827039D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827039D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827039D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827039DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827039E0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827039E4: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 827039E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827039EC: 390BD108  addi r8, r11, -0x2ef8
	ctx.r[8].s64 = ctx.r[11].s64 + -12024;
	// 827039F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827039F4: 388ABBBC  addi r4, r10, -0x4444
	ctx.r[4].s64 = ctx.r[10].s64 + -17476;
	// 827039F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827039FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703A00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703A08: 386AF9F4  addi r3, r10, -0x60c
	ctx.r[3].s64 = ctx.r[10].s64 + -1548;
	// 82703A0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703A24: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82703A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703A2C: 4BD633F5  bl 0x82466e20
	ctx.lr = 0x82703A30;
	sub_82466E20(ctx, base);
	// 82703A30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703A40 size=112
    let mut pc: u32 = 0x82703A40;
    'dispatch: loop {
        match pc {
            0x82703A40 => {
    //   block [0x82703A40..0x82703AB0)
	// 82703A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703A4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703A50: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703A54: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703A58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703A5C: 390BD120  addi r8, r11, -0x2ee0
	ctx.r[8].s64 = ctx.r[11].s64 + -12000;
	// 82703A60: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82703A64: 388ABBD4  addi r4, r10, -0x442c
	ctx.r[4].s64 = ctx.r[10].s64 + -17452;
	// 82703A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703A6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703A70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703A78: 386AFA24  addi r3, r10, -0x5dc
	ctx.r[3].s64 = ctx.r[10].s64 + -1500;
	// 82703A7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703A94: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82703A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703A9C: 4BD63385  bl 0x82466e20
	ctx.lr = 0x82703AA0;
	sub_82466E20(ctx, base);
	// 82703AA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703AB0 size=112
    let mut pc: u32 = 0x82703AB0;
    'dispatch: loop {
        match pc {
            0x82703AB0 => {
    //   block [0x82703AB0..0x82703B20)
	// 82703AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703ABC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703AC0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703AC4: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703AC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703ACC: 390BD168  addi r8, r11, -0x2e98
	ctx.r[8].s64 = ctx.r[11].s64 + -11928;
	// 82703AD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82703AD4: 388ABBF0  addi r4, r10, -0x4410
	ctx.r[4].s64 = ctx.r[10].s64 + -17424;
	// 82703AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703ADC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703AE8: 386AFA54  addi r3, r10, -0x5ac
	ctx.r[3].s64 = ctx.r[10].s64 + -1452;
	// 82703AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703B04: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82703B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703B0C: 4BD63315  bl 0x82466e20
	ctx.lr = 0x82703B10;
	sub_82466E20(ctx, base);
	// 82703B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703B20 size=112
    let mut pc: u32 = 0x82703B20;
    'dispatch: loop {
        match pc {
            0x82703B20 => {
    //   block [0x82703B20..0x82703B90)
	// 82703B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703B2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703B30: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703B34: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703B38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703B3C: 390BD1B0  addi r8, r11, -0x2e50
	ctx.r[8].s64 = ctx.r[11].s64 + -11856;
	// 82703B40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82703B44: 388ABC0C  addi r4, r10, -0x43f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17396;
	// 82703B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703B4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703B50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703B58: 386AFA84  addi r3, r10, -0x57c
	ctx.r[3].s64 = ctx.r[10].s64 + -1404;
	// 82703B5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703B74: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82703B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703B7C: 4BD632A5  bl 0x82466e20
	ctx.lr = 0x82703B80;
	sub_82466E20(ctx, base);
	// 82703B80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703B90 size=112
    let mut pc: u32 = 0x82703B90;
    'dispatch: loop {
        match pc {
            0x82703B90 => {
    //   block [0x82703B90..0x82703C00)
	// 82703B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703B9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703BA0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703BA4: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703BA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703BAC: 390BD1C8  addi r8, r11, -0x2e38
	ctx.r[8].s64 = ctx.r[11].s64 + -11832;
	// 82703BB0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82703BB4: 388ABC24  addi r4, r10, -0x43dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17372;
	// 82703BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703BBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703BC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703BC8: 386AFAB4  addi r3, r10, -0x54c
	ctx.r[3].s64 = ctx.r[10].s64 + -1356;
	// 82703BCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703BE4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82703BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703BEC: 4BD63235  bl 0x82466e20
	ctx.lr = 0x82703BF0;
	sub_82466E20(ctx, base);
	// 82703BF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703C00 size=112
    let mut pc: u32 = 0x82703C00;
    'dispatch: loop {
        match pc {
            0x82703C00 => {
    //   block [0x82703C00..0x82703C70)
	// 82703C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703C0C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703C10: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703C14: 396BD1F8  addi r11, r11, -0x2e08
	ctx.r[11].s64 = ctx.r[11].s64 + -11784;
	// 82703C18: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703C1C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703C20: 390B0078  addi r8, r11, 0x78
	ctx.r[8].s64 = ctx.r[11].s64 + 120;
	// 82703C24: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82703C28: 388ABC3C  addi r4, r10, -0x43c4
	ctx.r[4].s64 = ctx.r[10].s64 + -17348;
	// 82703C2C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82703C30: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703C34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703C38: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82703C3C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82703C40: 386AFAE4  addi r3, r10, -0x51c
	ctx.r[3].s64 = ctx.r[10].s64 + -1308;
	// 82703C44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82703C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703C50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82703C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703C58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82703C5C: 4BD631C5  bl 0x82466e20
	ctx.lr = 0x82703C60;
	sub_82466E20(ctx, base);
	// 82703C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703C70 size=112
    let mut pc: u32 = 0x82703C70;
    'dispatch: loop {
        match pc {
            0x82703C70 => {
    //   block [0x82703C70..0x82703CE0)
	// 82703C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703C7C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703C80: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703C84: 396BD288  addi r11, r11, -0x2d78
	ctx.r[11].s64 = ctx.r[11].s64 + -11640;
	// 82703C88: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703C8C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703C90: 390B0090  addi r8, r11, 0x90
	ctx.r[8].s64 = ctx.r[11].s64 + 144;
	// 82703C94: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82703C98: 388ABC58  addi r4, r10, -0x43a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17320;
	// 82703C9C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82703CA0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703CA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703CA8: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82703CAC: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82703CB0: 386AFB14  addi r3, r10, -0x4ec
	ctx.r[3].s64 = ctx.r[10].s64 + -1260;
	// 82703CB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82703CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703CC0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82703CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703CC8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82703CCC: 4BD63155  bl 0x82466e20
	ctx.lr = 0x82703CD0;
	sub_82466E20(ctx, base);
	// 82703CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82703CE0 size=24
    let mut pc: u32 = 0x82703CE0;
    'dispatch: loop {
        match pc {
            0x82703CE0 => {
    //   block [0x82703CE0..0x82703CF8)
	// 82703CE0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82703CE4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82703CE8: 394A9F38  addi r10, r10, -0x60c8
	ctx.r[10].s64 = ctx.r[10].s64 + -24776;
	// 82703CEC: 816B9D38  lwz r11, -0x62c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25288 as u32) ) } as u64;
	// 82703CF0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82703CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703CF8 size=116
    let mut pc: u32 = 0x82703CF8;
    'dispatch: loop {
        match pc {
            0x82703CF8 => {
    //   block [0x82703CF8..0x82703D6C)
	// 82703CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703D04: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703D08: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703D0C: 392BD344  addi r9, r11, -0x2cbc
	ctx.r[9].s64 = ctx.r[11].s64 + -11452;
	// 82703D10: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703D14: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703D18: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82703D1C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82703D20: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82703D24: 388ABC8C  addi r4, r10, -0x4374
	ctx.r[4].s64 = ctx.r[10].s64 + -17268;
	// 82703D28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703D2C: 396B9F38  addi r11, r11, -0x60c8
	ctx.r[11].s64 = ctx.r[11].s64 + -24776;
	// 82703D30: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82703D34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703D38: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82703D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703D40: 386AFB44  addi r3, r10, -0x4bc
	ctx.r[3].s64 = ctx.r[10].s64 + -1212;
	// 82703D44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82703D48: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82703D4C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82703D50: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82703D54: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82703D58: 4BD630C9  bl 0x82466e20
	ctx.lr = 0x82703D5C;
	sub_82466E20(ctx, base);
	// 82703D5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703D70 size=112
    let mut pc: u32 = 0x82703D70;
    'dispatch: loop {
        match pc {
            0x82703D70 => {
    //   block [0x82703D70..0x82703DE0)
	// 82703D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703D7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703D80: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703D84: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703D88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703D8C: 390BD380  addi r8, r11, -0x2c80
	ctx.r[8].s64 = ctx.r[11].s64 + -11392;
	// 82703D90: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82703D94: 388ABCA8  addi r4, r10, -0x4358
	ctx.r[4].s64 = ctx.r[10].s64 + -17240;
	// 82703D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703D9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703DA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703DA8: 386AFB74  addi r3, r10, -0x48c
	ctx.r[3].s64 = ctx.r[10].s64 + -1164;
	// 82703DAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703DC4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82703DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703DCC: 4BD63055  bl 0x82466e20
	ctx.lr = 0x82703DD0;
	sub_82466E20(ctx, base);
	// 82703DD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703DE0 size=112
    let mut pc: u32 = 0x82703DE0;
    'dispatch: loop {
        match pc {
            0x82703DE0 => {
    //   block [0x82703DE0..0x82703E50)
	// 82703DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703DEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703DF0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703DF4: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703DF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703DFC: 390BD3E0  addi r8, r11, -0x2c20
	ctx.r[8].s64 = ctx.r[11].s64 + -11296;
	// 82703E00: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82703E04: 388ABCC8  addi r4, r10, -0x4338
	ctx.r[4].s64 = ctx.r[10].s64 + -17208;
	// 82703E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703E0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703E10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703E18: 386AFBA4  addi r3, r10, -0x45c
	ctx.r[3].s64 = ctx.r[10].s64 + -1116;
	// 82703E1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703E20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703E34: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82703E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703E3C: 4BD62FE5  bl 0x82466e20
	ctx.lr = 0x82703E40;
	sub_82466E20(ctx, base);
	// 82703E40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703E50 size=112
    let mut pc: u32 = 0x82703E50;
    'dispatch: loop {
        match pc {
            0x82703E50 => {
    //   block [0x82703E50..0x82703EC0)
	// 82703E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703E5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703E60: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703E64: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703E68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703E6C: 390BD488  addi r8, r11, -0x2b78
	ctx.r[8].s64 = ctx.r[11].s64 + -11128;
	// 82703E70: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82703E74: 388ABCE4  addi r4, r10, -0x431c
	ctx.r[4].s64 = ctx.r[10].s64 + -17180;
	// 82703E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703E7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703E80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703E88: 386AFBD4  addi r3, r10, -0x42c
	ctx.r[3].s64 = ctx.r[10].s64 + -1068;
	// 82703E8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703E90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703E94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703E98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703EA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703EA4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82703EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703EAC: 4BD62F75  bl 0x82466e20
	ctx.lr = 0x82703EB0;
	sub_82466E20(ctx, base);
	// 82703EB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703EC0 size=112
    let mut pc: u32 = 0x82703EC0;
    'dispatch: loop {
        match pc {
            0x82703EC0 => {
    //   block [0x82703EC0..0x82703F30)
	// 82703EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703ECC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703ED0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703ED4: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703ED8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703EDC: 390BD500  addi r8, r11, -0x2b00
	ctx.r[8].s64 = ctx.r[11].s64 + -11008;
	// 82703EE0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82703EE4: 388ABD04  addi r4, r10, -0x42fc
	ctx.r[4].s64 = ctx.r[10].s64 + -17148;
	// 82703EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703EEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703EF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703EF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703EF8: 386AFC04  addi r3, r10, -0x3fc
	ctx.r[3].s64 = ctx.r[10].s64 + -1020;
	// 82703EFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703F14: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82703F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703F1C: 4BD62F05  bl 0x82466e20
	ctx.lr = 0x82703F20;
	sub_82466E20(ctx, base);
	// 82703F20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703F30 size=112
    let mut pc: u32 = 0x82703F30;
    'dispatch: loop {
        match pc {
            0x82703F30 => {
    //   block [0x82703F30..0x82703FA0)
	// 82703F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703F3C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703F40: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703F44: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703F48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703F4C: 390BD548  addi r8, r11, -0x2ab8
	ctx.r[8].s64 = ctx.r[11].s64 + -10936;
	// 82703F50: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82703F54: 388ABD24  addi r4, r10, -0x42dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17116;
	// 82703F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703F5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703F68: 386AFC34  addi r3, r10, -0x3cc
	ctx.r[3].s64 = ctx.r[10].s64 + -972;
	// 82703F6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703F84: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82703F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703F8C: 4BD62E95  bl 0x82466e20
	ctx.lr = 0x82703F90;
	sub_82466E20(ctx, base);
	// 82703F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703FA0 size=112
    let mut pc: u32 = 0x82703FA0;
    'dispatch: loop {
        match pc {
            0x82703FA0 => {
    //   block [0x82703FA0..0x82704010)
	// 82703FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703FAC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703FB0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703FB4: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703FB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703FBC: 390BD5D8  addi r8, r11, -0x2a28
	ctx.r[8].s64 = ctx.r[11].s64 + -10792;
	// 82703FC0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82703FC4: 388ABD40  addi r4, r10, -0x42c0
	ctx.r[4].s64 = ctx.r[10].s64 + -17088;
	// 82703FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703FCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703FD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703FD8: 386AFC64  addi r3, r10, -0x39c
	ctx.r[3].s64 = ctx.r[10].s64 + -924;
	// 82703FDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703FE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703FE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703FEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703FF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703FF4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82703FF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703FFC: 4BD62E25  bl 0x82466e20
	ctx.lr = 0x82704000;
	sub_82466E20(ctx, base);
	// 82704000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270400C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704010 size=112
    let mut pc: u32 = 0x82704010;
    'dispatch: loop {
        match pc {
            0x82704010 => {
    //   block [0x82704010..0x82704080)
	// 82704010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270401C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704020: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704024: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82704028: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270402C: 390BD638  addi r8, r11, -0x29c8
	ctx.r[8].s64 = ctx.r[11].s64 + -10696;
	// 82704030: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82704034: 388ABD58  addi r4, r10, -0x42a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17064;
	// 82704038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270403C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704040: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704048: 386AFC94  addi r3, r10, -0x36c
	ctx.r[3].s64 = ctx.r[10].s64 + -876;
	// 8270404C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270405C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704064: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82704068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270406C: 4BD62DB5  bl 0x82466e20
	ctx.lr = 0x82704070;
	sub_82466E20(ctx, base);
	// 82704070: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270407C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704080 size=112
    let mut pc: u32 = 0x82704080;
    'dispatch: loop {
        match pc {
            0x82704080 => {
    //   block [0x82704080..0x827040F0)
	// 82704080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270408C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704090: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704094: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82704098: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270409C: 390BD698  addi r8, r11, -0x2968
	ctx.r[8].s64 = ctx.r[11].s64 + -10600;
	// 827040A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827040A4: 388ABD74  addi r4, r10, -0x428c
	ctx.r[4].s64 = ctx.r[10].s64 + -17036;
	// 827040A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827040AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827040B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827040B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827040B8: 386AFCC4  addi r3, r10, -0x33c
	ctx.r[3].s64 = ctx.r[10].s64 + -828;
	// 827040BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827040C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827040C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827040C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827040CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827040D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827040D4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 827040D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827040DC: 4BD62D45  bl 0x82466e20
	ctx.lr = 0x827040E0;
	sub_82466E20(ctx, base);
	// 827040E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827040E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827040E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827040EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827040F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827040F0 size=112
    let mut pc: u32 = 0x827040F0;
    'dispatch: loop {
        match pc {
            0x827040F0 => {
    //   block [0x827040F0..0x82704160)
	// 827040F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827040F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827040F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827040FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704100: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704104: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82704108: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270410C: 390BD6C8  addi r8, r11, -0x2938
	ctx.r[8].s64 = ctx.r[11].s64 + -10552;
	// 82704110: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82704114: 388ABD9C  addi r4, r10, -0x4264
	ctx.r[4].s64 = ctx.r[10].s64 + -16996;
	// 82704118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270411C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704128: 386AFCF4  addi r3, r10, -0x30c
	ctx.r[3].s64 = ctx.r[10].s64 + -780;
	// 8270412C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270413C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704144: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82704148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270414C: 4BD62CD5  bl 0x82466e20
	ctx.lr = 0x82704150;
	sub_82466E20(ctx, base);
	// 82704150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270415C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704160 size=100
    let mut pc: u32 = 0x82704160;
    'dispatch: loop {
        match pc {
            0x82704160 => {
    //   block [0x82704160..0x827041C4)
	// 82704160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270416C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704174: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82704178: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270417C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704180: 388ABDC4  addi r4, r10, -0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + -16956;
	// 82704184: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270418C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704194: 386AFD24  addi r3, r10, -0x2dc
	ctx.r[3].s64 = ctx.r[10].s64 + -732;
	// 82704198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270419C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827041A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827041A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827041A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827041AC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 827041B0: 4BD62C71  bl 0x82466e20
	ctx.lr = 0x827041B4;
	sub_82466E20(ctx, base);
	// 827041B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827041B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827041BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827041C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827041C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827041C8 size=112
    let mut pc: u32 = 0x827041C8;
    'dispatch: loop {
        match pc {
            0x827041C8 => {
    //   block [0x827041C8..0x82704238)
	// 827041C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827041CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827041D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827041D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827041D8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827041DC: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 827041E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827041E4: 390BD710  addi r8, r11, -0x28f0
	ctx.r[8].s64 = ctx.r[11].s64 + -10480;
	// 827041E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827041EC: 388ABDEC  addi r4, r10, -0x4214
	ctx.r[4].s64 = ctx.r[10].s64 + -16916;
	// 827041F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827041F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827041F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827041FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704200: 386AFD54  addi r3, r10, -0x2ac
	ctx.r[3].s64 = ctx.r[10].s64 + -684;
	// 82704204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270420C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270421C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82704220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704224: 4BD62BFD  bl 0x82466e20
	ctx.lr = 0x82704228;
	sub_82466E20(ctx, base);
	// 82704228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270422C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704238 size=100
    let mut pc: u32 = 0x82704238;
    'dispatch: loop {
        match pc {
            0x82704238 => {
    //   block [0x82704238..0x8270429C)
	// 82704238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270423C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704244: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270424C: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82704250: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82704254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704258: 388AB3BC  addi r4, r10, -0x4c44
	ctx.r[4].s64 = ctx.r[10].s64 + -19524;
	// 8270425C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270426C: 386AFD84  addi r3, r10, -0x27c
	ctx.r[3].s64 = ctx.r[10].s64 + -636;
	// 82704270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704274: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704278: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270427C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704280: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82704284: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82704288: 4BD62B99  bl 0x82466e20
	ctx.lr = 0x8270428C;
	sub_82466E20(ctx, base);
	// 8270428C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827042A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827042A0 size=108
    let mut pc: u32 = 0x827042A0;
    'dispatch: loop {
        match pc {
            0x827042A0 => {
    //   block [0x827042A0..0x8270430C)
	// 827042A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827042A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827042A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827042AC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827042B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827042B4: 392BD748  addi r9, r11, -0x28b8
	ctx.r[9].s64 = ctx.r[11].s64 + -10424;
	// 827042B8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 827042BC: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 827042C0: 388AC0D8  addi r4, r10, -0x3f28
	ctx.r[4].s64 = ctx.r[10].s64 + -16168;
	// 827042C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827042C8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827042CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827042D0: 38C00130  li r6, 0x130
	ctx.r[6].s64 = 304;
	// 827042D4: 386AFDB4  addi r3, r10, -0x24c
	ctx.r[3].s64 = ctx.r[10].s64 + -588;
	// 827042D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827042DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827042E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827042E4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827042E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827042EC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827042F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827042F4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827042F8: 4BD62B29  bl 0x82466e20
	ctx.lr = 0x827042FC;
	sub_82466E20(ctx, base);
	// 827042FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704310 size=112
    let mut pc: u32 = 0x82704310;
    'dispatch: loop {
        match pc {
            0x82704310 => {
    //   block [0x82704310..0x82704380)
	// 82704310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270431C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704320: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704324: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82704328: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270432C: 390BD808  addi r8, r11, -0x27f8
	ctx.r[8].s64 = ctx.r[11].s64 + -10232;
	// 82704330: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82704334: 388AC0F4  addi r4, r10, -0x3f0c
	ctx.r[4].s64 = ctx.r[10].s64 + -16140;
	// 82704338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270433C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704348: 386AFDE4  addi r3, r10, -0x21c
	ctx.r[3].s64 = ctx.r[10].s64 + -540;
	// 8270434C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270435C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704364: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 82704368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270436C: 4BD62AB5  bl 0x82466e20
	ctx.lr = 0x82704370;
	sub_82466E20(ctx, base);
	// 82704370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270437C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704380 size=100
    let mut pc: u32 = 0x82704380;
    'dispatch: loop {
        match pc {
            0x82704380 => {
    //   block [0x82704380..0x827043E4)
	// 82704380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270438C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704394: 38AA04E4  addi r5, r10, 0x4e4
	ctx.r[5].s64 = ctx.r[10].s64 + 1252;
	// 82704398: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270439C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827043A0: 388AC5F0  addi r4, r10, -0x3a10
	ctx.r[4].s64 = ctx.r[10].s64 + -14864;
	// 827043A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827043A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827043AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827043B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827043B4: 386AFE14  addi r3, r10, -0x1ec
	ctx.r[3].s64 = ctx.r[10].s64 + -492;
	// 827043B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827043BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827043C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827043C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827043C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827043CC: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 827043D0: 4BD62A51  bl 0x82466e20
	ctx.lr = 0x827043D4;
	sub_82466E20(ctx, base);
	// 827043D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827043D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827043DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827043E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827043E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827043E8 size=112
    let mut pc: u32 = 0x827043E8;
    'dispatch: loop {
        match pc {
            0x827043E8 => {
    //   block [0x827043E8..0x82704458)
	// 827043E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827043EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827043F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827043F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827043F8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827043FC: 38AA0C48  addi r5, r10, 0xc48
	ctx.r[5].s64 = ctx.r[10].s64 + 3144;
	// 82704400: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704404: 390BD884  addi r8, r11, -0x277c
	ctx.r[8].s64 = ctx.r[11].s64 + -10108;
	// 82704408: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270440C: 388AC19C  addi r4, r10, -0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + -15972;
	// 82704410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704414: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704418: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270441C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704420: 386AFE44  addi r3, r10, -0x1bc
	ctx.r[3].s64 = ctx.r[10].s64 + -444;
	// 82704424: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270442C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270443C: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 82704440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704444: 4BD629DD  bl 0x82466e20
	ctx.lr = 0x82704448;
	sub_82466E20(ctx, base);
	// 82704448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270444C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704458 size=92
    let mut pc: u32 = 0x82704458;
    'dispatch: loop {
        match pc {
            0x82704458 => {
    //   block [0x82704458..0x827044B4)
	// 82704458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270445C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704460: 9421FC90  stwu r1, -0x370(r1)
	ea = ctx.r[1].u32.wrapping_add(-880 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704464: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82704468: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8270446C: 4BD92C5D  bl 0x824970c8
	ctx.lr = 0x82704470;
	sub_824970C8(ctx, base);
	// 82704470: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82704474: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 82704478: 394BC6B8  addi r10, r11, -0x3948
	ctx.r[10].s64 = ctx.r[11].s64 + -14664;
	// 8270447C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82704480: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 82704484: 396BFE74  addi r11, r11, -0x18c
	ctx.r[11].s64 = ctx.r[11].s64 + -396;
	// 82704488: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8270448C: 394810D8  addi r10, r8, 0x10d8
	ctx.r[10].s64 = ctx.r[8].s64 + 4312;
	// 82704490: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82704494: 394910F0  addi r10, r9, 0x10f0
	ctx.r[10].s64 = ctx.r[9].s64 + 4336;
	// 82704498: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8270449C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827044A0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827044A4: 38210370  addi r1, r1, 0x370
	ctx.r[1].s64 = ctx.r[1].s64 + 880;
	// 827044A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827044AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827044B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827044B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827044B8 size=116
    let mut pc: u32 = 0x827044B8;
    'dispatch: loop {
        match pc {
            0x827044B8 => {
    //   block [0x827044B8..0x8270452C)
	// 827044B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827044BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827044C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827044C4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827044C8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827044CC: 392ADA18  addi r9, r10, -0x25e8
	ctx.r[9].s64 = ctx.r[10].s64 + -9704;
	// 827044D0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 827044D4: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 827044D8: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 827044DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827044E0: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 827044E4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827044E8: 388AC6B8  addi r4, r10, -0x3948
	ctx.r[4].s64 = ctx.r[10].s64 + -14664;
	// 827044EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827044F0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 827044F4: 396BDA40  addi r11, r11, -0x25c0
	ctx.r[11].s64 = ctx.r[11].s64 + -9664;
	// 827044F8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827044FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704500: 386AFE84  addi r3, r10, -0x17c
	ctx.r[3].s64 = ctx.r[10].s64 + -380;
	// 82704504: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82704508: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8270450C: 38C00310  li r6, 0x310
	ctx.r[6].s64 = 784;
	// 82704510: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82704514: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82704518: 4BD62909  bl 0x82466e20
	ctx.lr = 0x8270451C;
	sub_82466E20(ctx, base);
	// 8270451C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704530 size=108
    let mut pc: u32 = 0x82704530;
    'dispatch: loop {
        match pc {
            0x82704530 => {
    //   block [0x82704530..0x8270459C)
	// 82704530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270453C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704540: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704544: 38EBE0A0  addi r7, r11, -0x1f60
	ctx.r[7].s64 = ctx.r[11].s64 + -8032;
	// 82704548: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8270454C: 388ABFD8  addi r4, r10, -0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + -16424;
	// 82704550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704554: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704558: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270455C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704560: 386AFEB4  addi r3, r10, -0x14c
	ctx.r[3].s64 = ctx.r[10].s64 + -332;
	// 82704564: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82704568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270456C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270457C: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 82704580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704584: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82704588: 4BD62899  bl 0x82466e20
	ctx.lr = 0x8270458C;
	sub_82466E20(ctx, base);
	// 8270458C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


