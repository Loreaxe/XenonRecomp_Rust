pub fn sub_832AAA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAA28 size=12
    let mut pc: u32 = 0x832AAA28;
    'dispatch: loop {
        match pc {
            0x832AAA28 => {
    //   block [0x832AAA28..0x832AAA34)
	// 832AAA28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAA2C: 386B8814  addi r3, r11, -0x77ec
	ctx.r[3].s64 = ctx.r[11].s64 + -30700;
	// 832AAA30: 4AF6A3A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAA38 size=12
    let mut pc: u32 = 0x832AAA38;
    'dispatch: loop {
        match pc {
            0x832AAA38 => {
    //   block [0x832AAA38..0x832AAA44)
	// 832AAA38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAA3C: 386B8818  addi r3, r11, -0x77e8
	ctx.r[3].s64 = ctx.r[11].s64 + -30696;
	// 832AAA40: 4AF6A398  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAA48 size=12
    let mut pc: u32 = 0x832AAA48;
    'dispatch: loop {
        match pc {
            0x832AAA48 => {
    //   block [0x832AAA48..0x832AAA54)
	// 832AAA48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAA4C: 386B881C  addi r3, r11, -0x77e4
	ctx.r[3].s64 = ctx.r[11].s64 + -30692;
	// 832AAA50: 4AF6A388  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAA58 size=12
    let mut pc: u32 = 0x832AAA58;
    'dispatch: loop {
        match pc {
            0x832AAA58 => {
    //   block [0x832AAA58..0x832AAA64)
	// 832AAA58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAA5C: 386B8820  addi r3, r11, -0x77e0
	ctx.r[3].s64 = ctx.r[11].s64 + -30688;
	// 832AAA60: 4AF6A378  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AAA68 size=72
    let mut pc: u32 = 0x832AAA68;
    'dispatch: loop {
        match pc {
            0x832AAA68 => {
    //   block [0x832AAA68..0x832AAAB0)
	// 832AAA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AAA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AAA70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AAA74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AAA78: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832AAA7C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832AAA80: 3BEA9F34  addi r31, r10, -0x60cc
	ctx.r[31].s64 = ctx.r[10].s64 + -24780;
	// 832AAA84: 396B0B7C  addi r11, r11, 0xb7c
	ctx.r[11].s64 = ctx.r[11].s64 + 2940;
	// 832AAA88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AAA8C: 916A9F34  stw r11, -0x60cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24780 as u32), ctx.r[11].u32 ) };
	// 832AAA90: 4AEE93A9  bl 0x82193e38
	ctx.lr = 0x832AAA94;
	sub_82193E38(ctx, base);
	// 832AAA94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AAA98: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AAA9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AAAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AAAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AAAA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AAAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AAAB0 size=72
    let mut pc: u32 = 0x832AAAB0;
    'dispatch: loop {
        match pc {
            0x832AAAB0 => {
    //   block [0x832AAAB0..0x832AAAF8)
	// 832AAAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AAAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AAAB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AAABC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AAAC0: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832AAAC4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832AAAC8: 3BEA9F44  addi r31, r10, -0x60bc
	ctx.r[31].s64 = ctx.r[10].s64 + -24764;
	// 832AAACC: 396B0B7C  addi r11, r11, 0xb7c
	ctx.r[11].s64 = ctx.r[11].s64 + 2940;
	// 832AAAD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AAAD4: 916A9F44  stw r11, -0x60bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24764 as u32), ctx.r[11].u32 ) };
	// 832AAAD8: 4AEE9361  bl 0x82193e38
	ctx.lr = 0x832AAADC;
	sub_82193E38(ctx, base);
	// 832AAADC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AAAE0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AAAE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AAAE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AAAEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AAAF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AAAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AAB08 size=116
    let mut pc: u32 = 0x832AAB08;
    'dispatch: loop {
        match pc {
            0x832AAB08 => {
    //   block [0x832AAB08..0x832AAB2C)
	// 832AAB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AAB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AAB10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AAB14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AAB18: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832AAB1C: 807F8838  lwz r3, -0x77c8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-30664 as u32) ) } as u64;
	// 832AAB20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AAB24: 419A0044  beq cr6, 0x832aab68
	if ctx.cr[6].eq {
	pc = 0x832AAB68; continue 'dispatch;
	}
	// 832AAB28: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	pc = 0x832AAB2C; continue 'dispatch;
            }
            0x832AAB2C => {
    //   block [0x832AAB2C..0x832AAB60)
	// 832AAB2C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AAB30: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AAB34: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AAB38: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AAB3C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AAB40: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AAB44: 4082FFE8  bne 0x832aab2c
	if !ctx.cr[0].eq {
	pc = 0x832AAB2C; continue 'dispatch;
	}
	// 832AAB48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832AAB4C: 409A0014  bne cr6, 0x832aab60
	if !ctx.cr[6].eq {
	pc = 0x832AAB60; continue 'dispatch;
	}
	// 832AAB50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AAB54: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AAB58: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832AAB5C: 4E800421  bctrl
	ctx.lr = 0x832AAB60;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832AAB60 => {
    //   block [0x832AAB60..0x832AAB68)
	// 832AAB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AAB64: 917F8838  stw r11, -0x77c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-30664 as u32), ctx.r[11].u32 ) };
	pc = 0x832AAB68; continue 'dispatch;
            }
            0x832AAB68 => {
    //   block [0x832AAB68..0x832AAB7C)
	// 832AAB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AAB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AAB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AAB74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AAB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAB80 size=12
    let mut pc: u32 = 0x832AAB80;
    'dispatch: loop {
        match pc {
            0x832AAB80 => {
    //   block [0x832AAB80..0x832AAB8C)
	// 832AAB80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAB84: 386B883C  addi r3, r11, -0x77c4
	ctx.r[3].s64 = ctx.r[11].s64 + -30660;
	// 832AAB88: 4AF6A250  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAB90 size=12
    let mut pc: u32 = 0x832AAB90;
    'dispatch: loop {
        match pc {
            0x832AAB90 => {
    //   block [0x832AAB90..0x832AAB9C)
	// 832AAB90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAB94: 386B8840  addi r3, r11, -0x77c0
	ctx.r[3].s64 = ctx.r[11].s64 + -30656;
	// 832AAB98: 4AF6A240  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AABA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AABA0 size=12
    let mut pc: u32 = 0x832AABA0;
    'dispatch: loop {
        match pc {
            0x832AABA0 => {
    //   block [0x832AABA0..0x832AABAC)
	// 832AABA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AABA4: 386B8844  addi r3, r11, -0x77bc
	ctx.r[3].s64 = ctx.r[11].s64 + -30652;
	// 832AABA8: 4AF6A230  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AABB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AABB0 size=12
    let mut pc: u32 = 0x832AABB0;
    'dispatch: loop {
        match pc {
            0x832AABB0 => {
    //   block [0x832AABB0..0x832AABBC)
	// 832AABB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AABB4: 386B8848  addi r3, r11, -0x77b8
	ctx.r[3].s64 = ctx.r[11].s64 + -30648;
	// 832AABB8: 4AF6A220  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AABC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AABC0 size=12
    let mut pc: u32 = 0x832AABC0;
    'dispatch: loop {
        match pc {
            0x832AABC0 => {
    //   block [0x832AABC0..0x832AABCC)
	// 832AABC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AABC4: 386B884C  addi r3, r11, -0x77b4
	ctx.r[3].s64 = ctx.r[11].s64 + -30644;
	// 832AABC8: 4B1D53B0  b 0x8247ff78
	sub_8247FF78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AABD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AABD0 size=12
    let mut pc: u32 = 0x832AABD0;
    'dispatch: loop {
        match pc {
            0x832AABD0 => {
    //   block [0x832AABD0..0x832AABDC)
	// 832AABD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AABD4: 386B8858  addi r3, r11, -0x77a8
	ctx.r[3].s64 = ctx.r[11].s64 + -30632;
	// 832AABD8: 4AF6A200  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AABE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AABE0 size=12
    let mut pc: u32 = 0x832AABE0;
    'dispatch: loop {
        match pc {
            0x832AABE0 => {
    //   block [0x832AABE0..0x832AABEC)
	// 832AABE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AABE4: 386B885C  addi r3, r11, -0x77a4
	ctx.r[3].s64 = ctx.r[11].s64 + -30628;
	// 832AABE8: 4AF6A1F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AABF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AABF0 size=12
    let mut pc: u32 = 0x832AABF0;
    'dispatch: loop {
        match pc {
            0x832AABF0 => {
    //   block [0x832AABF0..0x832AABFC)
	// 832AABF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AABF4: 386B8860  addi r3, r11, -0x77a0
	ctx.r[3].s64 = ctx.r[11].s64 + -30624;
	// 832AABF8: 4AF6A1E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAC00 size=12
    let mut pc: u32 = 0x832AAC00;
    'dispatch: loop {
        match pc {
            0x832AAC00 => {
    //   block [0x832AAC00..0x832AAC0C)
	// 832AAC00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAC04: 386B8868  addi r3, r11, -0x7798
	ctx.r[3].s64 = ctx.r[11].s64 + -30616;
	// 832AAC08: 4AF6A1D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAC10 size=12
    let mut pc: u32 = 0x832AAC10;
    'dispatch: loop {
        match pc {
            0x832AAC10 => {
    //   block [0x832AAC10..0x832AAC1C)
	// 832AAC10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAC14: 386B886C  addi r3, r11, -0x7794
	ctx.r[3].s64 = ctx.r[11].s64 + -30612;
	// 832AAC18: 4AF6A1C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAC20 size=12
    let mut pc: u32 = 0x832AAC20;
    'dispatch: loop {
        match pc {
            0x832AAC20 => {
    //   block [0x832AAC20..0x832AAC2C)
	// 832AAC20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAC24: 386B8870  addi r3, r11, -0x7790
	ctx.r[3].s64 = ctx.r[11].s64 + -30608;
	// 832AAC28: 4AF6A1B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAC30 size=12
    let mut pc: u32 = 0x832AAC30;
    'dispatch: loop {
        match pc {
            0x832AAC30 => {
    //   block [0x832AAC30..0x832AAC3C)
	// 832AAC30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAC34: 386B8874  addi r3, r11, -0x778c
	ctx.r[3].s64 = ctx.r[11].s64 + -30604;
	// 832AAC38: 4AF6A1A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAC40 size=4
    let mut pc: u32 = 0x832AAC40;
    'dispatch: loop {
        match pc {
            0x832AAC40 => {
    //   block [0x832AAC40..0x832AAC44)
	// 832AAC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAC48 size=12
    let mut pc: u32 = 0x832AAC48;
    'dispatch: loop {
        match pc {
            0x832AAC48 => {
    //   block [0x832AAC48..0x832AAC54)
	// 832AAC48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAC4C: 386B8878  addi r3, r11, -0x7788
	ctx.r[3].s64 = ctx.r[11].s64 + -30600;
	// 832AAC50: 4AF6A188  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAC58 size=12
    let mut pc: u32 = 0x832AAC58;
    'dispatch: loop {
        match pc {
            0x832AAC58 => {
    //   block [0x832AAC58..0x832AAC64)
	// 832AAC58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAC5C: 386B887C  addi r3, r11, -0x7784
	ctx.r[3].s64 = ctx.r[11].s64 + -30596;
	// 832AAC60: 4AF6A178  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAC68 size=12
    let mut pc: u32 = 0x832AAC68;
    'dispatch: loop {
        match pc {
            0x832AAC68 => {
    //   block [0x832AAC68..0x832AAC74)
	// 832AAC68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAC6C: 386B8880  addi r3, r11, -0x7780
	ctx.r[3].s64 = ctx.r[11].s64 + -30592;
	// 832AAC70: 4AF6A168  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAC78 size=12
    let mut pc: u32 = 0x832AAC78;
    'dispatch: loop {
        match pc {
            0x832AAC78 => {
    //   block [0x832AAC78..0x832AAC84)
	// 832AAC78: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AAC7C: 386B3BE8  addi r3, r11, 0x3be8
	ctx.r[3].s64 = ctx.r[11].s64 + 15336;
	// 832AAC80: 4AF0CE98  b 0x821b7b18
	sub_821B7B18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAC88 size=12
    let mut pc: u32 = 0x832AAC88;
    'dispatch: loop {
        match pc {
            0x832AAC88 => {
    //   block [0x832AAC88..0x832AAC94)
	// 832AAC88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAC8C: 386B8884  addi r3, r11, -0x777c
	ctx.r[3].s64 = ctx.r[11].s64 + -30588;
	// 832AAC90: 4B76D0D0  b 0x82a17d60
	sub_82A17D60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAC98 size=12
    let mut pc: u32 = 0x832AAC98;
    'dispatch: loop {
        match pc {
            0x832AAC98 => {
    //   block [0x832AAC98..0x832AACA4)
	// 832AAC98: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AAC9C: 386B3CBC  addi r3, r11, 0x3cbc
	ctx.r[3].s64 = ctx.r[11].s64 + 15548;
	// 832AACA0: 4B131B88  b 0x823dc828
	sub_823DC828(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AACA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AACA8 size=12
    let mut pc: u32 = 0x832AACA8;
    'dispatch: loop {
        match pc {
            0x832AACA8 => {
    //   block [0x832AACA8..0x832AACB4)
	// 832AACA8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AACAC: 386B3CA8  addi r3, r11, 0x3ca8
	ctx.r[3].s64 = ctx.r[11].s64 + 15528;
	// 832AACB0: 4B131B78  b 0x823dc828
	sub_823DC828(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AACB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AACB8 size=12
    let mut pc: u32 = 0x832AACB8;
    'dispatch: loop {
        match pc {
            0x832AACB8 => {
    //   block [0x832AACB8..0x832AACC4)
	// 832AACB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AACBC: 386B8890  addi r3, r11, -0x7770
	ctx.r[3].s64 = ctx.r[11].s64 + -30576;
	// 832AACC0: 4AF6A118  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AACC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AACC8 size=12
    let mut pc: u32 = 0x832AACC8;
    'dispatch: loop {
        match pc {
            0x832AACC8 => {
    //   block [0x832AACC8..0x832AACD4)
	// 832AACC8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AACCC: 386B8894  addi r3, r11, -0x776c
	ctx.r[3].s64 = ctx.r[11].s64 + -30572;
	// 832AACD0: 4AF6A108  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AACD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AACD8 size=12
    let mut pc: u32 = 0x832AACD8;
    'dispatch: loop {
        match pc {
            0x832AACD8 => {
    //   block [0x832AACD8..0x832AACE4)
	// 832AACD8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AACDC: 386B8898  addi r3, r11, -0x7768
	ctx.r[3].s64 = ctx.r[11].s64 + -30568;
	// 832AACE0: 4AF6A0F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AACE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AACE8 size=12
    let mut pc: u32 = 0x832AACE8;
    'dispatch: loop {
        match pc {
            0x832AACE8 => {
    //   block [0x832AACE8..0x832AACF4)
	// 832AACE8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AACEC: 386B889C  addi r3, r11, -0x7764
	ctx.r[3].s64 = ctx.r[11].s64 + -30564;
	// 832AACF0: 4AF6A0E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AACF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AACF8 size=12
    let mut pc: u32 = 0x832AACF8;
    'dispatch: loop {
        match pc {
            0x832AACF8 => {
    //   block [0x832AACF8..0x832AAD04)
	// 832AACF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AACFC: 386B88A0  addi r3, r11, -0x7760
	ctx.r[3].s64 = ctx.r[11].s64 + -30560;
	// 832AAD00: 4AF6A0D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAD08 size=12
    let mut pc: u32 = 0x832AAD08;
    'dispatch: loop {
        match pc {
            0x832AAD08 => {
    //   block [0x832AAD08..0x832AAD14)
	// 832AAD08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAD0C: 386B88A4  addi r3, r11, -0x775c
	ctx.r[3].s64 = ctx.r[11].s64 + -30556;
	// 832AAD10: 4AF6A0C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAD18 size=12
    let mut pc: u32 = 0x832AAD18;
    'dispatch: loop {
        match pc {
            0x832AAD18 => {
    //   block [0x832AAD18..0x832AAD24)
	// 832AAD18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAD1C: 386B88A8  addi r3, r11, -0x7758
	ctx.r[3].s64 = ctx.r[11].s64 + -30552;
	// 832AAD20: 4AF6A0B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAD28 size=12
    let mut pc: u32 = 0x832AAD28;
    'dispatch: loop {
        match pc {
            0x832AAD28 => {
    //   block [0x832AAD28..0x832AAD34)
	// 832AAD28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAD2C: 386B88AC  addi r3, r11, -0x7754
	ctx.r[3].s64 = ctx.r[11].s64 + -30548;
	// 832AAD30: 4AF6A0A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AAD38 size=132
    let mut pc: u32 = 0x832AAD38;
    'dispatch: loop {
        match pc {
            0x832AAD38 => {
    //   block [0x832AAD38..0x832AAD7C)
	// 832AAD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AAD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AAD40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832AAD44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AAD48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AAD4C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAD50: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AAD54: 3BEB88B0  addi r31, r11, -0x7750
	ctx.r[31].s64 = ctx.r[11].s64 + -30544;
	// 832AAD58: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AAD5C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AAD60: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832AAD64: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AAD68: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832AAD6C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AAD70: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832AAD74: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832AAD78: 419A0020  beq cr6, 0x832aad98
	if ctx.cr[6].eq {
	pc = 0x832AAD98; continue 'dispatch;
	}
	pc = 0x832AAD7C; continue 'dispatch;
            }
            0x832AAD7C => {
    //   block [0x832AAD7C..0x832AAD98)
	// 832AAD7C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 832AAD80: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AAD84: 4AF70FB5  bl 0x8221bd38
	ctx.lr = 0x832AAD88;
	sub_8221BD38(ctx, base);
	// 832AAD88: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AAD8C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832AAD90: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832AAD94: 409AFFE8  bne cr6, 0x832aad7c
	if !ctx.cr[6].eq {
	pc = 0x832AAD7C; continue 'dispatch;
	}
	pc = 0x832AAD98; continue 'dispatch;
            }
            0x832AAD98 => {
    //   block [0x832AAD98..0x832AADBC)
	// 832AAD98: 4AF70FA1  bl 0x8221bd38
	ctx.lr = 0x832AAD9C;
	sub_8221BD38(ctx, base);
	// 832AAD9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AADA0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AADA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832AADA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AADAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AADB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832AADB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AADB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AADC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AADC0 size=12
    let mut pc: u32 = 0x832AADC0;
    'dispatch: loop {
        match pc {
            0x832AADC0 => {
    //   block [0x832AADC0..0x832AADCC)
	// 832AADC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AADC4: 386B88BC  addi r3, r11, -0x7744
	ctx.r[3].s64 = ctx.r[11].s64 + -30532;
	// 832AADC8: 4AF6A010  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AADD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AADD0 size=12
    let mut pc: u32 = 0x832AADD0;
    'dispatch: loop {
        match pc {
            0x832AADD0 => {
    //   block [0x832AADD0..0x832AADDC)
	// 832AADD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AADD4: 386B88C0  addi r3, r11, -0x7740
	ctx.r[3].s64 = ctx.r[11].s64 + -30528;
	// 832AADD8: 4AF6A000  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AADE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AADE0 size=12
    let mut pc: u32 = 0x832AADE0;
    'dispatch: loop {
        match pc {
            0x832AADE0 => {
    //   block [0x832AADE0..0x832AADEC)
	// 832AADE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AADE4: 386B88C4  addi r3, r11, -0x773c
	ctx.r[3].s64 = ctx.r[11].s64 + -30524;
	// 832AADE8: 4AF69FF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AADF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AADF0 size=12
    let mut pc: u32 = 0x832AADF0;
    'dispatch: loop {
        match pc {
            0x832AADF0 => {
    //   block [0x832AADF0..0x832AADFC)
	// 832AADF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AADF4: 386B88C8  addi r3, r11, -0x7738
	ctx.r[3].s64 = ctx.r[11].s64 + -30520;
	// 832AADF8: 4AF69FE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAE00 size=12
    let mut pc: u32 = 0x832AAE00;
    'dispatch: loop {
        match pc {
            0x832AAE00 => {
    //   block [0x832AAE00..0x832AAE0C)
	// 832AAE00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAE04: 386B88CC  addi r3, r11, -0x7734
	ctx.r[3].s64 = ctx.r[11].s64 + -30516;
	// 832AAE08: 4AF69FD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAE10 size=12
    let mut pc: u32 = 0x832AAE10;
    'dispatch: loop {
        match pc {
            0x832AAE10 => {
    //   block [0x832AAE10..0x832AAE1C)
	// 832AAE10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAE14: 386B88D0  addi r3, r11, -0x7730
	ctx.r[3].s64 = ctx.r[11].s64 + -30512;
	// 832AAE18: 4AF69FC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAE20 size=12
    let mut pc: u32 = 0x832AAE20;
    'dispatch: loop {
        match pc {
            0x832AAE20 => {
    //   block [0x832AAE20..0x832AAE2C)
	// 832AAE20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAE24: 386B88D4  addi r3, r11, -0x772c
	ctx.r[3].s64 = ctx.r[11].s64 + -30508;
	// 832AAE28: 4AF69FB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAE30 size=12
    let mut pc: u32 = 0x832AAE30;
    'dispatch: loop {
        match pc {
            0x832AAE30 => {
    //   block [0x832AAE30..0x832AAE3C)
	// 832AAE30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAE34: 386B88D8  addi r3, r11, -0x7728
	ctx.r[3].s64 = ctx.r[11].s64 + -30504;
	// 832AAE38: 4AF69FA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAE40 size=12
    let mut pc: u32 = 0x832AAE40;
    'dispatch: loop {
        match pc {
            0x832AAE40 => {
    //   block [0x832AAE40..0x832AAE4C)
	// 832AAE40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAE44: 386B88DC  addi r3, r11, -0x7724
	ctx.r[3].s64 = ctx.r[11].s64 + -30500;
	// 832AAE48: 4AF69F90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAE50 size=12
    let mut pc: u32 = 0x832AAE50;
    'dispatch: loop {
        match pc {
            0x832AAE50 => {
    //   block [0x832AAE50..0x832AAE5C)
	// 832AAE50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAE54: 386B88E0  addi r3, r11, -0x7720
	ctx.r[3].s64 = ctx.r[11].s64 + -30496;
	// 832AAE58: 4AF69F80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAE60 size=12
    let mut pc: u32 = 0x832AAE60;
    'dispatch: loop {
        match pc {
            0x832AAE60 => {
    //   block [0x832AAE60..0x832AAE6C)
	// 832AAE60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAE64: 386B88E4  addi r3, r11, -0x771c
	ctx.r[3].s64 = ctx.r[11].s64 + -30492;
	// 832AAE68: 4AF69F70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAE70 size=12
    let mut pc: u32 = 0x832AAE70;
    'dispatch: loop {
        match pc {
            0x832AAE70 => {
    //   block [0x832AAE70..0x832AAE7C)
	// 832AAE70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAE74: 386B88E8  addi r3, r11, -0x7718
	ctx.r[3].s64 = ctx.r[11].s64 + -30488;
	// 832AAE78: 4AF69F60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAE80 size=12
    let mut pc: u32 = 0x832AAE80;
    'dispatch: loop {
        match pc {
            0x832AAE80 => {
    //   block [0x832AAE80..0x832AAE8C)
	// 832AAE80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAE84: 386B88EC  addi r3, r11, -0x7714
	ctx.r[3].s64 = ctx.r[11].s64 + -30484;
	// 832AAE88: 4AF69F50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAE90 size=12
    let mut pc: u32 = 0x832AAE90;
    'dispatch: loop {
        match pc {
            0x832AAE90 => {
    //   block [0x832AAE90..0x832AAE9C)
	// 832AAE90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAE94: 386B88F0  addi r3, r11, -0x7710
	ctx.r[3].s64 = ctx.r[11].s64 + -30480;
	// 832AAE98: 4AF69F40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAEA0 size=12
    let mut pc: u32 = 0x832AAEA0;
    'dispatch: loop {
        match pc {
            0x832AAEA0 => {
    //   block [0x832AAEA0..0x832AAEAC)
	// 832AAEA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAEA4: 386B88F4  addi r3, r11, -0x770c
	ctx.r[3].s64 = ctx.r[11].s64 + -30476;
	// 832AAEA8: 4AF69F30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAEB0 size=12
    let mut pc: u32 = 0x832AAEB0;
    'dispatch: loop {
        match pc {
            0x832AAEB0 => {
    //   block [0x832AAEB0..0x832AAEBC)
	// 832AAEB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAEB4: 386B88F8  addi r3, r11, -0x7708
	ctx.r[3].s64 = ctx.r[11].s64 + -30472;
	// 832AAEB8: 4AF69F20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAEC0 size=12
    let mut pc: u32 = 0x832AAEC0;
    'dispatch: loop {
        match pc {
            0x832AAEC0 => {
    //   block [0x832AAEC0..0x832AAECC)
	// 832AAEC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAEC4: 386B88FC  addi r3, r11, -0x7704
	ctx.r[3].s64 = ctx.r[11].s64 + -30468;
	// 832AAEC8: 4AF69F10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAED0 size=12
    let mut pc: u32 = 0x832AAED0;
    'dispatch: loop {
        match pc {
            0x832AAED0 => {
    //   block [0x832AAED0..0x832AAEDC)
	// 832AAED0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAED4: 386B8900  addi r3, r11, -0x7700
	ctx.r[3].s64 = ctx.r[11].s64 + -30464;
	// 832AAED8: 4AF69F00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAEE0 size=12
    let mut pc: u32 = 0x832AAEE0;
    'dispatch: loop {
        match pc {
            0x832AAEE0 => {
    //   block [0x832AAEE0..0x832AAEEC)
	// 832AAEE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAEE4: 386B8904  addi r3, r11, -0x76fc
	ctx.r[3].s64 = ctx.r[11].s64 + -30460;
	// 832AAEE8: 4AF69EF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAEF0 size=12
    let mut pc: u32 = 0x832AAEF0;
    'dispatch: loop {
        match pc {
            0x832AAEF0 => {
    //   block [0x832AAEF0..0x832AAEFC)
	// 832AAEF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAEF4: 386B8908  addi r3, r11, -0x76f8
	ctx.r[3].s64 = ctx.r[11].s64 + -30456;
	// 832AAEF8: 4AF69EE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAF00 size=12
    let mut pc: u32 = 0x832AAF00;
    'dispatch: loop {
        match pc {
            0x832AAF00 => {
    //   block [0x832AAF00..0x832AAF0C)
	// 832AAF00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAF04: 386B890C  addi r3, r11, -0x76f4
	ctx.r[3].s64 = ctx.r[11].s64 + -30452;
	// 832AAF08: 4AF69ED0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAF10 size=12
    let mut pc: u32 = 0x832AAF10;
    'dispatch: loop {
        match pc {
            0x832AAF10 => {
    //   block [0x832AAF10..0x832AAF1C)
	// 832AAF10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAF14: 386B8910  addi r3, r11, -0x76f0
	ctx.r[3].s64 = ctx.r[11].s64 + -30448;
	// 832AAF18: 4AF69EC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAF20 size=12
    let mut pc: u32 = 0x832AAF20;
    'dispatch: loop {
        match pc {
            0x832AAF20 => {
    //   block [0x832AAF20..0x832AAF2C)
	// 832AAF20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAF24: 386B8914  addi r3, r11, -0x76ec
	ctx.r[3].s64 = ctx.r[11].s64 + -30444;
	// 832AAF28: 4AF69EB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAF30 size=12
    let mut pc: u32 = 0x832AAF30;
    'dispatch: loop {
        match pc {
            0x832AAF30 => {
    //   block [0x832AAF30..0x832AAF3C)
	// 832AAF30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAF34: 386B8918  addi r3, r11, -0x76e8
	ctx.r[3].s64 = ctx.r[11].s64 + -30440;
	// 832AAF38: 4AF69EA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAF40 size=12
    let mut pc: u32 = 0x832AAF40;
    'dispatch: loop {
        match pc {
            0x832AAF40 => {
    //   block [0x832AAF40..0x832AAF4C)
	// 832AAF40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAF44: 386B891C  addi r3, r11, -0x76e4
	ctx.r[3].s64 = ctx.r[11].s64 + -30436;
	// 832AAF48: 4AF69E90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAF50 size=12
    let mut pc: u32 = 0x832AAF50;
    'dispatch: loop {
        match pc {
            0x832AAF50 => {
    //   block [0x832AAF50..0x832AAF5C)
	// 832AAF50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAF54: 386B8920  addi r3, r11, -0x76e0
	ctx.r[3].s64 = ctx.r[11].s64 + -30432;
	// 832AAF58: 4AF69E80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAF60 size=12
    let mut pc: u32 = 0x832AAF60;
    'dispatch: loop {
        match pc {
            0x832AAF60 => {
    //   block [0x832AAF60..0x832AAF6C)
	// 832AAF60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAF64: 386B8924  addi r3, r11, -0x76dc
	ctx.r[3].s64 = ctx.r[11].s64 + -30428;
	// 832AAF68: 4AF69E70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAF70 size=12
    let mut pc: u32 = 0x832AAF70;
    'dispatch: loop {
        match pc {
            0x832AAF70 => {
    //   block [0x832AAF70..0x832AAF7C)
	// 832AAF70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAF74: 386B8928  addi r3, r11, -0x76d8
	ctx.r[3].s64 = ctx.r[11].s64 + -30424;
	// 832AAF78: 4AF69E60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAF80 size=12
    let mut pc: u32 = 0x832AAF80;
    'dispatch: loop {
        match pc {
            0x832AAF80 => {
    //   block [0x832AAF80..0x832AAF8C)
	// 832AAF80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAF84: 386B892C  addi r3, r11, -0x76d4
	ctx.r[3].s64 = ctx.r[11].s64 + -30420;
	// 832AAF88: 4AF69E50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAF90 size=12
    let mut pc: u32 = 0x832AAF90;
    'dispatch: loop {
        match pc {
            0x832AAF90 => {
    //   block [0x832AAF90..0x832AAF9C)
	// 832AAF90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAF94: 386B8930  addi r3, r11, -0x76d0
	ctx.r[3].s64 = ctx.r[11].s64 + -30416;
	// 832AAF98: 4AF69E40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAFA0 size=12
    let mut pc: u32 = 0x832AAFA0;
    'dispatch: loop {
        match pc {
            0x832AAFA0 => {
    //   block [0x832AAFA0..0x832AAFAC)
	// 832AAFA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAFA4: 386B8934  addi r3, r11, -0x76cc
	ctx.r[3].s64 = ctx.r[11].s64 + -30412;
	// 832AAFA8: 4AF69E30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAFB0 size=12
    let mut pc: u32 = 0x832AAFB0;
    'dispatch: loop {
        match pc {
            0x832AAFB0 => {
    //   block [0x832AAFB0..0x832AAFBC)
	// 832AAFB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAFB4: 386B8938  addi r3, r11, -0x76c8
	ctx.r[3].s64 = ctx.r[11].s64 + -30408;
	// 832AAFB8: 4AF69E20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAFC0 size=12
    let mut pc: u32 = 0x832AAFC0;
    'dispatch: loop {
        match pc {
            0x832AAFC0 => {
    //   block [0x832AAFC0..0x832AAFCC)
	// 832AAFC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAFC4: 386B893C  addi r3, r11, -0x76c4
	ctx.r[3].s64 = ctx.r[11].s64 + -30404;
	// 832AAFC8: 4AF69E10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAFD0 size=12
    let mut pc: u32 = 0x832AAFD0;
    'dispatch: loop {
        match pc {
            0x832AAFD0 => {
    //   block [0x832AAFD0..0x832AAFDC)
	// 832AAFD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAFD4: 386B8940  addi r3, r11, -0x76c0
	ctx.r[3].s64 = ctx.r[11].s64 + -30400;
	// 832AAFD8: 4AF69E00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAFE0 size=12
    let mut pc: u32 = 0x832AAFE0;
    'dispatch: loop {
        match pc {
            0x832AAFE0 => {
    //   block [0x832AAFE0..0x832AAFEC)
	// 832AAFE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAFE4: 386B8944  addi r3, r11, -0x76bc
	ctx.r[3].s64 = ctx.r[11].s64 + -30396;
	// 832AAFE8: 4AF69DF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AAFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAFF0 size=12
    let mut pc: u32 = 0x832AAFF0;
    'dispatch: loop {
        match pc {
            0x832AAFF0 => {
    //   block [0x832AAFF0..0x832AAFFC)
	// 832AAFF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAFF4: 386B8948  addi r3, r11, -0x76b8
	ctx.r[3].s64 = ctx.r[11].s64 + -30392;
	// 832AAFF8: 4AF69DE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB000 size=12
    let mut pc: u32 = 0x832AB000;
    'dispatch: loop {
        match pc {
            0x832AB000 => {
    //   block [0x832AB000..0x832AB00C)
	// 832AB000: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB004: 386B894C  addi r3, r11, -0x76b4
	ctx.r[3].s64 = ctx.r[11].s64 + -30388;
	// 832AB008: 4AF69DD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB010 size=12
    let mut pc: u32 = 0x832AB010;
    'dispatch: loop {
        match pc {
            0x832AB010 => {
    //   block [0x832AB010..0x832AB01C)
	// 832AB010: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB014: 386B8950  addi r3, r11, -0x76b0
	ctx.r[3].s64 = ctx.r[11].s64 + -30384;
	// 832AB018: 4AF69DC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB020 size=12
    let mut pc: u32 = 0x832AB020;
    'dispatch: loop {
        match pc {
            0x832AB020 => {
    //   block [0x832AB020..0x832AB02C)
	// 832AB020: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB024: 386B8954  addi r3, r11, -0x76ac
	ctx.r[3].s64 = ctx.r[11].s64 + -30380;
	// 832AB028: 4AF69DB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB030 size=12
    let mut pc: u32 = 0x832AB030;
    'dispatch: loop {
        match pc {
            0x832AB030 => {
    //   block [0x832AB030..0x832AB03C)
	// 832AB030: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB034: 386B8958  addi r3, r11, -0x76a8
	ctx.r[3].s64 = ctx.r[11].s64 + -30376;
	// 832AB038: 4AF69DA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB040 size=12
    let mut pc: u32 = 0x832AB040;
    'dispatch: loop {
        match pc {
            0x832AB040 => {
    //   block [0x832AB040..0x832AB04C)
	// 832AB040: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB044: 386B895C  addi r3, r11, -0x76a4
	ctx.r[3].s64 = ctx.r[11].s64 + -30372;
	// 832AB048: 4AF69D90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB050 size=12
    let mut pc: u32 = 0x832AB050;
    'dispatch: loop {
        match pc {
            0x832AB050 => {
    //   block [0x832AB050..0x832AB05C)
	// 832AB050: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB054: 386B8978  addi r3, r11, -0x7688
	ctx.r[3].s64 = ctx.r[11].s64 + -30344;
	// 832AB058: 4AF69D80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB060 size=12
    let mut pc: u32 = 0x832AB060;
    'dispatch: loop {
        match pc {
            0x832AB060 => {
    //   block [0x832AB060..0x832AB06C)
	// 832AB060: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB064: 386B897C  addi r3, r11, -0x7684
	ctx.r[3].s64 = ctx.r[11].s64 + -30340;
	// 832AB068: 4AF69D70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB070 size=12
    let mut pc: u32 = 0x832AB070;
    'dispatch: loop {
        match pc {
            0x832AB070 => {
    //   block [0x832AB070..0x832AB07C)
	// 832AB070: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB074: 386B8980  addi r3, r11, -0x7680
	ctx.r[3].s64 = ctx.r[11].s64 + -30336;
	// 832AB078: 4AF69D60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB080 size=12
    let mut pc: u32 = 0x832AB080;
    'dispatch: loop {
        match pc {
            0x832AB080 => {
    //   block [0x832AB080..0x832AB08C)
	// 832AB080: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB084: 386B8984  addi r3, r11, -0x767c
	ctx.r[3].s64 = ctx.r[11].s64 + -30332;
	// 832AB088: 4AF69D50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB090 size=12
    let mut pc: u32 = 0x832AB090;
    'dispatch: loop {
        match pc {
            0x832AB090 => {
    //   block [0x832AB090..0x832AB09C)
	// 832AB090: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB094: 386B8988  addi r3, r11, -0x7678
	ctx.r[3].s64 = ctx.r[11].s64 + -30328;
	// 832AB098: 4AF69D40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB0A0 size=12
    let mut pc: u32 = 0x832AB0A0;
    'dispatch: loop {
        match pc {
            0x832AB0A0 => {
    //   block [0x832AB0A0..0x832AB0AC)
	// 832AB0A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB0A4: 386B898C  addi r3, r11, -0x7674
	ctx.r[3].s64 = ctx.r[11].s64 + -30324;
	// 832AB0A8: 4AF69D30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB0B0 size=12
    let mut pc: u32 = 0x832AB0B0;
    'dispatch: loop {
        match pc {
            0x832AB0B0 => {
    //   block [0x832AB0B0..0x832AB0BC)
	// 832AB0B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB0B4: 386B8990  addi r3, r11, -0x7670
	ctx.r[3].s64 = ctx.r[11].s64 + -30320;
	// 832AB0B8: 4AF69D20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB0C0 size=12
    let mut pc: u32 = 0x832AB0C0;
    'dispatch: loop {
        match pc {
            0x832AB0C0 => {
    //   block [0x832AB0C0..0x832AB0CC)
	// 832AB0C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB0C4: 386B8994  addi r3, r11, -0x766c
	ctx.r[3].s64 = ctx.r[11].s64 + -30316;
	// 832AB0C8: 4AF69D10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB0D0 size=12
    let mut pc: u32 = 0x832AB0D0;
    'dispatch: loop {
        match pc {
            0x832AB0D0 => {
    //   block [0x832AB0D0..0x832AB0DC)
	// 832AB0D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB0D4: 386B8998  addi r3, r11, -0x7668
	ctx.r[3].s64 = ctx.r[11].s64 + -30312;
	// 832AB0D8: 4AF69D00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB0E0 size=12
    let mut pc: u32 = 0x832AB0E0;
    'dispatch: loop {
        match pc {
            0x832AB0E0 => {
    //   block [0x832AB0E0..0x832AB0EC)
	// 832AB0E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB0E4: 386B899C  addi r3, r11, -0x7664
	ctx.r[3].s64 = ctx.r[11].s64 + -30308;
	// 832AB0E8: 4AF69CF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB0F0 size=12
    let mut pc: u32 = 0x832AB0F0;
    'dispatch: loop {
        match pc {
            0x832AB0F0 => {
    //   block [0x832AB0F0..0x832AB0FC)
	// 832AB0F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB0F4: 386B89E0  addi r3, r11, -0x7620
	ctx.r[3].s64 = ctx.r[11].s64 + -30240;
	// 832AB0F8: 4AF69CE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB100 size=12
    let mut pc: u32 = 0x832AB100;
    'dispatch: loop {
        match pc {
            0x832AB100 => {
    //   block [0x832AB100..0x832AB10C)
	// 832AB100: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB104: 386B89E4  addi r3, r11, -0x761c
	ctx.r[3].s64 = ctx.r[11].s64 + -30236;
	// 832AB108: 4AF69CD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB110 size=12
    let mut pc: u32 = 0x832AB110;
    'dispatch: loop {
        match pc {
            0x832AB110 => {
    //   block [0x832AB110..0x832AB11C)
	// 832AB110: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB114: 386B89E8  addi r3, r11, -0x7618
	ctx.r[3].s64 = ctx.r[11].s64 + -30232;
	// 832AB118: 4AF69CC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB120 size=100
    let mut pc: u32 = 0x832AB120;
    'dispatch: loop {
        match pc {
            0x832AB120 => {
    //   block [0x832AB120..0x832AB158)
	// 832AB120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB12C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB130: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB134: 3BEB89EC  addi r31, r11, -0x7614
	ctx.r[31].s64 = ctx.r[11].s64 + -30228;
	// 832AB138: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB13C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832AB140: 419A0018  beq cr6, 0x832ab158
	if ctx.cr[6].eq {
	pc = 0x832AB158; continue 'dispatch;
	}
	// 832AB144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB148: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832AB14C: 4B2247C5  bl 0x824cf910
	ctx.lr = 0x832AB150;
	sub_824CF910(ctx, base);
	// 832AB150: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB154: 4AF70BE5  bl 0x8221bd38
	ctx.lr = 0x832AB158;
	sub_8221BD38(ctx, base);
	pc = 0x832AB158; continue 'dispatch;
            }
            0x832AB158 => {
    //   block [0x832AB158..0x832AB184)
	// 832AB158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB15C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB160: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AB164: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB168: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AB16C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AB170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB17C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB188 size=100
    let mut pc: u32 = 0x832AB188;
    'dispatch: loop {
        match pc {
            0x832AB188 => {
    //   block [0x832AB188..0x832AB1C0)
	// 832AB188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB198: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB19C: 3BEB89FC  addi r31, r11, -0x7604
	ctx.r[31].s64 = ctx.r[11].s64 + -30212;
	// 832AB1A0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB1A4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832AB1A8: 419A0018  beq cr6, 0x832ab1c0
	if ctx.cr[6].eq {
	pc = 0x832AB1C0; continue 'dispatch;
	}
	// 832AB1AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB1B0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832AB1B4: 4B22475D  bl 0x824cf910
	ctx.lr = 0x832AB1B8;
	sub_824CF910(ctx, base);
	// 832AB1B8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB1BC: 4AF70B7D  bl 0x8221bd38
	ctx.lr = 0x832AB1C0;
	sub_8221BD38(ctx, base);
	pc = 0x832AB1C0; continue 'dispatch;
            }
            0x832AB1C0 => {
    //   block [0x832AB1C0..0x832AB1EC)
	// 832AB1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB1C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB1C8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AB1CC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB1D0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AB1D4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AB1D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB1E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB1F0 size=84
    let mut pc: u32 = 0x832AB1F0;
    'dispatch: loop {
        match pc {
            0x832AB1F0 => {
    //   block [0x832AB1F0..0x832AB218)
	// 832AB1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB1F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB1FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB200: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB204: 3BEB8A0C  addi r31, r11, -0x75f4
	ctx.r[31].s64 = ctx.r[11].s64 + -30196;
	// 832AB208: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB20C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AB210: 419A0008  beq cr6, 0x832ab218
	if ctx.cr[6].eq {
	pc = 0x832AB218; continue 'dispatch;
	}
	// 832AB214: 4AF70B25  bl 0x8221bd38
	ctx.lr = 0x832AB218;
	sub_8221BD38(ctx, base);
	pc = 0x832AB218; continue 'dispatch;
            }
            0x832AB218 => {
    //   block [0x832AB218..0x832AB244)
	// 832AB218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB21C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB220: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AB224: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB228: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AB22C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AB230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB23C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB248 size=12
    let mut pc: u32 = 0x832AB248;
    'dispatch: loop {
        match pc {
            0x832AB248 => {
    //   block [0x832AB248..0x832AB254)
	// 832AB248: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB24C: 386B8A1C  addi r3, r11, -0x75e4
	ctx.r[3].s64 = ctx.r[11].s64 + -30180;
	// 832AB250: 4AF69B88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB258 size=12
    let mut pc: u32 = 0x832AB258;
    'dispatch: loop {
        match pc {
            0x832AB258 => {
    //   block [0x832AB258..0x832AB264)
	// 832AB258: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB25C: 386B8A20  addi r3, r11, -0x75e0
	ctx.r[3].s64 = ctx.r[11].s64 + -30176;
	// 832AB260: 4AF69B78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB268 size=12
    let mut pc: u32 = 0x832AB268;
    'dispatch: loop {
        match pc {
            0x832AB268 => {
    //   block [0x832AB268..0x832AB274)
	// 832AB268: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB26C: 386B8A24  addi r3, r11, -0x75dc
	ctx.r[3].s64 = ctx.r[11].s64 + -30172;
	// 832AB270: 4AF69B68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB278 size=12
    let mut pc: u32 = 0x832AB278;
    'dispatch: loop {
        match pc {
            0x832AB278 => {
    //   block [0x832AB278..0x832AB284)
	// 832AB278: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB27C: 386B8A28  addi r3, r11, -0x75d8
	ctx.r[3].s64 = ctx.r[11].s64 + -30168;
	// 832AB280: 4AF69B58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB288 size=12
    let mut pc: u32 = 0x832AB288;
    'dispatch: loop {
        match pc {
            0x832AB288 => {
    //   block [0x832AB288..0x832AB294)
	// 832AB288: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB28C: 386B8A2C  addi r3, r11, -0x75d4
	ctx.r[3].s64 = ctx.r[11].s64 + -30164;
	// 832AB290: 4AF69B48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB298 size=12
    let mut pc: u32 = 0x832AB298;
    'dispatch: loop {
        match pc {
            0x832AB298 => {
    //   block [0x832AB298..0x832AB2A4)
	// 832AB298: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB29C: 386B8A30  addi r3, r11, -0x75d0
	ctx.r[3].s64 = ctx.r[11].s64 + -30160;
	// 832AB2A0: 4AF69B38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB2A8 size=12
    let mut pc: u32 = 0x832AB2A8;
    'dispatch: loop {
        match pc {
            0x832AB2A8 => {
    //   block [0x832AB2A8..0x832AB2B4)
	// 832AB2A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB2AC: 386B8A34  addi r3, r11, -0x75cc
	ctx.r[3].s64 = ctx.r[11].s64 + -30156;
	// 832AB2B0: 4AF69B28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB2B8 size=104
    let mut pc: u32 = 0x832AB2B8;
    'dispatch: loop {
        match pc {
            0x832AB2B8 => {
    //   block [0x832AB2B8..0x832AB2E0)
	// 832AB2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB2BC: 4B9FE14D  bl 0x82ca9408
	ctx.lr = 0x832AB2C0;
	sub_82CA93D0(ctx, base);
	// 832AB2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB2C4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB2C8: 3BC0001F  li r30, 0x1f
	ctx.r[30].s64 = 31;
	// 832AB2CC: 396B8A38  addi r11, r11, -0x75c8
	ctx.r[11].s64 = ctx.r[11].s64 + -30152;
	// 832AB2D0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AB2D4: 3BEB0080  addi r31, r11, 0x80
	ctx.r[31].s64 = ctx.r[11].s64 + 128;
	// 832AB2D8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AB2DC: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832AB2E0; continue 'dispatch;
            }
            0x832AB2E0 => {
    //   block [0x832AB2E0..0x832AB2F0)
	// 832AB2E0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AB2E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB2E8: 4AF1B481  bl 0x821c6768
	ctx.lr = 0x832AB2EC;
	sub_821C6768(ctx, base);
	// 832AB2EC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832AB2F0; continue 'dispatch;
            }
            0x832AB2F0 => {
    //   block [0x832AB2F0..0x832AB320)
	// 832AB2F0: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AB2F4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB2F8: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AB2FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AB300: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AB304: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB308: 4082FFE8  bne 0x832ab2f0
	if !ctx.cr[0].eq {
	pc = 0x832AB2F0; continue 'dispatch;
	}
	// 832AB30C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AB310: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AB314: 4080FFCC  bge 0x832ab2e0
	if !ctx.cr[0].lt {
	pc = 0x832AB2E0; continue 'dispatch;
	}
	// 832AB318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AB31C: 4B9FE13C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB320 size=84
    let mut pc: u32 = 0x832AB320;
    'dispatch: loop {
        match pc {
            0x832AB320 => {
    //   block [0x832AB320..0x832AB348)
	// 832AB320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB328: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB32C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB330: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB334: 3BEB8AB8  addi r31, r11, -0x7548
	ctx.r[31].s64 = ctx.r[11].s64 + -30024;
	// 832AB338: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB33C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AB340: 419A0008  beq cr6, 0x832ab348
	if ctx.cr[6].eq {
	pc = 0x832AB348; continue 'dispatch;
	}
	// 832AB344: 4AF709F5  bl 0x8221bd38
	ctx.lr = 0x832AB348;
	sub_8221BD38(ctx, base);
	pc = 0x832AB348; continue 'dispatch;
            }
            0x832AB348 => {
    //   block [0x832AB348..0x832AB374)
	// 832AB348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB34C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB350: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AB354: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB358: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AB35C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AB360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB36C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB378 size=12
    let mut pc: u32 = 0x832AB378;
    'dispatch: loop {
        match pc {
            0x832AB378 => {
    //   block [0x832AB378..0x832AB384)
	// 832AB378: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB37C: 386B8AD0  addi r3, r11, -0x7530
	ctx.r[3].s64 = ctx.r[11].s64 + -30000;
	// 832AB380: 4AF69A58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB388 size=12
    let mut pc: u32 = 0x832AB388;
    'dispatch: loop {
        match pc {
            0x832AB388 => {
    //   block [0x832AB388..0x832AB394)
	// 832AB388: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB38C: 386B8AD4  addi r3, r11, -0x752c
	ctx.r[3].s64 = ctx.r[11].s64 + -29996;
	// 832AB390: 4AF69A48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB398 size=12
    let mut pc: u32 = 0x832AB398;
    'dispatch: loop {
        match pc {
            0x832AB398 => {
    //   block [0x832AB398..0x832AB3A4)
	// 832AB398: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB39C: 386B8AD8  addi r3, r11, -0x7528
	ctx.r[3].s64 = ctx.r[11].s64 + -29992;
	// 832AB3A0: 4AF69A38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB3A8 size=12
    let mut pc: u32 = 0x832AB3A8;
    'dispatch: loop {
        match pc {
            0x832AB3A8 => {
    //   block [0x832AB3A8..0x832AB3B4)
	// 832AB3A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB3AC: 386B8ADC  addi r3, r11, -0x7524
	ctx.r[3].s64 = ctx.r[11].s64 + -29988;
	// 832AB3B0: 4AF69A28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB3B8 size=12
    let mut pc: u32 = 0x832AB3B8;
    'dispatch: loop {
        match pc {
            0x832AB3B8 => {
    //   block [0x832AB3B8..0x832AB3C4)
	// 832AB3B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB3BC: 386B8AE0  addi r3, r11, -0x7520
	ctx.r[3].s64 = ctx.r[11].s64 + -29984;
	// 832AB3C0: 4AF69A18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB3C8 size=12
    let mut pc: u32 = 0x832AB3C8;
    'dispatch: loop {
        match pc {
            0x832AB3C8 => {
    //   block [0x832AB3C8..0x832AB3D4)
	// 832AB3C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB3CC: 386B8AE4  addi r3, r11, -0x751c
	ctx.r[3].s64 = ctx.r[11].s64 + -29980;
	// 832AB3D0: 4AF69A08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB3D8 size=12
    let mut pc: u32 = 0x832AB3D8;
    'dispatch: loop {
        match pc {
            0x832AB3D8 => {
    //   block [0x832AB3D8..0x832AB3E4)
	// 832AB3D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB3DC: 386B8B08  addi r3, r11, -0x74f8
	ctx.r[3].s64 = ctx.r[11].s64 + -29944;
	// 832AB3E0: 4AF699F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB3E8 size=12
    let mut pc: u32 = 0x832AB3E8;
    'dispatch: loop {
        match pc {
            0x832AB3E8 => {
    //   block [0x832AB3E8..0x832AB3F4)
	// 832AB3E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB3EC: 386B8B0C  addi r3, r11, -0x74f4
	ctx.r[3].s64 = ctx.r[11].s64 + -29940;
	// 832AB3F0: 4AF699E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB3F8 size=12
    let mut pc: u32 = 0x832AB3F8;
    'dispatch: loop {
        match pc {
            0x832AB3F8 => {
    //   block [0x832AB3F8..0x832AB404)
	// 832AB3F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB3FC: 386B8B10  addi r3, r11, -0x74f0
	ctx.r[3].s64 = ctx.r[11].s64 + -29936;
	// 832AB400: 4AF699D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB408 size=12
    let mut pc: u32 = 0x832AB408;
    'dispatch: loop {
        match pc {
            0x832AB408 => {
    //   block [0x832AB408..0x832AB414)
	// 832AB408: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB40C: 386B8B14  addi r3, r11, -0x74ec
	ctx.r[3].s64 = ctx.r[11].s64 + -29932;
	// 832AB410: 4AF699C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB418 size=12
    let mut pc: u32 = 0x832AB418;
    'dispatch: loop {
        match pc {
            0x832AB418 => {
    //   block [0x832AB418..0x832AB424)
	// 832AB418: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB41C: 386B8B18  addi r3, r11, -0x74e8
	ctx.r[3].s64 = ctx.r[11].s64 + -29928;
	// 832AB420: 4AF699B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB428 size=12
    let mut pc: u32 = 0x832AB428;
    'dispatch: loop {
        match pc {
            0x832AB428 => {
    //   block [0x832AB428..0x832AB434)
	// 832AB428: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB42C: 386B8B1C  addi r3, r11, -0x74e4
	ctx.r[3].s64 = ctx.r[11].s64 + -29924;
	// 832AB430: 4AF699A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB438 size=12
    let mut pc: u32 = 0x832AB438;
    'dispatch: loop {
        match pc {
            0x832AB438 => {
    //   block [0x832AB438..0x832AB444)
	// 832AB438: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB43C: 386B8B20  addi r3, r11, -0x74e0
	ctx.r[3].s64 = ctx.r[11].s64 + -29920;
	// 832AB440: 4AF69998  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB448 size=12
    let mut pc: u32 = 0x832AB448;
    'dispatch: loop {
        match pc {
            0x832AB448 => {
    //   block [0x832AB448..0x832AB454)
	// 832AB448: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB44C: 386B8B24  addi r3, r11, -0x74dc
	ctx.r[3].s64 = ctx.r[11].s64 + -29916;
	// 832AB450: 4AF69988  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB458 size=104
    let mut pc: u32 = 0x832AB458;
    'dispatch: loop {
        match pc {
            0x832AB458 => {
    //   block [0x832AB458..0x832AB480)
	// 832AB458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB45C: 4B9FDFAD  bl 0x82ca9408
	ctx.lr = 0x832AB460;
	sub_82CA93D0(ctx, base);
	// 832AB460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB464: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB468: 3BC00007  li r30, 7
	ctx.r[30].s64 = 7;
	// 832AB46C: 396B8B28  addi r11, r11, -0x74d8
	ctx.r[11].s64 = ctx.r[11].s64 + -29912;
	// 832AB470: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AB474: 3BEB0020  addi r31, r11, 0x20
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	// 832AB478: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AB47C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832AB480; continue 'dispatch;
            }
            0x832AB480 => {
    //   block [0x832AB480..0x832AB490)
	// 832AB480: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AB484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB488: 4AF1B2E1  bl 0x821c6768
	ctx.lr = 0x832AB48C;
	sub_821C6768(ctx, base);
	// 832AB48C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832AB490; continue 'dispatch;
            }
            0x832AB490 => {
    //   block [0x832AB490..0x832AB4C0)
	// 832AB490: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AB494: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB498: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AB49C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AB4A0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AB4A4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB4A8: 4082FFE8  bne 0x832ab490
	if !ctx.cr[0].eq {
	pc = 0x832AB490; continue 'dispatch;
	}
	// 832AB4AC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AB4B0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AB4B4: 4080FFCC  bge 0x832ab480
	if !ctx.cr[0].lt {
	pc = 0x832AB480; continue 'dispatch;
	}
	// 832AB4B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AB4BC: 4B9FDF9C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB4C0 size=68
    let mut pc: u32 = 0x832AB4C0;
    'dispatch: loop {
        match pc {
            0x832AB4C0 => {
    //   block [0x832AB4C0..0x832AB504)
	// 832AB4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB4C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB4CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB4D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB4D4: 3BEB8B90  addi r31, r11, -0x7470
	ctx.r[31].s64 = ctx.r[11].s64 + -29808;
	// 832AB4D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB4DC: 4B481E5D  bl 0x8272d338
	ctx.lr = 0x832AB4E0;
	sub_8272D338(ctx, base);
	// 832AB4E0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB4E4: 4AF70855  bl 0x8221bd38
	ctx.lr = 0x832AB4E8;
	sub_8221BD38(ctx, base);
	// 832AB4E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB4EC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB4F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB4FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB508 size=12
    let mut pc: u32 = 0x832AB508;
    'dispatch: loop {
        match pc {
            0x832AB508 => {
    //   block [0x832AB508..0x832AB514)
	// 832AB508: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB50C: 386B8B9C  addi r3, r11, -0x7464
	ctx.r[3].s64 = ctx.r[11].s64 + -29796;
	// 832AB510: 4AF698C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB518 size=12
    let mut pc: u32 = 0x832AB518;
    'dispatch: loop {
        match pc {
            0x832AB518 => {
    //   block [0x832AB518..0x832AB524)
	// 832AB518: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB51C: 386B8BA0  addi r3, r11, -0x7460
	ctx.r[3].s64 = ctx.r[11].s64 + -29792;
	// 832AB520: 4AF698B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB528 size=12
    let mut pc: u32 = 0x832AB528;
    'dispatch: loop {
        match pc {
            0x832AB528 => {
    //   block [0x832AB528..0x832AB534)
	// 832AB528: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB52C: 386B8BA4  addi r3, r11, -0x745c
	ctx.r[3].s64 = ctx.r[11].s64 + -29788;
	// 832AB530: 4AF698A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB538 size=12
    let mut pc: u32 = 0x832AB538;
    'dispatch: loop {
        match pc {
            0x832AB538 => {
    //   block [0x832AB538..0x832AB544)
	// 832AB538: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB53C: 386B8BA8  addi r3, r11, -0x7458
	ctx.r[3].s64 = ctx.r[11].s64 + -29784;
	// 832AB540: 4AF69898  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB548 size=12
    let mut pc: u32 = 0x832AB548;
    'dispatch: loop {
        match pc {
            0x832AB548 => {
    //   block [0x832AB548..0x832AB554)
	// 832AB548: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB54C: 386B8BAC  addi r3, r11, -0x7454
	ctx.r[3].s64 = ctx.r[11].s64 + -29780;
	// 832AB550: 4AF69888  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB558 size=12
    let mut pc: u32 = 0x832AB558;
    'dispatch: loop {
        match pc {
            0x832AB558 => {
    //   block [0x832AB558..0x832AB564)
	// 832AB558: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB55C: 386B8BB0  addi r3, r11, -0x7450
	ctx.r[3].s64 = ctx.r[11].s64 + -29776;
	// 832AB560: 4AF69878  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB568 size=104
    let mut pc: u32 = 0x832AB568;
    'dispatch: loop {
        match pc {
            0x832AB568 => {
    //   block [0x832AB568..0x832AB590)
	// 832AB568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB56C: 4B9FDE9D  bl 0x82ca9408
	ctx.lr = 0x832AB570;
	sub_82CA93D0(ctx, base);
	// 832AB570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB574: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AB578: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832AB57C: 396B3B04  addi r11, r11, 0x3b04
	ctx.r[11].s64 = ctx.r[11].s64 + 15108;
	// 832AB580: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AB584: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832AB588: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AB58C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832AB590; continue 'dispatch;
            }
            0x832AB590 => {
    //   block [0x832AB590..0x832AB5A0)
	// 832AB590: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AB594: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB598: 4AF1B1D1  bl 0x821c6768
	ctx.lr = 0x832AB59C;
	sub_821C6768(ctx, base);
	// 832AB59C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832AB5A0; continue 'dispatch;
            }
            0x832AB5A0 => {
    //   block [0x832AB5A0..0x832AB5D0)
	// 832AB5A0: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AB5A4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB5A8: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AB5AC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AB5B0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AB5B4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB5B8: 4082FFE8  bne 0x832ab5a0
	if !ctx.cr[0].eq {
	pc = 0x832AB5A0; continue 'dispatch;
	}
	// 832AB5BC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AB5C0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AB5C4: 4080FFCC  bge 0x832ab590
	if !ctx.cr[0].lt {
	pc = 0x832AB590; continue 'dispatch;
	}
	// 832AB5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AB5CC: 4B9FDE8C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB5E0 size=84
    let mut pc: u32 = 0x832AB5E0;
    'dispatch: loop {
        match pc {
            0x832AB5E0 => {
    //   block [0x832AB5E0..0x832AB608)
	// 832AB5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB5E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB5EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB5F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB5F4: 3BEB8BD0  addi r31, r11, -0x7430
	ctx.r[31].s64 = ctx.r[11].s64 + -29744;
	// 832AB5F8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB5FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AB600: 419A0008  beq cr6, 0x832ab608
	if ctx.cr[6].eq {
	pc = 0x832AB608; continue 'dispatch;
	}
	// 832AB604: 4AF70735  bl 0x8221bd38
	ctx.lr = 0x832AB608;
	sub_8221BD38(ctx, base);
	pc = 0x832AB608; continue 'dispatch;
            }
            0x832AB608 => {
    //   block [0x832AB608..0x832AB634)
	// 832AB608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB60C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB610: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AB614: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB618: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AB61C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AB620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB62C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB638 size=12
    let mut pc: u32 = 0x832AB638;
    'dispatch: loop {
        match pc {
            0x832AB638 => {
    //   block [0x832AB638..0x832AB644)
	// 832AB638: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB63C: 386B8BE4  addi r3, r11, -0x741c
	ctx.r[3].s64 = ctx.r[11].s64 + -29724;
	// 832AB640: 4AF69798  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB648 size=12
    let mut pc: u32 = 0x832AB648;
    'dispatch: loop {
        match pc {
            0x832AB648 => {
    //   block [0x832AB648..0x832AB654)
	// 832AB648: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB64C: 386B8BE8  addi r3, r11, -0x7418
	ctx.r[3].s64 = ctx.r[11].s64 + -29720;
	// 832AB650: 4AF69788  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB658 size=12
    let mut pc: u32 = 0x832AB658;
    'dispatch: loop {
        match pc {
            0x832AB658 => {
    //   block [0x832AB658..0x832AB664)
	// 832AB658: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB65C: 386B8BEC  addi r3, r11, -0x7414
	ctx.r[3].s64 = ctx.r[11].s64 + -29716;
	// 832AB660: 4AF69778  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB668 size=12
    let mut pc: u32 = 0x832AB668;
    'dispatch: loop {
        match pc {
            0x832AB668 => {
    //   block [0x832AB668..0x832AB674)
	// 832AB668: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB66C: 386B8BF0  addi r3, r11, -0x7410
	ctx.r[3].s64 = ctx.r[11].s64 + -29712;
	// 832AB670: 4AF69768  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB678 size=12
    let mut pc: u32 = 0x832AB678;
    'dispatch: loop {
        match pc {
            0x832AB678 => {
    //   block [0x832AB678..0x832AB684)
	// 832AB678: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB67C: 386B8BF4  addi r3, r11, -0x740c
	ctx.r[3].s64 = ctx.r[11].s64 + -29708;
	// 832AB680: 4B366EB8  b 0x82612538
	sub_82612538(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB688 size=20
    let mut pc: u32 = 0x832AB688;
    'dispatch: loop {
        match pc {
            0x832AB688 => {
    //   block [0x832AB688..0x832AB69C)
	// 832AB688: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB68C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832AB690: 396B2B90  addi r11, r11, 0x2b90
	ctx.r[11].s64 = ctx.r[11].s64 + 11152;
	// 832AB694: 916AA0F4  stw r11, -0x5f0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24332 as u32), ctx.r[11].u32 ) };
	// 832AB698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB6A0 size=20
    let mut pc: u32 = 0x832AB6A0;
    'dispatch: loop {
        match pc {
            0x832AB6A0 => {
    //   block [0x832AB6A0..0x832AB6B4)
	// 832AB6A0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832AB6A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB6A8: 392A1C70  addi r9, r10, 0x1c70
	ctx.r[9].s64 = ctx.r[10].s64 + 7280;
	// 832AB6AC: 91690400  stw r11, 0x400(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1024 as u32), ctx.r[11].u32 ) };
	// 832AB6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB6B8 size=20
    let mut pc: u32 = 0x832AB6B8;
    'dispatch: loop {
        match pc {
            0x832AB6B8 => {
    //   block [0x832AB6B8..0x832AB6CC)
	// 832AB6B8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB6BC: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB6C0: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB6C4: 916A8C00  stw r11, -0x7400(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29696 as u32), ctx.r[11].u32 ) };
	// 832AB6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB6D0 size=4
    let mut pc: u32 = 0x832AB6D0;
    'dispatch: loop {
        match pc {
            0x832AB6D0 => {
    //   block [0x832AB6D0..0x832AB6D4)
	// 832AB6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB6D8 size=4
    let mut pc: u32 = 0x832AB6D8;
    'dispatch: loop {
        match pc {
            0x832AB6D8 => {
    //   block [0x832AB6D8..0x832AB6DC)
	// 832AB6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB6E0 size=20
    let mut pc: u32 = 0x832AB6E0;
    'dispatch: loop {
        match pc {
            0x832AB6E0 => {
    //   block [0x832AB6E0..0x832AB6F4)
	// 832AB6E0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB6E4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB6E8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB6EC: 916A8D4C  stw r11, -0x72b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29364 as u32), ctx.r[11].u32 ) };
	// 832AB6F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB6F8 size=20
    let mut pc: u32 = 0x832AB6F8;
    'dispatch: loop {
        match pc {
            0x832AB6F8 => {
    //   block [0x832AB6F8..0x832AB70C)
	// 832AB6F8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB6FC: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB700: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB704: 916A8E60  stw r11, -0x71a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29088 as u32), ctx.r[11].u32 ) };
	// 832AB708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB710 size=20
    let mut pc: u32 = 0x832AB710;
    'dispatch: loop {
        match pc {
            0x832AB710 => {
    //   block [0x832AB710..0x832AB724)
	// 832AB710: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB714: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB718: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB71C: 916A8F74  stw r11, -0x708c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28812 as u32), ctx.r[11].u32 ) };
	// 832AB720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB728 size=20
    let mut pc: u32 = 0x832AB728;
    'dispatch: loop {
        match pc {
            0x832AB728 => {
    //   block [0x832AB728..0x832AB73C)
	// 832AB728: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB72C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB730: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB734: 916A9088  stw r11, -0x6f78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28536 as u32), ctx.r[11].u32 ) };
	// 832AB738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB740 size=20
    let mut pc: u32 = 0x832AB740;
    'dispatch: loop {
        match pc {
            0x832AB740 => {
    //   block [0x832AB740..0x832AB754)
	// 832AB740: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB744: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB748: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB74C: 916A919C  stw r11, -0x6e64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28260 as u32), ctx.r[11].u32 ) };
	// 832AB750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB758 size=68
    let mut pc: u32 = 0x832AB758;
    'dispatch: loop {
        match pc {
            0x832AB758 => {
    //   block [0x832AB758..0x832AB79C)
	// 832AB758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB760: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB764: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB768: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB76C: 3BEB92B0  addi r31, r11, -0x6d50
	ctx.r[31].s64 = ctx.r[11].s64 + -27984;
	// 832AB770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB774: 4B077075  bl 0x823227e8
	ctx.lr = 0x832AB778;
	sub_823227E8(ctx, base);
	// 832AB778: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB77C: 4AF705BD  bl 0x8221bd38
	ctx.lr = 0x832AB780;
	sub_8221BD38(ctx, base);
	// 832AB780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB784: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB7A0 size=20
    let mut pc: u32 = 0x832AB7A0;
    'dispatch: loop {
        match pc {
            0x832AB7A0 => {
    //   block [0x832AB7A0..0x832AB7B4)
	// 832AB7A0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB7A4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB7A8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB7AC: 916A92BC  stw r11, -0x6d44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27972 as u32), ctx.r[11].u32 ) };
	// 832AB7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB7B8 size=20
    let mut pc: u32 = 0x832AB7B8;
    'dispatch: loop {
        match pc {
            0x832AB7B8 => {
    //   block [0x832AB7B8..0x832AB7CC)
	// 832AB7B8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB7BC: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB7C0: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB7C4: 916A93D0  stw r11, -0x6c30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27696 as u32), ctx.r[11].u32 ) };
	// 832AB7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB7D0 size=20
    let mut pc: u32 = 0x832AB7D0;
    'dispatch: loop {
        match pc {
            0x832AB7D0 => {
    //   block [0x832AB7D0..0x832AB7E4)
	// 832AB7D0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB7D4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB7D8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB7DC: 916A94E4  stw r11, -0x6b1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27420 as u32), ctx.r[11].u32 ) };
	// 832AB7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB7E8 size=20
    let mut pc: u32 = 0x832AB7E8;
    'dispatch: loop {
        match pc {
            0x832AB7E8 => {
    //   block [0x832AB7E8..0x832AB7FC)
	// 832AB7E8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB7EC: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB7F0: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB7F4: 916A95F8  stw r11, -0x6a08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27144 as u32), ctx.r[11].u32 ) };
	// 832AB7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB800 size=20
    let mut pc: u32 = 0x832AB800;
    'dispatch: loop {
        match pc {
            0x832AB800 => {
    //   block [0x832AB800..0x832AB814)
	// 832AB800: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB804: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB808: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB80C: 916A970C  stw r11, -0x68f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26868 as u32), ctx.r[11].u32 ) };
	// 832AB810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB818 size=20
    let mut pc: u32 = 0x832AB818;
    'dispatch: loop {
        match pc {
            0x832AB818 => {
    //   block [0x832AB818..0x832AB82C)
	// 832AB818: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB81C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB820: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB824: 916A9824  stw r11, -0x67dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26588 as u32), ctx.r[11].u32 ) };
	// 832AB828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB830 size=20
    let mut pc: u32 = 0x832AB830;
    'dispatch: loop {
        match pc {
            0x832AB830 => {
    //   block [0x832AB830..0x832AB844)
	// 832AB830: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB834: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB838: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB83C: 916A9938  stw r11, -0x66c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26312 as u32), ctx.r[11].u32 ) };
	// 832AB840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB848 size=20
    let mut pc: u32 = 0x832AB848;
    'dispatch: loop {
        match pc {
            0x832AB848 => {
    //   block [0x832AB848..0x832AB85C)
	// 832AB848: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB84C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB850: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB854: 916A9A4C  stw r11, -0x65b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26036 as u32), ctx.r[11].u32 ) };
	// 832AB858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB860 size=20
    let mut pc: u32 = 0x832AB860;
    'dispatch: loop {
        match pc {
            0x832AB860 => {
    //   block [0x832AB860..0x832AB874)
	// 832AB860: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB864: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB868: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB86C: 916A9B60  stw r11, -0x64a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25760 as u32), ctx.r[11].u32 ) };
	// 832AB870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB878 size=12
    let mut pc: u32 = 0x832AB878;
    'dispatch: loop {
        match pc {
            0x832AB878 => {
    //   block [0x832AB878..0x832AB884)
	// 832AB878: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB87C: 386B9C78  addi r3, r11, -0x6388
	ctx.r[3].s64 = ctx.r[11].s64 + -25480;
	// 832AB880: 4B37F990  b 0x8262b210
	sub_8262B210(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB888 size=12
    let mut pc: u32 = 0x832AB888;
    'dispatch: loop {
        match pc {
            0x832AB888 => {
    //   block [0x832AB888..0x832AB894)
	// 832AB888: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AB88C: 386B3BD4  addi r3, r11, 0x3bd4
	ctx.r[3].s64 = ctx.r[11].s64 + 15316;
	// 832AB890: 4B130F98  b 0x823dc828
	sub_823DC828(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB898 size=20
    let mut pc: u32 = 0x832AB898;
    'dispatch: loop {
        match pc {
            0x832AB898 => {
    //   block [0x832AB898..0x832AB8AC)
	// 832AB898: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB89C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB8A0: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB8A4: 916A9C84  stw r11, -0x637c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25468 as u32), ctx.r[11].u32 ) };
	// 832AB8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB8B0 size=20
    let mut pc: u32 = 0x832AB8B0;
    'dispatch: loop {
        match pc {
            0x832AB8B0 => {
    //   block [0x832AB8B0..0x832AB8C4)
	// 832AB8B0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB8B4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB8B8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB8BC: 916A9D98  stw r11, -0x6268(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25192 as u32), ctx.r[11].u32 ) };
	// 832AB8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB8C8 size=68
    let mut pc: u32 = 0x832AB8C8;
    'dispatch: loop {
        match pc {
            0x832AB8C8 => {
    //   block [0x832AB8C8..0x832AB90C)
	// 832AB8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB8D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB8D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB8D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB8DC: 3BEB9EAC  addi r31, r11, -0x6154
	ctx.r[31].s64 = ctx.r[11].s64 + -24916;
	// 832AB8E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB8E4: 4B37FC65  bl 0x8262b548
	ctx.lr = 0x832AB8E8;
	sub_8262B548(ctx, base);
	// 832AB8E8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB8EC: 4AF7044D  bl 0x8221bd38
	ctx.lr = 0x832AB8F0;
	sub_8221BD38(ctx, base);
	// 832AB8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB8F4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB8F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB904: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB910 size=20
    let mut pc: u32 = 0x832AB910;
    'dispatch: loop {
        match pc {
            0x832AB910 => {
    //   block [0x832AB910..0x832AB924)
	// 832AB910: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832AB914: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB918: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832AB91C: 916A9EB8  stw r11, -0x6148(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24904 as u32), ctx.r[11].u32 ) };
	// 832AB920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB928 size=20
    let mut pc: u32 = 0x832AB928;
    'dispatch: loop {
        match pc {
            0x832AB928 => {
    //   block [0x832AB928..0x832AB93C)
	// 832AB928: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832AB92C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB930: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832AB934: 916A9FC4  stw r11, -0x603c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24636 as u32), ctx.r[11].u32 ) };
	// 832AB938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB940 size=20
    let mut pc: u32 = 0x832AB940;
    'dispatch: loop {
        match pc {
            0x832AB940 => {
    //   block [0x832AB940..0x832AB954)
	// 832AB940: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB944: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB948: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB94C: 916AA0D0  stw r11, -0x5f30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24368 as u32), ctx.r[11].u32 ) };
	// 832AB950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB958 size=20
    let mut pc: u32 = 0x832AB958;
    'dispatch: loop {
        match pc {
            0x832AB958 => {
    //   block [0x832AB958..0x832AB96C)
	// 832AB958: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832AB95C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB960: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832AB964: 916AA1E4  stw r11, -0x5e1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24092 as u32), ctx.r[11].u32 ) };
	// 832AB968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB970 size=20
    let mut pc: u32 = 0x832AB970;
    'dispatch: loop {
        match pc {
            0x832AB970 => {
    //   block [0x832AB970..0x832AB984)
	// 832AB970: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832AB974: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB978: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832AB97C: 916AA2F0  stw r11, -0x5d10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23824 as u32), ctx.r[11].u32 ) };
	// 832AB980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB988 size=20
    let mut pc: u32 = 0x832AB988;
    'dispatch: loop {
        match pc {
            0x832AB988 => {
    //   block [0x832AB988..0x832AB99C)
	// 832AB988: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832AB98C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB990: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832AB994: 916AA3FC  stw r11, -0x5c04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23556 as u32), ctx.r[11].u32 ) };
	// 832AB998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB9A0 size=28
    let mut pc: u32 = 0x832AB9A0;
    'dispatch: loop {
        match pc {
            0x832AB9A0 => {
    //   block [0x832AB9A0..0x832AB9BC)
	// 832AB9A0: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832AB9A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB9A8: 392A1C70  addi r9, r10, 0x1c70
	ctx.r[9].s64 = ctx.r[10].s64 + 7280;
	// 832AB9AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB9B0: 91690040  stw r11, 0x40(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 832AB9B4: 99490044  stb r10, 0x44(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 832AB9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB9C0 size=28
    let mut pc: u32 = 0x832AB9C0;
    'dispatch: loop {
        match pc {
            0x832AB9C0 => {
    //   block [0x832AB9C0..0x832AB9DC)
	// 832AB9C0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832AB9C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB9C8: 392A1B30  addi r9, r10, 0x1b30
	ctx.r[9].s64 = ctx.r[10].s64 + 6960;
	// 832AB9CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB9D0: 91690040  stw r11, 0x40(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 832AB9D4: 99490044  stb r10, 0x44(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 832AB9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB9E0 size=28
    let mut pc: u32 = 0x832AB9E0;
    'dispatch: loop {
        match pc {
            0x832AB9E0 => {
    //   block [0x832AB9E0..0x832AB9FC)
	// 832AB9E0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832AB9E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB9E8: 392A1B80  addi r9, r10, 0x1b80
	ctx.r[9].s64 = ctx.r[10].s64 + 7040;
	// 832AB9EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB9F0: 91690040  stw r11, 0x40(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 832AB9F4: 99490044  stb r10, 0x44(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 832AB9F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA00 size=28
    let mut pc: u32 = 0x832ABA00;
    'dispatch: loop {
        match pc {
            0x832ABA00 => {
    //   block [0x832ABA00..0x832ABA1C)
	// 832ABA00: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832ABA04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABA08: 392A1BD0  addi r9, r10, 0x1bd0
	ctx.r[9].s64 = ctx.r[10].s64 + 7120;
	// 832ABA0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832ABA10: 91690040  stw r11, 0x40(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 832ABA14: 99490044  stb r10, 0x44(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 832ABA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA20 size=28
    let mut pc: u32 = 0x832ABA20;
    'dispatch: loop {
        match pc {
            0x832ABA20 => {
    //   block [0x832ABA20..0x832ABA3C)
	// 832ABA20: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832ABA24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABA28: 392A1C20  addi r9, r10, 0x1c20
	ctx.r[9].s64 = ctx.r[10].s64 + 7200;
	// 832ABA2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832ABA30: 91690040  stw r11, 0x40(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 832ABA34: 99490044  stb r10, 0x44(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 832ABA38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA40 size=20
    let mut pc: u32 = 0x832ABA40;
    'dispatch: loop {
        match pc {
            0x832ABA40 => {
    //   block [0x832ABA40..0x832ABA54)
	// 832ABA40: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832ABA44: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832ABA48: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832ABA4C: 916AA508  stw r11, -0x5af8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23288 as u32), ctx.r[11].u32 ) };
	// 832ABA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA58 size=12
    let mut pc: u32 = 0x832ABA58;
    'dispatch: loop {
        match pc {
            0x832ABA58 => {
    //   block [0x832ABA58..0x832ABA64)
	// 832ABA58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABA5C: 386BA61C  addi r3, r11, -0x59e4
	ctx.r[3].s64 = ctx.r[11].s64 + -23012;
	// 832ABA60: 4AF69378  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA68 size=12
    let mut pc: u32 = 0x832ABA68;
    'dispatch: loop {
        match pc {
            0x832ABA68 => {
    //   block [0x832ABA68..0x832ABA74)
	// 832ABA68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABA6C: 386BA620  addi r3, r11, -0x59e0
	ctx.r[3].s64 = ctx.r[11].s64 + -23008;
	// 832ABA70: 4AF69368  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA78 size=12
    let mut pc: u32 = 0x832ABA78;
    'dispatch: loop {
        match pc {
            0x832ABA78 => {
    //   block [0x832ABA78..0x832ABA84)
	// 832ABA78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABA7C: 386BA624  addi r3, r11, -0x59dc
	ctx.r[3].s64 = ctx.r[11].s64 + -23004;
	// 832ABA80: 4AF69358  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA88 size=12
    let mut pc: u32 = 0x832ABA88;
    'dispatch: loop {
        match pc {
            0x832ABA88 => {
    //   block [0x832ABA88..0x832ABA94)
	// 832ABA88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABA8C: 386BA628  addi r3, r11, -0x59d8
	ctx.r[3].s64 = ctx.r[11].s64 + -23000;
	// 832ABA90: 4AF69348  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA98 size=12
    let mut pc: u32 = 0x832ABA98;
    'dispatch: loop {
        match pc {
            0x832ABA98 => {
    //   block [0x832ABA98..0x832ABAA4)
	// 832ABA98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABA9C: 386BA62C  addi r3, r11, -0x59d4
	ctx.r[3].s64 = ctx.r[11].s64 + -22996;
	// 832ABAA0: 4AF69338  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABAA8 size=12
    let mut pc: u32 = 0x832ABAA8;
    'dispatch: loop {
        match pc {
            0x832ABAA8 => {
    //   block [0x832ABAA8..0x832ABAB4)
	// 832ABAA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABAAC: 386BA630  addi r3, r11, -0x59d0
	ctx.r[3].s64 = ctx.r[11].s64 + -22992;
	// 832ABAB0: 4AF69328  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABAB8 size=80
    let mut pc: u32 = 0x832ABAB8;
    'dispatch: loop {
        match pc {
            0x832ABAB8 => {
    //   block [0x832ABAB8..0x832ABADC)
	// 832ABAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABAC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832ABAC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABAC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABACC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABAD0: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832ABAD4: 396BA638  addi r11, r11, -0x59c8
	ctx.r[11].s64 = ctx.r[11].s64 + -22984;
	// 832ABAD8: 3BEB02C0  addi r31, r11, 0x2c0
	ctx.r[31].s64 = ctx.r[11].s64 + 704;
	pc = 0x832ABADC; continue 'dispatch;
            }
            0x832ABADC => {
    //   block [0x832ABADC..0x832ABB08)
	// 832ABADC: 3BFFFF50  addi r31, r31, -0xb0
	ctx.r[31].s64 = ctx.r[31].s64 + -176;
	// 832ABAE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABAE4: 4B3AF085  bl 0x8265ab68
	ctx.lr = 0x832ABAE8;
	sub_8265AB68(ctx, base);
	// 832ABAE8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ABAEC: 4080FFF0  bge 0x832abadc
	if !ctx.cr[0].lt {
	pc = 0x832ABADC; continue 'dispatch;
	}
	// 832ABAF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832ABAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABAFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832ABB00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABB28 size=84
    let mut pc: u32 = 0x832ABB28;
    'dispatch: loop {
        match pc {
            0x832ABB28 => {
    //   block [0x832ABB28..0x832ABB50)
	// 832ABB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABB30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABB34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABB38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABB3C: 3BEBA920  addi r31, r11, -0x56e0
	ctx.r[31].s64 = ctx.r[11].s64 + -22240;
	// 832ABB40: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABB44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832ABB48: 419A0008  beq cr6, 0x832abb50
	if ctx.cr[6].eq {
	pc = 0x832ABB50; continue 'dispatch;
	}
	// 832ABB4C: 4AF701ED  bl 0x8221bd38
	ctx.lr = 0x832ABB50;
	sub_8221BD38(ctx, base);
	pc = 0x832ABB50; continue 'dispatch;
            }
            0x832ABB50 => {
    //   block [0x832ABB50..0x832ABB7C)
	// 832ABB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABB54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832ABB58: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832ABB5C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABB60: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832ABB64: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832ABB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABB74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABB80 size=68
    let mut pc: u32 = 0x832ABB80;
    'dispatch: loop {
        match pc {
            0x832ABB80 => {
    //   block [0x832ABB80..0x832ABBC4)
	// 832ABB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABB88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABB8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABB90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABB94: 3BEBA930  addi r31, r11, -0x56d0
	ctx.r[31].s64 = ctx.r[11].s64 + -22224;
	// 832ABB98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABB9C: 4B3BC5BD  bl 0x82668158
	ctx.lr = 0x832ABBA0;
	sub_82668158(ctx, base);
	// 832ABBA0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABBA4: 4AF70195  bl 0x8221bd38
	ctx.lr = 0x832ABBA8;
	sub_8221BD38(ctx, base);
	// 832ABBA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABBAC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABBB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABBBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABBC8 size=12
    let mut pc: u32 = 0x832ABBC8;
    'dispatch: loop {
        match pc {
            0x832ABBC8 => {
    //   block [0x832ABBC8..0x832ABBD4)
	// 832ABBC8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABBCC: 386BA93C  addi r3, r11, -0x56c4
	ctx.r[3].s64 = ctx.r[11].s64 + -22212;
	// 832ABBD0: 4AF69208  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABBD8 size=12
    let mut pc: u32 = 0x832ABBD8;
    'dispatch: loop {
        match pc {
            0x832ABBD8 => {
    //   block [0x832ABBD8..0x832ABBE4)
	// 832ABBD8: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 832ABBDC: 386B755C  addi r3, r11, 0x755c
	ctx.r[3].s64 = ctx.r[11].s64 + 30044;
	// 832ABBE0: 4AF691F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABBE8 size=12
    let mut pc: u32 = 0x832ABBE8;
    'dispatch: loop {
        match pc {
            0x832ABBE8 => {
    //   block [0x832ABBE8..0x832ABBF4)
	// 832ABBE8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABBEC: 386BA960  addi r3, r11, -0x56a0
	ctx.r[3].s64 = ctx.r[11].s64 + -22176;
	// 832ABBF0: 4AF691E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABBF8 size=12
    let mut pc: u32 = 0x832ABBF8;
    'dispatch: loop {
        match pc {
            0x832ABBF8 => {
    //   block [0x832ABBF8..0x832ABC04)
	// 832ABBF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABBFC: 386BA964  addi r3, r11, -0x569c
	ctx.r[3].s64 = ctx.r[11].s64 + -22172;
	// 832ABC00: 4AF691D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC08 size=12
    let mut pc: u32 = 0x832ABC08;
    'dispatch: loop {
        match pc {
            0x832ABC08 => {
    //   block [0x832ABC08..0x832ABC14)
	// 832ABC08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC0C: 386BA968  addi r3, r11, -0x5698
	ctx.r[3].s64 = ctx.r[11].s64 + -22168;
	// 832ABC10: 4AF691C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC18 size=12
    let mut pc: u32 = 0x832ABC18;
    'dispatch: loop {
        match pc {
            0x832ABC18 => {
    //   block [0x832ABC18..0x832ABC24)
	// 832ABC18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC1C: 386BA96C  addi r3, r11, -0x5694
	ctx.r[3].s64 = ctx.r[11].s64 + -22164;
	// 832ABC20: 4AF691B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC28 size=12
    let mut pc: u32 = 0x832ABC28;
    'dispatch: loop {
        match pc {
            0x832ABC28 => {
    //   block [0x832ABC28..0x832ABC34)
	// 832ABC28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC2C: 386BA970  addi r3, r11, -0x5690
	ctx.r[3].s64 = ctx.r[11].s64 + -22160;
	// 832ABC30: 4AF691A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC38 size=12
    let mut pc: u32 = 0x832ABC38;
    'dispatch: loop {
        match pc {
            0x832ABC38 => {
    //   block [0x832ABC38..0x832ABC44)
	// 832ABC38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC3C: 386BA974  addi r3, r11, -0x568c
	ctx.r[3].s64 = ctx.r[11].s64 + -22156;
	// 832ABC40: 4AF69198  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC48 size=12
    let mut pc: u32 = 0x832ABC48;
    'dispatch: loop {
        match pc {
            0x832ABC48 => {
    //   block [0x832ABC48..0x832ABC54)
	// 832ABC48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC4C: 386BA978  addi r3, r11, -0x5688
	ctx.r[3].s64 = ctx.r[11].s64 + -22152;
	// 832ABC50: 4AF69188  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC58 size=12
    let mut pc: u32 = 0x832ABC58;
    'dispatch: loop {
        match pc {
            0x832ABC58 => {
    //   block [0x832ABC58..0x832ABC64)
	// 832ABC58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC5C: 386BA97C  addi r3, r11, -0x5684
	ctx.r[3].s64 = ctx.r[11].s64 + -22148;
	// 832ABC60: 4AF69178  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC68 size=12
    let mut pc: u32 = 0x832ABC68;
    'dispatch: loop {
        match pc {
            0x832ABC68 => {
    //   block [0x832ABC68..0x832ABC74)
	// 832ABC68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC6C: 386BA980  addi r3, r11, -0x5680
	ctx.r[3].s64 = ctx.r[11].s64 + -22144;
	// 832ABC70: 4B3C1EE0  b 0x8266db50
	sub_8266DB50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC78 size=12
    let mut pc: u32 = 0x832ABC78;
    'dispatch: loop {
        match pc {
            0x832ABC78 => {
    //   block [0x832ABC78..0x832ABC84)
	// 832ABC78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC7C: 386BA98C  addi r3, r11, -0x5674
	ctx.r[3].s64 = ctx.r[11].s64 + -22132;
	// 832ABC80: 4B3C21E0  b 0x8266de60
	sub_8266DE60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC88 size=12
    let mut pc: u32 = 0x832ABC88;
    'dispatch: loop {
        match pc {
            0x832ABC88 => {
    //   block [0x832ABC88..0x832ABC94)
	// 832ABC88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC8C: 386BA998  addi r3, r11, -0x5668
	ctx.r[3].s64 = ctx.r[11].s64 + -22120;
	// 832ABC90: 4B3C7F50  b 0x82673be0
	sub_82673BE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABC98 size=68
    let mut pc: u32 = 0x832ABC98;
    'dispatch: loop {
        match pc {
            0x832ABC98 => {
    //   block [0x832ABC98..0x832ABCDC)
	// 832ABC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABCA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABCA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABCA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABCAC: 3BEBA9A4  addi r31, r11, -0x565c
	ctx.r[31].s64 = ctx.r[11].s64 + -22108;
	// 832ABCB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABCB4: 4B3C80E5  bl 0x82673d98
	ctx.lr = 0x832ABCB8;
	sub_82673D98(ctx, base);
	// 832ABCB8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABCBC: 4AF7007D  bl 0x8221bd38
	ctx.lr = 0x832ABCC0;
	sub_8221BD38(ctx, base);
	// 832ABCC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABCC4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABCC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABCCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABCD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABCD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABCE0 size=84
    let mut pc: u32 = 0x832ABCE0;
    'dispatch: loop {
        match pc {
            0x832ABCE0 => {
    //   block [0x832ABCE0..0x832ABD08)
	// 832ABCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABCE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABCEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABCF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABCF4: 3BEBA9B0  addi r31, r11, -0x5650
	ctx.r[31].s64 = ctx.r[11].s64 + -22096;
	// 832ABCF8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABCFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832ABD00: 419A0008  beq cr6, 0x832abd08
	if ctx.cr[6].eq {
	pc = 0x832ABD08; continue 'dispatch;
	}
	// 832ABD04: 4AF70035  bl 0x8221bd38
	ctx.lr = 0x832ABD08;
	sub_8221BD38(ctx, base);
	pc = 0x832ABD08; continue 'dispatch;
            }
            0x832ABD08 => {
    //   block [0x832ABD08..0x832ABD34)
	// 832ABD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABD0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832ABD10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832ABD14: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABD18: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832ABD1C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832ABD20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABD2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABD38 size=68
    let mut pc: u32 = 0x832ABD38;
    'dispatch: loop {
        match pc {
            0x832ABD38 => {
    //   block [0x832ABD38..0x832ABD7C)
	// 832ABD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABD40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABD44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABD48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABD4C: 3BEBA9D0  addi r31, r11, -0x5630
	ctx.r[31].s64 = ctx.r[11].s64 + -22064;
	// 832ABD50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABD54: 4AEC7185  bl 0x82172ed8
	ctx.lr = 0x832ABD58;
	sub_82172ED8(ctx, base);
	// 832ABD58: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABD5C: 4AF6FFDD  bl 0x8221bd38
	ctx.lr = 0x832ABD60;
	sub_8221BD38(ctx, base);
	// 832ABD60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABD64: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABD68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABD74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABD80 size=68
    let mut pc: u32 = 0x832ABD80;
    'dispatch: loop {
        match pc {
            0x832ABD80 => {
    //   block [0x832ABD80..0x832ABDC4)
	// 832ABD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABD88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABD8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABD90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABD94: 3BEBA9DC  addi r31, r11, -0x5624
	ctx.r[31].s64 = ctx.r[11].s64 + -22052;
	// 832ABD98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABD9C: 4B029055  bl 0x822d4df0
	ctx.lr = 0x832ABDA0;
	sub_822D4DF0(ctx, base);
	// 832ABDA0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABDA4: 4AF6FF95  bl 0x8221bd38
	ctx.lr = 0x832ABDA8;
	sub_8221BD38(ctx, base);
	// 832ABDA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABDAC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABDB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABDBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABDC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABDF0 size=104
    let mut pc: u32 = 0x832ABDF0;
    'dispatch: loop {
        match pc {
            0x832ABDF0 => {
    //   block [0x832ABDF0..0x832ABE18)
	// 832ABDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABDF4: 4B9FD615  bl 0x82ca9408
	ctx.lr = 0x832ABDF8;
	sub_82CA93D0(ctx, base);
	// 832ABDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABDFC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABE00: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 832ABE04: 396BAA08  addi r11, r11, -0x55f8
	ctx.r[11].s64 = ctx.r[11].s64 + -22008;
	// 832ABE08: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832ABE0C: 3BEB0088  addi r31, r11, 0x88
	ctx.r[31].s64 = ctx.r[11].s64 + 136;
	// 832ABE10: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832ABE14: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832ABE18; continue 'dispatch;
            }
            0x832ABE18 => {
    //   block [0x832ABE18..0x832ABE28)
	// 832ABE18: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832ABE1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABE20: 4AF1A949  bl 0x821c6768
	ctx.lr = 0x832ABE24;
	sub_821C6768(ctx, base);
	// 832ABE24: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832ABE28; continue 'dispatch;
            }
            0x832ABE28 => {
    //   block [0x832ABE28..0x832ABE58)
	// 832ABE28: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832ABE2C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ABE30: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832ABE34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832ABE38: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832ABE3C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ABE40: 4082FFE8  bne 0x832abe28
	if !ctx.cr[0].eq {
	pc = 0x832ABE28; continue 'dispatch;
	}
	// 832ABE44: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ABE48: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832ABE4C: 4080FFCC  bge 0x832abe18
	if !ctx.cr[0].lt {
	pc = 0x832ABE18; continue 'dispatch;
	}
	// 832ABE50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832ABE54: 4B9FD604  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABE88 size=100
    let mut pc: u32 = 0x832ABE88;
    'dispatch: loop {
        match pc {
            0x832ABE88 => {
    //   block [0x832ABE88..0x832ABEC0)
	// 832ABE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABE90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABE94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABE98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABE9C: 3BEBAA9C  addi r31, r11, -0x5564
	ctx.r[31].s64 = ctx.r[11].s64 + -21860;
	// 832ABEA0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABEA4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832ABEA8: 419A0018  beq cr6, 0x832abec0
	if ctx.cr[6].eq {
	pc = 0x832ABEC0; continue 'dispatch;
	}
	// 832ABEAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABEB0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832ABEB4: 4B134015  bl 0x823dfec8
	ctx.lr = 0x832ABEB8;
	sub_823DFEC8(ctx, base);
	// 832ABEB8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABEBC: 4AF6FE7D  bl 0x8221bd38
	ctx.lr = 0x832ABEC0;
	sub_8221BD38(ctx, base);
	pc = 0x832ABEC0; continue 'dispatch;
            }
            0x832ABEC0 => {
    //   block [0x832ABEC0..0x832ABEEC)
	// 832ABEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABEC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832ABEC8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832ABECC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABED0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832ABED4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832ABED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABEE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABEF0 size=12
    let mut pc: u32 = 0x832ABEF0;
    'dispatch: loop {
        match pc {
            0x832ABEF0 => {
    //   block [0x832ABEF0..0x832ABEFC)
	// 832ABEF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABEF4: 386BAAB0  addi r3, r11, -0x5550
	ctx.r[3].s64 = ctx.r[11].s64 + -21840;
	// 832ABEF8: 4AF68EE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF00 size=12
    let mut pc: u32 = 0x832ABF00;
    'dispatch: loop {
        match pc {
            0x832ABF00 => {
    //   block [0x832ABF00..0x832ABF0C)
	// 832ABF00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF04: 386BAAB4  addi r3, r11, -0x554c
	ctx.r[3].s64 = ctx.r[11].s64 + -21836;
	// 832ABF08: 4AF68ED0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF10 size=12
    let mut pc: u32 = 0x832ABF10;
    'dispatch: loop {
        match pc {
            0x832ABF10 => {
    //   block [0x832ABF10..0x832ABF1C)
	// 832ABF10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF14: 386BAAB8  addi r3, r11, -0x5548
	ctx.r[3].s64 = ctx.r[11].s64 + -21832;
	// 832ABF18: 4AF68EC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF20 size=12
    let mut pc: u32 = 0x832ABF20;
    'dispatch: loop {
        match pc {
            0x832ABF20 => {
    //   block [0x832ABF20..0x832ABF2C)
	// 832ABF20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF24: 386BAABC  addi r3, r11, -0x5544
	ctx.r[3].s64 = ctx.r[11].s64 + -21828;
	// 832ABF28: 4AF68EB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF30 size=12
    let mut pc: u32 = 0x832ABF30;
    'dispatch: loop {
        match pc {
            0x832ABF30 => {
    //   block [0x832ABF30..0x832ABF3C)
	// 832ABF30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF34: 386BAAC0  addi r3, r11, -0x5540
	ctx.r[3].s64 = ctx.r[11].s64 + -21824;
	// 832ABF38: 4AF68EA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF40 size=12
    let mut pc: u32 = 0x832ABF40;
    'dispatch: loop {
        match pc {
            0x832ABF40 => {
    //   block [0x832ABF40..0x832ABF4C)
	// 832ABF40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF44: 386BAAC4  addi r3, r11, -0x553c
	ctx.r[3].s64 = ctx.r[11].s64 + -21820;
	// 832ABF48: 4AF68E90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF50 size=12
    let mut pc: u32 = 0x832ABF50;
    'dispatch: loop {
        match pc {
            0x832ABF50 => {
    //   block [0x832ABF50..0x832ABF5C)
	// 832ABF50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF54: 386BAAC8  addi r3, r11, -0x5538
	ctx.r[3].s64 = ctx.r[11].s64 + -21816;
	// 832ABF58: 4AF68E80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF60 size=12
    let mut pc: u32 = 0x832ABF60;
    'dispatch: loop {
        match pc {
            0x832ABF60 => {
    //   block [0x832ABF60..0x832ABF6C)
	// 832ABF60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF64: 386BAACC  addi r3, r11, -0x5534
	ctx.r[3].s64 = ctx.r[11].s64 + -21812;
	// 832ABF68: 4AF68E70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF70 size=12
    let mut pc: u32 = 0x832ABF70;
    'dispatch: loop {
        match pc {
            0x832ABF70 => {
    //   block [0x832ABF70..0x832ABF7C)
	// 832ABF70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF74: 386BAAD0  addi r3, r11, -0x5530
	ctx.r[3].s64 = ctx.r[11].s64 + -21808;
	// 832ABF78: 4AF68E60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF80 size=12
    let mut pc: u32 = 0x832ABF80;
    'dispatch: loop {
        match pc {
            0x832ABF80 => {
    //   block [0x832ABF80..0x832ABF8C)
	// 832ABF80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF84: 386BAAD4  addi r3, r11, -0x552c
	ctx.r[3].s64 = ctx.r[11].s64 + -21804;
	// 832ABF88: 4AF68E50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF90 size=12
    let mut pc: u32 = 0x832ABF90;
    'dispatch: loop {
        match pc {
            0x832ABF90 => {
    //   block [0x832ABF90..0x832ABF9C)
	// 832ABF90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF94: 386BAAD8  addi r3, r11, -0x5528
	ctx.r[3].s64 = ctx.r[11].s64 + -21800;
	// 832ABF98: 4AF68E40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFA0 size=12
    let mut pc: u32 = 0x832ABFA0;
    'dispatch: loop {
        match pc {
            0x832ABFA0 => {
    //   block [0x832ABFA0..0x832ABFAC)
	// 832ABFA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFA4: 386BAADC  addi r3, r11, -0x5524
	ctx.r[3].s64 = ctx.r[11].s64 + -21796;
	// 832ABFA8: 4AF68E30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFB0 size=12
    let mut pc: u32 = 0x832ABFB0;
    'dispatch: loop {
        match pc {
            0x832ABFB0 => {
    //   block [0x832ABFB0..0x832ABFBC)
	// 832ABFB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFB4: 386BAAE0  addi r3, r11, -0x5520
	ctx.r[3].s64 = ctx.r[11].s64 + -21792;
	// 832ABFB8: 4AF68E20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFC0 size=12
    let mut pc: u32 = 0x832ABFC0;
    'dispatch: loop {
        match pc {
            0x832ABFC0 => {
    //   block [0x832ABFC0..0x832ABFCC)
	// 832ABFC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFC4: 386BAAE4  addi r3, r11, -0x551c
	ctx.r[3].s64 = ctx.r[11].s64 + -21788;
	// 832ABFC8: 4AF68E10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFD0 size=12
    let mut pc: u32 = 0x832ABFD0;
    'dispatch: loop {
        match pc {
            0x832ABFD0 => {
    //   block [0x832ABFD0..0x832ABFDC)
	// 832ABFD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFD4: 386BAAE8  addi r3, r11, -0x5518
	ctx.r[3].s64 = ctx.r[11].s64 + -21784;
	// 832ABFD8: 4AF68E00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFE0 size=12
    let mut pc: u32 = 0x832ABFE0;
    'dispatch: loop {
        match pc {
            0x832ABFE0 => {
    //   block [0x832ABFE0..0x832ABFEC)
	// 832ABFE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFE4: 386BAAEC  addi r3, r11, -0x5514
	ctx.r[3].s64 = ctx.r[11].s64 + -21780;
	// 832ABFE8: 4AF68DF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFF0 size=12
    let mut pc: u32 = 0x832ABFF0;
    'dispatch: loop {
        match pc {
            0x832ABFF0 => {
    //   block [0x832ABFF0..0x832ABFFC)
	// 832ABFF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFF4: 386BAAF0  addi r3, r11, -0x5510
	ctx.r[3].s64 = ctx.r[11].s64 + -21776;
	// 832ABFF8: 4AF68DE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC000 size=12
    let mut pc: u32 = 0x832AC000;
    'dispatch: loop {
        match pc {
            0x832AC000 => {
    //   block [0x832AC000..0x832AC00C)
	// 832AC000: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC004: 386BAAF4  addi r3, r11, -0x550c
	ctx.r[3].s64 = ctx.r[11].s64 + -21772;
	// 832AC008: 4AF68DD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC010 size=12
    let mut pc: u32 = 0x832AC010;
    'dispatch: loop {
        match pc {
            0x832AC010 => {
    //   block [0x832AC010..0x832AC01C)
	// 832AC010: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC014: 386BAAF8  addi r3, r11, -0x5508
	ctx.r[3].s64 = ctx.r[11].s64 + -21768;
	// 832AC018: 4AF68DC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC020 size=12
    let mut pc: u32 = 0x832AC020;
    'dispatch: loop {
        match pc {
            0x832AC020 => {
    //   block [0x832AC020..0x832AC02C)
	// 832AC020: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC024: 386BAAFC  addi r3, r11, -0x5504
	ctx.r[3].s64 = ctx.r[11].s64 + -21764;
	// 832AC028: 4AF68DB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC030 size=12
    let mut pc: u32 = 0x832AC030;
    'dispatch: loop {
        match pc {
            0x832AC030 => {
    //   block [0x832AC030..0x832AC03C)
	// 832AC030: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC034: 386BAB00  addi r3, r11, -0x5500
	ctx.r[3].s64 = ctx.r[11].s64 + -21760;
	// 832AC038: 4AF68DA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC040 size=12
    let mut pc: u32 = 0x832AC040;
    'dispatch: loop {
        match pc {
            0x832AC040 => {
    //   block [0x832AC040..0x832AC04C)
	// 832AC040: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC044: 386BAB04  addi r3, r11, -0x54fc
	ctx.r[3].s64 = ctx.r[11].s64 + -21756;
	// 832AC048: 4AF68D90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC050 size=12
    let mut pc: u32 = 0x832AC050;
    'dispatch: loop {
        match pc {
            0x832AC050 => {
    //   block [0x832AC050..0x832AC05C)
	// 832AC050: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC054: 386BAB08  addi r3, r11, -0x54f8
	ctx.r[3].s64 = ctx.r[11].s64 + -21752;
	// 832AC058: 4AF68D80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC060 size=12
    let mut pc: u32 = 0x832AC060;
    'dispatch: loop {
        match pc {
            0x832AC060 => {
    //   block [0x832AC060..0x832AC06C)
	// 832AC060: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC064: 386BAB0C  addi r3, r11, -0x54f4
	ctx.r[3].s64 = ctx.r[11].s64 + -21748;
	// 832AC068: 4B075678  b 0x823216e0
	sub_823216E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC070 size=12
    let mut pc: u32 = 0x832AC070;
    'dispatch: loop {
        match pc {
            0x832AC070 => {
    //   block [0x832AC070..0x832AC07C)
	// 832AC070: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC074: 386BAB18  addi r3, r11, -0x54e8
	ctx.r[3].s64 = ctx.r[11].s64 + -21736;
	// 832AC078: 4AF68D60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC080 size=12
    let mut pc: u32 = 0x832AC080;
    'dispatch: loop {
        match pc {
            0x832AC080 => {
    //   block [0x832AC080..0x832AC08C)
	// 832AC080: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC084: 386BAB1C  addi r3, r11, -0x54e4
	ctx.r[3].s64 = ctx.r[11].s64 + -21732;
	// 832AC088: 4AF68D50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC090 size=12
    let mut pc: u32 = 0x832AC090;
    'dispatch: loop {
        match pc {
            0x832AC090 => {
    //   block [0x832AC090..0x832AC09C)
	// 832AC090: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC094: 386BAB20  addi r3, r11, -0x54e0
	ctx.r[3].s64 = ctx.r[11].s64 + -21728;
	// 832AC098: 4AF68D40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC0A0 size=12
    let mut pc: u32 = 0x832AC0A0;
    'dispatch: loop {
        match pc {
            0x832AC0A0 => {
    //   block [0x832AC0A0..0x832AC0AC)
	// 832AC0A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC0A4: 386BAB24  addi r3, r11, -0x54dc
	ctx.r[3].s64 = ctx.r[11].s64 + -21724;
	// 832AC0A8: 4B76BCB8  b 0x82a17d60
	sub_82A17D60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AC0B0 size=104
    let mut pc: u32 = 0x832AC0B0;
    'dispatch: loop {
        match pc {
            0x832AC0B0 => {
    //   block [0x832AC0B0..0x832AC0D8)
	// 832AC0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AC0B4: 4B9FD355  bl 0x82ca9408
	ctx.lr = 0x832AC0B8;
	sub_82CA93D0(ctx, base);
	// 832AC0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AC0BC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC0C0: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 832AC0C4: 396BAB30  addi r11, r11, -0x54d0
	ctx.r[11].s64 = ctx.r[11].s64 + -21712;
	// 832AC0C8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AC0CC: 3BEB0024  addi r31, r11, 0x24
	ctx.r[31].s64 = ctx.r[11].s64 + 36;
	// 832AC0D0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AC0D4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832AC0D8; continue 'dispatch;
            }
            0x832AC0D8 => {
    //   block [0x832AC0D8..0x832AC0E8)
	// 832AC0D8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AC0DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AC0E0: 4AF1A689  bl 0x821c6768
	ctx.lr = 0x832AC0E4;
	sub_821C6768(ctx, base);
	// 832AC0E4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832AC0E8; continue 'dispatch;
            }
            0x832AC0E8 => {
    //   block [0x832AC0E8..0x832AC118)
	// 832AC0E8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AC0EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AC0F0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AC0F4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AC0F8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AC0FC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AC100: 4082FFE8  bne 0x832ac0e8
	if !ctx.cr[0].eq {
	pc = 0x832AC0E8; continue 'dispatch;
	}
	// 832AC104: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AC108: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AC10C: 4080FFCC  bge 0x832ac0d8
	if !ctx.cr[0].lt {
	pc = 0x832AC0D8; continue 'dispatch;
	}
	// 832AC110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AC114: 4B9FD344  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC118 size=4
    let mut pc: u32 = 0x832AC118;
    'dispatch: loop {
        match pc {
            0x832AC118 => {
    //   block [0x832AC118..0x832AC11C)
	// 832AC118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC120 size=4
    let mut pc: u32 = 0x832AC120;
    'dispatch: loop {
        match pc {
            0x832AC120 => {
    //   block [0x832AC120..0x832AC124)
	// 832AC120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC128 size=12
    let mut pc: u32 = 0x832AC128;
    'dispatch: loop {
        match pc {
            0x832AC128 => {
    //   block [0x832AC128..0x832AC134)
	// 832AC128: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AC12C: 386B3D2C  addi r3, r11, 0x3d2c
	ctx.r[3].s64 = ctx.r[11].s64 + 15660;
	// 832AC130: 4AF68CA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC138 size=12
    let mut pc: u32 = 0x832AC138;
    'dispatch: loop {
        match pc {
            0x832AC138 => {
    //   block [0x832AC138..0x832AC144)
	// 832AC138: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC13C: 386BAB54  addi r3, r11, -0x54ac
	ctx.r[3].s64 = ctx.r[11].s64 + -21676;
	// 832AC140: 4AF68C98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC148 size=12
    let mut pc: u32 = 0x832AC148;
    'dispatch: loop {
        match pc {
            0x832AC148 => {
    //   block [0x832AC148..0x832AC154)
	// 832AC148: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC14C: 386BAB58  addi r3, r11, -0x54a8
	ctx.r[3].s64 = ctx.r[11].s64 + -21672;
	// 832AC150: 4AF68C88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC158 size=12
    let mut pc: u32 = 0x832AC158;
    'dispatch: loop {
        match pc {
            0x832AC158 => {
    //   block [0x832AC158..0x832AC164)
	// 832AC158: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC15C: 386BAB5C  addi r3, r11, -0x54a4
	ctx.r[3].s64 = ctx.r[11].s64 + -21668;
	// 832AC160: 4AF68C78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC168 size=12
    let mut pc: u32 = 0x832AC168;
    'dispatch: loop {
        match pc {
            0x832AC168 => {
    //   block [0x832AC168..0x832AC174)
	// 832AC168: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC16C: 386BABB4  addi r3, r11, -0x544c
	ctx.r[3].s64 = ctx.r[11].s64 + -21580;
	// 832AC170: 4AF68C68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC178 size=12
    let mut pc: u32 = 0x832AC178;
    'dispatch: loop {
        match pc {
            0x832AC178 => {
    //   block [0x832AC178..0x832AC184)
	// 832AC178: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC17C: 386BABB8  addi r3, r11, -0x5448
	ctx.r[3].s64 = ctx.r[11].s64 + -21576;
	// 832AC180: 4AF68C58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC188 size=12
    let mut pc: u32 = 0x832AC188;
    'dispatch: loop {
        match pc {
            0x832AC188 => {
    //   block [0x832AC188..0x832AC194)
	// 832AC188: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC18C: 386BABBC  addi r3, r11, -0x5444
	ctx.r[3].s64 = ctx.r[11].s64 + -21572;
	// 832AC190: 4AF68C48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC198 size=12
    let mut pc: u32 = 0x832AC198;
    'dispatch: loop {
        match pc {
            0x832AC198 => {
    //   block [0x832AC198..0x832AC1A4)
	// 832AC198: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC19C: 386BABC0  addi r3, r11, -0x5440
	ctx.r[3].s64 = ctx.r[11].s64 + -21568;
	// 832AC1A0: 4AF68C38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1A8 size=12
    let mut pc: u32 = 0x832AC1A8;
    'dispatch: loop {
        match pc {
            0x832AC1A8 => {
    //   block [0x832AC1A8..0x832AC1B4)
	// 832AC1A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1AC: 386BABC4  addi r3, r11, -0x543c
	ctx.r[3].s64 = ctx.r[11].s64 + -21564;
	// 832AC1B0: 4AF68C28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1B8 size=12
    let mut pc: u32 = 0x832AC1B8;
    'dispatch: loop {
        match pc {
            0x832AC1B8 => {
    //   block [0x832AC1B8..0x832AC1C4)
	// 832AC1B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1BC: 386BABC8  addi r3, r11, -0x5438
	ctx.r[3].s64 = ctx.r[11].s64 + -21560;
	// 832AC1C0: 4AF68C18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1C8 size=12
    let mut pc: u32 = 0x832AC1C8;
    'dispatch: loop {
        match pc {
            0x832AC1C8 => {
    //   block [0x832AC1C8..0x832AC1D4)
	// 832AC1C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1CC: 386BABCC  addi r3, r11, -0x5434
	ctx.r[3].s64 = ctx.r[11].s64 + -21556;
	// 832AC1D0: 4AF68C08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1D8 size=12
    let mut pc: u32 = 0x832AC1D8;
    'dispatch: loop {
        match pc {
            0x832AC1D8 => {
    //   block [0x832AC1D8..0x832AC1E4)
	// 832AC1D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1DC: 386BABD0  addi r3, r11, -0x5430
	ctx.r[3].s64 = ctx.r[11].s64 + -21552;
	// 832AC1E0: 4AF68BF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1E8 size=12
    let mut pc: u32 = 0x832AC1E8;
    'dispatch: loop {
        match pc {
            0x832AC1E8 => {
    //   block [0x832AC1E8..0x832AC1F4)
	// 832AC1E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1EC: 386BABD4  addi r3, r11, -0x542c
	ctx.r[3].s64 = ctx.r[11].s64 + -21548;
	// 832AC1F0: 4AF68BE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1F8 size=12
    let mut pc: u32 = 0x832AC1F8;
    'dispatch: loop {
        match pc {
            0x832AC1F8 => {
    //   block [0x832AC1F8..0x832AC204)
	// 832AC1F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1FC: 386BABD8  addi r3, r11, -0x5428
	ctx.r[3].s64 = ctx.r[11].s64 + -21544;
	// 832AC200: 4AF68BD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC208 size=12
    let mut pc: u32 = 0x832AC208;
    'dispatch: loop {
        match pc {
            0x832AC208 => {
    //   block [0x832AC208..0x832AC214)
	// 832AC208: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC20C: 386BABDC  addi r3, r11, -0x5424
	ctx.r[3].s64 = ctx.r[11].s64 + -21540;
	// 832AC210: 4AF68BC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC218 size=12
    let mut pc: u32 = 0x832AC218;
    'dispatch: loop {
        match pc {
            0x832AC218 => {
    //   block [0x832AC218..0x832AC224)
	// 832AC218: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC21C: 386BABE0  addi r3, r11, -0x5420
	ctx.r[3].s64 = ctx.r[11].s64 + -21536;
	// 832AC220: 4AF68BB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC228 size=12
    let mut pc: u32 = 0x832AC228;
    'dispatch: loop {
        match pc {
            0x832AC228 => {
    //   block [0x832AC228..0x832AC234)
	// 832AC228: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC22C: 386BABE4  addi r3, r11, -0x541c
	ctx.r[3].s64 = ctx.r[11].s64 + -21532;
	// 832AC230: 4AF68BA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC238 size=12
    let mut pc: u32 = 0x832AC238;
    'dispatch: loop {
        match pc {
            0x832AC238 => {
    //   block [0x832AC238..0x832AC244)
	// 832AC238: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC23C: 386BABE8  addi r3, r11, -0x5418
	ctx.r[3].s64 = ctx.r[11].s64 + -21528;
	// 832AC240: 4AF68B98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC248 size=12
    let mut pc: u32 = 0x832AC248;
    'dispatch: loop {
        match pc {
            0x832AC248 => {
    //   block [0x832AC248..0x832AC254)
	// 832AC248: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC24C: 386BABEC  addi r3, r11, -0x5414
	ctx.r[3].s64 = ctx.r[11].s64 + -21524;
	// 832AC250: 4AF68B88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC258 size=12
    let mut pc: u32 = 0x832AC258;
    'dispatch: loop {
        match pc {
            0x832AC258 => {
    //   block [0x832AC258..0x832AC264)
	// 832AC258: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC25C: 386BABF0  addi r3, r11, -0x5410
	ctx.r[3].s64 = ctx.r[11].s64 + -21520;
	// 832AC260: 4AF68B78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC268 size=12
    let mut pc: u32 = 0x832AC268;
    'dispatch: loop {
        match pc {
            0x832AC268 => {
    //   block [0x832AC268..0x832AC274)
	// 832AC268: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC26C: 386BABF4  addi r3, r11, -0x540c
	ctx.r[3].s64 = ctx.r[11].s64 + -21516;
	// 832AC270: 4AF68B68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC278 size=12
    let mut pc: u32 = 0x832AC278;
    'dispatch: loop {
        match pc {
            0x832AC278 => {
    //   block [0x832AC278..0x832AC284)
	// 832AC278: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC27C: 386BABF8  addi r3, r11, -0x5408
	ctx.r[3].s64 = ctx.r[11].s64 + -21512;
	// 832AC280: 4AF68B58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC288 size=12
    let mut pc: u32 = 0x832AC288;
    'dispatch: loop {
        match pc {
            0x832AC288 => {
    //   block [0x832AC288..0x832AC294)
	// 832AC288: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC28C: 386BABFC  addi r3, r11, -0x5404
	ctx.r[3].s64 = ctx.r[11].s64 + -21508;
	// 832AC290: 4AF68B48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


