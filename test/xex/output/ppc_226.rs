pub fn sub_8328F9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8328F9C8 size=96
    let mut pc: u32 = 0x8328F9C8;
    'dispatch: loop {
        match pc {
            0x8328F9C8 => {
    //   block [0x8328F9C8..0x8328FA28)
	// 8328F9C8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328F9CC: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 8328F9D0: 392B9484  addi r9, r11, -0x6b7c
	ctx.r[9].s64 = ctx.r[11].s64 + -27516;
	// 8328F9D4: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 8328F9D8: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 8328F9DC: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 8328F9E0: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8328F9E4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8328F9E8: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 8328F9EC: C1A9000C  lfs f13, 0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8328F9F0: D1A1FFF4  stfs f13, -0xc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 8328F9F4: 388550D0  addi r4, r5, 0x50d0
	ctx.r[4].s64 = ctx.r[5].s64 + 20688;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FA28 size=28
    let mut pc: u32 = 0x8328FA28;
    'dispatch: loop {
        match pc {
            0x8328FA28 => {
    //   block [0x8328FA28..0x8328FA44)
	// 8328FA28: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328FA2C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328FA30: 392B9360  addi r9, r11, -0x6ca0
	ctx.r[9].s64 = ctx.r[11].s64 + -27808;
	// 8328FA34: 390A50E0  addi r8, r10, 0x50e0
	ctx.r[8].s64 = ctx.r[10].s64 + 20704;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FA48 size=28
    let mut pc: u32 = 0x8328FA48;
    'dispatch: loop {
        match pc {
            0x8328FA48 => {
    //   block [0x8328FA48..0x8328FA64)
	// 8328FA48: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328FA4C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328FA50: 392B9370  addi r9, r11, -0x6c90
	ctx.r[9].s64 = ctx.r[11].s64 + -27792;
	// 8328FA54: 390A50F0  addi r8, r10, 0x50f0
	ctx.r[8].s64 = ctx.r[10].s64 + 20720;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FA68 size=28
    let mut pc: u32 = 0x8328FA68;
    'dispatch: loop {
        match pc {
            0x8328FA68 => {
    //   block [0x8328FA68..0x8328FA84)
	// 8328FA68: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328FA6C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328FA70: 392B9380  addi r9, r11, -0x6c80
	ctx.r[9].s64 = ctx.r[11].s64 + -27776;
	// 8328FA74: 390A5100  addi r8, r10, 0x5100
	ctx.r[8].s64 = ctx.r[10].s64 + 20736;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FA88 size=28
    let mut pc: u32 = 0x8328FA88;
    'dispatch: loop {
        match pc {
            0x8328FA88 => {
    //   block [0x8328FA88..0x8328FAA4)
	// 8328FA88: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328FA8C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328FA90: 392B9390  addi r9, r11, -0x6c70
	ctx.r[9].s64 = ctx.r[11].s64 + -27760;
	// 8328FA94: 390A5110  addi r8, r10, 0x5110
	ctx.r[8].s64 = ctx.r[10].s64 + 20752;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FAA8 size=12
    let mut pc: u32 = 0x8328FAA8;
    'dispatch: loop {
        match pc {
            0x8328FAA8 => {
    //   block [0x8328FAA8..0x8328FAB4)
	// 8328FAA8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328FAAC: 386B5868  addi r3, r11, 0x5868
	ctx.r[3].s64 = ctx.r[11].s64 + 22632;
	// 8328FAB0: 4BA1A470  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FAB8 size=12
    let mut pc: u32 = 0x8328FAB8;
    'dispatch: loop {
        match pc {
            0x8328FAB8 => {
    //   block [0x8328FAB8..0x8328FAC4)
	// 8328FAB8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328FABC: 386B58A8  addi r3, r11, 0x58a8
	ctx.r[3].s64 = ctx.r[11].s64 + 22696;
	// 8328FAC0: 4BA1A460  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FAC8 size=12
    let mut pc: u32 = 0x8328FAC8;
    'dispatch: loop {
        match pc {
            0x8328FAC8 => {
    //   block [0x8328FAC8..0x8328FAD4)
	// 8328FAC8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328FACC: 386B58E8  addi r3, r11, 0x58e8
	ctx.r[3].s64 = ctx.r[11].s64 + 22760;
	// 8328FAD0: 4BA1A450  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FAD8 size=12
    let mut pc: u32 = 0x8328FAD8;
    'dispatch: loop {
        match pc {
            0x8328FAD8 => {
    //   block [0x8328FAD8..0x8328FAE4)
	// 8328FAD8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328FADC: 386B5928  addi r3, r11, 0x5928
	ctx.r[3].s64 = ctx.r[11].s64 + 22824;
	// 8328FAE0: 4BA1A440  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FAE8 size=12
    let mut pc: u32 = 0x8328FAE8;
    'dispatch: loop {
        match pc {
            0x8328FAE8 => {
    //   block [0x8328FAE8..0x8328FAF4)
	// 8328FAE8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328FAEC: 386B5968  addi r3, r11, 0x5968
	ctx.r[3].s64 = ctx.r[11].s64 + 22888;
	// 8328FAF0: 4BA1A430  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FAF8 size=40
    let mut pc: u32 = 0x8328FAF8;
    'dispatch: loop {
        match pc {
            0x8328FAF8 => {
    //   block [0x8328FAF8..0x8328FB20)
	// 8328FAF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328FAFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FB00: 396B5144  addi r11, r11, 0x5144
	ctx.r[11].s64 = ctx.r[11].s64 + 20804;
	// 8328FB04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328FB08: 386959A8  addi r3, r9, 0x59a8
	ctx.r[3].s64 = ctx.r[9].s64 + 22952;
	// 8328FB0C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328FB10: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FB14: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8328FB18: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8328FB1C: 4BA1A404  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FB20 size=12
    let mut pc: u32 = 0x8328FB20;
    'dispatch: loop {
        match pc {
            0x8328FB20 => {
    //   block [0x8328FB20..0x8328FB2C)
	// 8328FB20: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328FB24: 386B59C0  addi r3, r11, 0x59c0
	ctx.r[3].s64 = ctx.r[11].s64 + 22976;
	// 8328FB28: 4BA1A3F8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FB30 size=144
    let mut pc: u32 = 0x8328FB30;
    'dispatch: loop {
        match pc {
            0x8328FB30 => {
    //   block [0x8328FB30..0x8328FB54)
	// 8328FB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FB38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FB3C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FB40: 4AF8F719  bl 0x8221f258
	ctx.lr = 0x8328FB44;
	sub_8221F258(ctx, base);
	// 8328FB44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FB48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FB4C: 419A0008  beq cr6, 0x8328fb54
	if ctx.cr[6].eq {
	pc = 0x8328FB54; continue 'dispatch;
	}
	// 8328FB50: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FB54; continue 'dispatch;
            }
            0x8328FB54 => {
    //   block [0x8328FB54..0x8328FB60)
	// 8328FB54: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FB58: 41820008  beq 0x8328fb60
	if ctx.cr[0].eq {
	pc = 0x8328FB60; continue 'dispatch;
	}
	// 8328FB5C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FB60; continue 'dispatch;
            }
            0x8328FB60 => {
    //   block [0x8328FB60..0x8328FB6C)
	// 8328FB60: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FB64: 41820008  beq 0x8328fb6c
	if ctx.cr[0].eq {
	pc = 0x8328FB6C; continue 'dispatch;
	}
	// 8328FB68: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FB6C; continue 'dispatch;
            }
            0x8328FB6C => {
    //   block [0x8328FB6C..0x8328FBC0)
	// 8328FB6C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FB70: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FB74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FB78: 39095158  addi r8, r9, 0x5158
	ctx.r[8].s64 = ctx.r[9].s64 + 20824;
	// 8328FB7C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FB80: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FB84: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FB88: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FB8C: 38675A08  addi r3, r7, 0x5a08
	ctx.r[3].s64 = ctx.r[7].s64 + 23048;
	// 8328FB90: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FB94: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FB98: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FB9C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FBA0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FBA4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FBA8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FBAC: 4BA1A375  bl 0x82ca9f20
	ctx.lr = 0x8328FBB0;
	sub_82CA9F20(ctx, base);
	// 8328FBB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FBC0 size=144
    let mut pc: u32 = 0x8328FBC0;
    'dispatch: loop {
        match pc {
            0x8328FBC0 => {
    //   block [0x8328FBC0..0x8328FBE4)
	// 8328FBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FBC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FBCC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FBD0: 4AF8F689  bl 0x8221f258
	ctx.lr = 0x8328FBD4;
	sub_8221F258(ctx, base);
	// 8328FBD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FBD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FBDC: 419A0008  beq cr6, 0x8328fbe4
	if ctx.cr[6].eq {
	pc = 0x8328FBE4; continue 'dispatch;
	}
	// 8328FBE0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FBE4; continue 'dispatch;
            }
            0x8328FBE4 => {
    //   block [0x8328FBE4..0x8328FBF0)
	// 8328FBE4: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FBE8: 41820008  beq 0x8328fbf0
	if ctx.cr[0].eq {
	pc = 0x8328FBF0; continue 'dispatch;
	}
	// 8328FBEC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FBF0; continue 'dispatch;
            }
            0x8328FBF0 => {
    //   block [0x8328FBF0..0x8328FBFC)
	// 8328FBF0: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FBF4: 41820008  beq 0x8328fbfc
	if ctx.cr[0].eq {
	pc = 0x8328FBFC; continue 'dispatch;
	}
	// 8328FBF8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FBFC; continue 'dispatch;
            }
            0x8328FBFC => {
    //   block [0x8328FBFC..0x8328FC50)
	// 8328FBFC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FC00: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FC04: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FC08: 39095164  addi r8, r9, 0x5164
	ctx.r[8].s64 = ctx.r[9].s64 + 20836;
	// 8328FC0C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FC10: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FC14: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FC18: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FC1C: 38675A18  addi r3, r7, 0x5a18
	ctx.r[3].s64 = ctx.r[7].s64 + 23064;
	// 8328FC20: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FC24: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FC28: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FC2C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FC30: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FC34: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FC38: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FC3C: 4BA1A2E5  bl 0x82ca9f20
	ctx.lr = 0x8328FC40;
	sub_82CA9F20(ctx, base);
	// 8328FC40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FC44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FC48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FC4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FC50 size=144
    let mut pc: u32 = 0x8328FC50;
    'dispatch: loop {
        match pc {
            0x8328FC50 => {
    //   block [0x8328FC50..0x8328FC74)
	// 8328FC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FC58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FC5C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FC60: 4AF8F5F9  bl 0x8221f258
	ctx.lr = 0x8328FC64;
	sub_8221F258(ctx, base);
	// 8328FC64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FC68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FC6C: 419A0008  beq cr6, 0x8328fc74
	if ctx.cr[6].eq {
	pc = 0x8328FC74; continue 'dispatch;
	}
	// 8328FC70: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FC74; continue 'dispatch;
            }
            0x8328FC74 => {
    //   block [0x8328FC74..0x8328FC80)
	// 8328FC74: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FC78: 41820008  beq 0x8328fc80
	if ctx.cr[0].eq {
	pc = 0x8328FC80; continue 'dispatch;
	}
	// 8328FC7C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FC80; continue 'dispatch;
            }
            0x8328FC80 => {
    //   block [0x8328FC80..0x8328FC8C)
	// 8328FC80: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FC84: 41820008  beq 0x8328fc8c
	if ctx.cr[0].eq {
	pc = 0x8328FC8C; continue 'dispatch;
	}
	// 8328FC88: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FC8C; continue 'dispatch;
            }
            0x8328FC8C => {
    //   block [0x8328FC8C..0x8328FCE0)
	// 8328FC8C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FC90: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FC94: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FC98: 39095170  addi r8, r9, 0x5170
	ctx.r[8].s64 = ctx.r[9].s64 + 20848;
	// 8328FC9C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FCA0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FCA4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FCA8: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FCAC: 38675A28  addi r3, r7, 0x5a28
	ctx.r[3].s64 = ctx.r[7].s64 + 23080;
	// 8328FCB0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FCB4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FCB8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FCBC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FCC0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FCC4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FCC8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FCCC: 4BA1A255  bl 0x82ca9f20
	ctx.lr = 0x8328FCD0;
	sub_82CA9F20(ctx, base);
	// 8328FCD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FCD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FCD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FCDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FCE0 size=144
    let mut pc: u32 = 0x8328FCE0;
    'dispatch: loop {
        match pc {
            0x8328FCE0 => {
    //   block [0x8328FCE0..0x8328FD04)
	// 8328FCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FCE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FCEC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FCF0: 4AF8F569  bl 0x8221f258
	ctx.lr = 0x8328FCF4;
	sub_8221F258(ctx, base);
	// 8328FCF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FCF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FCFC: 419A0008  beq cr6, 0x8328fd04
	if ctx.cr[6].eq {
	pc = 0x8328FD04; continue 'dispatch;
	}
	// 8328FD00: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FD04; continue 'dispatch;
            }
            0x8328FD04 => {
    //   block [0x8328FD04..0x8328FD10)
	// 8328FD04: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FD08: 41820008  beq 0x8328fd10
	if ctx.cr[0].eq {
	pc = 0x8328FD10; continue 'dispatch;
	}
	// 8328FD0C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FD10; continue 'dispatch;
            }
            0x8328FD10 => {
    //   block [0x8328FD10..0x8328FD1C)
	// 8328FD10: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FD14: 41820008  beq 0x8328fd1c
	if ctx.cr[0].eq {
	pc = 0x8328FD1C; continue 'dispatch;
	}
	// 8328FD18: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FD1C; continue 'dispatch;
            }
            0x8328FD1C => {
    //   block [0x8328FD1C..0x8328FD70)
	// 8328FD1C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FD20: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FD24: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FD28: 3909517C  addi r8, r9, 0x517c
	ctx.r[8].s64 = ctx.r[9].s64 + 20860;
	// 8328FD2C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FD30: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FD34: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FD38: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FD3C: 38675A38  addi r3, r7, 0x5a38
	ctx.r[3].s64 = ctx.r[7].s64 + 23096;
	// 8328FD40: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FD44: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FD48: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FD4C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FD50: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FD54: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FD58: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FD5C: 4BA1A1C5  bl 0x82ca9f20
	ctx.lr = 0x8328FD60;
	sub_82CA9F20(ctx, base);
	// 8328FD60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FD70 size=144
    let mut pc: u32 = 0x8328FD70;
    'dispatch: loop {
        match pc {
            0x8328FD70 => {
    //   block [0x8328FD70..0x8328FD94)
	// 8328FD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FD78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FD7C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FD80: 4AF8F4D9  bl 0x8221f258
	ctx.lr = 0x8328FD84;
	sub_8221F258(ctx, base);
	// 8328FD84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FD88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FD8C: 419A0008  beq cr6, 0x8328fd94
	if ctx.cr[6].eq {
	pc = 0x8328FD94; continue 'dispatch;
	}
	// 8328FD90: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FD94; continue 'dispatch;
            }
            0x8328FD94 => {
    //   block [0x8328FD94..0x8328FDA0)
	// 8328FD94: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FD98: 41820008  beq 0x8328fda0
	if ctx.cr[0].eq {
	pc = 0x8328FDA0; continue 'dispatch;
	}
	// 8328FD9C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FDA0; continue 'dispatch;
            }
            0x8328FDA0 => {
    //   block [0x8328FDA0..0x8328FDAC)
	// 8328FDA0: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FDA4: 41820008  beq 0x8328fdac
	if ctx.cr[0].eq {
	pc = 0x8328FDAC; continue 'dispatch;
	}
	// 8328FDA8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FDAC; continue 'dispatch;
            }
            0x8328FDAC => {
    //   block [0x8328FDAC..0x8328FE00)
	// 8328FDAC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FDB0: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FDB4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FDB8: 39095188  addi r8, r9, 0x5188
	ctx.r[8].s64 = ctx.r[9].s64 + 20872;
	// 8328FDBC: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FDC0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FDC4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FDC8: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FDCC: 38675A48  addi r3, r7, 0x5a48
	ctx.r[3].s64 = ctx.r[7].s64 + 23112;
	// 8328FDD0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FDD4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FDD8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FDDC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FDE0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FDE4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FDE8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FDEC: 4BA1A135  bl 0x82ca9f20
	ctx.lr = 0x8328FDF0;
	sub_82CA9F20(ctx, base);
	// 8328FDF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FE00 size=64
    let mut pc: u32 = 0x8328FE00;
    'dispatch: loop {
        match pc {
            0x8328FE00 => {
    //   block [0x8328FE00..0x8328FE40)
	// 8328FE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FE0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328FE10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328FE14: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328FE18: 386A5194  addi r3, r10, 0x5194
	ctx.r[3].s64 = ctx.r[10].s64 + 20884;
	// 8328FE1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328FE20: 4AF9D0B1  bl 0x8222ced0
	ctx.lr = 0x8328FE24;
	sub_8222CED0(ctx, base);
	// 8328FE24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328FE28: 38695A58  addi r3, r9, 0x5a58
	ctx.r[3].s64 = ctx.r[9].s64 + 23128;
	// 8328FE2C: 4BA1A0F5  bl 0x82ca9f20
	ctx.lr = 0x8328FE30;
	sub_82CA9F20(ctx, base);
	// 8328FE30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FE40 size=64
    let mut pc: u32 = 0x8328FE40;
    'dispatch: loop {
        match pc {
            0x8328FE40 => {
    //   block [0x8328FE40..0x8328FE80)
	// 8328FE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FE48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FE4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328FE50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328FE54: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328FE58: 386A5198  addi r3, r10, 0x5198
	ctx.r[3].s64 = ctx.r[10].s64 + 20888;
	// 8328FE5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328FE60: 4AF9D071  bl 0x8222ced0
	ctx.lr = 0x8328FE64;
	sub_8222CED0(ctx, base);
	// 8328FE64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328FE68: 38695A68  addi r3, r9, 0x5a68
	ctx.r[3].s64 = ctx.r[9].s64 + 23144;
	// 8328FE6C: 4BA1A0B5  bl 0x82ca9f20
	ctx.lr = 0x8328FE70;
	sub_82CA9F20(ctx, base);
	// 8328FE70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FE74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FE78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FE80 size=444
    let mut pc: u32 = 0x8328FE80;
    'dispatch: loop {
        match pc {
            0x8328FE80 => {
    //   block [0x8328FE80..0x8329003C)
	// 8328FE80: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 8328FE84: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8328FE88: 38C851A0  addi r6, r8, 0x51a0
	ctx.r[6].s64 = ctx.r[8].s64 + 20896;
	// 8328FE8C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8328FE90: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8328FE94: 916851A0  stw r11, 0x51a0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(20896 as u32), ctx.r[11].u32 ) };
	// 8328FE98: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FE9C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8328FEA0: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328FEA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FEA8: 91260008  stw r9, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8328FEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8328FEB0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8328FEB4: 91660010  stw r11, 0x10(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8328FEB8: 91460018  stw r10, 0x18(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8328FEBC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8328FEC0: 9926001C  stb r9, 0x1c(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(28 as u32), ctx.r[9].u8 ) };
	// 8328FEC4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8328FEC8: 9106000C  stw r8, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8328FECC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8328FED0: 90E60014  stw r7, 0x14(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8328FED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8328FED8: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 8328FEDC: 91660024  stw r11, 0x24(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8328FEE0: 9146002C  stw r10, 0x2c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8328FEE4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FEE8: 91260030  stw r9, 0x30(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 8328FEEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FEF0: 91060020  stw r8, 0x20(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 8328FEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8328FEF8: 90E60028  stw r7, 0x28(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 8328FEFC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8328FF00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8328FF04: 91660038  stw r11, 0x38(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8328FF08: 99460040  stb r10, 0x40(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(64 as u32), ctx.r[10].u8 ) };
	// 8328FF0C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8328FF10: 91260044  stw r9, 0x44(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 8328FF14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8328FF18: 91060034  stw r8, 0x34(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(52 as u32), ctx.r[8].u32 ) };
	// 8328FF1C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8328FF20: 90E6003C  stw r7, 0x3c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(60 as u32), ctx.r[7].u32 ) };
	// 8328FF24: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8328FF28: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 8328FF2C: 9166004C  stw r11, 0x4c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8328FF30: 91460054  stw r10, 0x54(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8328FF34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328FF38: 91260058  stw r9, 0x58(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 8328FF3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FF40: 91060048  stw r8, 0x48(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(72 as u32), ctx.r[8].u32 ) };
	// 8328FF44: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8328FF48: 90E60050  stw r7, 0x50(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 8328FF4C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8328FF50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8328FF54: 91660060  stw r11, 0x60(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8328FF58: 91460068  stw r10, 0x68(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 8328FF5C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8328FF60: 9126006C  stw r9, 0x6c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8328FF64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8328FF68: 9106005C  stw r8, 0x5c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8328FF6C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8328FF70: 98E60064  stb r7, 0x64(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(100 as u32), ctx.r[7].u8 ) };
	// 8328FF74: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8328FF78: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8328FF7C: 91660074  stw r11, 0x74(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8328FF80: 9146007C  stw r10, 0x7c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 8328FF84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328FF88: 91260080  stw r9, 0x80(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 8328FF8C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8328FF90: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8328FF94: 91060070  stw r8, 0x70(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 8328FF98: 90E60078  stw r7, 0x78(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(120 as u32), ctx.r[7].u32 ) };
	// 8328FF9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8328FFA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8328FFA4: 99660088  stb r11, 0x88(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(136 as u32), ctx.r[11].u8 ) };
	// 8328FFA8: 91460090  stw r10, 0x90(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 8328FFAC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8328FFB0: 91260094  stw r9, 0x94(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 8328FFB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8328FFB8: 91060084  stw r8, 0x84(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(132 as u32), ctx.r[8].u32 ) };
	// 8328FFBC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8328FFC0: 90E6008C  stw r7, 0x8c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(140 as u32), ctx.r[7].u32 ) };
	// 8328FFC4: 91660098  stw r11, 0x98(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 8328FFC8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FFCC: 9146009C  stw r10, 0x9c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 8328FFD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FFD4: 912600A0  stw r9, 0xa0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(160 as u32), ctx.r[9].u32 ) };
	// 8328FFD8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8328FFDC: 916600A4  stw r11, 0xa4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 8328FFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328FFE4: 914600A8  stw r10, 0xa8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(168 as u32), ctx.r[10].u32 ) };
	// 8328FFE8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8328FFEC: 992600AC  stb r9, 0xac(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(172 as u32), ctx.r[9].u8 ) };
	// 8328FFF0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8328FFF4: 916600B0  stw r11, 0xb0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8328FFF8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8328FFFC: 914600B4  stw r10, 0xb4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 83290000: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83290004: 912600B8  stw r9, 0xb8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(184 as u32), ctx.r[9].u32 ) };
	// 83290008: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329000C: 916600BC  stw r11, 0xbc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 83290010: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83290014: 914600C0  stw r10, 0xc0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(192 as u32), ctx.r[10].u32 ) };
	// 83290018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329001C: 912600C4  stw r9, 0xc4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(196 as u32), ctx.r[9].u32 ) };
	// 83290020: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83290024: 916600C8  stw r11, 0xc8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 83290028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329002C: 914600CC  stw r10, 0xcc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(204 as u32), ctx.r[10].u32 ) };
	// 83290030: 992600D0  stb r9, 0xd0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(208 as u32), ctx.r[9].u8 ) };
	// 83290034: 916600D4  stw r11, 0xd4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 83290038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290040 size=64
    let mut pc: u32 = 0x83290040;
    'dispatch: loop {
        match pc {
            0x83290040 => {
    //   block [0x83290040..0x83290080)
	// 83290040: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83290044: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83290048: 38C85278  addi r6, r8, 0x5278
	ctx.r[6].s64 = ctx.r[8].s64 + 21112;
	// 8329004C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83290050: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83290054: 91685278  stw r11, 0x5278(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(21112 as u32), ctx.r[11].u32 ) };
	// 83290058: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329005C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83290060: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83290064: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83290068: 91260008  stw r9, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8329006C: 9106000C  stw r8, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 83290070: 38655A78  addi r3, r5, 0x5a78
	ctx.r[3].s64 = ctx.r[5].s64 + 23160;
	// 83290074: 91660010  stw r11, 0x10(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83290078: 90E60014  stw r7, 0x14(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8329007C: 4BA19EA4  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290080 size=64
    let mut pc: u32 = 0x83290080;
    'dispatch: loop {
        match pc {
            0x83290080 => {
    //   block [0x83290080..0x832900C0)
	// 83290080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329008C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290090: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290094: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290098: 386A5290  addi r3, r10, 0x5290
	ctx.r[3].s64 = ctx.r[10].s64 + 21136;
	// 8329009C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832900A0: 4AF9CE31  bl 0x8222ced0
	ctx.lr = 0x832900A4;
	sub_8222CED0(ctx, base);
	// 832900A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832900A8: 38695A80  addi r3, r9, 0x5a80
	ctx.r[3].s64 = ctx.r[9].s64 + 23168;
	// 832900AC: 4BA19E75  bl 0x82ca9f20
	ctx.lr = 0x832900B0;
	sub_82CA9F20(ctx, base);
	// 832900B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832900B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832900B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832900BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832900C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832900C0 size=64
    let mut pc: u32 = 0x832900C0;
    'dispatch: loop {
        match pc {
            0x832900C0 => {
    //   block [0x832900C0..0x83290100)
	// 832900C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832900C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832900C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832900CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832900D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832900D4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832900D8: 386A5294  addi r3, r10, 0x5294
	ctx.r[3].s64 = ctx.r[10].s64 + 21140;
	// 832900DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832900E0: 4AF9CDF1  bl 0x8222ced0
	ctx.lr = 0x832900E4;
	sub_8222CED0(ctx, base);
	// 832900E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832900E8: 38695A90  addi r3, r9, 0x5a90
	ctx.r[3].s64 = ctx.r[9].s64 + 23184;
	// 832900EC: 4BA19E35  bl 0x82ca9f20
	ctx.lr = 0x832900F0;
	sub_82CA9F20(ctx, base);
	// 832900F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832900F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832900F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832900FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290100 size=52
    let mut pc: u32 = 0x83290100;
    'dispatch: loop {
        match pc {
            0x83290100 => {
    //   block [0x83290100..0x83290134)
	// 83290100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329010C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83290110: 386B5298  addi r3, r11, 0x5298
	ctx.r[3].s64 = ctx.r[11].s64 + 21144;
	// 83290114: 4AFFBC45  bl 0x8228bd58
	ctx.lr = 0x83290118;
	sub_8228BD58(ctx, base);
	// 83290118: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329011C: 386A5AA0  addi r3, r10, 0x5aa0
	ctx.r[3].s64 = ctx.r[10].s64 + 23200;
	// 83290120: 4BA19E01  bl 0x82ca9f20
	ctx.lr = 0x83290124;
	sub_82CA9F20(ctx, base);
	// 83290124: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329012C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290138 size=76
    let mut pc: u32 = 0x83290138;
    'dispatch: loop {
        match pc {
            0x83290138 => {
    //   block [0x83290138..0x83290184)
	// 83290138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329013C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290144: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83290148: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8329014C: 388B7F00  addi r4, r11, 0x7f00
	ctx.r[4].s64 = ctx.r[11].s64 + 32512;
	// 83290150: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83290154: 4AF9CD7D  bl 0x8222ced0
	ctx.lr = 0x83290158;
	sub_8222CED0(ctx, base);
	// 83290158: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329015C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83290160: 386A5654  addi r3, r10, 0x5654
	ctx.r[3].s64 = ctx.r[10].s64 + 22100;
	// 83290164: 4B01B3A5  bl 0x822ab508
	ctx.lr = 0x83290168;
	sub_822AB508(ctx, base);
	// 83290168: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8329016C: 38695AB0  addi r3, r9, 0x5ab0
	ctx.r[3].s64 = ctx.r[9].s64 + 23216;
	// 83290170: 4BA19DB1  bl 0x82ca9f20
	ctx.lr = 0x83290174;
	sub_82CA9F20(ctx, base);
	// 83290174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329017C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290188 size=64
    let mut pc: u32 = 0x83290188;
    'dispatch: loop {
        match pc {
            0x83290188 => {
    //   block [0x83290188..0x832901C8)
	// 83290188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329018C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290194: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290198: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329019C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832901A0: 386A56A0  addi r3, r10, 0x56a0
	ctx.r[3].s64 = ctx.r[10].s64 + 22176;
	// 832901A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832901A8: 4AF9CD29  bl 0x8222ced0
	ctx.lr = 0x832901AC;
	sub_8222CED0(ctx, base);
	// 832901AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832901B0: 38695AC0  addi r3, r9, 0x5ac0
	ctx.r[3].s64 = ctx.r[9].s64 + 23232;
	// 832901B4: 4BA19D6D  bl 0x82ca9f20
	ctx.lr = 0x832901B8;
	sub_82CA9F20(ctx, base);
	// 832901B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832901BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832901C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832901C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832901C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832901C8 size=64
    let mut pc: u32 = 0x832901C8;
    'dispatch: loop {
        match pc {
            0x832901C8 => {
    //   block [0x832901C8..0x83290208)
	// 832901C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832901CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832901D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832901D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832901D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832901DC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832901E0: 386A56A4  addi r3, r10, 0x56a4
	ctx.r[3].s64 = ctx.r[10].s64 + 22180;
	// 832901E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832901E8: 4AF9CCE9  bl 0x8222ced0
	ctx.lr = 0x832901EC;
	sub_8222CED0(ctx, base);
	// 832901EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832901F0: 38695AD0  addi r3, r9, 0x5ad0
	ctx.r[3].s64 = ctx.r[9].s64 + 23248;
	// 832901F4: 4BA19D2D  bl 0x82ca9f20
	ctx.lr = 0x832901F8;
	sub_82CA9F20(ctx, base);
	// 832901F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832901FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290208 size=64
    let mut pc: u32 = 0x83290208;
    'dispatch: loop {
        match pc {
            0x83290208 => {
    //   block [0x83290208..0x83290248)
	// 83290208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329020C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290214: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290218: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329021C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290220: 386A56A8  addi r3, r10, 0x56a8
	ctx.r[3].s64 = ctx.r[10].s64 + 22184;
	// 83290224: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290228: 4AF9CCA9  bl 0x8222ced0
	ctx.lr = 0x8329022C;
	sub_8222CED0(ctx, base);
	// 8329022C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290230: 38695AE0  addi r3, r9, 0x5ae0
	ctx.r[3].s64 = ctx.r[9].s64 + 23264;
	// 83290234: 4BA19CED  bl 0x82ca9f20
	ctx.lr = 0x83290238;
	sub_82CA9F20(ctx, base);
	// 83290238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329023C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290248 size=64
    let mut pc: u32 = 0x83290248;
    'dispatch: loop {
        match pc {
            0x83290248 => {
    //   block [0x83290248..0x83290288)
	// 83290248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329024C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290254: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290258: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329025C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290260: 386A56AC  addi r3, r10, 0x56ac
	ctx.r[3].s64 = ctx.r[10].s64 + 22188;
	// 83290264: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290268: 4AF9CC69  bl 0x8222ced0
	ctx.lr = 0x8329026C;
	sub_8222CED0(ctx, base);
	// 8329026C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290270: 38695AF0  addi r3, r9, 0x5af0
	ctx.r[3].s64 = ctx.r[9].s64 + 23280;
	// 83290274: 4BA19CAD  bl 0x82ca9f20
	ctx.lr = 0x83290278;
	sub_82CA9F20(ctx, base);
	// 83290278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329027C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290288 size=64
    let mut pc: u32 = 0x83290288;
    'dispatch: loop {
        match pc {
            0x83290288 => {
    //   block [0x83290288..0x832902C8)
	// 83290288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329028C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290294: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290298: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329029C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832902A0: 386A56B0  addi r3, r10, 0x56b0
	ctx.r[3].s64 = ctx.r[10].s64 + 22192;
	// 832902A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832902A8: 4AF9CC29  bl 0x8222ced0
	ctx.lr = 0x832902AC;
	sub_8222CED0(ctx, base);
	// 832902AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832902B0: 38695B00  addi r3, r9, 0x5b00
	ctx.r[3].s64 = ctx.r[9].s64 + 23296;
	// 832902B4: 4BA19C6D  bl 0x82ca9f20
	ctx.lr = 0x832902B8;
	sub_82CA9F20(ctx, base);
	// 832902B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832902BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832902C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832902C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832902C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832902C8 size=64
    let mut pc: u32 = 0x832902C8;
    'dispatch: loop {
        match pc {
            0x832902C8 => {
    //   block [0x832902C8..0x83290308)
	// 832902C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832902CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832902D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832902D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832902D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832902DC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832902E0: 386A56B4  addi r3, r10, 0x56b4
	ctx.r[3].s64 = ctx.r[10].s64 + 22196;
	// 832902E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832902E8: 4AF9CBE9  bl 0x8222ced0
	ctx.lr = 0x832902EC;
	sub_8222CED0(ctx, base);
	// 832902EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832902F0: 38695B10  addi r3, r9, 0x5b10
	ctx.r[3].s64 = ctx.r[9].s64 + 23312;
	// 832902F4: 4BA19C2D  bl 0x82ca9f20
	ctx.lr = 0x832902F8;
	sub_82CA9F20(ctx, base);
	// 832902F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832902FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290308 size=64
    let mut pc: u32 = 0x83290308;
    'dispatch: loop {
        match pc {
            0x83290308 => {
    //   block [0x83290308..0x83290348)
	// 83290308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329030C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290314: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329031C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290320: 386A56B8  addi r3, r10, 0x56b8
	ctx.r[3].s64 = ctx.r[10].s64 + 22200;
	// 83290324: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290328: 4AF9CBA9  bl 0x8222ced0
	ctx.lr = 0x8329032C;
	sub_8222CED0(ctx, base);
	// 8329032C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290330: 38695B20  addi r3, r9, 0x5b20
	ctx.r[3].s64 = ctx.r[9].s64 + 23328;
	// 83290334: 4BA19BED  bl 0x82ca9f20
	ctx.lr = 0x83290338;
	sub_82CA9F20(ctx, base);
	// 83290338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329033C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290348 size=64
    let mut pc: u32 = 0x83290348;
    'dispatch: loop {
        match pc {
            0x83290348 => {
    //   block [0x83290348..0x83290388)
	// 83290348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329034C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290354: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329035C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290360: 386A56BC  addi r3, r10, 0x56bc
	ctx.r[3].s64 = ctx.r[10].s64 + 22204;
	// 83290364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290368: 4AF9CB69  bl 0x8222ced0
	ctx.lr = 0x8329036C;
	sub_8222CED0(ctx, base);
	// 8329036C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290370: 38695B30  addi r3, r9, 0x5b30
	ctx.r[3].s64 = ctx.r[9].s64 + 23344;
	// 83290374: 4BA19BAD  bl 0x82ca9f20
	ctx.lr = 0x83290378;
	sub_82CA9F20(ctx, base);
	// 83290378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329037C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290388 size=96
    let mut pc: u32 = 0x83290388;
    'dispatch: loop {
        match pc {
            0x83290388 => {
    //   block [0x83290388..0x832903AC)
	// 83290388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329038C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290394: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 83290398: 4AF8EEC1  bl 0x8221f258
	ctx.lr = 0x8329039C;
	sub_8221F258(ctx, base);
	// 8329039C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832903A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832903A4: 419A0008  beq cr6, 0x832903ac
	if ctx.cr[6].eq {
	pc = 0x832903AC; continue 'dispatch;
	}
	// 832903A8: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x832903AC; continue 'dispatch;
            }
            0x832903AC => {
    //   block [0x832903AC..0x832903B8)
	// 832903AC: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832903B0: 41820008  beq 0x832903b8
	if ctx.cr[0].eq {
	pc = 0x832903B8; continue 'dispatch;
	}
	// 832903B4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x832903B8; continue 'dispatch;
            }
            0x832903B8 => {
    //   block [0x832903B8..0x832903E8)
	// 832903B8: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832903BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832903C0: 390956C0  addi r8, r9, 0x56c0
	ctx.r[8].s64 = ctx.r[9].s64 + 22208;
	// 832903C4: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832903C8: 38675B40  addi r3, r7, 0x5b40
	ctx.r[3].s64 = ctx.r[7].s64 + 23360;
	// 832903CC: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832903D0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832903D4: 4BA19B4D  bl 0x82ca9f20
	ctx.lr = 0x832903D8;
	sub_82CA9F20(ctx, base);
	// 832903D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832903DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832903E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832903E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832903E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832903E8 size=96
    let mut pc: u32 = 0x832903E8;
    'dispatch: loop {
        match pc {
            0x832903E8 => {
    //   block [0x832903E8..0x8329040C)
	// 832903E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832903EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832903F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832903F4: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 832903F8: 4AF8EE61  bl 0x8221f258
	ctx.lr = 0x832903FC;
	sub_8221F258(ctx, base);
	// 832903FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83290400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83290404: 419A0008  beq cr6, 0x8329040c
	if ctx.cr[6].eq {
	pc = 0x8329040C; continue 'dispatch;
	}
	// 83290408: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8329040C; continue 'dispatch;
            }
            0x8329040C => {
    //   block [0x8329040C..0x83290418)
	// 8329040C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83290410: 41820008  beq 0x83290418
	if ctx.cr[0].eq {
	pc = 0x83290418; continue 'dispatch;
	}
	// 83290414: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83290418; continue 'dispatch;
            }
            0x83290418 => {
    //   block [0x83290418..0x83290448)
	// 83290418: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8329041C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83290420: 390956CC  addi r8, r9, 0x56cc
	ctx.r[8].s64 = ctx.r[9].s64 + 22220;
	// 83290424: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83290428: 38675B88  addi r3, r7, 0x5b88
	ctx.r[3].s64 = ctx.r[7].s64 + 23432;
	// 8329042C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83290430: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83290434: 4BA19AED  bl 0x82ca9f20
	ctx.lr = 0x83290438;
	sub_82CA9F20(ctx, base);
	// 83290438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329043C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290448 size=96
    let mut pc: u32 = 0x83290448;
    'dispatch: loop {
        match pc {
            0x83290448 => {
    //   block [0x83290448..0x8329046C)
	// 83290448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329044C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290454: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 83290458: 4AF8EE01  bl 0x8221f258
	ctx.lr = 0x8329045C;
	sub_8221F258(ctx, base);
	// 8329045C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83290460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83290464: 419A0008  beq cr6, 0x8329046c
	if ctx.cr[6].eq {
	pc = 0x8329046C; continue 'dispatch;
	}
	// 83290468: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8329046C; continue 'dispatch;
            }
            0x8329046C => {
    //   block [0x8329046C..0x83290478)
	// 8329046C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83290470: 41820008  beq 0x83290478
	if ctx.cr[0].eq {
	pc = 0x83290478; continue 'dispatch;
	}
	// 83290474: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83290478; continue 'dispatch;
            }
            0x83290478 => {
    //   block [0x83290478..0x832904A8)
	// 83290478: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8329047C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83290480: 390956D8  addi r8, r9, 0x56d8
	ctx.r[8].s64 = ctx.r[9].s64 + 22232;
	// 83290484: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83290488: 38675BD0  addi r3, r7, 0x5bd0
	ctx.r[3].s64 = ctx.r[7].s64 + 23504;
	// 8329048C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83290490: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83290494: 4BA19A8D  bl 0x82ca9f20
	ctx.lr = 0x83290498;
	sub_82CA9F20(ctx, base);
	// 83290498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329049C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832904A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832904A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832904A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832904A8 size=96
    let mut pc: u32 = 0x832904A8;
    'dispatch: loop {
        match pc {
            0x832904A8 => {
    //   block [0x832904A8..0x832904CC)
	// 832904A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832904AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832904B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832904B4: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 832904B8: 4AF8EDA1  bl 0x8221f258
	ctx.lr = 0x832904BC;
	sub_8221F258(ctx, base);
	// 832904BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832904C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832904C4: 419A0008  beq cr6, 0x832904cc
	if ctx.cr[6].eq {
	pc = 0x832904CC; continue 'dispatch;
	}
	// 832904C8: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x832904CC; continue 'dispatch;
            }
            0x832904CC => {
    //   block [0x832904CC..0x832904D8)
	// 832904CC: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832904D0: 41820008  beq 0x832904d8
	if ctx.cr[0].eq {
	pc = 0x832904D8; continue 'dispatch;
	}
	// 832904D4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x832904D8; continue 'dispatch;
            }
            0x832904D8 => {
    //   block [0x832904D8..0x83290508)
	// 832904D8: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832904DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832904E0: 390956E4  addi r8, r9, 0x56e4
	ctx.r[8].s64 = ctx.r[9].s64 + 22244;
	// 832904E4: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832904E8: 38675C18  addi r3, r7, 0x5c18
	ctx.r[3].s64 = ctx.r[7].s64 + 23576;
	// 832904EC: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832904F0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832904F4: 4BA19A2D  bl 0x82ca9f20
	ctx.lr = 0x832904F8;
	sub_82CA9F20(ctx, base);
	// 832904F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832904FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290508 size=96
    let mut pc: u32 = 0x83290508;
    'dispatch: loop {
        match pc {
            0x83290508 => {
    //   block [0x83290508..0x8329052C)
	// 83290508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329050C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290514: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 83290518: 4AF8ED41  bl 0x8221f258
	ctx.lr = 0x8329051C;
	sub_8221F258(ctx, base);
	// 8329051C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83290520: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83290524: 419A0008  beq cr6, 0x8329052c
	if ctx.cr[6].eq {
	pc = 0x8329052C; continue 'dispatch;
	}
	// 83290528: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8329052C; continue 'dispatch;
            }
            0x8329052C => {
    //   block [0x8329052C..0x83290538)
	// 8329052C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83290530: 41820008  beq 0x83290538
	if ctx.cr[0].eq {
	pc = 0x83290538; continue 'dispatch;
	}
	// 83290534: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83290538; continue 'dispatch;
            }
            0x83290538 => {
    //   block [0x83290538..0x83290568)
	// 83290538: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8329053C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83290540: 390956F0  addi r8, r9, 0x56f0
	ctx.r[8].s64 = ctx.r[9].s64 + 22256;
	// 83290544: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83290548: 38675C60  addi r3, r7, 0x5c60
	ctx.r[3].s64 = ctx.r[7].s64 + 23648;
	// 8329054C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83290550: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83290554: 4BA199CD  bl 0x82ca9f20
	ctx.lr = 0x83290558;
	sub_82CA9F20(ctx, base);
	// 83290558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329055C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290568 size=96
    let mut pc: u32 = 0x83290568;
    'dispatch: loop {
        match pc {
            0x83290568 => {
    //   block [0x83290568..0x8329058C)
	// 83290568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329056C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290574: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 83290578: 4AF8ECE1  bl 0x8221f258
	ctx.lr = 0x8329057C;
	sub_8221F258(ctx, base);
	// 8329057C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83290580: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83290584: 419A0008  beq cr6, 0x8329058c
	if ctx.cr[6].eq {
	pc = 0x8329058C; continue 'dispatch;
	}
	// 83290588: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8329058C; continue 'dispatch;
            }
            0x8329058C => {
    //   block [0x8329058C..0x83290598)
	// 8329058C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83290590: 41820008  beq 0x83290598
	if ctx.cr[0].eq {
	pc = 0x83290598; continue 'dispatch;
	}
	// 83290594: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83290598; continue 'dispatch;
            }
            0x83290598 => {
    //   block [0x83290598..0x832905C8)
	// 83290598: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8329059C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832905A0: 390956FC  addi r8, r9, 0x56fc
	ctx.r[8].s64 = ctx.r[9].s64 + 22268;
	// 832905A4: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832905A8: 38675CA8  addi r3, r7, 0x5ca8
	ctx.r[3].s64 = ctx.r[7].s64 + 23720;
	// 832905AC: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832905B0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832905B4: 4BA1996D  bl 0x82ca9f20
	ctx.lr = 0x832905B8;
	sub_82CA9F20(ctx, base);
	// 832905B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832905BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832905C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832905C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832905C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832905C8 size=12
    let mut pc: u32 = 0x832905C8;
    'dispatch: loop {
        match pc {
            0x832905C8 => {
    //   block [0x832905C8..0x832905D4)
	// 832905C8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832905CC: 386B5CF0  addi r3, r11, 0x5cf0
	ctx.r[3].s64 = ctx.r[11].s64 + 23792;
	// 832905D0: 4BA19950  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832905D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832905D8 size=12
    let mut pc: u32 = 0x832905D8;
    'dispatch: loop {
        match pc {
            0x832905D8 => {
    //   block [0x832905D8..0x832905E4)
	// 832905D8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832905DC: 386B5D48  addi r3, r11, 0x5d48
	ctx.r[3].s64 = ctx.r[11].s64 + 23880;
	// 832905E0: 4BA19940  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832905E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832905E8 size=12
    let mut pc: u32 = 0x832905E8;
    'dispatch: loop {
        match pc {
            0x832905E8 => {
    //   block [0x832905E8..0x832905F4)
	// 832905E8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832905EC: 386B5D58  addi r3, r11, 0x5d58
	ctx.r[3].s64 = ctx.r[11].s64 + 23896;
	// 832905F0: 4BA19930  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832905F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832905F8 size=64
    let mut pc: u32 = 0x832905F8;
    'dispatch: loop {
        match pc {
            0x832905F8 => {
    //   block [0x832905F8..0x83290638)
	// 832905F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832905FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290604: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290608: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329060C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290610: 386A5714  addi r3, r10, 0x5714
	ctx.r[3].s64 = ctx.r[10].s64 + 22292;
	// 83290614: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290618: 4AF9C8B9  bl 0x8222ced0
	ctx.lr = 0x8329061C;
	sub_8222CED0(ctx, base);
	// 8329061C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290620: 38695D68  addi r3, r9, 0x5d68
	ctx.r[3].s64 = ctx.r[9].s64 + 23912;
	// 83290624: 4BA198FD  bl 0x82ca9f20
	ctx.lr = 0x83290628;
	sub_82CA9F20(ctx, base);
	// 83290628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329062C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290638 size=64
    let mut pc: u32 = 0x83290638;
    'dispatch: loop {
        match pc {
            0x83290638 => {
    //   block [0x83290638..0x83290678)
	// 83290638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329063C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290644: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290648: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329064C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290650: 386A5718  addi r3, r10, 0x5718
	ctx.r[3].s64 = ctx.r[10].s64 + 22296;
	// 83290654: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290658: 4AF9C879  bl 0x8222ced0
	ctx.lr = 0x8329065C;
	sub_8222CED0(ctx, base);
	// 8329065C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290660: 38695D78  addi r3, r9, 0x5d78
	ctx.r[3].s64 = ctx.r[9].s64 + 23928;
	// 83290664: 4BA198BD  bl 0x82ca9f20
	ctx.lr = 0x83290668;
	sub_82CA9F20(ctx, base);
	// 83290668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329066C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290678 size=12
    let mut pc: u32 = 0x83290678;
    'dispatch: loop {
        match pc {
            0x83290678 => {
    //   block [0x83290678..0x83290684)
	// 83290678: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329067C: 386B5D88  addi r3, r11, 0x5d88
	ctx.r[3].s64 = ctx.r[11].s64 + 23944;
	// 83290680: 4BA198A0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290688 size=12
    let mut pc: u32 = 0x83290688;
    'dispatch: loop {
        match pc {
            0x83290688 => {
    //   block [0x83290688..0x83290694)
	// 83290688: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329068C: 386B5D98  addi r3, r11, 0x5d98
	ctx.r[3].s64 = ctx.r[11].s64 + 23960;
	// 83290690: 4BA19890  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290698 size=12
    let mut pc: u32 = 0x83290698;
    'dispatch: loop {
        match pc {
            0x83290698 => {
    //   block [0x83290698..0x832906A4)
	// 83290698: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329069C: 386B5DA8  addi r3, r11, 0x5da8
	ctx.r[3].s64 = ctx.r[11].s64 + 23976;
	// 832906A0: 4BA19880  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832906A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832906A8 size=52
    let mut pc: u32 = 0x832906A8;
    'dispatch: loop {
        match pc {
            0x832906A8 => {
    //   block [0x832906A8..0x832906DC)
	// 832906A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832906AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832906B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832906B4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832906B8: 386B573C  addi r3, r11, 0x573c
	ctx.r[3].s64 = ctx.r[11].s64 + 22332;
	// 832906BC: 480295C9  bl 0x832b9c84
	ctx.lr = 0x832906C0;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 832906C0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832906C4: 386A5DB8  addi r3, r10, 0x5db8
	ctx.r[3].s64 = ctx.r[10].s64 + 23992;
	// 832906C8: 4BA19859  bl 0x82ca9f20
	ctx.lr = 0x832906CC;
	sub_82CA9F20(ctx, base);
	// 832906CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832906D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832906D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832906D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832906E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832906E0 size=64
    let mut pc: u32 = 0x832906E0;
    'dispatch: loop {
        match pc {
            0x832906E0 => {
    //   block [0x832906E0..0x83290720)
	// 832906E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832906E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832906E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832906EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832906F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832906F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832906F8: 386A5758  addi r3, r10, 0x5758
	ctx.r[3].s64 = ctx.r[10].s64 + 22360;
	// 832906FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290700: 4AF9C7D1  bl 0x8222ced0
	ctx.lr = 0x83290704;
	sub_8222CED0(ctx, base);
	// 83290704: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290708: 38695DC0  addi r3, r9, 0x5dc0
	ctx.r[3].s64 = ctx.r[9].s64 + 24000;
	// 8329070C: 4BA19815  bl 0x82ca9f20
	ctx.lr = 0x83290710;
	sub_82CA9F20(ctx, base);
	// 83290710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329071C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290720 size=64
    let mut pc: u32 = 0x83290720;
    'dispatch: loop {
        match pc {
            0x83290720 => {
    //   block [0x83290720..0x83290760)
	// 83290720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329072C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290734: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290738: 386A575C  addi r3, r10, 0x575c
	ctx.r[3].s64 = ctx.r[10].s64 + 22364;
	// 8329073C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290740: 4AF9C791  bl 0x8222ced0
	ctx.lr = 0x83290744;
	sub_8222CED0(ctx, base);
	// 83290744: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290748: 38695DD0  addi r3, r9, 0x5dd0
	ctx.r[3].s64 = ctx.r[9].s64 + 24016;
	// 8329074C: 4BA197D5  bl 0x82ca9f20
	ctx.lr = 0x83290750;
	sub_82CA9F20(ctx, base);
	// 83290750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329075C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290760 size=12
    let mut pc: u32 = 0x83290760;
    'dispatch: loop {
        match pc {
            0x83290760 => {
    //   block [0x83290760..0x8329076C)
	// 83290760: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290764: 386B5DE0  addi r3, r11, 0x5de0
	ctx.r[3].s64 = ctx.r[11].s64 + 24032;
	// 83290768: 4BA197B8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290770 size=64
    let mut pc: u32 = 0x83290770;
    'dispatch: loop {
        match pc {
            0x83290770 => {
    //   block [0x83290770..0x832907B0)
	// 83290770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329077C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290784: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290788: 386A5764  addi r3, r10, 0x5764
	ctx.r[3].s64 = ctx.r[10].s64 + 22372;
	// 8329078C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290790: 4AF9C741  bl 0x8222ced0
	ctx.lr = 0x83290794;
	sub_8222CED0(ctx, base);
	// 83290794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290798: 38695E30  addi r3, r9, 0x5e30
	ctx.r[3].s64 = ctx.r[9].s64 + 24112;
	// 8329079C: 4BA19785  bl 0x82ca9f20
	ctx.lr = 0x832907A0;
	sub_82CA9F20(ctx, base);
	// 832907A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832907A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832907A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832907AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832907B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832907B0 size=64
    let mut pc: u32 = 0x832907B0;
    'dispatch: loop {
        match pc {
            0x832907B0 => {
    //   block [0x832907B0..0x832907F0)
	// 832907B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832907B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832907B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832907BC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832907C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832907C4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832907C8: 386A5768  addi r3, r10, 0x5768
	ctx.r[3].s64 = ctx.r[10].s64 + 22376;
	// 832907CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832907D0: 4AF9C701  bl 0x8222ced0
	ctx.lr = 0x832907D4;
	sub_8222CED0(ctx, base);
	// 832907D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832907D8: 38695E40  addi r3, r9, 0x5e40
	ctx.r[3].s64 = ctx.r[9].s64 + 24128;
	// 832907DC: 4BA19745  bl 0x82ca9f20
	ctx.lr = 0x832907E0;
	sub_82CA9F20(ctx, base);
	// 832907E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832907E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832907E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832907EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832907F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832907F0 size=64
    let mut pc: u32 = 0x832907F0;
    'dispatch: loop {
        match pc {
            0x832907F0 => {
    //   block [0x832907F0..0x83290830)
	// 832907F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832907F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832907F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832907FC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290804: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290808: 386A576C  addi r3, r10, 0x576c
	ctx.r[3].s64 = ctx.r[10].s64 + 22380;
	// 8329080C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290810: 4AF9C6C1  bl 0x8222ced0
	ctx.lr = 0x83290814;
	sub_8222CED0(ctx, base);
	// 83290814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290818: 38695E50  addi r3, r9, 0x5e50
	ctx.r[3].s64 = ctx.r[9].s64 + 24144;
	// 8329081C: 4BA19705  bl 0x82ca9f20
	ctx.lr = 0x83290820;
	sub_82CA9F20(ctx, base);
	// 83290820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329082C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290830 size=64
    let mut pc: u32 = 0x83290830;
    'dispatch: loop {
        match pc {
            0x83290830 => {
    //   block [0x83290830..0x83290870)
	// 83290830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329083C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290840: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290844: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290848: 386A5770  addi r3, r10, 0x5770
	ctx.r[3].s64 = ctx.r[10].s64 + 22384;
	// 8329084C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290850: 4AF9C681  bl 0x8222ced0
	ctx.lr = 0x83290854;
	sub_8222CED0(ctx, base);
	// 83290854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290858: 38695E60  addi r3, r9, 0x5e60
	ctx.r[3].s64 = ctx.r[9].s64 + 24160;
	// 8329085C: 4BA196C5  bl 0x82ca9f20
	ctx.lr = 0x83290860;
	sub_82CA9F20(ctx, base);
	// 83290860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329086C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290870 size=12
    let mut pc: u32 = 0x83290870;
    'dispatch: loop {
        match pc {
            0x83290870 => {
    //   block [0x83290870..0x8329087C)
	// 83290870: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290874: 386B5E70  addi r3, r11, 0x5e70
	ctx.r[3].s64 = ctx.r[11].s64 + 24176;
	// 83290878: 4BA196A8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290880 size=12
    let mut pc: u32 = 0x83290880;
    'dispatch: loop {
        match pc {
            0x83290880 => {
    //   block [0x83290880..0x8329088C)
	// 83290880: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290884: 386B5EC8  addi r3, r11, 0x5ec8
	ctx.r[3].s64 = ctx.r[11].s64 + 24264;
	// 83290888: 4BA19698  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290890 size=64
    let mut pc: u32 = 0x83290890;
    'dispatch: loop {
        match pc {
            0x83290890 => {
    //   block [0x83290890..0x832908D0)
	// 83290890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329089C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832908A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832908A4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832908A8: 386A5794  addi r3, r10, 0x5794
	ctx.r[3].s64 = ctx.r[10].s64 + 22420;
	// 832908AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832908B0: 4AF9C621  bl 0x8222ced0
	ctx.lr = 0x832908B4;
	sub_8222CED0(ctx, base);
	// 832908B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832908B8: 38695F88  addi r3, r9, 0x5f88
	ctx.r[3].s64 = ctx.r[9].s64 + 24456;
	// 832908BC: 4BA19665  bl 0x82ca9f20
	ctx.lr = 0x832908C0;
	sub_82CA9F20(ctx, base);
	// 832908C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832908C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832908C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832908CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832908D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832908D0 size=64
    let mut pc: u32 = 0x832908D0;
    'dispatch: loop {
        match pc {
            0x832908D0 => {
    //   block [0x832908D0..0x83290910)
	// 832908D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832908D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832908D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832908DC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832908E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832908E4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832908E8: 386A5798  addi r3, r10, 0x5798
	ctx.r[3].s64 = ctx.r[10].s64 + 22424;
	// 832908EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832908F0: 4AF9C5E1  bl 0x8222ced0
	ctx.lr = 0x832908F4;
	sub_8222CED0(ctx, base);
	// 832908F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832908F8: 38695F98  addi r3, r9, 0x5f98
	ctx.r[3].s64 = ctx.r[9].s64 + 24472;
	// 832908FC: 4BA19625  bl 0x82ca9f20
	ctx.lr = 0x83290900;
	sub_82CA9F20(ctx, base);
	// 83290900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329090C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290910 size=64
    let mut pc: u32 = 0x83290910;
    'dispatch: loop {
        match pc {
            0x83290910 => {
    //   block [0x83290910..0x83290950)
	// 83290910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329091C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290920: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290924: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290928: 386A579C  addi r3, r10, 0x579c
	ctx.r[3].s64 = ctx.r[10].s64 + 22428;
	// 8329092C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290930: 4AF9C5A1  bl 0x8222ced0
	ctx.lr = 0x83290934;
	sub_8222CED0(ctx, base);
	// 83290934: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290938: 38696000  addi r3, r9, 0x6000
	ctx.r[3].s64 = ctx.r[9].s64 + 24576;
	// 8329093C: 4BA195E5  bl 0x82ca9f20
	ctx.lr = 0x83290940;
	sub_82CA9F20(ctx, base);
	// 83290940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329094C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290950 size=64
    let mut pc: u32 = 0x83290950;
    'dispatch: loop {
        match pc {
            0x83290950 => {
    //   block [0x83290950..0x83290990)
	// 83290950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329095C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290964: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290968: 386A57A0  addi r3, r10, 0x57a0
	ctx.r[3].s64 = ctx.r[10].s64 + 22432;
	// 8329096C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290970: 4AF9C561  bl 0x8222ced0
	ctx.lr = 0x83290974;
	sub_8222CED0(ctx, base);
	// 83290974: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290978: 38696010  addi r3, r9, 0x6010
	ctx.r[3].s64 = ctx.r[9].s64 + 24592;
	// 8329097C: 4BA195A5  bl 0x82ca9f20
	ctx.lr = 0x83290980;
	sub_82CA9F20(ctx, base);
	// 83290980: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329098C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290990 size=64
    let mut pc: u32 = 0x83290990;
    'dispatch: loop {
        match pc {
            0x83290990 => {
    //   block [0x83290990..0x832909D0)
	// 83290990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329099C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832909A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832909A4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832909A8: 386A57A4  addi r3, r10, 0x57a4
	ctx.r[3].s64 = ctx.r[10].s64 + 22436;
	// 832909AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832909B0: 4AF9C521  bl 0x8222ced0
	ctx.lr = 0x832909B4;
	sub_8222CED0(ctx, base);
	// 832909B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832909B8: 38696020  addi r3, r9, 0x6020
	ctx.r[3].s64 = ctx.r[9].s64 + 24608;
	// 832909BC: 4BA19565  bl 0x82ca9f20
	ctx.lr = 0x832909C0;
	sub_82CA9F20(ctx, base);
	// 832909C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832909C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832909C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832909CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832909D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832909D0 size=64
    let mut pc: u32 = 0x832909D0;
    'dispatch: loop {
        match pc {
            0x832909D0 => {
    //   block [0x832909D0..0x83290A10)
	// 832909D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832909D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832909D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832909DC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832909E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832909E4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832909E8: 386A57A8  addi r3, r10, 0x57a8
	ctx.r[3].s64 = ctx.r[10].s64 + 22440;
	// 832909EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832909F0: 4AF9C4E1  bl 0x8222ced0
	ctx.lr = 0x832909F4;
	sub_8222CED0(ctx, base);
	// 832909F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832909F8: 38696030  addi r3, r9, 0x6030
	ctx.r[3].s64 = ctx.r[9].s64 + 24624;
	// 832909FC: 4BA19525  bl 0x82ca9f20
	ctx.lr = 0x83290A00;
	sub_82CA9F20(ctx, base);
	// 83290A00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290A10 size=12
    let mut pc: u32 = 0x83290A10;
    'dispatch: loop {
        match pc {
            0x83290A10 => {
    //   block [0x83290A10..0x83290A1C)
	// 83290A10: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290A14: 386B6040  addi r3, r11, 0x6040
	ctx.r[3].s64 = ctx.r[11].s64 + 24640;
	// 83290A18: 4BA19508  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290A20 size=52
    let mut pc: u32 = 0x83290A20;
    'dispatch: loop {
        match pc {
            0x83290A20 => {
    //   block [0x83290A20..0x83290A54)
	// 83290A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290A28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290A2C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83290A30: 386B57C0  addi r3, r11, 0x57c0
	ctx.r[3].s64 = ctx.r[11].s64 + 22464;
	// 83290A34: 4B82ADF5  bl 0x82abb828
	ctx.lr = 0x83290A38;
	sub_82ABB828(ctx, base);
	// 83290A38: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83290A3C: 386A6098  addi r3, r10, 0x6098
	ctx.r[3].s64 = ctx.r[10].s64 + 24728;
	// 83290A40: 4BA194E1  bl 0x82ca9f20
	ctx.lr = 0x83290A44;
	sub_82CA9F20(ctx, base);
	// 83290A44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290A58 size=12
    let mut pc: u32 = 0x83290A58;
    'dispatch: loop {
        match pc {
            0x83290A58 => {
    //   block [0x83290A58..0x83290A64)
	// 83290A58: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290A5C: 386B60A8  addi r3, r11, 0x60a8
	ctx.r[3].s64 = ctx.r[11].s64 + 24744;
	// 83290A60: 4BA194C0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290A68 size=12
    let mut pc: u32 = 0x83290A68;
    'dispatch: loop {
        match pc {
            0x83290A68 => {
    //   block [0x83290A68..0x83290A74)
	// 83290A68: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290A6C: 386B60B8  addi r3, r11, 0x60b8
	ctx.r[3].s64 = ctx.r[11].s64 + 24760;
	// 83290A70: 4BA194B0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290A78 size=12
    let mut pc: u32 = 0x83290A78;
    'dispatch: loop {
        match pc {
            0x83290A78 => {
    //   block [0x83290A78..0x83290A84)
	// 83290A78: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290A7C: 386B60C8  addi r3, r11, 0x60c8
	ctx.r[3].s64 = ctx.r[11].s64 + 24776;
	// 83290A80: 4BA194A0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290A88 size=12
    let mut pc: u32 = 0x83290A88;
    'dispatch: loop {
        match pc {
            0x83290A88 => {
    //   block [0x83290A88..0x83290A94)
	// 83290A88: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290A8C: 386B60D8  addi r3, r11, 0x60d8
	ctx.r[3].s64 = ctx.r[11].s64 + 24792;
	// 83290A90: 4BA19490  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290A98 size=12
    let mut pc: u32 = 0x83290A98;
    'dispatch: loop {
        match pc {
            0x83290A98 => {
    //   block [0x83290A98..0x83290AA4)
	// 83290A98: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290A9C: 386B60E8  addi r3, r11, 0x60e8
	ctx.r[3].s64 = ctx.r[11].s64 + 24808;
	// 83290AA0: 4BA19480  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290AA8 size=12
    let mut pc: u32 = 0x83290AA8;
    'dispatch: loop {
        match pc {
            0x83290AA8 => {
    //   block [0x83290AA8..0x83290AB4)
	// 83290AA8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290AAC: 386B60F8  addi r3, r11, 0x60f8
	ctx.r[3].s64 = ctx.r[11].s64 + 24824;
	// 83290AB0: 4BA19470  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290AB8 size=12
    let mut pc: u32 = 0x83290AB8;
    'dispatch: loop {
        match pc {
            0x83290AB8 => {
    //   block [0x83290AB8..0x83290AC4)
	// 83290AB8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290ABC: 386B6108  addi r3, r11, 0x6108
	ctx.r[3].s64 = ctx.r[11].s64 + 24840;
	// 83290AC0: 4BA19460  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290AC8 size=12
    let mut pc: u32 = 0x83290AC8;
    'dispatch: loop {
        match pc {
            0x83290AC8 => {
    //   block [0x83290AC8..0x83290AD4)
	// 83290AC8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290ACC: 386B6118  addi r3, r11, 0x6118
	ctx.r[3].s64 = ctx.r[11].s64 + 24856;
	// 83290AD0: 4BA19450  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290AD8 size=64
    let mut pc: u32 = 0x83290AD8;
    'dispatch: loop {
        match pc {
            0x83290AD8 => {
    //   block [0x83290AD8..0x83290B18)
	// 83290AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290AE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290AE4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290AE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290AEC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290AF0: 386A58D0  addi r3, r10, 0x58d0
	ctx.r[3].s64 = ctx.r[10].s64 + 22736;
	// 83290AF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290AF8: 4AF9C3D9  bl 0x8222ced0
	ctx.lr = 0x83290AFC;
	sub_8222CED0(ctx, base);
	// 83290AFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290B00: 38696128  addi r3, r9, 0x6128
	ctx.r[3].s64 = ctx.r[9].s64 + 24872;
	// 83290B04: 4BA1941D  bl 0x82ca9f20
	ctx.lr = 0x83290B08;
	sub_82CA9F20(ctx, base);
	// 83290B08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290B18 size=64
    let mut pc: u32 = 0x83290B18;
    'dispatch: loop {
        match pc {
            0x83290B18 => {
    //   block [0x83290B18..0x83290B58)
	// 83290B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290B20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290B24: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290B28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290B2C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290B30: 386A58D4  addi r3, r10, 0x58d4
	ctx.r[3].s64 = ctx.r[10].s64 + 22740;
	// 83290B34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290B38: 4AF9C399  bl 0x8222ced0
	ctx.lr = 0x83290B3C;
	sub_8222CED0(ctx, base);
	// 83290B3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290B40: 38696138  addi r3, r9, 0x6138
	ctx.r[3].s64 = ctx.r[9].s64 + 24888;
	// 83290B44: 4BA193DD  bl 0x82ca9f20
	ctx.lr = 0x83290B48;
	sub_82CA9F20(ctx, base);
	// 83290B48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290B58 size=64
    let mut pc: u32 = 0x83290B58;
    'dispatch: loop {
        match pc {
            0x83290B58 => {
    //   block [0x83290B58..0x83290B98)
	// 83290B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290B60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290B64: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290B68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290B6C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290B70: 386A58D8  addi r3, r10, 0x58d8
	ctx.r[3].s64 = ctx.r[10].s64 + 22744;
	// 83290B74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290B78: 4AF9C359  bl 0x8222ced0
	ctx.lr = 0x83290B7C;
	sub_8222CED0(ctx, base);
	// 83290B7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290B80: 38696148  addi r3, r9, 0x6148
	ctx.r[3].s64 = ctx.r[9].s64 + 24904;
	// 83290B84: 4BA1939D  bl 0x82ca9f20
	ctx.lr = 0x83290B88;
	sub_82CA9F20(ctx, base);
	// 83290B88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290B98 size=64
    let mut pc: u32 = 0x83290B98;
    'dispatch: loop {
        match pc {
            0x83290B98 => {
    //   block [0x83290B98..0x83290BD8)
	// 83290B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290BA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290BA4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290BA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290BAC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290BB0: 386A58DC  addi r3, r10, 0x58dc
	ctx.r[3].s64 = ctx.r[10].s64 + 22748;
	// 83290BB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290BB8: 4AF9C319  bl 0x8222ced0
	ctx.lr = 0x83290BBC;
	sub_8222CED0(ctx, base);
	// 83290BBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290BC0: 38696158  addi r3, r9, 0x6158
	ctx.r[3].s64 = ctx.r[9].s64 + 24920;
	// 83290BC4: 4BA1935D  bl 0x82ca9f20
	ctx.lr = 0x83290BC8;
	sub_82CA9F20(ctx, base);
	// 83290BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290BD8 size=64
    let mut pc: u32 = 0x83290BD8;
    'dispatch: loop {
        match pc {
            0x83290BD8 => {
    //   block [0x83290BD8..0x83290C18)
	// 83290BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290BE4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290BE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290BEC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290BF0: 386A58E0  addi r3, r10, 0x58e0
	ctx.r[3].s64 = ctx.r[10].s64 + 22752;
	// 83290BF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290BF8: 4AF9C2D9  bl 0x8222ced0
	ctx.lr = 0x83290BFC;
	sub_8222CED0(ctx, base);
	// 83290BFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290C00: 38696168  addi r3, r9, 0x6168
	ctx.r[3].s64 = ctx.r[9].s64 + 24936;
	// 83290C04: 4BA1931D  bl 0x82ca9f20
	ctx.lr = 0x83290C08;
	sub_82CA9F20(ctx, base);
	// 83290C08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290C18 size=64
    let mut pc: u32 = 0x83290C18;
    'dispatch: loop {
        match pc {
            0x83290C18 => {
    //   block [0x83290C18..0x83290C58)
	// 83290C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290C20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290C24: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290C28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290C2C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290C30: 386A58E4  addi r3, r10, 0x58e4
	ctx.r[3].s64 = ctx.r[10].s64 + 22756;
	// 83290C34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290C38: 4AF9C299  bl 0x8222ced0
	ctx.lr = 0x83290C3C;
	sub_8222CED0(ctx, base);
	// 83290C3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290C40: 38696178  addi r3, r9, 0x6178
	ctx.r[3].s64 = ctx.r[9].s64 + 24952;
	// 83290C44: 4BA192DD  bl 0x82ca9f20
	ctx.lr = 0x83290C48;
	sub_82CA9F20(ctx, base);
	// 83290C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290C58 size=12
    let mut pc: u32 = 0x83290C58;
    'dispatch: loop {
        match pc {
            0x83290C58 => {
    //   block [0x83290C58..0x83290C64)
	// 83290C58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83290C5C: 386B58F0  addi r3, r11, 0x58f0
	ctx.r[3].s64 = ctx.r[11].s64 + 22768;
	// 83290C60: 4AFAB150  b 0x8223bdb0
	sub_8223BDB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290C68 size=28
    let mut pc: u32 = 0x83290C68;
    'dispatch: loop {
        match pc {
            0x83290C68 => {
    //   block [0x83290C68..0x83290C84)
	// 83290C68: 1001038C  vspltisw v0, 1
	for i in 0..4 {
		ctx.v[0].u32[i] = 1;
	}
	// 83290C6C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83290C70: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 83290C74: 392BAD50  addi r9, r11, -0x52b0
	ctx.r[9].s64 = ctx.r[11].s64 + -21168;
	// 83290C78: 1000030A  vcfux v0, v0, 0
	// vcfux/vcuxwfp128: ctx.v[0].f32[i] = ( ctx.v[0].u32[i] as f32 ) * (2.0f32).powi(0);
	for i in 0..4 { ctx.v[0].f32[i] = (ctx.v[0].u32[i] as f32) * (2.0f32).powi(0); }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290C88 size=28
    let mut pc: u32 = 0x83290C88;
    'dispatch: loop {
        match pc {
            0x83290C88 => {
    //   block [0x83290C88..0x83290CA4)
	// 83290C88: 1001038C  vspltisw v0, 1
	for i in 0..4 {
		ctx.v[0].u32[i] = 1;
	}
	// 83290C8C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83290C90: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 83290C94: 392BAD70  addi r9, r11, -0x5290
	ctx.r[9].s64 = ctx.r[11].s64 + -21136;
	// 83290C98: 1000030A  vcfux v0, v0, 0
	// vcfux/vcuxwfp128: ctx.v[0].f32[i] = ( ctx.v[0].u32[i] as f32 ) * (2.0f32).powi(0);
	for i in 0..4 { ctx.v[0].f32[i] = (ctx.v[0].u32[i] as f32) * (2.0f32).powi(0); }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290CA8 size=24
    let mut pc: u32 = 0x83290CA8;
    'dispatch: loop {
        match pc {
            0x83290CA8 => {
    //   block [0x83290CA8..0x83290CC0)
	// 83290CA8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83290CAC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83290CB0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 83290CB4: 392BAD90  addi r9, r11, -0x5270
	ctx.r[9].s64 = ctx.r[11].s64 + -21104;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290CC0 size=64
    let mut pc: u32 = 0x83290CC0;
    'dispatch: loop {
        match pc {
            0x83290CC0 => {
    //   block [0x83290CC0..0x83290D00)
	// 83290CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290CC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290CCC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290CD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290CD4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290CD8: 386A5930  addi r3, r10, 0x5930
	ctx.r[3].s64 = ctx.r[10].s64 + 22832;
	// 83290CDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290CE0: 4AF9C1F1  bl 0x8222ced0
	ctx.lr = 0x83290CE4;
	sub_8222CED0(ctx, base);
	// 83290CE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290CE8: 38696188  addi r3, r9, 0x6188
	ctx.r[3].s64 = ctx.r[9].s64 + 24968;
	// 83290CEC: 4BA19235  bl 0x82ca9f20
	ctx.lr = 0x83290CF0;
	sub_82CA9F20(ctx, base);
	// 83290CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290D00 size=64
    let mut pc: u32 = 0x83290D00;
    'dispatch: loop {
        match pc {
            0x83290D00 => {
    //   block [0x83290D00..0x83290D40)
	// 83290D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290D0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290D10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290D14: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290D18: 386A5934  addi r3, r10, 0x5934
	ctx.r[3].s64 = ctx.r[10].s64 + 22836;
	// 83290D1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290D20: 4AF9C1B1  bl 0x8222ced0
	ctx.lr = 0x83290D24;
	sub_8222CED0(ctx, base);
	// 83290D24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290D28: 38696198  addi r3, r9, 0x6198
	ctx.r[3].s64 = ctx.r[9].s64 + 24984;
	// 83290D2C: 4BA191F5  bl 0x82ca9f20
	ctx.lr = 0x83290D30;
	sub_82CA9F20(ctx, base);
	// 83290D30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290D40 size=64
    let mut pc: u32 = 0x83290D40;
    'dispatch: loop {
        match pc {
            0x83290D40 => {
    //   block [0x83290D40..0x83290D80)
	// 83290D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290D4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290D50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290D54: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290D58: 386A5938  addi r3, r10, 0x5938
	ctx.r[3].s64 = ctx.r[10].s64 + 22840;
	// 83290D5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290D60: 4AF9C171  bl 0x8222ced0
	ctx.lr = 0x83290D64;
	sub_8222CED0(ctx, base);
	// 83290D64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290D68: 386961A8  addi r3, r9, 0x61a8
	ctx.r[3].s64 = ctx.r[9].s64 + 25000;
	// 83290D6C: 4BA191B5  bl 0x82ca9f20
	ctx.lr = 0x83290D70;
	sub_82CA9F20(ctx, base);
	// 83290D70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290D80 size=64
    let mut pc: u32 = 0x83290D80;
    'dispatch: loop {
        match pc {
            0x83290D80 => {
    //   block [0x83290D80..0x83290DC0)
	// 83290D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290D8C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290D90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290D94: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290D98: 386A593C  addi r3, r10, 0x593c
	ctx.r[3].s64 = ctx.r[10].s64 + 22844;
	// 83290D9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290DA0: 4AF9C131  bl 0x8222ced0
	ctx.lr = 0x83290DA4;
	sub_8222CED0(ctx, base);
	// 83290DA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290DA8: 386961B8  addi r3, r9, 0x61b8
	ctx.r[3].s64 = ctx.r[9].s64 + 25016;
	// 83290DAC: 4BA19175  bl 0x82ca9f20
	ctx.lr = 0x83290DB0;
	sub_82CA9F20(ctx, base);
	// 83290DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83290DC0 size=992
    let mut pc: u32 = 0x83290DC0;
    'dispatch: loop {
        match pc {
            0x83290DC0 => {
    //   block [0x83290DC0..0x832911A0)
	// 83290DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290DC4: 4BA18621  bl 0x82ca93e4
	ctx.lr = 0x83290DC8;
	sub_82CA93D0(ctx, base);
	// 83290DC8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83290DCC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83290DD0: 392BD5DC  addi r9, r11, -0x2a24
	ctx.r[9].s64 = ctx.r[11].s64 + -10788;
	// 83290DD4: 3901FF34  addi r8, r1, -0xcc
	ctx.r[8].s64 = ctx.r[1].s64 + -204;
	// 83290DD8: 38E1FF38  addi r7, r1, -0xc8
	ctx.r[7].s64 = ctx.r[1].s64 + -200;
	// 83290DDC: 38C1FF3C  addi r6, r1, -0xc4
	ctx.r[6].s64 = ctx.r[1].s64 + -196;
	// 83290DE0: C1AA0A9C  lfs f13, 0xa9c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2716 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83290DE4: 38A1FF30  addi r5, r1, -0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + -208;
	// 83290DE8: C009BEA8  lfs f0, -0x4158(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16728 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83290DEC: 3881FF40  addi r4, r1, -0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + -192;
	// 83290DF0: D001FF34  stfs f0, -0xcc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-204 as u32), tmp.u32 ) };
	// 83290DF4: 3861FF44  addi r3, r1, -0xbc
	ctx.r[3].s64 = ctx.r[1].s64 + -188;
	// 83290DF8: D001FF38  stfs f0, -0xc8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-200 as u32), tmp.u32 ) };
	// 83290DFC: 3BC1FF50  addi r30, r1, -0xb0
	ctx.r[30].s64 = ctx.r[1].s64 + -176;
	// 83290E00: C189BEB4  lfs f12, -0x414c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16716 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83290E04: 3BA1FF54  addi r29, r1, -0xac
	ctx.r[29].s64 = ctx.r[1].s64 + -172;
	// 83290E08: D181FF30  stfs f12, -0xd0(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-208 as u32), tmp.u32 ) };
	// 83290E0C: 3B81FF58  addi r28, r1, -0xa8
	ctx.r[28].s64 = ctx.r[1].s64 + -168;
	// 83290E10: C189BCF8  lfs f12, -0x4308(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-17160 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83290E14: 3B61FF5C  addi r27, r1, -0xa4
	ctx.r[27].s64 = ctx.r[1].s64 + -164;
	// 83290E18: D001FF3C  stfs f0, -0xc4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-196 as u32), tmp.u32 ) };
	// 83290E1C: 3941FF48  addi r10, r1, -0xb8
	ctx.r[10].s64 = ctx.r[1].s64 + -184;
	// 83290E20: D1A1FF40  stfs f13, -0xc0(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-192 as u32), tmp.u32 ) };
	// 83290E24: 3BE1FF4C  addi r31, r1, -0xb4
	ctx.r[31].s64 = ctx.r[1].s64 + -180;
	// 83290E28: D181FF44  stfs f12, -0xbc(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), tmp.u32 ) };
	// 83290E2C: 3B41FF60  addi r26, r1, -0xa0
	ctx.r[26].s64 = ctx.r[1].s64 + -160;
	// 83290E30: D001FF50  stfs f0, -0xb0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), tmp.u32 ) };
	// 83290E34: 3B21FF64  addi r25, r1, -0x9c
	ctx.r[25].s64 = ctx.r[1].s64 + -156;
	// 83290E38: D001FF54  stfs f0, -0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-172 as u32), tmp.u32 ) };
	// 83290E3C: 3B01FF68  addi r24, r1, -0x98
	ctx.r[24].s64 = ctx.r[1].s64 + -152;
	// 83290E40: D001FF58  stfs f0, -0xa8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), tmp.u32 ) };
	// 83290E44: 3AE1FF6C  addi r23, r1, -0x94
	ctx.r[23].s64 = ctx.r[1].s64 + -148;
	// 83290E48: D001FF5C  stfs f0, -0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-164 as u32), tmp.u32 ) };
	// 83290E4C: 3AA1FF74  addi r21, r1, -0x8c
	ctx.r[21].s64 = ctx.r[1].s64 + -140;
	// 83290E50: C16911D0  lfs f11, 0x11d0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4560 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 83290E54: 3AC1FF70  addi r22, r1, -0x90
	ctx.r[22].s64 = ctx.r[1].s64 + -144;
	// 83290E58: D001FF6C  stfs f0, -0x94(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-148 as u32), tmp.u32 ) };
	// 83290E5C: 3A81FF78  addi r20, r1, -0x88
	ctx.r[20].s64 = ctx.r[1].s64 + -136;
	// 83290E60: D1A1FF48  stfs f13, -0xb8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), tmp.u32 ) };
	// 83290E64: 3A61FF74  addi r19, r1, -0x8c
	ctx.r[19].s64 = ctx.r[1].s64 + -140;
	// 83290E68: D161FF4C  stfs f11, -0xb4(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-180 as u32), tmp.u32 ) };
	// 83290E6C: D001FF60  stfs f0, -0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), tmp.u32 ) };
	// 83290E70: D001FF64  stfs f0, -0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-156 as u32), tmp.u32 ) };
	// 83290E74: D001FF68  stfs f0, -0x98(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), tmp.u32 ) };
	// 83290E78: D001FF74  stfs f0, -0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-140 as u32), tmp.u32 ) };
	// 83290E7C: D001FF70  stfs f0, -0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832911A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832911A0 size=64
    let mut pc: u32 = 0x832911A0;
    'dispatch: loop {
        match pc {
            0x832911A0 => {
    //   block [0x832911A0..0x832911E0)
	// 832911A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832911A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832911A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832911AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832911B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832911B4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832911B8: 386A5A10  addi r3, r10, 0x5a10
	ctx.r[3].s64 = ctx.r[10].s64 + 23056;
	// 832911BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832911C0: 4AF9BD11  bl 0x8222ced0
	ctx.lr = 0x832911C4;
	sub_8222CED0(ctx, base);
	// 832911C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832911C8: 386961E0  addi r3, r9, 0x61e0
	ctx.r[3].s64 = ctx.r[9].s64 + 25056;
	// 832911CC: 4BA18D55  bl 0x82ca9f20
	ctx.lr = 0x832911D0;
	sub_82CA9F20(ctx, base);
	// 832911D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832911D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832911D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832911DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832911E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832911E0 size=64
    let mut pc: u32 = 0x832911E0;
    'dispatch: loop {
        match pc {
            0x832911E0 => {
    //   block [0x832911E0..0x83291220)
	// 832911E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832911E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832911E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832911EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832911F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832911F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832911F8: 386A5A14  addi r3, r10, 0x5a14
	ctx.r[3].s64 = ctx.r[10].s64 + 23060;
	// 832911FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291200: 4AF9BCD1  bl 0x8222ced0
	ctx.lr = 0x83291204;
	sub_8222CED0(ctx, base);
	// 83291204: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291208: 386961F0  addi r3, r9, 0x61f0
	ctx.r[3].s64 = ctx.r[9].s64 + 25072;
	// 8329120C: 4BA18D15  bl 0x82ca9f20
	ctx.lr = 0x83291210;
	sub_82CA9F20(ctx, base);
	// 83291210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329121C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291220 size=64
    let mut pc: u32 = 0x83291220;
    'dispatch: loop {
        match pc {
            0x83291220 => {
    //   block [0x83291220..0x83291260)
	// 83291220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329122C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291230: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291234: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291238: 386A5A18  addi r3, r10, 0x5a18
	ctx.r[3].s64 = ctx.r[10].s64 + 23064;
	// 8329123C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291240: 4AF9BC91  bl 0x8222ced0
	ctx.lr = 0x83291244;
	sub_8222CED0(ctx, base);
	// 83291244: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291248: 38696200  addi r3, r9, 0x6200
	ctx.r[3].s64 = ctx.r[9].s64 + 25088;
	// 8329124C: 4BA18CD5  bl 0x82ca9f20
	ctx.lr = 0x83291250;
	sub_82CA9F20(ctx, base);
	// 83291250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329125C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291260 size=64
    let mut pc: u32 = 0x83291260;
    'dispatch: loop {
        match pc {
            0x83291260 => {
    //   block [0x83291260..0x832912A0)
	// 83291260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329126C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291270: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291274: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291278: 386A5A1C  addi r3, r10, 0x5a1c
	ctx.r[3].s64 = ctx.r[10].s64 + 23068;
	// 8329127C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291280: 4AF9BC51  bl 0x8222ced0
	ctx.lr = 0x83291284;
	sub_8222CED0(ctx, base);
	// 83291284: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291288: 38696210  addi r3, r9, 0x6210
	ctx.r[3].s64 = ctx.r[9].s64 + 25104;
	// 8329128C: 4BA18C95  bl 0x82ca9f20
	ctx.lr = 0x83291290;
	sub_82CA9F20(ctx, base);
	// 83291290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329129C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832912A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832912A0 size=64
    let mut pc: u32 = 0x832912A0;
    'dispatch: loop {
        match pc {
            0x832912A0 => {
    //   block [0x832912A0..0x832912E0)
	// 832912A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832912A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832912A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832912AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832912B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832912B4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832912B8: 386A5A20  addi r3, r10, 0x5a20
	ctx.r[3].s64 = ctx.r[10].s64 + 23072;
	// 832912BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832912C0: 4AF9BC11  bl 0x8222ced0
	ctx.lr = 0x832912C4;
	sub_8222CED0(ctx, base);
	// 832912C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832912C8: 38696220  addi r3, r9, 0x6220
	ctx.r[3].s64 = ctx.r[9].s64 + 25120;
	// 832912CC: 4BA18C55  bl 0x82ca9f20
	ctx.lr = 0x832912D0;
	sub_82CA9F20(ctx, base);
	// 832912D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832912D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832912D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832912DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832912E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832912E0 size=64
    let mut pc: u32 = 0x832912E0;
    'dispatch: loop {
        match pc {
            0x832912E0 => {
    //   block [0x832912E0..0x83291320)
	// 832912E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832912E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832912E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832912EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832912F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832912F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832912F8: 386A5A24  addi r3, r10, 0x5a24
	ctx.r[3].s64 = ctx.r[10].s64 + 23076;
	// 832912FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291300: 4AF9BBD1  bl 0x8222ced0
	ctx.lr = 0x83291304;
	sub_8222CED0(ctx, base);
	// 83291304: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291308: 38696230  addi r3, r9, 0x6230
	ctx.r[3].s64 = ctx.r[9].s64 + 25136;
	// 8329130C: 4BA18C15  bl 0x82ca9f20
	ctx.lr = 0x83291310;
	sub_82CA9F20(ctx, base);
	// 83291310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329131C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291320 size=64
    let mut pc: u32 = 0x83291320;
    'dispatch: loop {
        match pc {
            0x83291320 => {
    //   block [0x83291320..0x83291360)
	// 83291320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329132C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291330: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291334: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291338: 386A5A28  addi r3, r10, 0x5a28
	ctx.r[3].s64 = ctx.r[10].s64 + 23080;
	// 8329133C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291340: 4AF9BB91  bl 0x8222ced0
	ctx.lr = 0x83291344;
	sub_8222CED0(ctx, base);
	// 83291344: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291348: 38696240  addi r3, r9, 0x6240
	ctx.r[3].s64 = ctx.r[9].s64 + 25152;
	// 8329134C: 4BA18BD5  bl 0x82ca9f20
	ctx.lr = 0x83291350;
	sub_82CA9F20(ctx, base);
	// 83291350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329135C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291360 size=64
    let mut pc: u32 = 0x83291360;
    'dispatch: loop {
        match pc {
            0x83291360 => {
    //   block [0x83291360..0x832913A0)
	// 83291360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329136C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291374: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291378: 386A5A2C  addi r3, r10, 0x5a2c
	ctx.r[3].s64 = ctx.r[10].s64 + 23084;
	// 8329137C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291380: 4AF9BB51  bl 0x8222ced0
	ctx.lr = 0x83291384;
	sub_8222CED0(ctx, base);
	// 83291384: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291388: 38696250  addi r3, r9, 0x6250
	ctx.r[3].s64 = ctx.r[9].s64 + 25168;
	// 8329138C: 4BA18B95  bl 0x82ca9f20
	ctx.lr = 0x83291390;
	sub_82CA9F20(ctx, base);
	// 83291390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329139C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832913A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832913A0 size=64
    let mut pc: u32 = 0x832913A0;
    'dispatch: loop {
        match pc {
            0x832913A0 => {
    //   block [0x832913A0..0x832913E0)
	// 832913A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832913A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832913A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832913AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832913B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832913B4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832913B8: 386A5A30  addi r3, r10, 0x5a30
	ctx.r[3].s64 = ctx.r[10].s64 + 23088;
	// 832913BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832913C0: 4AF9BB11  bl 0x8222ced0
	ctx.lr = 0x832913C4;
	sub_8222CED0(ctx, base);
	// 832913C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832913C8: 38696260  addi r3, r9, 0x6260
	ctx.r[3].s64 = ctx.r[9].s64 + 25184;
	// 832913CC: 4BA18B55  bl 0x82ca9f20
	ctx.lr = 0x832913D0;
	sub_82CA9F20(ctx, base);
	// 832913D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832913D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832913D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832913DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832913E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832913E0 size=64
    let mut pc: u32 = 0x832913E0;
    'dispatch: loop {
        match pc {
            0x832913E0 => {
    //   block [0x832913E0..0x83291420)
	// 832913E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832913E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832913E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832913EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832913F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832913F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832913F8: 386A5A34  addi r3, r10, 0x5a34
	ctx.r[3].s64 = ctx.r[10].s64 + 23092;
	// 832913FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291400: 4AF9BAD1  bl 0x8222ced0
	ctx.lr = 0x83291404;
	sub_8222CED0(ctx, base);
	// 83291404: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291408: 38696270  addi r3, r9, 0x6270
	ctx.r[3].s64 = ctx.r[9].s64 + 25200;
	// 8329140C: 4BA18B15  bl 0x82ca9f20
	ctx.lr = 0x83291410;
	sub_82CA9F20(ctx, base);
	// 83291410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329141C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291420 size=64
    let mut pc: u32 = 0x83291420;
    'dispatch: loop {
        match pc {
            0x83291420 => {
    //   block [0x83291420..0x83291460)
	// 83291420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329142C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291430: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291434: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291438: 386A5A38  addi r3, r10, 0x5a38
	ctx.r[3].s64 = ctx.r[10].s64 + 23096;
	// 8329143C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291440: 4AF9BA91  bl 0x8222ced0
	ctx.lr = 0x83291444;
	sub_8222CED0(ctx, base);
	// 83291444: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291448: 38696280  addi r3, r9, 0x6280
	ctx.r[3].s64 = ctx.r[9].s64 + 25216;
	// 8329144C: 4BA18AD5  bl 0x82ca9f20
	ctx.lr = 0x83291450;
	sub_82CA9F20(ctx, base);
	// 83291450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329145C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291460 size=64
    let mut pc: u32 = 0x83291460;
    'dispatch: loop {
        match pc {
            0x83291460 => {
    //   block [0x83291460..0x832914A0)
	// 83291460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329146C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291470: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291474: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291478: 386A5A3C  addi r3, r10, 0x5a3c
	ctx.r[3].s64 = ctx.r[10].s64 + 23100;
	// 8329147C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291480: 4AF9BA51  bl 0x8222ced0
	ctx.lr = 0x83291484;
	sub_8222CED0(ctx, base);
	// 83291484: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291488: 38696290  addi r3, r9, 0x6290
	ctx.r[3].s64 = ctx.r[9].s64 + 25232;
	// 8329148C: 4BA18A95  bl 0x82ca9f20
	ctx.lr = 0x83291490;
	sub_82CA9F20(ctx, base);
	// 83291490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329149C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832914A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832914A0 size=64
    let mut pc: u32 = 0x832914A0;
    'dispatch: loop {
        match pc {
            0x832914A0 => {
    //   block [0x832914A0..0x832914E0)
	// 832914A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832914A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832914A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832914AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832914B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832914B4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832914B8: 386A5A40  addi r3, r10, 0x5a40
	ctx.r[3].s64 = ctx.r[10].s64 + 23104;
	// 832914BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832914C0: 4AF9BA11  bl 0x8222ced0
	ctx.lr = 0x832914C4;
	sub_8222CED0(ctx, base);
	// 832914C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832914C8: 386962A0  addi r3, r9, 0x62a0
	ctx.r[3].s64 = ctx.r[9].s64 + 25248;
	// 832914CC: 4BA18A55  bl 0x82ca9f20
	ctx.lr = 0x832914D0;
	sub_82CA9F20(ctx, base);
	// 832914D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832914D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832914D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832914DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832914E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832914E0 size=64
    let mut pc: u32 = 0x832914E0;
    'dispatch: loop {
        match pc {
            0x832914E0 => {
    //   block [0x832914E0..0x83291520)
	// 832914E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832914E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832914E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832914EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832914F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832914F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832914F8: 386A5A44  addi r3, r10, 0x5a44
	ctx.r[3].s64 = ctx.r[10].s64 + 23108;
	// 832914FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291500: 4AF9B9D1  bl 0x8222ced0
	ctx.lr = 0x83291504;
	sub_8222CED0(ctx, base);
	// 83291504: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291508: 386962B0  addi r3, r9, 0x62b0
	ctx.r[3].s64 = ctx.r[9].s64 + 25264;
	// 8329150C: 4BA18A15  bl 0x82ca9f20
	ctx.lr = 0x83291510;
	sub_82CA9F20(ctx, base);
	// 83291510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329151C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291520 size=64
    let mut pc: u32 = 0x83291520;
    'dispatch: loop {
        match pc {
            0x83291520 => {
    //   block [0x83291520..0x83291560)
	// 83291520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329152C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291530: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291534: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291538: 386A5A48  addi r3, r10, 0x5a48
	ctx.r[3].s64 = ctx.r[10].s64 + 23112;
	// 8329153C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291540: 4AF9B991  bl 0x8222ced0
	ctx.lr = 0x83291544;
	sub_8222CED0(ctx, base);
	// 83291544: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291548: 386962C0  addi r3, r9, 0x62c0
	ctx.r[3].s64 = ctx.r[9].s64 + 25280;
	// 8329154C: 4BA189D5  bl 0x82ca9f20
	ctx.lr = 0x83291550;
	sub_82CA9F20(ctx, base);
	// 83291550: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329155C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291560 size=64
    let mut pc: u32 = 0x83291560;
    'dispatch: loop {
        match pc {
            0x83291560 => {
    //   block [0x83291560..0x832915A0)
	// 83291560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329156C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291570: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291574: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291578: 386A5A4C  addi r3, r10, 0x5a4c
	ctx.r[3].s64 = ctx.r[10].s64 + 23116;
	// 8329157C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291580: 4AF9B951  bl 0x8222ced0
	ctx.lr = 0x83291584;
	sub_8222CED0(ctx, base);
	// 83291584: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291588: 386962D0  addi r3, r9, 0x62d0
	ctx.r[3].s64 = ctx.r[9].s64 + 25296;
	// 8329158C: 4BA18995  bl 0x82ca9f20
	ctx.lr = 0x83291590;
	sub_82CA9F20(ctx, base);
	// 83291590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329159C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832915A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832915A0 size=64
    let mut pc: u32 = 0x832915A0;
    'dispatch: loop {
        match pc {
            0x832915A0 => {
    //   block [0x832915A0..0x832915E0)
	// 832915A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832915A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832915A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832915AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832915B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832915B4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832915B8: 386A5A50  addi r3, r10, 0x5a50
	ctx.r[3].s64 = ctx.r[10].s64 + 23120;
	// 832915BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832915C0: 4AF9B911  bl 0x8222ced0
	ctx.lr = 0x832915C4;
	sub_8222CED0(ctx, base);
	// 832915C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832915C8: 386962E0  addi r3, r9, 0x62e0
	ctx.r[3].s64 = ctx.r[9].s64 + 25312;
	// 832915CC: 4BA18955  bl 0x82ca9f20
	ctx.lr = 0x832915D0;
	sub_82CA9F20(ctx, base);
	// 832915D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832915D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832915D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832915DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832915E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832915E0 size=64
    let mut pc: u32 = 0x832915E0;
    'dispatch: loop {
        match pc {
            0x832915E0 => {
    //   block [0x832915E0..0x83291620)
	// 832915E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832915E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832915E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832915EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832915F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832915F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832915F8: 386A5A54  addi r3, r10, 0x5a54
	ctx.r[3].s64 = ctx.r[10].s64 + 23124;
	// 832915FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291600: 4AF9B8D1  bl 0x8222ced0
	ctx.lr = 0x83291604;
	sub_8222CED0(ctx, base);
	// 83291604: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291608: 386962F0  addi r3, r9, 0x62f0
	ctx.r[3].s64 = ctx.r[9].s64 + 25328;
	// 8329160C: 4BA18915  bl 0x82ca9f20
	ctx.lr = 0x83291610;
	sub_82CA9F20(ctx, base);
	// 83291610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329161C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83291620 size=88
    let mut pc: u32 = 0x83291620;
    'dispatch: loop {
        match pc {
            0x83291620 => {
    //   block [0x83291620..0x83291678)
	// 83291620: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83291624: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 83291628: 3921FFF4  addi r9, r1, -0xc
	ctx.r[9].s64 = ctx.r[1].s64 + -12;
	// 8329162C: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 83291630: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 83291634: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83291638: 3CC0834A  lis r6, -0x7cb6
	ctx.r[6].s64 = -2092302336;
	// 8329163C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83291678 size=132
    let mut pc: u32 = 0x83291678;
    'dispatch: loop {
        match pc {
            0x83291678 => {
    //   block [0x83291678..0x832916FC)
	// 83291678: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8329167C: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 83291680: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83291684: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83291688: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 8329168C: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83291690: 3CC0820A  lis r6, -0x7df6
	ctx.r[6].s64 = -2113273856;
	// 83291694: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83291698: 3CA0820A  lis r5, -0x7df6
	ctx.r[5].s64 = -2113273856;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291700 size=64
    let mut pc: u32 = 0x83291700;
    'dispatch: loop {
        match pc {
            0x83291700 => {
    //   block [0x83291700..0x83291740)
	// 83291700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329170C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291710: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291714: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291718: 386A5AA0  addi r3, r10, 0x5aa0
	ctx.r[3].s64 = ctx.r[10].s64 + 23200;
	// 8329171C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291720: 4AF9B7B1  bl 0x8222ced0
	ctx.lr = 0x83291724;
	sub_8222CED0(ctx, base);
	// 83291724: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291728: 38696300  addi r3, r9, 0x6300
	ctx.r[3].s64 = ctx.r[9].s64 + 25344;
	// 8329172C: 4BA187F5  bl 0x82ca9f20
	ctx.lr = 0x83291730;
	sub_82CA9F20(ctx, base);
	// 83291730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329173C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291740 size=64
    let mut pc: u32 = 0x83291740;
    'dispatch: loop {
        match pc {
            0x83291740 => {
    //   block [0x83291740..0x83291780)
	// 83291740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329174C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291750: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291754: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291758: 386A5AA4  addi r3, r10, 0x5aa4
	ctx.r[3].s64 = ctx.r[10].s64 + 23204;
	// 8329175C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291760: 4AF9B771  bl 0x8222ced0
	ctx.lr = 0x83291764;
	sub_8222CED0(ctx, base);
	// 83291764: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291768: 38696310  addi r3, r9, 0x6310
	ctx.r[3].s64 = ctx.r[9].s64 + 25360;
	// 8329176C: 4BA187B5  bl 0x82ca9f20
	ctx.lr = 0x83291770;
	sub_82CA9F20(ctx, base);
	// 83291770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329177C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291780 size=64
    let mut pc: u32 = 0x83291780;
    'dispatch: loop {
        match pc {
            0x83291780 => {
    //   block [0x83291780..0x832917C0)
	// 83291780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329178C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291790: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291794: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291798: 386A5AA8  addi r3, r10, 0x5aa8
	ctx.r[3].s64 = ctx.r[10].s64 + 23208;
	// 8329179C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832917A0: 4AF9B731  bl 0x8222ced0
	ctx.lr = 0x832917A4;
	sub_8222CED0(ctx, base);
	// 832917A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832917A8: 38696320  addi r3, r9, 0x6320
	ctx.r[3].s64 = ctx.r[9].s64 + 25376;
	// 832917AC: 4BA18775  bl 0x82ca9f20
	ctx.lr = 0x832917B0;
	sub_82CA9F20(ctx, base);
	// 832917B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832917B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832917B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832917BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832917C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832917C0 size=64
    let mut pc: u32 = 0x832917C0;
    'dispatch: loop {
        match pc {
            0x832917C0 => {
    //   block [0x832917C0..0x83291800)
	// 832917C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832917C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832917C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832917CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832917D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832917D4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832917D8: 386A5AAC  addi r3, r10, 0x5aac
	ctx.r[3].s64 = ctx.r[10].s64 + 23212;
	// 832917DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832917E0: 4AF9B6F1  bl 0x8222ced0
	ctx.lr = 0x832917E4;
	sub_8222CED0(ctx, base);
	// 832917E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832917E8: 38696330  addi r3, r9, 0x6330
	ctx.r[3].s64 = ctx.r[9].s64 + 25392;
	// 832917EC: 4BA18735  bl 0x82ca9f20
	ctx.lr = 0x832917F0;
	sub_82CA9F20(ctx, base);
	// 832917F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832917F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832917F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832917FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291800 size=64
    let mut pc: u32 = 0x83291800;
    'dispatch: loop {
        match pc {
            0x83291800 => {
    //   block [0x83291800..0x83291840)
	// 83291800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329180C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291814: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291818: 386A5AB0  addi r3, r10, 0x5ab0
	ctx.r[3].s64 = ctx.r[10].s64 + 23216;
	// 8329181C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291820: 4AF9B6B1  bl 0x8222ced0
	ctx.lr = 0x83291824;
	sub_8222CED0(ctx, base);
	// 83291824: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291828: 38696340  addi r3, r9, 0x6340
	ctx.r[3].s64 = ctx.r[9].s64 + 25408;
	// 8329182C: 4BA186F5  bl 0x82ca9f20
	ctx.lr = 0x83291830;
	sub_82CA9F20(ctx, base);
	// 83291830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329183C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291840 size=64
    let mut pc: u32 = 0x83291840;
    'dispatch: loop {
        match pc {
            0x83291840 => {
    //   block [0x83291840..0x83291880)
	// 83291840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329184C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291854: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291858: 386A5AB4  addi r3, r10, 0x5ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 23220;
	// 8329185C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291860: 4AF9B671  bl 0x8222ced0
	ctx.lr = 0x83291864;
	sub_8222CED0(ctx, base);
	// 83291864: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291868: 38696350  addi r3, r9, 0x6350
	ctx.r[3].s64 = ctx.r[9].s64 + 25424;
	// 8329186C: 4BA186B5  bl 0x82ca9f20
	ctx.lr = 0x83291870;
	sub_82CA9F20(ctx, base);
	// 83291870: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329187C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291880 size=64
    let mut pc: u32 = 0x83291880;
    'dispatch: loop {
        match pc {
            0x83291880 => {
    //   block [0x83291880..0x832918C0)
	// 83291880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329188C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291890: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291894: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291898: 386A5AB8  addi r3, r10, 0x5ab8
	ctx.r[3].s64 = ctx.r[10].s64 + 23224;
	// 8329189C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832918A0: 4AF9B631  bl 0x8222ced0
	ctx.lr = 0x832918A4;
	sub_8222CED0(ctx, base);
	// 832918A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832918A8: 38696360  addi r3, r9, 0x6360
	ctx.r[3].s64 = ctx.r[9].s64 + 25440;
	// 832918AC: 4BA18675  bl 0x82ca9f20
	ctx.lr = 0x832918B0;
	sub_82CA9F20(ctx, base);
	// 832918B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832918B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832918B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832918BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832918C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832918C0 size=64
    let mut pc: u32 = 0x832918C0;
    'dispatch: loop {
        match pc {
            0x832918C0 => {
    //   block [0x832918C0..0x83291900)
	// 832918C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832918C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832918C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832918CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832918D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832918D4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832918D8: 386A5ABC  addi r3, r10, 0x5abc
	ctx.r[3].s64 = ctx.r[10].s64 + 23228;
	// 832918DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832918E0: 4AF9B5F1  bl 0x8222ced0
	ctx.lr = 0x832918E4;
	sub_8222CED0(ctx, base);
	// 832918E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832918E8: 38696370  addi r3, r9, 0x6370
	ctx.r[3].s64 = ctx.r[9].s64 + 25456;
	// 832918EC: 4BA18635  bl 0x82ca9f20
	ctx.lr = 0x832918F0;
	sub_82CA9F20(ctx, base);
	// 832918F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832918F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832918F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832918FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291900 size=64
    let mut pc: u32 = 0x83291900;
    'dispatch: loop {
        match pc {
            0x83291900 => {
    //   block [0x83291900..0x83291940)
	// 83291900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329190C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291910: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291914: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291918: 386A5AC0  addi r3, r10, 0x5ac0
	ctx.r[3].s64 = ctx.r[10].s64 + 23232;
	// 8329191C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291920: 4AF9B5B1  bl 0x8222ced0
	ctx.lr = 0x83291924;
	sub_8222CED0(ctx, base);
	// 83291924: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291928: 38696380  addi r3, r9, 0x6380
	ctx.r[3].s64 = ctx.r[9].s64 + 25472;
	// 8329192C: 4BA185F5  bl 0x82ca9f20
	ctx.lr = 0x83291930;
	sub_82CA9F20(ctx, base);
	// 83291930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329193C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291940 size=64
    let mut pc: u32 = 0x83291940;
    'dispatch: loop {
        match pc {
            0x83291940 => {
    //   block [0x83291940..0x83291980)
	// 83291940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329194C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291950: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291954: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291958: 386A5AC4  addi r3, r10, 0x5ac4
	ctx.r[3].s64 = ctx.r[10].s64 + 23236;
	// 8329195C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291960: 4AF9B571  bl 0x8222ced0
	ctx.lr = 0x83291964;
	sub_8222CED0(ctx, base);
	// 83291964: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291968: 38696390  addi r3, r9, 0x6390
	ctx.r[3].s64 = ctx.r[9].s64 + 25488;
	// 8329196C: 4BA185B5  bl 0x82ca9f20
	ctx.lr = 0x83291970;
	sub_82CA9F20(ctx, base);
	// 83291970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329197C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291980 size=64
    let mut pc: u32 = 0x83291980;
    'dispatch: loop {
        match pc {
            0x83291980 => {
    //   block [0x83291980..0x832919C0)
	// 83291980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329198C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291990: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291994: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291998: 386A5AC8  addi r3, r10, 0x5ac8
	ctx.r[3].s64 = ctx.r[10].s64 + 23240;
	// 8329199C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832919A0: 4AF9B531  bl 0x8222ced0
	ctx.lr = 0x832919A4;
	sub_8222CED0(ctx, base);
	// 832919A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832919A8: 386963A0  addi r3, r9, 0x63a0
	ctx.r[3].s64 = ctx.r[9].s64 + 25504;
	// 832919AC: 4BA18575  bl 0x82ca9f20
	ctx.lr = 0x832919B0;
	sub_82CA9F20(ctx, base);
	// 832919B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832919B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832919B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832919BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832919C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832919C0 size=64
    let mut pc: u32 = 0x832919C0;
    'dispatch: loop {
        match pc {
            0x832919C0 => {
    //   block [0x832919C0..0x83291A00)
	// 832919C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832919C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832919C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832919CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832919D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832919D4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832919D8: 386A5ACC  addi r3, r10, 0x5acc
	ctx.r[3].s64 = ctx.r[10].s64 + 23244;
	// 832919DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832919E0: 4AF9B4F1  bl 0x8222ced0
	ctx.lr = 0x832919E4;
	sub_8222CED0(ctx, base);
	// 832919E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832919E8: 386963B0  addi r3, r9, 0x63b0
	ctx.r[3].s64 = ctx.r[9].s64 + 25520;
	// 832919EC: 4BA18535  bl 0x82ca9f20
	ctx.lr = 0x832919F0;
	sub_82CA9F20(ctx, base);
	// 832919F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832919F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832919F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832919FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291A00 size=64
    let mut pc: u32 = 0x83291A00;
    'dispatch: loop {
        match pc {
            0x83291A00 => {
    //   block [0x83291A00..0x83291A40)
	// 83291A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291A0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291A10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291A14: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291A18: 386A5AD0  addi r3, r10, 0x5ad0
	ctx.r[3].s64 = ctx.r[10].s64 + 23248;
	// 83291A1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291A20: 4AF9B4B1  bl 0x8222ced0
	ctx.lr = 0x83291A24;
	sub_8222CED0(ctx, base);
	// 83291A24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291A28: 386963C0  addi r3, r9, 0x63c0
	ctx.r[3].s64 = ctx.r[9].s64 + 25536;
	// 83291A2C: 4BA184F5  bl 0x82ca9f20
	ctx.lr = 0x83291A30;
	sub_82CA9F20(ctx, base);
	// 83291A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291A40 size=64
    let mut pc: u32 = 0x83291A40;
    'dispatch: loop {
        match pc {
            0x83291A40 => {
    //   block [0x83291A40..0x83291A80)
	// 83291A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291A4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291A50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291A54: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291A58: 386A5AD4  addi r3, r10, 0x5ad4
	ctx.r[3].s64 = ctx.r[10].s64 + 23252;
	// 83291A5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291A60: 4AF9B471  bl 0x8222ced0
	ctx.lr = 0x83291A64;
	sub_8222CED0(ctx, base);
	// 83291A64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291A68: 386963D0  addi r3, r9, 0x63d0
	ctx.r[3].s64 = ctx.r[9].s64 + 25552;
	// 83291A6C: 4BA184B5  bl 0x82ca9f20
	ctx.lr = 0x83291A70;
	sub_82CA9F20(ctx, base);
	// 83291A70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291A80 size=64
    let mut pc: u32 = 0x83291A80;
    'dispatch: loop {
        match pc {
            0x83291A80 => {
    //   block [0x83291A80..0x83291AC0)
	// 83291A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291A88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291A8C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291A90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291A94: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291A98: 386A5AD8  addi r3, r10, 0x5ad8
	ctx.r[3].s64 = ctx.r[10].s64 + 23256;
	// 83291A9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291AA0: 4AF9B431  bl 0x8222ced0
	ctx.lr = 0x83291AA4;
	sub_8222CED0(ctx, base);
	// 83291AA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291AA8: 386963E0  addi r3, r9, 0x63e0
	ctx.r[3].s64 = ctx.r[9].s64 + 25568;
	// 83291AAC: 4BA18475  bl 0x82ca9f20
	ctx.lr = 0x83291AB0;
	sub_82CA9F20(ctx, base);
	// 83291AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291AC0 size=64
    let mut pc: u32 = 0x83291AC0;
    'dispatch: loop {
        match pc {
            0x83291AC0 => {
    //   block [0x83291AC0..0x83291B00)
	// 83291AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291ACC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291AD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291AD4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291AD8: 386A5ADC  addi r3, r10, 0x5adc
	ctx.r[3].s64 = ctx.r[10].s64 + 23260;
	// 83291ADC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291AE0: 4AF9B3F1  bl 0x8222ced0
	ctx.lr = 0x83291AE4;
	sub_8222CED0(ctx, base);
	// 83291AE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291AE8: 386963F0  addi r3, r9, 0x63f0
	ctx.r[3].s64 = ctx.r[9].s64 + 25584;
	// 83291AEC: 4BA18435  bl 0x82ca9f20
	ctx.lr = 0x83291AF0;
	sub_82CA9F20(ctx, base);
	// 83291AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291B00 size=64
    let mut pc: u32 = 0x83291B00;
    'dispatch: loop {
        match pc {
            0x83291B00 => {
    //   block [0x83291B00..0x83291B40)
	// 83291B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291B0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291B10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291B14: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291B18: 386A5AE0  addi r3, r10, 0x5ae0
	ctx.r[3].s64 = ctx.r[10].s64 + 23264;
	// 83291B1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291B20: 4AF9B3B1  bl 0x8222ced0
	ctx.lr = 0x83291B24;
	sub_8222CED0(ctx, base);
	// 83291B24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291B28: 38696400  addi r3, r9, 0x6400
	ctx.r[3].s64 = ctx.r[9].s64 + 25600;
	// 83291B2C: 4BA183F5  bl 0x82ca9f20
	ctx.lr = 0x83291B30;
	sub_82CA9F20(ctx, base);
	// 83291B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291B40 size=64
    let mut pc: u32 = 0x83291B40;
    'dispatch: loop {
        match pc {
            0x83291B40 => {
    //   block [0x83291B40..0x83291B80)
	// 83291B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291B4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291B50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291B54: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291B58: 386A5AE4  addi r3, r10, 0x5ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 23268;
	// 83291B5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291B60: 4AF9B371  bl 0x8222ced0
	ctx.lr = 0x83291B64;
	sub_8222CED0(ctx, base);
	// 83291B64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291B68: 38696410  addi r3, r9, 0x6410
	ctx.r[3].s64 = ctx.r[9].s64 + 25616;
	// 83291B6C: 4BA183B5  bl 0x82ca9f20
	ctx.lr = 0x83291B70;
	sub_82CA9F20(ctx, base);
	// 83291B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83291B80 size=12
    let mut pc: u32 = 0x83291B80;
    'dispatch: loop {
        match pc {
            0x83291B80 => {
    //   block [0x83291B80..0x83291B8C)
	// 83291B80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83291B84: 386B5AE8  addi r3, r11, 0x5ae8
	ctx.r[3].s64 = ctx.r[11].s64 + 23272;
	// 83291B88: 4B829AF8  b 0x82abb680
	sub_82ABB680(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291B90 size=64
    let mut pc: u32 = 0x83291B90;
    'dispatch: loop {
        match pc {
            0x83291B90 => {
    //   block [0x83291B90..0x83291BD0)
	// 83291B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291B98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291B9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291BA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291BA4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291BA8: 386A5D2C  addi r3, r10, 0x5d2c
	ctx.r[3].s64 = ctx.r[10].s64 + 23852;
	// 83291BAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291BB0: 4AF9B321  bl 0x8222ced0
	ctx.lr = 0x83291BB4;
	sub_8222CED0(ctx, base);
	// 83291BB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291BB8: 38696420  addi r3, r9, 0x6420
	ctx.r[3].s64 = ctx.r[9].s64 + 25632;
	// 83291BBC: 4BA18365  bl 0x82ca9f20
	ctx.lr = 0x83291BC0;
	sub_82CA9F20(ctx, base);
	// 83291BC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291BD0 size=64
    let mut pc: u32 = 0x83291BD0;
    'dispatch: loop {
        match pc {
            0x83291BD0 => {
    //   block [0x83291BD0..0x83291C10)
	// 83291BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291BD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291BDC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291BE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291BE4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291BE8: 386A5D30  addi r3, r10, 0x5d30
	ctx.r[3].s64 = ctx.r[10].s64 + 23856;
	// 83291BEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291BF0: 4AF9B2E1  bl 0x8222ced0
	ctx.lr = 0x83291BF4;
	sub_8222CED0(ctx, base);
	// 83291BF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291BF8: 38696430  addi r3, r9, 0x6430
	ctx.r[3].s64 = ctx.r[9].s64 + 25648;
	// 83291BFC: 4BA18325  bl 0x82ca9f20
	ctx.lr = 0x83291C00;
	sub_82CA9F20(ctx, base);
	// 83291C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291C10 size=64
    let mut pc: u32 = 0x83291C10;
    'dispatch: loop {
        match pc {
            0x83291C10 => {
    //   block [0x83291C10..0x83291C50)
	// 83291C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291C1C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291C20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291C24: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291C28: 386A5D34  addi r3, r10, 0x5d34
	ctx.r[3].s64 = ctx.r[10].s64 + 23860;
	// 83291C2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291C30: 4AF9B2A1  bl 0x8222ced0
	ctx.lr = 0x83291C34;
	sub_8222CED0(ctx, base);
	// 83291C34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291C38: 38696440  addi r3, r9, 0x6440
	ctx.r[3].s64 = ctx.r[9].s64 + 25664;
	// 83291C3C: 4BA182E5  bl 0x82ca9f20
	ctx.lr = 0x83291C40;
	sub_82CA9F20(ctx, base);
	// 83291C40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291C50 size=64
    let mut pc: u32 = 0x83291C50;
    'dispatch: loop {
        match pc {
            0x83291C50 => {
    //   block [0x83291C50..0x83291C90)
	// 83291C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291C58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291C5C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291C60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291C64: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291C68: 386A5D38  addi r3, r10, 0x5d38
	ctx.r[3].s64 = ctx.r[10].s64 + 23864;
	// 83291C6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291C70: 4AF9B261  bl 0x8222ced0
	ctx.lr = 0x83291C74;
	sub_8222CED0(ctx, base);
	// 83291C74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291C78: 38696450  addi r3, r9, 0x6450
	ctx.r[3].s64 = ctx.r[9].s64 + 25680;
	// 83291C7C: 4BA182A5  bl 0x82ca9f20
	ctx.lr = 0x83291C80;
	sub_82CA9F20(ctx, base);
	// 83291C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291C90 size=64
    let mut pc: u32 = 0x83291C90;
    'dispatch: loop {
        match pc {
            0x83291C90 => {
    //   block [0x83291C90..0x83291CD0)
	// 83291C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291C9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291CA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291CA4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291CA8: 386A5D3C  addi r3, r10, 0x5d3c
	ctx.r[3].s64 = ctx.r[10].s64 + 23868;
	// 83291CAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291CB0: 4AF9B221  bl 0x8222ced0
	ctx.lr = 0x83291CB4;
	sub_8222CED0(ctx, base);
	// 83291CB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291CB8: 38696460  addi r3, r9, 0x6460
	ctx.r[3].s64 = ctx.r[9].s64 + 25696;
	// 83291CBC: 4BA18265  bl 0x82ca9f20
	ctx.lr = 0x83291CC0;
	sub_82CA9F20(ctx, base);
	// 83291CC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291CD0 size=64
    let mut pc: u32 = 0x83291CD0;
    'dispatch: loop {
        match pc {
            0x83291CD0 => {
    //   block [0x83291CD0..0x83291D10)
	// 83291CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291CDC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291CE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291CE4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291CE8: 386A5D40  addi r3, r10, 0x5d40
	ctx.r[3].s64 = ctx.r[10].s64 + 23872;
	// 83291CEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291CF0: 4AF9B1E1  bl 0x8222ced0
	ctx.lr = 0x83291CF4;
	sub_8222CED0(ctx, base);
	// 83291CF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291CF8: 38696470  addi r3, r9, 0x6470
	ctx.r[3].s64 = ctx.r[9].s64 + 25712;
	// 83291CFC: 4BA18225  bl 0x82ca9f20
	ctx.lr = 0x83291D00;
	sub_82CA9F20(ctx, base);
	// 83291D00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291D10 size=64
    let mut pc: u32 = 0x83291D10;
    'dispatch: loop {
        match pc {
            0x83291D10 => {
    //   block [0x83291D10..0x83291D50)
	// 83291D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291D18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291D1C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291D20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291D24: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291D28: 386A5D44  addi r3, r10, 0x5d44
	ctx.r[3].s64 = ctx.r[10].s64 + 23876;
	// 83291D2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291D30: 4AF9B1A1  bl 0x8222ced0
	ctx.lr = 0x83291D34;
	sub_8222CED0(ctx, base);
	// 83291D34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291D38: 38696480  addi r3, r9, 0x6480
	ctx.r[3].s64 = ctx.r[9].s64 + 25728;
	// 83291D3C: 4BA181E5  bl 0x82ca9f20
	ctx.lr = 0x83291D40;
	sub_82CA9F20(ctx, base);
	// 83291D40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291D50 size=64
    let mut pc: u32 = 0x83291D50;
    'dispatch: loop {
        match pc {
            0x83291D50 => {
    //   block [0x83291D50..0x83291D90)
	// 83291D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291D58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291D5C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291D60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291D64: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291D68: 386A5D48  addi r3, r10, 0x5d48
	ctx.r[3].s64 = ctx.r[10].s64 + 23880;
	// 83291D6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291D70: 4AF9B161  bl 0x8222ced0
	ctx.lr = 0x83291D74;
	sub_8222CED0(ctx, base);
	// 83291D74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291D78: 38696490  addi r3, r9, 0x6490
	ctx.r[3].s64 = ctx.r[9].s64 + 25744;
	// 83291D7C: 4BA181A5  bl 0x82ca9f20
	ctx.lr = 0x83291D80;
	sub_82CA9F20(ctx, base);
	// 83291D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83291D90 size=12
    let mut pc: u32 = 0x83291D90;
    'dispatch: loop {
        match pc {
            0x83291D90 => {
    //   block [0x83291D90..0x83291D9C)
	// 83291D90: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83291D94: 386B64A0  addi r3, r11, 0x64a0
	ctx.r[3].s64 = ctx.r[11].s64 + 25760;
	// 83291D98: 4BA18188  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291DA0 size=64
    let mut pc: u32 = 0x83291DA0;
    'dispatch: loop {
        match pc {
            0x83291DA0 => {
    //   block [0x83291DA0..0x83291DE0)
	// 83291DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291DA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291DAC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291DB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291DB4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291DB8: 386A5D5C  addi r3, r10, 0x5d5c
	ctx.r[3].s64 = ctx.r[10].s64 + 23900;
	// 83291DBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291DC0: 4AF9B111  bl 0x8222ced0
	ctx.lr = 0x83291DC4;
	sub_8222CED0(ctx, base);
	// 83291DC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291DC8: 386964F8  addi r3, r9, 0x64f8
	ctx.r[3].s64 = ctx.r[9].s64 + 25848;
	// 83291DCC: 4BA18155  bl 0x82ca9f20
	ctx.lr = 0x83291DD0;
	sub_82CA9F20(ctx, base);
	// 83291DD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291DE0 size=64
    let mut pc: u32 = 0x83291DE0;
    'dispatch: loop {
        match pc {
            0x83291DE0 => {
    //   block [0x83291DE0..0x83291E20)
	// 83291DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291DE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291DEC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291DF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291DF4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291DF8: 386A5D60  addi r3, r10, 0x5d60
	ctx.r[3].s64 = ctx.r[10].s64 + 23904;
	// 83291DFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291E00: 4AF9B0D1  bl 0x8222ced0
	ctx.lr = 0x83291E04;
	sub_8222CED0(ctx, base);
	// 83291E04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291E08: 38696508  addi r3, r9, 0x6508
	ctx.r[3].s64 = ctx.r[9].s64 + 25864;
	// 83291E0C: 4BA18115  bl 0x82ca9f20
	ctx.lr = 0x83291E10;
	sub_82CA9F20(ctx, base);
	// 83291E10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83291E20 size=800
    let mut pc: u32 = 0x83291E20;
    'dispatch: loop {
        match pc {
            0x83291E20 => {
    //   block [0x83291E20..0x83292140)
	// 83291E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291E24: 4BA175C1  bl 0x82ca93e4
	ctx.lr = 0x83291E28;
	sub_82CA93D0(ctx, base);
	// 83291E28: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83291E2C: 3941FF40  addi r10, r1, -0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + -192;
	// 83291E30: 392B92D4  addi r9, r11, -0x6d2c
	ctx.r[9].s64 = ctx.r[11].s64 + -27948;
	// 83291E34: 3901FF44  addi r8, r1, -0xbc
	ctx.r[8].s64 = ctx.r[1].s64 + -188;
	// 83291E38: 38E1FF48  addi r7, r1, -0xb8
	ctx.r[7].s64 = ctx.r[1].s64 + -184;
	// 83291E3C: 38C1FF4C  addi r6, r1, -0xb4
	ctx.r[6].s64 = ctx.r[1].s64 + -180;
	// 83291E40: C18B92D4  lfs f12, -0x6d2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83291E44: 38A1FF50  addi r5, r1, -0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + -176;
	// 83291E48: C00901B0  lfs f0, 0x1b0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(432 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83291E4C: 3881FF54  addi r4, r1, -0xac
	ctx.r[4].s64 = ctx.r[1].s64 + -172;
	// 83291E50: C1A901BC  lfs f13, 0x1bc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(444 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83291E54: 3861FF58  addi r3, r1, -0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + -168;
	// 83291E58: D001FF40  stfs f0, -0xc0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-192 as u32), tmp.u32 ) };
	// 83291E5C: 3921FF5C  addi r9, r1, -0xa4
	ctx.r[9].s64 = ctx.r[1].s64 + -164;
	// 83291E60: D1A1FF44  stfs f13, -0xbc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), tmp.u32 ) };
	// 83291E64: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83291E68: D1A1FF48  stfs f13, -0xb8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), tmp.u32 ) };
	// 83291E6C: 3BC1FF64  addi r30, r1, -0x9c
	ctx.r[30].s64 = ctx.r[1].s64 + -156;
	// 83291E70: D001FF4C  stfs f0, -0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-180 as u32), tmp.u32 ) };
	// 83291E74: 3BA1FF68  addi r29, r1, -0x98
	ctx.r[29].s64 = ctx.r[1].s64 + -152;
	// 83291E78: D001FF50  stfs f0, -0xb0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), tmp.u32 ) };
	// 83291E7C: 3B81FF6C  addi r28, r1, -0x94
	ctx.r[28].s64 = ctx.r[1].s64 + -148;
	// 83291E80: D001FF54  stfs f0, -0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-172 as u32), tmp.u32 ) };
	// 83291E84: 3B61FF70  addi r27, r1, -0x90
	ctx.r[27].s64 = ctx.r[1].s64 + -144;
	// 83291E88: D1A1FF58  stfs f13, -0xa8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), tmp.u32 ) };
	// 83291E8C: 3B41FF74  addi r26, r1, -0x8c
	ctx.r[26].s64 = ctx.r[1].s64 + -140;
	// 83291E90: D1A1FF5C  stfs f13, -0xa4(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-164 as u32), tmp.u32 ) };
	// 83291E94: 3B21FF78  addi r25, r1, -0x88
	ctx.r[25].s64 = ctx.r[1].s64 + -136;
	// 83291E98: D001FF60  stfs f0, -0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), tmp.u32 ) };
	// 83291E9C: 3B01FF7C  addi r24, r1, -0x84
	ctx.r[24].s64 = ctx.r[1].s64 + -132;
	// 83291EA0: D001FF64  stfs f0, -0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-156 as u32), tmp.u32 ) };
	// 83291EA4: 3AE1FF80  addi r23, r1, -0x80
	ctx.r[23].s64 = ctx.r[1].s64 + -128;
	// 83291EA8: D001FF68  stfs f0, -0x98(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), tmp.u32 ) };
	// 83291EAC: 3AC1FF84  addi r22, r1, -0x7c
	ctx.r[22].s64 = ctx.r[1].s64 + -124;
	// 83291EB0: D001FF6C  stfs f0, -0x94(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-148 as u32), tmp.u32 ) };
	// 83291EB4: 3AA1FF88  addi r21, r1, -0x78
	ctx.r[21].s64 = ctx.r[1].s64 + -120;
	// 83291EB8: D001FF70  stfs f0, -0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), tmp.u32 ) };
	// 83291EBC: 3A81FF8C  addi r20, r1, -0x74
	ctx.r[20].s64 = ctx.r[1].s64 + -116;
	// 83291EC0: D1A1FF74  stfs f13, -0x8c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-140 as u32), tmp.u32 ) };
	// 83291EC4: 3E608332  lis r19, -0x7cce
	ctx.r[19].s64 = -2093875200;
	// 83291EC8: D001FF78  stfs f0, -0x88(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), tmp.u32 ) };
	// 83291ECC: D001FF7C  stfs f0, -0x84(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-132 as u32), tmp.u32 ) };
	// 83291ED0: 3A73AE50  addi r19, r19, -0x51b0
	ctx.r[19].s64 = ctx.r[19].s64 + -20912;
	// 83291ED4: D001FF80  stfs f0, -0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), tmp.u32 ) };
	// 83291ED8: D1A1FF84  stfs f13, -0x7c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-124 as u32), tmp.u32 ) };
	// 83291EDC: D001FF88  stfs f0, -0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), tmp.u32 ) };
	// 83291EE0: D1A1FF8C  stfs f13, -0x74(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-116 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83292140 size=456
    let mut pc: u32 = 0x83292140;
    'dispatch: loop {
        match pc {
            0x83292140 => {
    //   block [0x83292140..0x83292308)
	// 83292140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292144: 4BA172A5  bl 0x82ca93e8
	ctx.lr = 0x83292148;
	sub_82CA93D0(ctx, base);
	// 83292148: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8329214C: 3901FF44  addi r8, r1, -0xbc
	ctx.r[8].s64 = ctx.r[1].s64 + -188;
	// 83292150: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 83292154: 3941FF40  addi r10, r1, -0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + -192;
	// 83292158: 38E1FF48  addi r7, r1, -0xb8
	ctx.r[7].s64 = ctx.r[1].s64 + -184;
	// 8329215C: C1AB9490  lfs f13, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83292160: 38C1FF4C  addi r6, r1, -0xb4
	ctx.r[6].s64 = ctx.r[1].s64 + -180;
	// 83292164: D1A1FF40  stfs f13, -0xc0(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-192 as u32), tmp.u32 ) };
	// 83292168: 38A1FF50  addi r5, r1, -0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + -176;
	// 8329216C: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83292170: 3881FF54  addi r4, r1, -0xac
	ctx.r[4].s64 = ctx.r[1].s64 + -172;
	// 83292174: D001FF44  stfs f0, -0xbc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), tmp.u32 ) };
	// 83292178: 3861FF58  addi r3, r1, -0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + -168;
	// 8329217C: D001FF48  stfs f0, -0xb8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), tmp.u32 ) };
	// 83292180: 3961FF5C  addi r11, r1, -0xa4
	ctx.r[11].s64 = ctx.r[1].s64 + -164;
	// 83292184: D001FF4C  stfs f0, -0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-180 as u32), tmp.u32 ) };
	// 83292188: 3921FF60  addi r9, r1, -0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + -160;
	// 8329218C: D1A1FF50  stfs f13, -0xb0(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), tmp.u32 ) };
	// 83292190: 3BE1FF64  addi r31, r1, -0x9c
	ctx.r[31].s64 = ctx.r[1].s64 + -156;
	// 83292194: D1A1FF54  stfs f13, -0xac(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-172 as u32), tmp.u32 ) };
	// 83292198: 3BC1FF68  addi r30, r1, -0x98
	ctx.r[30].s64 = ctx.r[1].s64 + -152;
	// 8329219C: D001FF58  stfs f0, -0xa8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), tmp.u32 ) };
	// 832921A0: 3BA1FF6C  addi r29, r1, -0x94
	ctx.r[29].s64 = ctx.r[1].s64 + -148;
	// 832921A4: D001FF5C  stfs f0, -0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-164 as u32), tmp.u32 ) };
	// 832921A8: 3B81FF70  addi r28, r1, -0x90
	ctx.r[28].s64 = ctx.r[1].s64 + -144;
	// 832921AC: D001FF60  stfs f0, -0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), tmp.u32 ) };
	// 832921B0: 3B61FF74  addi r27, r1, -0x8c
	ctx.r[27].s64 = ctx.r[1].s64 + -140;
	// 832921B4: D001FF64  stfs f0, -0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-156 as u32), tmp.u32 ) };
	// 832921B8: 3B41FF78  addi r26, r1, -0x88
	ctx.r[26].s64 = ctx.r[1].s64 + -136;
	// 832921BC: D001FF68  stfs f0, -0x98(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), tmp.u32 ) };
	// 832921C0: 3B21FF7C  addi r25, r1, -0x84
	ctx.r[25].s64 = ctx.r[1].s64 + -132;
	// 832921C4: D1A1FF6C  stfs f13, -0x94(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-148 as u32), tmp.u32 ) };
	// 832921C8: 3B01FF80  addi r24, r1, -0x80
	ctx.r[24].s64 = ctx.r[1].s64 + -128;
	// 832921CC: D001FF70  stfs f0, -0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), tmp.u32 ) };
	// 832921D0: 3AE1FF84  addi r23, r1, -0x7c
	ctx.r[23].s64 = ctx.r[1].s64 + -124;
	// 832921D4: D001FF74  stfs f0, -0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-140 as u32), tmp.u32 ) };
	// 832921D8: 3AC1FF88  addi r22, r1, -0x78
	ctx.r[22].s64 = ctx.r[1].s64 + -120;
	// 832921DC: D001FF78  stfs f0, -0x88(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), tmp.u32 ) };
	// 832921E0: 3AA1FF8C  addi r21, r1, -0x74
	ctx.r[21].s64 = ctx.r[1].s64 + -116;
	// 832921E4: D001FF7C  stfs f0, -0x84(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-132 as u32), tmp.u32 ) };
	// 832921E8: 3E808332  lis r20, -0x7cce
	ctx.r[20].s64 = -2093875200;
	// 832921EC: D001FF80  stfs f0, -0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), tmp.u32 ) };
	// 832921F0: D1A1FF84  stfs f13, -0x7c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-124 as u32), tmp.u32 ) };
	// 832921F4: 3A94AFB0  addi r20, r20, -0x5050
	ctx.r[20].s64 = ctx.r[20].s64 + -20560;
	// 832921F8: D001FF88  stfs f0, -0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), tmp.u32 ) };
	// 832921FC: D1A1FF8C  stfs f13, -0x74(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-116 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292308 size=64
    let mut pc: u32 = 0x83292308;
    'dispatch: loop {
        match pc {
            0x83292308 => {
    //   block [0x83292308..0x83292348)
	// 83292308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329230C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292314: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329231C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83292320: 386A5D64  addi r3, r10, 0x5d64
	ctx.r[3].s64 = ctx.r[10].s64 + 23908;
	// 83292324: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83292328: 4AF9ABA9  bl 0x8222ced0
	ctx.lr = 0x8329232C;
	sub_8222CED0(ctx, base);
	// 8329232C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292330: 38696518  addi r3, r9, 0x6518
	ctx.r[3].s64 = ctx.r[9].s64 + 25880;
	// 83292334: 4BA17BED  bl 0x82ca9f20
	ctx.lr = 0x83292338;
	sub_82CA9F20(ctx, base);
	// 83292338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329233C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292348 size=64
    let mut pc: u32 = 0x83292348;
    'dispatch: loop {
        match pc {
            0x83292348 => {
    //   block [0x83292348..0x83292388)
	// 83292348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329234C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292354: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329235C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83292360: 386A5D68  addi r3, r10, 0x5d68
	ctx.r[3].s64 = ctx.r[10].s64 + 23912;
	// 83292364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83292368: 4AF9AB69  bl 0x8222ced0
	ctx.lr = 0x8329236C;
	sub_8222CED0(ctx, base);
	// 8329236C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292370: 38696528  addi r3, r9, 0x6528
	ctx.r[3].s64 = ctx.r[9].s64 + 25896;
	// 83292374: 4BA17BAD  bl 0x82ca9f20
	ctx.lr = 0x83292378;
	sub_82CA9F20(ctx, base);
	// 83292378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329237C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292388 size=64
    let mut pc: u32 = 0x83292388;
    'dispatch: loop {
        match pc {
            0x83292388 => {
    //   block [0x83292388..0x832923C8)
	// 83292388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329238C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292394: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292398: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329239C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832923A0: 386A5D6C  addi r3, r10, 0x5d6c
	ctx.r[3].s64 = ctx.r[10].s64 + 23916;
	// 832923A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832923A8: 4AF9AB29  bl 0x8222ced0
	ctx.lr = 0x832923AC;
	sub_8222CED0(ctx, base);
	// 832923AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832923B0: 38696538  addi r3, r9, 0x6538
	ctx.r[3].s64 = ctx.r[9].s64 + 25912;
	// 832923B4: 4BA17B6D  bl 0x82ca9f20
	ctx.lr = 0x832923B8;
	sub_82CA9F20(ctx, base);
	// 832923B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832923BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832923C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832923C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832923C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832923C8 size=64
    let mut pc: u32 = 0x832923C8;
    'dispatch: loop {
        match pc {
            0x832923C8 => {
    //   block [0x832923C8..0x83292408)
	// 832923C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832923CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832923D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832923D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832923D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832923DC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832923E0: 386A5D70  addi r3, r10, 0x5d70
	ctx.r[3].s64 = ctx.r[10].s64 + 23920;
	// 832923E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832923E8: 4AF9AAE9  bl 0x8222ced0
	ctx.lr = 0x832923EC;
	sub_8222CED0(ctx, base);
	// 832923EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832923F0: 38696548  addi r3, r9, 0x6548
	ctx.r[3].s64 = ctx.r[9].s64 + 25928;
	// 832923F4: 4BA17B2D  bl 0x82ca9f20
	ctx.lr = 0x832923F8;
	sub_82CA9F20(ctx, base);
	// 832923F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832923FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292408 size=64
    let mut pc: u32 = 0x83292408;
    'dispatch: loop {
        match pc {
            0x83292408 => {
    //   block [0x83292408..0x83292448)
	// 83292408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329240C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292414: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292418: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329241C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83292420: 386A5D74  addi r3, r10, 0x5d74
	ctx.r[3].s64 = ctx.r[10].s64 + 23924;
	// 83292424: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83292428: 4AF9AAA9  bl 0x8222ced0
	ctx.lr = 0x8329242C;
	sub_8222CED0(ctx, base);
	// 8329242C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292430: 38696558  addi r3, r9, 0x6558
	ctx.r[3].s64 = ctx.r[9].s64 + 25944;
	// 83292434: 4BA17AED  bl 0x82ca9f20
	ctx.lr = 0x83292438;
	sub_82CA9F20(ctx, base);
	// 83292438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329243C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292448 size=64
    let mut pc: u32 = 0x83292448;
    'dispatch: loop {
        match pc {
            0x83292448 => {
    //   block [0x83292448..0x83292488)
	// 83292448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329244C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292454: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292458: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329245C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83292460: 386A5D78  addi r3, r10, 0x5d78
	ctx.r[3].s64 = ctx.r[10].s64 + 23928;
	// 83292464: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83292468: 4AF9AA69  bl 0x8222ced0
	ctx.lr = 0x8329246C;
	sub_8222CED0(ctx, base);
	// 8329246C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292470: 38696568  addi r3, r9, 0x6568
	ctx.r[3].s64 = ctx.r[9].s64 + 25960;
	// 83292474: 4BA17AAD  bl 0x82ca9f20
	ctx.lr = 0x83292478;
	sub_82CA9F20(ctx, base);
	// 83292478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329247C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292488 size=64
    let mut pc: u32 = 0x83292488;
    'dispatch: loop {
        match pc {
            0x83292488 => {
    //   block [0x83292488..0x832924C8)
	// 83292488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329248C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292494: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292498: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329249C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832924A0: 386A5D7C  addi r3, r10, 0x5d7c
	ctx.r[3].s64 = ctx.r[10].s64 + 23932;
	// 832924A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832924A8: 4AF9AA29  bl 0x8222ced0
	ctx.lr = 0x832924AC;
	sub_8222CED0(ctx, base);
	// 832924AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832924B0: 38696578  addi r3, r9, 0x6578
	ctx.r[3].s64 = ctx.r[9].s64 + 25976;
	// 832924B4: 4BA17A6D  bl 0x82ca9f20
	ctx.lr = 0x832924B8;
	sub_82CA9F20(ctx, base);
	// 832924B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832924BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832924C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832924C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832924C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832924C8 size=64
    let mut pc: u32 = 0x832924C8;
    'dispatch: loop {
        match pc {
            0x832924C8 => {
    //   block [0x832924C8..0x83292508)
	// 832924C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832924CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832924D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832924D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832924D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832924DC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832924E0: 386A5D80  addi r3, r10, 0x5d80
	ctx.r[3].s64 = ctx.r[10].s64 + 23936;
	// 832924E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832924E8: 4AF9A9E9  bl 0x8222ced0
	ctx.lr = 0x832924EC;
	sub_8222CED0(ctx, base);
	// 832924EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832924F0: 38696588  addi r3, r9, 0x6588
	ctx.r[3].s64 = ctx.r[9].s64 + 25992;
	// 832924F4: 4BA17A2D  bl 0x82ca9f20
	ctx.lr = 0x832924F8;
	sub_82CA9F20(ctx, base);
	// 832924F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832924FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292508 size=64
    let mut pc: u32 = 0x83292508;
    'dispatch: loop {
        match pc {
            0x83292508 => {
    //   block [0x83292508..0x83292548)
	// 83292508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329250C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292514: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292518: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329251C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83292520: 386A5D84  addi r3, r10, 0x5d84
	ctx.r[3].s64 = ctx.r[10].s64 + 23940;
	// 83292524: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83292528: 4AF9A9A9  bl 0x8222ced0
	ctx.lr = 0x8329252C;
	sub_8222CED0(ctx, base);
	// 8329252C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292530: 38696598  addi r3, r9, 0x6598
	ctx.r[3].s64 = ctx.r[9].s64 + 26008;
	// 83292534: 4BA179ED  bl 0x82ca9f20
	ctx.lr = 0x83292538;
	sub_82CA9F20(ctx, base);
	// 83292538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329253C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292548 size=64
    let mut pc: u32 = 0x83292548;
    'dispatch: loop {
        match pc {
            0x83292548 => {
    //   block [0x83292548..0x83292588)
	// 83292548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329254C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292554: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292558: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329255C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83292560: 386A5D88  addi r3, r10, 0x5d88
	ctx.r[3].s64 = ctx.r[10].s64 + 23944;
	// 83292564: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83292568: 4AF9A969  bl 0x8222ced0
	ctx.lr = 0x8329256C;
	sub_8222CED0(ctx, base);
	// 8329256C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292570: 386965A8  addi r3, r9, 0x65a8
	ctx.r[3].s64 = ctx.r[9].s64 + 26024;
	// 83292574: 4BA179AD  bl 0x82ca9f20
	ctx.lr = 0x83292578;
	sub_82CA9F20(ctx, base);
	// 83292578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329257C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83292588 size=100
    let mut pc: u32 = 0x83292588;
    'dispatch: loop {
        match pc {
            0x83292588 => {
    //   block [0x83292588..0x832925EC)
	// 83292588: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8329258C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83292590: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 83292594: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83292598: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 8329259C: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832925A0: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 832925A4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832925A8: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 832925AC: C1A9FFF4  lfs f13, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832925B0: D1A1FFF4  stfs f13, -0xc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 832925B4: 38855D90  addi r4, r5, 0x5d90
	ctx.r[4].s64 = ctx.r[5].s64 + 23952;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832925F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832925F0 size=64
    let mut pc: u32 = 0x832925F0;
    'dispatch: loop {
        match pc {
            0x832925F0 => {
    //   block [0x832925F0..0x83292630)
	// 832925F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832925F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832925F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832925FC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292600: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83292604: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83292608: 386A5DA0  addi r3, r10, 0x5da0
	ctx.r[3].s64 = ctx.r[10].s64 + 23968;
	// 8329260C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83292610: 4AF9A8C1  bl 0x8222ced0
	ctx.lr = 0x83292614;
	sub_8222CED0(ctx, base);
	// 83292614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292618: 386965B8  addi r3, r9, 0x65b8
	ctx.r[3].s64 = ctx.r[9].s64 + 26040;
	// 8329261C: 4BA17905  bl 0x82ca9f20
	ctx.lr = 0x83292620;
	sub_82CA9F20(ctx, base);
	// 83292620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329262C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292630 size=64
    let mut pc: u32 = 0x83292630;
    'dispatch: loop {
        match pc {
            0x83292630 => {
    //   block [0x83292630..0x83292670)
	// 83292630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329263C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83292644: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83292648: 386A5DA4  addi r3, r10, 0x5da4
	ctx.r[3].s64 = ctx.r[10].s64 + 23972;
	// 8329264C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83292650: 4AF9A881  bl 0x8222ced0
	ctx.lr = 0x83292654;
	sub_8222CED0(ctx, base);
	// 83292654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292658: 386965C8  addi r3, r9, 0x65c8
	ctx.r[3].s64 = ctx.r[9].s64 + 26056;
	// 8329265C: 4BA178C5  bl 0x82ca9f20
	ctx.lr = 0x83292660;
	sub_82CA9F20(ctx, base);
	// 83292660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329266C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292670 size=40
    let mut pc: u32 = 0x83292670;
    'dispatch: loop {
        match pc {
            0x83292670 => {
    //   block [0x83292670..0x83292698)
	// 83292670: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292674: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292678: 396B5DA8  addi r11, r11, 0x5da8
	ctx.r[11].s64 = ctx.r[11].s64 + 23976;
	// 8329267C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292680: 386965D8  addi r3, r9, 0x65d8
	ctx.r[3].s64 = ctx.r[9].s64 + 26072;
	// 83292684: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83292688: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329268C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83292690: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83292694: 4BA1788C  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292698 size=64
    let mut pc: u32 = 0x83292698;
    'dispatch: loop {
        match pc {
            0x83292698 => {
    //   block [0x83292698..0x832926D8)
	// 83292698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329269C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832926A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832926A4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832926A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832926AC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832926B0: 386A5DBC  addi r3, r10, 0x5dbc
	ctx.r[3].s64 = ctx.r[10].s64 + 23996;
	// 832926B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832926B8: 4AF9A819  bl 0x8222ced0
	ctx.lr = 0x832926BC;
	sub_8222CED0(ctx, base);
	// 832926BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832926C0: 386965F0  addi r3, r9, 0x65f0
	ctx.r[3].s64 = ctx.r[9].s64 + 26096;
	// 832926C4: 4BA1785D  bl 0x82ca9f20
	ctx.lr = 0x832926C8;
	sub_82CA9F20(ctx, base);
	// 832926C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832926CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832926D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832926D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832926D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832926D8 size=64
    let mut pc: u32 = 0x832926D8;
    'dispatch: loop {
        match pc {
            0x832926D8 => {
    //   block [0x832926D8..0x83292718)
	// 832926D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832926DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832926E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832926E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832926E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832926EC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832926F0: 386A5DC0  addi r3, r10, 0x5dc0
	ctx.r[3].s64 = ctx.r[10].s64 + 24000;
	// 832926F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832926F8: 4AF9A7D9  bl 0x8222ced0
	ctx.lr = 0x832926FC;
	sub_8222CED0(ctx, base);
	// 832926FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292700: 38696600  addi r3, r9, 0x6600
	ctx.r[3].s64 = ctx.r[9].s64 + 26112;
	// 83292704: 4BA1781D  bl 0x82ca9f20
	ctx.lr = 0x83292708;
	sub_82CA9F20(ctx, base);
	// 83292708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329270C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83292718 size=312
    let mut pc: u32 = 0x83292718;
    'dispatch: loop {
        match pc {
            0x83292718 => {
    //   block [0x83292718..0x83292850)
	// 83292718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329271C: 4BA16CE9  bl 0x82ca9404
	ctx.lr = 0x83292720;
	sub_82CA93D0(ctx, base);
	// 83292720: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83292724: 38E1FFA8  addi r7, r1, -0x58
	ctx.r[7].s64 = ctx.r[1].s64 + -88;
	// 83292728: 392B9484  addi r9, r11, -0x6b7c
	ctx.r[9].s64 = ctx.r[11].s64 + -27516;
	// 8329272C: 38C1FFAC  addi r6, r1, -0x54
	ctx.r[6].s64 = ctx.r[1].s64 + -84;
	// 83292730: 3941FFA0  addi r10, r1, -0x60
	ctx.r[10].s64 = ctx.r[1].s64 + -96;
	// 83292734: C1AB9484  lfs f13, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83292738: 3901FFA4  addi r8, r1, -0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + -92;
	// 8329273C: D1A1FFAC  stfs f13, -0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-84 as u32), tmp.u32 ) };
	// 83292740: 3BC1FFC0  addi r30, r1, -0x40
	ctx.r[30].s64 = ctx.r[1].s64 + -64;
	// 83292744: C009000C  lfs f0, 0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83292748: 3BA1FFC4  addi r29, r1, -0x3c
	ctx.r[29].s64 = ctx.r[1].s64 + -60;
	// 8329274C: D001FFA8  stfs f0, -0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), tmp.u32 ) };
	// 83292750: 38A1FFB0  addi r5, r1, -0x50
	ctx.r[5].s64 = ctx.r[1].s64 + -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292850 size=4
    let mut pc: u32 = 0x83292850;
    'dispatch: loop {
        match pc {
            0x83292850 => {
    //   block [0x83292850..0x83292854)
	// 83292850: 4B8AAC58  b 0x82b3d4a8
	sub_82B3D4A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292858 size=16
    let mut pc: u32 = 0x83292858;
    'dispatch: loop {
        match pc {
            0x83292858 => {
    //   block [0x83292858..0x83292868)
	// 83292858: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8329285C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292860: 914B5E14  stw r10, 0x5e14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24084 as u32), ctx.r[10].u32 ) };
	// 83292864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292868 size=16
    let mut pc: u32 = 0x83292868;
    'dispatch: loop {
        match pc {
            0x83292868 => {
    //   block [0x83292868..0x83292878)
	// 83292868: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8329286C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292870: 914B5E18  stw r10, 0x5e18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24088 as u32), ctx.r[10].u32 ) };
	// 83292874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292878 size=16
    let mut pc: u32 = 0x83292878;
    'dispatch: loop {
        match pc {
            0x83292878 => {
    //   block [0x83292878..0x83292888)
	// 83292878: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8329287C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292880: 914B5E1C  stw r10, 0x5e1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24092 as u32), ctx.r[10].u32 ) };
	// 83292884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292888 size=16
    let mut pc: u32 = 0x83292888;
    'dispatch: loop {
        match pc {
            0x83292888 => {
    //   block [0x83292888..0x83292898)
	// 83292888: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8329288C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292890: 914B5E20  stw r10, 0x5e20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24096 as u32), ctx.r[10].u32 ) };
	// 83292894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292898 size=16
    let mut pc: u32 = 0x83292898;
    'dispatch: loop {
        match pc {
            0x83292898 => {
    //   block [0x83292898..0x832928A8)
	// 83292898: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8329289C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832928A0: 914B5E24  stw r10, 0x5e24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24100 as u32), ctx.r[10].u32 ) };
	// 832928A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832928A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832928A8 size=16
    let mut pc: u32 = 0x832928A8;
    'dispatch: loop {
        match pc {
            0x832928A8 => {
    //   block [0x832928A8..0x832928B8)
	// 832928A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832928AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832928B0: 914B5E28  stw r10, 0x5e28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24104 as u32), ctx.r[10].u32 ) };
	// 832928B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832928B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832928B8 size=16
    let mut pc: u32 = 0x832928B8;
    'dispatch: loop {
        match pc {
            0x832928B8 => {
    //   block [0x832928B8..0x832928C8)
	// 832928B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832928BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832928C0: 914B5E2C  stw r10, 0x5e2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24108 as u32), ctx.r[10].u32 ) };
	// 832928C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832928C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832928C8 size=16
    let mut pc: u32 = 0x832928C8;
    'dispatch: loop {
        match pc {
            0x832928C8 => {
    //   block [0x832928C8..0x832928D8)
	// 832928C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832928CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832928D0: 914B5E30  stw r10, 0x5e30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24112 as u32), ctx.r[10].u32 ) };
	// 832928D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832928D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832928D8 size=12
    let mut pc: u32 = 0x832928D8;
    'dispatch: loop {
        match pc {
            0x832928D8 => {
    //   block [0x832928D8..0x832928E4)
	// 832928D8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832928DC: 386B6610  addi r3, r11, 0x6610
	ctx.r[3].s64 = ctx.r[11].s64 + 26128;
	// 832928E0: 4BA17640  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832928E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832928E8 size=52
    let mut pc: u32 = 0x832928E8;
    'dispatch: loop {
        match pc {
            0x832928E8 => {
    //   block [0x832928E8..0x8329291C)
	// 832928E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832928EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832928F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832928F4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832928F8: 386B5E3C  addi r3, r11, 0x5e3c
	ctx.r[3].s64 = ctx.r[11].s64 + 24124;
	// 832928FC: 48027389  bl 0x832b9c84
	ctx.lr = 0x83292900;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 83292900: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292904: 386A6620  addi r3, r10, 0x6620
	ctx.r[3].s64 = ctx.r[10].s64 + 26144;
	// 83292908: 4BA17619  bl 0x82ca9f20
	ctx.lr = 0x8329290C;
	sub_82CA9F20(ctx, base);
	// 8329290C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292920 size=48
    let mut pc: u32 = 0x83292920;
    'dispatch: loop {
        match pc {
            0x83292920 => {
    //   block [0x83292920..0x83292950)
	// 83292920: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83292924: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83292928: 38E85E58  addi r7, r8, 0x5e58
	ctx.r[7].s64 = ctx.r[8].s64 + 24152;
	// 8329292C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83292930: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83292934: 99685E58  stb r11, 0x5e58(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(24152 as u32), ctx.r[11].u8 ) };
	// 83292938: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329293C: 99470001  stb r10, 1(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 83292940: 99270002  stb r9, 2(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(2 as u32), ctx.r[9].u8 ) };
	// 83292944: 99070003  stb r8, 3(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(3 as u32), ctx.r[8].u8 ) };
	// 83292948: 99670004  stb r11, 4(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 8329294C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292950 size=52
    let mut pc: u32 = 0x83292950;
    'dispatch: loop {
        match pc {
            0x83292950 => {
    //   block [0x83292950..0x83292984)
	// 83292950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329295C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292960: 386B5E60  addi r3, r11, 0x5e60
	ctx.r[3].s64 = ctx.r[11].s64 + 24160;
	// 83292964: 4B1ED8D5  bl 0x82480238
	ctx.lr = 0x83292968;
	sub_82480238(ctx, base);
	// 83292968: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329296C: 386A6628  addi r3, r10, 0x6628
	ctx.r[3].s64 = ctx.r[10].s64 + 26152;
	// 83292970: 4BA175B1  bl 0x82ca9f20
	ctx.lr = 0x83292974;
	sub_82CA9F20(ctx, base);
	// 83292974: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329297C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292988 size=52
    let mut pc: u32 = 0x83292988;
    'dispatch: loop {
        match pc {
            0x83292988 => {
    //   block [0x83292988..0x832929BC)
	// 83292988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329298C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292994: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292998: 386B5E6C  addi r3, r11, 0x5e6c
	ctx.r[3].s64 = ctx.r[11].s64 + 24172;
	// 8329299C: 4B1ED89D  bl 0x82480238
	ctx.lr = 0x832929A0;
	sub_82480238(ctx, base);
	// 832929A0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832929A4: 386A6638  addi r3, r10, 0x6638
	ctx.r[3].s64 = ctx.r[10].s64 + 26168;
	// 832929A8: 4BA17579  bl 0x82ca9f20
	ctx.lr = 0x832929AC;
	sub_82CA9F20(ctx, base);
	// 832929AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832929B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832929B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832929B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832929C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832929C0 size=64
    let mut pc: u32 = 0x832929C0;
    'dispatch: loop {
        match pc {
            0x832929C0 => {
    //   block [0x832929C0..0x83292A00)
	// 832929C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832929C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832929C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832929CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832929D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832929D4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832929D8: 386A5E78  addi r3, r10, 0x5e78
	ctx.r[3].s64 = ctx.r[10].s64 + 24184;
	// 832929DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832929E0: 4AF9A4F1  bl 0x8222ced0
	ctx.lr = 0x832929E4;
	sub_8222CED0(ctx, base);
	// 832929E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832929E8: 38696648  addi r3, r9, 0x6648
	ctx.r[3].s64 = ctx.r[9].s64 + 26184;
	// 832929EC: 4BA17535  bl 0x82ca9f20
	ctx.lr = 0x832929F0;
	sub_82CA9F20(ctx, base);
	// 832929F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832929F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832929F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832929FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292A00 size=60
    let mut pc: u32 = 0x83292A00;
    'dispatch: loop {
        match pc {
            0x83292A00 => {
    //   block [0x83292A00..0x83292A3C)
	// 83292A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292A0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292A10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83292A14: 388B0AF8  addi r4, r11, 0xaf8
	ctx.r[4].s64 = ctx.r[11].s64 + 2808;
	// 83292A18: 386A5E7C  addi r3, r10, 0x5e7c
	ctx.r[3].s64 = ctx.r[10].s64 + 24188;
	// 83292A1C: 4B0439ED  bl 0x822d6408
	ctx.lr = 0x83292A20;
	sub_822D6408(ctx, base);
	// 83292A20: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292A24: 38696658  addi r3, r9, 0x6658
	ctx.r[3].s64 = ctx.r[9].s64 + 26200;
	// 83292A28: 4BA174F9  bl 0x82ca9f20
	ctx.lr = 0x83292A2C;
	sub_82CA9F20(ctx, base);
	// 83292A2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292A40 size=48
    let mut pc: u32 = 0x83292A40;
    'dispatch: loop {
        match pc {
            0x83292A40 => {
    //   block [0x83292A40..0x83292A48)
	// 83292A40: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 83292A44: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x83292A48; continue 'dispatch;
            }
            0x83292A48 => {
    //   block [0x83292A48..0x83292A70)
	// 83292A48: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83292A4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83292A50: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83292A54: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83292A58: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83292A5C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83292A60: 4082FFE8  bne 0x83292a48
	if !ctx.cr[0].eq {
	pc = 0x83292A48; continue 'dispatch;
	}
	// 83292A64: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83292A68: 38676668  addi r3, r7, 0x6668
	ctx.r[3].s64 = ctx.r[7].s64 + 26216;
	// 83292A6C: 4BA174B4  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292A70 size=144
    let mut pc: u32 = 0x83292A70;
    'dispatch: loop {
        match pc {
            0x83292A70 => {
    //   block [0x83292A70..0x83292A94)
	// 83292A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292A7C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83292A80: 4AF8C7D9  bl 0x8221f258
	ctx.lr = 0x83292A84;
	sub_8221F258(ctx, base);
	// 83292A84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292A88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83292A8C: 419A0008  beq cr6, 0x83292a94
	if ctx.cr[6].eq {
	pc = 0x83292A94; continue 'dispatch;
	}
	// 83292A90: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292A94; continue 'dispatch;
            }
            0x83292A94 => {
    //   block [0x83292A94..0x83292AA0)
	// 83292A94: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292A98: 41820008  beq 0x83292aa0
	if ctx.cr[0].eq {
	pc = 0x83292AA0; continue 'dispatch;
	}
	// 83292A9C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292AA0; continue 'dispatch;
            }
            0x83292AA0 => {
    //   block [0x83292AA0..0x83292AAC)
	// 83292AA0: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292AA4: 41820008  beq 0x83292aac
	if ctx.cr[0].eq {
	pc = 0x83292AAC; continue 'dispatch;
	}
	// 83292AA8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292AAC; continue 'dispatch;
            }
            0x83292AAC => {
    //   block [0x83292AAC..0x83292B00)
	// 83292AAC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83292AB0: 9943000E  stb r10, 0xe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[10].u8 ) };
	// 83292AB4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83292AB8: 39095E84  addi r8, r9, 0x5e84
	ctx.r[8].s64 = ctx.r[9].s64 + 24196;
	// 83292ABC: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 83292AC0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83292AC4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83292AC8: 9963000E  stb r11, 0xe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[11].u8 ) };
	// 83292ACC: 38676678  addi r3, r7, 0x6678
	ctx.r[3].s64 = ctx.r[7].s64 + 26232;
	// 83292AD0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292AD4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83292AD8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292ADC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83292AE0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292AE4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83292AE8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83292AEC: 4BA17435  bl 0x82ca9f20
	ctx.lr = 0x83292AF0;
	sub_82CA9F20(ctx, base);
	// 83292AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292B00 size=16
    let mut pc: u32 = 0x83292B00;
    'dispatch: loop {
        match pc {
            0x83292B00 => {
    //   block [0x83292B00..0x83292B10)
	// 83292B00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292B04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292B08: 914B5E98  stw r10, 0x5e98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24216 as u32), ctx.r[10].u32 ) };
	// 83292B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292B10 size=152
    let mut pc: u32 = 0x83292B10;
    'dispatch: loop {
        match pc {
            0x83292B10 => {
    //   block [0x83292B10..0x83292BA8)
	// 83292B10: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83292B14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83292B18: 38C85EA8  addi r6, r8, 0x5ea8
	ctx.r[6].s64 = ctx.r[8].s64 + 24232;
	// 83292B1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292B20: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83292B24: 91685EA8  stw r11, 0x5ea8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24232 as u32), ctx.r[11].u32 ) };
	// 83292B28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83292B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83292B30: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83292B34: 91260008  stw r9, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83292B38: 9106000C  stw r8, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 83292B3C: 91660010  stw r11, 0x10(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83292B40: 90E60014  stw r7, 0x14(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 83292B44: 91460018  stw r10, 0x18(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83292B48: 9126001C  stw r9, 0x1c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 83292B4C: 91060020  stw r8, 0x20(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 83292B50: 91660024  stw r11, 0x24(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83292B54: 90E60028  stw r7, 0x28(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 83292B58: 9146002C  stw r10, 0x2c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 83292B5C: 91260030  stw r9, 0x30(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 83292B60: 91060034  stw r8, 0x34(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(52 as u32), ctx.r[8].u32 ) };
	// 83292B64: 91660038  stw r11, 0x38(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83292B68: 90E6003C  stw r7, 0x3c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(60 as u32), ctx.r[7].u32 ) };
	// 83292B6C: 91460040  stw r10, 0x40(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 83292B70: 91260044  stw r9, 0x44(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 83292B74: 91060048  stw r8, 0x48(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(72 as u32), ctx.r[8].u32 ) };
	// 83292B78: 9166004C  stw r11, 0x4c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83292B7C: 90E60050  stw r7, 0x50(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 83292B80: 91460054  stw r10, 0x54(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83292B84: 91260058  stw r9, 0x58(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 83292B88: 9106005C  stw r8, 0x5c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83292B8C: 91660060  stw r11, 0x60(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 83292B90: 90E60064  stw r7, 0x64(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 83292B94: 91460068  stw r10, 0x68(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 83292B98: 9126006C  stw r9, 0x6c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 83292B9C: 91060070  stw r8, 0x70(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 83292BA0: 91660074  stw r11, 0x74(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83292BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292BA8 size=144
    let mut pc: u32 = 0x83292BA8;
    'dispatch: loop {
        match pc {
            0x83292BA8 => {
    //   block [0x83292BA8..0x83292BCC)
	// 83292BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292BB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292BB4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83292BB8: 4AF8C6A1  bl 0x8221f258
	ctx.lr = 0x83292BBC;
	sub_8221F258(ctx, base);
	// 83292BBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292BC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83292BC4: 419A0008  beq cr6, 0x83292bcc
	if ctx.cr[6].eq {
	pc = 0x83292BCC; continue 'dispatch;
	}
	// 83292BC8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292BCC; continue 'dispatch;
            }
            0x83292BCC => {
    //   block [0x83292BCC..0x83292BD8)
	// 83292BCC: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292BD0: 41820008  beq 0x83292bd8
	if ctx.cr[0].eq {
	pc = 0x83292BD8; continue 'dispatch;
	}
	// 83292BD4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292BD8; continue 'dispatch;
            }
            0x83292BD8 => {
    //   block [0x83292BD8..0x83292BE4)
	// 83292BD8: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292BDC: 41820008  beq 0x83292be4
	if ctx.cr[0].eq {
	pc = 0x83292BE4; continue 'dispatch;
	}
	// 83292BE0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292BE4; continue 'dispatch;
            }
            0x83292BE4 => {
    //   block [0x83292BE4..0x83292C38)
	// 83292BE4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83292BE8: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 83292BEC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83292BF0: 39095E9C  addi r8, r9, 0x5e9c
	ctx.r[8].s64 = ctx.r[9].s64 + 24220;
	// 83292BF4: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83292BF8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83292BFC: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83292C00: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83292C04: 386766F8  addi r3, r7, 0x66f8
	ctx.r[3].s64 = ctx.r[7].s64 + 26360;
	// 83292C08: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292C0C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83292C10: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292C14: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83292C18: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292C1C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83292C20: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83292C24: 4BA172FD  bl 0x82ca9f20
	ctx.lr = 0x83292C28;
	sub_82CA9F20(ctx, base);
	// 83292C28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292C38 size=52
    let mut pc: u32 = 0x83292C38;
    'dispatch: loop {
        match pc {
            0x83292C38 => {
    //   block [0x83292C38..0x83292C6C)
	// 83292C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292C40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292C44: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292C48: 386B5F20  addi r3, r11, 0x5f20
	ctx.r[3].s64 = ctx.r[11].s64 + 24352;
	// 83292C4C: 4B0ADEC5  bl 0x82340b10
	ctx.lr = 0x83292C50;
	sub_82340B10(ctx, base);
	// 83292C50: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292C54: 386A6708  addi r3, r10, 0x6708
	ctx.r[3].s64 = ctx.r[10].s64 + 26376;
	// 83292C58: 4BA172C9  bl 0x82ca9f20
	ctx.lr = 0x83292C5C;
	sub_82CA9F20(ctx, base);
	// 83292C5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292C70 size=52
    let mut pc: u32 = 0x83292C70;
    'dispatch: loop {
        match pc {
            0x83292C70 => {
    //   block [0x83292C70..0x83292CA4)
	// 83292C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292C7C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292C80: 386B5F2C  addi r3, r11, 0x5f2c
	ctx.r[3].s64 = ctx.r[11].s64 + 24364;
	// 83292C84: 48027001  bl 0x832b9c84
	ctx.lr = 0x83292C88;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 83292C88: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292C8C: 386A6718  addi r3, r10, 0x6718
	ctx.r[3].s64 = ctx.r[10].s64 + 26392;
	// 83292C90: 4BA17291  bl 0x82ca9f20
	ctx.lr = 0x83292C94;
	sub_82CA9F20(ctx, base);
	// 83292C94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292CA8 size=144
    let mut pc: u32 = 0x83292CA8;
    'dispatch: loop {
        match pc {
            0x83292CA8 => {
    //   block [0x83292CA8..0x83292CCC)
	// 83292CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292CB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292CB4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83292CB8: 4AF8C5A1  bl 0x8221f258
	ctx.lr = 0x83292CBC;
	sub_8221F258(ctx, base);
	// 83292CBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292CC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83292CC4: 419A0008  beq cr6, 0x83292ccc
	if ctx.cr[6].eq {
	pc = 0x83292CCC; continue 'dispatch;
	}
	// 83292CC8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292CCC; continue 'dispatch;
            }
            0x83292CCC => {
    //   block [0x83292CCC..0x83292CD8)
	// 83292CCC: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292CD0: 41820008  beq 0x83292cd8
	if ctx.cr[0].eq {
	pc = 0x83292CD8; continue 'dispatch;
	}
	// 83292CD4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292CD8; continue 'dispatch;
            }
            0x83292CD8 => {
    //   block [0x83292CD8..0x83292CE4)
	// 83292CD8: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292CDC: 41820008  beq 0x83292ce4
	if ctx.cr[0].eq {
	pc = 0x83292CE4; continue 'dispatch;
	}
	// 83292CE0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292CE4; continue 'dispatch;
            }
            0x83292CE4 => {
    //   block [0x83292CE4..0x83292D38)
	// 83292CE4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83292CE8: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 83292CEC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83292CF0: 39095F48  addi r8, r9, 0x5f48
	ctx.r[8].s64 = ctx.r[9].s64 + 24392;
	// 83292CF4: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83292CF8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83292CFC: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83292D00: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83292D04: 38676720  addi r3, r7, 0x6720
	ctx.r[3].s64 = ctx.r[7].s64 + 26400;
	// 83292D08: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292D0C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83292D10: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292D14: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83292D18: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292D1C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83292D20: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83292D24: 4BA171FD  bl 0x82ca9f20
	ctx.lr = 0x83292D28;
	sub_82CA9F20(ctx, base);
	// 83292D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292D38 size=144
    let mut pc: u32 = 0x83292D38;
    'dispatch: loop {
        match pc {
            0x83292D38 => {
    //   block [0x83292D38..0x83292D5C)
	// 83292D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292D40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292D44: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83292D48: 4AF8C511  bl 0x8221f258
	ctx.lr = 0x83292D4C;
	sub_8221F258(ctx, base);
	// 83292D4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292D50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83292D54: 419A0008  beq cr6, 0x83292d5c
	if ctx.cr[6].eq {
	pc = 0x83292D5C; continue 'dispatch;
	}
	// 83292D58: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292D5C; continue 'dispatch;
            }
            0x83292D5C => {
    //   block [0x83292D5C..0x83292D68)
	// 83292D5C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292D60: 41820008  beq 0x83292d68
	if ctx.cr[0].eq {
	pc = 0x83292D68; continue 'dispatch;
	}
	// 83292D64: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292D68; continue 'dispatch;
            }
            0x83292D68 => {
    //   block [0x83292D68..0x83292D74)
	// 83292D68: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292D6C: 41820008  beq 0x83292d74
	if ctx.cr[0].eq {
	pc = 0x83292D74; continue 'dispatch;
	}
	// 83292D70: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292D74; continue 'dispatch;
            }
            0x83292D74 => {
    //   block [0x83292D74..0x83292DC8)
	// 83292D74: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83292D78: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 83292D7C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83292D80: 39095F54  addi r8, r9, 0x5f54
	ctx.r[8].s64 = ctx.r[9].s64 + 24404;
	// 83292D84: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83292D88: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83292D8C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83292D90: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83292D94: 38676730  addi r3, r7, 0x6730
	ctx.r[3].s64 = ctx.r[7].s64 + 26416;
	// 83292D98: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292D9C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83292DA0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292DA4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83292DA8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292DAC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83292DB0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83292DB4: 4BA1716D  bl 0x82ca9f20
	ctx.lr = 0x83292DB8;
	sub_82CA9F20(ctx, base);
	// 83292DB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292DC8 size=28
    let mut pc: u32 = 0x83292DC8;
    'dispatch: loop {
        match pc {
            0x83292DC8 => {
    //   block [0x83292DC8..0x83292DE4)
	// 83292DC8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83292DCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83292DD0: 392BB088  addi r9, r11, -0x4f78
	ctx.r[9].s64 = ctx.r[11].s64 + -20344;
	// 83292DD4: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83292DD8: 38686740  addi r3, r8, 0x6740
	ctx.r[3].s64 = ctx.r[8].s64 + 26432;
	// 83292DDC: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83292DE0: 4BA17140  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292DE8 size=144
    let mut pc: u32 = 0x83292DE8;
    'dispatch: loop {
        match pc {
            0x83292DE8 => {
    //   block [0x83292DE8..0x83292E0C)
	// 83292DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292DF4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83292DF8: 4AF8C461  bl 0x8221f258
	ctx.lr = 0x83292DFC;
	sub_8221F258(ctx, base);
	// 83292DFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292E00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83292E04: 419A0008  beq cr6, 0x83292e0c
	if ctx.cr[6].eq {
	pc = 0x83292E0C; continue 'dispatch;
	}
	// 83292E08: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292E0C; continue 'dispatch;
            }
            0x83292E0C => {
    //   block [0x83292E0C..0x83292E18)
	// 83292E0C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292E10: 41820008  beq 0x83292e18
	if ctx.cr[0].eq {
	pc = 0x83292E18; continue 'dispatch;
	}
	// 83292E14: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292E18; continue 'dispatch;
            }
            0x83292E18 => {
    //   block [0x83292E18..0x83292E24)
	// 83292E18: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292E1C: 41820008  beq 0x83292e24
	if ctx.cr[0].eq {
	pc = 0x83292E24; continue 'dispatch;
	}
	// 83292E20: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83292E24; continue 'dispatch;
            }
            0x83292E24 => {
    //   block [0x83292E24..0x83292E78)
	// 83292E24: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83292E28: 99430019  stb r10, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 83292E2C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83292E30: 39095F60  addi r8, r9, 0x5f60
	ctx.r[8].s64 = ctx.r[9].s64 + 24416;
	// 83292E34: 99630018  stb r11, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 83292E38: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83292E3C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83292E40: 99630019  stb r11, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 83292E44: 38676750  addi r3, r7, 0x6750
	ctx.r[3].s64 = ctx.r[7].s64 + 26448;
	// 83292E48: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292E4C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83292E50: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292E54: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83292E58: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292E5C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83292E60: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83292E64: 4BA170BD  bl 0x82ca9f20
	ctx.lr = 0x83292E68;
	sub_82CA9F20(ctx, base);
	// 83292E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292E78 size=48
    let mut pc: u32 = 0x83292E78;
    'dispatch: loop {
        match pc {
            0x83292E78 => {
    //   block [0x83292E78..0x83292E80)
	// 83292E78: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 83292E7C: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x83292E80; continue 'dispatch;
            }
            0x83292E80 => {
    //   block [0x83292E80..0x83292EA8)
	// 83292E80: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83292E84: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83292E88: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83292E8C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83292E90: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83292E94: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83292E98: 4082FFE8  bne 0x83292e80
	if !ctx.cr[0].eq {
	pc = 0x83292E80; continue 'dispatch;
	}
	// 83292E9C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83292EA0: 38676760  addi r3, r7, 0x6760
	ctx.r[3].s64 = ctx.r[7].s64 + 26464;
	// 83292EA4: 4BA1707C  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292EA8 size=12
    let mut pc: u32 = 0x83292EA8;
    'dispatch: loop {
        match pc {
            0x83292EA8 => {
    //   block [0x83292EA8..0x83292EB4)
	// 83292EA8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83292EAC: 386B6770  addi r3, r11, 0x6770
	ctx.r[3].s64 = ctx.r[11].s64 + 26480;
	// 83292EB0: 4BA17070  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292EB8 size=12
    let mut pc: u32 = 0x83292EB8;
    'dispatch: loop {
        match pc {
            0x83292EB8 => {
    //   block [0x83292EB8..0x83292EC4)
	// 83292EB8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83292EBC: 386B67D0  addi r3, r11, 0x67d0
	ctx.r[3].s64 = ctx.r[11].s64 + 26576;
	// 83292EC0: 4BA17060  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292EC8 size=12
    let mut pc: u32 = 0x83292EC8;
    'dispatch: loop {
        match pc {
            0x83292EC8 => {
    //   block [0x83292EC8..0x83292ED4)
	// 83292EC8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83292ECC: 386B6830  addi r3, r11, 0x6830
	ctx.r[3].s64 = ctx.r[11].s64 + 26672;
	// 83292ED0: 4BA17050  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292ED8 size=12
    let mut pc: u32 = 0x83292ED8;
    'dispatch: loop {
        match pc {
            0x83292ED8 => {
    //   block [0x83292ED8..0x83292EE4)
	// 83292ED8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83292EDC: 386B6890  addi r3, r11, 0x6890
	ctx.r[3].s64 = ctx.r[11].s64 + 26768;
	// 83292EE0: 4BA17040  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292EE8 size=52
    let mut pc: u32 = 0x83292EE8;
    'dispatch: loop {
        match pc {
            0x83292EE8 => {
    //   block [0x83292EE8..0x83292F1C)
	// 83292EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292EF4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292EF8: 386B5F7C  addi r3, r11, 0x5f7c
	ctx.r[3].s64 = ctx.r[11].s64 + 24444;
	// 83292EFC: 48026D89  bl 0x832b9c84
	ctx.lr = 0x83292F00;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 83292F00: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292F04: 386A68A0  addi r3, r10, 0x68a0
	ctx.r[3].s64 = ctx.r[10].s64 + 26784;
	// 83292F08: 4BA17019  bl 0x82ca9f20
	ctx.lr = 0x83292F0C;
	sub_82CA9F20(ctx, base);
	// 83292F0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83292F20 size=12
    let mut pc: u32 = 0x83292F20;
    'dispatch: loop {
        match pc {
            0x83292F20 => {
    //   block [0x83292F20..0x83292F2C)
	// 83292F20: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83292F24: 386B68A8  addi r3, r11, 0x68a8
	ctx.r[3].s64 = ctx.r[11].s64 + 26792;
	// 83292F28: 4BA16FF8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292F30 size=52
    let mut pc: u32 = 0x83292F30;
    'dispatch: loop {
        match pc {
            0x83292F30 => {
    //   block [0x83292F30..0x83292F64)
	// 83292F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292F3C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292F40: 386B5F98  addi r3, r11, 0x5f98
	ctx.r[3].s64 = ctx.r[11].s64 + 24472;
	// 83292F44: 4B56C6F5  bl 0x827ff638
	ctx.lr = 0x83292F48;
	sub_827FF638(ctx, base);
	// 83292F48: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292F4C: 386A68B8  addi r3, r10, 0x68b8
	ctx.r[3].s64 = ctx.r[10].s64 + 26808;
	// 83292F50: 4BA16FD1  bl 0x82ca9f20
	ctx.lr = 0x83292F54;
	sub_82CA9F20(ctx, base);
	// 83292F54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292F68 size=52
    let mut pc: u32 = 0x83292F68;
    'dispatch: loop {
        match pc {
            0x83292F68 => {
    //   block [0x83292F68..0x83292F9C)
	// 83292F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292F70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292F74: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292F78: 386B5FA4  addi r3, r11, 0x5fa4
	ctx.r[3].s64 = ctx.r[11].s64 + 24484;
	// 83292F7C: 4B56C6BD  bl 0x827ff638
	ctx.lr = 0x83292F80;
	sub_827FF638(ctx, base);
	// 83292F80: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292F84: 386A68C8  addi r3, r10, 0x68c8
	ctx.r[3].s64 = ctx.r[10].s64 + 26824;
	// 83292F88: 4BA16F99  bl 0x82ca9f20
	ctx.lr = 0x83292F8C;
	sub_82CA9F20(ctx, base);
	// 83292F8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292FA0 size=52
    let mut pc: u32 = 0x83292FA0;
    'dispatch: loop {
        match pc {
            0x83292FA0 => {
    //   block [0x83292FA0..0x83292FD4)
	// 83292FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292FAC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292FB0: 386B5FB0  addi r3, r11, 0x5fb0
	ctx.r[3].s64 = ctx.r[11].s64 + 24496;
	// 83292FB4: 4B56C685  bl 0x827ff638
	ctx.lr = 0x83292FB8;
	sub_827FF638(ctx, base);
	// 83292FB8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292FBC: 386A68D8  addi r3, r10, 0x68d8
	ctx.r[3].s64 = ctx.r[10].s64 + 26840;
	// 83292FC0: 4BA16F61  bl 0x82ca9f20
	ctx.lr = 0x83292FC4;
	sub_82CA9F20(ctx, base);
	// 83292FC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292FD8 size=52
    let mut pc: u32 = 0x83292FD8;
    'dispatch: loop {
        match pc {
            0x83292FD8 => {
    //   block [0x83292FD8..0x8329300C)
	// 83292FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292FE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292FE4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292FE8: 386B5FBC  addi r3, r11, 0x5fbc
	ctx.r[3].s64 = ctx.r[11].s64 + 24508;
	// 83292FEC: 4B56C64D  bl 0x827ff638
	ctx.lr = 0x83292FF0;
	sub_827FF638(ctx, base);
	// 83292FF0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292FF4: 386A68E8  addi r3, r10, 0x68e8
	ctx.r[3].s64 = ctx.r[10].s64 + 26856;
	// 83292FF8: 4BA16F29  bl 0x82ca9f20
	ctx.lr = 0x83292FFC;
	sub_82CA9F20(ctx, base);
	// 83292FFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293010 size=108
    let mut pc: u32 = 0x83293010;
    'dispatch: loop {
        match pc {
            0x83293010 => {
    //   block [0x83293010..0x83293044)
	// 83293010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8329301C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293020: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83293024: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83293028: 3BEBB0E8  addi r31, r11, -0x4f18
	ctx.r[31].s64 = ctx.r[11].s64 + -20248;
	// 8329302C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83293030: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83293034: 4AF8C225  bl 0x8221f258
	ctx.lr = 0x83293038;
	sub_8221F258(ctx, base);
	// 83293038: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8329303C: 419A0008  beq cr6, 0x83293044
	if ctx.cr[6].eq {
	pc = 0x83293044; continue 'dispatch;
	}
	// 83293040: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	pc = 0x83293044; continue 'dispatch;
            }
            0x83293044 => {
    //   block [0x83293044..0x83293050)
	// 83293044: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83293048: 41820008  beq 0x83293050
	if ctx.cr[0].eq {
	pc = 0x83293050; continue 'dispatch;
	}
	// 8329304C: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	pc = 0x83293050; continue 'dispatch;
            }
            0x83293050 => {
    //   block [0x83293050..0x8329307C)
	// 83293050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83293054: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83293058: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329305C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83293060: 386A68F8  addi r3, r10, 0x68f8
	ctx.r[3].s64 = ctx.r[10].s64 + 26872;
	// 83293064: 4BA16EBD  bl 0x82ca9f20
	ctx.lr = 0x83293068;
	sub_82CA9F20(ctx, base);
	// 83293068: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329306C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293074: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83293078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293080 size=36
    let mut pc: u32 = 0x83293080;
    'dispatch: loop {
        match pc {
            0x83293080 => {
    //   block [0x83293080..0x832930A4)
	// 83293080: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83293084: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83293088: 38E85FC8  addi r7, r8, 0x5fc8
	ctx.r[7].s64 = ctx.r[8].s64 + 24520;
	// 8329308C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83293090: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83293094: 91685FC8  stw r11, 0x5fc8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24520 as u32), ctx.r[11].u32 ) };
	// 83293098: 91470004  stw r10, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329309C: 91270008  stw r9, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832930A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832930A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832930A8 size=36
    let mut pc: u32 = 0x832930A8;
    'dispatch: loop {
        match pc {
            0x832930A8 => {
    //   block [0x832930A8..0x832930CC)
	// 832930A8: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 832930AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832930B0: 38E85FD8  addi r7, r8, 0x5fd8
	ctx.r[7].s64 = ctx.r[8].s64 + 24536;
	// 832930B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832930B8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832930BC: 91685FD8  stw r11, 0x5fd8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24536 as u32), ctx.r[11].u32 ) };
	// 832930C0: 91470004  stw r10, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832930C4: 91270008  stw r9, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832930C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832930D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832930D0 size=52
    let mut pc: u32 = 0x832930D0;
    'dispatch: loop {
        match pc {
            0x832930D0 => {
    //   block [0x832930D0..0x83293104)
	// 832930D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832930D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832930D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832930DC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832930E0: 386B5FF0  addi r3, r11, 0x5ff0
	ctx.r[3].s64 = ctx.r[11].s64 + 24560;
	// 832930E4: 48026BA1  bl 0x832b9c84
	ctx.lr = 0x832930E8;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 832930E8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832930EC: 386A6950  addi r3, r10, 0x6950
	ctx.r[3].s64 = ctx.r[10].s64 + 26960;
	// 832930F0: 4BA16E31  bl 0x82ca9f20
	ctx.lr = 0x832930F4;
	sub_82CA9F20(ctx, base);
	// 832930F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832930F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832930FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293108 size=52
    let mut pc: u32 = 0x83293108;
    'dispatch: loop {
        match pc {
            0x83293108 => {
    //   block [0x83293108..0x8329313C)
	// 83293108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329310C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293114: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83293118: 386B6010  addi r3, r11, 0x6010
	ctx.r[3].s64 = ctx.r[11].s64 + 24592;
	// 8329311C: 48026B69  bl 0x832b9c84
	ctx.lr = 0x83293120;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 83293120: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83293124: 386A6958  addi r3, r10, 0x6958
	ctx.r[3].s64 = ctx.r[10].s64 + 26968;
	// 83293128: 4BA16DF9  bl 0x82ca9f20
	ctx.lr = 0x8329312C;
	sub_82CA9F20(ctx, base);
	// 8329312C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293140 size=96
    let mut pc: u32 = 0x83293140;
    'dispatch: loop {
        match pc {
            0x83293140 => {
    //   block [0x83293140..0x832931A0)
	// 83293140: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83293144: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83293148: 390A1D98  addi r8, r10, 0x1d98
	ctx.r[8].s64 = ctx.r[10].s64 + 7576;
	// 8329314C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83293150: 3CE08349  lis r7, -0x7cb7
	ctx.r[7].s64 = -2092367872;
	// 83293154: 3CC08349  lis r6, -0x7cb7
	ctx.r[6].s64 = -2092367872;
	// 83293158: 91697124  stw r11, 0x7124(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(28964 as u32), ctx.r[11].u32 ) };
	// 8329315C: 3CA08349  lis r5, -0x7cb7
	ctx.r[5].s64 = -2092367872;
	// 83293160: F96A1D98  std r11, 0x1d98(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(7576 as u32), ctx.r[11].u64 ) };
	// 83293164: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83293168: F9680008  std r11, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8329316C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 83293170: F9680018  std r11, 0x18(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 83293174: 3C80832B  lis r4, -0x7cd5
	ctx.r[4].s64 = -2094333952;
	// 83293178: F9680020  std r11, 0x20(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 8329317C: 91680010  stw r11, 0x10(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83293180: 38646960  addi r3, r4, 0x6960
	ctx.r[3].s64 = ctx.r[4].s64 + 26976;
	// 83293184: 91680014  stw r11, 0x14(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83293188: 91680028  stw r11, 0x28(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8329318C: 9168002C  stw r11, 0x2c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83293190: 91477120  stw r10, 0x7120(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(28960 as u32), ctx.r[10].u32 ) };
	// 83293194: F9261DD0  std r9, 0x1dd0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(7632 as u32), ctx.r[9].u64 ) };
	// 83293198: F9651DC8  std r11, 0x1dc8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(7624 as u32), ctx.r[11].u64 ) };
	// 8329319C: 4BA16D84  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832931A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832931A0 size=52
    let mut pc: u32 = 0x832931A0;
    'dispatch: loop {
        match pc {
            0x832931A0 => {
    //   block [0x832931A0..0x832931D4)
	// 832931A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832931A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832931A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832931AC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832931B0: 386B6040  addi r3, r11, 0x6040
	ctx.r[3].s64 = ctx.r[11].s64 + 24640;
	// 832931B4: 4B78578D  bl 0x82a18940
	ctx.lr = 0x832931B8;
	sub_82A18940(ctx, base);
	// 832931B8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832931BC: 386A6968  addi r3, r10, 0x6968
	ctx.r[3].s64 = ctx.r[10].s64 + 26984;
	// 832931C0: 4BA16D61  bl 0x82ca9f20
	ctx.lr = 0x832931C4;
	sub_82CA9F20(ctx, base);
	// 832931C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832931C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832931CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832931D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832931D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832931D8 size=52
    let mut pc: u32 = 0x832931D8;
    'dispatch: loop {
        match pc {
            0x832931D8 => {
    //   block [0x832931D8..0x8329320C)
	// 832931D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832931DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832931E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832931E4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832931E8: 386B604C  addi r3, r11, 0x604c
	ctx.r[3].s64 = ctx.r[11].s64 + 24652;
	// 832931EC: 4B785755  bl 0x82a18940
	ctx.lr = 0x832931F0;
	sub_82A18940(ctx, base);
	// 832931F0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832931F4: 386A6978  addi r3, r10, 0x6978
	ctx.r[3].s64 = ctx.r[10].s64 + 27000;
	// 832931F8: 4BA16D29  bl 0x82ca9f20
	ctx.lr = 0x832931FC;
	sub_82CA9F20(ctx, base);
	// 832931FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293210 size=144
    let mut pc: u32 = 0x83293210;
    'dispatch: loop {
        match pc {
            0x83293210 => {
    //   block [0x83293210..0x83293234)
	// 83293210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329321C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 83293220: 4AF8C039  bl 0x8221f258
	ctx.lr = 0x83293224;
	sub_8221F258(ctx, base);
	// 83293224: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83293228: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8329322C: 419A0008  beq cr6, 0x83293234
	if ctx.cr[6].eq {
	pc = 0x83293234; continue 'dispatch;
	}
	// 83293230: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83293234; continue 'dispatch;
            }
            0x83293234 => {
    //   block [0x83293234..0x83293240)
	// 83293234: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83293238: 41820008  beq 0x83293240
	if ctx.cr[0].eq {
	pc = 0x83293240; continue 'dispatch;
	}
	// 8329323C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83293240; continue 'dispatch;
            }
            0x83293240 => {
    //   block [0x83293240..0x8329324C)
	// 83293240: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83293244: 41820008  beq 0x8329324c
	if ctx.cr[0].eq {
	pc = 0x8329324C; continue 'dispatch;
	}
	// 83293248: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8329324C; continue 'dispatch;
            }
            0x8329324C => {
    //   block [0x8329324C..0x832932A0)
	// 8329324C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83293250: 99430011  stb r10, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[10].u8 ) };
	// 83293254: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83293258: 39096058  addi r8, r9, 0x6058
	ctx.r[8].s64 = ctx.r[9].s64 + 24664;
	// 8329325C: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 83293260: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83293264: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83293268: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 8329326C: 38676988  addi r3, r7, 0x6988
	ctx.r[3].s64 = ctx.r[7].s64 + 27016;
	// 83293270: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83293274: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83293278: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8329327C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83293280: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83293284: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83293288: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329328C: 4BA16C95  bl 0x82ca9f20
	ctx.lr = 0x83293290;
	sub_82CA9F20(ctx, base);
	// 83293290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329329C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832932A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832932A0 size=52
    let mut pc: u32 = 0x832932A0;
    'dispatch: loop {
        match pc {
            0x832932A0 => {
    //   block [0x832932A0..0x832932D4)
	// 832932A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832932A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832932A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832932AC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832932B0: 386B606C  addi r3, r11, 0x606c
	ctx.r[3].s64 = ctx.r[11].s64 + 24684;
	// 832932B4: 480269D1  bl 0x832b9c84
	ctx.lr = 0x832932B8;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 832932B8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832932BC: 386A6998  addi r3, r10, 0x6998
	ctx.r[3].s64 = ctx.r[10].s64 + 27032;
	// 832932C0: 4BA16C61  bl 0x82ca9f20
	ctx.lr = 0x832932C4;
	sub_82CA9F20(ctx, base);
	// 832932C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832932C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832932CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832932D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832932D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832932D8 size=100
    let mut pc: u32 = 0x832932D8;
    'dispatch: loop {
        match pc {
            0x832932D8 => {
    //   block [0x832932D8..0x8329333C)
	// 832932D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832932DC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832932E0: 38CA6088  addi r6, r10, 0x6088
	ctx.r[6].s64 = ctx.r[10].s64 + 24712;
	// 832932E4: 38EB2A2C  addi r7, r11, 0x2a2c
	ctx.r[7].s64 = ctx.r[11].s64 + 10796;
	// 832932E8: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 832932EC: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 832932F0: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 832932F4: 916A6088  stw r11, 0x6088(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24712 as u32), ctx.r[11].u32 ) };
	// 832932F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832932FC: 9126000C  stw r9, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 83293300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83293304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83293308: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329330C: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 83293310: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83293314: 91460010  stw r10, 0x10(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83293318: 386569A0  addi r3, r5, 0x69a0
	ctx.r[3].s64 = ctx.r[5].s64 + 27040;
	// 8329331C: 91260014  stw r9, 0x14(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 83293320: 9166001C  stw r11, 0x1c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83293324: 91060018  stw r8, 0x18(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 83293328: 91460020  stw r10, 0x20(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8329332C: 90E60024  stw r7, 0x24(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 83293330: 91260028  stw r9, 0x28(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 83293334: 9166002C  stw r11, 0x2c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83293338: 4BA16BE8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293340 size=12
    let mut pc: u32 = 0x83293340;
    'dispatch: loop {
        match pc {
            0x83293340 => {
    //   block [0x83293340..0x8329334C)
	// 83293340: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83293344: 386B69F8  addi r3, r11, 0x69f8
	ctx.r[3].s64 = ctx.r[11].s64 + 27128;
	// 83293348: 4BA16BD8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293350 size=108
    let mut pc: u32 = 0x83293350;
    'dispatch: loop {
        match pc {
            0x83293350 => {
    //   block [0x83293350..0x83293378)
	// 83293350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293354: 4BA160B5  bl 0x82ca9408
	ctx.lr = 0x83293358;
	sub_82CA93D0(ctx, base);
	// 83293358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329335C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83293360: 3BC00007  li r30, 7
	ctx.r[30].s64 = 7;
	// 83293364: 396B60C8  addi r11, r11, 0x60c8
	ctx.r[11].s64 = ctx.r[11].s64 + 24776;
	// 83293368: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8329336C: 3BEB0038  addi r31, r11, 0x38
	ctx.r[31].s64 = ctx.r[11].s64 + 56;
	// 83293370: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83293374: 3B8B2A2C  addi r28, r11, 0x2a2c
	ctx.r[28].s64 = ctx.r[11].s64 + 10796;
	pc = 0x83293378; continue 'dispatch;
            }
            0x83293378 => {
    //   block [0x83293378..0x832933BC)
	// 83293378: 3D6082A5  lis r11, -0x7d5b
	ctx.r[11].s64 = -2103115776;
	// 8329337C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83293380: 38CB0668  addi r6, r11, 0x668
	ctx.r[6].s64 = ctx.r[11].s64 + 1640;
	// 83293384: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 83293388: 387FFFC8  addi r3, r31, -0x38
	ctx.r[3].s64 = ctx.r[31].s64 + -56;
	// 8329338C: 4AF8C77D  bl 0x8221fb08
	ctx.lr = 0x83293390;
	sub_8221FB08(ctx, base);
	// 83293390: 939FFFF8  stw r28, -8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), ctx.r[28].u32 ) };
	// 83293394: 93BFFFFC  stw r29, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[29].u32 ) };
	// 83293398: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8329339C: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832933A0: 3BFF004C  addi r31, r31, 0x4c
	ctx.r[31].s64 = ctx.r[31].s64 + 76;
	// 832933A4: 4080FFD4  bge 0x83293378
	if !ctx.cr[0].lt {
	pc = 0x83293378; continue 'dispatch;
	}
	// 832933A8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832933AC: 386B6A50  addi r3, r11, 0x6a50
	ctx.r[3].s64 = ctx.r[11].s64 + 27216;
	// 832933B0: 4BA16B71  bl 0x82ca9f20
	ctx.lr = 0x832933B4;
	sub_82CA9F20(ctx, base);
	// 832933B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832933B8: 4BA160A0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832933C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832933C0 size=12
    let mut pc: u32 = 0x832933C0;
    'dispatch: loop {
        match pc {
            0x832933C0 => {
    //   block [0x832933C0..0x832933CC)
	// 832933C0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832933C4: 386B6AA0  addi r3, r11, 0x6aa0
	ctx.r[3].s64 = ctx.r[11].s64 + 27296;
	// 832933C8: 4BA16B58  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832933D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832933D0 size=12
    let mut pc: u32 = 0x832933D0;
    'dispatch: loop {
        match pc {
            0x832933D0 => {
    //   block [0x832933D0..0x832933DC)
	// 832933D0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832933D4: 386B6B00  addi r3, r11, 0x6b00
	ctx.r[3].s64 = ctx.r[11].s64 + 27392;
	// 832933D8: 4BA16B48  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832933E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832933E0 size=12
    let mut pc: u32 = 0x832933E0;
    'dispatch: loop {
        match pc {
            0x832933E0 => {
    //   block [0x832933E0..0x832933EC)
	// 832933E0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832933E4: 386B6B58  addi r3, r11, 0x6b58
	ctx.r[3].s64 = ctx.r[11].s64 + 27480;
	// 832933E8: 4BA16B38  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832933F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832933F0 size=12
    let mut pc: u32 = 0x832933F0;
    'dispatch: loop {
        match pc {
            0x832933F0 => {
    //   block [0x832933F0..0x832933FC)
	// 832933F0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832933F4: 386B6BB0  addi r3, r11, 0x6bb0
	ctx.r[3].s64 = ctx.r[11].s64 + 27568;
	// 832933F8: 4BA16B28  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293400 size=144
    let mut pc: u32 = 0x83293400;
    'dispatch: loop {
        match pc {
            0x83293400 => {
    //   block [0x83293400..0x83293424)
	// 83293400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329340C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83293410: 4AF8BE49  bl 0x8221f258
	ctx.lr = 0x83293414;
	sub_8221F258(ctx, base);
	// 83293414: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83293418: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8329341C: 419A0008  beq cr6, 0x83293424
	if ctx.cr[6].eq {
	pc = 0x83293424; continue 'dispatch;
	}
	// 83293420: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83293424; continue 'dispatch;
            }
            0x83293424 => {
    //   block [0x83293424..0x83293430)
	// 83293424: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83293428: 41820008  beq 0x83293430
	if ctx.cr[0].eq {
	pc = 0x83293430; continue 'dispatch;
	}
	// 8329342C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83293430; continue 'dispatch;
            }
            0x83293430 => {
    //   block [0x83293430..0x8329343C)
	// 83293430: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83293434: 41820008  beq 0x8329343c
	if ctx.cr[0].eq {
	pc = 0x8329343C; continue 'dispatch;
	}
	// 83293438: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8329343C; continue 'dispatch;
            }
            0x8329343C => {
    //   block [0x8329343C..0x83293490)
	// 8329343C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83293440: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 83293444: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83293448: 3909632C  addi r8, r9, 0x632c
	ctx.r[8].s64 = ctx.r[9].s64 + 25388;
	// 8329344C: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83293450: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83293454: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83293458: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 8329345C: 38676BC0  addi r3, r7, 0x6bc0
	ctx.r[3].s64 = ctx.r[7].s64 + 27584;
	// 83293460: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83293464: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83293468: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8329346C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83293470: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83293474: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83293478: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329347C: 4BA16AA5  bl 0x82ca9f20
	ctx.lr = 0x83293480;
	sub_82CA9F20(ctx, base);
	// 83293480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329348C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293490 size=52
    let mut pc: u32 = 0x83293490;
    'dispatch: loop {
        match pc {
            0x83293490 => {
    //   block [0x83293490..0x832934C4)
	// 83293490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329349C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832934A0: 386B6338  addi r3, r11, 0x6338
	ctx.r[3].s64 = ctx.r[11].s64 + 25400;
	// 832934A4: 480267E1  bl 0x832b9c84
	ctx.lr = 0x832934A8;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 832934A8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832934AC: 386A6BD0  addi r3, r10, 0x6bd0
	ctx.r[3].s64 = ctx.r[10].s64 + 27600;
	// 832934B0: 4BA16A71  bl 0x82ca9f20
	ctx.lr = 0x832934B4;
	sub_82CA9F20(ctx, base);
	// 832934B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832934B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832934BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832934C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832934C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832934C8 size=88
    let mut pc: u32 = 0x832934C8;
    'dispatch: loop {
        match pc {
            0x832934C8 => {
    //   block [0x832934C8..0x83293520)
	// 832934C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832934CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832934D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832934D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832934D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832934DC: 3BEB6354  addi r31, r11, 0x6354
	ctx.r[31].s64 = ctx.r[11].s64 + 25428;
	// 832934E0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832934E4: 480267A1  bl 0x832b9c84
	ctx.lr = 0x832934E8;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 832934E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832934EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832934F0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832934F4: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 832934F8: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 832934FC: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83293500: 913F0034  stw r9, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[9].u32 ) };
	// 83293504: 386A6BD8  addi r3, r10, 0x6bd8
	ctx.r[3].s64 = ctx.r[10].s64 + 27608;
	// 83293508: 4BA16A19  bl 0x82ca9f20
	ctx.lr = 0x8329350C;
	sub_82CA9F20(ctx, base);
	// 8329350C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293518: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8329351C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293520 size=44
    let mut pc: u32 = 0x83293520;
    'dispatch: loop {
        match pc {
            0x83293520 => {
    //   block [0x83293520..0x8329354C)
	// 83293520: 3D40834B  lis r10, -0x7cb5
	ctx.r[10].s64 = -2092236800;
	// 83293524: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83293528: 38EA2390  addi r7, r10, 0x2390
	ctx.r[7].s64 = ctx.r[10].s64 + 9104;
	// 8329352C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83293530: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83293534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83293538: 91672000  stw r11, 0x2000(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8192 as u32), ctx.r[11].u32 ) };
	// 8329353C: 91472004  stw r10, 0x2004(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8196 as u32), ctx.r[10].u32 ) };
	// 83293540: 91274008  stw r9, 0x4008(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16392 as u32), ctx.r[9].u32 ) };
	// 83293544: 9107400C  stw r8, 0x400c(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16396 as u32), ctx.r[8].u32 ) };
	// 83293548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293550 size=12
    let mut pc: u32 = 0x83293550;
    'dispatch: loop {
        match pc {
            0x83293550 => {
    //   block [0x83293550..0x8329355C)
	// 83293550: 3D60834B  lis r11, -0x7cb5
	ctx.r[11].s64 = -2092236800;
	// 83293554: 386B63A0  addi r3, r11, 0x63a0
	ctx.r[3].s64 = ctx.r[11].s64 + 25504;
	// 83293558: 4B8E1740  b 0x82b74c98
	sub_82B74C98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293560 size=52
    let mut pc: u32 = 0x83293560;
    'dispatch: loop {
        match pc {
            0x83293560 => {
    //   block [0x83293560..0x83293594)
	// 83293560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329356C: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 83293570: 386BDE90  addi r3, r11, -0x2170
	ctx.r[3].s64 = ctx.r[11].s64 + -8560;
	// 83293574: 4B1ECCC5  bl 0x82480238
	ctx.lr = 0x83293578;
	sub_82480238(ctx, base);
	// 83293578: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329357C: 386A6BE8  addi r3, r10, 0x6be8
	ctx.r[3].s64 = ctx.r[10].s64 + 27624;
	// 83293580: 4BA169A1  bl 0x82ca9f20
	ctx.lr = 0x83293584;
	sub_82CA9F20(ctx, base);
	// 83293584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329358C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293598 size=12
    let mut pc: u32 = 0x83293598;
    'dispatch: loop {
        match pc {
            0x83293598 => {
    //   block [0x83293598..0x832935A4)
	// 83293598: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329359C: 386B6BF8  addi r3, r11, 0x6bf8
	ctx.r[3].s64 = ctx.r[11].s64 + 27640;
	// 832935A0: 4BA16980  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832935A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832935A8 size=52
    let mut pc: u32 = 0x832935A8;
    'dispatch: loop {
        match pc {
            0x832935A8 => {
    //   block [0x832935A8..0x832935DC)
	// 832935A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832935AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832935B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832935B4: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832935B8: 386BDEAC  addi r3, r11, -0x2154
	ctx.r[3].s64 = ctx.r[11].s64 + -8532;
	// 832935BC: 4B776FF5  bl 0x82a0a5b0
	ctx.lr = 0x832935C0;
	sub_82A0A5B0(ctx, base);
	// 832935C0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832935C4: 386A6C08  addi r3, r10, 0x6c08
	ctx.r[3].s64 = ctx.r[10].s64 + 27656;
	// 832935C8: 4BA16959  bl 0x82ca9f20
	ctx.lr = 0x832935CC;
	sub_82CA9F20(ctx, base);
	// 832935CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832935D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832935D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832935D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832935E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832935E0 size=52
    let mut pc: u32 = 0x832935E0;
    'dispatch: loop {
        match pc {
            0x832935E0 => {
    //   block [0x832935E0..0x83293614)
	// 832935E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832935E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832935E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832935EC: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832935F0: 386BDEB8  addi r3, r11, -0x2148
	ctx.r[3].s64 = ctx.r[11].s64 + -8520;
	// 832935F4: 4B776FBD  bl 0x82a0a5b0
	ctx.lr = 0x832935F8;
	sub_82A0A5B0(ctx, base);
	// 832935F8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832935FC: 386A6C18  addi r3, r10, 0x6c18
	ctx.r[3].s64 = ctx.r[10].s64 + 27672;
	// 83293600: 4BA16921  bl 0x82ca9f20
	ctx.lr = 0x83293604;
	sub_82CA9F20(ctx, base);
	// 83293604: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329360C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293618 size=12
    let mut pc: u32 = 0x83293618;
    'dispatch: loop {
        match pc {
            0x83293618 => {
    //   block [0x83293618..0x83293624)
	// 83293618: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329361C: 386B6C28  addi r3, r11, 0x6c28
	ctx.r[3].s64 = ctx.r[11].s64 + 27688;
	// 83293620: 4BA16900  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293628 size=12
    let mut pc: u32 = 0x83293628;
    'dispatch: loop {
        match pc {
            0x83293628 => {
    //   block [0x83293628..0x83293634)
	// 83293628: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329362C: 386B6C38  addi r3, r11, 0x6c38
	ctx.r[3].s64 = ctx.r[11].s64 + 27704;
	// 83293630: 4BA168F0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293638 size=48
    let mut pc: u32 = 0x83293638;
    'dispatch: loop {
        match pc {
            0x83293638 => {
    //   block [0x83293638..0x83293640)
	// 83293638: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8329363C: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x83293640; continue 'dispatch;
            }
            0x83293640 => {
    //   block [0x83293640..0x83293668)
	// 83293640: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83293644: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83293648: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8329364C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83293650: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83293654: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83293658: 4082FFE8  bne 0x83293640
	if !ctx.cr[0].eq {
	pc = 0x83293640; continue 'dispatch;
	}
	// 8329365C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83293660: 38676C90  addi r3, r7, 0x6c90
	ctx.r[3].s64 = ctx.r[7].s64 + 27792;
	// 83293664: 4BA168BC  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293668 size=12
    let mut pc: u32 = 0x83293668;
    'dispatch: loop {
        match pc {
            0x83293668 => {
    //   block [0x83293668..0x83293674)
	// 83293668: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329366C: 386B6CA0  addi r3, r11, 0x6ca0
	ctx.r[3].s64 = ctx.r[11].s64 + 27808;
	// 83293670: 4BA168B0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293678 size=12
    let mut pc: u32 = 0x83293678;
    'dispatch: loop {
        match pc {
            0x83293678 => {
    //   block [0x83293678..0x83293684)
	// 83293678: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329367C: 386B6CF8  addi r3, r11, 0x6cf8
	ctx.r[3].s64 = ctx.r[11].s64 + 27896;
	// 83293680: 4BA168A0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293688 size=12
    let mut pc: u32 = 0x83293688;
    'dispatch: loop {
        match pc {
            0x83293688 => {
    //   block [0x83293688..0x83293694)
	// 83293688: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329368C: 386B6D38  addi r3, r11, 0x6d38
	ctx.r[3].s64 = ctx.r[11].s64 + 27960;
	// 83293690: 4BA16890  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293698 size=12
    let mut pc: u32 = 0x83293698;
    'dispatch: loop {
        match pc {
            0x83293698 => {
    //   block [0x83293698..0x832936A4)
	// 83293698: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329369C: 386B6D90  addi r3, r11, 0x6d90
	ctx.r[3].s64 = ctx.r[11].s64 + 28048;
	// 832936A0: 4BA16880  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832936A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832936A8 size=12
    let mut pc: u32 = 0x832936A8;
    'dispatch: loop {
        match pc {
            0x832936A8 => {
    //   block [0x832936A8..0x832936B4)
	// 832936A8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832936AC: 386B6DE8  addi r3, r11, 0x6de8
	ctx.r[3].s64 = ctx.r[11].s64 + 28136;
	// 832936B0: 4BA16870  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832936B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832936B8 size=12
    let mut pc: u32 = 0x832936B8;
    'dispatch: loop {
        match pc {
            0x832936B8 => {
    //   block [0x832936B8..0x832936C4)
	// 832936B8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832936BC: 386B6E40  addi r3, r11, 0x6e40
	ctx.r[3].s64 = ctx.r[11].s64 + 28224;
	// 832936C0: 4BA16860  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832936C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832936C8 size=64
    let mut pc: u32 = 0x832936C8;
    'dispatch: loop {
        match pc {
            0x832936C8 => {
    //   block [0x832936C8..0x83293708)
	// 832936C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832936CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832936D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832936D4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832936D8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832936DC: 388BAFAC  addi r4, r11, -0x5054
	ctx.r[4].s64 = ctx.r[11].s64 + -20564;
	// 832936E0: 386ADF38  addi r3, r10, -0x20c8
	ctx.r[3].s64 = ctx.r[10].s64 + -8392;
	// 832936E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832936E8: 4AF997E9  bl 0x8222ced0
	ctx.lr = 0x832936EC;
	sub_8222CED0(ctx, base);
	// 832936EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832936F0: 38696E98  addi r3, r9, 0x6e98
	ctx.r[3].s64 = ctx.r[9].s64 + 28312;
	// 832936F4: 4BA1682D  bl 0x82ca9f20
	ctx.lr = 0x832936F8;
	sub_82CA9F20(ctx, base);
	// 832936F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832936FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293708 size=60
    let mut pc: u32 = 0x83293708;
    'dispatch: loop {
        match pc {
            0x83293708 => {
    //   block [0x83293708..0x83293718)
	// 83293708: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 8329370C: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 83293710: 396BDF40  addi r11, r11, -0x20c0
	ctx.r[11].s64 = ctx.r[11].s64 + -8384;
	// 83293714: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x83293718; continue 'dispatch;
            }
            0x83293718 => {
    //   block [0x83293718..0x83293744)
	// 83293718: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329371C: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83293720: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83293724: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83293728: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329372C: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83293730: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 83293734: 4080FFE4  bge 0x83293718
	if !ctx.cr[0].lt {
	pc = 0x83293718; continue 'dispatch;
	}
	// 83293738: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329373C: 386B6EA8  addi r3, r11, 0x6ea8
	ctx.r[3].s64 = ctx.r[11].s64 + 28328;
	// 83293740: 4BA167E0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293748 size=64
    let mut pc: u32 = 0x83293748;
    'dispatch: loop {
        match pc {
            0x83293748 => {
    //   block [0x83293748..0x83293788)
	// 83293748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329374C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293754: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83293758: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329375C: 388BB160  addi r4, r11, -0x4ea0
	ctx.r[4].s64 = ctx.r[11].s64 + -20128;
	// 83293760: 386AE008  addi r3, r10, -0x1ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -8184;
	// 83293764: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293768: 4AF99769  bl 0x8222ced0
	ctx.lr = 0x8329376C;
	sub_8222CED0(ctx, base);
	// 8329376C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83293770: 38696F00  addi r3, r9, 0x6f00
	ctx.r[3].s64 = ctx.r[9].s64 + 28416;
	// 83293774: 4BA167AD  bl 0x82ca9f20
	ctx.lr = 0x83293778;
	sub_82CA9F20(ctx, base);
	// 83293778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329377C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293788 size=1508
    let mut pc: u32 = 0x83293788;
    'dispatch: loop {
        match pc {
            0x83293788 => {
    //   block [0x83293788..0x83293CF4)
	// 83293788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329378C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293790: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83293794: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83293798: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329379C: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832937A0: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 832937A4: 3BEBE010  addi r31, r11, -0x1ff0
	ctx.r[31].s64 = ctx.r[11].s64 + -8176;
	// 832937A8: 388AB378  addi r4, r10, -0x4c88
	ctx.r[4].s64 = ctx.r[10].s64 + -19592;
	// 832937AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832937B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832937B4: 4AF9971D  bl 0x8222ced0
	ctx.lr = 0x832937B8;
	sub_8222CED0(ctx, base);
	// 832937B8: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 832937BC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 832937C0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 832937C4: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832937C8: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832937CC: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 832937D0: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 832937D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832937D8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832937DC: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 832937E0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832937E4: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 832937E8: 39400500  li r10, 0x500
	ctx.r[10].s64 = 1280;
	// 832937EC: 392002D0  li r9, 0x2d0
	ctx.r[9].s64 = 720;
	// 832937F0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 832937F4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 832937F8: 3D008210  lis r8, -0x7df0
	ctx.r[8].s64 = -2112880640;
	// 832937FC: 997F0068  stb r11, 0x68(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u8 ) };
	// 83293800: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293804: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 83293808: 3888B33C  addi r4, r8, -0x4cc4
	ctx.r[4].s64 = ctx.r[8].s64 + -19652;
	// 8329380C: 913F0024  stw r9, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 83293810: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 83293814: 4AF996BD  bl 0x8222ced0
	ctx.lr = 0x83293818;
	sub_8222CED0(ctx, base);
	// 83293818: 39600400  li r11, 0x400
	ctx.r[11].s64 = 1024;
	// 8329381C: 39400240  li r10, 0x240
	ctx.r[10].s64 = 576;
	// 83293820: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83293824: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 83293828: 915F0074  stw r10, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 8329382C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83293830: 913F0078  stw r9, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 83293834: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293838: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329383C: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 83293840: 913F0084  stw r9, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 83293844: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293848: 915F0080  stw r10, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 8329384C: 39400400  li r10, 0x400
	ctx.r[10].s64 = 1024;
	// 83293850: 39200240  li r9, 0x240
	ctx.r[9].s64 = 576;
	// 83293854: 917F0088  stw r11, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 83293858: 997F00D4  stb r11, 0xd4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u8 ) };
	// 8329385C: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 83293860: 915F008C  stw r10, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 83293864: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293868: 913F0090  stw r9, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[9].u32 ) };
	// 8329386C: 3887B2F8  addi r4, r7, -0x4d08
	ctx.r[4].s64 = ctx.r[7].s64 + -19720;
	// 83293870: 387F00D8  addi r3, r31, 0xd8
	ctx.r[3].s64 = ctx.r[31].s64 + 216;
	// 83293874: 4AF9965D  bl 0x8222ced0
	ctx.lr = 0x83293878;
	sub_8222CED0(ctx, base);
	// 83293878: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 8329387C: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293880: 917F00DC  stw r11, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 83293884: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83293888: 915F00E0  stw r10, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 8329388C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83293890: 917F00E8  stw r11, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 83293894: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83293898: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8329389C: 915F00EC  stw r10, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[10].u32 ) };
	// 832938A0: 913F00E4  stw r9, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 832938A4: 39400500  li r10, 0x500
	ctx.r[10].s64 = 1280;
	// 832938A8: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 832938AC: 917F00F4  stw r11, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 832938B0: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 832938B4: 915F00F8  stw r10, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 832938B8: 913F00F0  stw r9, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[9].u32 ) };
	// 832938BC: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 832938C0: 39200200  li r9, 0x200
	ctx.r[9].s64 = 512;
	// 832938C4: 917F00FC  stw r11, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[11].u32 ) };
	// 832938C8: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 832938CC: 915F0100  stw r10, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[10].u32 ) };
	// 832938D0: 913F0104  stw r9, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[9].u32 ) };
	// 832938D4: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 832938D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832938DC: 917F0108  stw r11, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[11].u32 ) };
	// 832938E0: 915F010C  stw r10, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[10].u32 ) };
	// 832938E4: 3CC08210  lis r6, -0x7df0
	ctx.r[6].s64 = -2112880640;
	// 832938E8: 993F0140  stb r9, 0x140(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[9].u8 ) };
	// 832938EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832938F0: 3886B2BC  addi r4, r6, -0x4d44
	ctx.r[4].s64 = ctx.r[6].s64 + -19780;
	// 832938F4: 387F0144  addi r3, r31, 0x144
	ctx.r[3].s64 = ctx.r[31].s64 + 324;
	// 832938F8: 4AF995D9  bl 0x8222ced0
	ctx.lr = 0x832938FC;
	sub_8222CED0(ctx, base);
	// 832938FC: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293900: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293904: 917F0148  stw r11, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 83293908: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329390C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83293910: 915F014C  stw r10, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[10].u32 ) };
	// 83293914: 913F0150  stw r9, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[9].u32 ) };
	// 83293918: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8329391C: 917F0154  stw r11, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 83293920: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293924: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293928: 915F0158  stw r10, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 8329392C: 913F015C  stw r9, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[9].u32 ) };
	// 83293930: 39400280  li r10, 0x280
	ctx.r[10].s64 = 640;
	// 83293934: 917F0160  stw r11, 0x160(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), ctx.r[11].u32 ) };
	// 83293938: 392002D0  li r9, 0x2d0
	ctx.r[9].s64 = 720;
	// 8329393C: 39600280  li r11, 0x280
	ctx.r[11].s64 = 640;
	// 83293940: 915F0164  stw r10, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[10].u32 ) };
	// 83293944: 913F0168  stw r9, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[9].u32 ) };
	// 83293948: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8329394C: 917F016C  stw r11, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[11].u32 ) };
	// 83293950: 39200500  li r9, 0x500
	ctx.r[9].s64 = 1280;
	// 83293954: 396002D0  li r11, 0x2d0
	ctx.r[11].s64 = 720;
	// 83293958: 915F0170  stw r10, 0x170(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 8329395C: 995F01AC  stb r10, 0x1ac(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(428 as u32), ctx.r[10].u8 ) };
	// 83293960: 3C808210  lis r4, -0x7df0
	ctx.r[4].s64 = -2112880640;
	// 83293964: 913F0174  stw r9, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[9].u32 ) };
	// 83293968: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8329396C: 917F0178  stw r11, 0x178(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), ctx.r[11].u32 ) };
	// 83293970: 3884B278  addi r4, r4, -0x4d88
	ctx.r[4].s64 = ctx.r[4].s64 + -19848;
	// 83293974: 387F01B0  addi r3, r31, 0x1b0
	ctx.r[3].s64 = ctx.r[31].s64 + 432;
	// 83293978: 4AF99559  bl 0x8222ced0
	ctx.lr = 0x8329397C;
	sub_8222CED0(ctx, base);
	// 8329397C: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293980: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293984: 917F01B4  stw r11, 0x1b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(436 as u32), ctx.r[11].u32 ) };
	// 83293988: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8329398C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83293990: 915F01B8  stw r10, 0x1b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 83293994: 913F01BC  stw r9, 0x1bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(444 as u32), ctx.r[9].u32 ) };
	// 83293998: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8329399C: 917F01C0  stw r11, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[11].u32 ) };
	// 832939A0: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 832939A4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832939A8: 915F01C4  stw r10, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[10].u32 ) };
	// 832939AC: 913F01C8  stw r9, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[9].u32 ) };
	// 832939B0: 39400280  li r10, 0x280
	ctx.r[10].s64 = 640;
	// 832939B4: 917F01CC  stw r11, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[11].u32 ) };
	// 832939B8: 392002D0  li r9, 0x2d0
	ctx.r[9].s64 = 720;
	// 832939BC: 39600280  li r11, 0x280
	ctx.r[11].s64 = 640;
	// 832939C0: 915F01D0  stw r10, 0x1d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 832939C4: 913F01D4  stw r9, 0x1d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(468 as u32), ctx.r[9].u32 ) };
	// 832939C8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 832939CC: 917F01D8  stw r11, 0x1d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(472 as u32), ctx.r[11].u32 ) };
	// 832939D0: 39200500  li r9, 0x500
	ctx.r[9].s64 = 1280;
	// 832939D4: 396002D0  li r11, 0x2d0
	ctx.r[11].s64 = 720;
	// 832939D8: 915F01DC  stw r10, 0x1dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(476 as u32), ctx.r[10].u32 ) };
	// 832939DC: 913F01E0  stw r9, 0x1e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(480 as u32), ctx.r[9].u32 ) };
	// 832939E0: 3C608210  lis r3, -0x7df0
	ctx.r[3].s64 = -2112880640;
	// 832939E4: 917F01E4  stw r11, 0x1e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(484 as u32), ctx.r[11].u32 ) };
	// 832939E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832939EC: 995F0218  stb r10, 0x218(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(536 as u32), ctx.r[10].u8 ) };
	// 832939F0: 3883B230  addi r4, r3, -0x4dd0
	ctx.r[4].s64 = ctx.r[3].s64 + -19920;
	// 832939F4: 387F021C  addi r3, r31, 0x21c
	ctx.r[3].s64 = ctx.r[31].s64 + 540;
	// 832939F8: 4AF994D9  bl 0x8222ced0
	ctx.lr = 0x832939FC;
	sub_8222CED0(ctx, base);
	// 832939FC: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293A00: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293A04: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83293A08: 917F0220  stw r11, 0x220(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(544 as u32), ctx.r[11].u32 ) };
	// 83293A0C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83293A10: 915F0224  stw r10, 0x224(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(548 as u32), ctx.r[10].u32 ) };
	// 83293A14: 913F0228  stw r9, 0x228(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(552 as u32), ctx.r[9].u32 ) };
	// 83293A18: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 83293A1C: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293A20: 917F022C  stw r11, 0x22c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(556 as u32), ctx.r[11].u32 ) };
	// 83293A24: 915F0230  stw r10, 0x230(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 83293A28: 39400500  li r10, 0x500
	ctx.r[10].s64 = 1280;
	// 83293A2C: 913F0234  stw r9, 0x234(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(564 as u32), ctx.r[9].u32 ) };
	// 83293A30: 39200100  li r9, 0x100
	ctx.r[9].s64 = 256;
	// 83293A34: 915F023C  stw r10, 0x23c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(572 as u32), ctx.r[10].u32 ) };
	// 83293A38: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293A3C: 913F0240  stw r9, 0x240(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(576 as u32), ctx.r[9].u32 ) };
	// 83293A40: 39400100  li r10, 0x100
	ctx.r[10].s64 = 256;
	// 83293A44: 39200500  li r9, 0x500
	ctx.r[9].s64 = 1280;
	// 83293A48: 917F0238  stw r11, 0x238(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(568 as u32), ctx.r[11].u32 ) };
	// 83293A4C: 917F0244  stw r11, 0x244(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(580 as u32), ctx.r[11].u32 ) };
	// 83293A50: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 83293A54: 915F0248  stw r10, 0x248(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(584 as u32), ctx.r[10].u32 ) };
	// 83293A58: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 83293A5C: 913F024C  stw r9, 0x24c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(588 as u32), ctx.r[9].u32 ) };
	// 83293A60: 39200200  li r9, 0x200
	ctx.r[9].s64 = 512;
	// 83293A64: 917F0250  stw r11, 0x250(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(592 as u32), ctx.r[11].u32 ) };
	// 83293A68: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293A6C: 915F0254  stw r10, 0x254(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(596 as u32), ctx.r[10].u32 ) };
	// 83293A70: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293A74: 913F0258  stw r9, 0x258(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(600 as u32), ctx.r[9].u32 ) };
	// 83293A78: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293A7C: 917F025C  stw r11, 0x25c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(604 as u32), ctx.r[11].u32 ) };
	// 83293A80: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83293A84: 915F0260  stw r10, 0x260(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(608 as u32), ctx.r[10].u32 ) };
	// 83293A88: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293A8C: 993F0284  stb r9, 0x284(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(644 as u32), ctx.r[9].u8 ) };
	// 83293A90: 388BB1F4  addi r4, r11, -0x4e0c
	ctx.r[4].s64 = ctx.r[11].s64 + -19980;
	// 83293A94: 387F0288  addi r3, r31, 0x288
	ctx.r[3].s64 = ctx.r[31].s64 + 648;
	// 83293A98: 4AF99439  bl 0x8222ced0
	ctx.lr = 0x83293A9C;
	sub_8222CED0(ctx, base);
	// 83293A9C: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293AA0: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293AA4: 917F028C  stw r11, 0x28c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(652 as u32), ctx.r[11].u32 ) };
	// 83293AA8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83293AAC: 915F0290  stw r10, 0x290(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(656 as u32), ctx.r[10].u32 ) };
	// 83293AB0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 83293AB4: 917F0298  stw r11, 0x298(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(664 as u32), ctx.r[11].u32 ) };
	// 83293AB8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293ABC: 915F029C  stw r10, 0x29c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(668 as u32), ctx.r[10].u32 ) };
	// 83293AC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 83293AC4: 917F02A4  stw r11, 0x2a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(676 as u32), ctx.r[11].u32 ) };
	// 83293AC8: 39400140  li r10, 0x140
	ctx.r[10].s64 = 320;
	// 83293ACC: 39600140  li r11, 0x140
	ctx.r[11].s64 = 320;
	// 83293AD0: 913F0294  stw r9, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[9].u32 ) };
	// 83293AD4: 915F02A8  stw r10, 0x2a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(680 as u32), ctx.r[10].u32 ) };
	// 83293AD8: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293ADC: 917F02B0  stw r11, 0x2b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(688 as u32), ctx.r[11].u32 ) };
	// 83293AE0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 83293AE4: 396002D0  li r11, 0x2d0
	ctx.r[11].s64 = 720;
	// 83293AE8: 913F02A0  stw r9, 0x2a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(672 as u32), ctx.r[9].u32 ) };
	// 83293AEC: 915F02B4  stw r10, 0x2b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(692 as u32), ctx.r[10].u32 ) };
	// 83293AF0: 392002D0  li r9, 0x2d0
	ctx.r[9].s64 = 720;
	// 83293AF4: 917F02BC  stw r11, 0x2bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(700 as u32), ctx.r[11].u32 ) };
	// 83293AF8: 39400280  li r10, 0x280
	ctx.r[10].s64 = 640;
	// 83293AFC: 396003C0  li r11, 0x3c0
	ctx.r[11].s64 = 960;
	// 83293B00: 913F02AC  stw r9, 0x2ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(684 as u32), ctx.r[9].u32 ) };
	// 83293B04: 915F02C0  stw r10, 0x2c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(704 as u32), ctx.r[10].u32 ) };
	// 83293B08: 39200280  li r9, 0x280
	ctx.r[9].s64 = 640;
	// 83293B0C: 917F02C8  stw r11, 0x2c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(712 as u32), ctx.r[11].u32 ) };
	// 83293B10: 394003C0  li r10, 0x3c0
	ctx.r[10].s64 = 960;
	// 83293B14: 396002D0  li r11, 0x2d0
	ctx.r[11].s64 = 720;
	// 83293B18: 913F02B8  stw r9, 0x2b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(696 as u32), ctx.r[9].u32 ) };
	// 83293B1C: 915F02D0  stw r10, 0x2d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(720 as u32), ctx.r[10].u32 ) };
	// 83293B20: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293B24: 917F02CC  stw r11, 0x2cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(716 as u32), ctx.r[11].u32 ) };
	// 83293B28: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293B2C: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293B30: 913F02C4  stw r9, 0x2c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(708 as u32), ctx.r[9].u32 ) };
	// 83293B34: 913F02D4  stw r9, 0x2d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(724 as u32), ctx.r[9].u32 ) };
	// 83293B38: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293B3C: 915F02DC  stw r10, 0x2dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(732 as u32), ctx.r[10].u32 ) };
	// 83293B40: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 83293B44: 917F02D8  stw r11, 0x2d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(728 as u32), ctx.r[11].u32 ) };
	// 83293B48: 387F02F4  addi r3, r31, 0x2f4
	ctx.r[3].s64 = ctx.r[31].s64 + 756;
	// 83293B4C: 993F02F0  stb r9, 0x2f0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(752 as u32), ctx.r[9].u8 ) };
	// 83293B50: 388AB1B0  addi r4, r10, -0x4e50
	ctx.r[4].s64 = ctx.r[10].s64 + -20048;
	// 83293B54: 4AF9937D  bl 0x8222ced0
	ctx.lr = 0x83293B58;
	sub_8222CED0(ctx, base);
	// 83293B58: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293B5C: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293B60: 917F02F8  stw r11, 0x2f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(760 as u32), ctx.r[11].u32 ) };
	// 83293B64: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83293B68: 915F02FC  stw r10, 0x2fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(764 as u32), ctx.r[10].u32 ) };
	// 83293B6C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83293B70: 913F0300  stw r9, 0x300(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(768 as u32), ctx.r[9].u32 ) };
	// 83293B74: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293B78: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 83293B7C: 917F0304  stw r11, 0x304(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(772 as u32), ctx.r[11].u32 ) };
	// 83293B80: 913F030C  stw r9, 0x30c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(780 as u32), ctx.r[9].u32 ) };
	// 83293B84: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293B88: 915F0308  stw r10, 0x308(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(776 as u32), ctx.r[10].u32 ) };
	// 83293B8C: 392002D0  li r9, 0x2d0
	ctx.r[9].s64 = 720;
	// 83293B90: 39400140  li r10, 0x140
	ctx.r[10].s64 = 320;
	// 83293B94: 917F0310  stw r11, 0x310(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(784 as u32), ctx.r[11].u32 ) };
	// 83293B98: 913F0318  stw r9, 0x318(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(792 as u32), ctx.r[9].u32 ) };
	// 83293B9C: 39600140  li r11, 0x140
	ctx.r[11].s64 = 320;
	// 83293BA0: 915F0314  stw r10, 0x314(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(788 as u32), ctx.r[10].u32 ) };
	// 83293BA4: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 83293BA8: 39200280  li r9, 0x280
	ctx.r[9].s64 = 640;
	// 83293BAC: 917F031C  stw r11, 0x31c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(796 as u32), ctx.r[11].u32 ) };
	// 83293BB0: 915F0320  stw r10, 0x320(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(800 as u32), ctx.r[10].u32 ) };
	// 83293BB4: 396002D0  li r11, 0x2d0
	ctx.r[11].s64 = 720;
	// 83293BB8: 913F0324  stw r9, 0x324(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(804 as u32), ctx.r[9].u32 ) };
	// 83293BBC: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293BC0: 39400280  li r10, 0x280
	ctx.r[10].s64 = 640;
	// 83293BC4: 917F0328  stw r11, 0x328(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(808 as u32), ctx.r[11].u32 ) };
	// 83293BC8: 913F0330  stw r9, 0x330(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(816 as u32), ctx.r[9].u32 ) };
	// 83293BCC: 396003C0  li r11, 0x3c0
	ctx.r[11].s64 = 960;
	// 83293BD0: 915F032C  stw r10, 0x32c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(812 as u32), ctx.r[10].u32 ) };
	// 83293BD4: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293BD8: 392003C0  li r9, 0x3c0
	ctx.r[9].s64 = 960;
	// 83293BDC: 917F0334  stw r11, 0x334(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(820 as u32), ctx.r[11].u32 ) };
	// 83293BE0: 915F0338  stw r10, 0x338(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(824 as u32), ctx.r[10].u32 ) };
	// 83293BE4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293BE8: 913F033C  stw r9, 0x33c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(828 as u32), ctx.r[9].u32 ) };
	// 83293BEC: 39400500  li r10, 0x500
	ctx.r[10].s64 = 1280;
	// 83293BF0: 392002D0  li r9, 0x2d0
	ctx.r[9].s64 = 720;
	// 83293BF4: 917F0340  stw r11, 0x340(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(832 as u32), ctx.r[11].u32 ) };
	// 83293BF8: 997F035C  stb r11, 0x35c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(860 as u32), ctx.r[11].u8 ) };
	// 83293BFC: 3D008210  lis r8, -0x7df0
	ctx.r[8].s64 = -2112880640;
	// 83293C00: 915F0344  stw r10, 0x344(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(836 as u32), ctx.r[10].u32 ) };
	// 83293C04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293C08: 913F0348  stw r9, 0x348(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(840 as u32), ctx.r[9].u32 ) };
	// 83293C0C: 3888B16C  addi r4, r8, -0x4e94
	ctx.r[4].s64 = ctx.r[8].s64 + -20116;
	// 83293C10: 387F0360  addi r3, r31, 0x360
	ctx.r[3].s64 = ctx.r[31].s64 + 864;
	// 83293C14: 4AF992BD  bl 0x8222ced0
	ctx.lr = 0x83293C18;
	sub_8222CED0(ctx, base);
	// 83293C18: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293C1C: 3920003A  li r9, 0x3a
	ctx.r[9].s64 = 58;
	// 83293C20: 917F0364  stw r11, 0x364(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(868 as u32), ctx.r[11].u32 ) };
	// 83293C24: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83293C28: 913F036C  stw r9, 0x36c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(876 as u32), ctx.r[9].u32 ) };
	// 83293C2C: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293C30: 917F0370  stw r11, 0x370(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(880 as u32), ctx.r[11].u32 ) };
	// 83293C34: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293C38: 39200500  li r9, 0x500
	ctx.r[9].s64 = 1280;
	// 83293C3C: 915F0368  stw r10, 0x368(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(872 as u32), ctx.r[10].u32 ) };
	// 83293C40: 917F0378  stw r11, 0x378(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(888 as u32), ctx.r[11].u32 ) };
	// 83293C44: 39600090  li r11, 0x90
	ctx.r[11].s64 = 144;
	// 83293C48: 913F0380  stw r9, 0x380(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(896 as u32), ctx.r[9].u32 ) };
	// 83293C4C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 83293C50: 917F0384  stw r11, 0x384(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(900 as u32), ctx.r[11].u32 ) };
	// 83293C54: 39200090  li r9, 0x90
	ctx.r[9].s64 = 144;
	// 83293C58: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293C5C: 915F0374  stw r10, 0x374(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(884 as u32), ctx.r[10].u32 ) };
	// 83293C60: 913F038C  stw r9, 0x38c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(908 as u32), ctx.r[9].u32 ) };
	// 83293C64: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 83293C68: 917F0390  stw r11, 0x390(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(912 as u32), ctx.r[11].u32 ) };
	// 83293C6C: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293C70: 39600120  li r11, 0x120
	ctx.r[11].s64 = 288;
	// 83293C74: 915F037C  stw r10, 0x37c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(892 as u32), ctx.r[10].u32 ) };
	// 83293C78: 915F0388  stw r10, 0x388(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(904 as u32), ctx.r[10].u32 ) };
	// 83293C7C: 39400120  li r10, 0x120
	ctx.r[10].s64 = 288;
	// 83293C80: 913F0398  stw r9, 0x398(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(920 as u32), ctx.r[9].u32 ) };
	// 83293C84: 392001B0  li r9, 0x1b0
	ctx.r[9].s64 = 432;
	// 83293C88: 917F039C  stw r11, 0x39c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(924 as u32), ctx.r[11].u32 ) };
	// 83293C8C: 3CA08349  lis r5, -0x7cb7
	ctx.r[5].s64 = -2092367872;
	// 83293C90: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293C94: 915F0394  stw r10, 0x394(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(916 as u32), ctx.r[10].u32 ) };
	// 83293C98: 913F03A4  stw r9, 0x3a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(932 as u32), ctx.r[9].u32 ) };
	// 83293C9C: 38857088  addi r4, r5, 0x7088
	ctx.r[4].s64 = ctx.r[5].s64 + 28808;
	// 83293CA0: 39400500  li r10, 0x500
	ctx.r[10].s64 = 1280;
	// 83293CA4: 917F03A8  stw r11, 0x3a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(936 as u32), ctx.r[11].u32 ) };
	// 83293CA8: 39200500  li r9, 0x500
	ctx.r[9].s64 = 1280;
	// 83293CAC: 915F03A0  stw r10, 0x3a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(928 as u32), ctx.r[10].u32 ) };
	// 83293CB0: 394001B0  li r10, 0x1b0
	ctx.r[10].s64 = 432;
	// 83293CB4: 39600240  li r11, 0x240
	ctx.r[11].s64 = 576;
	// 83293CB8: 913F03B0  stw r9, 0x3b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(944 as u32), ctx.r[9].u32 ) };
	// 83293CBC: 915F03AC  stw r10, 0x3ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(940 as u32), ctx.r[10].u32 ) };
	// 83293CC0: 39200240  li r9, 0x240
	ctx.r[9].s64 = 576;
	// 83293CC4: 917F03B4  stw r11, 0x3b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(948 as u32), ctx.r[11].u32 ) };
	// 83293CC8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 83293CCC: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293CD0: 913F03BC  stw r9, 0x3bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(956 as u32), ctx.r[9].u32 ) };
	// 83293CD4: 915F03B8  stw r10, 0x3b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(952 as u32), ctx.r[10].u32 ) };
	// 83293CD8: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293CDC: 917F03C0  stw r11, 0x3c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(960 as u32), ctx.r[11].u32 ) };
	// 83293CE0: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293CE4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293CE8: 993F03C8  stb r9, 0x3c8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(968 as u32), ctx.r[9].u8 ) };
	// 83293CEC: 915F03C4  stw r10, 0x3c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(964 as u32), ctx.r[10].u32 ) };
	// 83293CF0: 917F03CC  stw r11, 0x3cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(972 as u32), ctx.r[11].u32 ) };
	pc = 0x83293CF4; continue 'dispatch;
            }
            0x83293CF4 => {
    //   block [0x83293CF4..0x83293D24)
	// 83293CF4: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 83293CF8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83293CFC: 7CE02028  lwarx r7, 0, r4
	// lwarx
	let ea = ctx.r[4].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 83293D00: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 83293D04: 7CE0212D  stwcx. r7, 0, r4
	// stwcx.
	let addr = ctx.r[4].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83293D08: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83293D0C: 4082FFE8  bne 0x83293cf4
	if !ctx.cr[0].eq {
	pc = 0x83293CF4; continue 'dispatch;
	}
	// 83293D10: 817F03CC  lwz r11, 0x3cc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(972 as u32) ) } as u64;
	// 83293D14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83293D18: 419A000C  beq cr6, 0x83293d24
	if ctx.cr[6].eq {
	pc = 0x83293D24; continue 'dispatch;
	}
	// 83293D1C: 387F03CC  addi r3, r31, 0x3cc
	ctx.r[3].s64 = ctx.r[31].s64 + 972;
	// 83293D20: 4AF32A49  bl 0x821c6768
	ctx.lr = 0x83293D24;
	sub_821C6768(ctx, base);
	pc = 0x83293D24; continue 'dispatch;
            }
            0x83293D24 => {
    //   block [0x83293D24..0x83293D6C)
	// 83293D24: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293D28: FBDF03D0  std r30, 0x3d0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(976 as u32), ctx.r[30].u64 ) };
	// 83293D2C: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83293D30: 917F03D8  stw r11, 0x3d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(984 as u32), ctx.r[11].u32 ) };
	// 83293D34: 397F03D0  addi r11, r31, 0x3d0
	ctx.r[11].s64 = ctx.r[31].s64 + 976;
	// 83293D38: 93DF03DC  stw r30, 0x3dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(988 as u32), ctx.r[30].u32 ) };
	// 83293D3C: 386A6F10  addi r3, r10, 0x6f10
	ctx.r[3].s64 = ctx.r[10].s64 + 28432;
	// 83293D40: 93DF03E0  stw r30, 0x3e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(992 as u32), ctx.r[30].u32 ) };
	// 83293D44: 93DF0434  stw r30, 0x434(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1076 as u32), ctx.r[30].u32 ) };
	// 83293D48: 397F03DC  addi r11, r31, 0x3dc
	ctx.r[11].s64 = ctx.r[31].s64 + 988;
	// 83293D4C: 397F0434  addi r11, r31, 0x434
	ctx.r[11].s64 = ctx.r[31].s64 + 1076;
	// 83293D50: 4BA161D1  bl 0x82ca9f20
	ctx.lr = 0x83293D54;
	sub_82CA9F20(ctx, base);
	// 83293D54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83293D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293D60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83293D64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83293D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83293D70 size=12
    let mut pc: u32 = 0x83293D70;
    'dispatch: loop {
        match pc {
            0x83293D70 => {
    //   block [0x83293D70..0x83293D7C)
	// 83293D70: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83293D74: 386B6F78  addi r3, r11, 0x6f78
	ctx.r[3].s64 = ctx.r[11].s64 + 28536;
	// 83293D78: 4BA161A8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293D80 size=64
    let mut pc: u32 = 0x83293D80;
    'dispatch: loop {
        match pc {
            0x83293D80 => {
    //   block [0x83293D80..0x83293DC0)
	// 83293D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293D8C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83293D90: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83293D94: 388BB400  addi r4, r11, -0x4c00
	ctx.r[4].s64 = ctx.r[11].s64 + -19456;
	// 83293D98: 386AE448  addi r3, r10, -0x1bb8
	ctx.r[3].s64 = ctx.r[10].s64 + -7096;
	// 83293D9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293DA0: 4AF99131  bl 0x8222ced0
	ctx.lr = 0x83293DA4;
	sub_8222CED0(ctx, base);
	// 83293DA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83293DA8: 38696FB8  addi r3, r9, 0x6fb8
	ctx.r[3].s64 = ctx.r[9].s64 + 28600;
	// 83293DAC: 4BA16175  bl 0x82ca9f20
	ctx.lr = 0x83293DB0;
	sub_82CA9F20(ctx, base);
	// 83293DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293DC0 size=64
    let mut pc: u32 = 0x83293DC0;
    'dispatch: loop {
        match pc {
            0x83293DC0 => {
    //   block [0x83293DC0..0x83293E00)
	// 83293DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293DC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293DCC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83293DD0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83293DD4: 388BB40C  addi r4, r11, -0x4bf4
	ctx.r[4].s64 = ctx.r[11].s64 + -19444;
	// 83293DD8: 386AE44C  addi r3, r10, -0x1bb4
	ctx.r[3].s64 = ctx.r[10].s64 + -7092;
	// 83293DDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293DE0: 4AF990F1  bl 0x8222ced0
	ctx.lr = 0x83293DE4;
	sub_8222CED0(ctx, base);
	// 83293DE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83293DE8: 38696FC8  addi r3, r9, 0x6fc8
	ctx.r[3].s64 = ctx.r[9].s64 + 28616;
	// 83293DEC: 4BA16135  bl 0x82ca9f20
	ctx.lr = 0x83293DF0;
	sub_82CA9F20(ctx, base);
	// 83293DF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293E00 size=64
    let mut pc: u32 = 0x83293E00;
    'dispatch: loop {
        match pc {
            0x83293E00 => {
    //   block [0x83293E00..0x83293E40)
	// 83293E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293E08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293E0C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83293E10: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83293E14: 388BB420  addi r4, r11, -0x4be0
	ctx.r[4].s64 = ctx.r[11].s64 + -19424;
	// 83293E18: 386AE450  addi r3, r10, -0x1bb0
	ctx.r[3].s64 = ctx.r[10].s64 + -7088;
	// 83293E1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293E20: 4AF990B1  bl 0x8222ced0
	ctx.lr = 0x83293E24;
	sub_8222CED0(ctx, base);
	// 83293E24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83293E28: 38696FD8  addi r3, r9, 0x6fd8
	ctx.r[3].s64 = ctx.r[9].s64 + 28632;
	// 83293E2C: 4BA160F5  bl 0x82ca9f20
	ctx.lr = 0x83293E30;
	sub_82CA9F20(ctx, base);
	// 83293E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293E40 size=64
    let mut pc: u32 = 0x83293E40;
    'dispatch: loop {
        match pc {
            0x83293E40 => {
    //   block [0x83293E40..0x83293E80)
	// 83293E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293E4C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83293E50: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83293E54: 388BB430  addi r4, r11, -0x4bd0
	ctx.r[4].s64 = ctx.r[11].s64 + -19408;
	// 83293E58: 386AE454  addi r3, r10, -0x1bac
	ctx.r[3].s64 = ctx.r[10].s64 + -7084;
	// 83293E5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293E60: 4AF99071  bl 0x8222ced0
	ctx.lr = 0x83293E64;
	sub_8222CED0(ctx, base);
	// 83293E64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83293E68: 38696FE8  addi r3, r9, 0x6fe8
	ctx.r[3].s64 = ctx.r[9].s64 + 28648;
	// 83293E6C: 4BA160B5  bl 0x82ca9f20
	ctx.lr = 0x83293E70;
	sub_82CA9F20(ctx, base);
	// 83293E70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293E80 size=64
    let mut pc: u32 = 0x83293E80;
    'dispatch: loop {
        match pc {
            0x83293E80 => {
    //   block [0x83293E80..0x83293EC0)
	// 83293E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293E88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293E8C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83293E90: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83293E94: 388BB448  addi r4, r11, -0x4bb8
	ctx.r[4].s64 = ctx.r[11].s64 + -19384;
	// 83293E98: 386AE458  addi r3, r10, -0x1ba8
	ctx.r[3].s64 = ctx.r[10].s64 + -7080;
	// 83293E9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293EA0: 4AF99031  bl 0x8222ced0
	ctx.lr = 0x83293EA4;
	sub_8222CED0(ctx, base);
	// 83293EA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83293EA8: 38696FF8  addi r3, r9, 0x6ff8
	ctx.r[3].s64 = ctx.r[9].s64 + 28664;
	// 83293EAC: 4BA16075  bl 0x82ca9f20
	ctx.lr = 0x83293EB0;
	sub_82CA9F20(ctx, base);
	// 83293EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293EC0 size=64
    let mut pc: u32 = 0x83293EC0;
    'dispatch: loop {
        match pc {
            0x83293EC0 => {
    //   block [0x83293EC0..0x83293F00)
	// 83293EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293EC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293ECC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83293ED0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83293ED4: 388BB460  addi r4, r11, -0x4ba0
	ctx.r[4].s64 = ctx.r[11].s64 + -19360;
	// 83293ED8: 386AE45C  addi r3, r10, -0x1ba4
	ctx.r[3].s64 = ctx.r[10].s64 + -7076;
	// 83293EDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293EE0: 4AF98FF1  bl 0x8222ced0
	ctx.lr = 0x83293EE4;
	sub_8222CED0(ctx, base);
	// 83293EE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83293EE8: 38697008  addi r3, r9, 0x7008
	ctx.r[3].s64 = ctx.r[9].s64 + 28680;
	// 83293EEC: 4BA16035  bl 0x82ca9f20
	ctx.lr = 0x83293EF0;
	sub_82CA9F20(ctx, base);
	// 83293EF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293F00 size=64
    let mut pc: u32 = 0x83293F00;
    'dispatch: loop {
        match pc {
            0x83293F00 => {
    //   block [0x83293F00..0x83293F40)
	// 83293F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293F0C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83293F10: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83293F14: 388BB474  addi r4, r11, -0x4b8c
	ctx.r[4].s64 = ctx.r[11].s64 + -19340;
	// 83293F18: 386AE460  addi r3, r10, -0x1ba0
	ctx.r[3].s64 = ctx.r[10].s64 + -7072;
	// 83293F1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293F20: 4AF98FB1  bl 0x8222ced0
	ctx.lr = 0x83293F24;
	sub_8222CED0(ctx, base);
	// 83293F24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83293F28: 38697018  addi r3, r9, 0x7018
	ctx.r[3].s64 = ctx.r[9].s64 + 28696;
	// 83293F2C: 4BA15FF5  bl 0x82ca9f20
	ctx.lr = 0x83293F30;
	sub_82CA9F20(ctx, base);
	// 83293F30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293F40 size=64
    let mut pc: u32 = 0x83293F40;
    'dispatch: loop {
        match pc {
            0x83293F40 => {
    //   block [0x83293F40..0x83293F80)
	// 83293F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293F48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293F4C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83293F50: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83293F54: 388B93D0  addi r4, r11, -0x6c30
	ctx.r[4].s64 = ctx.r[11].s64 + -27696;
	// 83293F58: 386AE464  addi r3, r10, -0x1b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -7068;
	// 83293F5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293F60: 4AF98F71  bl 0x8222ced0
	ctx.lr = 0x83293F64;
	sub_8222CED0(ctx, base);
	// 83293F64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83293F68: 38697028  addi r3, r9, 0x7028
	ctx.r[3].s64 = ctx.r[9].s64 + 28712;
	// 83293F6C: 4BA15FB5  bl 0x82ca9f20
	ctx.lr = 0x83293F70;
	sub_82CA9F20(ctx, base);
	// 83293F70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293F80 size=64
    let mut pc: u32 = 0x83293F80;
    'dispatch: loop {
        match pc {
            0x83293F80 => {
    //   block [0x83293F80..0x83293FC0)
	// 83293F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293F88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293F8C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83293F90: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83293F94: 388BB484  addi r4, r11, -0x4b7c
	ctx.r[4].s64 = ctx.r[11].s64 + -19324;
	// 83293F98: 386AE468  addi r3, r10, -0x1b98
	ctx.r[3].s64 = ctx.r[10].s64 + -7064;
	// 83293F9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293FA0: 4AF98F31  bl 0x8222ced0
	ctx.lr = 0x83293FA4;
	sub_8222CED0(ctx, base);
	// 83293FA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83293FA8: 38697038  addi r3, r9, 0x7038
	ctx.r[3].s64 = ctx.r[9].s64 + 28728;
	// 83293FAC: 4BA15F75  bl 0x82ca9f20
	ctx.lr = 0x83293FB0;
	sub_82CA9F20(ctx, base);
	// 83293FB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83293FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293FC0 size=64
    let mut pc: u32 = 0x83293FC0;
    'dispatch: loop {
        match pc {
            0x83293FC0 => {
    //   block [0x83293FC0..0x83294000)
	// 83293FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293FC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293FCC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83293FD0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83293FD4: 388BB49C  addi r4, r11, -0x4b64
	ctx.r[4].s64 = ctx.r[11].s64 + -19300;
	// 83293FD8: 386AE46C  addi r3, r10, -0x1b94
	ctx.r[3].s64 = ctx.r[10].s64 + -7060;
	// 83293FDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293FE0: 4AF98EF1  bl 0x8222ced0
	ctx.lr = 0x83293FE4;
	sub_8222CED0(ctx, base);
	// 83293FE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83293FE8: 38697048  addi r3, r9, 0x7048
	ctx.r[3].s64 = ctx.r[9].s64 + 28744;
	// 83293FEC: 4BA15F35  bl 0x82ca9f20
	ctx.lr = 0x83293FF0;
	sub_82CA9F20(ctx, base);
	// 83293FF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294000 size=64
    let mut pc: u32 = 0x83294000;
    'dispatch: loop {
        match pc {
            0x83294000 => {
    //   block [0x83294000..0x83294040)
	// 83294000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294008: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329400C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294010: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294014: 388BB4B4  addi r4, r11, -0x4b4c
	ctx.r[4].s64 = ctx.r[11].s64 + -19276;
	// 83294018: 386AE470  addi r3, r10, -0x1b90
	ctx.r[3].s64 = ctx.r[10].s64 + -7056;
	// 8329401C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83294020: 4AF98EB1  bl 0x8222ced0
	ctx.lr = 0x83294024;
	sub_8222CED0(ctx, base);
	// 83294024: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83294028: 38697058  addi r3, r9, 0x7058
	ctx.r[3].s64 = ctx.r[9].s64 + 28760;
	// 8329402C: 4BA15EF5  bl 0x82ca9f20
	ctx.lr = 0x83294030;
	sub_82CA9F20(ctx, base);
	// 83294030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329403C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294040 size=64
    let mut pc: u32 = 0x83294040;
    'dispatch: loop {
        match pc {
            0x83294040 => {
    //   block [0x83294040..0x83294080)
	// 83294040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329404C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83294050: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294054: 388B3810  addi r4, r11, 0x3810
	ctx.r[4].s64 = ctx.r[11].s64 + 14352;
	// 83294058: 386AE474  addi r3, r10, -0x1b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -7052;
	// 8329405C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83294060: 4AF98E71  bl 0x8222ced0
	ctx.lr = 0x83294064;
	sub_8222CED0(ctx, base);
	// 83294064: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83294068: 38697068  addi r3, r9, 0x7068
	ctx.r[3].s64 = ctx.r[9].s64 + 28776;
	// 8329406C: 4BA15EB5  bl 0x82ca9f20
	ctx.lr = 0x83294070;
	sub_82CA9F20(ctx, base);
	// 83294070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329407C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294080 size=64
    let mut pc: u32 = 0x83294080;
    'dispatch: loop {
        match pc {
            0x83294080 => {
    //   block [0x83294080..0x832940C0)
	// 83294080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329408C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294090: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294094: 388BB4C4  addi r4, r11, -0x4b3c
	ctx.r[4].s64 = ctx.r[11].s64 + -19260;
	// 83294098: 386AE478  addi r3, r10, -0x1b88
	ctx.r[3].s64 = ctx.r[10].s64 + -7048;
	// 8329409C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832940A0: 4AF98E31  bl 0x8222ced0
	ctx.lr = 0x832940A4;
	sub_8222CED0(ctx, base);
	// 832940A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832940A8: 38697078  addi r3, r9, 0x7078
	ctx.r[3].s64 = ctx.r[9].s64 + 28792;
	// 832940AC: 4BA15E75  bl 0x82ca9f20
	ctx.lr = 0x832940B0;
	sub_82CA9F20(ctx, base);
	// 832940B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832940B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832940B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832940BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832940C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832940C0 size=12
    let mut pc: u32 = 0x832940C0;
    'dispatch: loop {
        match pc {
            0x832940C0 => {
    //   block [0x832940C0..0x832940CC)
	// 832940C0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832940C4: 386B7088  addi r3, r11, 0x7088
	ctx.r[3].s64 = ctx.r[11].s64 + 28808;
	// 832940C8: 4BA15E58  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832940D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832940D0 size=12
    let mut pc: u32 = 0x832940D0;
    'dispatch: loop {
        match pc {
            0x832940D0 => {
    //   block [0x832940D0..0x832940DC)
	// 832940D0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832940D4: 386B7098  addi r3, r11, 0x7098
	ctx.r[3].s64 = ctx.r[11].s64 + 28824;
	// 832940D8: 4BA15E48  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


