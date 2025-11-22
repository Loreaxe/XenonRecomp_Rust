pub fn sub_82C62C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C62C20 size=4
    let mut pc: u32 = 0x82C62C20;
    'dispatch: loop {
        match pc {
            0x82C62C20 => {
    //   block [0x82C62C20..0x82C62C24)
	// 82C62C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C62C28 size=20
    let mut pc: u32 = 0x82C62C28;
    'dispatch: loop {
        match pc {
            0x82C62C28 => {
    //   block [0x82C62C28..0x82C62C3C)
	// 82C62C28: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C62C2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C62C30: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C62C34: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C62C38: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62C3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C62C3C size=8
    let mut pc: u32 = 0x82C62C3C;
    'dispatch: loop {
        match pc {
            0x82C62C3C => {
    //   block [0x82C62C3C..0x82C62C44)
	// 82C62C3C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C62C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C62C48 size=24
    let mut pc: u32 = 0x82C62C48;
    'dispatch: loop {
        match pc {
            0x82C62C48 => {
    //   block [0x82C62C48..0x82C62C60)
	// 82C62C48: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82C62C4C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C62C50: 419A0010  beq cr6, 0x82c62c60
	if ctx.cr[6].eq {
		sub_82C62C60(ctx, base);
		return;
	}
	// 82C62C54: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C62C58: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82C62C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C62C60 size=16
    let mut pc: u32 = 0x82C62C60;
    'dispatch: loop {
        match pc {
            0x82C62C60 => {
    //   block [0x82C62C60..0x82C62C70)
	// 82C62C60: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C62C64: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C62C68: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82C62C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C62C70 size=20
    let mut pc: u32 = 0x82C62C70;
    'dispatch: loop {
        match pc {
            0x82C62C70 => {
    //   block [0x82C62C70..0x82C62C84)
	// 82C62C70: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C62C74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C62C78: 419A000C  beq cr6, 0x82c62c84
	if ctx.cr[6].eq {
		sub_82C62C84(ctx, base);
		return;
	}
	// 82C62C7C: C02B000C  lfs f1, 0xc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C62C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62C84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C62C84 size=12
    let mut pc: u32 = 0x82C62C84;
    'dispatch: loop {
        match pc {
            0x82C62C84 => {
    //   block [0x82C62C84..0x82C62C90)
	// 82C62C84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C62C88: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C62C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C62C90 size=12
    let mut pc: u32 = 0x82C62C90;
    'dispatch: loop {
        match pc {
            0x82C62C90 => {
    //   block [0x82C62C90..0x82C62C9C)
	// 82C62C90: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C62C94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C62C98: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62C9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C62C9C size=8
    let mut pc: u32 = 0x82C62C9C;
    'dispatch: loop {
        match pc {
            0x82C62C9C => {
    //   block [0x82C62C9C..0x82C62CA4)
	// 82C62C9C: 48003FBC  b 0x82c66c58
	sub_82C66C58(ctx, base);
	return;
	// 82C62CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C62CA8 size=20
    let mut pc: u32 = 0x82C62CA8;
    'dispatch: loop {
        match pc {
            0x82C62CA8 => {
    //   block [0x82C62CA8..0x82C62CBC)
	// 82C62CA8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C62CAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C62CB0: 419A000C  beq cr6, 0x82c62cbc
	if ctx.cr[6].eq {
		sub_82C62CBC(ctx, base);
		return;
	}
	// 82C62CB4: C02B0010  lfs f1, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C62CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62CBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C62CBC size=12
    let mut pc: u32 = 0x82C62CBC;
    'dispatch: loop {
        match pc {
            0x82C62CBC => {
    //   block [0x82C62CBC..0x82C62CC8)
	// 82C62CBC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C62CC0: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C62CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C62CC8 size=12
    let mut pc: u32 = 0x82C62CC8;
    'dispatch: loop {
        match pc {
            0x82C62CC8 => {
    //   block [0x82C62CC8..0x82C62CD4)
	// 82C62CC8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C62CCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C62CD0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62CD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C62CD4 size=8
    let mut pc: u32 = 0x82C62CD4;
    'dispatch: loop {
        match pc {
            0x82C62CD4 => {
    //   block [0x82C62CD4..0x82C62CDC)
	// 82C62CD4: 48003F8C  b 0x82c66c60
	sub_82C66C60(ctx, base);
	return;
	// 82C62CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C62CE0 size=20
    let mut pc: u32 = 0x82C62CE0;
    'dispatch: loop {
        match pc {
            0x82C62CE0 => {
    //   block [0x82C62CE0..0x82C62CF4)
	// 82C62CE0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C62CE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C62CE8: 419A000C  beq cr6, 0x82c62cf4
	if ctx.cr[6].eq {
		sub_82C62CF4(ctx, base);
		return;
	}
	// 82C62CEC: C02B0014  lfs f1, 0x14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C62CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62CF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C62CF4 size=12
    let mut pc: u32 = 0x82C62CF4;
    'dispatch: loop {
        match pc {
            0x82C62CF4 => {
    //   block [0x82C62CF4..0x82C62D00)
	// 82C62CF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C62CF8: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C62CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C62D00 size=100
    let mut pc: u32 = 0x82C62D00;
    'dispatch: loop {
        match pc {
            0x82C62D00 => {
    //   block [0x82C62D00..0x82C62D64)
	// 82C62D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C62D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C62D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C62D0C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82C62D10: 54C307BC  rlwinm r3, r6, 0, 0x1e, 0x1e
	ctx.r[3].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82C62D14: 21630000  subfic r11, r3, 0
	ctx.xer.ca = ctx.r[3].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[3].s64;
	// 82C62D18: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82C62D1C: 910A001C  stw r8, 0x1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 82C62D20: 7D0B5910  subfe r8, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[8].u32 = res;
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C62D24: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C62D28: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82C62D2C: 550B07FA  rlwinm r11, r8, 0, 0x1f, 0x1d
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82C62D30: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82C62D34: 90CA000C  stw r6, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82C62D38: 38EB0003  addi r7, r11, 3
	ctx.r[7].s64 = ctx.r[11].s64 + 3;
	// 82C62D3C: 912A0020  stw r9, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82C62D40: 90EA0014  stw r7, 0x14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82C62D44: 4BFFFF05  bl 0x82c62c48
	ctx.lr = 0x82C62D48;
	sub_82C62C48(ctx, base);
	// 82C62D48: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82C62D4C: C0060C14  lfs f0, 0xc14(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C62D50: D00A0024  stfs f0, 0x24(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82C62D54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C62D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C62D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C62D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C62D68 size=12
    let mut pc: u32 = 0x82C62D68;
    'dispatch: loop {
        match pc {
            0x82C62D68 => {
    //   block [0x82C62D68..0x82C62D74)
	// 82C62D68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C62D6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82C62D70: 4BFFFF90  b 0x82c62d00
	sub_82C62D00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C62D78 size=204
    let mut pc: u32 = 0x82C62D78;
    'dispatch: loop {
        match pc {
            0x82C62D78 => {
    //   block [0x82C62D78..0x82C62E44)
	// 82C62D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C62D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C62D80: DBC1FFE8  stfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[30].u64 ) };
	// 82C62D84: DBE1FFF0  stfd f31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 82C62D88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C62D8C: FD800890  fmr f12, f1
	ctx.f[12].f64 = ctx.f[1].f64;
	// 82C62D90: 4BFFFEE1  bl 0x82c62c70
	ctx.lr = 0x82C62D94;
	sub_82C62C70(ctx, base);
	// 82C62D94: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C62D98: 4BFFFF11  bl 0x82c62ca8
	ctx.lr = 0x82C62D9C;
	sub_82C62CA8(ctx, base);
	// 82C62D9C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C62DA0: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82C62DA4: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82C62DA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C62DAC: 419A0040  beq cr6, 0x82c62dec
	if ctx.cr[6].eq {
	pc = 0x82C62DEC; continue 'dispatch;
	}
	// 82C62DB0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C62DB4: C02B0C18  lfs f1, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C62DB8: FF1E0800  fcmpu cr6, f30, f1
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[1].f64);
	// 82C62DBC: 40990070  ble cr6, 0x82c62e2c
	if !ctx.cr[6].gt {
	pc = 0x82C62E2C; continue 'dispatch;
	}
	// 82C62DC0: FF1F0800  fcmpu cr6, f31, f1
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 82C62DC4: 40990068  ble cr6, 0x82c62e2c
	if !ctx.cr[6].gt {
	pc = 0x82C62E2C; continue 'dispatch;
	}
	// 82C62DC8: C0430024  lfs f2, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82C62DCC: FC206090  fmr f1, f12
	ctx.f[1].f64 = ctx.f[12].f64;
	// 82C62DD0: 4B59B6D9  bl 0x821fe4a8
	ctx.lr = 0x82C62DD4;
	sub_821FE4A8(ctx, base);
	// 82C62DD4: FC400818  frsp f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C62DD8: EC3EF824  fdivs f1, f30, f31
	ctx.f[1].f64 = ((ctx.f[30].f64 / ctx.f[31].f64) as f32) as f64;
	// 82C62DDC: 4B59B6CD  bl 0x821fe4a8
	ctx.lr = 0x82C62DE0;
	sub_821FE4A8(ctx, base);
	// 82C62DE0: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C62DE4: EC2007F2  fmuls f1, f0, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82C62DE8: 48000044  b 0x82c62e2c
	pc = 0x82C62E2C; continue 'dispatch;
	// 82C62DEC: 556B0672  rlwinm r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C62DF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C62DF4: 419A0030  beq cr6, 0x82c62e24
	if ctx.cr[6].eq {
	pc = 0x82C62E24; continue 'dispatch;
	}
	// 82C62DF8: ED7EF828  fsubs f11, f30, f31
	ctx.f[11].f64 = (((ctx.f[30].f64 - ctx.f[31].f64) as f32) as f64);
	// 82C62DFC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C62E00: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 82C62E04: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C62E08: C9AA13F8  lfd f13, 0x13f8(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(5112 as u32) ) };
	// 82C62E0C: ED4B002A  fadds f10, f11, f0
	ctx.f[10].f64 = ((ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64;
	// 82C62E10: FD2A6828  fsub f9, f10, f13
	ctx.f[9].f64 = ctx.f[10].f64 - ctx.f[13].f64;
	// 82C62E14: FC29FB3A  fmadd f1, f9, f12, f31
	ctx.f[1].f64 = ctx.f[9].f64 * ctx.f[12].f64 + ctx.f[31].f64;
	// 82C62E18: 4B5C9579  bl 0x8222c390
	ctx.lr = 0x82C62E1C;
	sub_8222C390(ctx, base);
	// 82C62E1C: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C62E20: 4800000C  b 0x82c62e2c
	pc = 0x82C62E2C; continue 'dispatch;
	// 82C62E24: EC1EF828  fsubs f0, f30, f31
	ctx.f[0].f64 = (((ctx.f[30].f64 - ctx.f[31].f64) as f32) as f64);
	// 82C62E28: EC20FB3A  fmadds f1, f0, f12, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[12].f64 + ctx.f[31].f64) as f32) as f64);
	// 82C62E2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C62E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C62E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C62E38: CBC1FFE8  lfd f30, -0x18(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C62E3C: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C62E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C62E48 size=244
    let mut pc: u32 = 0x82C62E48;
    'dispatch: loop {
        match pc {
            0x82C62E48 => {
    //   block [0x82C62E48..0x82C62F3C)
	// 82C62E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C62E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C62E50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C62E54: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 82C62E58: 4804AE7D  bl 0x82cadcd4
	ctx.lr = 0x82C62E5C;
	sub_82CADCA0(ctx, base);
	// 82C62E5C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C62E60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C62E64: FDA00890  fmr f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ctx.f[1].f64;
	// 82C62E68: 4BFFFE09  bl 0x82c62c70
	ctx.lr = 0x82C62E6C;
	sub_82C62C70(ctx, base);
	// 82C62E6C: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 82C62E70: 4BFFFE39  bl 0x82c62ca8
	ctx.lr = 0x82C62E74;
	sub_82C62CA8(ctx, base);
	// 82C62E74: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C62E78: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C62E7C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C62E80: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C62E84: 556807FE  clrlwi r8, r11, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82C62E88: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82C62E8C: C3AA0C14  lfs f29, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82C62E90: C3C90C18  lfs f30, 0xc18(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3096 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82C62E94: 419A0050  beq cr6, 0x82c62ee4
	if ctx.cr[6].eq {
	pc = 0x82C62EE4; continue 'dispatch;
	}
	// 82C62E98: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 82C62E9C: 40990070  ble cr6, 0x82c62f0c
	if !ctx.cr[6].gt {
	pc = 0x82C62F0C; continue 'dispatch;
	}
	// 82C62EA0: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 82C62EA4: 40990068  ble cr6, 0x82c62f0c
	if !ctx.cr[6].gt {
	pc = 0x82C62F0C; continue 'dispatch;
	}
	// 82C62EA8: FF0DF000  fcmpu cr6, f13, f30
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[30].f64);
	// 82C62EAC: 419A0060  beq cr6, 0x82c62f0c
	if ctx.cr[6].eq {
	pc = 0x82C62F0C; continue 'dispatch;
	}
	// 82C62EB0: EF9D0024  fdivs f28, f29, f0
	ctx.f[28].f64 = ((ctx.f[29].f64 / ctx.f[0].f64) as f32) as f64;
	// 82C62EB4: EC3C0372  fmuls f1, f28, f13
	ctx.f[1].f64 = (((ctx.f[28].f64 * ctx.f[13].f64) as f32) as f64);
	// 82C62EB8: 4B590EF9  bl 0x821f3db0
	ctx.lr = 0x82C62EBC;
	sub_821F3DB0(ctx, base);
	// 82C62EBC: FF600818  frsp f27, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[27].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C62EC0: EC3C07F2  fmuls f1, f28, f31
	ctx.f[1].f64 = (((ctx.f[28].f64 * ctx.f[31].f64) as f32) as f64);
	// 82C62EC4: 4B590EED  bl 0x821f3db0
	ctx.lr = 0x82C62EC8;
	sub_821F3DB0(ctx, base);
	// 82C62EC8: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C62ECC: C1BF0024  lfs f13, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C62ED0: EC5D6824  fdivs f2, f29, f13
	ctx.f[2].f64 = ((ctx.f[29].f64 / ctx.f[13].f64) as f32) as f64;
	// 82C62ED4: EC3B0024  fdivs f1, f27, f0
	ctx.f[1].f64 = ((ctx.f[27].f64 / ctx.f[0].f64) as f32) as f64;
	// 82C62ED8: 4B59B5D1  bl 0x821fe4a8
	ctx.lr = 0x82C62EDC;
	sub_821FE4A8(ctx, base);
	// 82C62EDC: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C62EE0: 48000024  b 0x82c62f04
	pc = 0x82C62F04; continue 'dispatch;
	// 82C62EE4: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82C62EE8: 409A0010  bne cr6, 0x82c62ef8
	if !ctx.cr[6].eq {
	pc = 0x82C62EF8; continue 'dispatch;
	}
	// 82C62EEC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C62EF0: C02B0C18  lfs f1, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C62EF4: 4800002C  b 0x82c62f20
	pc = 0x82C62F20; continue 'dispatch;
	// 82C62EF8: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C62EFC: ED9F0028  fsubs f12, f31, f0
	ctx.f[12].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C62F00: EC2D6024  fdivs f1, f13, f12
	ctx.f[1].f64 = ((ctx.f[13].f64 / ctx.f[12].f64) as f32) as f64;
	// 82C62F04: FF01F000  fcmpu cr6, f1, f30
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[30].f64);
	// 82C62F08: 4098000C  bge cr6, 0x82c62f14
	if !ctx.cr[6].lt {
	pc = 0x82C62F14; continue 'dispatch;
	}
	// 82C62F0C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82C62F10: 48000010  b 0x82c62f20
	pc = 0x82C62F20; continue 'dispatch;
	// 82C62F14: FF01E800  fcmpu cr6, f1, f29
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 82C62F18: 40990008  ble cr6, 0x82c62f20
	if !ctx.cr[6].gt {
	pc = 0x82C62F20; continue 'dispatch;
	}
	// 82C62F1C: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82C62F20: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C62F24: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 82C62F28: 4804ADF9  bl 0x82cadd20
	ctx.lr = 0x82C62F2C;
	sub_82CADCEC(ctx, base);
	// 82C62F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C62F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C62F34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C62F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C62F40 size=40
    let mut pc: u32 = 0x82C62F40;
    'dispatch: loop {
        match pc {
            0x82C62F40 => {
    //   block [0x82C62F40..0x82C62F68)
	// 82C62F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C62F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C62F48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C62F4C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82C62F50: 48309E09  bl 0x82f6cd58
	ctx.lr = 0x82C62F54;
	sub_82F6CD58(ctx, base);
	// 82C62F54: 4BFFFEF5  bl 0x82c62e48
	ctx.lr = 0x82C62F58;
	sub_82C62E48(ctx, base);
	// 82C62F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C62F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C62F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C62F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C62F68 size=128
    let mut pc: u32 = 0x82C62F68;
    'dispatch: loop {
        match pc {
            0x82C62F68 => {
    //   block [0x82C62F68..0x82C62FE8)
	// 82C62F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C62F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C62F70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C62F74: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82C62F78: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C62F7C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82C62F80: 48309DD9  bl 0x82f6cd58
	ctx.lr = 0x82C62F84;
	sub_82F6CD58(ctx, base);
	// 82C62F84: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C62F88: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C62F8C: D0210054  stfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C62F90: 556806B4  rlwinm r8, r11, 0, 0x1a, 0x1a
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C62F94: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82C62F98: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82C62F9C: 419A000C  beq cr6, 0x82c62fa8
	if ctx.cr[6].eq {
	pc = 0x82C62FA8; continue 'dispatch;
	}
	// 82C62FA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82C62FA4: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82C62FA8: 556B05EE  rlwinm r11, r11, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C62FAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C62FB0: 419A000C  beq cr6, 0x82c62fbc
	if ctx.cr[6].eq {
	pc = 0x82C62FBC; continue 'dispatch;
	}
	// 82C62FB4: 612B0002  ori r11, r9, 2
	ctx.r[11].u64 = ctx.r[9].u64 | 2;
	// 82C62FB8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82C62FBC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C62FC0: 419A0018  beq cr6, 0x82c62fd8
	if ctx.cr[6].eq {
	pc = 0x82C62FD8; continue 'dispatch;
	}
	// 82C62FC4: 806A001C  lwz r3, 0x1c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C62FC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C62FCC: 419A000C  beq cr6, 0x82c62fd8
	if ctx.cr[6].eq {
	pc = 0x82C62FD8; continue 'dispatch;
	}
	// 82C62FD0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C62FD4: 48000745  bl 0x82c63718
	ctx.lr = 0x82C62FD8;
	sub_82C63718(ctx, base);
	// 82C62FD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C62FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C62FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C62FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C62FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C62FE8 size=68
    let mut pc: u32 = 0x82C62FE8;
    'dispatch: loop {
        match pc {
            0x82C62FE8 => {
    //   block [0x82C62FE8..0x82C6302C)
	// 82C62FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C62FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C62FF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C62FF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C62FF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C62FFC: 4BFFFF6D  bl 0x82c62f68
	ctx.lr = 0x82C63000;
	sub_82C62F68(ctx, base);
	// 82C63000: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63004: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C63008: 4BFFFB89  bl 0x82c62b90
	ctx.lr = 0x82C6300C;
	sub_82C62B90(ctx, base);
	// 82C6300C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C63010: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63014: 4BFFFBCD  bl 0x82c62be0
	ctx.lr = 0x82C63018;
	sub_82C62BE0(ctx, base);
	// 82C63018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C6301C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C63024: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C63028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63030 size=84
    let mut pc: u32 = 0x82C63030;
    'dispatch: loop {
        match pc {
            0x82C63030 => {
    //   block [0x82C63030..0x82C63084)
	// 82C63030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63038: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C6303C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63040: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C63044: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82C63048: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C6304C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C63050: 419A0020  beq cr6, 0x82c63070
	if ctx.cr[6].eq {
	pc = 0x82C63070; continue 'dispatch;
	}
	// 82C63054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C63058: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C6305C: 480044B5  bl 0x82c67510
	ctx.lr = 0x82C63060;
	sub_82C67510(ctx, base);
	// 82C63060: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C63064: 419A000C  beq cr6, 0x82c63070
	if ctx.cr[6].eq {
	pc = 0x82C63070; continue 'dispatch;
	}
	// 82C63068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C6306C: 4BFFFEFD  bl 0x82c62f68
	ctx.lr = 0x82C63070;
	sub_82C62F68(ctx, base);
	// 82C63070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C63074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C6307C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C63080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C63088 size=172
    let mut pc: u32 = 0x82C63088;
    'dispatch: loop {
        match pc {
            0x82C63088 => {
    //   block [0x82C63088..0x82C63134)
	// 82C63088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6308C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63090: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C63094: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C63098: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C6309C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C630A0: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 82C630A4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C630A8: 4BFFFC01  bl 0x82c62ca8
	ctx.lr = 0x82C630AC;
	sub_82C62CA8(ctx, base);
	// 82C630AC: FF000800  fcmpu cr6, f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 82C630B0: 4099000C  ble cr6, 0x82c630bc
	if !ctx.cr[6].gt {
	pc = 0x82C630BC; continue 'dispatch;
	}
	// 82C630B4: 4BFFFBF5  bl 0x82c62ca8
	ctx.lr = 0x82C630B8;
	sub_82C62CA8(ctx, base);
	// 82C630B8: 48000018  b 0x82c630d0
	pc = 0x82C630D0; continue 'dispatch;
	// 82C630BC: 4BFFFBB5  bl 0x82c62c70
	ctx.lr = 0x82C630C0;
	sub_82C62C70(ctx, base);
	// 82C630C0: FF000800  fcmpu cr6, f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 82C630C4: 40980010  bge cr6, 0x82c630d4
	if !ctx.cr[6].lt {
	pc = 0x82C630D4; continue 'dispatch;
	}
	// 82C630C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C630CC: 4BFFFBA5  bl 0x82c62c70
	ctx.lr = 0x82C630D0;
	sub_82C62C70(ctx, base);
	// 82C630D0: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 82C630D4: C1BF0018  lfs f13, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C630D8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C630DC: 419A0014  beq cr6, 0x82c630f0
	if ctx.cr[6].eq {
	pc = 0x82C630F0; continue 'dispatch;
	}
	// 82C630E0: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82C630E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C630E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C630EC: 4BFFFB15  bl 0x82c62c00
	ctx.lr = 0x82C630F0;
	sub_82C62C00(ctx, base);
	// 82C630F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C630F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C630F8: 4BFFFA79  bl 0x82c62b70
	ctx.lr = 0x82C630FC;
	sub_82C62B70(ctx, base);
	// 82C630FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C63100: 4BFFFA91  bl 0x82c62b90
	ctx.lr = 0x82C63104;
	sub_82C62B90(ctx, base);
	// 82C63104: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C63108: 556A0738  rlwinm r10, r11, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C6310C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C63110: 409A000C  bne cr6, 0x82c6311c
	if !ctx.cr[6].eq {
	pc = 0x82C6311C; continue 'dispatch;
	}
	// 82C63114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63118: 4BFFFED1  bl 0x82c62fe8
	ctx.lr = 0x82C6311C;
	sub_82C62FE8(ctx, base);
	// 82C6311C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C63120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C63128: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C6312C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C63130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C63138 size=252
    let mut pc: u32 = 0x82C63138;
    'dispatch: loop {
        match pc {
            0x82C63138 => {
    //   block [0x82C63138..0x82C63190)
	// 82C63138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6313C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63140: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C63144: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C63148: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82C6314C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63150: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C63154: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C63158: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C6315C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C63160: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82C63164: 419900B4  bgt cr6, 0x82c63218
	if ctx.cr[6].gt {
	pc = 0x82C63218; continue 'dispatch;
	}
	// 82C63168: 3D8082C6  lis r12, -0x7d3a
	ctx.r[12].s64 = -2100953088;
	// 82C6316C: 398C3180  addi r12, r12, 0x3180
	ctx.r[12].s64 = ctx.r[12].s64 + 12672;
	// 82C63170: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82C63174: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82C63178: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82C6317C: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82C63190; continue 'dispatch;
		},
		1 => {
	pc = 0x82C631BC; continue 'dispatch;
		},
		2 => {
	pc = 0x82C631DC; continue 'dispatch;
		},
		3 => {
	pc = 0x82C63200; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82C63180: 82C63190  lwz r22, 0x3190(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12688 as u32) ) } as u64;
	// 82C63184: 82C631BC  lwz r22, 0x31bc(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12732 as u32) ) } as u64;
	// 82C63188: 82C631DC  lwz r22, 0x31dc(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12764 as u32) ) } as u64;
	// 82C6318C: 82C63200  lwz r22, 0x3200(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12800 as u32) ) } as u64;
            }
            0x82C63190 => {
    //   block [0x82C63190..0x82C631BC)
	// 82C63190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63194: 4BFFFDAD  bl 0x82c62f40
	ctx.lr = 0x82C63198;
	sub_82C62F40(ctx, base);
	// 82C63198: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 82C6319C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C631A0: 41980008  blt cr6, 0x82c631a8
	if ctx.cr[6].lt {
	pc = 0x82C631A8; continue 'dispatch;
	}
	// 82C631A4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82C631A8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C631AC: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C631B0: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82C631B4: 4099FFB4  ble cr6, 0x82c63168
	if !ctx.cr[6].gt {
	pc = 0x82C63168; continue 'dispatch;
	}
	// 82C631B8: 48000060  b 0x82c63218
	pc = 0x82C63218; continue 'dispatch;
            }
            0x82C631BC => {
    //   block [0x82C631BC..0x82C631DC)
	// 82C631BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C631C0: 4BFFFD81  bl 0x82c62f40
	ctx.lr = 0x82C631C4;
	sub_82C62F40(ctx, base);
	// 82C631C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C631C8: C00B0A90  lfs f0, 0xa90(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2704 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C631CC: EC010028  fsubs f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C631D0: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82C631D4: 41980044  blt cr6, 0x82c63218
	if ctx.cr[6].lt {
	pc = 0x82C63218; continue 'dispatch;
	}
	// 82C631D8: 48000020  b 0x82c631f8
	pc = 0x82C631F8; continue 'dispatch;
            }
            0x82C631DC => {
    //   block [0x82C631DC..0x82C63200)
	// 82C631DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C631E0: 4BFFFD61  bl 0x82c62f40
	ctx.lr = 0x82C631E4;
	sub_82C62F40(ctx, base);
	// 82C631E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C631E8: C00B0A90  lfs f0, 0xa90(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2704 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C631EC: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 82C631F0: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82C631F4: 41990024  bgt cr6, 0x82c63218
	if ctx.cr[6].gt {
	pc = 0x82C63218; continue 'dispatch;
	}
	// 82C631F8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82C631FC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	pc = 0x82C63200; continue 'dispatch;
            }
            0x82C63200 => {
    //   block [0x82C63200..0x82C63234)
	// 82C63200: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63204: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C63208: 4BFFFB71  bl 0x82c62d78
	ctx.lr = 0x82C6320C;
	sub_82C62D78(ctx, base);
	// 82C6320C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63210: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C63214: 4BFFFE75  bl 0x82c63088
	ctx.lr = 0x82C63218;
	sub_82C63088(ctx, base);
	// 82C63218: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C6321C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C63224: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82C63228: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C6322C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C63230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63238 size=72
    let mut pc: u32 = 0x82C63238;
    'dispatch: loop {
        match pc {
            0x82C63238 => {
    //   block [0x82C63238..0x82C63280)
	// 82C63238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6323C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63240: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C63244: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63248: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C6324C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C63250: 48000BA9  bl 0x82c63df8
	ctx.lr = 0x82C63254;
	sub_82C63DF8(ctx, base);
	// 82C63254: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C63258: 556A077A  rlwinm r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C6325C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C63260: 419A000C  beq cr6, 0x82c6326c
	if ctx.cr[6].eq {
	pc = 0x82C6326C; continue 'dispatch;
	}
	// 82C63264: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63268: 4BFFFB11  bl 0x82c62d78
	ctx.lr = 0x82C6326C;
	sub_82C62D78(ctx, base);
	// 82C6326C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C63270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C63278: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C6327C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63280 size=72
    let mut pc: u32 = 0x82C63280;
    'dispatch: loop {
        match pc {
            0x82C63280 => {
    //   block [0x82C63280..0x82C632C8)
	// 82C63280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C6328C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63290: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C63294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82C63298: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82C6329C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82C632A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C632A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C632A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C632AC: 4BFFFA55  bl 0x82c62d00
	ctx.lr = 0x82C632B0;
	sub_82C62D00(ctx, base);
	// 82C632B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C632B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C632B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C632BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C632C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C632C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C632C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C632C8 size=76
    let mut pc: u32 = 0x82C632C8;
    'dispatch: loop {
        match pc {
            0x82C632C8 => {
    //   block [0x82C632C8..0x82C63314)
	// 82C632C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C632CC: 48046141  bl 0x82ca940c
	ctx.lr = 0x82C632D0;
	sub_82CA93D0(ctx, base);
	// 82C632D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C632D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C632D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C632DC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C632E0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C632E4: 556A077A  rlwinm r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C632E8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C632EC: 419A000C  beq cr6, 0x82c632f8
	if ctx.cr[6].eq {
	pc = 0x82C632F8; continue 'dispatch;
	}
	// 82C632F0: 4BFFFC51  bl 0x82c62f40
	ctx.lr = 0x82C632F4;
	sub_82C62F40(ctx, base);
	// 82C632F4: 48000008  b 0x82c632fc
	pc = 0x82C632FC; continue 'dispatch;
	// 82C632F8: 48309A61  bl 0x82f6cd58
	ctx.lr = 0x82C632FC;
	sub_82F6CD58(ctx, base);
	// 82C632FC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82C63300: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C63304: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C63308: 480009E1  bl 0x82c63ce8
	ctx.lr = 0x82C6330C;
	sub_82C63CE8(ctx, base);
	// 82C6330C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C63310: 4804614C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63318 size=8
    let mut pc: u32 = 0x82C63318;
    'dispatch: loop {
        match pc {
            0x82C63318 => {
    //   block [0x82C63318..0x82C63320)
	// 82C63318: 82CAE348  lwz r22, -0x1cb8(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7352 as u32) ) } as u64;
	// 82C6331C: 8200DBF0  lwz r16, -0x2410(0)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, -9232u32 ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C63320 size=200
    let mut pc: u32 = 0x82C63320;
    'dispatch: loop {
        match pc {
            0x82C63320 => {
    //   block [0x82C63320..0x82C633E8)
	// 82C63320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63324: 480460E1  bl 0x82ca9404
	ctx.lr = 0x82C63328;
	sub_82CA93D0(ctx, base);
	// 82C63328: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82C6332C: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82C63330: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63334: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C63338: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C6333C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C63340: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82C63344: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82C63348: 394BDBC4  addi r10, r11, -0x243c
	ctx.r[10].s64 = ctx.r[11].s64 + -9276;
	// 82C6334C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C63350: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C63354: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C63358: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82C6335C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82C63360: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82C63364: 48003905  bl 0x82c66c68
	ctx.lr = 0x82C63368;
	sub_82C66C68(ctx, base);
	// 82C63368: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C6336C: 387E0054  addi r3, r30, 0x54
	ctx.r[3].s64 = ctx.r[30].s64 + 84;
	// 82C63370: 48003BA9  bl 0x82c66f18
	ctx.lr = 0x82C63374;
	sub_82C66F18(ctx, base);
	// 82C63374: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82C63378: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82C6337C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C63380: 387E0068  addi r3, r30, 0x68
	ctx.r[3].s64 = ctx.r[30].s64 + 104;
	// 82C63384: 48004265  bl 0x82c675e8
	ctx.lr = 0x82C63388;
	sub_82C675E8(ctx, base);
	// 82C63388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C6338C: 395E0034  addi r10, r30, 0x34
	ctx.r[10].s64 = ctx.r[30].s64 + 52;
	// 82C63390: D3FE0014  stfs f31, 0x14(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82C63394: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82C63398: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82C6339C: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82C633A0: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82C633A4: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82C633A8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82C633AC: 913E0018  stw r9, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 82C633B0: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82C633B4: 939E001C  stw r28, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82C633B8: 937E0020  stw r27, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 82C633BC: 913E0030  stw r9, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 82C633C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C633C4: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C633C8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C633CC: 4200FFF8  bdnz 0x82c633c4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82C633C4; continue 'dispatch;
	}
	// 82C633D0: 48004191  bl 0x82c67560
	ctx.lr = 0x82C633D4;
	sub_82C67560(ctx, base);
	// 82C633D4: 907E0084  stw r3, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82C633D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C633DC: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82C633E0: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82C633E4: 48046070  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C633E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C633E8 size=44
    let mut pc: u32 = 0x82C633E8;
    'dispatch: loop {
        match pc {
            0x82C633E8 => {
    //   block [0x82C633E8..0x82C63414)
	// 82C633E8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82C633EC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C633F0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C633F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C633F8: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82C633FC: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C63400: 480038C1  bl 0x82c66cc0
	ctx.lr = 0x82C63404;
	sub_82C66CC0(ctx, base);
	// 82C63404: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C63408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C6340C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C63410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63414(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63414 size=44
    let mut pc: u32 = 0x82C63414;
    'dispatch: loop {
        match pc {
            0x82C63414 => {
    //   block [0x82C63414..0x82C63440)
	// 82C63414: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82C63418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6341C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63424: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82C63428: 386B0054  addi r3, r11, 0x54
	ctx.r[3].s64 = ctx.r[11].s64 + 84;
	// 82C6342C: 48003905  bl 0x82c66d30
	ctx.lr = 0x82C63430;
	sub_82C66D30(ctx, base);
	// 82C63430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C63434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C6343C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63440 size=8
    let mut pc: u32 = 0x82C63440;
    'dispatch: loop {
        match pc {
            0x82C63440 => {
    //   block [0x82C63440..0x82C63448)
	// 82C63440: 82CAE348  lwz r22, -0x1cb8(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7352 as u32) ) } as u64;
	// 82C63444: 8200DC48  lwz r16, -0x23b8(0)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, -9144u32 ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63448 size=140
    let mut pc: u32 = 0x82C63448;
    'dispatch: loop {
        match pc {
            0x82C63448 => {
    //   block [0x82C63448..0x82C634D4)
	// 82C63448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6344C: 48045FB9  bl 0x82ca9404
	ctx.lr = 0x82C63450;
	sub_82CA93D0(ctx, base);
	// 82C63450: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82C63454: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63458: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C6345C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C63460: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82C63464: 394BDBC4  addi r10, r11, -0x243c
	ctx.r[10].s64 = ctx.r[11].s64 + -9276;
	// 82C63468: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C6346C: 813D0020  lwz r9, 0x20(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C63470: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82C63474: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C63478: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 82C6347C: 40990030  ble cr6, 0x82c634ac
	if !ctx.cr[6].gt {
	pc = 0x82C634AC; continue 'dispatch;
	}
	// 82C63480: 3BDD0034  addi r30, r29, 0x34
	ctx.r[30].s64 = ctx.r[29].s64 + 52;
	// 82C63484: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C63488: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C6348C: 419A000C  beq cr6, 0x82c63498
	if ctx.cr[6].eq {
	pc = 0x82C63498; continue 'dispatch;
	}
	// 82C63490: 4BFFCBE9  bl 0x82c60078
	ctx.lr = 0x82C63494;
	sub_82C60078(ctx, base);
	// 82C63494: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82C63498: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C6349C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82C634A0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82C634A4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C634A8: 4198FFDC  blt cr6, 0x82c63484
	if ctx.cr[6].lt {
	pc = 0x82C63484; continue 'dispatch;
	}
	// 82C634AC: 807D0084  lwz r3, 0x84(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82C634B0: 48004119  bl 0x82c675c8
	ctx.lr = 0x82C634B4;
	sub_82C675C8(ctx, base);
	// 82C634B4: 387D0068  addi r3, r29, 0x68
	ctx.r[3].s64 = ctx.r[29].s64 + 104;
	// 82C634B8: 48004199  bl 0x82c67650
	ctx.lr = 0x82C634BC;
	sub_82C67650(ctx, base);
	// 82C634BC: 387D0054  addi r3, r29, 0x54
	ctx.r[3].s64 = ctx.r[29].s64 + 84;
	// 82C634C0: 48003871  bl 0x82c66d30
	ctx.lr = 0x82C634C4;
	sub_82C66D30(ctx, base);
	// 82C634C4: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 82C634C8: 480037F9  bl 0x82c66cc0
	ctx.lr = 0x82C634CC;
	sub_82C66CC0(ctx, base);
	// 82C634CC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82C634D0: 48045F84  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C634D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C634D4 size=44
    let mut pc: u32 = 0x82C634D4;
    'dispatch: loop {
        match pc {
            0x82C634D4 => {
    //   block [0x82C634D4..0x82C63500)
	// 82C634D4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82C634D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C634DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C634E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C634E4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82C634E8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C634EC: 480037D5  bl 0x82c66cc0
	ctx.lr = 0x82C634F0;
	sub_82C66CC0(ctx, base);
	// 82C634F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C634F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C634F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C634FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63500 size=44
    let mut pc: u32 = 0x82C63500;
    'dispatch: loop {
        match pc {
            0x82C63500 => {
    //   block [0x82C63500..0x82C6352C)
	// 82C63500: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82C63504: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63508: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C6350C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63510: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82C63514: 386B0054  addi r3, r11, 0x54
	ctx.r[3].s64 = ctx.r[11].s64 + 84;
	// 82C63518: 48003819  bl 0x82c66d30
	ctx.lr = 0x82C6351C;
	sub_82C66D30(ctx, base);
	// 82C6351C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C63520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C63528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C6352C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C6352C size=44
    let mut pc: u32 = 0x82C6352C;
    'dispatch: loop {
        match pc {
            0x82C6352C => {
    //   block [0x82C6352C..0x82C63558)
	// 82C6352C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82C63530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C6353C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82C63540: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 82C63544: 4800410D  bl 0x82c67650
	ctx.lr = 0x82C63548;
	sub_82C67650(ctx, base);
	// 82C63548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C6354C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C63554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63558 size=32
    let mut pc: u32 = 0x82C63558;
    'dispatch: loop {
        match pc {
            0x82C63558 => {
    //   block [0x82C63558..0x82C63578)
	// 82C63558: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6355C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C63560: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82C63564: 9143002C  stw r10, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82C63568: 91230030  stw r9, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 82C6356C: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C63570: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C63574: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63578 size=28
    let mut pc: u32 = 0x82C63578;
    'dispatch: loop {
        match pc {
            0x82C63578 => {
    //   block [0x82C63578..0x82C63594)
	// 82C63578: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C6357C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82C63580: 616A0001  ori r10, r11, 1
	ctx.r[10].u64 = ctx.r[11].u64 | 1;
	// 82C63584: 409A0008  bne cr6, 0x82c6358c
	if !ctx.cr[6].eq {
	pc = 0x82C6358C; continue 'dispatch;
	}
	// 82C63588: 556A003C  rlwinm r10, r11, 0, 0, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C6358C: 91430018  stw r10, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82C63590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63598 size=32
    let mut pc: u32 = 0x82C63598;
    'dispatch: loop {
        match pc {
            0x82C63598 => {
    //   block [0x82C63598..0x82C635B8)
	// 82C63598: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82C6359C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82C635A0: 816A0018  lwz r11, 0x18(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C635A4: 5563F7FE  rlwinm r3, r11, 0x1e, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82C635A8: 419A0010  beq cr6, 0x82c635b8
	if ctx.cr[6].eq {
		sub_82C635B8(ctx, base);
		return;
	}
	// 82C635AC: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 82C635B0: 916A0018  stw r11, 0x18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82C635B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C635B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C635B8 size=12
    let mut pc: u32 = 0x82C635B8;
    'dispatch: loop {
        match pc {
            0x82C635B8 => {
    //   block [0x82C635B8..0x82C635C4)
	// 82C635B8: 556B07B8  rlwinm r11, r11, 0, 0x1e, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C635BC: 916A0018  stw r11, 0x18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82C635C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C635C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C635C8 size=32
    let mut pc: u32 = 0x82C635C8;
    'dispatch: loop {
        match pc {
            0x82C635C8 => {
    //   block [0x82C635C8..0x82C635E8)
	// 82C635C8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82C635CC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82C635D0: 816A0018  lwz r11, 0x18(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C635D4: 5563EFFE  rlwinm r3, r11, 0x1d, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82C635D8: 419A0010  beq cr6, 0x82c635e8
	if ctx.cr[6].eq {
		sub_82C635E8(ctx, base);
		return;
	}
	// 82C635DC: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 82C635E0: 916A0018  stw r11, 0x18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82C635E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C635E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C635E8 size=12
    let mut pc: u32 = 0x82C635E8;
    'dispatch: loop {
        match pc {
            0x82C635E8 => {
    //   block [0x82C635E8..0x82C635F4)
	// 82C635E8: 556B0776  rlwinm r11, r11, 0, 0x1d, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C635EC: 916A0018  stw r11, 0x18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82C635F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C635F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C635F8 size=20
    let mut pc: u32 = 0x82C635F8;
    'dispatch: loop {
        match pc {
            0x82C635F8 => {
    //   block [0x82C635F8..0x82C6360C)
	// 82C635F8: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82C635FC: 40980010  bge cr6, 0x82c6360c
	if !ctx.cr[6].lt {
		sub_82C6360C(ctx, base);
		return;
	}
	// 82C63600: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C63604: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82C63608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C6360C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C6360C size=8
    let mut pc: u32 = 0x82C6360C;
    'dispatch: loop {
        match pc {
            0x82C6360C => {
    //   block [0x82C6360C..0x82C63614)
	// 82C6360C: 90830028  stw r4, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[4].u32 ) };
	// 82C63610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63618 size=100
    let mut pc: u32 = 0x82C63618;
    'dispatch: loop {
        match pc {
            0x82C63618 => {
    //   block [0x82C63618..0x82C6367C)
	// 82C63618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6361C: 48045DF1  bl 0x82ca940c
	ctx.lr = 0x82C63620;
	sub_82CA93D0(ctx, base);
	// 82C63620: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63624: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C63628: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C6362C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C63630: 409A000C  bne cr6, 0x82c6363c
	if !ctx.cr[6].eq {
	pc = 0x82C6363C; continue 'dispatch;
	}
	// 82C63634: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 82C63638: 4BFFFFC1  bl 0x82c635f8
	ctx.lr = 0x82C6363C;
	sub_82C635F8(ctx, base);
	// 82C6363C: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C63640: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C63644: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C63648: 4099002C  ble cr6, 0x82c63674
	if !ctx.cr[6].gt {
	pc = 0x82C63674; continue 'dispatch;
	}
	// 82C6364C: 3BFE0034  addi r31, r30, 0x34
	ctx.r[31].s64 = ctx.r[30].s64 + 52;
	// 82C63650: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C63654: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C63658: 4BFFC989  bl 0x82c5ffe0
	ctx.lr = 0x82C6365C;
	sub_82C5FFE0(ctx, base);
	// 82C6365C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82C63660: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C63664: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82C63668: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C6366C: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82C63670: 4198FFE0  blt cr6, 0x82c63650
	if ctx.cr[6].lt {
	pc = 0x82C63650; continue 'dispatch;
	}
	// 82C63674: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C63678: 48045DE4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C63680 size=116
    let mut pc: u32 = 0x82C63680;
    'dispatch: loop {
        match pc {
            0x82C63680 => {
    //   block [0x82C63680..0x82C636F4)
	// 82C63680: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 82C63684: 41980068  blt cr6, 0x82c636ec
	if ctx.cr[6].lt {
	pc = 0x82C636EC; continue 'dispatch;
	}
	// 82C63688: 3965FFFC  addi r11, r5, -4
	ctx.r[11].s64 = ctx.r[5].s64 + -4;
	// 82C6368C: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C63690: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C63694: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C63698: 7CAA2850  subf r5, r10, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 82C6369C: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C636A0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C636A4: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C636A8: ED8D002A  fadds f12, f13, f0
	ctx.f[12].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82C636AC: C1630004  lfs f11, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82C636B0: D1830000  stfs f12, 0(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C636B4: C1440004  lfs f10, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82C636B8: ED2A582A  fadds f9, f10, f11
	ctx.f[9].f64 = ((ctx.f[10].f64 + ctx.f[11].f64) as f32) as f64;
	// 82C636BC: C1030008  lfs f8, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82C636C0: D1230004  stfs f9, 4(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C636C4: C0E40008  lfs f7, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82C636C8: ECC7402A  fadds f6, f7, f8
	ctx.f[6].f64 = ((ctx.f[7].f64 + ctx.f[8].f64) as f32) as f64;
	// 82C636CC: C0A3000C  lfs f5, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82C636D0: D0C30008  stfs f6, 8(r3)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C636D4: C084000C  lfs f4, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82C636D8: EC64282A  fadds f3, f4, f5
	ctx.f[3].f64 = ((ctx.f[4].f64 + ctx.f[5].f64) as f32) as f64;
	// 82C636DC: D063000C  stfs f3, 0xc(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C636E0: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82C636E4: 38840010  addi r4, r4, 0x10
	ctx.r[4].s64 = ctx.r[4].s64 + 16;
	// 82C636E8: 4082FFB4  bne 0x82c6369c
	if !ctx.cr[0].eq {
	pc = 0x82C6369C; continue 'dispatch;
	}
	// 82C636EC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82C636F0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C636F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C636F4 size=36
    let mut pc: u32 = 0x82C636F4;
    'dispatch: loop {
        match pc {
            0x82C636F4 => {
    //   block [0x82C636F4..0x82C63718)
	// 82C636F4: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C636F8: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82C636FC: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C63700: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82C63704: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82C63708: D1830000  stfs f12, 0(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C6370C: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82C63710: 4181FFE4  bgt 0x82c636f4
	if ctx.cr[0].gt {
	pc = 0x82C636F4; continue 'dispatch;
	}
	// 82C63714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63718 size=80
    let mut pc: u32 = 0x82C63718;
    'dispatch: loop {
        match pc {
            0x82C63718 => {
    //   block [0x82C63718..0x82C63768)
	// 82C63718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6371C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63720: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C63724: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C63728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C6372C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C63730: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C63734: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82C63738: 48003E51  bl 0x82c67588
	ctx.lr = 0x82C6373C;
	sub_82C67588(ctx, base);
	// 82C6373C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C63740: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 82C63744: 480035F5  bl 0x82c66d38
	ctx.lr = 0x82C63748;
	sub_82C66D38(ctx, base);
	// 82C63748: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82C6374C: 48003E5D  bl 0x82c675a8
	ctx.lr = 0x82C63750;
	sub_82C675A8(ctx, base);
	// 82C63750: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C63754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C6375C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C63760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C63764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63768 size=16
    let mut pc: u32 = 0x82C63768;
    'dispatch: loop {
        match pc {
            0x82C63768 => {
    //   block [0x82C63768..0x82C63778)
	// 82C63768: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82C6376C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C63770: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 82C63774: 48003EF4  b 0x82c67668
	sub_82C67668(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C63778 size=188
    let mut pc: u32 = 0x82C63778;
    'dispatch: loop {
        match pc {
            0x82C63778 => {
    //   block [0x82C63778..0x82C63834)
	// 82C63778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6377C: 48045C8D  bl 0x82ca9408
	ctx.lr = 0x82C63780;
	sub_82CA93D0(ctx, base);
	// 82C63780: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C63788: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82C6378C: 48003DFD  bl 0x82c67588
	ctx.lr = 0x82C63790;
	sub_82C67588(ctx, base);
	// 82C63790: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C63794: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C63798: 40990018  ble cr6, 0x82c637b0
	if !ctx.cr[6].gt {
	pc = 0x82C637B0; continue 'dispatch;
	}
	// 82C6379C: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 82C637A0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C637A4: 388B3768  addi r4, r11, 0x3768
	ctx.r[4].s64 = ctx.r[11].s64 + 14184;
	// 82C637A8: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 82C637AC: 48003675  bl 0x82c66e20
	ctx.lr = 0x82C637B0;
	sub_82C66E20(ctx, base);
	// 82C637B0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82C637B4: 48003DF5  bl 0x82c675a8
	ctx.lr = 0x82C637B8;
	sub_82C675A8(ctx, base);
	// 82C637B8: 3BDF0068  addi r30, r31, 0x68
	ctx.r[30].s64 = ctx.r[31].s64 + 104;
	// 82C637BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C637C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C637C4: 48003F65  bl 0x82c67728
	ctx.lr = 0x82C637C8;
	sub_82C67728(ctx, base);
	// 82C637C8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C637CC: 419A0060  beq cr6, 0x82c6382c
	if ctx.cr[6].eq {
	pc = 0x82C6382C; continue 'dispatch;
	}
	// 82C637D0: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C637D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C637D8: 552407FE  clrlwi r4, r9, 0x1f
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82C637DC: 4BFFFDBD  bl 0x82c63598
	ctx.lr = 0x82C637E0;
	sub_82C63598(ctx, base);
	// 82C637E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C637E4: 5524FFFE  rlwinm r4, r9, 0x1f, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82C637E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C637EC: 4BFFFDDD  bl 0x82c635c8
	ctx.lr = 0x82C637F0;
	sub_82C635C8(ctx, base);
	// 82C637F0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C637F4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C637F8: C0210054  lfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C637FC: 4800340D  bl 0x82c66c08
	ctx.lr = 0x82C63800;
	sub_82C66C08(ctx, base);
	// 82C63800: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C63804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63808: 4BFFFD91  bl 0x82c63598
	ctx.lr = 0x82C6380C;
	sub_82C63598(ctx, base);
	// 82C6380C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C63810: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63814: 4BFFFDB5  bl 0x82c635c8
	ctx.lr = 0x82C63818;
	sub_82C635C8(ctx, base);
	// 82C63818: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C6381C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C63820: 48003F09  bl 0x82c67728
	ctx.lr = 0x82C63824;
	sub_82C67728(ctx, base);
	// 82C63824: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C63828: 409AFFA8  bne cr6, 0x82c637d0
	if !ctx.cr[6].eq {
	pc = 0x82C637D0; continue 'dispatch;
	}
	// 82C6382C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C63830: 48045C28  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63838 size=20
    let mut pc: u32 = 0x82C63838;
    'dispatch: loop {
        match pc {
            0x82C63838 => {
    //   block [0x82C63838..0x82C6384C)
	// 82C63838: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6383C: 90830024  stw r4, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[4].u32 ) };
	// 82C63840: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C63844: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C63848: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63850 size=476
    let mut pc: u32 = 0x82C63850;
    'dispatch: loop {
        match pc {
            0x82C63850 => {
    //   block [0x82C63850..0x82C63A2C)
	// 82C63850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63854: 48045BB1  bl 0x82ca9404
	ctx.lr = 0x82C63858;
	sub_82CA93D0(ctx, base);
	// 82C63858: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C6385C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C63860: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C63864: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C63868: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82C6386C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C63870: 556A07BC  rlwinm r10, r11, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C63874: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C63878: 419A0174  beq cr6, 0x82c639ec
	if ctx.cr[6].eq {
	pc = 0x82C639EC; continue 'dispatch;
	}
	// 82C6387C: 83BF0024  lwz r29, 0x24(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C63880: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C63884: 7D6BEA15  add. r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C63888: 4081001C  ble 0x82c638a4
	if !ctx.cr[0].gt {
	pc = 0x82C638A4; continue 'dispatch;
	}
	// 82C6388C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C63890: 57AA003E  slwi r10, r29, 0
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C63894: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C63898: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C6389C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82C638A0: 48046069  bl 0x82ca9908
	ctx.lr = 0x82C638A4;
	sub_82CA9908(ctx, base);
	// 82C638A4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C638A8: 7D6BEA15  add. r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C638AC: 40810020  ble 0x82c638cc
	if !ctx.cr[0].gt {
	pc = 0x82C638CC; continue 'dispatch;
	}
	// 82C638B0: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C638B4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C638B8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C638BC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C638C0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C638C4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82C638C8: 48046041  bl 0x82ca9908
	ctx.lr = 0x82C638CC;
	sub_82CA9908(ctx, base);
	// 82C638CC: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82C638D0: 40990154  ble cr6, 0x82c63a24
	if !ctx.cr[6].gt {
	pc = 0x82C63A24; continue 'dispatch;
	}
	// 82C638D4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C638D8: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C638DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C638E0: 409A0018  bne cr6, 0x82c638f8
	if !ctx.cr[6].eq {
	pc = 0x82C638F8; continue 'dispatch;
	}
	// 82C638E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C638E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C638EC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C638F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C638F4: 4E800421  bctrl
	ctx.lr = 0x82C638F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C638F8: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C638FC: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C63900: 7FCA5850  subf r30, r10, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C63904: 7F1ED800  cmpw cr6, r30, r27
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82C63908: 41980008  blt cr6, 0x82c63910
	if ctx.cr[6].lt {
	pc = 0x82C63910; continue 'dispatch;
	}
	// 82C6390C: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82C63910: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C63914: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C63918: 419A000C  beq cr6, 0x82c63924
	if ctx.cr[6].eq {
	pc = 0x82C63924; continue 'dispatch;
	}
	// 82C6391C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63920: 4BFFFE59  bl 0x82c63778
	ctx.lr = 0x82C63924;
	sub_82C63778(ctx, base);
	// 82C63924: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C63928: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C6392C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82C63930: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C63934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63938: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C6393C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C63940: 4E800421  bctrl
	ctx.lr = 0x82C63944;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C63944: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C63948: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C6394C: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82C63950: 7D2B3A15  add. r9, r11, r7
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C63954: 40810030  ble 0x82c63984
	if !ctx.cr[0].gt {
	pc = 0x82C63984; continue 'dispatch;
	}
	// 82C63958: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C6395C: 54E9003E  slwi r9, r7, 0
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C63960: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C63964: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82C63968: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82C6396C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C63970: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C63974: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82C63978: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C6397C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C63980: 4082FFEC  bne 0x82c6396c
	if !ctx.cr[0].eq {
	pc = 0x82C6396C; continue 'dispatch;
	}
	// 82C63984: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C63988: 7D6B3A15  add. r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C6398C: 40810030  ble 0x82c639bc
	if !ctx.cr[0].gt {
	pc = 0x82C639BC; continue 'dispatch;
	}
	// 82C63990: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C63994: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C63998: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C6399C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82C639A0: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82C639A4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C639A8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C639AC: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82C639B0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C639B4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C639B8: 4082FFEC  bne 0x82c639a4
	if !ctx.cr[0].eq {
	pc = 0x82C639A4; continue 'dispatch;
	}
	// 82C639BC: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C639C0: 7F7ED850  subf r27, r30, r27
	ctx.r[27].s64 = ctx.r[27].s64 - ctx.r[30].s64;
	// 82C639C4: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C639C8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82C639CC: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82C639D0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82C639D4: 41980008  blt cr6, 0x82c639dc
	if ctx.cr[6].lt {
	pc = 0x82C639DC; continue 'dispatch;
	}
	// 82C639D8: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82C639DC: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82C639E0: 4199FEF8  bgt cr6, 0x82c638d8
	if ctx.cr[6].gt {
	pc = 0x82C638D8; continue 'dispatch;
	}
	// 82C639E4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82C639E8: 48045A6C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C639EC: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C639F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C639F4: 419A000C  beq cr6, 0x82c63a00
	if ctx.cr[6].eq {
	pc = 0x82C63A00; continue 'dispatch;
	}
	// 82C639F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C639FC: 4BFFFD7D  bl 0x82c63778
	ctx.lr = 0x82C63A00;
	sub_82C63778(ctx, base);
	// 82C63A00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C63A04: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82C63A08: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C63A0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C63A10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63A14: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C63A18: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C63A1C: 4E800421  bctrl
	ctx.lr = 0x82C63A20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C63A20: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82C63A24: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82C63A28: 48045A2C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63A30 size=460
    let mut pc: u32 = 0x82C63A30;
    'dispatch: loop {
        match pc {
            0x82C63A30 => {
    //   block [0x82C63A30..0x82C63BFC)
	// 82C63A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63A34: 480459D1  bl 0x82ca9404
	ctx.lr = 0x82C63A38;
	sub_82CA93D0(ctx, base);
	// 82C63A38: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63A3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C63A40: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C63A44: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82C63A48: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C63A4C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C63A50: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C63A54: 4081001C  ble 0x82c63a70
	if !ctx.cr[0].gt {
	pc = 0x82C63A70; continue 'dispatch;
	}
	// 82C63A58: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C63A5C: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C63A60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C63A64: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C63A68: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82C63A6C: 48045E9D  bl 0x82ca9908
	ctx.lr = 0x82C63A70;
	sub_82CA9908(ctx, base);
	// 82C63A70: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C63A74: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C63A78: 40990018  ble cr6, 0x82c63a90
	if !ctx.cr[6].gt {
	pc = 0x82C63A90; continue 'dispatch;
	}
	// 82C63A7C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C63A80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C63A84: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C63A88: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82C63A8C: 48045E7D  bl 0x82ca9908
	ctx.lr = 0x82C63A90;
	sub_82CA9908(ctx, base);
	// 82C63A90: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82C63A94: 40990160  ble cr6, 0x82c63bf4
	if !ctx.cr[6].gt {
	pc = 0x82C63BF4; continue 'dispatch;
	}
	// 82C63A98: 3B9F0034  addi r28, r31, 0x34
	ctx.r[28].s64 = ctx.r[31].s64 + 52;
	// 82C63A9C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82C63AA0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C63AA4: 556A07BC  rlwinm r10, r11, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C63AA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C63AAC: 419A0024  beq cr6, 0x82c63ad0
	if ctx.cr[6].eq {
	pc = 0x82C63AD0; continue 'dispatch;
	}
	// 82C63AB0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C63AB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C63AB8: 409A0018  bne cr6, 0x82c63ad0
	if !ctx.cr[6].eq {
	pc = 0x82C63AD0; continue 'dispatch;
	}
	// 82C63ABC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C63AC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63AC4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C63AC8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C63ACC: 4E800421  bctrl
	ctx.lr = 0x82C63AD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C63AD0: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C63AD4: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C63AD8: 7FCA5850  subf r30, r10, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C63ADC: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82C63AE0: 41980008  blt cr6, 0x82c63ae8
	if ctx.cr[6].lt {
	pc = 0x82C63AE8; continue 'dispatch;
	}
	// 82C63AE4: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82C63AE8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C63AEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C63AF0: 419A000C  beq cr6, 0x82c63afc
	if ctx.cr[6].eq {
	pc = 0x82C63AFC; continue 'dispatch;
	}
	// 82C63AF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63AF8: 4BFFFC81  bl 0x82c63778
	ctx.lr = 0x82C63AFC;
	sub_82C63778(ctx, base);
	// 82C63AFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C63B00: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C63B04: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C63B08: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C63B0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63B10: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C63B14: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C63B18: 4E800421  bctrl
	ctx.lr = 0x82C63B1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C63B1C: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C63B20: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82C63B24: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82C63B28: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C63B2C: 40990034  ble cr6, 0x82c63b60
	if !ctx.cr[6].gt {
	pc = 0x82C63B60; continue 'dispatch;
	}
	// 82C63B30: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82C63B34: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82C63B38: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C63B3C: 80890000  lwz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C63B40: 80680000  lwz r3, 0(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C63B44: 4BFFFB3D  bl 0x82c63680
	ctx.lr = 0x82C63B48;
	sub_82C63680(ctx, base);
	// 82C63B48: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C63B4C: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82C63B50: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82C63B54: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82C63B58: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C63B5C: 4198FFDC  blt cr6, 0x82c63b38
	if ctx.cr[6].lt {
	pc = 0x82C63B38; continue 'dispatch;
	}
	// 82C63B60: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C63B64: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C63B68: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C63B6C: 40810030  ble 0x82c63b9c
	if !ctx.cr[0].gt {
	pc = 0x82C63B9C; continue 'dispatch;
	}
	// 82C63B70: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C63B74: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C63B78: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C63B7C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82C63B80: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82C63B84: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C63B88: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C63B8C: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82C63B90: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C63B94: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C63B98: 4082FFEC  bne 0x82c63b84
	if !ctx.cr[0].eq {
	pc = 0x82C63B84; continue 'dispatch;
	}
	// 82C63B9C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C63BA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C63BA4: 40990028  ble cr6, 0x82c63bcc
	if !ctx.cr[6].gt {
	pc = 0x82C63BCC; continue 'dispatch;
	}
	// 82C63BA8: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C63BAC: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C63BB0: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82C63BB4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C63BB8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C63BBC: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82C63BC0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C63BC4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C63BC8: 4082FFEC  bne 0x82c63bb4
	if !ctx.cr[0].eq {
	pc = 0x82C63BB4; continue 'dispatch;
	}
	// 82C63BCC: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C63BD0: 7FBEE850  subf r29, r30, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82C63BD4: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C63BD8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82C63BDC: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82C63BE0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82C63BE4: 41980008  blt cr6, 0x82c63bec
	if ctx.cr[6].lt {
	pc = 0x82C63BEC; continue 'dispatch;
	}
	// 82C63BE8: 937F002C  stw r27, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[27].u32 ) };
	// 82C63BEC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82C63BF0: 4199FEB0  bgt cr6, 0x82c63aa0
	if ctx.cr[6].gt {
	pc = 0x82C63AA0; continue 'dispatch;
	}
	// 82C63BF4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82C63BF8: 4804585C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63C00 size=68
    let mut pc: u32 = 0x82C63C00;
    'dispatch: loop {
        match pc {
            0x82C63C00 => {
    //   block [0x82C63C00..0x82C63C44)
	// 82C63C00: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C63C04: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C63C08: 396B0CA0  addi r11, r11, 0xca0
	ctx.r[11].s64 = ctx.r[11].s64 + 3232;
	// 82C63C0C: 409A0008  bne cr6, 0x82c63c14
	if !ctx.cr[6].eq {
	pc = 0x82C63C14; continue 'dispatch;
	}
	// 82C63C10: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82C63C14: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82C63C18: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C63C1C: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82C63C20: 409A0008  bne cr6, 0x82c63c28
	if !ctx.cr[6].eq {
	pc = 0x82C63C28; continue 'dispatch;
	}
	// 82C63C24: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82C63C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C63C2C: 90C30018  stw r6, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 82C63C30: 90E30004  stw r7, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82C63C34: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82C63C38: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82C63C3C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C63C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63C48 size=24
    let mut pc: u32 = 0x82C63C48;
    'dispatch: loop {
        match pc {
            0x82C63C48 => {
    //   block [0x82C63C48..0x82C63C60)
	// 82C63C48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C63C4C: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82C63C50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C63C54: 419A000C  beq cr6, 0x82c63c60
	if ctx.cr[6].eq {
		sub_82C63C60(ctx, base);
		return;
	}
	// 82C63C58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C63C5C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63C60 size=16
    let mut pc: u32 = 0x82C63C60;
    'dispatch: loop {
        match pc {
            0x82C63C60 => {
    //   block [0x82C63C60..0x82C63C70)
	// 82C63C60: 80830014  lwz r4, 0x14(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C63C64: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82C63C68: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82C63C6C: 4BFFD31C  b 0x82c60f88
	sub_82C60F88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63C70 size=116
    let mut pc: u32 = 0x82C63C70;
    'dispatch: loop {
        match pc {
            0x82C63C70 => {
    //   block [0x82C63C70..0x82C63CE4)
	// 82C63C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63C78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C63C7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C63C80: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63C84: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C63C88: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82C63C8C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82C63C90: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C63C94: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C63C98: 419A0010  beq cr6, 0x82c63ca8
	if ctx.cr[6].eq {
	pc = 0x82C63CA8; continue 'dispatch;
	}
	// 82C63C9C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C63CA0: 4E800421  bctrl
	ctx.lr = 0x82C63CA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C63CA4: 48000028  b 0x82c63ccc
	pc = 0x82C63CCC; continue 'dispatch;
	// 82C63CA8: D8210020  stfd f1, 0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.f[1].u64 ) };
	// 82C63CAC: E8A10020  ld r5, 0x20(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) };
	// 82C63CB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C63CB4: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C63CB8: 4B5DBBE9  bl 0x8223f8a0
	ctx.lr = 0x82C63CBC;
	sub_8223F8A0(ctx, base);
	// 82C63CBC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C63CC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C63CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63CC8: 4BFFD2C1  bl 0x82c60f88
	ctx.lr = 0x82C63CCC;
	sub_82C60F88(ctx, base);
	// 82C63CCC: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82C63CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C63CD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C63CDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C63CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63CE8 size=264
    let mut pc: u32 = 0x82C63CE8;
    'dispatch: loop {
        match pc {
            0x82C63CE8 => {
    //   block [0x82C63CE8..0x82C63DF0)
	// 82C63CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63CEC: 48045721  bl 0x82ca940c
	ctx.lr = 0x82C63CF0;
	sub_82CA93D0(ctx, base);
	// 82C63CF0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82C63CF4: 9421FE00  stwu r1, -0x200(r1)
	ea = ctx.r[1].u32.wrapping_add(-512 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63CF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C63CFC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C63D00: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C63D04: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82C63D08: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C63D0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C63D10: 419A0018  beq cr6, 0x82c63d28
	if ctx.cr[6].eq {
	pc = 0x82C63D28; continue 'dispatch;
	}
	// 82C63D14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C63D18: 4E800421  bctrl
	ctx.lr = 0x82C63D1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C63D1C: 38210200  addi r1, r1, 0x200
	ctx.r[1].s64 = ctx.r[1].s64 + 512;
	// 82C63D20: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82C63D24: 48045738  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C63D28: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 82C63D2C: 38A100D0  addi r5, r1, 0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + 208;
	// 82C63D30: 4BFFFF41  bl 0x82c63c70
	ctx.lr = 0x82C63D34;
	sub_82C63C70(ctx, base);
	// 82C63D34: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 82C63D38: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C63D3C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C63D40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C63D44: 4BFFFF05  bl 0x82c63c48
	ctx.lr = 0x82C63D48;
	sub_82C63C48(ctx, base);
	// 82C63D48: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C63D4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C63D50: 419A0084  beq cr6, 0x82c63dd4
	if ctx.cr[6].eq {
	pc = 0x82C63DD4; continue 'dispatch;
	}
	// 82C63D54: 396100D0  addi r11, r1, 0xd0
	ctx.r[11].s64 = ctx.r[1].s64 + 208;
	// 82C63D58: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C63D5C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C63D60: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C63D64: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C63D68: 409AFFF4  bne cr6, 0x82c63d5c
	if !ctx.cr[6].eq {
	pc = 0x82C63D5C; continue 'dispatch;
	}
	// 82C63D6C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C63D70: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82C63D74: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 82C63D78: 392100D0  addi r9, r1, 0xd0
	ctx.r[9].s64 = ctx.r[1].s64 + 208;
	// 82C63D7C: 5508003E  slwi r8, r8, 0
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C63D80: 396100D0  addi r11, r1, 0xd0
	ctx.r[11].s64 = ctx.r[1].s64 + 208;
	// 82C63D84: A14A4A00  lhz r10, 0x4a00(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(18944 as u32) ) } as u64;
	// 82C63D88: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 82C63D8C: 7D484B2E  sthx r10, r8, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u16) };
	// 82C63D90: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C63D94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C63D98: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C63D9C: 409AFFF4  bne cr6, 0x82c63d90
	if !ctx.cr[6].eq {
	pc = 0x82C63D90; continue 'dispatch;
	}
	// 82C63DA0: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 82C63DA4: 392100D0  addi r9, r1, 0xd0
	ctx.r[9].s64 = ctx.r[1].s64 + 208;
	// 82C63DA8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C63DAC: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82C63DB0: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C63DB4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82C63DB8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82C63DBC: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 82C63DC0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C63DC4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C63DC8: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82C63DCC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C63DD0: 409AFFF0  bne cr6, 0x82c63dc0
	if !ctx.cr[6].eq {
	pc = 0x82C63DC0; continue 'dispatch;
	}
	// 82C63DD4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C63DD8: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
	// 82C63DDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C63DE0: 4BFFD1A9  bl 0x82c60f88
	ctx.lr = 0x82C63DE4;
	sub_82C60F88(ctx, base);
	// 82C63DE4: 38210200  addi r1, r1, 0x200
	ctx.r[1].s64 = ctx.r[1].s64 + 512;
	// 82C63DE8: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82C63DEC: 48045670  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63DF0 size=8
    let mut pc: u32 = 0x82C63DF0;
    'dispatch: loop {
        match pc {
            0x82C63DF0 => {
    //   block [0x82C63DF0..0x82C63DF8)
	// 82C63DF0: 82CAE348  lwz r22, -0x1cb8(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7352 as u32) ) } as u64;
	// 82C63DF4: 8200DCB0  lwz r16, -0x2350(0)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, -9040u32 ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C63DF8 size=152
    let mut pc: u32 = 0x82C63DF8;
    'dispatch: loop {
        match pc {
            0x82C63DF8 => {
    //   block [0x82C63DF8..0x82C63E90)
	// 82C63DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63E00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C63E04: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82C63E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63E0C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C63E10: 909F009C  stw r4, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 82C63E14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C63E18: 419A0010  beq cr6, 0x82c63e28
	if ctx.cr[6].eq {
	pc = 0x82C63E28; continue 'dispatch;
	}
	// 82C63E1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C63E20: 4E800421  bctrl
	ctx.lr = 0x82C63E24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C63E24: 48000058  b 0x82c63e7c
	pc = 0x82C63E7C; continue 'dispatch;
	// 82C63E28: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C63E2C: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82C63E30: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82C63E34: 48003695  bl 0x82c674c8
	ctx.lr = 0x82C63E38;
	sub_82C674C8(ctx, base);
	// 82C63E38: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C63E3C: 389F009C  addi r4, r31, 0x9c
	ctx.r[4].s64 = ctx.r[31].s64 + 156;
	// 82C63E40: 38ABDC98  addi r5, r11, -0x2368
	ctx.r[5].s64 = ctx.r[11].s64 + -9064;
	// 82C63E44: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82C63E48: 480033B9  bl 0x82c67200
	ctx.lr = 0x82C63E4C;
	sub_82C67200(ctx, base);
	// 82C63E4C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82C63E50: 389F009C  addi r4, r31, 0x9c
	ctx.r[4].s64 = ctx.r[31].s64 + 156;
	// 82C63E54: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82C63E58: 48003611  bl 0x82c67468
	ctx.lr = 0x82C63E5C;
	sub_82C67468(ctx, base);
	// 82C63E5C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C63E60: 409A0010  bne cr6, 0x82c63e70
	if !ctx.cr[6].eq {
	pc = 0x82C63E70; continue 'dispatch;
	}
	// 82C63E64: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C63E68: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C63E6C: D01F0050  stfs f0, 0x50(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C63E70: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82C63E74: 4800314D  bl 0x82c66fc0
	ctx.lr = 0x82C63E78;
	sub_82C66FC0(ctx, base);
	// 82C63E78: C03F0050  lfs f1, 0x50(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C63E7C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82C63E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C63E88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C63E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63E90 size=40
    let mut pc: u32 = 0x82C63E90;
    'dispatch: loop {
        match pc {
            0x82C63E90 => {
    //   block [0x82C63E90..0x82C63EB8)
	// 82C63E90: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82C63E94: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63E98: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63E9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63EA0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82C63EA4: 4800311D  bl 0x82c66fc0
	ctx.lr = 0x82C63EA8;
	sub_82C66FC0(ctx, base);
	// 82C63EA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C63EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C63EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63EB8 size=36
    let mut pc: u32 = 0x82C63EB8;
    'dispatch: loop {
        match pc {
            0x82C63EB8 => {
    //   block [0x82C63EB8..0x82C63EDC)
	// 82C63EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63EC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63EC4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82C63EC8: 4BFFFD39  bl 0x82c63c00
	ctx.lr = 0x82C63ECC;
	sub_82C63C00(ctx, base);
	// 82C63ECC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C63ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C63ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63EE0 size=8
    let mut pc: u32 = 0x82C63EE0;
    'dispatch: loop {
        match pc {
            0x82C63EE0 => {
    //   block [0x82C63EE0..0x82C63EE8)
	// 82C63EE0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C63EE4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63EE8 size=36
    let mut pc: u32 = 0x82C63EE8;
    'dispatch: loop {
        match pc {
            0x82C63EE8 => {
    //   block [0x82C63EE8..0x82C63F0C)
	// 82C63EE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C63EEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C63EF0: 7C8903A6  mtctr r4
	ctx.ctr.u64 = ctx.r[4].u64;
	// 82C63EF4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C63EF8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C63EFC: 4200FFF8  bdnz 0x82c63ef4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82C63EF4; continue 'dispatch;
	}
	// 82C63F00: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C63F04: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82C63F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63F10 size=12
    let mut pc: u32 = 0x82C63F10;
    'dispatch: loop {
        match pc {
            0x82C63F10 => {
    //   block [0x82C63F10..0x82C63F1C)
	// 82C63F10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C63F14: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C63F18: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63F1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63F1C size=24
    let mut pc: u32 = 0x82C63F1C;
    'dispatch: loop {
        match pc {
            0x82C63F1C => {
    //   block [0x82C63F1C..0x82C63F34)
	// 82C63F1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C63F20: 7C8903A6  mtctr r4
	ctx.ctr.u64 = ctx.r[4].u64;
	// 82C63F24: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82C63F28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C63F2C: 4200FFF8  bdnz 0x82c63f24
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82C63F24; continue 'dispatch;
	}
	// 82C63F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63F38 size=48
    let mut pc: u32 = 0x82C63F38;
    'dispatch: loop {
        match pc {
            0x82C63F38 => {
    //   block [0x82C63F38..0x82C63F68)
	// 82C63F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63F44: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82C63F48: 5524F0BE  srwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82C63F4C: 4BFFFF95  bl 0x82c63ee0
	ctx.lr = 0x82C63F50;
	sub_82C63EE0(ctx, base);
	// 82C63F50: 552407BE  clrlwi r4, r9, 0x1e
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0x00000003u64;
	// 82C63F54: 4BFFFFBD  bl 0x82c63f10
	ctx.lr = 0x82C63F58;
	sub_82C63F10(ctx, base);
	// 82C63F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C63F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C63F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63F68 size=12
    let mut pc: u32 = 0x82C63F68;
    'dispatch: loop {
        match pc {
            0x82C63F68 => {
    //   block [0x82C63F68..0x82C63F74)
	// 82C63F68: 3C80E582  lis r4, -0x1a7e
	ctx.r[4].s64 = -444465152;
	// 82C63F6C: 60844000  ori r4, r4, 0x4000
	ctx.r[4].u64 = ctx.r[4].u64 | 16384;
	// 82C63F70: 4B590F48  b 0x821f4eb8
	sub_821F4EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63F78 size=8
    let mut pc: u32 = 0x82C63F78;
    'dispatch: loop {
        match pc {
            0x82C63F78 => {
    //   block [0x82C63F78..0x82C63F80)
	// 82C63F78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C63F7C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63F80 size=12
    let mut pc: u32 = 0x82C63F80;
    'dispatch: loop {
        match pc {
            0x82C63F80 => {
    //   block [0x82C63F80..0x82C63F8C)
	// 82C63F80: 3C80E582  lis r4, -0x1a7e
	ctx.r[4].s64 = -444465152;
	// 82C63F84: 60844000  ori r4, r4, 0x4000
	ctx.r[4].u64 = ctx.r[4].u64 | 16384;
	// 82C63F88: 4B5920C0  b 0x821f6048
	sub_821F6048(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63F8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C63F8C size=4
    let mut pc: u32 = 0x82C63F8C;
    'dispatch: loop {
        match pc {
            0x82C63F8C => {
    //   block [0x82C63F8C..0x82C63F90)
	// 82C63F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C63F90 size=76
    let mut pc: u32 = 0x82C63F90;
    'dispatch: loop {
        match pc {
            0x82C63F90 => {
    //   block [0x82C63F90..0x82C63FDC)
	// 82C63F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63F98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C63F9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C63FA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63FA4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C63FA8: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82C63FAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C63FB0: 4BFFFF89  bl 0x82c63f38
	ctx.lr = 0x82C63FB4;
	sub_82C63F38(ctx, base);
	// 82C63FB4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C63FB8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C63FBC: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C63FC0: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C63FC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C63FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C63FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C63FD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C63FD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C63FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C63FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C63FE0 size=72
    let mut pc: u32 = 0x82C63FE0;
    'dispatch: loop {
        match pc {
            0x82C63FE0 => {
    //   block [0x82C63FE0..0x82C64028)
	// 82C63FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C63FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C63FE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C63FEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C63FF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C63FF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C63FF8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82C63FFC: 4BFFFF6D  bl 0x82c63f68
	ctx.lr = 0x82C64000;
	sub_82C63F68(ctx, base);
	// 82C64000: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C64004: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C64008: 4BFFFF89  bl 0x82c63f90
	ctx.lr = 0x82C6400C;
	sub_82C63F90(ctx, base);
	// 82C6400C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C64010: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C64014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C64018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C6401C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C64020: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C64024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C64028 size=100
    let mut pc: u32 = 0x82C64028;
    'dispatch: loop {
        match pc {
            0x82C64028 => {
    //   block [0x82C64028..0x82C6408C)
	// 82C64028: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 82C6402C: 41980058  blt cr6, 0x82c64084
	if ctx.cr[6].lt {
	pc = 0x82C64084; continue 'dispatch;
	}
	// 82C64030: 3966FFFC  addi r11, r6, -4
	ctx.r[11].s64 = ctx.r[6].s64 + -4;
	// 82C64034: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C64038: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C6403C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C64040: 7CCA3050  subf r6, r10, r6
	ctx.r[6].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 82C64044: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C64048: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C6404C: EDA00072  fmuls f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82C64050: D1A40000  stfs f13, 0(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C64054: C1830004  lfs f12, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C64058: ED6C0072  fmuls f11, f12, f1
	ctx.f[11].f64 = (((ctx.f[12].f64 * ctx.f[1].f64) as f32) as f64);
	// 82C6405C: D1640004  stfs f11, 4(r4)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C64060: C1430008  lfs f10, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82C64064: ED2A0072  fmuls f9, f10, f1
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[1].f64) as f32) as f64);
	// 82C64068: D1240008  stfs f9, 8(r4)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C6406C: C103000C  lfs f8, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82C64070: ECE80072  fmuls f7, f8, f1
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[1].f64) as f32) as f64);
	// 82C64074: D0E4000C  stfs f7, 0xc(r4)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C64078: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82C6407C: 38840010  addi r4, r4, 0x10
	ctx.r[4].s64 = ctx.r[4].s64 + 16;
	// 82C64080: 4082FFC4  bne 0x82c64044
	if !ctx.cr[0].eq {
	pc = 0x82C64044; continue 'dispatch;
	}
	// 82C64084: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82C64088: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C6408C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C6408C size=32
    let mut pc: u32 = 0x82C6408C;
    'dispatch: loop {
        match pc {
            0x82C6408C => {
    //   block [0x82C6408C..0x82C640AC)
	// 82C6408C: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C64090: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82C64094: EDA00072  fmuls f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82C64098: D1A40000  stfs f13, 0(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C6409C: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82C640A0: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82C640A4: 4181FFE8  bgt 0x82c6408c
	if ctx.cr[0].gt {
	pc = 0x82C6408C; continue 'dispatch;
	}
	// 82C640A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C640B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C640B0 size=116
    let mut pc: u32 = 0x82C640B0;
    'dispatch: loop {
        match pc {
            0x82C640B0 => {
    //   block [0x82C640B0..0x82C64124)
	// 82C640B0: 2F070004  cmpwi cr6, r7, 4
	ctx.cr[6].compare_i32(ctx.r[7].s32, 4, &mut ctx.xer);
	// 82C640B4: 41980068  blt cr6, 0x82c6411c
	if ctx.cr[6].lt {
	pc = 0x82C6411C; continue 'dispatch;
	}
	// 82C640B8: 3967FFFC  addi r11, r7, -4
	ctx.r[11].s64 = ctx.r[7].s64 + -4;
	// 82C640BC: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C640C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C640C4: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C640C8: 7CEA3850  subf r7, r10, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[10].s64;
	// 82C640CC: EC01102A  fadds f0, f1, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[2].f64) as f32) as f64;
	// 82C640D0: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C640D4: ED8D0072  fmuls f12, f13, f1
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64) as f32) as f64);
	// 82C640D8: D1840000  stfs f12, 0(r4)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C640DC: C1630004  lfs f11, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82C640E0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C640E4: ED40102A  fadds f10, f0, f2
	ctx.f[10].f64 = ((ctx.f[0].f64 + ctx.f[2].f64) as f32) as f64;
	// 82C640E8: ED2B0032  fmuls f9, f11, f0
	ctx.f[9].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C640EC: D1240004  stfs f9, 4(r4)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C640F0: ED0A102A  fadds f8, f10, f2
	ctx.f[8].f64 = ((ctx.f[10].f64 + ctx.f[2].f64) as f32) as f64;
	// 82C640F4: C0E30008  lfs f7, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82C640F8: ECC702B2  fmuls f6, f7, f10
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[10].f64) as f32) as f64);
	// 82C640FC: D0C40008  stfs f6, 8(r4)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C64100: C0A3000C  lfs f5, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82C64104: EC850232  fmuls f4, f5, f8
	ctx.f[4].f64 = (((ctx.f[5].f64 * ctx.f[8].f64) as f32) as f64);
	// 82C64108: D084000C  stfs f4, 0xc(r4)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C6410C: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82C64110: EC28102A  fadds f1, f8, f2
	ctx.f[1].f64 = ((ctx.f[8].f64 + ctx.f[2].f64) as f32) as f64;
	// 82C64114: 38840010  addi r4, r4, 0x10
	ctx.r[4].s64 = ctx.r[4].s64 + 16;
	// 82C64118: 4082FFB4  bne 0x82c640cc
	if !ctx.cr[0].eq {
	pc = 0x82C640CC; continue 'dispatch;
	}
	// 82C6411C: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82C64120: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64124(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C64124 size=36
    let mut pc: u32 = 0x82C64124;
    'dispatch: loop {
        match pc {
            0x82C64124 => {
    //   block [0x82C64124..0x82C64148)
	// 82C64124: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C64128: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82C6412C: EDA00072  fmuls f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82C64130: D1A40000  stfs f13, 0(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C64134: EC21102A  fadds f1, f1, f2
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[2].f64) as f32) as f64;
	// 82C64138: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82C6413C: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82C64140: 4181FFE4  bgt 0x82c64124
	if ctx.cr[0].gt {
	pc = 0x82C64124; continue 'dispatch;
	}
	// 82C64144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C64148 size=108
    let mut pc: u32 = 0x82C64148;
    'dispatch: loop {
        match pc {
            0x82C64148 => {
    //   block [0x82C64148..0x82C641B4)
	// 82C64148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6414C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C64150: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C64154: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C64158: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82C6415C: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82C64160: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82C64164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82C64168: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6416C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C64170: 40990030  ble cr6, 0x82c641a0
	if !ctx.cr[6].gt {
	pc = 0x82C641A0; continue 'dispatch;
	}
	// 82C64174: 7CA92850  subf r5, r9, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[9].s64;
	// 82C64178: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82C6417C: C0270004  lfs f1, 4(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C64180: 7C85482E  lwzx r4, r5, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82C64184: 80690000  lwz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64188: 4BFFFEA1  bl 0x82c64028
	ctx.lr = 0x82C6418C;
	sub_82C64028(ctx, base);
	// 82C6418C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64190: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82C64194: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82C64198: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C6419C: 4198FFDC  blt cr6, 0x82c64178
	if ctx.cr[6].lt {
	pc = 0x82C64178; continue 'dispatch;
	}
	// 82C641A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C641A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C641A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C641AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C641B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C641B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C641B8 size=140
    let mut pc: u32 = 0x82C641B8;
    'dispatch: loop {
        match pc {
            0x82C641B8 => {
    //   block [0x82C641B8..0x82C64244)
	// 82C641B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C641BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C641C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C641C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C641C8: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82C641CC: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82C641D0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C641D4: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C641D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C641DC: 4099004C  ble cr6, 0x82c64228
	if !ctx.cr[6].gt {
	pc = 0x82C64228; continue 'dispatch;
	}
	// 82C641E0: 7CA92850  subf r5, r9, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[9].s64;
	// 82C641E4: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82C641E8: C0480008  lfs f2, 8(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82C641EC: C0280004  lfs f1, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C641F0: 7C85482E  lwzx r4, r5, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82C641F4: 80690000  lwz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C641F8: 4BFFFEB9  bl 0x82c640b0
	ctx.lr = 0x82C641FC;
	sub_82C640B0(ctx, base);
	// 82C641FC: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64200: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82C64204: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82C64208: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C6420C: 4198FFD8  blt cr6, 0x82c641e4
	if ctx.cr[6].lt {
	pc = 0x82C641E4; continue 'dispatch;
	}
	// 82C64210: D0280004  stfs f1, 4(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C64214: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C64218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C6421C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C64220: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C64224: 4E800020  blr
	return;
	// 82C64228: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C6422C: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C64230: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C64234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C64238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C6423C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C64240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C64248 size=288
    let mut pc: u32 = 0x82C64248;
    'dispatch: loop {
        match pc {
            0x82C64248 => {
    //   block [0x82C64248..0x82C64368)
	// 82C64248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6424C: 480451B9  bl 0x82ca9404
	ctx.lr = 0x82C64250;
	sub_82CA93D0(ctx, base);
	// 82C64250: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C64254: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C64258: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C6425C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82C64260: 839F0014  lwz r28, 0x14(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C64264: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82C64268: 409900E8  ble cr6, 0x82c64350
	if !ctx.cr[6].gt {
	pc = 0x82C64350; continue 'dispatch;
	}
	// 82C6426C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64270: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C64274: 4099002C  ble cr6, 0x82c642a0
	if !ctx.cr[6].gt {
	pc = 0x82C642A0; continue 'dispatch;
	}
	// 82C64278: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C6427C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C64280: 557B103A  slwi r27, r11, 2
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82C64284: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C64288: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82C6428C: 4804567D  bl 0x82ca9908
	ctx.lr = 0x82C64290;
	sub_82CA9908(ctx, base);
	// 82C64290: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C64294: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C64298: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82C6429C: 4804566D  bl 0x82ca9908
	ctx.lr = 0x82C642A0;
	sub_82CA9908(ctx, base);
	// 82C642A0: 7F1CF000  cmpw cr6, r28, r30
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82C642A4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82C642A8: 41980008  blt cr6, 0x82c642b0
	if ctx.cr[6].lt {
	pc = 0x82C642B0; continue 'dispatch;
	}
	// 82C642AC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C642B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C642B4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82C642B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C642BC: 4BFFFEFD  bl 0x82c641b8
	ctx.lr = 0x82C642C0;
	sub_82C641B8(ctx, base);
	// 82C642C0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C642C4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C642C8: 7FC6F050  subf r30, r6, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[6].s64;
	// 82C642CC: 7C665850  subf r3, r6, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 82C642D0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C642D4: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82C642D8: 40990044  ble cr6, 0x82c6431c
	if !ctx.cr[6].gt {
	pc = 0x82C6431C; continue 'dispatch;
	}
	// 82C642DC: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C642E0: 54C7103A  slwi r7, r6, 2
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82C642E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C642E8: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82C642EC: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82C642F0: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82C642F4: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82C642F8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C642FC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C64300: 80890000  lwz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64304: 80A80000  lwz r5, 0(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64308: 7CC43A14  add r6, r4, r7
	ctx.r[6].u64 = ctx.r[4].u64 + ctx.r[7].u64;
	// 82C6430C: 7CA53A14  add r5, r5, r7
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[7].u64;
	// 82C64310: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82C64314: 90A80000  stw r5, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82C64318: 4082FFD0  bne 0x82c642e8
	if !ctx.cr[0].eq {
	pc = 0x82C642E8; continue 'dispatch;
	}
	// 82C6431C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C64320: 409A000C  bne cr6, 0x82c6432c
	if !ctx.cr[6].eq {
	pc = 0x82C6432C; continue 'dispatch;
	}
	// 82C64324: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C64328: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C6432C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C64330: 40990030  ble cr6, 0x82c64360
	if !ctx.cr[6].gt {
	pc = 0x82C64360; continue 'dispatch;
	}
	// 82C64334: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C64338: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82C6433C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C64340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C64344: 4BFFFE05  bl 0x82c64148
	ctx.lr = 0x82C64348;
	sub_82C64148(ctx, base);
	// 82C64348: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82C6434C: 48045108  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C64350: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C64354: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C64358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C6435C: 4BFFFDED  bl 0x82c64148
	ctx.lr = 0x82C64360;
	sub_82C64148(ctx, base);
	// 82C64360: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82C64364: 480450F0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C64368 size=8
    let mut pc: u32 = 0x82C64368;
    'dispatch: loop {
        match pc {
            0x82C64368 => {
    //   block [0x82C64368..0x82C64370)
	// 82C64368: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82C6436C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C64370 size=68
    let mut pc: u32 = 0x82C64370;
    'dispatch: loop {
        match pc {
            0x82C64370 => {
    //   block [0x82C64370..0x82C643B4)
	// 82C64370: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C64374: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C64378: 4099003C  ble cr6, 0x82c643b4
	if !ctx.cr[6].gt {
		sub_82C643B4(ctx, base);
		return;
	}
	// 82C6437C: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C64380: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82C64384: 419A0030  beq cr6, 0x82c643b4
	if ctx.cr[6].eq {
		sub_82C643B4(ctx, base);
		return;
	}
	// 82C64388: 7D6A07B4  extsw r10, r11
	ctx.r[10].s64 = ctx.r[11].s32 as i64;
	// 82C6438C: EC010028  fsubs f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C64390: D023000C  stfs f1, 0xc(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C64394: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C64398: F941FFF0  std r10, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u64 ) };
	// 82C6439C: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C643A0: FD806E9C  fcfid f12, f13
	ctx.f[12].f64 = (ctx.f[13].s64 as f64);
	// 82C643A4: FD606018  frsp f11, f12
	ctx.f[11].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82C643A8: ED405824  fdivs f10, f0, f11
	ctx.f[10].f64 = ((ctx.f[0].f64 / ctx.f[11].f64) as f32) as f64;
	// 82C643AC: D1430008  stfs f10, 8(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C643B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C643B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C643B4 size=8
    let mut pc: u32 = 0x82C643B4;
    'dispatch: loop {
        match pc {
            0x82C643B4 => {
    //   block [0x82C643B4..0x82C643BC)
	// 82C643B4: D0230004  stfs f1, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C643B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C643C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C643C0 size=16
    let mut pc: u32 = 0x82C643C0;
    'dispatch: loop {
        match pc {
            0x82C643C0 => {
    //   block [0x82C643C0..0x82C643D0)
	// 82C643C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C643C4: D0230004  stfs f1, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C643C8: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C643CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C643D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C643D0 size=12
    let mut pc: u32 = 0x82C643D0;
    'dispatch: loop {
        match pc {
            0x82C643D0 => {
    //   block [0x82C643D0..0x82C643DC)
	// 82C643D0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82C643D4: 419A0008  beq cr6, 0x82c643dc
	if ctx.cr[6].eq {
		sub_82C643DC(ctx, base);
		return;
	}
	// 82C643D8: 4BFFFF98  b 0x82c64370
	sub_82C64370(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C643DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C643DC size=4
    let mut pc: u32 = 0x82C643DC;
    'dispatch: loop {
        match pc {
            0x82C643DC => {
    //   block [0x82C643DC..0x82C643E0)
	// 82C643DC: 4BFFFFE4  b 0x82c643c0
	sub_82C643C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C643E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C643E0 size=88
    let mut pc: u32 = 0x82C643E0;
    'dispatch: loop {
        match pc {
            0x82C643E0 => {
    //   block [0x82C643E0..0x82C64438)
	// 82C643E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C643E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C643E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C643EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C643F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C643F4: 4BFFEF2D  bl 0x82c63320
	ctx.lr = 0x82C643F8;
	sub_82C63320(ctx, base);
	// 82C643F8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C643FC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C64400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C64404: 3909DCE8  addi r8, r9, -0x2318
	ctx.r[8].s64 = ctx.r[9].s64 + -8984;
	// 82C64408: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82C6440C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C64410: C00A0C18  lfs f0, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C64414: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C64418: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82C6441C: 917F0088  stw r11, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82C64420: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82C64424: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C64428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C6442C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C64430: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C64434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C64438 size=16
    let mut pc: u32 = 0x82C64438;
    'dispatch: loop {
        match pc {
            0x82C64438 => {
    //   block [0x82C64438..0x82C64448)
	// 82C64438: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C6443C: 394BDCE8  addi r10, r11, -0x2318
	ctx.r[10].s64 = ctx.r[11].s64 + -8984;
	// 82C64440: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C64444: 4BFFF004  b 0x82c63448
	sub_82C63448(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C64448 size=424
    let mut pc: u32 = 0x82C64448;
    'dispatch: loop {
        match pc {
            0x82C64448 => {
    //   block [0x82C64448..0x82C645F0)
	// 82C64448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6444C: 48044FB5  bl 0x82ca9400
	ctx.lr = 0x82C64450;
	sub_82CA93D0(ctx, base);
	// 82C64450: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C64454: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C64458: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82C6445C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C64460: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C64464: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82C64468: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82C6446C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C64470: 7FBAEB78  mr r26, r29
	ctx.r[26].u64 = ctx.r[29].u64;
	// 82C64474: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C64478: 7D0B5215  add. r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82C6447C: 4081001C  ble 0x82c64498
	if !ctx.cr[0].gt {
	pc = 0x82C64498; continue 'dispatch;
	}
	// 82C64480: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C64484: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C64488: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82C6448C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C64490: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82C64494: 48045475  bl 0x82ca9908
	ctx.lr = 0x82C64498;
	sub_82CA9908(ctx, base);
	// 82C64498: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C6449C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C644A0: 40990018  ble cr6, 0x82c644b8
	if !ctx.cr[6].gt {
	pc = 0x82C644B8; continue 'dispatch;
	}
	// 82C644A4: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C644A8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C644AC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C644B0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82C644B4: 48045455  bl 0x82ca9908
	ctx.lr = 0x82C644B8;
	sub_82CA9908(ctx, base);
	// 82C644B8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82C644BC: 40990104  ble cr6, 0x82c645c0
	if !ctx.cr[6].gt {
	pc = 0x82C645C0; continue 'dispatch;
	}
	// 82C644C0: 83DF0088  lwz r30, 0x88(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C644C4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C644C8: 409900D0  ble cr6, 0x82c64598
	if !ctx.cr[6].gt {
	pc = 0x82C64598; continue 'dispatch;
	}
	// 82C644CC: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82C644D0: 41980008  blt cr6, 0x82c644d8
	if ctx.cr[6].lt {
	pc = 0x82C644D8; continue 'dispatch;
	}
	// 82C644D4: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82C644D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C644DC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C644E0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82C644E4: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82C644E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C644EC: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C644F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C644F4: 4E800421  bctrl
	ctx.lr = 0x82C644F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C644F8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C644FC: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C64500: 7D2B5215  add. r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C64504: 40810030  ble 0x82c64534
	if !ctx.cr[0].gt {
	pc = 0x82C64534; continue 'dispatch;
	}
	// 82C64508: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C6450C: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C64510: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C64514: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82C64518: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82C6451C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64520: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C64524: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82C64528: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C6452C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C64530: 4082FFEC  bne 0x82c6451c
	if !ctx.cr[0].eq {
	pc = 0x82C6451C; continue 'dispatch;
	}
	// 82C64534: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C64538: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C6453C: 40990028  ble cr6, 0x82c64564
	if !ctx.cr[6].gt {
	pc = 0x82C64564; continue 'dispatch;
	}
	// 82C64540: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C64544: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C64548: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82C6454C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64550: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C64554: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82C64558: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C6455C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C64560: 4082FFEC  bne 0x82c6454c
	if !ctx.cr[0].eq {
	pc = 0x82C6454C; continue 'dispatch;
	}
	// 82C64564: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C64568: 7FBEE850  subf r29, r30, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82C6456C: 7D5E5851  subf. r10, r30, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C64570: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82C64574: 40820018  bne 0x82c6458c
	if !ctx.cr[0].eq {
	pc = 0x82C6458C; continue 'dispatch;
	}
	// 82C64578: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6457C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C64580: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C64584: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C64588: 4E800421  bctrl
	ctx.lr = 0x82C6458C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C6458C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82C64590: 4199FF30  bgt cr6, 0x82c644c0
	if ctx.cr[6].gt {
	pc = 0x82C644C0; continue 'dispatch;
	}
	// 82C64594: 4800002C  b 0x82c645c0
	pc = 0x82C645C0; continue 'dispatch;
	// 82C64598: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6459C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82C645A0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82C645A4: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82C645A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C645AC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C645B0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C645B4: 4E800421  bctrl
	ctx.lr = 0x82C645B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C645B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82C645BC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82C645C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C645C4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82C645C8: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82C645CC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82C645D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C645D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C645D8: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C645DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C645E0: 4E800421  bctrl
	ctx.lr = 0x82C645E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C645E4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C645E8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82C645EC: 48044E64  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C645F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C645F0 size=8
    let mut pc: u32 = 0x82C645F0;
    'dispatch: loop {
        match pc {
            0x82C645F0 => {
    //   block [0x82C645F0..0x82C645F8)
	// 82C645F0: 9083008C  stw r4, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82C645F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C645F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C645F8 size=8
    let mut pc: u32 = 0x82C645F8;
    'dispatch: loop {
        match pc {
            0x82C645F8 => {
    //   block [0x82C645F8..0x82C64600)
	// 82C645F8: 82CAE348  lwz r22, -0x1cb8(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7352 as u32) ) } as u64;
	// 82C645FC: 8200DD48  lwz r16, -0x22b8(0)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, -8888u32 ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C64600 size=84
    let mut pc: u32 = 0x82C64600;
    'dispatch: loop {
        match pc {
            0x82C64600 => {
    //   block [0x82C64600..0x82C64654)
	// 82C64600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C64604: 48044E09  bl 0x82ca940c
	ctx.lr = 0x82C64608;
	sub_82CA93D0(ctx, base);
	// 82C64608: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82C6460C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C64610: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C64614: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 82C64618: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82C6461C: 4BFFFDC5  bl 0x82c643e0
	ctx.lr = 0x82C64620;
	sub_82C643E0(ctx, base);
	// 82C64620: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C64624: 387E009C  addi r3, r30, 0x9c
	ctx.r[3].s64 = ctx.r[30].s64 + 156;
	// 82C64628: 394BDD10  addi r10, r11, -0x22f0
	ctx.r[10].s64 = ctx.r[11].s64 + -8944;
	// 82C6462C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82C64630: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C64634: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C64638: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C6463C: 48002FAD  bl 0x82c675e8
	ctx.lr = 0x82C64640;
	sub_82C675E8(ctx, base);
	// 82C64640: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C64644: 913E0098  stw r9, 0x98(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(152 as u32), ctx.r[9].u32 ) };
	// 82C64648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C6464C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82C64650: 48044E0C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64654(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C64654 size=40
    let mut pc: u32 = 0x82C64654;
    'dispatch: loop {
        match pc {
            0x82C64654 => {
    //   block [0x82C64654..0x82C6467C)
	// 82C64654: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82C64658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6465C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C64660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C64664: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82C64668: 4BFFFDD1  bl 0x82c64438
	ctx.lr = 0x82C6466C;
	sub_82C64438(ctx, base);
	// 82C6466C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C64670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C64674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C64678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C64680 size=8
    let mut pc: u32 = 0x82C64680;
    'dispatch: loop {
        match pc {
            0x82C64680 => {
    //   block [0x82C64680..0x82C64688)
	// 82C64680: 82CAE348  lwz r22, -0x1cb8(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7352 as u32) ) } as u64;
	// 82C64684: 8200DD80  lwz r16, -0x2280(0)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, -8832u32 ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C64688 size=84
    let mut pc: u32 = 0x82C64688;
    'dispatch: loop {
        match pc {
            0x82C64688 => {
    //   block [0x82C64688..0x82C646DC)
	// 82C64688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6468C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C64690: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C64694: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C64698: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82C6469C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C646A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C646A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C646A8: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82C646AC: 394BDD10  addi r10, r11, -0x22f0
	ctx.r[10].s64 = ctx.r[11].s64 + -8944;
	// 82C646B0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C646B4: 387E009C  addi r3, r30, 0x9c
	ctx.r[3].s64 = ctx.r[30].s64 + 156;
	// 82C646B8: 48002F99  bl 0x82c67650
	ctx.lr = 0x82C646BC;
	sub_82C67650(ctx, base);
	// 82C646BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C646C0: 4BFFFD79  bl 0x82c64438
	ctx.lr = 0x82C646C4;
	sub_82C64438(ctx, base);
	// 82C646C4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82C646C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C646CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C646D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C646D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C646D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C646DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C646DC size=40
    let mut pc: u32 = 0x82C646DC;
    'dispatch: loop {
        match pc {
            0x82C646DC => {
    //   block [0x82C646DC..0x82C64704)
	// 82C646DC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82C646E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C646E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C646E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C646EC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82C646F0: 4BFFFD49  bl 0x82c64438
	ctx.lr = 0x82C646F4;
	sub_82C64438(ctx, base);
	// 82C646F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C646F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C646FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C64700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C64708 size=16
    let mut pc: u32 = 0x82C64708;
    'dispatch: loop {
        match pc {
            0x82C64708 => {
    //   block [0x82C64708..0x82C64718)
	// 82C64708: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C6470C: 556A077A  rlwinm r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C64710: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C64714: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C64718 size=48
    let mut pc: u32 = 0x82C64718;
    'dispatch: loop {
        match pc {
            0x82C64718 => {
    //   block [0x82C64718..0x82C64748)
	// 82C64718: 7C8B07B4  extsw r11, r4
	ctx.r[11].s64 = ctx.r[4].s32 as i64;
	// 82C6471C: 90830098  stw r4, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[4].u32 ) };
	// 82C64720: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C64724: 90830090  stw r4, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[4].u32 ) };
	// 82C64728: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82C6472C: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C64730: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82C64734: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82C64738: C00A0C14  lfs f0, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C6473C: ED606024  fdivs f11, f0, f12
	ctx.f[11].f64 = ((ctx.f[0].f64 / ctx.f[12].f64) as f32) as f64;
	// 82C64740: D1630094  stfs f11, 0x94(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82C64744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C64748 size=8
    let mut pc: u32 = 0x82C64748;
    'dispatch: loop {
        match pc {
            0x82C64748 => {
    //   block [0x82C64748..0x82C64750)
	// 82C64748: 8083008C  lwz r4, 0x8c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 82C6474C: 4BFFFFBC  b 0x82c64708
	sub_82C64708(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C64750 size=560
    let mut pc: u32 = 0x82C64750;
    'dispatch: loop {
        match pc {
            0x82C64750 => {
    //   block [0x82C64750..0x82C64980)
	// 82C64750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C64754: 48044CA5  bl 0x82ca93f8
	ctx.lr = 0x82C64758;
	sub_82CA93D0(ctx, base);
	// 82C64758: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C6475C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C64760: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82C64764: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82C64768: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82C6476C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82C64770: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82C64774: 83BF0024  lwz r29, 0x24(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C64778: 7F98E378  mr r24, r28
	ctx.r[24].u64 = ctx.r[28].u64;
	// 82C6477C: 83DF001C  lwz r30, 0x1c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C64780: 7D7EEA15  add. r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C64784: 4081001C  ble 0x82c647a0
	if !ctx.cr[0].gt {
	pc = 0x82C647A0; continue 'dispatch;
	}
	// 82C64788: 57AA003E  slwi r10, r29, 0
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C6478C: 57CB003E  slwi r11, r30, 0
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C64790: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82C64794: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C64798: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82C6479C: 4804516D  bl 0x82ca9908
	ctx.lr = 0x82C647A0;
	sub_82CA9908(ctx, base);
	// 82C647A0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C647A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C647A8: 40990018  ble cr6, 0x82c647c0
	if !ctx.cr[6].gt {
	pc = 0x82C647C0; continue 'dispatch;
	}
	// 82C647AC: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C647B0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82C647B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C647B8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82C647BC: 4804514D  bl 0x82ca9908
	ctx.lr = 0x82C647C0;
	sub_82CA9908(ctx, base);
	// 82C647C0: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82C647C4: 40990030  ble cr6, 0x82c647f4
	if !ctx.cr[6].gt {
	pc = 0x82C647F4; continue 'dispatch;
	}
	// 82C647C8: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82C647CC: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C647D0: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C647D4: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82C647D8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82C647DC: 7D28D850  subf r9, r8, r27
	ctx.r[9].s64 = ctx.r[27].s64 - ctx.r[8].s64;
	// 82C647E0: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82C647E4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C647E8: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C647EC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C647F0: 4082FFF0  bne 0x82c647e0
	if !ctx.cr[0].eq {
	pc = 0x82C647E0; continue 'dispatch;
	}
	// 82C647F4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82C647F8: 40990158  ble cr6, 0x82c64950
	if !ctx.cr[6].gt {
	pc = 0x82C64950; continue 'dispatch;
	}
	// 82C647FC: 83DF0088  lwz r30, 0x88(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C64800: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C64804: 409900CC  ble cr6, 0x82c648d0
	if !ctx.cr[6].gt {
	pc = 0x82C648D0; continue 'dispatch;
	}
	// 82C64808: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82C6480C: 41980008  blt cr6, 0x82c64814
	if ctx.cr[6].lt {
	pc = 0x82C64814; continue 'dispatch;
	}
	// 82C64810: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82C64814: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64818: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C6481C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82C64820: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82C64824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C64828: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C6482C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C64830: 4E800421  bctrl
	ctx.lr = 0x82C64834;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C64834: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C64838: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C6483C: 7D275A15  add. r9, r7, r11
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C64840: 40810030  ble 0x82c64870
	if !ctx.cr[0].gt {
	pc = 0x82C64870; continue 'dispatch;
	}
	// 82C64844: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C64848: 54E9003E  slwi r9, r7, 0
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C6484C: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C64850: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82C64854: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82C64858: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6485C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C64860: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82C64864: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C64868: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C6486C: 4082FFEC  bne 0x82c64858
	if !ctx.cr[0].eq {
	pc = 0x82C64858; continue 'dispatch;
	}
	// 82C64870: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C64874: 7D675A15  add. r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C64878: 40810030  ble 0x82c648a8
	if !ctx.cr[0].gt {
	pc = 0x82C648A8; continue 'dispatch;
	}
	// 82C6487C: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C64880: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C64884: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C64888: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82C6488C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82C64890: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64894: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C64898: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82C6489C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C648A0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C648A4: 4082FFEC  bne 0x82c64890
	if !ctx.cr[0].eq {
	pc = 0x82C64890; continue 'dispatch;
	}
	// 82C648A8: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C648AC: 7F9EE050  subf r28, r30, r28
	ctx.r[28].s64 = ctx.r[28].s64 - ctx.r[30].s64;
	// 82C648B0: 7D5E5851  subf. r10, r30, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C648B4: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82C648B8: 40820064  bne 0x82c6491c
	if !ctx.cr[0].eq {
	pc = 0x82C6491C; continue 'dispatch;
	}
	// 82C648BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C648C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C648C4: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C648C8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C648CC: 4800004C  b 0x82c64918
	pc = 0x82C64918; continue 'dispatch;
	// 82C648D0: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C648D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C648D8: 40990020  ble cr6, 0x82c648f8
	if !ctx.cr[6].gt {
	pc = 0x82C648F8; continue 'dispatch;
	}
	// 82C648DC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C648E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C648E4: 917F0088  stw r11, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82C648E8: 935F0098  stw r26, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[26].u32 ) };
	// 82C648EC: 812A0028  lwz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C648F0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C648F4: 48000024  b 0x82c64918
	pc = 0x82C64918; continue 'dispatch;
	// 82C648F8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82C648FC: 387F009C  addi r3, r31, 0x9c
	ctx.r[3].s64 = ctx.r[31].s64 + 156;
	// 82C64900: 48002E29  bl 0x82c67728
	ctx.lr = 0x82C64904;
	sub_82C67728(ctx, base);
	// 82C64904: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C64908: 419A0020  beq cr6, 0x82c64928
	if ctx.cr[6].eq {
	pc = 0x82C64928; continue 'dispatch;
	}
	// 82C6490C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C64910: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C64914: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C64918: 4E800421  bctrl
	ctx.lr = 0x82C6491C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C6491C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82C64920: 4199FEDC  bgt cr6, 0x82c647fc
	if ctx.cr[6].gt {
	pc = 0x82C647FC; continue 'dispatch;
	}
	// 82C64924: 4800002C  b 0x82c64950
	pc = 0x82C64950; continue 'dispatch;
	// 82C64928: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6492C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82C64930: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82C64934: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82C64938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C6493C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C64940: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C64944: 4E800421  bctrl
	ctx.lr = 0x82C64948;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C64948: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82C6494C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82C64950: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64954: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82C64958: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82C6495C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82C64960: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C64964: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C64968: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C6496C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C64970: 4E800421  bctrl
	ctx.lr = 0x82C64974;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C64974: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C64978: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82C6497C: 48044ACC  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C64980 size=84
    let mut pc: u32 = 0x82C64980;
    'dispatch: loop {
        match pc {
            0x82C64980 => {
    //   block [0x82C64980..0x82C649D4)
	// 82C64980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C64984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C64988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C6498C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C64990: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82C64994: 556A077A  rlwinm r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C64998: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C6499C: 419A0020  beq cr6, 0x82c649bc
	if ctx.cr[6].eq {
	pc = 0x82C649BC; continue 'dispatch;
	}
	// 82C649A0: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82C649A4: 3863009C  addi r3, r3, 0x9c
	ctx.r[3].s64 = ctx.r[3].s64 + 156;
	// 82C649A8: 48002CC1  bl 0x82c67668
	ctx.lr = 0x82C649AC;
	sub_82C67668(ctx, base);
	// 82C649AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C649B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C649B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C649B8: 4E800020  blr
	return;
	// 82C649BC: 7C8903A6  mtctr r4
	ctx.ctr.u64 = ctx.r[4].u64;
	// 82C649C0: 4E800421  bctrl
	ctx.lr = 0x82C649C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C649C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C649C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C649CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C649D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C649D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C649D8 size=80
    let mut pc: u32 = 0x82C649D8;
    'dispatch: loop {
        match pc {
            0x82C649D8 => {
    //   block [0x82C649D8..0x82C64A28)
	// 82C649D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C649DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C649E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C649E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C649E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C649EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C649F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C649F4: 4BFFFA45  bl 0x82c64438
	ctx.lr = 0x82C649F8;
	sub_82C64438(ctx, base);
	// 82C649F8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C649FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C64A00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C64A04: 419A000C  beq cr6, 0x82c64a10
	if ctx.cr[6].eq {
	pc = 0x82C64A10; continue 'dispatch;
	}
	// 82C64A08: 4BBE0DA9  bl 0x828457b0
	ctx.lr = 0x82C64A0C;
	sub_828457B0(ctx, base);
	// 82C64A0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C64A10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C64A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C64A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C64A1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C64A20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C64A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C64A28 size=80
    let mut pc: u32 = 0x82C64A28;
    'dispatch: loop {
        match pc {
            0x82C64A28 => {
    //   block [0x82C64A28..0x82C64A78)
	// 82C64A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C64A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C64A30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C64A34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C64A38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C64A3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C64A40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C64A44: 4BFFFC45  bl 0x82c64688
	ctx.lr = 0x82C64A48;
	sub_82C64688(ctx, base);
	// 82C64A48: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C64A4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C64A50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C64A54: 419A000C  beq cr6, 0x82c64a60
	if ctx.cr[6].eq {
	pc = 0x82C64A60; continue 'dispatch;
	}
	// 82C64A58: 4BBE0D59  bl 0x828457b0
	ctx.lr = 0x82C64A5C;
	sub_828457B0(ctx, base);
	// 82C64A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C64A60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C64A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C64A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C64A6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C64A70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C64A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C64A78 size=148
    let mut pc: u32 = 0x82C64A78;
    'dispatch: loop {
        match pc {
            0x82C64A78 => {
    //   block [0x82C64A78..0x82C64B0C)
	// 82C64A78: 7CAA07B4  extsw r10, r5
	ctx.r[10].s64 = ctx.r[5].s32 as i64;
	// 82C64A7C: C123000C  lfs f9, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82C64A80: 7C8B07B4  extsw r11, r4
	ctx.r[11].s64 = ctx.r[4].s32 as i64;
	// 82C64A84: C1030004  lfs f8, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82C64A88: F941FFF0  std r10, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u64 ) };
	// 82C64A8C: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C64A90: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82C64A94: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C64A98: FD60069C  fcfid f11, f0
	ctx.f[11].f64 = (ctx.f[0].s64 as f64);
	// 82C64A9C: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82C64AA0: FD806E9C  fcfid f12, f13
	ctx.f[12].f64 = (ctx.f[13].s64 as f64);
	// 82C64AA4: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C64AA8: FCA05818  frsp f5, f11
	ctx.f[5].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82C64AAC: C1A9BDF4  lfs f13, -0x420c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16908 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C64AB0: C0E30008  lfs f7, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82C64AB4: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82C64AB8: C0C30000  lfs f6, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82C64ABC: C0080C4C  lfs f0, 0xc4c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C64AC0: FD406018  frsp f10, f12
	ctx.f[10].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82C64AC4: C1870C14  lfs f12, 0xc14(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(3092 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C64AC8: EC8A2824  fdivs f4, f10, f5
	ctx.f[4].f64 = ((ctx.f[10].f64 / ctx.f[5].f64) as f32) as f64;
	// 82C64ACC: EC640132  fmuls f3, f4, f4
	ctx.f[3].f64 = (((ctx.f[4].f64 * ctx.f[4].f64) as f32) as f64);
	// 82C64AD0: EC430132  fmuls f2, f3, f4
	ctx.f[2].f64 = (((ctx.f[3].f64 * ctx.f[4].f64) as f32) as f64);
	// 82C64AD4: EC230372  fmuls f1, f3, f13
	ctx.f[1].f64 = (((ctx.f[3].f64 * ctx.f[13].f64) as f32) as f64);
	// 82C64AD8: EDA20032  fmuls f13, f2, f0
	ctx.f[13].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C64ADC: ED63103C  fnmsubs f11, f3, f0, f2
	ctx.f[11].f64 = -(((ctx.f[3].f64 * ctx.f[0].f64 - ctx.f[2].f64) as f32) as f64);
	// 82C64AE0: ED421828  fsubs f10, f2, f3
	ctx.f[10].f64 = (((ctx.f[2].f64 - ctx.f[3].f64) as f32) as f64);
	// 82C64AE4: EC616828  fsubs f3, f1, f13
	ctx.f[3].f64 = (((ctx.f[1].f64 - ctx.f[13].f64) as f32) as f64);
	// 82C64AE8: EC4B202A  fadds f2, f11, f4
	ctx.f[2].f64 = ((ctx.f[11].f64 + ctx.f[4].f64) as f32) as f64;
	// 82C64AEC: EC0A0272  fmuls f0, f10, f9
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[9].f64) as f32) as f64);
	// 82C64AF0: EDAD0828  fsubs f13, f13, f1
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[1].f64) as f32) as f64);
	// 82C64AF4: ED6301F2  fmuls f11, f3, f7
	ctx.f[11].f64 = (((ctx.f[3].f64 * ctx.f[7].f64) as f32) as f64);
	// 82C64AF8: ED42023A  fmadds f10, f2, f8, f0
	ctx.f[10].f64 = (((ctx.f[2].f64 * ctx.f[8].f64 + ctx.f[0].f64) as f32) as f64);
	// 82C64AFC: ED2D602A  fadds f9, f13, f12
	ctx.f[9].f64 = ((ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64;
	// 82C64B00: ED0A597A  fmadds f8, f10, f5, f11
	ctx.f[8].f64 = (((ctx.f[10].f64 * ctx.f[5].f64 + ctx.f[11].f64) as f32) as f64);
	// 82C64B04: EC2941BA  fmadds f1, f9, f6, f8
	ctx.f[1].f64 = (((ctx.f[9].f64 * ctx.f[6].f64 + ctx.f[8].f64) as f32) as f64);
	// 82C64B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C64B10 size=164
    let mut pc: u32 = 0x82C64B10;
    'dispatch: loop {
        match pc {
            0x82C64B10 => {
    //   block [0x82C64B10..0x82C64BB4)
	// 82C64B10: 7CAA07B4  extsw r10, r5
	ctx.r[10].s64 = ctx.r[5].s32 as i64;
	// 82C64B14: C0E3000C  lfs f7, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82C64B18: 7C8B07B4  extsw r11, r4
	ctx.r[11].s64 = ctx.r[4].s32 as i64;
	// 82C64B1C: C0C30004  lfs f6, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82C64B20: F941FFF0  std r10, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u64 ) };
	// 82C64B24: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C64B28: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82C64B2C: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C64B30: FD20069C  fcfid f9, f0
	ctx.f[9].f64 = (ctx.f[0].s64 as f64);
	// 82C64B34: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82C64B38: FD806E9C  fcfid f12, f13
	ctx.f[12].f64 = (ctx.f[13].s64 as f64);
	// 82C64B3C: C1A9BDF4  lfs f13, -0x420c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16908 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C64B40: FC604818  frsp f3, f9
	ctx.f[3].f64 = (ctx.f[9].f64 as f32) as f64;
	// 82C64B44: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C64B48: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82C64B4C: C0A30008  lfs f5, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82C64B50: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82C64B54: C0830000  lfs f4, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82C64B58: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82C64B5C: C0080B88  lfs f0, 0xb88(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2952 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C64B60: C1660C4C  lfs f11, 0xc4c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(3148 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82C64B64: C1450C14  lfs f10, 0xc14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(3092 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82C64B68: FD006018  frsp f8, f12
	ctx.f[8].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82C64B6C: C1870A4C  lfs f12, 0xa4c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2636 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C64B70: EC481824  fdivs f2, f8, f3
	ctx.f[2].f64 = ((ctx.f[8].f64 / ctx.f[3].f64) as f32) as f64;
	// 82C64B74: EC2200B2  fmuls f1, f2, f2
	ctx.f[1].f64 = (((ctx.f[2].f64 * ctx.f[2].f64) as f32) as f64);
	// 82C64B78: ED220032  fmuls f9, f2, f0
	ctx.f[9].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C64B7C: ED010372  fmuls f8, f1, f13
	ctx.f[8].f64 = (((ctx.f[1].f64 * ctx.f[13].f64) as f32) as f64);
	// 82C64B80: EC210032  fmuls f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C64B84: EC02433C  fnmsubs f0, f2, f12, f8
	ctx.f[0].f64 = -(((ctx.f[2].f64 * ctx.f[12].f64 - ctx.f[8].f64) as f32) as f64);
	// 82C64B88: EDA242FC  fnmsubs f13, f2, f11, f8
	ctx.f[13].f64 = -(((ctx.f[2].f64 * ctx.f[11].f64 - ctx.f[8].f64) as f32) as f64);
	// 82C64B8C: ED890828  fsubs f12, f9, f1
	ctx.f[12].f64 = (((ctx.f[9].f64 - ctx.f[1].f64) as f32) as f64);
	// 82C64B90: ED614828  fsubs f11, f1, f9
	ctx.f[11].f64 = (((ctx.f[1].f64 - ctx.f[9].f64) as f32) as f64);
	// 82C64B94: ED40502A  fadds f10, f0, f10
	ctx.f[10].f64 = ((ctx.f[0].f64 + ctx.f[10].f64) as f32) as f64;
	// 82C64B98: ED2D01F2  fmuls f9, f13, f7
	ctx.f[9].f64 = (((ctx.f[13].f64 * ctx.f[7].f64) as f32) as f64);
	// 82C64B9C: ED0C0172  fmuls f8, f12, f5
	ctx.f[8].f64 = (((ctx.f[12].f64 * ctx.f[5].f64) as f32) as f64);
	// 82C64BA0: ECEA49BA  fmadds f7, f10, f6, f9
	ctx.f[7].f64 = (((ctx.f[10].f64 * ctx.f[6].f64 + ctx.f[9].f64) as f32) as f64);
	// 82C64BA4: ECC740FA  fmadds f6, f7, f3, f8
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[3].f64 + ctx.f[8].f64) as f32) as f64);
	// 82C64BA8: ECAB313A  fmadds f5, f11, f4, f6
	ctx.f[5].f64 = (((ctx.f[11].f64 * ctx.f[4].f64 + ctx.f[6].f64) as f32) as f64);
	// 82C64BAC: EC251824  fdivs f1, f5, f3
	ctx.f[1].f64 = ((ctx.f[5].f64 / ctx.f[3].f64) as f32) as f64;
	// 82C64BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C64BB8 size=832
    let mut pc: u32 = 0x82C64BB8;
    'dispatch: loop {
        match pc {
            0x82C64BB8 => {
    //   block [0x82C64BB8..0x82C64EF8)
	// 82C64BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C64BBC: 48044839  bl 0x82ca93f4
	ctx.lr = 0x82C64BC0;
	sub_82CA93D0(ctx, base);
	// 82C64BC0: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 82C64BC4: 48049111  bl 0x82cadcd4
	ctx.lr = 0x82C64BC8;
	sub_82CADCA0(ctx, base);
	// 82C64BC8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C64BCC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82C64BD0: F9010130  std r8, 0x130(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), ctx.r[8].u64 ) };
	// 82C64BD4: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82C64BD8: F9210138  std r9, 0x138(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(312 as u32), ctx.r[9].u64 ) };
	// 82C64BDC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82C64BE0: 7F9BB850  subf r28, r27, r23
	ctx.r[28].s64 = ctx.r[23].s64 - ctx.r[27].s64;
	// 82C64BE4: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82C64BE8: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82C64BEC: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 82C64BF0: 419A028C  beq cr6, 0x82c64e7c
	if ctx.cr[6].eq {
	pc = 0x82C64E7C; continue 'dispatch;
	}
	// 82C64BF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C64BF8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C64BFC: C36B0C18  lfs f27, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 82C64C00: C38A0C14  lfs f28, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82C64C04: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C64C08: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82C64C0C: 41980064  blt cr6, 0x82c64c70
	if ctx.cr[6].lt {
	pc = 0x82C64C70; continue 'dispatch;
	}
	// 82C64C10: 576B2036  slwi r11, r27, 4
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C64C14: 7FBCC1D6  mullw r29, r28, r24
	ctx.r[29].s64 = (ctx.r[28].s32 as i64) * (ctx.r[24].s32 as i64);
	// 82C64C18: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82C64C1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C64C20: 3BCBFFF8  addi r30, r11, -8
	ctx.r[30].s64 = ctx.r[11].s64 + -8;
	// 82C64C24: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C64C28: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 82C64C2C: 4BFFFE4D  bl 0x82c64a78
	ctx.lr = 0x82C64C30;
	sub_82C64A78(ctx, base);
	// 82C64C30: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 82C64C34: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C64C38: 4BFFFED9  bl 0x82c64b10
	ctx.lr = 0x82C64C3C;
	sub_82C64B10(ctx, base);
	// 82C64C3C: 7F1FE000  cmpw cr6, r31, r28
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82C64C40: 4098000C  bge cr6, 0x82c64c4c
	if !ctx.cr[6].lt {
	pc = 0x82C64C4C; continue 'dispatch;
	}
	// 82C64C44: D3FE0008  stfs f31, 8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C64C48: D03E000C  stfs f1, 0xc(r30)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C64C4C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82C64C50: 4099000C  ble cr6, 0x82c64c5c
	if !ctx.cr[6].gt {
	pc = 0x82C64C5C; continue 'dispatch;
	}
	// 82C64C54: D3FE0000  stfs f31, 0(r30)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C64C58: D03E0004  stfs f1, 4(r30)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C64C5C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82C64C60: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82C64C64: 7C84C214  add r4, r4, r24
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[24].u64;
	// 82C64C68: 7F1FE000  cmpw cr6, r31, r28
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82C64C6C: 4099FFB8  ble cr6, 0x82c64c24
	if !ctx.cr[6].gt {
	pc = 0x82C64C24; continue 'dispatch;
	}
	// 82C64C70: FC00E090  fmr f0, f28
	ctx.f[0].f64 = ctx.f[28].f64;
	// 82C64C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82C64C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C64C7C: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 82C64C80: 419800A0  blt cr6, 0x82c64d20
	if ctx.cr[6].lt {
	pc = 0x82C64D20; continue 'dispatch;
	}
	// 82C64C84: 576A2036  slwi r10, r27, 4
	ctx.r[10].u32 = ctx.r[27].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C64C88: 5768103A  slwi r8, r27, 2
	ctx.r[8].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C64C8C: 7D2AD214  add r9, r10, r26
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 82C64C90: 38FCFFFD  addi r7, r28, -3
	ctx.r[7].s64 = ctx.r[28].s64 + -3;
	// 82C64C94: 7D48CA14  add r10, r8, r25
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[25].u64;
	// 82C64C98: 39290018  addi r9, r9, 0x18
	ctx.r[9].s64 = ctx.r[9].s64 + 24;
	// 82C64C9C: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C64CA0: C189FFF0  lfs f12, -0x10(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C64CA4: EDAD6028  fsubs f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82C64CA8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C64CAC: 4098000C  bge cr6, 0x82c64cb8
	if !ctx.cr[6].lt {
	pc = 0x82C64CB8; continue 'dispatch;
	}
	// 82C64CB0: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82C64CB4: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82C64CB8: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C64CBC: C1890000  lfs f12, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C64CC0: EDAD6028  fsubs f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82C64CC4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C64CC8: 4098000C  bge cr6, 0x82c64cd4
	if !ctx.cr[6].lt {
	pc = 0x82C64CD4; continue 'dispatch;
	}
	// 82C64CCC: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82C64CD0: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82C64CD4: C1AA0008  lfs f13, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C64CD8: C1890010  lfs f12, 0x10(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C64CDC: EDAD6028  fsubs f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82C64CE0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C64CE4: 4098000C  bge cr6, 0x82c64cf0
	if !ctx.cr[6].lt {
	pc = 0x82C64CF0; continue 'dispatch;
	}
	// 82C64CE8: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82C64CEC: 38CB0002  addi r6, r11, 2
	ctx.r[6].s64 = ctx.r[11].s64 + 2;
	// 82C64CF0: C1AA000C  lfs f13, 0xc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C64CF4: C1890020  lfs f12, 0x20(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C64CF8: EDAD6028  fsubs f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82C64CFC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C64D00: 4098000C  bge cr6, 0x82c64d0c
	if !ctx.cr[6].lt {
	pc = 0x82C64D0C; continue 'dispatch;
	}
	// 82C64D04: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82C64D08: 38CB0003  addi r6, r11, 3
	ctx.r[6].s64 = ctx.r[11].s64 + 3;
	// 82C64D0C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C64D10: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82C64D14: 39290040  addi r9, r9, 0x40
	ctx.r[9].s64 = ctx.r[9].s64 + 64;
	// 82C64D18: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82C64D1C: 4198FF80  blt cr6, 0x82c64c9c
	if ctx.cr[6].lt {
	pc = 0x82C64C9C; continue 'dispatch;
	}
	// 82C64D20: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82C64D24: 4098004C  bge cr6, 0x82c64d70
	if !ctx.cr[6].lt {
	pc = 0x82C64D70; continue 'dispatch;
	}
	// 82C64D28: 7D4BDA14  add r10, r11, r27
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82C64D2C: 55492036  slwi r9, r10, 4
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C64D30: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C64D34: 7D29D214  add r9, r9, r26
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[26].u64;
	// 82C64D38: 7D4ACA14  add r10, r10, r25
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[25].u64;
	// 82C64D3C: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82C64D40: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C64D44: C1890000  lfs f12, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C64D48: EDAD6028  fsubs f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82C64D4C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C64D50: 4098000C  bge cr6, 0x82c64d5c
	if !ctx.cr[6].lt {
	pc = 0x82C64D5C; continue 'dispatch;
	}
	// 82C64D54: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82C64D58: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82C64D5C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C64D60: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82C64D64: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82C64D68: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82C64D6C: 4198FFD4  blt cr6, 0x82c64d40
	if ctx.cr[6].lt {
	pc = 0x82C64D40; continue 'dispatch;
	}
	// 82C64D70: FF00D800  fcmpu cr6, f0, f27
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[27].f64);
	// 82C64D74: 40980174  bge cr6, 0x82c64ee8
	if !ctx.cr[6].lt {
	pc = 0x82C64EE8; continue 'dispatch;
	}
	// 82C64D78: 7FE6DA14  add r31, r6, r27
	ctx.r[31].u64 = ctx.r[6].u64 + ctx.r[27].u64;
	// 82C64D7C: C0010130  lfs f0, 0x130(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(304 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C64D80: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82C64D84: 7D7FB850  subf r11, r31, r23
	ctx.r[11].s64 = ctx.r[23].s64 - ctx.r[31].s64;
	// 82C64D88: C3E10138  lfs f31, 0x138(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(312 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82C64D8C: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C64D90: C1A10134  lfs f13, 0x134(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C64D94: 7D2BC1D6  mullw r9, r11, r24
	ctx.r[9].s64 = (ctx.r[11].s32 as i64) * (ctx.r[24].s32 as i64);
	// 82C64D98: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82C64D9C: 7C0ACC2E  lfsx f0, r10, r25
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[25].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C64DA0: ED9F0028  fsubs f12, f31, f0
	ctx.f[12].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C64DA4: FFC00090  fmr f30, f0
	ctx.f[30].f64 = ctx.f[0].f64;
	// 82C64DA8: D3C10068  stfs f30, 0x68(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82C64DAC: 7D2807B4  extsw r8, r9
	ctx.r[8].s64 = ctx.r[9].s32 as i64;
	// 82C64DB0: 7F1FD800  cmpw cr6, r31, r27
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82C64DB4: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 82C64DB8: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C64DBC: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 82C64DC0: FD205018  frsp f9, f10
	ctx.f[9].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82C64DC4: EFAC4824  fdivs f29, f12, f9
	ctx.f[29].f64 = ((ctx.f[12].f64 / ctx.f[9].f64) as f32) as f64;
	// 82C64DC8: D3A1006C  stfs f29, 0x6c(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82C64DCC: 409A0038  bne cr6, 0x82c64e04
	if !ctx.cr[6].eq {
	pc = 0x82C64E04; continue 'dispatch;
	}
	// 82C64DD0: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82C64DD4: 576B2036  slwi r11, r27, 4
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C64DD8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82C64DDC: 7D2BD214  add r9, r11, r26
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82C64DE0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64DE4: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C64DE8: 80CA0008  lwz r6, 8(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C64DEC: 80AA000C  lwz r5, 0xc(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C64DF0: 7D0BD12E  stwx r8, r11, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32), ctx.r[8].u32) };
	// 82C64DF4: 90E90004  stw r7, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82C64DF8: 90C90008  stw r6, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82C64DFC: 90A9000C  stw r5, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82C64E00: 48000024  b 0x82c64e24
	pc = 0x82C64E24; continue 'dispatch;
	// 82C64E04: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82C64E08: E9010060  ld r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C64E0C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82C64E10: E9210068  ld r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82C64E14: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82C64E18: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82C64E1C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C64E20: 4BFFFD99  bl 0x82c64bb8
	ctx.lr = 0x82C64E24;
	sub_82C64BB8(ctx, base);
	// 82C64E24: 3977FFFF  addi r11, r23, -1
	ctx.r[11].s64 = ctx.r[23].s64 + -1;
	// 82C64E28: C001013C  lfs f0, 0x13c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(316 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C64E2C: D3C10060  stfs f30, 0x60(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82C64E30: D3A10064  stfs f29, 0x64(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82C64E34: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C64E38: D3E10068  stfs f31, 0x68(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82C64E3C: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82C64E40: 419A0078  beq cr6, 0x82c64eb8
	if ctx.cr[6].eq {
	pc = 0x82C64EB8; continue 'dispatch;
	}
	// 82C64E44: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82C64E48: 39410130  addi r10, r1, 0x130
	ctx.r[10].s64 = ctx.r[1].s64 + 304;
	// 82C64E4C: 7F9FB850  subf r28, r31, r23
	ctx.r[28].s64 = ctx.r[23].s64 - ctx.r[31].s64;
	// 82C64E50: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82C64E54: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 82C64E58: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64E5C: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C64E60: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C64E64: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C64E68: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C64E6C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C64E70: 90EA0008  stw r7, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82C64E74: 90CA000C  stw r6, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82C64E78: 409AFD8C  bne cr6, 0x82c64c04
	if !ctx.cr[6].eq {
	pc = 0x82C64C04; continue 'dispatch;
	}
	// 82C64E7C: 39410130  addi r10, r1, 0x130
	ctx.r[10].s64 = ctx.r[1].s64 + 304;
	// 82C64E80: 576B2036  slwi r11, r27, 4
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C64E84: 7D2BD214  add r9, r11, r26
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82C64E88: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64E8C: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C64E90: 80CA0008  lwz r6, 8(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C64E94: 80AA000C  lwz r5, 0xc(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C64E98: 7D0BD12E  stwx r8, r11, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32), ctx.r[8].u32) };
	// 82C64E9C: 90E90004  stw r7, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82C64EA0: 90C90008  stw r6, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82C64EA4: 90A9000C  stw r5, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82C64EA8: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82C64EAC: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 82C64EB0: 48048E71  bl 0x82cadd20
	ctx.lr = 0x82C64EB4;
	sub_82CADCEC(ctx, base);
	// 82C64EB4: 48044590  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
	// 82C64EB8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82C64EBC: 56EB2036  slwi r11, r23, 4
	ctx.r[11].u32 = ctx.r[23].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C64EC0: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82C64EC4: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64EC8: 390BFFF0  addi r8, r11, -0x10
	ctx.r[8].s64 = ctx.r[11].s64 + -16;
	// 82C64ECC: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C64ED0: 80CA0008  lwz r6, 8(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C64ED4: 80AA000C  lwz r5, 0xc(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C64ED8: 912BFFF0  stw r9, -0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-16 as u32), ctx.r[9].u32 ) };
	// 82C64EDC: 90EBFFF4  stw r7, -0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-12 as u32), ctx.r[7].u32 ) };
	// 82C64EE0: 90CBFFF8  stw r6, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[6].u32 ) };
	// 82C64EE4: 90ABFFFC  stw r5, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[5].u32 ) };
	// 82C64EE8: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82C64EEC: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 82C64EF0: 48048E31  bl 0x82cadd20
	ctx.lr = 0x82C64EF4;
	sub_82CADCEC(ctx, base);
	// 82C64EF4: 48044550  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C64EF8 size=92
    let mut pc: u32 = 0x82C64EF8;
    'dispatch: loop {
        match pc {
            0x82C64EF8 => {
    //   block [0x82C64EF8..0x82C64F54)
	// 82C64EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C64EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C64F00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C64F04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C64F08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C64F0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C64F10: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 82C64F14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C64F18: 4BFFF021  bl 0x82c63f38
	ctx.lr = 0x82C64F1C;
	sub_82C63F38(ctx, base);
	// 82C64F1C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C64F20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C64F24: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 82C64F28: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C64F2C: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C64F30: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C64F34: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C64F38: D01F005C  stfs f0, 0x5c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82C64F3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C64F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C64F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C64F48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C64F4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C64F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C64F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C64F58 size=192
    let mut pc: u32 = 0x82C64F58;
    'dispatch: loop {
        match pc {
            0x82C64F58 => {
    //   block [0x82C64F58..0x82C65018)
	// 82C64F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C64F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C64F60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C64F64: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82C64F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C64F6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C64F70: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C64F74: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C64F78: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C64F7C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82C64F80: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C64F84: 396B0C18  addi r11, r11, 0xc18
	ctx.r[11].s64 = ctx.r[11].s64 + 3096;
	// 82C64F88: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C64F8C: 409A0020  bne cr6, 0x82c64fac
	if !ctx.cr[6].eq {
	pc = 0x82C64FAC; continue 'dispatch;
	}
	// 82C64F90: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C64F94: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C64F98: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82C64F9C: C1A90014  lfs f13, 0x14(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C64FA0: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C64FA4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C64FA8: 40990008  ble cr6, 0x82c64fb0
	if !ctx.cr[6].gt {
	pc = 0x82C64FB0; continue 'dispatch;
	}
	// 82C64FAC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C64FB0: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C64FB4: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82C64FB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C64FBC: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82C64FC0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82C64FC4: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C64FC8: D3E10058  stfs f31, 0x58(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82C64FCC: 80FF0048  lwz r7, 0x48(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C64FD0: E9010050  ld r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C64FD4: E9210058  ld r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82C64FD8: 4BFFFBE1  bl 0x82c64bb8
	ctx.lr = 0x82C64FDC;
	sub_82C64BB8(ctx, base);
	// 82C64FDC: 813F0048  lwz r9, 0x48(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C64FE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82C64FE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C64FE8: D3FF0044  stfs f31, 0x44(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82C64FEC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C64FF0: 915F004C  stw r10, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82C64FF4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C64FF8: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82C64FFC: 913F0050  stw r9, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82C65000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C65004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C65008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C6500C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C65010: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C65014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C65018 size=28
    let mut pc: u32 = 0x82C65018;
    'dispatch: loop {
        match pc {
            0x82C65018 => {
    //   block [0x82C65018..0x82C65034)
	// 82C65018: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6501C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82C65020: 409A0014  bne cr6, 0x82c65034
	if !ctx.cr[6].eq {
		sub_82C65034(ctx, base);
		return;
	}
	// 82C65024: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C65028: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82C6502C: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82C65030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65034(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C65034 size=24
    let mut pc: u32 = 0x82C65034;
    'dispatch: loop {
        match pc {
            0x82C65034 => {
    //   block [0x82C65034..0x82C6504C)
	// 82C65034: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C65038: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82C6503C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C65040: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C65044: 91230050  stw r9, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82C65048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C65050 size=112
    let mut pc: u32 = 0x82C65050;
    'dispatch: loop {
        match pc {
            0x82C65050 => {
    //   block [0x82C65050..0x82C650C0)
	// 82C65050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C65054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C65058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C6505C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C65060: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82C65064: 409A0028  bne cr6, 0x82c6508c
	if !ctx.cr[6].eq {
	pc = 0x82C6508C; continue 'dispatch;
	}
	// 82C65068: C0030044  lfs f0, 0x44(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C6506C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82C65070: 4198001C  blt cr6, 0x82c6508c
	if ctx.cr[6].lt {
	pc = 0x82C6508C; continue 'dispatch;
	}
	// 82C65074: 4BFFFFA5  bl 0x82c65018
	ctx.lr = 0x82C65078;
	sub_82C65018(ctx, base);
	// 82C65078: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C6507C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C65080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C65084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C65088: 4E800020  blr
	return;
	// 82C6508C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82C65090: 419A000C  beq cr6, 0x82c6509c
	if ctx.cr[6].eq {
	pc = 0x82C6509C; continue 'dispatch;
	}
	// 82C65094: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82C65098: 409A0010  bne cr6, 0x82c650a8
	if !ctx.cr[6].eq {
	pc = 0x82C650A8; continue 'dispatch;
	}
	// 82C6509C: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C650A0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82C650A4: 4098FFD0  bge cr6, 0x82c65074
	if !ctx.cr[6].lt {
	pc = 0x82C65074; continue 'dispatch;
	}
	// 82C650A8: 4BFFFEB1  bl 0x82c64f58
	ctx.lr = 0x82C650AC;
	sub_82C64F58(ctx, base);
	// 82C650AC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C650B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C650B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C650B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C650BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C650C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C650C0 size=964
    let mut pc: u32 = 0x82C650C0;
    'dispatch: loop {
        match pc {
            0x82C650C0 => {
    //   block [0x82C650C0..0x82C6510C)
	// 82C650C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C650C4: 48044349  bl 0x82ca940c
	ctx.lr = 0x82C650C8;
	sub_82CA93D0(ctx, base);
	// 82C650C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C650CC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82C650D0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C650D4: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C650D8: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82C650DC: 419902E8  bgt cr6, 0x82c653c4
	if ctx.cr[6].gt {
	pc = 0x82C653C4; continue 'dispatch;
	}
	// 82C650E0: 3D8082C6  lis r12, -0x7d3a
	ctx.r[12].s64 = -2100953088;
	// 82C650E4: 398C50F8  addi r12, r12, 0x50f8
	ctx.r[12].s64 = ctx.r[12].s64 + 20728;
	// 82C650E8: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82C650EC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82C650F0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82C650F4: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82C6510C; continue 'dispatch;
		},
		1 => {
	pc = 0x82C65184; continue 'dispatch;
		},
		2 => {
	pc = 0x82C651E0; continue 'dispatch;
		},
		3 => {
	pc = 0x82C65258; continue 'dispatch;
		},
		4 => {
	pc = 0x82C65258; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82C650F8: 82C6510C  lwz r22, 0x510c(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(20748 as u32) ) } as u64;
	// 82C650FC: 82C65184  lwz r22, 0x5184(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(20868 as u32) ) } as u64;
	// 82C65100: 82C651E0  lwz r22, 0x51e0(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(20960 as u32) ) } as u64;
	// 82C65104: 82C65258  lwz r22, 0x5258(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(21080 as u32) ) } as u64;
	// 82C65108: 82C65258  lwz r22, 0x5258(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(21080 as u32) ) } as u64;
            }
            0x82C6510C => {
    //   block [0x82C6510C..0x82C65184)
	// 82C6510C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C65110: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 82C65114: 41980044  blt cr6, 0x82c65158
	if ctx.cr[6].lt {
	pc = 0x82C65158; continue 'dispatch;
	}
	// 82C65118: 395DFFFC  addi r10, r29, -4
	ctx.r[10].s64 = ctx.r[29].s64 + -4;
	// 82C6511C: 39640008  addi r11, r4, 8
	ctx.r[11].s64 = ctx.r[4].s64 + 8;
	// 82C65120: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C65124: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C65128: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C6512C: C0060004  lfs f0, 4(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65130: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C65134: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82C65138: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C6513C: D1ABFFFC  stfs f13, -4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82C65140: C1860004  lfs f12, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C65144: D18B0000  stfs f12, 0(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C65148: C1660004  lfs f11, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82C6514C: D16B0004  stfs f11, 4(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C65150: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82C65154: 4082FFD8  bne 0x82c6512c
	if !ctx.cr[0].eq {
	pc = 0x82C6512C; continue 'dispatch;
	}
	// 82C65158: 7F09E800  cmpw cr6, r9, r29
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82C6515C: 40980268  bge cr6, 0x82c653c4
	if !ctx.cr[6].lt {
	pc = 0x82C653C4; continue 'dispatch;
	}
	// 82C65160: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C65164: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82C65168: 7D69E850  subf r11, r9, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[9].s64;
	// 82C6516C: C0060004  lfs f0, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65170: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65174: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C65178: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82C6517C: 4082FFF0  bne 0x82c6516c
	if !ctx.cr[0].eq {
	pc = 0x82C6516C; continue 'dispatch;
	}
	// 82C65180: 48000244  b 0x82c653c4
	pc = 0x82C653C4; continue 'dispatch;
            }
            0x82C65184 => {
    //   block [0x82C65184..0x82C651E0)
	// 82C65184: 81660040  lwz r11, 0x40(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C65188: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C6518C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82C65190: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C65194: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C65198: 7C6B3214  add r3, r11, r6
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82C6519C: 40990034  ble cr6, 0x82c651d0
	if !ctx.cr[6].gt {
	pc = 0x82C651D0; continue 'dispatch;
	}
	// 82C651A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C651A4: 8166000C  lwz r11, 0xc(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C651A8: 80A60048  lwz r5, 0x48(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C651AC: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82C651B0: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 82C651B4: 4BFFF8C5  bl 0x82c64a78
	ctx.lr = 0x82C651B8;
	sub_82C64A78(ctx, base);
	// 82C651B8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82C651BC: D0260004  stfs f1, 4(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C651C0: D03E0000  stfs f1, 0(r30)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C651C4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82C651C8: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82C651CC: 4198FFD8  blt cr6, 0x82c651a4
	if ctx.cr[6].lt {
	pc = 0x82C651A4; continue 'dispatch;
	}
	// 82C651D0: 8166000C  lwz r11, 0xc(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C651D4: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82C651D8: 9166000C  stw r11, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C651DC: 480001E8  b 0x82c653c4
	pc = 0x82C653C4; continue 'dispatch;
            }
            0x82C651E0 => {
    //   block [0x82C651E0..0x82C65258)
	// 82C651E0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C651E4: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 82C651E8: 41980044  blt cr6, 0x82c6522c
	if ctx.cr[6].lt {
	pc = 0x82C6522C; continue 'dispatch;
	}
	// 82C651EC: 395DFFFC  addi r10, r29, -4
	ctx.r[10].s64 = ctx.r[29].s64 + -4;
	// 82C651F0: 39640008  addi r11, r4, 8
	ctx.r[11].s64 = ctx.r[4].s64 + 8;
	// 82C651F4: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C651F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C651FC: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C65200: C0060004  lfs f0, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65204: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C65208: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82C6520C: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C65210: D1ABFFFC  stfs f13, -4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82C65214: C1860004  lfs f12, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C65218: D18B0000  stfs f12, 0(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C6521C: C1660004  lfs f11, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82C65220: D16B0004  stfs f11, 4(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C65224: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82C65228: 4082FFD8  bne 0x82c65200
	if !ctx.cr[0].eq {
	pc = 0x82C65200; continue 'dispatch;
	}
	// 82C6522C: 7F09E800  cmpw cr6, r9, r29
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82C65230: 40980194  bge cr6, 0x82c653c4
	if !ctx.cr[6].lt {
	pc = 0x82C653C4; continue 'dispatch;
	}
	// 82C65234: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C65238: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82C6523C: 7D69E850  subf r11, r9, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[9].s64;
	// 82C65240: C0060004  lfs f0, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65244: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65248: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C6524C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82C65250: 4082FFF0  bne 0x82c65240
	if !ctx.cr[0].eq {
	pc = 0x82C65240; continue 'dispatch;
	}
	// 82C65254: 48000170  b 0x82c653c4
	pc = 0x82C653C4; continue 'dispatch;
            }
            0x82C65258 => {
    //   block [0x82C65258..0x82C6540C)
	// 82C65258: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C6525C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C65260: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 82C65264: C80B0CB8  lfd f0, 0xcb8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(3256 as u32) ) };
	// 82C65268: 41980104  blt cr6, 0x82c6536c
	if ctx.cr[6].lt {
	pc = 0x82C6536C; continue 'dispatch;
	}
	// 82C6526C: 397DFFFC  addi r11, r29, -4
	ctx.r[11].s64 = ctx.r[29].s64 + -4;
	// 82C65270: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C65274: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82C65278: 39640008  addi r11, r4, 8
	ctx.r[11].s64 = ctx.r[4].s64 + 8;
	// 82C6527C: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C65280: C1A60008  lfs f13, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C65284: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C65288: FD806828  fsub f12, f0, f13
	ctx.f[12].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 82C6528C: C146005C  lfs f10, 0x5c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(92 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82C65290: FD606890  fmr f11, f13
	ctx.f[11].f64 = ctx.f[13].f64;
	// 82C65294: C1260004  lfs f9, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82C65298: FD004890  fmr f8, f9
	ctx.f[8].f64 = ctx.f[9].f64;
	// 82C6529C: C0E60054  lfs f7, 0x54(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(84 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82C652A0: FCCC5ABA  fmadd f6, f12, f10, f11
	ctx.f[6].f64 = ctx.f[12].f64 * ctx.f[10].f64 + ctx.f[11].f64;
	// 82C652A4: FCA03018  frsp f5, f6
	ctx.f[5].f64 = (ctx.f[6].f64 as f32) as f64;
	// 82C652A8: D0A60008  stfs f5, 8(r6)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C652AC: EC854828  fsubs f4, f5, f9
	ctx.f[4].f64 = (((ctx.f[5].f64 - ctx.f[9].f64) as f32) as f64);
	// 82C652B0: EC6441FA  fmadds f3, f4, f7, f8
	ctx.f[3].f64 = (((ctx.f[4].f64 * ctx.f[7].f64 + ctx.f[8].f64) as f32) as f64);
	// 82C652B4: D0660004  stfs f3, 4(r6)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C652B8: D06BFFF8  stfs f3, -8(r11)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82C652BC: C026005C  lfs f1, 0x5c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(92 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C652C0: C0460008  lfs f2, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82C652C4: FD801090  fmr f12, f2
	ctx.f[12].f64 = ctx.f[2].f64;
	// 82C652C8: FD606028  fsub f11, f0, f12
	ctx.f[11].f64 = ctx.f[0].f64 - ctx.f[12].f64;
	// 82C652CC: C1060054  lfs f8, 0x54(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(84 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82C652D0: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C652D4: FD206890  fmr f9, f13
	ctx.f[9].f64 = ctx.f[13].f64;
	// 82C652D8: FD4B107A  fmadd f10, f11, f1, f2
	ctx.f[10].f64 = ctx.f[11].f64 * ctx.f[1].f64 + ctx.f[2].f64;
	// 82C652DC: FCE05018  frsp f7, f10
	ctx.f[7].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82C652E0: D0E60008  stfs f7, 8(r6)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C652E4: ECC76828  fsubs f6, f7, f13
	ctx.f[6].f64 = (((ctx.f[7].f64 - ctx.f[13].f64) as f32) as f64);
	// 82C652E8: ECA64A3A  fmadds f5, f6, f8, f9
	ctx.f[5].f64 = (((ctx.f[6].f64 * ctx.f[8].f64 + ctx.f[9].f64) as f32) as f64);
	// 82C652EC: D0A60004  stfs f5, 4(r6)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C652F0: D0ABFFFC  stfs f5, -4(r11)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82C652F4: C066005C  lfs f3, 0x5c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(92 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82C652F8: C0860008  lfs f4, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82C652FC: FC202090  fmr f1, f4
	ctx.f[1].f64 = ctx.f[4].f64;
	// 82C65300: FDA00828  fsub f13, f0, f1
	ctx.f[13].f64 = ctx.f[0].f64 - ctx.f[1].f64;
	// 82C65304: C1460054  lfs f10, 0x54(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(84 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82C65308: C0460004  lfs f2, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82C6530C: FD601090  fmr f11, f2
	ctx.f[11].f64 = ctx.f[2].f64;
	// 82C65310: FD8D20FA  fmadd f12, f13, f3, f4
	ctx.f[12].f64 = ctx.f[13].f64 * ctx.f[3].f64 + ctx.f[4].f64;
	// 82C65314: FD206018  frsp f9, f12
	ctx.f[9].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82C65318: D1260008  stfs f9, 8(r6)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C6531C: ED091028  fsubs f8, f9, f2
	ctx.f[8].f64 = (((ctx.f[9].f64 - ctx.f[2].f64) as f32) as f64);
	// 82C65320: ECE85ABA  fmadds f7, f8, f10, f11
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[10].f64 + ctx.f[11].f64) as f32) as f64);
	// 82C65324: D0E60004  stfs f7, 4(r6)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C65328: D0EB0000  stfs f7, 0(r11)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C6532C: C0A6005C  lfs f5, 0x5c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(92 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82C65330: C0C60008  lfs f6, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82C65334: FC603090  fmr f3, f6
	ctx.f[3].f64 = ctx.f[6].f64;
	// 82C65338: FC401828  fsub f2, f0, f3
	ctx.f[2].f64 = ctx.f[0].f64 - ctx.f[3].f64;
	// 82C6533C: C1860054  lfs f12, 0x54(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(84 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C65340: C0860004  lfs f4, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82C65344: FDA02090  fmr f13, f4
	ctx.f[13].f64 = ctx.f[4].f64;
	// 82C65348: FC22317A  fmadd f1, f2, f5, f6
	ctx.f[1].f64 = ctx.f[2].f64 * ctx.f[5].f64 + ctx.f[6].f64;
	// 82C6534C: FD600818  frsp f11, f1
	ctx.f[11].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C65350: D1660008  stfs f11, 8(r6)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C65354: ED4B2028  fsubs f10, f11, f4
	ctx.f[10].f64 = (((ctx.f[11].f64 - ctx.f[4].f64) as f32) as f64);
	// 82C65358: ED2A6B3A  fmadds f9, f10, f12, f13
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64);
	// 82C6535C: D1260004  stfs f9, 4(r6)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C65360: D12B0004  stfs f9, 4(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C65364: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82C65368: 4082FF18  bne 0x82c65280
	if !ctx.cr[0].eq {
	pc = 0x82C65280; continue 'dispatch;
	}
	// 82C6536C: 7F09E800  cmpw cr6, r9, r29
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82C65370: 40980054  bge cr6, 0x82c653c4
	if !ctx.cr[6].lt {
	pc = 0x82C653C4; continue 'dispatch;
	}
	// 82C65374: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C65378: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82C6537C: 7D69E850  subf r11, r9, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[9].s64;
	// 82C65380: C1A60008  lfs f13, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C65384: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65388: FD806828  fsub f12, f0, f13
	ctx.f[12].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 82C6538C: C146005C  lfs f10, 0x5c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(92 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82C65390: FD606890  fmr f11, f13
	ctx.f[11].f64 = ctx.f[13].f64;
	// 82C65394: C1260004  lfs f9, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82C65398: FD004890  fmr f8, f9
	ctx.f[8].f64 = ctx.f[9].f64;
	// 82C6539C: C0E60054  lfs f7, 0x54(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(84 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82C653A0: FCCC5ABA  fmadd f6, f12, f10, f11
	ctx.f[6].f64 = ctx.f[12].f64 * ctx.f[10].f64 + ctx.f[11].f64;
	// 82C653A4: FCA03018  frsp f5, f6
	ctx.f[5].f64 = (ctx.f[6].f64 as f32) as f64;
	// 82C653A8: D0A60008  stfs f5, 8(r6)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C653AC: EC854828  fsubs f4, f5, f9
	ctx.f[4].f64 = (((ctx.f[5].f64 - ctx.f[9].f64) as f32) as f64);
	// 82C653B0: EC6441FA  fmadds f3, f4, f7, f8
	ctx.f[3].f64 = (((ctx.f[4].f64 * ctx.f[7].f64 + ctx.f[8].f64) as f32) as f64);
	// 82C653B4: D0660004  stfs f3, 4(r6)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C653B8: D06A0000  stfs f3, 0(r10)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C653BC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82C653C0: 4082FFC0  bne 0x82c65380
	if !ctx.cr[0].eq {
	pc = 0x82C65380; continue 'dispatch;
	}
	// 82C653C4: 81660050  lwz r11, 0x50(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C653C8: 7D5D5851  subf. r10, r29, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C653CC: 91460050  stw r10, 0x50(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82C653D0: 408200AC  bne 0x82c6547c
	if !ctx.cr[0].eq {
	pc = 0x82C6547C; continue 'dispatch;
	}
	// 82C653D4: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C653D8: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82C653DC: 419900A0  bgt cr6, 0x82c6547c
	if ctx.cr[6].gt {
	pc = 0x82C6547C; continue 'dispatch;
	}
	// 82C653E0: 3D8082C6  lis r12, -0x7d3a
	ctx.r[12].s64 = -2100953088;
	// 82C653E4: 398C53F8  addi r12, r12, 0x53f8
	ctx.r[12].s64 = ctx.r[12].s64 + 21496;
	// 82C653E8: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82C653EC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82C653F0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82C653F4: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82C65474; continue 'dispatch;
		},
		1 => {
	pc = 0x82C6540C; continue 'dispatch;
		},
		2 => {
	pc = 0x82C65458; continue 'dispatch;
		},
		3 => {
	pc = 0x82C65474; continue 'dispatch;
		},
		4 => {
	pc = 0x82C65464; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82C653F8: 82C65474  lwz r22, 0x5474(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(21620 as u32) ) } as u64;
	// 82C653FC: 82C6540C  lwz r22, 0x540c(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(21516 as u32) ) } as u64;
	// 82C65400: 82C65458  lwz r22, 0x5458(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(21592 as u32) ) } as u64;
	// 82C65404: 82C65474  lwz r22, 0x5474(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(21620 as u32) ) } as u64;
	// 82C65408: 82C65464  lwz r22, 0x5464(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(21604 as u32) ) } as u64;
            }
            0x82C6540C => {
    //   block [0x82C6540C..0x82C65458)
	// 82C6540C: 81660040  lwz r11, 0x40(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C65410: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82C65414: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82C65418: 91460040  stw r10, 0x40(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 82C6541C: 409A0024  bne cr6, 0x82c65440
	if !ctx.cr[6].eq {
	pc = 0x82C65440; continue 'dispatch;
	}
	// 82C65420: 8166004C  lwz r11, 0x4c(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C65424: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82C65428: 81260048  lwz r9, 0x48(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C6542C: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C65430: 7D0B49D6  mullw r8, r11, r9
	ctx.r[8].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82C65434: 91060050  stw r8, 0x50(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82C65438: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C6543C: 48044020  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C65440: 81660048  lwz r11, 0x48(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C65444: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C65448: 9146000C  stw r10, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82C6544C: 91660050  stw r11, 0x50(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C65450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C65454: 48044008  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82C65458 => {
    //   block [0x82C65458..0x82C65464)
	// 82C65458: C0060004  lfs f0, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C6545C: D0060008  stfs f0, 8(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C65460: 4800000C  b 0x82c6546c
	pc = 0x82C6546C; continue 'dispatch;
            }
            0x82C65464 => {
    //   block [0x82C65464..0x82C65474)
	// 82C65464: C0060058  lfs f0, 0x58(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65468: D0060054  stfs f0, 0x54(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C6546C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82C65470: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82C65474; continue 'dispatch;
            }
            0x82C65474 => {
    //   block [0x82C65474..0x82C65484)
	// 82C65474: 81660048  lwz r11, 0x48(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C65478: 91660050  stw r11, 0x50(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C6547C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C65480: 48043FDC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C65488 size=8
    let mut pc: u32 = 0x82C65488;
    'dispatch: loop {
        match pc {
            0x82C65488 => {
    //   block [0x82C65488..0x82C65490)
	// 82C65488: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82C6548C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C65490 size=28
    let mut pc: u32 = 0x82C65490;
    'dispatch: loop {
        match pc {
            0x82C65490 => {
    //   block [0x82C65490..0x82C654AC)
	// 82C65490: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C65494: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82C65498: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82C6549C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82C654A0: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82C654A4: 4082FFEC  bne 0x82c65490
	if !ctx.cr[0].eq {
	pc = 0x82C65490; continue 'dispatch;
	}
	// 82C654A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C654B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C654B0 size=20
    let mut pc: u32 = 0x82C654B0;
    'dispatch: loop {
        match pc {
            0x82C654B0 => {
    //   block [0x82C654B0..0x82C654C4)
	// 82C654B0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82C654B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C654B8: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 82C654BC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C654C0: 4BFFFFC8  b 0x82c65488
	sub_82C65488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C654C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C654C8 size=8
    let mut pc: u32 = 0x82C654C8;
    'dispatch: loop {
        match pc {
            0x82C654C8 => {
    //   block [0x82C654C8..0x82C654D0)
	// 82C654C8: D0230054  stfs f1, 0x54(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C654CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C654D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C654D0 size=8
    let mut pc: u32 = 0x82C654D0;
    'dispatch: loop {
        match pc {
            0x82C654D0 => {
    //   block [0x82C654D0..0x82C654D8)
	// 82C654D0: D023005C  stfs f1, 0x5c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82C654D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C654D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C654D8 size=28
    let mut pc: u32 = 0x82C654D8;
    'dispatch: loop {
        match pc {
            0x82C654D8 => {
    //   block [0x82C654D8..0x82C654F4)
	// 82C654D8: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C654DC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82C654E0: D0230058  stfs f1, 0x58(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82C654E4: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C654E8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C654EC: 91230050  stw r9, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82C654F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C654F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C654F8 size=104
    let mut pc: u32 = 0x82C654F8;
    'dispatch: loop {
        match pc {
            0x82C654F8 => {
    //   block [0x82C654F8..0x82C65560)
	// 82C654F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C654FC: 48043F11  bl 0x82ca940c
	ctx.lr = 0x82C65500;
	sub_82CA93D0(ctx, base);
	// 82C65500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C65504: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C65508: 807D009C  lwz r3, 0x9c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(156 as u32) ) } as u64;
	// 82C6550C: 4800132D  bl 0x82c66838
	ctx.lr = 0x82C65510;
	sub_82C66838(ctx, base);
	// 82C65510: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C65514: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C65518: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C6551C: 40990024  ble cr6, 0x82c65540
	if !ctx.cr[6].gt {
	pc = 0x82C65540; continue 'dispatch;
	}
	// 82C65520: 3BFD0044  addi r31, r29, 0x44
	ctx.r[31].s64 = ctx.r[29].s64 + 68;
	// 82C65524: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C65528: 48002709  bl 0x82c67c30
	ctx.lr = 0x82C6552C;
	sub_82C67C30(ctx, base);
	// 82C6552C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C65530: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82C65534: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82C65538: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C6553C: 4198FFE8  blt cr6, 0x82c65524
	if ctx.cr[6].lt {
	pc = 0x82C65524; continue 'dispatch;
	}
	// 82C65540: 807D0094  lwz r3, 0x94(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(148 as u32) ) } as u64;
	// 82C65544: 480012F5  bl 0x82c66838
	ctx.lr = 0x82C65548;
	sub_82C66838(ctx, base);
	// 82C65548: 807D0098  lwz r3, 0x98(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C6554C: 480012ED  bl 0x82c66838
	ctx.lr = 0x82C65550;
	sub_82C66838(ctx, base);
	// 82C65550: 807D0040  lwz r3, 0x40(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C65554: 4BFFEA25  bl 0x82c63f78
	ctx.lr = 0x82C65558;
	sub_82C63F78(ctx, base);
	// 82C65558: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C6555C: 48043F00  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C65560 size=52
    let mut pc: u32 = 0x82C65560;
    'dispatch: loop {
        match pc {
            0x82C65560 => {
    //   block [0x82C65560..0x82C65594)
	// 82C65560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C65564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C65568: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C6556C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C65570: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C65574: 4BFFFF85  bl 0x82c654f8
	ctx.lr = 0x82C65578;
	sub_82C654F8(ctx, base);
	// 82C65578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C6557C: 4BFFE9FD  bl 0x82c63f78
	ctx.lr = 0x82C65580;
	sub_82C63F78(ctx, base);
	// 82C65580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C65584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C65588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C6558C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C65590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C65598 size=288
    let mut pc: u32 = 0x82C65598;
    'dispatch: loop {
        match pc {
            0x82C65598 => {
    //   block [0x82C65598..0x82C656B8)
	// 82C65598: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82C6559C: C1A60000  lfs f13, 0(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C655A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82C655A4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82C655A8: 40990100  ble cr6, 0x82c656a8
	if !ctx.cr[6].gt {
	pc = 0x82C656A8; continue 'dispatch;
	}
	// 82C655AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C655B0: C18B0C18  lfs f12, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C655B4: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C655B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C655BC: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 82C655C0: 4198009C  blt cr6, 0x82c6565c
	if ctx.cr[6].lt {
	pc = 0x82C6565C; continue 'dispatch;
	}
	// 82C655C4: 3925FFFD  addi r9, r5, -3
	ctx.r[9].s64 = ctx.r[5].s64 + -3;
	// 82C655C8: 395F0008  addi r10, r31, 8
	ctx.r[10].s64 = ctx.r[31].s64 + 8;
	// 82C655CC: C00AFFF8  lfs f0, -8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C655D0: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82C655D4: 40980008  bge cr6, 0x82c655dc
	if !ctx.cr[6].lt {
	pc = 0x82C655DC; continue 'dispatch;
	}
	// 82C655D8: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82C655DC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C655E0: 4099000C  ble cr6, 0x82c655ec
	if !ctx.cr[6].gt {
	pc = 0x82C655EC; continue 'dispatch;
	}
	// 82C655E4: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 82C655E8: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82C655EC: C00AFFFC  lfs f0, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C655F0: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82C655F4: 40980008  bge cr6, 0x82c655fc
	if !ctx.cr[6].lt {
	pc = 0x82C655FC; continue 'dispatch;
	}
	// 82C655F8: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82C655FC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C65600: 4099000C  ble cr6, 0x82c6560c
	if !ctx.cr[6].gt {
	pc = 0x82C6560C; continue 'dispatch;
	}
	// 82C65604: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 82C65608: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82C6560C: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65610: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82C65614: 40980008  bge cr6, 0x82c6561c
	if !ctx.cr[6].lt {
	pc = 0x82C6561C; continue 'dispatch;
	}
	// 82C65618: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82C6561C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C65620: 4099000C  ble cr6, 0x82c6562c
	if !ctx.cr[6].gt {
	pc = 0x82C6562C; continue 'dispatch;
	}
	// 82C65624: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 82C65628: 390B0002  addi r8, r11, 2
	ctx.r[8].s64 = ctx.r[11].s64 + 2;
	// 82C6562C: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65630: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82C65634: 40980008  bge cr6, 0x82c6563c
	if !ctx.cr[6].lt {
	pc = 0x82C6563C; continue 'dispatch;
	}
	// 82C65638: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82C6563C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C65640: 4099000C  ble cr6, 0x82c6564c
	if !ctx.cr[6].gt {
	pc = 0x82C6564C; continue 'dispatch;
	}
	// 82C65644: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 82C65648: 390B0003  addi r8, r11, 3
	ctx.r[8].s64 = ctx.r[11].s64 + 3;
	// 82C6564C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C65650: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82C65654: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82C65658: 4198FF74  blt cr6, 0x82c655cc
	if ctx.cr[6].lt {
	pc = 0x82C655CC; continue 'dispatch;
	}
	// 82C6565C: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82C65660: 4098003C  bge cr6, 0x82c6569c
	if !ctx.cr[6].lt {
	pc = 0x82C6569C; continue 'dispatch;
	}
	// 82C65664: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C65668: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82C6566C: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65670: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82C65674: 40980008  bge cr6, 0x82c6567c
	if !ctx.cr[6].lt {
	pc = 0x82C6567C; continue 'dispatch;
	}
	// 82C65678: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82C6567C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C65680: 4099000C  ble cr6, 0x82c6568c
	if !ctx.cr[6].gt {
	pc = 0x82C6568C; continue 'dispatch;
	}
	// 82C65684: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 82C65688: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82C6568C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C65690: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82C65694: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82C65698: 4198FFD4  blt cr6, 0x82c6566c
	if ctx.cr[6].lt {
	pc = 0x82C6566C; continue 'dispatch;
	}
	// 82C6569C: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82C656A0: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82C656A4: 4082FF10  bne 0x82c655b4
	if !ctx.cr[0].eq {
	pc = 0x82C655B4; continue 'dispatch;
	}
	// 82C656A8: D1A60000  stfs f13, 0(r6)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C656AC: 91070000  stw r8, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C656B0: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82C656B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C656B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C656B8 size=8
    let mut pc: u32 = 0x82C656B8;
    'dispatch: loop {
        match pc {
            0x82C656B8 => {
    //   block [0x82C656B8..0x82C656C0)
	// 82C656B8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82C656BC: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C656C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C656C0 size=40
    let mut pc: u32 = 0x82C656C0;
    'dispatch: loop {
        match pc {
            0x82C656C0 => {
    //   block [0x82C656C0..0x82C656E8)
	// 82C656C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C656C4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82C656C8: 40990038  ble cr6, 0x82c65700
	if !ctx.cr[6].gt {
		sub_82C656E8(ctx, base);
		return;
	}
	// 82C656CC: FDA00850  fneg f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 82C656D0: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82C656D4: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C656D8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C656DC: 4098000C  bge cr6, 0x82c656e8
	if !ctx.cr[6].lt {
		sub_82C656E8(ctx, base);
		return;
	}
	// 82C656E0: D1AB0000  stfs f13, 0(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C656E4: 48000010  b 0x82c656f4
	sub_82C656E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C656E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C656E8 size=40
    let mut pc: u32 = 0x82C656E8;
    'dispatch: loop {
        match pc {
            0x82C656E8 => {
    //   block [0x82C656E8..0x82C65710)
	// 82C656E8: FF000800  fcmpu cr6, f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 82C656EC: 40990008  ble cr6, 0x82c656f4
	if !ctx.cr[6].gt {
	pc = 0x82C656F4; continue 'dispatch;
	}
	// 82C656F0: D02B0000  stfs f1, 0(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C656F4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C656F8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C656FC: 4082FFD8  bne 0x82c656d4
	if !ctx.cr[0].eq {
		sub_82C656C0(ctx, base);
		return;
	}
	// 82C65700: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82C65704: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82C65708: 4082FFB8  bne 0x82c656c0
	if !ctx.cr[0].eq {
		sub_82C656C0(ctx, base);
		return;
	}
	// 82C6570C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C65710 size=200
    let mut pc: u32 = 0x82C65710;
    'dispatch: loop {
        match pc {
            0x82C65710 => {
    //   block [0x82C65710..0x82C657D8)
	// 82C65710: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82C65714: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 82C65718: 419800B8  blt cr6, 0x82c657d0
	if ctx.cr[6].lt {
	pc = 0x82C657D0; continue 'dispatch;
	}
	// 82C6571C: 3964FFFC  addi r11, r4, -4
	ctx.r[11].s64 = ctx.r[4].s64 + -4;
	// 82C65720: 81430040  lwz r10, 0x40(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C65724: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C65728: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82C6572C: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 82C65730: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C65734: C00BFFF8  lfs f0, -8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65738: C1A30160  lfs f13, 0x160(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(352 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C6573C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C65740: 40980008  bge cr6, 0x82c65748
	if !ctx.cr[6].lt {
	pc = 0x82C65748; continue 'dispatch;
	}
	// 82C65744: D0030160  stfs f0, 0x160(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(352 as u32), tmp.u32 ) };
	// 82C65748: C1A30164  lfs f13, 0x164(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C6574C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C65750: 40990008  ble cr6, 0x82c65758
	if !ctx.cr[6].gt {
	pc = 0x82C65758; continue 'dispatch;
	}
	// 82C65754: D0030164  stfs f0, 0x164(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(356 as u32), tmp.u32 ) };
	// 82C65758: C00BFFFC  lfs f0, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C6575C: C1A30160  lfs f13, 0x160(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(352 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C65760: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C65764: 40980008  bge cr6, 0x82c6576c
	if !ctx.cr[6].lt {
	pc = 0x82C6576C; continue 'dispatch;
	}
	// 82C65768: D0030160  stfs f0, 0x160(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(352 as u32), tmp.u32 ) };
	// 82C6576C: C1A30164  lfs f13, 0x164(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C65770: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C65774: 40990008  ble cr6, 0x82c6577c
	if !ctx.cr[6].gt {
	pc = 0x82C6577C; continue 'dispatch;
	}
	// 82C65778: D0030164  stfs f0, 0x164(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(356 as u32), tmp.u32 ) };
	// 82C6577C: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65780: C1A30160  lfs f13, 0x160(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(352 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C65784: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C65788: 40980008  bge cr6, 0x82c65790
	if !ctx.cr[6].lt {
	pc = 0x82C65790; continue 'dispatch;
	}
	// 82C6578C: D0030160  stfs f0, 0x160(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(352 as u32), tmp.u32 ) };
	// 82C65790: C1A30164  lfs f13, 0x164(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C65794: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C65798: 40990008  ble cr6, 0x82c657a0
	if !ctx.cr[6].gt {
	pc = 0x82C657A0; continue 'dispatch;
	}
	// 82C6579C: D0030164  stfs f0, 0x164(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(356 as u32), tmp.u32 ) };
	// 82C657A0: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C657A4: C1A30160  lfs f13, 0x160(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(352 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C657A8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C657AC: 40980008  bge cr6, 0x82c657b4
	if !ctx.cr[6].lt {
	pc = 0x82C657B4; continue 'dispatch;
	}
	// 82C657B0: D0030160  stfs f0, 0x160(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(352 as u32), tmp.u32 ) };
	// 82C657B4: C1A30164  lfs f13, 0x164(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C657B8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C657BC: 40990008  ble cr6, 0x82c657c4
	if !ctx.cr[6].gt {
	pc = 0x82C657C4; continue 'dispatch;
	}
	// 82C657C0: D0030164  stfs f0, 0x164(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(356 as u32), tmp.u32 ) };
	// 82C657C4: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C657C8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82C657CC: 4082FF68  bne 0x82c65734
	if !ctx.cr[0].eq {
	pc = 0x82C65734; continue 'dispatch;
	}
	// 82C657D0: 7F082000  cmpw cr6, r8, r4
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82C657D4: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C657D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C657D8 size=68
    let mut pc: u32 = 0x82C657D8;
    'dispatch: loop {
        match pc {
            0x82C657D8 => {
    //   block [0x82C657D8..0x82C6581C)
	// 82C657D8: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C657DC: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C657E0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C657E4: 7D482050  subf r10, r8, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[8].s64;
	// 82C657E8: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C657EC: C1A30160  lfs f13, 0x160(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(352 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C657F0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C657F4: 40980008  bge cr6, 0x82c657fc
	if !ctx.cr[6].lt {
	pc = 0x82C657FC; continue 'dispatch;
	}
	// 82C657F8: D0030160  stfs f0, 0x160(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(352 as u32), tmp.u32 ) };
	// 82C657FC: C1A30164  lfs f13, 0x164(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C65800: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C65804: 40990008  ble cr6, 0x82c6580c
	if !ctx.cr[6].gt {
	pc = 0x82C6580C; continue 'dispatch;
	}
	// 82C65808: D0030164  stfs f0, 0x164(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(356 as u32), tmp.u32 ) };
	// 82C6580C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C65810: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C65814: 4082FFD4  bne 0x82c657e8
	if !ctx.cr[0].eq {
	pc = 0x82C657E8; continue 'dispatch;
	}
	// 82C65818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C65820 size=40
    let mut pc: u32 = 0x82C65820;
    'dispatch: loop {
        match pc {
            0x82C65820 => {
    //   block [0x82C65820..0x82C65848)
	// 82C65820: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C65824: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82C65828: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C6582C: C1AABE14  lfs f13, -0x41ec(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16876 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C65830: EC000824  fdivs f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[1].f64) as f32) as f64;
	// 82C65834: EDA20372  fmuls f13, f2, f13
	ctx.f[13].f64 = (((ctx.f[2].f64 * ctx.f[13].f64) as f32) as f64);
	// 82C65838: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C6583C: 4098000C  bge cr6, 0x82c65848
	if !ctx.cr[6].lt {
		sub_82C65848(ctx, base);
		return;
	}
	// 82C65840: EC200024  fdivs f1, f0, f0
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[0].f64) as f32) as f64;
	// 82C65844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C65848 size=8
    let mut pc: u32 = 0x82C65848;
    'dispatch: loop {
        match pc {
            0x82C65848 => {
    //   block [0x82C65848..0x82C65850)
	// 82C65848: EC206824  fdivs f1, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82C6584C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C65850 size=60
    let mut pc: u32 = 0x82C65850;
    'dispatch: loop {
        match pc {
            0x82C65850 => {
    //   block [0x82C65850..0x82C6588C)
	// 82C65850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C65854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C65858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C6585C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82C65860: FC400890  fmr f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[1].f64;
	// 82C65864: D0290034  stfs f1, 0x34(r9)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82C65868: C0290000  lfs f1, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C6586C: 4BFFFFB5  bl 0x82c65820
	ctx.lr = 0x82C65870;
	sub_82C65820(ctx, base);
	// 82C65870: 386900A0  addi r3, r9, 0xa0
	ctx.r[3].s64 = ctx.r[9].s64 + 160;
	// 82C65874: D0290038  stfs f1, 0x38(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82C65878: 4BFFFC51  bl 0x82c654c8
	ctx.lr = 0x82C6587C;
	sub_82C654C8(ctx, base);
	// 82C6587C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C65880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C65884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C65888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C65890 size=8
    let mut pc: u32 = 0x82C65890;
    'dispatch: loop {
        match pc {
            0x82C65890 => {
    //   block [0x82C65890..0x82C65898)
	// 82C65890: 9083003C  stw r4, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[4].u32 ) };
	// 82C65894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C65898 size=136
    let mut pc: u32 = 0x82C65898;
    'dispatch: loop {
        match pc {
            0x82C65898 => {
    //   block [0x82C65898..0x82C65920)
	// 82C65898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6589C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C658A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C658A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C658A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C658AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C658B0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C658B4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C658B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C658BC: C01F0018  lfs f0, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C658C0: C1BF0020  lfs f13, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C658C4: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82C658C8: D19F0028  stfs f12, 0x28(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82C658CC: C00A0AB4  lfs f0, 0xab4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2740 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C658D0: C82B0D40  lfd f1, 0xd40(r11)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(3392 as u32) ) };
	// 82C658D4: EC4C0032  fmuls f2, f12, f0
	ctx.f[2].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C658D8: 4B598BD1  bl 0x821fe4a8
	ctx.lr = 0x82C658DC;
	sub_821FE4A8(ctx, base);
	// 82C658DC: 813F0030  lwz r9, 0x30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C658E0: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C658E4: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82C658E8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C658EC: 419A000C  beq cr6, 0x82c658f8
	if ctx.cr[6].eq {
	pc = 0x82C658F8; continue 'dispatch;
	}
	// 82C658F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C658F4: C80B0CB8  lfd f0, 0xcb8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(3256 as u32) ) };
	// 82C658F8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C658FC: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82C65900: FC200018  frsp f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82C65904: 4BFFEACD  bl 0x82c643d0
	ctx.lr = 0x82C65908;
	sub_82C643D0(ctx, base);
	// 82C65908: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C6590C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C65910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C65914: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C65918: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C6591C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C65920 size=104
    let mut pc: u32 = 0x82C65920;
    'dispatch: loop {
        match pc {
            0x82C65920 => {
    //   block [0x82C65920..0x82C65988)
	// 82C65920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C65924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C65928: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C6592C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C65930: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C65934: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C65938: FDA00890  fmr f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ctx.f[1].f64;
	// 82C6593C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C65940: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C65944: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C65948: D1BF0018  stfs f13, 0x18(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82C6594C: C00B0AB4  lfs f0, 0xab4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2740 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65950: C82A0D40  lfd f1, 0xd40(r10)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(3392 as u32) ) };
	// 82C65954: EC4D0032  fmuls f2, f13, f0
	ctx.f[2].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C65958: 4B598B51  bl 0x821fe4a8
	ctx.lr = 0x82C6595C;
	sub_821FE4A8(ctx, base);
	// 82C6595C: FD800818  frsp f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C65960: D19F001C  stfs f12, 0x1c(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82C65964: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C65968: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C6596C: 4BFFFF2D  bl 0x82c65898
	ctx.lr = 0x82C65970;
	sub_82C65898(ctx, base);
	// 82C65970: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C65974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C65978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C6597C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C65980: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C65984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C65988 size=104
    let mut pc: u32 = 0x82C65988;
    'dispatch: loop {
        match pc {
            0x82C65988 => {
    //   block [0x82C65988..0x82C659F0)
	// 82C65988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6598C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C65990: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C65994: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C65998: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C6599C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C659A0: FDA00890  fmr f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ctx.f[1].f64;
	// 82C659A4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C659A8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C659AC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C659B0: D1BF0020  stfs f13, 0x20(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82C659B4: C00B0AB4  lfs f0, 0xab4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2740 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C659B8: C82A0D40  lfd f1, 0xd40(r10)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(3392 as u32) ) };
	// 82C659BC: EC4D0032  fmuls f2, f13, f0
	ctx.f[2].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C659C0: 4B598AE9  bl 0x821fe4a8
	ctx.lr = 0x82C659C4;
	sub_821FE4A8(ctx, base);
	// 82C659C4: FD800818  frsp f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C659C8: D19F0024  stfs f12, 0x24(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82C659CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C659D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C659D4: 4BFFFEC5  bl 0x82c65898
	ctx.lr = 0x82C659D8;
	sub_82C65898(ctx, base);
	// 82C659D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C659DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C659E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C659E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C659E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C659EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C659F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C659F0 size=248
    let mut pc: u32 = 0x82C659F0;
    'dispatch: loop {
        match pc {
            0x82C659F0 => {
    //   block [0x82C659F0..0x82C65AE8)
	// 82C659F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C659F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C659F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C659FC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82C65A00: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82C65A04: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 82C65A08: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82C65A0C: 4BFFFAA5  bl 0x82c654b0
	ctx.lr = 0x82C65A10;
	sub_82C654B0(ctx, base);
	// 82C65A10: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C65A14: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82C65A18: 409A0074  bne cr6, 0x82c65a8c
	if !ctx.cr[6].eq {
	pc = 0x82C65A8C; continue 'dispatch;
	}
	// 82C65A1C: 816A004C  lwz r11, 0x4c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C65A20: 810A0040  lwz r8, 0x40(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C65A24: 7CE85850  subf r7, r8, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 82C65A28: 3567FFFF  addic. r11, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65A2C: 40810030  ble 0x82c65a5c
	if !ctx.cr[0].gt {
	pc = 0x82C65A5C; continue 'dispatch;
	}
	// 82C65A30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82C65A34: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C65A38: 80EA0048  lwz r7, 0x48(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C65A3C: 7CC759D6  mullw r6, r7, r11
	ctx.r[6].s64 = (ctx.r[7].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82C65A40: 90C90050  stw r6, 0x50(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82C65A44: C00A0044  lfs f0, 0x44(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(68 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65A48: D0090004  stfs f0, 4(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C65A4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C65A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C65A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C65A58: 4E800020  blr
	return;
	// 82C65A5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65A60: 409A0014  bne cr6, 0x82c65a74
	if !ctx.cr[6].eq {
	pc = 0x82C65A74; continue 'dispatch;
	}
	// 82C65A64: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82C65A68: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C65A6C: 810A0048  lwz r8, 0x48(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C65A70: 91090050  stw r8, 0x50(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82C65A74: C00A0044  lfs f0, 0x44(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(68 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65A78: D0090004  stfs f0, 4(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C65A7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C65A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C65A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C65A88: 4E800020  blr
	return;
	// 82C65A8C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82C65A90: 409A0048  bne cr6, 0x82c65ad8
	if !ctx.cr[6].eq {
	pc = 0x82C65AD8; continue 'dispatch;
	}
	// 82C65A94: 816A0048  lwz r11, 0x48(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C65A98: 810A0050  lwz r8, 0x50(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C65A9C: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82C65AA0: 7D674051  subf. r11, r7, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65AA4: 40810014  ble 0x82c65ab8
	if !ctx.cr[0].gt {
	pc = 0x82C65AB8; continue 'dispatch;
	}
	// 82C65AA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82C65AAC: 91690050  stw r11, 0x50(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C65AB0: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C65AB4: 4800001C  b 0x82c65ad0
	pc = 0x82C65AD0; continue 'dispatch;
	// 82C65AB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65ABC: 409A0014  bne cr6, 0x82c65ad0
	if !ctx.cr[6].eq {
	pc = 0x82C65AD0; continue 'dispatch;
	}
	// 82C65AC0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82C65AC4: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C65AC8: 810A0048  lwz r8, 0x48(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C65ACC: 91090050  stw r8, 0x50(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82C65AD0: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65AD4: D0090004  stfs f0, 4(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C65AD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C65ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C65AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C65AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C65AE8 size=92
    let mut pc: u32 = 0x82C65AE8;
    'dispatch: loop {
        match pc {
            0x82C65AE8 => {
    //   block [0x82C65AE8..0x82C65B44)
	// 82C65AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C65AEC: 48043921  bl 0x82ca940c
	ctx.lr = 0x82C65AF0;
	sub_82CA93D0(ctx, base);
	// 82C65AF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C65AF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C65AF8: 3BDF00A0  addi r30, r31, 0xa0
	ctx.r[30].s64 = ctx.r[31].s64 + 160;
	// 82C65AFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C65B00: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C65B04: 4BFFF3F5  bl 0x82c64ef8
	ctx.lr = 0x82C65B08;
	sub_82C64EF8(ctx, base);
	// 82C65B08: 3BBF0100  addi r29, r31, 0x100
	ctx.r[29].s64 = ctx.r[31].s64 + 256;
	// 82C65B0C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C65B10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C65B14: 4BFFF3E5  bl 0x82c64ef8
	ctx.lr = 0x82C65B18;
	sub_82C64EF8(ctx, base);
	// 82C65B18: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C65B1C: C03F0000  lfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C65B20: C04B0C14  lfs f2, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82C65B24: 4BFFFCFD  bl 0x82c65820
	ctx.lr = 0x82C65B28;
	sub_82C65820(ctx, base);
	// 82C65B28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C65B2C: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 82C65B30: 4BFFF9A1  bl 0x82c654d0
	ctx.lr = 0x82C65B34;
	sub_82C654D0(ctx, base);
	// 82C65B34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C65B38: 4BFFF999  bl 0x82c654d0
	ctx.lr = 0x82C65B3C;
	sub_82C654D0(ctx, base);
	// 82C65B3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C65B40: 4804391C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C65B48 size=164
    let mut pc: u32 = 0x82C65B48;
    'dispatch: loop {
        match pc {
            0x82C65B48 => {
    //   block [0x82C65B48..0x82C65BEC)
	// 82C65B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C65B4C: 480438C1  bl 0x82ca940c
	ctx.lr = 0x82C65B50;
	sub_82CA93D0(ctx, base);
	// 82C65B50: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82C65B54: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C65B58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C65B5C: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82C65B60: 482954D9  bl 0x82efb038
	ctx.lr = 0x82C65B64;
	sub_82EFB038(ctx, base);
	// 82C65B64: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C65B68: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C65B6C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65B70: 40990024  ble cr6, 0x82c65b94
	if !ctx.cr[6].gt {
	pc = 0x82C65B94; continue 'dispatch;
	}
	// 82C65B74: 3BDF0044  addi r30, r31, 0x44
	ctx.r[30].s64 = ctx.r[31].s64 + 68;
	// 82C65B78: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C65B7C: 480020ED  bl 0x82c67c68
	ctx.lr = 0x82C65B80;
	sub_82C67C68(ctx, base);
	// 82C65B80: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C65B84: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82C65B88: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82C65B8C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C65B90: 4198FFE8  blt cr6, 0x82c65b78
	if ctx.cr[6].lt {
	pc = 0x82C65B78; continue 'dispatch;
	}
	// 82C65B94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C65B98: 4BFFFF51  bl 0x82c65ae8
	ctx.lr = 0x82C65B9C;
	sub_82C65AE8(ctx, base);
	// 82C65B9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C65BA0: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82C65BA4: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82C65BA8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C65BAC: 48001EC5  bl 0x82c67a70
	ctx.lr = 0x82C65BB0;
	sub_82C67A70(ctx, base);
	// 82C65BB0: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C65BB4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C65BB8: 48001EB9  bl 0x82c67a70
	ctx.lr = 0x82C65BBC;
	sub_82C67A70(ctx, base);
	// 82C65BBC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C65BC0: 397F0074  addi r11, r31, 0x74
	ctx.r[11].s64 = ctx.r[31].s64 + 116;
	// 82C65BC4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82C65BC8: C0090BD0  lfs f0, 0xbd0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3024 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65BCC: D3EBFFF0  stfs f31, -0x10(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82C65BD0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C65BD4: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C65BD8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C65BDC: 4082FFF0  bne 0x82c65bcc
	if !ctx.cr[0].eq {
	pc = 0x82C65BCC; continue 'dispatch;
	}
	// 82C65BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C65BE4: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82C65BE8: 48043874  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C65BF0 size=952
    let mut pc: u32 = 0x82C65BF0;
    'dispatch: loop {
        match pc {
            0x82C65BF0 => {
    //   block [0x82C65BF0..0x82C65FA8)
	// 82C65BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C65BF4: 480437ED  bl 0x82ca93e0
	ctx.lr = 0x82C65BF8;
	sub_82CA93D0(ctx, base);
	// 82C65BF8: 3981FF88  addi r12, r1, -0x78
	ctx.r[12].s64 = ctx.r[1].s64 + -120;
	// 82C65BFC: 480480D1  bl 0x82cadccc
	ctx.lr = 0x82C65C00;
	sub_82CADCA0(ctx, base);
	// 82C65C00: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C65C04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C65C08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C65C0C: 7CB42B78  mr r20, r5
	ctx.r[20].u64 = ctx.r[5].u64;
	// 82C65C10: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82C65C14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C65C18: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65C1C: 40990018  ble cr6, 0x82c65c34
	if !ctx.cr[6].gt {
	pc = 0x82C65C34; continue 'dispatch;
	}
	// 82C65C20: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C65C24: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82C65C28: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C65C2C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82C65C30: 48043CD9  bl 0x82ca9908
	ctx.lr = 0x82C65C34;
	sub_82CA9908(ctx, base);
	// 82C65C34: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82C65C38: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82C65C3C: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 82C65C40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C65C44: 4BFFE605  bl 0x82c64248
	ctx.lr = 0x82C65C48;
	sub_82C64248(ctx, base);
	// 82C65C48: 7F12C378  mr r18, r24
	ctx.r[18].u64 = ctx.r[24].u64;
	// 82C65C4C: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82C65C50: 40990334  ble cr6, 0x82c65f84
	if !ctx.cr[6].gt {
	pc = 0x82C65F84; continue 'dispatch;
	}
	// 82C65C54: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C65C58: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C65C5C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C65C60: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C65C64: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 82C65C68: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82C65C6C: C34B0A54  lfs f26, 0xa54(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2644 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 82C65C70: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82C65C74: C36A0AC8  lfs f27, 0xac8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2760 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 82C65C78: C3A90B10  lfs f29, 0xb10(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2832 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82C65C7C: 3B3F00A0  addi r25, r31, 0xa0
	ctx.r[25].s64 = ctx.r[31].s64 + 160;
	// 82C65C80: C3280C14  lfs f25, 0xc14(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3092 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 82C65C84: 3ADF0100  addi r22, r31, 0x100
	ctx.r[22].s64 = ctx.r[31].s64 + 256;
	// 82C65C88: CB8713F0  lfd f28, 0x13f0(r7)
	ctx.f[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(5104 as u32) ) };
	// 82C65C8C: 3AFF0084  addi r23, r31, 0x84
	ctx.r[23].s64 = ctx.r[31].s64 + 132;
	// 82C65C90: C3E60AA4  lfs f31, 0xaa4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(2724 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82C65C94: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82C65C98: C3C50C18  lfs f30, 0xc18(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(3096 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82C65C9C: 3A600001  li r19, 1
	ctx.r[19].s64 = 1;
	// 82C65CA0: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C65CA4: 7F1EC000  cmpw cr6, r30, r24
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[24].s32, &mut ctx.xer);
	// 82C65CA8: 41980008  blt cr6, 0x82c65cb0
	if ctx.cr[6].lt {
	pc = 0x82C65CB0; continue 'dispatch;
	}
	// 82C65CAC: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82C65CB0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82C65CB4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C65CB8: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 82C65CBC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C65CC0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C65CC4: 4BFFF8D5  bl 0x82c65598
	ctx.lr = 0x82C65CC8;
	sub_82C65598(ctx, base);
	// 82C65CC8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C65CCC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82C65CD0: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82C65CD4: 80BF0040  lwz r5, 0x40(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C65CD8: 48001AD9  bl 0x82c677b0
	ctx.lr = 0x82C65CDC;
	sub_82C677B0(ctx, base);
	// 82C65CDC: 80BF0040  lwz r5, 0x40(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C65CE0: 57DA103A  slwi r26, r30, 2
	ctx.r[26].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82C65CE4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C65CE8: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C65CEC: 7D7A2A14  add r11, r26, r5
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[5].u64;
	// 82C65CF0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82C65CF4: C00BFFFC  lfs f0, -4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65CF8: D01F0088  stfs f0, 0x88(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82C65CFC: 48001AB5  bl 0x82c677b0
	ctx.lr = 0x82C65D00;
	sub_82C677B0(ctx, base);
	// 82C65D00: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C65D04: C1BF0088  lfs f13, 0x88(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C65D08: 7D4BD214  add r10, r11, r26
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82C65D0C: FF0DF000  fcmpu cr6, f13, f30
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[30].f64);
	// 82C65D10: C00AFFFC  lfs f0, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65D14: D01F008C  stfs f0, 0x8c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82C65D18: 40990010  ble cr6, 0x82c65d28
	if !ctx.cr[6].gt {
	pc = 0x82C65D28; continue 'dispatch;
	}
	// 82C65D1C: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C65D20: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82C65D24: 48000008  b 0x82c65d2c
	pc = 0x82C65D2C; continue 'dispatch;
	// 82C65D28: FC00F890  fmr f0, f31
	ctx.f[0].f64 = ctx.f[31].f64;
	// 82C65D2C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C65D30: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82C65D34: 7EBBAB78  mr r27, r21
	ctx.r[27].u64 = ctx.r[21].u64;
	// 82C65D38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65D3C: 40990038  ble cr6, 0x82c65d74
	if !ctx.cr[6].gt {
	pc = 0x82C65D74; continue 'dispatch;
	}
	// 82C65D40: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82C65D44: 3B9F0044  addi r28, r31, 0x44
	ctx.r[28].s64 = ctx.r[31].s64 + 68;
	// 82C65D48: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C65D4C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C65D50: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C65D54: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82C65D58: 48002899  bl 0x82c685f0
	ctx.lr = 0x82C65D5C;
	sub_82C685F0(ctx, base);
	// 82C65D5C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C65D60: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82C65D64: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82C65D68: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82C65D6C: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C65D70: 4198FFD8  blt cr6, 0x82c65d48
	if ctx.cr[6].lt {
	pc = 0x82C65D48; continue 'dispatch;
	}
	// 82C65D74: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C65D78: 809F0040  lwz r4, 0x40(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C65D7C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82C65D80: 4BFFF341  bl 0x82c650c0
	ctx.lr = 0x82C65D84;
	sub_82C650C0(ctx, base);
	// 82C65D84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C65D88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C65D8C: 4BFFF985  bl 0x82c65710
	ctx.lr = 0x82C65D90;
	sub_82C65710(ctx, base);
	// 82C65D90: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C65D94: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82C65D98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C65D9C: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C65DA0: 80BF0040  lwz r5, 0x40(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C65DA4: 48000D2D  bl 0x82c66ad0
	ctx.lr = 0x82C65DA8;
	sub_82C66AD0(ctx, base);
	// 82C65DA8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C65DAC: 809F0040  lwz r4, 0x40(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C65DB0: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82C65DB4: 4BFFF30D  bl 0x82c650c0
	ctx.lr = 0x82C65DB8;
	sub_82C650C0(ctx, base);
	// 82C65DB8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C65DBC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65DC0: 40990024  ble cr6, 0x82c65de4
	if !ctx.cr[6].gt {
	pc = 0x82C65DE4; continue 'dispatch;
	}
	// 82C65DC4: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C65DC8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82C65DCC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C65DD0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C65DD4: 7D29D214  add r9, r9, r26
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[26].u64;
	// 82C65DD8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C65DDC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C65DE0: 4082FFEC  bne 0x82c65dcc
	if !ctx.cr[0].eq {
	pc = 0x82C65DCC; continue 'dispatch;
	}
	// 82C65DE4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C65DE8: 7F1EC050  subf r24, r30, r24
	ctx.r[24].s64 = ctx.r[24].s64 - ctx.r[30].s64;
	// 82C65DEC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C65DF0: 7D3E5051  subf. r9, r30, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[30].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C65DF4: 7D0BF214  add r8, r11, r30
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82C65DF8: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82C65DFC: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82C65E00: 4082017C  bne 0x82c65f7c
	if !ctx.cr[0].eq {
	pc = 0x82C65F7C; continue 'dispatch;
	}
	// 82C65E04: C01F0068  lfs f0, 0x68(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65E08: 38BF0078  addi r5, r31, 0x78
	ctx.r[5].s64 = ctx.r[31].s64 + 120;
	// 82C65E0C: C1BF007C  lfs f13, 0x7c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C65E10: C19F006C  lfs f12, 0x6c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C65E14: C17F0078  lfs f11, 0x78(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82C65E18: C15F0070  lfs f10, 0x70(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82C65E1C: C13F0080  lfs f9, 0x80(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82C65E20: D01F0064  stfs f0, 0x64(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82C65E24: D17F0074  stfs f11, 0x74(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82C65E28: D1BF0078  stfs f13, 0x78(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82C65E2C: D19F0068  stfs f12, 0x68(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82C65E30: D13F007C  stfs f9, 0x7c(r31)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82C65E34: D15F006C  stfs f10, 0x6c(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82C65E38: C0170000  lfs f0, 0(r23)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65E3C: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 82C65E40: D01F0070  stfs f0, 0x70(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82C65E44: 409A000C  bne cr6, 0x82c65e50
	if !ctx.cr[6].eq {
	pc = 0x82C65E50; continue 'dispatch;
	}
	// 82C65E48: FC00E090  fmr f0, f28
	ctx.f[0].f64 = ctx.f[28].f64;
	// 82C65E4C: 4800000C  b 0x82c65e58
	pc = 0x82C65E58; continue 'dispatch;
	// 82C65E50: C1BF001C  lfs f13, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C65E54: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82C65E58: 817F0168  lwz r11, 0x168(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) } as u64;
	// 82C65E5C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82C65E60: D01F0080  stfs f0, 0x80(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82C65E64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65E68: 409A006C  bne cr6, 0x82c65ed4
	if !ctx.cr[6].eq {
	pc = 0x82C65ED4; continue 'dispatch;
	}
	// 82C65E6C: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C65E70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65E74: 419A0060  beq cr6, 0x82c65ed4
	if ctx.cr[6].eq {
	pc = 0x82C65ED4; continue 'dispatch;
	}
	// 82C65E78: C01F0090  lfs f0, 0x90(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65E7C: FF00C800  fcmpu cr6, f0, f25
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[25].f64);
	// 82C65E80: 40980054  bge cr6, 0x82c65ed4
	if !ctx.cr[6].lt {
	pc = 0x82C65ED4; continue 'dispatch;
	}
	// 82C65E84: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C65E88: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82C65E8C: 409A0048  bne cr6, 0x82c65ed4
	if !ctx.cr[6].eq {
	pc = 0x82C65ED4; continue 'dispatch;
	}
	// 82C65E90: C01F016C  lfs f0, 0x16c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(364 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65E94: 927F0168  stw r19, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[19].u32 ) };
	// 82C65E98: FF00E800  fcmpu cr6, f0, f29
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[29].f64);
	// 82C65E9C: 409A000C  bne cr6, 0x82c65ea8
	if !ctx.cr[6].eq {
	pc = 0x82C65EA8; continue 'dispatch;
	}
	// 82C65EA0: D3FF016C  stfs f31, 0x16c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), tmp.u32 ) };
	// 82C65EA4: 48000010  b 0x82c65eb4
	pc = 0x82C65EB4; continue 'dispatch;
	// 82C65EA8: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82C65EAC: 409A0008  bne cr6, 0x82c65eb4
	if !ctx.cr[6].eq {
	pc = 0x82C65EB4; continue 'dispatch;
	}
	// 82C65EB0: D37F016C  stfs f27, 0x16c(r31)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), tmp.u32 ) };
	// 82C65EB4: C05F016C  lfs f2, 0x16c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(364 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82C65EB8: C03F0000  lfs f1, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C65EBC: 4BFFF965  bl 0x82c65820
	ctx.lr = 0x82C65EC0;
	sub_82C65820(ctx, base);
	// 82C65EC0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82C65EC4: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 82C65EC8: 4BFFF611  bl 0x82c654d8
	ctx.lr = 0x82C65ECC;
	sub_82C654D8(ctx, base);
	// 82C65ECC: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82C65ED0: 4BFFF5F9  bl 0x82c654c8
	ctx.lr = 0x82C65ED4;
	sub_82C654C8(ctx, base);
	// 82C65ED4: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C65ED8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65EDC: 409A0094  bne cr6, 0x82c65f70
	if !ctx.cr[6].eq {
	pc = 0x82C65F70; continue 'dispatch;
	}
	// 82C65EE0: C03F0080  lfs f1, 0x80(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C65EE4: C01F0104  lfs f0, 0x104(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65EE8: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 82C65EEC: 40990084  ble cr6, 0x82c65f70
	if !ctx.cr[6].gt {
	pc = 0x82C65F70; continue 'dispatch;
	}
	// 82C65EF0: 92BF0168  stw r21, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[21].u32 ) };
	// 82C65EF4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82C65EF8: 4BFFF159  bl 0x82c65050
	ctx.lr = 0x82C65EFC;
	sub_82C65050(ctx, base);
	// 82C65EFC: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C65F00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C65F04: 419A0054  beq cr6, 0x82c65f58
	if ctx.cr[6].eq {
	pc = 0x82C65F58; continue 'dispatch;
	}
	// 82C65F08: C01F0090  lfs f0, 0x90(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C65F0C: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82C65F10: 40990018  ble cr6, 0x82c65f28
	if !ctx.cr[6].gt {
	pc = 0x82C65F28; continue 'dispatch;
	}
	// 82C65F14: C03F0000  lfs f1, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C65F18: FC40D890  fmr f2, f27
	ctx.f[2].f64 = ctx.f[27].f64;
	// 82C65F1C: D37F016C  stfs f27, 0x16c(r31)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), tmp.u32 ) };
	// 82C65F20: 4BFFF901  bl 0x82c65820
	ctx.lr = 0x82C65F24;
	sub_82C65820(ctx, base);
	// 82C65F24: 48000038  b 0x82c65f5c
	pc = 0x82C65F5C; continue 'dispatch;
	// 82C65F28: FF00D000  fcmpu cr6, f0, f26
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[26].f64);
	// 82C65F2C: 40990018  ble cr6, 0x82c65f44
	if !ctx.cr[6].gt {
	pc = 0x82C65F44; continue 'dispatch;
	}
	// 82C65F30: C03F0000  lfs f1, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C65F34: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 82C65F38: D3FF016C  stfs f31, 0x16c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), tmp.u32 ) };
	// 82C65F3C: 4BFFF8E5  bl 0x82c65820
	ctx.lr = 0x82C65F40;
	sub_82C65820(ctx, base);
	// 82C65F40: 4800001C  b 0x82c65f5c
	pc = 0x82C65F5C; continue 'dispatch;
	// 82C65F44: C03F0000  lfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C65F48: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 82C65F4C: D3BF016C  stfs f29, 0x16c(r31)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), tmp.u32 ) };
	// 82C65F50: 4BFFF8D1  bl 0x82c65820
	ctx.lr = 0x82C65F54;
	sub_82C65820(ctx, base);
	// 82C65F54: 48000008  b 0x82c65f5c
	pc = 0x82C65F5C; continue 'dispatch;
	// 82C65F58: C03F0038  lfs f1, 0x38(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C65F5C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82C65F60: 4BFFF569  bl 0x82c654c8
	ctx.lr = 0x82C65F64;
	sub_82C654C8(ctx, base);
	// 82C65F64: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82C65F68: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82C65F6C: 4BFFFA85  bl 0x82c659f0
	ctx.lr = 0x82C65F70;
	sub_82C659F0(ctx, base);
	// 82C65F70: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C65F74: D3D70000  stfs f30, 0(r23)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C65F78: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C65F7C: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82C65F80: 4199FD20  bgt cr6, 0x82c65ca0
	if ctx.cr[6].gt {
	pc = 0x82C65CA0; continue 'dispatch;
	}
	// 82C65F84: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 82C65F88: C03F001C  lfs f1, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C65F8C: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82C65F90: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C65F94: 4BFFF725  bl 0x82c656b8
	ctx.lr = 0x82C65F98;
	sub_82C656B8(ctx, base);
	// 82C65F98: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82C65F9C: 3981FF88  addi r12, r1, -0x78
	ctx.r[12].s64 = ctx.r[1].s64 + -120;
	// 82C65FA0: 48047D79  bl 0x82cadd18
	ctx.lr = 0x82C65FA4;
	sub_82CADCEC(ctx, base);
	// 82C65FA4: 4804348C  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C65FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C65FA8 size=412
    let mut pc: u32 = 0x82C65FA8;
    'dispatch: loop {
        match pc {
            0x82C65FA8 => {
    //   block [0x82C65FA8..0x82C66144)
	// 82C65FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C65FAC: 4804345D  bl 0x82ca9408
	ctx.lr = 0x82C65FB0;
	sub_82CA93D0(ctx, base);
	// 82C65FB0: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 82C65FB4: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82C65FB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C65FBC: 38800170  li r4, 0x170
	ctx.r[4].s64 = 368;
	// 82C65FC0: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82C65FC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C65FC8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C65FCC: 4BFFDF6D  bl 0x82c63f38
	ctx.lr = 0x82C65FD0;
	sub_82C63F38(ctx, base);
	// 82C65FD0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C65FD4: D3DF0000  stfs f30, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C65FD8: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C65FDC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C65FE0: C80BD6F0  lfd f0, -0x2910(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-10512 as u32) ) };
	// 82C65FE4: FC1E0032  fmul f0, f30, f0
	ctx.f[0].f64 = ctx.f[30].f64 * ctx.f[0].f64;
	// 82C65FE8: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82C65FEC: D9A10050  stfd f13, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[13].u64 ) };
	// 82C65FF0: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C65FF4: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C65FF8: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82C65FFC: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C66000: 4BFFDFE1  bl 0x82c63fe0
	ctx.lr = 0x82C66004;
	sub_82C63FE0(ctx, base);
	// 82C66004: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C66008: 907F009C  stw r3, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[3].u32 ) };
	// 82C6600C: C19F0000  lfs f12, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C66010: C0080BE8  lfs f0, 0xbe8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3048 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C66014: ED6C0032  fmuls f11, f12, f0
	ctx.f[11].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C66018: FD40581E  fctiwz f10, f11
	ctx.f[10].s64 = if ctx.f[11].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[11].f64.trunc() as i32 as i64 };
	// 82C6601C: D9410050  stfd f10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[10].u64 ) };
	// 82C66020: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C66024: 4BFFE345  bl 0x82c64368
	ctx.lr = 0x82C66028;
	sub_82C64368(ctx, base);
	// 82C66028: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82C6602C: 40990028  ble cr6, 0x82c66054
	if !ctx.cr[6].gt {
	pc = 0x82C66054; continue 'dispatch;
	}
	// 82C66030: 3BBF0044  addi r29, r31, 0x44
	ctx.r[29].s64 = ctx.r[31].s64 + 68;
	// 82C66034: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82C66038: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C6603C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C66040: 48001BA1  bl 0x82c67be0
	ctx.lr = 0x82C66044;
	sub_82C67BE0(ctx, base);
	// 82C66044: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C66048: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C6604C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82C66050: 4082FFE8  bne 0x82c66038
	if !ctx.cr[0].eq {
	pc = 0x82C66038; continue 'dispatch;
	}
	// 82C66054: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C66058: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C6605C: 4BFFDF0D  bl 0x82c63f68
	ctx.lr = 0x82C66060;
	sub_82C63F68(ctx, base);
	// 82C66060: 907F0040  stw r3, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[3].u32 ) };
	// 82C66064: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C66068: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82C6606C: 48001A85  bl 0x82c67af0
	ctx.lr = 0x82C66070;
	sub_82C67AF0(ctx, base);
	// 82C66070: 907F0094  stw r3, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 82C66074: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C66078: 480019E9  bl 0x82c67a60
	ctx.lr = 0x82C6607C;
	sub_82C67A60(ctx, base);
	// 82C6607C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C66080: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82C66084: C3EA0AA4  lfs f31, 0xaa4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2724 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82C66088: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C6608C: 4800194D  bl 0x82c679d8
	ctx.lr = 0x82C66090;
	sub_82C679D8(ctx, base);
	// 82C66090: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82C66094: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C66098: 48001981  bl 0x82c67a18
	ctx.lr = 0x82C6609C;
	sub_82C67A18(ctx, base);
	// 82C6609C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C660A0: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82C660A4: 48001A4D  bl 0x82c67af0
	ctx.lr = 0x82C660A8;
	sub_82C67AF0(ctx, base);
	// 82C660A8: 907F0098  stw r3, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[3].u32 ) };
	// 82C660AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C660B0: 480019B1  bl 0x82c67a60
	ctx.lr = 0x82C660B4;
	sub_82C67A60(ctx, base);
	// 82C660B4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C660B8: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C660BC: C3C90C18  lfs f30, 0xc18(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3096 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82C660C0: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82C660C4: 48001915  bl 0x82c679d8
	ctx.lr = 0x82C660C8;
	sub_82C679D8(ctx, base);
	// 82C660C8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C660CC: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C660D0: C0280C4C  lfs f1, 0xc4c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3148 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C660D4: 48001945  bl 0x82c67a18
	ctx.lr = 0x82C660D8;
	sub_82C67A18(ctx, base);
	// 82C660D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C660DC: 4BFFFA0D  bl 0x82c65ae8
	ctx.lr = 0x82C660E0;
	sub_82C65AE8(ctx, base);
	// 82C660E0: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82C660E4: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82C660E8: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82C660EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C660F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C660F4: C0070C14  lfs f0, 0xc14(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C660F8: C1A60DB0  lfs f13, 0xdb0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(3504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C660FC: D01F0160  stfs f0, 0x160(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), tmp.u32 ) };
	// 82C66100: D1BF0164  stfs f13, 0x164(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), tmp.u32 ) };
	// 82C66104: 4BFFF81D  bl 0x82c65920
	ctx.lr = 0x82C66108;
	sub_82C65920(ctx, base);
	// 82C66108: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C6610C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C66110: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82C66114: 4BFFF875  bl 0x82c65988
	ctx.lr = 0x82C66118;
	sub_82C65988(ctx, base);
	// 82C66118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C6611C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C66120: 4BFFF731  bl 0x82c65850
	ctx.lr = 0x82C66124;
	sub_82C65850(ctx, base);
	// 82C66124: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C66128: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C6612C: 90BF0010  stw r5, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82C66130: 4BFFFA19  bl 0x82c65b48
	ctx.lr = 0x82C66134;
	sub_82C65B48(ctx, base);
	// 82C66134: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C66138: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82C6613C: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82C66140: 48043318  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C66148 size=88
    let mut pc: u32 = 0x82C66148;
    'dispatch: loop {
        match pc {
            0x82C66148 => {
    //   block [0x82C66148..0x82C661A0)
	// 82C66148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6614C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C66150: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C66154: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C66158: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82C6615C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C66160: 38600170  li r3, 0x170
	ctx.r[3].s64 = 368;
	// 82C66164: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C66168: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C6616C: 4BFFDDFD  bl 0x82c63f68
	ctx.lr = 0x82C66170;
	sub_82C63F68(ctx, base);
	// 82C66170: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C66174: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C66178: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C6617C: 4BFFFE2D  bl 0x82c65fa8
	ctx.lr = 0x82C66180;
	sub_82C65FA8(ctx, base);
	// 82C66180: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C66184: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C66188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C6618C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C66190: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82C66194: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C66198: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C6619C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C661A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C661A0 size=88
    let mut pc: u32 = 0x82C661A0;
    'dispatch: loop {
        match pc {
            0x82C661A0 => {
    //   block [0x82C661A0..0x82C661F8)
	// 82C661A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C661A4: 48043265  bl 0x82ca9408
	ctx.lr = 0x82C661A8;
	sub_82CA93D0(ctx, base);
	// 82C661A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C661AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C661B0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82C661B4: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82C661B8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C661BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C661C0: 40990030  ble cr6, 0x82c661f0
	if !ctx.cr[6].gt {
	pc = 0x82C661F0; continue 'dispatch;
	}
	// 82C661C4: 3BFD0008  addi r31, r29, 8
	ctx.r[31].s64 = ctx.r[29].s64 + 8;
	// 82C661C8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C661CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C661D0: 419A000C  beq cr6, 0x82c661dc
	if ctx.cr[6].eq {
	pc = 0x82C661DC; continue 'dispatch;
	}
	// 82C661D4: 48002645  bl 0x82c68818
	ctx.lr = 0x82C661D8;
	sub_82C68818(ctx, base);
	// 82C661D8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C661DC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C661E0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82C661E4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82C661E8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C661EC: 4198FFDC  blt cr6, 0x82c661c8
	if ctx.cr[6].lt {
	pc = 0x82C661C8; continue 'dispatch;
	}
	// 82C661F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C661F4: 48043264  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C661F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C661F8 size=56
    let mut pc: u32 = 0x82C661F8;
    'dispatch: loop {
        match pc {
            0x82C661F8 => {
    //   block [0x82C661F8..0x82C66230)
	// 82C661F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C661FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C66200: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C66204: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C66208: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C6620C: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82C66210: 48294E29  bl 0x82efb038
	ctx.lr = 0x82C66214;
	sub_82EFB038(ctx, base);
	// 82C66214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C66218: 4BFFFF89  bl 0x82c661a0
	ctx.lr = 0x82C6621C;
	sub_82C661A0(ctx, base);
	// 82C6621C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C66220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C66224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C66228: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C6622C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C66230 size=52
    let mut pc: u32 = 0x82C66230;
    'dispatch: loop {
        match pc {
            0x82C66230 => {
    //   block [0x82C66230..0x82C66264)
	// 82C66230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C66234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C66238: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C6623C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C66240: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C66244: 4BFFFFB5  bl 0x82c661f8
	ctx.lr = 0x82C66248;
	sub_82C661F8(ctx, base);
	// 82C66248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C6624C: 4BFFDD2D  bl 0x82c63f78
	ctx.lr = 0x82C66250;
	sub_82C63F78(ctx, base);
	// 82C66250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C66254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C66258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C6625C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C66260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C66268 size=144
    let mut pc: u32 = 0x82C66268;
    'dispatch: loop {
        match pc {
            0x82C66268 => {
    //   block [0x82C66268..0x82C662F8)
	// 82C66268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6626C: 480431A1  bl 0x82ca940c
	ctx.lr = 0x82C66270;
	sub_82CA93D0(ctx, base);
	// 82C66270: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C66274: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C66278: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82C6627C: 48002925  bl 0x82c68ba0
	ctx.lr = 0x82C66280;
	sub_82C68BA0(ctx, base);
	// 82C66280: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C66284: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C66288: 419A0034  beq cr6, 0x82c662bc
	if ctx.cr[6].eq {
	pc = 0x82C662BC; continue 'dispatch;
	}
	// 82C6628C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66290: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C66294: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C66298: 40990024  ble cr6, 0x82c662bc
	if !ctx.cr[6].gt {
	pc = 0x82C662BC; continue 'dispatch;
	}
	// 82C6629C: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 82C662A0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C662A4: 480025AD  bl 0x82c68850
	ctx.lr = 0x82C662A8;
	sub_82C68850(ctx, base);
	// 82C662A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C662AC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82C662B0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82C662B4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C662B8: 4198FFE8  blt cr6, 0x82c662a0
	if ctx.cr[6].lt {
	pc = 0x82C662A0; continue 'dispatch;
	}
	// 82C662BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C662C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C662C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C662C8: 40990028  ble cr6, 0x82c662f0
	if !ctx.cr[6].gt {
	pc = 0x82C662F0; continue 'dispatch;
	}
	// 82C662CC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C662D0: 397F0058  addi r11, r31, 0x58
	ctx.r[11].s64 = ctx.r[31].s64 + 88;
	// 82C662D4: C8090D38  lfd f0, 0xd38(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(3384 as u32) ) };
	// 82C662D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C662DC: D80B0000  stfd f0, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 82C662E0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C662E4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82C662E8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82C662EC: 4198FFEC  blt cr6, 0x82c662d8
	if ctx.cr[6].lt {
	pc = 0x82C662D8; continue 'dispatch;
	}
	// 82C662F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C662F4: 48043168  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C662F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C662F8 size=72
    let mut pc: u32 = 0x82C662F8;
    'dispatch: loop {
        match pc {
            0x82C662F8 => {
    //   block [0x82C662F8..0x82C66340)
	// 82C662F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C662FC: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C66300: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82C66304: 7D692030  slw r9, r11, r4
	if (ctx.r[4].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[4].u8 & 0x1F) as u32)) as u64;
	}
	// 82C66308: 7D685030  slw r8, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82C6630C: 7D0707B4  extsw r7, r8
	ctx.r[7].s64 = ctx.r[8].s32 as i64;
	// 82C66310: 7D2607B4  extsw r6, r9
	ctx.r[6].s64 = ctx.r[9].s32 as i64;
	// 82C66314: F8E1FFF0  std r7, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[7].u64 ) };
	// 82C66318: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C6631C: F8C1FFF0  std r6, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[6].u64 ) };
	// 82C66320: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C66324: FD60069C  fcfid f11, f0
	ctx.f[11].f64 = (ctx.f[0].s64 as f64);
	// 82C66328: FD806E9C  fcfid f12, f13
	ctx.f[12].f64 = (ctx.f[13].s64 as f64);
	// 82C6632C: FD205818  frsp f9, f11
	ctx.f[9].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82C66330: D9230050  stfd f9, 0x50(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.f[9].u64 ) };
	// 82C66334: FD406018  frsp f10, f12
	ctx.f[10].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82C66338: D9430048  stfd f10, 0x48(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.f[10].u64 ) };
	// 82C6633C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66340 size=52
    let mut pc: u32 = 0x82C66340;
    'dispatch: loop {
        match pc {
            0x82C66340 => {
    //   block [0x82C66340..0x82C66374)
	// 82C66340: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C66344: 40980008  bge cr6, 0x82c6634c
	if !ctx.cr[6].lt {
	pc = 0x82C6634C; continue 'dispatch;
	}
	// 82C66348: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C6634C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82C66350: 816BA878  lwz r11, -0x5788(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22408 as u32) ) } as u64;
	// 82C66354: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C66358: 41980008  blt cr6, 0x82c66360
	if ctx.cr[6].lt {
	pc = 0x82C66360; continue 'dispatch;
	}
	// 82C6635C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C66360: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82C66364: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C66368: 392BA848  addi r9, r11, -0x57b8
	ctx.r[9].s64 = ctx.r[11].s64 + -22456;
	// 82C6636C: 7C6A482E  lwzx r3, r10, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82C66370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66378 size=88
    let mut pc: u32 = 0x82C66378;
    'dispatch: loop {
        match pc {
            0x82C66378 => {
    //   block [0x82C66378..0x82C663D0)
	// 82C66378: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82C6637C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C66380: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C66384: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C66388: 40990040  ble cr6, 0x82c663c8
	if !ctx.cr[6].gt {
	pc = 0x82C663C8; continue 'dispatch;
	}
	// 82C6638C: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C66390: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66394: 7D0707B4  extsw r7, r8
	ctx.r[7].s64 = ctx.r[8].s32 as i64;
	// 82C66398: F8E1FFF0  std r7, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[7].u64 ) };
	// 82C6639C: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C663A0: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82C663A4: FC006818  frsp f0, f13
	ctx.f[0].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82C663A8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82C663AC: 419A0020  beq cr6, 0x82c663cc
	if ctx.cr[6].eq {
	pc = 0x82C663CC; continue 'dispatch;
	}
	// 82C663B0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82C663B4: 4198001C  blt cr6, 0x82c663d0
	if ctx.cr[6].lt {
		sub_82C663D0(ctx, base);
		return;
	}
	// 82C663B8: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82C663BC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C663C0: 7F035000  cmpw cr6, r3, r10
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82C663C4: 4198FFCC  blt cr6, 0x82c66390
	if ctx.cr[6].lt {
	pc = 0x82C66390; continue 'dispatch;
	}
	// 82C663C8: 386AFFFF  addi r3, r10, -1
	ctx.r[3].s64 = ctx.r[10].s64 + -1;
	// 82C663CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C663D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C663D0 size=92
    let mut pc: u32 = 0x82C663D0;
    'dispatch: loop {
        match pc {
            0x82C663D0 => {
    //   block [0x82C663D0..0x82C6642C)
	// 82C663D0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C663D4: 40990058  ble cr6, 0x82c6642c
	if !ctx.cr[6].gt {
		sub_82C6642C(ctx, base);
		return;
	}
	// 82C663D8: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C663DC: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C663E0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82C663E4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C663E8: 812BFFFC  lwz r9, -4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82C663EC: 7D4807B4  extsw r8, r10
	ctx.r[8].s64 = ctx.r[10].s32 as i64;
	// 82C663F0: 7D2707B4  extsw r7, r9
	ctx.r[7].s64 = ctx.r[9].s32 as i64;
	// 82C663F4: F901FFF0  std r8, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u64 ) };
	// 82C663F8: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C663FC: F8E1FFF0  std r7, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[7].u64 ) };
	// 82C66400: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C66404: FD60069C  fcfid f11, f0
	ctx.f[11].f64 = (ctx.f[0].s64 as f64);
	// 82C66408: FD806E9C  fcfid f12, f13
	ctx.f[12].f64 = (ctx.f[13].s64 as f64);
	// 82C6640C: FD205818  frsp f9, f11
	ctx.f[9].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82C66410: FD406018  frsp f10, f12
	ctx.f[10].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82C66414: ECE90828  fsubs f7, f9, f1
	ctx.f[7].f64 = (((ctx.f[9].f64 - ctx.f[1].f64) as f32) as f64);
	// 82C66418: ED015028  fsubs f8, f1, f10
	ctx.f[8].f64 = (((ctx.f[1].f64 - ctx.f[10].f64) as f32) as f64);
	// 82C6641C: FF083800  fcmpu cr6, f8, f7
	ctx.cr[6].compare_f64(ctx.f[8].f64, ctx.f[7].f64);
	// 82C66420: 4098FFAC  bge cr6, 0x82c663cc
	if !ctx.cr[6].lt {
		sub_82C66378(ctx, base);
		return;
	}
	// 82C66424: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 82C66428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C6642C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C6642C size=8
    let mut pc: u32 = 0x82C6642C;
    'dispatch: loop {
        match pc {
            0x82C6642C => {
    //   block [0x82C6642C..0x82C66434)
	// 82C6642C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C66430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C66438 size=168
    let mut pc: u32 = 0x82C66438;
    'dispatch: loop {
        match pc {
            0x82C66438 => {
    //   block [0x82C66438..0x82C664E0)
	// 82C66438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6643C: 48042FD1  bl 0x82ca940c
	ctx.lr = 0x82C66440;
	sub_82CA93D0(ctx, base);
	// 82C66440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C66444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C66448: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C6644C: 909F0030  stw r4, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[4].u32 ) };
	// 82C66450: 419A0080  beq cr6, 0x82c664d0
	if ctx.cr[6].eq {
	pc = 0x82C664D0; continue 'dispatch;
	}
	// 82C66454: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C66458: C03F0028  lfs f1, 0x28(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C6645C: 4BFFFF1D  bl 0x82c66378
	ctx.lr = 0x82C66460;
	sub_82C66378(ctx, base);
	// 82C66460: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66464: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 82C66468: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C6646C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C66470: 40990068  ble cr6, 0x82c664d8
	if !ctx.cr[6].gt {
	pc = 0x82C664D8; continue 'dispatch;
	}
	// 82C66474: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 82C66478: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C6647C: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C66480: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66484: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C66488: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C6648C: 7D69402E  lwzx r11, r9, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82C66490: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82C66494: 4800245D  bl 0x82c688f0
	ctx.lr = 0x82C66498;
	sub_82C688F0(ctx, base);
	// 82C66498: 80FF0030  lwz r7, 0x30(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C6649C: 80DF002C  lwz r6, 0x2c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C664A0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C664A4: 54C5103A  slwi r5, r6, 2
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82C664A8: 80870010  lwz r4, 0x10(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C664AC: 7C85202E  lwzx r4, r5, r4
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82C664B0: 480023E9  bl 0x82c68898
	ctx.lr = 0x82C664B4;
	sub_82C68898(ctx, base);
	// 82C664B4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C664B8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82C664BC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82C664C0: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82C664C4: 4198FFB4  blt cr6, 0x82c66478
	if ctx.cr[6].lt {
	pc = 0x82C66478; continue 'dispatch;
	}
	// 82C664C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C664CC: 48042F90  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C664D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C664D4: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82C664D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C664DC: 48042F80  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C664E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C664E0 size=52
    let mut pc: u32 = 0x82C664E0;
    'dispatch: loop {
        match pc {
            0x82C664E0 => {
    //   block [0x82C664E0..0x82C66514)
	// 82C664E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C664E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C664E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C664EC: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82C664F0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C664F4: 4BFFFE4D  bl 0x82c66340
	ctx.lr = 0x82C664F8;
	sub_82C66340(ctx, base);
	// 82C664F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C664FC: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82C66500: 4BFFFF39  bl 0x82c66438
	ctx.lr = 0x82C66504;
	sub_82C66438(ctx, base);
	// 82C66504: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C66508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C6650C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C66510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66518 size=8
    let mut pc: u32 = 0x82C66518;
    'dispatch: loop {
        match pc {
            0x82C66518 => {
    //   block [0x82C66518..0x82C66520)
	// 82C66518: 90830038  stw r4, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[4].u32 ) };
	// 82C6651C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66520 size=44
    let mut pc: u32 = 0x82C66520;
    'dispatch: loop {
        match pc {
            0x82C66520 => {
    //   block [0x82C66520..0x82C6654C)
	// 82C66520: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C66524: C80B0D38  lfd f0, 0xd38(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(3384 as u32) ) };
	// 82C66528: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C6652C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82C66530: C80B0D30  lfd f0, 0xd30(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(3376 as u32) ) };
	// 82C66534: 40990018  ble cr6, 0x82c6654c
	if !ctx.cr[6].gt {
		sub_82C6654C(ctx, base);
		return;
	}
	// 82C66538: FC01002A  fadd f0, f1, f0
	ctx.f[0].f64 = ctx.f[1].f64 + ctx.f[0].f64;
	// 82C6653C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82C66540: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82C66544: 8061FFF4  lwz r3, -0xc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82C66548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C6654C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C6654C size=20
    let mut pc: u32 = 0x82C6654C;
    'dispatch: loop {
        match pc {
            0x82C6654C => {
    //   block [0x82C6654C..0x82C66560)
	// 82C6654C: FC010028  fsub f0, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64 - ctx.f[0].f64;
	// 82C66550: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82C66554: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82C66558: 8061FFF4  lwz r3, -0xc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82C6655C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C66560 size=504
    let mut pc: u32 = 0x82C66560;
    'dispatch: loop {
        match pc {
            0x82C66560 => {
    //   block [0x82C66560..0x82C66758)
	// 82C66560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C66564: 48042E85  bl 0x82ca93e8
	ctx.lr = 0x82C66568;
	sub_82CA93D0(ctx, base);
	// 82C66568: 3981FF98  addi r12, r1, -0x68
	ctx.r[12].s64 = ctx.r[1].s64 + -104;
	// 82C6656C: 48047759  bl 0x82cadcc4
	ctx.lr = 0x82C66570;
	sub_82CADCA0(ctx, base);
	// 82C66570: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C66574: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C66578: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C6657C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C66580: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82C66584: C9BF0048  lfd f13, 0x48(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) };
	// 82C66588: 813F0034  lwz r9, 0x34(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C6658C: C80B0DE8  lfd f0, 0xde8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(3560 as u32) ) };
	// 82C66590: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66594: FFA06824  fdiv f29, f0, f13
	ctx.f[29].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 82C66598: CB6A0CB8  lfd f27, 0xcb8(r10)
	ctx.f[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(3256 as u32) ) };
	// 82C6659C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C665A0: FEFBE828  fsub f23, f27, f29
	ctx.f[23].f64 = ctx.f[27].f64 - ctx.f[29].f64;
	// 82C665A4: 419A016C  beq cr6, 0x82c66710
	if ctx.cr[6].eq {
	pc = 0x82C66710; continue 'dispatch;
	}
	// 82C665A8: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82C665AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C665B0: 40990198  ble cr6, 0x82c66748
	if !ctx.cr[6].gt {
	pc = 0x82C66748; continue 'dispatch;
	}
	// 82C665B4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C665B8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C665BC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C665C0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82C665C4: 3B3F0058  addi r25, r31, 0x58
	ctx.r[25].s64 = ctx.r[31].s64 + 88;
	// 82C665C8: CB0B0CB0  lfd f24, 0xcb0(r11)
	ctx.f[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(3248 as u32) ) };
	// 82C665CC: 3B1F0008  addi r24, r31, 8
	ctx.r[24].s64 = ctx.r[31].s64 + 8;
	// 82C665D0: CB2A0D38  lfd f25, 0xd38(r10)
	ctx.f[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(3384 as u32) ) };
	// 82C665D4: 7EA52050  subf r21, r5, r4
	ctx.r[21].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	// 82C665D8: CB89DDB8  lfd f28, -0x2248(r9)
	ctx.f[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-8776 as u32) ) };
	// 82C665DC: 7D55D02E  lwzx r10, r21, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82C665E0: C8390000  lfd f1, 0(r25)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) };
	// 82C665E4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C665E8: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82C665EC: 82D80000  lwz r22, 0(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C665F0: 409900F0  ble cr6, 0x82c666e0
	if !ctx.cr[6].gt {
	pc = 0x82C666E0; continue 'dispatch;
	}
	// 82C665F4: 3BBF003C  addi r29, r31, 0x3c
	ctx.r[29].s64 = ctx.r[31].s64 + 60;
	// 82C665F8: FF5BE824  fdiv f26, f27, f29
	ctx.f[26].f64 = ctx.f[27].f64 / ctx.f[29].f64;
	// 82C665FC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82C66600: 7F6B5050  subf r27, r11, r10
	ctx.r[27].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C66604: 7EFCBB78  mr r28, r23
	ctx.r[28].u64 = ctx.r[23].u64;
	// 82C66608: 7C1BF42E  lfsx f0, r27, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C6660C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C66610: FFC00828  fsub f30, f0, f1
	ctx.f[30].f64 = ctx.f[0].f64 - ctx.f[1].f64;
	// 82C66614: 480025AD  bl 0x82c68bc0
	ctx.lr = 0x82C66618;
	sub_82C68BC0(ctx, base);
	// 82C66618: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82C6661C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C66620: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82C66624: C9A10050  lfd f13, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C66628: FD806E9C  fcfid f12, f13
	ctx.f[12].f64 = (ctx.f[13].s64 as f64);
	// 82C6662C: FFEC0732  fmul f31, f12, f28
	ctx.f[31].f64 = ctx.f[12].f64 * ctx.f[28].f64;
	// 82C66630: 48002591  bl 0x82c68bc0
	ctx.lr = 0x82C66634;
	sub_82C68BC0(ctx, base);
	// 82C66634: 786A0020  clrldi r10, r3, 0x20
	ctx.r[10].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82C66638: 813F0038  lwz r9, 0x38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C6663C: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 82C66640: C9610058  lfd f11, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82C66644: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 82C66648: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C6664C: FD2AFF3A  fmadd f9, f10, f28, f31
	ctx.f[9].f64 = ctx.f[10].f64 * ctx.f[28].f64 + ctx.f[31].f64;
	// 82C66650: FD09D828  fsub f8, f9, f27
	ctx.f[8].f64 = ctx.f[9].f64 - ctx.f[27].f64;
	// 82C66654: FC080772  fmul f0, f8, f29
	ctx.f[0].f64 = ctx.f[8].f64 * ctx.f[29].f64;
	// 82C66658: 419A0008  beq cr6, 0x82c66660
	if ctx.cr[6].eq {
	pc = 0x82C66660; continue 'dispatch;
	}
	// 82C6665C: FC00C890  fmr f0, f25
	ctx.f[0].f64 = ctx.f[25].f64;
	// 82C66660: FC00F02A  fadd f0, f0, f30
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[30].f64;
	// 82C66664: FC2006B2  fmul f1, f0, f26
	ctx.f[1].f64 = ctx.f[0].f64 * ctx.f[26].f64;
	// 82C66668: 4BFFFEB9  bl 0x82c66520
	ctx.lr = 0x82C6666C;
	sub_82C66520(ctx, base);
	// 82C6666C: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 82C66670: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82C66674: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 82C66678: C9A10060  lfd f13, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C6667C: FD806E9C  fcfid f12, f13
	ctx.f[12].f64 = (ctx.f[13].s64 as f64);
	// 82C66680: FFEC0772  fmul f31, f12, f29
	ctx.f[31].f64 = ctx.f[12].f64 * ctx.f[29].f64;
	// 82C66684: 419A0018  beq cr6, 0x82c6669c
	if ctx.cr[6].eq {
	pc = 0x82C6669C; continue 'dispatch;
	}
	// 82C66688: FC1FF028  fsub f0, f31, f30
	ctx.f[0].f64 = ctx.f[31].f64 - ctx.f[30].f64;
	// 82C6668C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82C66690: FC200018  frsp f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82C66694: 48002295  bl 0x82c68928
	ctx.lr = 0x82C66698;
	sub_82C68928(ctx, base);
	// 82C66698: 48000008  b 0x82c666a0
	pc = 0x82C666A0; continue 'dispatch;
	// 82C6669C: FC20C890  fmr f1, f25
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[25].f64;
	// 82C666A0: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C666A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C666A8: 419A0008  beq cr6, 0x82c666b0
	if ctx.cr[6].eq {
	pc = 0x82C666B0; continue 'dispatch;
	}
	// 82C666AC: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 82C666B0: FF1FB800  fcmpu cr6, f31, f23
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[23].f64);
	// 82C666B4: 4099000C  ble cr6, 0x82c666c0
	if !ctx.cr[6].gt {
	pc = 0x82C666C0; continue 'dispatch;
	}
	// 82C666B8: FFE0B890  fmr f31, f23
	ctx.f[31].f64 = ctx.f[23].f64;
	// 82C666BC: 48000010  b 0x82c666cc
	pc = 0x82C666CC; continue 'dispatch;
	// 82C666C0: FF1FC000  fcmpu cr6, f31, f24
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[24].f64);
	// 82C666C4: 40980008  bge cr6, 0x82c666cc
	if !ctx.cr[6].lt {
	pc = 0x82C666CC; continue 'dispatch;
	}
	// 82C666C8: FFE0C090  fmr f31, f24
	ctx.f[31].f64 = ctx.f[24].f64;
	// 82C666CC: FC00F818  frsp f0, f31
	ctx.f[0].f64 = (ctx.f[31].f64 as f32) as f64;
	// 82C666D0: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C666D4: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82C666D8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82C666DC: 4082FF2C  bne 0x82c66608
	if !ctx.cr[0].eq {
	pc = 0x82C66608; continue 'dispatch;
	}
	// 82C666E0: 3A940001  addi r20, r20, 1
	ctx.r[20].s64 = ctx.r[20].s64 + 1;
	// 82C666E4: D8390000  stfd f1, 0(r25)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.f[1].u64 ) };
	// 82C666E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C666EC: 3B390008  addi r25, r25, 8
	ctx.r[25].s64 = ctx.r[25].s64 + 8;
	// 82C666F0: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82C666F4: 3B180004  addi r24, r24, 4
	ctx.r[24].s64 = ctx.r[24].s64 + 4;
	// 82C666F8: 7F145800  cmpw cr6, r20, r11
	ctx.cr[6].compare_i32(ctx.r[20].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C666FC: 4198FEE0  blt cr6, 0x82c665dc
	if ctx.cr[6].lt {
	pc = 0x82C665DC; continue 'dispatch;
	}
	// 82C66700: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82C66704: 3981FF98  addi r12, r1, -0x68
	ctx.r[12].s64 = ctx.r[1].s64 + -104;
	// 82C66708: 48047609  bl 0x82cadd10
	ctx.lr = 0x82C6670C;
	sub_82CADCEC(ctx, base);
	// 82C6670C: 48042D2C  b 0x82ca9438
	sub_82CA9420(ctx, base);
	return;
	// 82C66710: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C66714: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C66718: 40990030  ble cr6, 0x82c66748
	if !ctx.cr[6].gt {
	pc = 0x82C66748; continue 'dispatch;
	}
	// 82C6671C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C66720: 7F852050  subf r28, r5, r4
	ctx.r[28].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	// 82C66724: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82C66728: 7C9EE02E  lwzx r4, r30, r28
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82C6672C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66730: 48000291  bl 0x82c669c0
	ctx.lr = 0x82C66734;
	sub_82C669C0(ctx, base);
	// 82C66734: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66738: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82C6673C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82C66740: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C66744: 4198FFE0  blt cr6, 0x82c66724
	if ctx.cr[6].lt {
	pc = 0x82C66724; continue 'dispatch;
	}
	// 82C66748: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82C6674C: 3981FF98  addi r12, r1, -0x68
	ctx.r[12].s64 = ctx.r[1].s64 + -104;
	// 82C66750: 480475C1  bl 0x82cadd10
	ctx.lr = 0x82C66754;
	sub_82CADCEC(ctx, base);
	// 82C66754: 48042CE4  b 0x82ca9438
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C66758 size=132
    let mut pc: u32 = 0x82C66758;
    'dispatch: loop {
        match pc {
            0x82C66758 => {
    //   block [0x82C66758..0x82C667DC)
	// 82C66758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6675C: 48042CB1  bl 0x82ca940c
	ctx.lr = 0x82C66760;
	sub_82CA93D0(ctx, base);
	// 82C66760: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82C66764: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C66768: 38800098  li r4, 0x98
	ctx.r[4].s64 = 152;
	// 82C6676C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C66770: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C66774: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C66778: 4BFFD7C1  bl 0x82c63f38
	ctx.lr = 0x82C6677C;
	sub_82C63F38(ctx, base);
	// 82C6677C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82C66780: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C66784: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C66788: 4BFFFB71  bl 0x82c662f8
	ctx.lr = 0x82C6678C;
	sub_82C662F8(ctx, base);
	// 82C6678C: D3FF0028  stfs f31, 0x28(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82C66790: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82C66794: 4800244D  bl 0x82c68be0
	ctx.lr = 0x82C66798;
	sub_82C68BE0(ctx, base);
	// 82C66798: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6679C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C667A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C667A4: 4099002C  ble cr6, 0x82c667d0
	if !ctx.cr[6].gt {
	pc = 0x82C667D0; continue 'dispatch;
	}
	// 82C667A8: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 82C667AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C667B0: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82C667B4: 4800235D  bl 0x82c68b10
	ctx.lr = 0x82C667B8;
	sub_82C68B10(ctx, base);
	// 82C667B8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82C667BC: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C667C0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82C667C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C667C8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C667CC: 4198FFE0  blt cr6, 0x82c667ac
	if ctx.cr[6].lt {
	pc = 0x82C667AC; continue 'dispatch;
	}
	// 82C667D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C667D4: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82C667D8: 48042C84  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C667E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C667E0 size=88
    let mut pc: u32 = 0x82C667E0;
    'dispatch: loop {
        match pc {
            0x82C667E0 => {
    //   block [0x82C667E0..0x82C66838)
	// 82C667E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C667E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C667E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C667EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C667F0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82C667F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C667F8: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82C667FC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C66800: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C66804: 4BFFD765  bl 0x82c63f68
	ctx.lr = 0x82C66808;
	sub_82C63F68(ctx, base);
	// 82C66808: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C6680C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C66810: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C66814: 4BFFFF45  bl 0x82c66758
	ctx.lr = 0x82C66818;
	sub_82C66758(ctx, base);
	// 82C66818: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C6681C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C66820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C66824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C66828: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82C6682C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C66830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C66834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66838 size=4
    let mut pc: u32 = 0x82C66838;
    'dispatch: loop {
        match pc {
            0x82C66838 => {
    //   block [0x82C66838..0x82C6683C)
	// 82C66838: 4BFFD740  b 0x82c63f78
	sub_82C63F78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C66840 size=156
    let mut pc: u32 = 0x82C66840;
    'dispatch: loop {
        match pc {
            0x82C66840 => {
    //   block [0x82C66840..0x82C668DC)
	// 82C66840: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C66844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82C66848: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C6684C: 40990080  ble cr6, 0x82c668cc
	if !ctx.cr[6].gt {
	pc = 0x82C668CC; continue 'dispatch;
	}
	// 82C66850: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 82C66854: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C66858: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82C6685C: 7CE32050  subf r7, r3, r4
	ctx.r[7].s64 = ctx.r[4].s64 - ctx.r[3].s64;
	// 82C66860: C96B13F8  lfd f11, 0x13f8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(5112 as u32) ) };
	// 82C66864: C18A0C18  lfs f12, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C66868: 7D67482E  lwzx r11, r7, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82C6686C: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C66870: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82C66874: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82C66878: 4099003C  ble cr6, 0x82c668b4
	if !ctx.cr[6].gt {
	pc = 0x82C668B4; continue 'dispatch;
	}
	// 82C6687C: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C66880: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C66884: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82C66888: 40980008  bge cr6, 0x82c66890
	if !ctx.cr[6].lt {
	pc = 0x82C66890; continue 'dispatch;
	}
	// 82C6688C: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82C66890: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C66894: 40990008  ble cr6, 0x82c6689c
	if !ctx.cr[6].gt {
	pc = 0x82C6689C; continue 'dispatch;
	}
	// 82C66898: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 82C6689C: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82C668A0: 40980008  bge cr6, 0x82c668a8
	if !ctx.cr[6].lt {
	pc = 0x82C668A8; continue 'dispatch;
	}
	// 82C668A4: D18B0000  stfs f12, 0(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C668A8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C668AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C668B0: 4199FFCC  bgt cr6, 0x82c6687c
	if ctx.cr[6].gt {
	pc = 0x82C6687C; continue 'dispatch;
	}
	// 82C668B4: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82C668B8: D1A90000  stfs f13, 0(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C668BC: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C668C0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82C668C4: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C668C8: 4198FFA0  blt cr6, 0x82c66868
	if ctx.cr[6].lt {
	pc = 0x82C66868; continue 'dispatch;
	}
	// 82C668CC: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C668D0: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82C668D4: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82C668D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C668E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C668E0 size=16
    let mut pc: u32 = 0x82C668E0;
    'dispatch: loop {
        match pc {
            0x82C668E0 => {
    //   block [0x82C668E0..0x82C668F0)
	// 82C668E0: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C668E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C668E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C668EC: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C668F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C668F0 size=40
    let mut pc: u32 = 0x82C668F0;
    'dispatch: loop {
        match pc {
            0x82C668F0 => {
    //   block [0x82C668F0..0x82C66918)
	// 82C668F0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C668F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C668F8: C0090C18  lfs f0, 0xc18(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C668FC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C66900: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C66904: 81230024  lwz r9, 0x24(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C66908: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C6690C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82C66910: 4198FFEC  blt cr6, 0x82c668fc
	if ctx.cr[6].lt {
	pc = 0x82C668FC; continue 'dispatch;
	}
	// 82C66914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C66918 size=92
    let mut pc: u32 = 0x82C66918;
    'dispatch: loop {
        match pc {
            0x82C66918 => {
    //   block [0x82C66918..0x82C66974)
	// 82C66918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6691C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C66920: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C66924: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C66928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C6692C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C66930: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82C66934: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C66938: 4BFFD601  bl 0x82c63f38
	ctx.lr = 0x82C6693C;
	sub_82C63F38(ctx, base);
	// 82C6693C: 2F1E0008  cmpwi cr6, r30, 8
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8, &mut ctx.xer);
	// 82C66940: 40990008  ble cr6, 0x82c66948
	if !ctx.cr[6].gt {
	pc = 0x82C66948; continue 'dispatch;
	}
	// 82C66944: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 82C66948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C6694C: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82C66950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C66954: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82C66958: 4BFFFF89  bl 0x82c668e0
	ctx.lr = 0x82C6695C;
	sub_82C668E0(ctx, base);
	// 82C6695C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C66960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C66964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C66968: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C6696C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C66970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C66978 size=72
    let mut pc: u32 = 0x82C66978;
    'dispatch: loop {
        match pc {
            0x82C66978 => {
    //   block [0x82C66978..0x82C669C0)
	// 82C66978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6697C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C66980: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C66984: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C66988: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C6698C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C66990: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82C66994: 4BFFD5D5  bl 0x82c63f68
	ctx.lr = 0x82C66998;
	sub_82C63F68(ctx, base);
	// 82C66998: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C6699C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C669A0: 4BFFFF79  bl 0x82c66918
	ctx.lr = 0x82C669A4;
	sub_82C66918(ctx, base);
	// 82C669A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C669A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C669AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C669B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C669B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C669B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C669BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C669C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C669C0 size=28
    let mut pc: u32 = 0x82C669C0;
    'dispatch: loop {
        match pc {
            0x82C669C0 => {
    //   block [0x82C669C0..0x82C669DC)
	// 82C669C0: 7F032040  cmplw cr6, r3, r4
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82C669C4: 40990034  ble cr6, 0x82c669f8
	if !ctx.cr[6].gt {
		sub_82C669F8(ctx, base);
		return;
	}
	// 82C669C8: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C669CC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82C669D0: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82C669D4: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82C669D8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C669DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C669DC size=28
    let mut pc: u32 = 0x82C669DC;
    'dispatch: loop {
        match pc {
            0x82C669DC => {
    //   block [0x82C669DC..0x82C669F8)
	// 82C669DC: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82C669E0: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 82C669E4: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82C669E8: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C669EC: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C669F0: 4082FFEC  bne 0x82c669dc
	if !ctx.cr[0].eq {
	pc = 0x82C669DC; continue 'dispatch;
	}
	// 82C669F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C669F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C669F8 size=8
    let mut pc: u32 = 0x82C669F8;
    'dispatch: loop {
        match pc {
            0x82C669F8 => {
    //   block [0x82C669F8..0x82C66A00)
	// 82C669F8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82C669FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C66A00 size=28
    let mut pc: u32 = 0x82C66A00;
    'dispatch: loop {
        match pc {
            0x82C66A00 => {
    //   block [0x82C66A00..0x82C66A1C)
	// 82C66A00: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C66A04: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82C66A08: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C66A0C: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82C66A10: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82C66A14: 4082FFEC  bne 0x82c66a00
	if !ctx.cr[0].eq {
	pc = 0x82C66A00; continue 'dispatch;
	}
	// 82C66A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C66A20 size=128
    let mut pc: u32 = 0x82C66A20;
    'dispatch: loop {
        match pc {
            0x82C66A20 => {
    //   block [0x82C66A20..0x82C66AA0)
	// 82C66A20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82C66A24: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 82C66A28: 41980070  blt cr6, 0x82c66a98
	if ctx.cr[6].lt {
	pc = 0x82C66A98; continue 'dispatch;
	}
	// 82C66A2C: 3965FFFC  addi r11, r5, -4
	ctx.r[11].s64 = ctx.r[5].s64 + -4;
	// 82C66A30: 3944000C  addi r10, r4, 0xc
	ctx.r[10].s64 = ctx.r[4].s64 + 12;
	// 82C66A34: 5569F0BE  srwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C66A38: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82C66A3C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82C66A40: 7CE32050  subf r7, r3, r4
	ctx.r[7].s64 = ctx.r[4].s64 - ctx.r[3].s64;
	// 82C66A44: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C66A48: C00AFFF4  lfs f0, -0xc(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C66A4C: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C66A50: C1ABFFFC  lfs f13, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C66A54: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82C66A58: C16B0000  lfs f11, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82C66A5C: D18BFFFC  stfs f12, -4(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82C66A60: 7D475C2E  lfsx f10, r7, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82C66A64: ED2A582A  fadds f9, f10, f11
	ctx.f[9].f64 = ((ctx.f[10].f64 + ctx.f[11].f64) as f32) as f64;
	// 82C66A68: C10B0004  lfs f8, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82C66A6C: D12B0000  stfs f9, 0(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C66A70: C0EAFFFC  lfs f7, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82C66A74: ECC7402A  fadds f6, f7, f8
	ctx.f[6].f64 = ((ctx.f[7].f64 + ctx.f[8].f64) as f32) as f64;
	// 82C66A78: C0AB0008  lfs f5, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82C66A7C: D0CB0004  stfs f6, 4(r11)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C66A80: C08A0000  lfs f4, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82C66A84: EC65202A  fadds f3, f5, f4
	ctx.f[3].f64 = ((ctx.f[5].f64 + ctx.f[4].f64) as f32) as f64;
	// 82C66A88: D06B0008  stfs f3, 8(r11)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C66A8C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82C66A90: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82C66A94: 4082FFB4  bne 0x82c66a48
	if !ctx.cr[0].eq {
	pc = 0x82C66A48; continue 'dispatch;
	}
	// 82C66A98: 7F082840  cmplw cr6, r8, r5
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82C66A9C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C66AA0 size=48
    let mut pc: u32 = 0x82C66AA0;
    'dispatch: loop {
        match pc {
            0x82C66AA0 => {
    //   block [0x82C66AA0..0x82C66AD0)
	// 82C66AA0: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C66AA4: 7D232050  subf r9, r3, r4
	ctx.r[9].s64 = ctx.r[4].s64 - ctx.r[3].s64;
	// 82C66AA8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82C66AAC: 7D482850  subf r10, r8, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[8].s64;
	// 82C66AB0: 7C0B4C2E  lfsx f0, r11, r9
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C66AB4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C66AB8: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C66ABC: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82C66AC0: D18B0000  stfs f12, 0(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C66AC4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C66AC8: 4082FFE8  bne 0x82c66ab0
	if !ctx.cr[0].eq {
	pc = 0x82C66AB0; continue 'dispatch;
	}
	// 82C66ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C66AD0 size=244
    let mut pc: u32 = 0x82C66AD0;
    'dispatch: loop {
        match pc {
            0x82C66AD0 => {
    //   block [0x82C66AD0..0x82C66BC4)
	// 82C66AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C66AD4: 48042935  bl 0x82ca9408
	ctx.lr = 0x82C66AD8;
	sub_82CA93D0(ctx, base);
	// 82C66AD8: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82C66ADC: 409900E4  ble cr6, 0x82c66bc0
	if !ctx.cr[6].gt {
	pc = 0x82C66BC0; continue 'dispatch;
	}
	// 82C66AE0: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82C66AE4: 7FA41850  subf r29, r4, r3
	ctx.r[29].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	// 82C66AE8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C66AEC: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 82C66AF0: 41980084  blt cr6, 0x82c66b74
	if ctx.cr[6].lt {
	pc = 0x82C66B74; continue 'dispatch;
	}
	// 82C66AF4: 3946FFFC  addi r10, r6, -4
	ctx.r[10].s64 = ctx.r[6].s64 + -4;
	// 82C66AF8: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66AFC: 7D1D202E  lwzx r8, r29, r4
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82C66B00: 39650004  addi r11, r5, 4
	ctx.r[11].s64 = ctx.r[5].s64 + 4;
	// 82C66B04: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C66B08: 7FE53850  subf r31, r5, r7
	ctx.r[31].s64 = ctx.r[7].s64 - ctx.r[5].s64;
	// 82C66B0C: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82C66B10: 39470008  addi r10, r7, 8
	ctx.r[10].s64 = ctx.r[7].s64 + 8;
	// 82C66B14: 7C654050  subf r3, r5, r8
	ctx.r[3].s64 = ctx.r[8].s64 - ctx.r[5].s64;
	// 82C66B18: 7CE74050  subf r7, r7, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 82C66B1C: 553E103A  slwi r30, r9, 2
	ctx.r[30].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82C66B20: C00BFFFC  lfs f0, -4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C66B24: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C66B28: C1A80000  lfs f13, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C66B2C: ED800372  fmuls f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82C66B30: D18AFFF8  stfs f12, -8(r10)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82C66B34: 7D635C2E  lfsx f11, r3, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82C66B38: C14B0000  lfs f10, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82C66B3C: ED2B02B2  fmuls f9, f11, f10
	ctx.f[9].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 82C66B40: 7D3F5D2E  stfsx f9, r31, r11
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 82C66B44: 7D07542E  lfsx f8, r7, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82C66B48: C0EB0004  lfs f7, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82C66B4C: ECC801F2  fmuls f6, f8, f7
	ctx.f[6].f64 = (((ctx.f[8].f64 * ctx.f[7].f64) as f32) as f64);
	// 82C66B50: D0CA0000  stfs f6, 0(r10)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C66B54: C0AB0008  lfs f5, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82C66B58: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82C66B5C: C088000C  lfs f4, 0xc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82C66B60: EC650132  fmuls f3, f5, f4
	ctx.f[3].f64 = (((ctx.f[5].f64 * ctx.f[4].f64) as f32) as f64);
	// 82C66B64: D06A0004  stfs f3, 4(r10)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C66B68: 39080010  addi r8, r8, 0x10
	ctx.r[8].s64 = ctx.r[8].s64 + 16;
	// 82C66B6C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82C66B70: 4082FFB0  bne 0x82c66b20
	if !ctx.cr[0].eq {
	pc = 0x82C66B20; continue 'dispatch;
	}
	// 82C66B74: 7F1E3040  cmplw cr6, r30, r6
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82C66B78: 4098003C  bge cr6, 0x82c66bb4
	if !ctx.cr[6].lt {
	pc = 0x82C66BB4; continue 'dispatch;
	}
	// 82C66B7C: 7D3D202E  lwzx r9, r29, r4
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82C66B80: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C66B84: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66B88: 7D5E3050  subf r10, r30, r6
	ctx.r[10].s64 = ctx.r[6].s64 - ctx.r[30].s64;
	// 82C66B8C: 7D254850  subf r9, r5, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[5].s64;
	// 82C66B90: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82C66B94: 7D054050  subf r8, r5, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[5].s64;
	// 82C66B98: 7C095C2E  lfsx f0, r9, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C66B9C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C66BA0: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C66BA4: ED800372  fmuls f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82C66BA8: 7D885D2E  stfsx f12, r8, r11
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 82C66BAC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C66BB0: 4082FFE8  bne 0x82c66b98
	if !ctx.cr[0].eq {
	pc = 0x82C66B98; continue 'dispatch;
	}
	// 82C66BB4: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82C66BB8: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82C66BBC: 4082FF2C  bne 0x82c66ae8
	if !ctx.cr[0].eq {
	pc = 0x82C66AE8; continue 'dispatch;
	}
	// 82C66BC0: 48042898  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66BC8 size=12
    let mut pc: u32 = 0x82C66BC8;
    'dispatch: loop {
        match pc {
            0x82C66BC8 => {
    //   block [0x82C66BC8..0x82C66BD4)
	// 82C66BC8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C66BCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C66BD0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66BD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C66BD4 size=32
    let mut pc: u32 = 0x82C66BD4;
    'dispatch: loop {
        match pc {
            0x82C66BD4 => {
    //   block [0x82C66BD4..0x82C66BF4)
	// 82C66BD4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C66BD8: C0230000  lfs f1, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C66BDC: 554907BC  rlwinm r9, r10, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82C66BE0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C66BE4: 419A0010  beq cr6, 0x82c66bf4
	if ctx.cr[6].eq {
		sub_82C66BF4(ctx, base);
		return;
	}
	// 82C66BE8: 80A3001C  lwz r5, 0x1c(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C66BEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C66BF0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66BF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66BF4 size=12
    let mut pc: u32 = 0x82C66BF4;
    'dispatch: loop {
        match pc {
            0x82C66BF4 => {
    //   block [0x82C66BF4..0x82C66C00)
	// 82C66BF4: 8083001C  lwz r4, 0x1c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C66BF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C66BFC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66C00 size=4
    let mut pc: u32 = 0x82C66C00;
    'dispatch: loop {
        match pc {
            0x82C66C00 => {
    //   block [0x82C66C00..0x82C66C04)
	// 82C66C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C66C08 size=52
    let mut pc: u32 = 0x82C66C08;
    'dispatch: loop {
        match pc {
            0x82C66C08 => {
    //   block [0x82C66C08..0x82C66C3C)
	// 82C66C08: C003000C  lfs f0, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C66C0C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82C66C10: 41980010  blt cr6, 0x82c66c20
	if ctx.cr[6].lt {
	pc = 0x82C66C20; continue 'dispatch;
	}
	// 82C66C14: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C66C18: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82C66C1C: 40990008  ble cr6, 0x82c66c24
	if !ctx.cr[6].gt {
	pc = 0x82C66C24; continue 'dispatch;
	}
	// 82C66C20: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 82C66C24: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C66C28: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 82C66C2C: 409A0020  bne cr6, 0x82c66c4c
	if !ctx.cr[6].eq {
		sub_82C66C4C(ctx, base);
		return;
	}
	// 82C66C30: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C66C34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C66C38: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66C3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66C3C size=16
    let mut pc: u32 = 0x82C66C3C;
    'dispatch: loop {
        match pc {
            0x82C66C3C => {
    //   block [0x82C66C3C..0x82C66C4C)
	// 82C66C3C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C66C40: 556A0738  rlwinm r10, r11, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C66C44: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C66C48: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66C4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C66C4C size=8
    let mut pc: u32 = 0x82C66C4C;
    'dispatch: loop {
        match pc {
            0x82C66C4C => {
    //   block [0x82C66C4C..0x82C66C54)
	// 82C66C4C: D0230000  stfs f1, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C66C50: 4BFFFF78  b 0x82c66bc8
	sub_82C66BC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66C54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66C54 size=4
    let mut pc: u32 = 0x82C66C54;
    'dispatch: loop {
        match pc {
            0x82C66C54 => {
    //   block [0x82C66C54..0x82C66C58)
	// 82C66C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C66C58 size=8
    let mut pc: u32 = 0x82C66C58;
    'dispatch: loop {
        match pc {
            0x82C66C58 => {
    //   block [0x82C66C58..0x82C66C60)
	// 82C66C58: D023000C  stfs f1, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C66C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C66C60 size=8
    let mut pc: u32 = 0x82C66C60;
    'dispatch: loop {
        match pc {
            0x82C66C60 => {
    //   block [0x82C66C60..0x82C66C68)
	// 82C66C60: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82C66C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C66C68 size=88
    let mut pc: u32 = 0x82C66C68;
    'dispatch: loop {
        match pc {
            0x82C66C68 => {
    //   block [0x82C66C68..0x82C66CC0)
	// 82C66C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C66C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C66C70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C66C74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C66C78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C66C7C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C66C80: 409A000C  bne cr6, 0x82c66c8c
	if !ctx.cr[6].eq {
	pc = 0x82C66C8C; continue 'dispatch;
	}
	// 82C66C84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C66C88: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 82C66C8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C66C90: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C66C94: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82C66C98: 54A3103A  slwi r3, r5, 2
	ctx.r[3].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C66C9C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C66CA0: 4BFF92C9  bl 0x82c5ff68
	ctx.lr = 0x82C66CA4;
	sub_82C5FF68(ctx, base);
	// 82C66CA4: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82C66CA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C66CAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C66CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C66CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C66CB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C66CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66CC0 size=8
    let mut pc: u32 = 0x82C66CC0;
    'dispatch: loop {
        match pc {
            0x82C66CC0 => {
    //   block [0x82C66CC0..0x82C66CC8)
	// 82C66CC0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C66CC4: 4BFF9394  b 0x82c60058
	sub_82C60058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66CC8 size=32
    let mut pc: u32 = 0x82C66CC8;
    'dispatch: loop {
        match pc {
            0x82C66CC8 => {
    //   block [0x82C66CC8..0x82C66CE8)
	// 82C66CC8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C66CCC: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C66CD0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C66CD4: 7C8A492E  stwx r4, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[4].u32) };
	// 82C66CD8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C66CDC: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82C66CE0: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82C66CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66CE8 size=16
    let mut pc: u32 = 0x82C66CE8;
    'dispatch: loop {
        match pc {
            0x82C66CE8 => {
    //   block [0x82C66CE8..0x82C66CF8)
	// 82C66CE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C66CEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C66CF0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82C66CF4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66CF8 size=16
    let mut pc: u32 = 0x82C66CF8;
    'dispatch: loop {
        match pc {
            0x82C66CF8 => {
    //   block [0x82C66CF8..0x82C66D08)
	// 82C66CF8: 5463083C  slwi r3, r3, 1
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C66CFC: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82C66D00: 4199FFF8  bgt cr6, 0x82c66cf8
	if ctx.cr[6].gt {
	pc = 0x82C66CF8; continue 'dispatch;
	}
	// 82C66D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66D08 size=16
    let mut pc: u32 = 0x82C66D08;
    'dispatch: loop {
        match pc {
            0x82C66D08 => {
    //   block [0x82C66D08..0x82C66D18)
	// 82C66D08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C66D0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C66D10: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82C66D14: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66D18 size=20
    let mut pc: u32 = 0x82C66D18;
    'dispatch: loop {
        match pc {
            0x82C66D18 => {
    //   block [0x82C66D18..0x82C66D2C)
	// 82C66D18: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82C66D1C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82C66D20: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82C66D24: 4199FFF4  bgt cr6, 0x82c66d18
	if ctx.cr[6].gt {
	pc = 0x82C66D18; continue 'dispatch;
	}
	// 82C66D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66D30 size=8
    let mut pc: u32 = 0x82C66D30;
    'dispatch: loop {
        match pc {
            0x82C66D30 => {
    //   block [0x82C66D30..0x82C66D38)
	// 82C66D30: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66D34: 4BFF9324  b 0x82c60058
	sub_82C60058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C66D38 size=164
    let mut pc: u32 = 0x82C66D38;
    'dispatch: loop {
        match pc {
            0x82C66D38 => {
    //   block [0x82C66D38..0x82C66DDC)
	// 82C66D38: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C66D3C: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66D40: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C66D44: 7D075E30  sraw r7, r8, r11
	tmp.u32 = ctx.r[11].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[8].s32 >> tmp.u32) as i64;
	// 82C66D48: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66D4C: 7CEB5038  and r11, r7, r10
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[10].u64;
	// 82C66D50: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C66D54: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C66D58: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C66D5C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82C66D60: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66D64: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82C66D68: 419A004C  beq cr6, 0x82c66db4
	if ctx.cr[6].eq {
	pc = 0x82C66DB4; continue 'dispatch;
	}
	// 82C66D6C: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66D70: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C66D74: 419A0034  beq cr6, 0x82c66da8
	if ctx.cr[6].eq {
	pc = 0x82C66DA8; continue 'dispatch;
	}
	// 82C66D78: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C66D7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C66D80: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82C66D84: 41980008  blt cr6, 0x82c66d8c
	if ctx.cr[6].lt {
	pc = 0x82C66D8C; continue 'dispatch;
	}
	// 82C66D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C66D8C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C66D90: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C66D94: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C66D98: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82C66D9C: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66DA0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C66DA4: 409AFFC8  bne cr6, 0x82c66d6c
	if !ctx.cr[6].eq {
	pc = 0x82C66D6C; continue 'dispatch;
	}
	// 82C66DA8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66DAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C66DB0: 409A0010  bne cr6, 0x82c66dc0
	if !ctx.cr[6].eq {
	pc = 0x82C66DC0; continue 'dispatch;
	}
	// 82C66DB4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C66DB8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C66DBC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C66DC0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66DC4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C66DC8: 81240004  lwz r9, 4(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C66DCC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C66DD0: 81040008  lwz r8, 8(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C66DD4: 910A0008  stw r8, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82C66DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66DE0 size=60
    let mut pc: u32 = 0x82C66DE0;
    'dispatch: loop {
        match pc {
            0x82C66DE0 => {
    //   block [0x82C66DE0..0x82C66E1C)
	// 82C66DE0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C66DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C66DE8: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82C66DEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C66DF0: 40990024  ble cr6, 0x82c66e14
	if !ctx.cr[6].gt {
	pc = 0x82C66E14; continue 'dispatch;
	}
	// 82C66DF4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82C66DF8: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66DFC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C66E00: 7D2B412E  stwx r9, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82C66E04: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82C66E08: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C66E0C: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82C66E10: 4198FFE8  blt cr6, 0x82c66df8
	if ctx.cr[6].lt {
	pc = 0x82C66DF8; continue 'dispatch;
	}
	// 82C66E14: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C66E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C66E20 size=120
    let mut pc: u32 = 0x82C66E20;
    'dispatch: loop {
        match pc {
            0x82C66E20 => {
    //   block [0x82C66E20..0x82C66E98)
	// 82C66E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C66E24: 480425DD  bl 0x82ca9400
	ctx.lr = 0x82C66E28;
	sub_82CA93D0(ctx, base);
	// 82C66E28: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C66E2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C66E30: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C66E34: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82C66E38: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82C66E3C: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82C66E40: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C66E44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C66E48: 40990044  ble cr6, 0x82c66e8c
	if !ctx.cr[6].gt {
	pc = 0x82C66E8C; continue 'dispatch;
	}
	// 82C66E4C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82C66E50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66E54: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82C66E58: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82C66E5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C66E60: 419A0018  beq cr6, 0x82c66e78
	if ctx.cr[6].eq {
	pc = 0x82C66E78; continue 'dispatch;
	}
	// 82C66E64: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82C66E68: 7F6903A6  mtctr r27
	ctx.ctr.u64 = ctx.r[27].u64;
	// 82C66E6C: 4E800421  bctrl
	ctx.lr = 0x82C66E70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C66E70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66E74: 7FBE592E  stwx r29, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 82C66E78: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C66E7C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82C66E80: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82C66E84: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C66E88: 4198FFC8  blt cr6, 0x82c66e50
	if ctx.cr[6].lt {
	pc = 0x82C66E50; continue 'dispatch;
	}
	// 82C66E8C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C66E90: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C66E94: 480425BC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C66E98 size=128
    let mut pc: u32 = 0x82C66E98;
    'dispatch: loop {
        match pc {
            0x82C66E98 => {
    //   block [0x82C66E98..0x82C66F18)
	// 82C66E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C66E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C66EA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C66EA4: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82C66EA8: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82C66EAC: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C66EB0: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C66EB4: 409A000C  bne cr6, 0x82c66ec0
	if !ctx.cr[6].eq {
	pc = 0x82C66EC0; continue 'dispatch;
	}
	// 82C66EB8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C66EBC: 38CB0CA0  addi r6, r11, 0xca0
	ctx.r[6].s64 = ctx.r[11].s64 + 3232;
	// 82C66EC0: D027000C  stfs f1, 0xc(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C66EC4: 90C70008  stw r6, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82C66EC8: D0470010  stfs f2, 0x10(r7)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82C66ECC: 91470018  stw r10, 0x18(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82C66ED0: D0670014  stfs f3, 0x14(r7)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82C66ED4: 9087001C  stw r4, 0x1c(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82C66ED8: D0670000  stfs f3, 0(r7)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C66EDC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82C66EE0: 419A0010  beq cr6, 0x82c66ef0
	if ctx.cr[6].eq {
	pc = 0x82C66EF0; continue 'dispatch;
	}
	// 82C66EE4: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82C66EE8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82C66EEC: 4BFFFDDD  bl 0x82c66cc8
	ctx.lr = 0x82C66EF0;
	sub_82C66CC8(ctx, base);
	// 82C66EF0: 81670004  lwz r11, 4(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C66EF4: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82C66EF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C66EFC: 419A000C  beq cr6, 0x82c66f08
	if ctx.cr[6].eq {
	pc = 0x82C66F08; continue 'dispatch;
	}
	// 82C66F00: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82C66F04: 4BFFFCC5  bl 0x82c66bc8
	ctx.lr = 0x82C66F08;
	sub_82C66BC8(ctx, base);
	// 82C66F08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C66F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C66F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C66F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C66F18 size=112
    let mut pc: u32 = 0x82C66F18;
    'dispatch: loop {
        match pc {
            0x82C66F18 => {
    //   block [0x82C66F18..0x82C66F88)
	// 82C66F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C66F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C66F20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C66F24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C66F28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C66F2C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C66F30: 4BFFFDB9  bl 0x82c66ce8
	ctx.lr = 0x82C66F34;
	sub_82C66CE8(ctx, base);
	// 82C66F34: 546B083C  slwi r11, r3, 1
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C66F38: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C66F3C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C66F40: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C66F44: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C66F48: 4BFF9021  bl 0x82c5ff68
	ctx.lr = 0x82C66F4C;
	sub_82C5FF68(ctx, base);
	// 82C66F4C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C66F50: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C66F54: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82C66F58: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82C66F5C: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82C66F60: 4BFFFDA9  bl 0x82c66d08
	ctx.lr = 0x82C66F64;
	sub_82C66D08(ctx, base);
	// 82C66F64: 3923FFFF  addi r9, r3, -1
	ctx.r[9].s64 = ctx.r[3].s64 + -1;
	// 82C66F68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C66F6C: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82C66F70: 4BFFFE71  bl 0x82c66de0
	ctx.lr = 0x82C66F74;
	sub_82C66DE0(ctx, base);
	// 82C66F74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C66F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C66F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C66F80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C66F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C66F88 size=56
    let mut pc: u32 = 0x82C66F88;
    'dispatch: loop {
        match pc {
            0x82C66F88 => {
    //   block [0x82C66F88..0x82C66FC0)
	// 82C66F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C66F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C66F90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C66F94: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C66F98: 816100C4  lwz r11, 0xc4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 82C66F9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C66FA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82C66FA4: 4BFFFEF5  bl 0x82c66e98
	ctx.lr = 0x82C66FA8;
	sub_82C66E98(ctx, base);
	// 82C66FA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C66FAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C66FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C66FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C66FB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C66FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66FC0 size=8
    let mut pc: u32 = 0x82C66FC0;
    'dispatch: loop {
        match pc {
            0x82C66FC0 => {
    //   block [0x82C66FC0..0x82C66FC8)
	// 82C66FC0: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C66FC4: 4BBDE7EC  b 0x828457b0
	sub_828457B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66FC8 size=12
    let mut pc: u32 = 0x82C66FC8;
    'dispatch: loop {
        match pc {
            0x82C66FC8 => {
    //   block [0x82C66FC8..0x82C66FD4)
	// 82C66FC8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C66FCC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C66FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66FD8 size=28
    let mut pc: u32 = 0x82C66FD8;
    'dispatch: loop {
        match pc {
            0x82C66FD8 => {
    //   block [0x82C66FD8..0x82C66FF4)
	// 82C66FD8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C66FDC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C66FE0: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C66FE4: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C66FE8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C66FEC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C66FF0: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C66FF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C66FF4 size=20
    let mut pc: u32 = 0x82C66FF4;
    'dispatch: loop {
        match pc {
            0x82C66FF4 => {
    //   block [0x82C66FF4..0x82C67008)
	// 82C66FF4: 988B0000  stb r4, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 82C66FF8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C66FFC: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82C67000: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82C67004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67008 size=16
    let mut pc: u32 = 0x82C67008;
    'dispatch: loop {
        match pc {
            0x82C67008 => {
    //   block [0x82C67008..0x82C67018)
	// 82C67008: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C6700C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C67010: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82C67014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C67018 size=92
    let mut pc: u32 = 0x82C67018;
    'dispatch: loop {
        match pc {
            0x82C67018 => {
    //   block [0x82C67018..0x82C67074)
	// 82C67018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6701C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C67020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C67024: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82C67028: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C6702C: 806A000C  lwz r3, 0xc(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C67030: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67034: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82C67038: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82C6703C: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82C67040: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C67044: 7F055800  cmpw cr6, r5, r11
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C67048: 41980008  blt cr6, 0x82c67050
	if ctx.cr[6].lt {
	pc = 0x82C67050; continue 'dispatch;
	}
	// 82C6704C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82C67050: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 82C67054: 4BFFE435  bl 0x82c65488
	ctx.lr = 0x82C67058;
	sub_82C65488(ctx, base);
	// 82C67058: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C6705C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82C67060: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C67064: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C67068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C6706C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C67070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67078 size=52
    let mut pc: u32 = 0x82C67078;
    'dispatch: loop {
        match pc {
            0x82C67078 => {
    //   block [0x82C67078..0x82C670AC)
	// 82C67078: 89650000  lbz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6707C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82C67080: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C67084: 419A0020  beq cr6, 0x82c670a4
	if ctx.cr[6].eq {
	pc = 0x82C670A4; continue 'dispatch;
	}
	// 82C67088: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82C6708C: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 82C67090: 419A001C  beq cr6, 0x82c670ac
	if ctx.cr[6].eq {
		sub_82C670AC(ctx, base);
		return;
	}
	// 82C67094: 89650000  lbz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67098: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82C6709C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C670A0: 409AFFE8  bne cr6, 0x82c67088
	if !ctx.cr[6].eq {
	pc = 0x82C67088; continue 'dispatch;
	}
	// 82C670A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C670A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C670AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C670AC size=8
    let mut pc: u32 = 0x82C670AC;
    'dispatch: loop {
        match pc {
            0x82C670AC => {
    //   block [0x82C670AC..0x82C670B4)
	// 82C670AC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C670B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C670B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C670B8 size=12
    let mut pc: u32 = 0x82C670B8;
    'dispatch: loop {
        match pc {
            0x82C670B8 => {
    //   block [0x82C670B8..0x82C670C4)
	// 82C670B8: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82C670BC: 80ABA8A0  lwz r5, -0x5760(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22368 as u32) ) } as u64;
	// 82C670C0: 4BFFFFB8  b 0x82c67078
	sub_82C67078(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C670C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C670C8 size=172
    let mut pc: u32 = 0x82C670C8;
    'dispatch: loop {
        match pc {
            0x82C670C8 => {
    //   block [0x82C670C8..0x82C67174)
	// 82C670C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C670CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C670D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C670D4: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 82C670D8: 4BFFFEF1  bl 0x82c66fc8
	ctx.lr = 0x82C670DC;
	sub_82C66FC8(ctx, base);
	// 82C670DC: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C670E0: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C670E4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C670E8: 419A0074  beq cr6, 0x82c6715c
	if ctx.cr[6].eq {
	pc = 0x82C6715C; continue 'dispatch;
	}
	// 82C670EC: 89650000  lbz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C670F0: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82C670F4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82C670F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C670FC: 419A002C  beq cr6, 0x82c67128
	if ctx.cr[6].eq {
	pc = 0x82C67128; continue 'dispatch;
	}
	// 82C67100: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67104: 88E90000  lbz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67108: 7CE90774  extsb r9, r7
	ctx.r[9].s64 = ctx.r[7].s8 as i64;
	// 82C6710C: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C67110: 419A0018  beq cr6, 0x82c67128
	if ctx.cr[6].eq {
	pc = 0x82C67128; continue 'dispatch;
	}
	// 82C67114: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C67118: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6711C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82C67120: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C67124: 409AFFE8  bne cr6, 0x82c6710c
	if !ctx.cr[6].eq {
	pc = 0x82C6710C; continue 'dispatch;
	}
	// 82C67128: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6712C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C67130: 419A002C  beq cr6, 0x82c6715c
	if ctx.cr[6].eq {
	pc = 0x82C6715C; continue 'dispatch;
	}
	// 82C67134: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67138: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82C6713C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67140: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C67144: 7D240774  extsb r4, r9
	ctx.r[4].s64 = ctx.r[9].s8 as i64;
	// 82C67148: 4BFFFE91  bl 0x82c66fd8
	ctx.lr = 0x82C6714C;
	sub_82C66FD8(ctx, base);
	// 82C6714C: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67150: 88C70000  lbz r6, 0(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67154: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C67158: 409AFF94  bne cr6, 0x82c670ec
	if !ctx.cr[6].eq {
	pc = 0x82C670EC; continue 'dispatch;
	}
	// 82C6715C: 4BFFFEAD  bl 0x82c67008
	ctx.lr = 0x82C67160;
	sub_82C67008(ctx, base);
	// 82C67160: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C67164: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C67168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C6716C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C67170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C67178 size=132
    let mut pc: u32 = 0x82C67178;
    'dispatch: loop {
        match pc {
            0x82C67178 => {
    //   block [0x82C67178..0x82C671FC)
	// 82C67178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6717C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C67180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C67184: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82C67188: 4BFFFE41  bl 0x82c66fc8
	ctx.lr = 0x82C6718C;
	sub_82C66FC8(ctx, base);
	// 82C6718C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67190: 89690000  lbz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67194: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 82C67198: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82C6719C: 419A0048  beq cr6, 0x82c671e4
	if ctx.cr[6].eq {
	pc = 0x82C671E4; continue 'dispatch;
	}
	// 82C671A0: 89650000  lbz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C671A4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82C671A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C671AC: 419A0020  beq cr6, 0x82c671cc
	if ctx.cr[6].eq {
	pc = 0x82C671CC; continue 'dispatch;
	}
	// 82C671B0: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C671B4: 419A0018  beq cr6, 0x82c671cc
	if ctx.cr[6].eq {
	pc = 0x82C671CC; continue 'dispatch;
	}
	// 82C671B8: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 82C671BC: 89650000  lbz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C671C0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82C671C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C671C8: 409AFFE8  bne cr6, 0x82c671b0
	if !ctx.cr[6].eq {
	pc = 0x82C671B0; continue 'dispatch;
	}
	// 82C671CC: 89650000  lbz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C671D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C671D4: 419A0010  beq cr6, 0x82c671e4
	if ctx.cr[6].eq {
	pc = 0x82C671E4; continue 'dispatch;
	}
	// 82C671D8: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 82C671DC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C671E0: 4BFFFDF9  bl 0x82c66fd8
	ctx.lr = 0x82C671E4;
	sub_82C66FD8(ctx, base);
	// 82C671E4: 4BFFFE25  bl 0x82c67008
	ctx.lr = 0x82C671E8;
	sub_82C67008(ctx, base);
	// 82C671E8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C671EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C671F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C671F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C671F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C67200 size=172
    let mut pc: u32 = 0x82C67200;
    'dispatch: loop {
        match pc {
            0x82C67200 => {
    //   block [0x82C67200..0x82C672AC)
	// 82C67200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C67204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C67208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C6720C: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 82C67210: 4BFFFDB9  bl 0x82c66fc8
	ctx.lr = 0x82C67214;
	sub_82C66FC8(ctx, base);
	// 82C67214: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67218: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6721C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C67220: 419A0074  beq cr6, 0x82c67294
	if ctx.cr[6].eq {
	pc = 0x82C67294; continue 'dispatch;
	}
	// 82C67224: 89650000  lbz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67228: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82C6722C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82C67230: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C67234: 419A002C  beq cr6, 0x82c67260
	if ctx.cr[6].eq {
	pc = 0x82C67260; continue 'dispatch;
	}
	// 82C67238: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6723C: 88E90000  lbz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67240: 7CE90774  extsb r9, r7
	ctx.r[9].s64 = ctx.r[7].s8 as i64;
	// 82C67244: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82C67248: 419A0018  beq cr6, 0x82c67260
	if ctx.cr[6].eq {
	pc = 0x82C67260; continue 'dispatch;
	}
	// 82C6724C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C67250: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67254: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82C67258: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C6725C: 409AFFE8  bne cr6, 0x82c67244
	if !ctx.cr[6].eq {
	pc = 0x82C67244; continue 'dispatch;
	}
	// 82C67260: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67264: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C67268: 409A002C  bne cr6, 0x82c67294
	if !ctx.cr[6].eq {
	pc = 0x82C67294; continue 'dispatch;
	}
	// 82C6726C: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67270: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82C67274: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67278: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C6727C: 7D240774  extsb r4, r9
	ctx.r[4].s64 = ctx.r[9].s8 as i64;
	// 82C67280: 4BFFFD59  bl 0x82c66fd8
	ctx.lr = 0x82C67284;
	sub_82C66FD8(ctx, base);
	// 82C67284: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67288: 88C70000  lbz r6, 0(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6728C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C67290: 409AFF94  bne cr6, 0x82c67224
	if !ctx.cr[6].eq {
	pc = 0x82C67224; continue 'dispatch;
	}
	// 82C67294: 4BFFFD75  bl 0x82c67008
	ctx.lr = 0x82C67298;
	sub_82C67008(ctx, base);
	// 82C67298: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C6729C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C672A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C672A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C672A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C672B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C672B0 size=12
    let mut pc: u32 = 0x82C672B0;
    'dispatch: loop {
        match pc {
            0x82C672B0 => {
    //   block [0x82C672B0..0x82C672BC)
	// 82C672B0: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82C672B4: 80ABA8A0  lwz r5, -0x5760(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22368 as u32) ) } as u64;
	// 82C672B8: 4BFFFE10  b 0x82c670c8
	sub_82C670C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C672C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C672C0 size=292
    let mut pc: u32 = 0x82C672C0;
    'dispatch: loop {
        match pc {
            0x82C672C0 => {
    //   block [0x82C672C0..0x82C673E4)
	// 82C672C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C672C4: 4804213D  bl 0x82ca9400
	ctx.lr = 0x82C672C8;
	sub_82CA93D0(ctx, base);
	// 82C672C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C672CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C672D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C672D4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82C672D8: 4BFFFFD9  bl 0x82c672b0
	ctx.lr = 0x82C672DC;
	sub_82C672B0(ctx, base);
	// 82C672DC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82C672E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C672E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C672E8: 38AB9350  addi r5, r11, -0x6cb0
	ctx.r[5].s64 = ctx.r[11].s64 + -27824;
	// 82C672EC: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C672F0: 4BFFFDD9  bl 0x82c670c8
	ctx.lr = 0x82C672F4;
	sub_82C670C8(ctx, base);
	// 82C672F4: 3FA0832F  lis r29, -0x7cd1
	ctx.r[29].s64 = -2094071808;
	// 82C672F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C672FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C67300: 80BDA8A4  lwz r5, -0x575c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-22364 as u32) ) } as u64;
	// 82C67304: 4BFFFDC5  bl 0x82c670c8
	ctx.lr = 0x82C67308;
	sub_82C670C8(ctx, base);
	// 82C67308: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82C6730C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 82C67310: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C67314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C67318: 38A9230C  addi r5, r9, 0x230c
	ctx.r[5].s64 = ctx.r[9].s64 + 8972;
	// 82C6731C: 8B4A0000  lbz r26, 0(r10)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67320: 4BFFFDA9  bl 0x82c670c8
	ctx.lr = 0x82C67324;
	sub_82C670C8(ctx, base);
	// 82C67324: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C67328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C6732C: 80BDA8A4  lwz r5, -0x575c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-22364 as u32) ) } as u64;
	// 82C67330: 4BFFFD99  bl 0x82c670c8
	ctx.lr = 0x82C67334;
	sub_82C670C8(ctx, base);
	// 82C67334: 88E30000  lbz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67338: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82C6733C: 7CEB0774  extsb r11, r7
	ctx.r[11].s64 = ctx.r[7].s8 as i64;
	// 82C67340: 409A000C  bne cr6, 0x82c6734c
	if !ctx.cr[6].eq {
	pc = 0x82C6734C; continue 'dispatch;
	}
	// 82C67344: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C67348: 419A0054  beq cr6, 0x82c6739c
	if ctx.cr[6].eq {
	pc = 0x82C6739C; continue 'dispatch;
	}
	// 82C6734C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C67350: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C67354: 38ABDE30  addi r5, r11, -0x21d0
	ctx.r[5].s64 = ctx.r[11].s64 + -8656;
	// 82C67358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C6735C: 4BFFFE1D  bl 0x82c67178
	ctx.lr = 0x82C67360;
	sub_82C67178(ctx, base);
	// 82C67360: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67364: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C67368: 419A0040  beq cr6, 0x82c673a8
	if ctx.cr[6].eq {
	pc = 0x82C673A8; continue 'dispatch;
	}
	// 82C6736C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 82C67370: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C67374: 38AB3F58  addi r5, r11, 0x3f58
	ctx.r[5].s64 = ctx.r[11].s64 + 16216;
	// 82C67378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C6737C: 4BFFFDFD  bl 0x82c67178
	ctx.lr = 0x82C67380;
	sub_82C67178(ctx, base);
	// 82C67380: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C67384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C67388: 80BDA8A4  lwz r5, -0x575c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-22364 as u32) ) } as u64;
	// 82C6738C: 4BFFFD3D  bl 0x82c670c8
	ctx.lr = 0x82C67390;
	sub_82C670C8(ctx, base);
	// 82C67390: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67394: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C67398: 409A0010  bne cr6, 0x82c673a8
	if !ctx.cr[6].eq {
	pc = 0x82C673A8; continue 'dispatch;
	}
	// 82C6739C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C673A0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C673A4: 480420AC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82C673A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C673AC: 4BFFFC1D  bl 0x82c66fc8
	ctx.lr = 0x82C673B0;
	sub_82C66FC8(ctx, base);
	// 82C673B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C673B4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C673B8: 7CBC5850  subf r5, r28, r11
	ctx.r[5].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82C673BC: 4BFFFC5D  bl 0x82c67018
	ctx.lr = 0x82C673C0;
	sub_82C67018(ctx, base);
	// 82C673C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C673C4: 4BFFFC45  bl 0x82c67008
	ctx.lr = 0x82C673C8;
	sub_82C67008(ctx, base);
	// 82C673C8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82C673CC: 419A000C  beq cr6, 0x82c673d8
	if ctx.cr[6].eq {
	pc = 0x82C673D8; continue 'dispatch;
	}
	// 82C673D0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C673D4: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C673D8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C673DC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C673E0: 48042070  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C673E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C673E8 size=128
    let mut pc: u32 = 0x82C673E8;
    'dispatch: loop {
        match pc {
            0x82C673E8 => {
    //   block [0x82C673E8..0x82C67468)
	// 82C673E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C673EC: 48042021  bl 0x82ca940c
	ctx.lr = 0x82C673F0;
	sub_82CA93D0(ctx, base);
	// 82C673F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C673F4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C673F8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C673FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C67400: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C67404: 4BFFFEBD  bl 0x82c672c0
	ctx.lr = 0x82C67408;
	sub_82C672C0(ctx, base);
	// 82C67408: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C6740C: 409A0010  bne cr6, 0x82c6741c
	if !ctx.cr[6].eq {
	pc = 0x82C6741C; continue 'dispatch;
	}
	// 82C67410: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C67414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C67418: 48042044  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C6741C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C67420: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82C67424: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C67428: 419A0028  beq cr6, 0x82c67450
	if ctx.cr[6].eq {
	pc = 0x82C67450; continue 'dispatch;
	}
	// 82C6742C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67430: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67434: 7D440774  extsb r4, r10
	ctx.r[4].s64 = ctx.r[10].s8 as i64;
	// 82C67438: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82C6743C: 419A0014  beq cr6, 0x82c67450
	if ctx.cr[6].eq {
	pc = 0x82C67450; continue 'dispatch;
	}
	// 82C67440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C67444: 4BFFFC75  bl 0x82c670b8
	ctx.lr = 0x82C67448;
	sub_82C670B8(ctx, base);
	// 82C67448: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C6744C: 419AFFC4  beq cr6, 0x82c67410
	if ctx.cr[6].eq {
	pc = 0x82C67410; continue 'dispatch;
	}
	// 82C67450: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C67454: 4804475D  bl 0x82cabbb0
	ctx.lr = 0x82C67458;
	sub_82CABBB0(ctx, base);
	// 82C67458: D83D0000  stfd f1, 0(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.f[1].u64 ) };
	// 82C6745C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C67460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C67464: 48041FF8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C67468 size=96
    let mut pc: u32 = 0x82C67468;
    'dispatch: loop {
        match pc {
            0x82C67468 => {
    //   block [0x82C67468..0x82C674C8)
	// 82C67468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C6746C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C67470: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C67474: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C67478: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82C6747C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C67480: 4BFFFF69  bl 0x82c673e8
	ctx.lr = 0x82C67484;
	sub_82C673E8(ctx, base);
	// 82C67484: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C67488: 419A0028  beq cr6, 0x82c674b0
	if ctx.cr[6].eq {
	pc = 0x82C674B0; continue 'dispatch;
	}
	// 82C6748C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C67490: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C67494: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82C67498: D1BF0000  stfs f13, 0(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C6749C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C674A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C674A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C674A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C674AC: 4E800020  blr
	return;
	// 82C674B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C674B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C674B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C674BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C674C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C674C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C674C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C674C8 size=68
    let mut pc: u32 = 0x82C674C8;
    'dispatch: loop {
        match pc {
            0x82C674C8 => {
    //   block [0x82C674C8..0x82C6750C)
	// 82C674C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C674CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C674D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C674D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C674D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C674DC: 38640001  addi r3, r4, 1
	ctx.r[3].s64 = ctx.r[4].s64 + 1;
	// 82C674E0: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C674E4: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82C674E8: 4B682CD1  bl 0x822ea1b8
	ctx.lr = 0x82C674EC;
	sub_822EA1B8(ctx, base);
	// 82C674EC: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82C674F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C674F4: 4BFFFAD5  bl 0x82c66fc8
	ctx.lr = 0x82C674F8;
	sub_82C66FC8(ctx, base);
	// 82C674F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C674FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C67500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C67504: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C67508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67510 size=52
    let mut pc: u32 = 0x82C67510;
    'dispatch: loop {
        match pc {
            0x82C67510 => {
    //   block [0x82C67510..0x82C67544)
	// 82C67510: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 82C67514: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82C67518: 7D601828  lwarx r11, 0, r3
	// lwarx
	let ea = ctx.r[3].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82C6751C: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82C67520: 409A0024  bne cr6, 0x82c67544
	if !ctx.cr[6].eq {
		sub_82C67544(ctx, base);
		return;
	}
	// 82C67524: 7CA0192D  stwcx. r5, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82C67528: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82C6752C: 4082FFE4  bne 0x82c67510
	if !ctx.cr[0].eq {
	pc = 0x82C67510; continue 'dispatch;
	}
	// 82C67530: 7D6B5B78  mr r11, r11
	ctx.r[11].u64 = ctx.r[11].u64;
	// 82C67534: 7D245850  subf r9, r4, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82C67538: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82C6753C: 5503DFFE  rlwinm r3, r8, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82C67540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67544(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67544 size=24
    let mut pc: u32 = 0x82C67544;
    'dispatch: loop {
        match pc {
            0x82C67544 => {
    //   block [0x82C67544..0x82C6755C)
	// 82C67544: 7D60192D  stwcx. r11, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82C67548: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82C6754C: 7D245850  subf r9, r4, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82C67550: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82C67554: 5503DFFE  rlwinm r3, r8, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82C67558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67560 size=28
    let mut pc: u32 = 0x82C67560;
    'dispatch: loop {
        match pc {
            0x82C67560 => {
    //   block [0x82C67560..0x82C6757C)
	// 82C67560: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 82C67564: 814B7734  lwz r10, 0x7734(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30516 as u32) ) } as u64;
	// 82C67568: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C6756C: 419A0010  beq cr6, 0x82c6757c
	if ctx.cr[6].eq {
		sub_82C6757C(ctx, base);
		return;
	}
	// 82C67570: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C67574: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C67578: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C6757C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C6757C size=8
    let mut pc: u32 = 0x82C6757C;
    'dispatch: loop {
        match pc {
            0x82C6757C => {
    //   block [0x82C6757C..0x82C67584)
	// 82C6757C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C67580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67588 size=16
    let mut pc: u32 = 0x82C67588;
    'dispatch: loop {
        match pc {
            0x82C67588 => {
    //   block [0x82C67588..0x82C67598)
	// 82C67588: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 82C6758C: 814B7738  lwz r10, 0x7738(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30520 as u32) ) } as u64;
	// 82C67590: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C67594: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67598 size=12
    let mut pc: u32 = 0x82C67598;
    'dispatch: loop {
        match pc {
            0x82C67598 => {
    //   block [0x82C67598..0x82C675A4)
	// 82C67598: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C6759C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C675A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C675A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C675A4 size=4
    let mut pc: u32 = 0x82C675A4;
    'dispatch: loop {
        match pc {
            0x82C675A4 => {
    //   block [0x82C675A4..0x82C675A8)
	// 82C675A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C675A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C675A8 size=16
    let mut pc: u32 = 0x82C675A8;
    'dispatch: loop {
        match pc {
            0x82C675A8 => {
    //   block [0x82C675A8..0x82C675B8)
	// 82C675A8: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 82C675AC: 814B772C  lwz r10, 0x772c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30508 as u32) ) } as u64;
	// 82C675B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C675B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C675B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C675B8 size=12
    let mut pc: u32 = 0x82C675B8;
    'dispatch: loop {
        match pc {
            0x82C675B8 => {
    //   block [0x82C675B8..0x82C675C4)
	// 82C675B8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C675BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C675C0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C675C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C675C4 size=4
    let mut pc: u32 = 0x82C675C4;
    'dispatch: loop {
        match pc {
            0x82C675C4 => {
    //   block [0x82C675C4..0x82C675C8)
	// 82C675C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C675C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C675C8 size=16
    let mut pc: u32 = 0x82C675C8;
    'dispatch: loop {
        match pc {
            0x82C675C8 => {
    //   block [0x82C675C8..0x82C675D8)
	// 82C675C8: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 82C675CC: 814B7730  lwz r10, 0x7730(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30512 as u32) ) } as u64;
	// 82C675D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C675D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C675D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C675D8 size=12
    let mut pc: u32 = 0x82C675D8;
    'dispatch: loop {
        match pc {
            0x82C675D8 => {
    //   block [0x82C675D8..0x82C675E4)
	// 82C675D8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C675DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C675E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C675E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C675E4 size=4
    let mut pc: u32 = 0x82C675E4;
    'dispatch: loop {
        match pc {
            0x82C675E4 => {
    //   block [0x82C675E4..0x82C675E8)
	// 82C675E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C675E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C675E8 size=104
    let mut pc: u32 = 0x82C675E8;
    'dispatch: loop {
        match pc {
            0x82C675E8 => {
    //   block [0x82C675E8..0x82C67650)
	// 82C675E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C675EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C675F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C675F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C675F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C675FC: 7D4429D6  mullw r10, r4, r5
	ctx.r[10].s64 = (ctx.r[4].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82C67600: 909F0010  stw r4, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82C67604: 90BF000C  stw r5, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82C67608: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82C6760C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C67610: 5543103A  slwi r3, r10, 2
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C67614: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C67618: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C6761C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C67620: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C67624: 419A0010  beq cr6, 0x82c67634
	if ctx.cr[6].eq {
	pc = 0x82C67634; continue 'dispatch;
	}
	// 82C67628: 4BFF8941  bl 0x82c5ff68
	ctx.lr = 0x82C6762C;
	sub_82C5FF68(ctx, base);
	// 82C6762C: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82C67630: 48000008  b 0x82c67638
	pc = 0x82C67638; continue 'dispatch;
	// 82C67634: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82C67638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C6763C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C67640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C67644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C67648: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C6764C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67650 size=12
    let mut pc: u32 = 0x82C67650;
    'dispatch: loop {
        match pc {
            0x82C67650 => {
    //   block [0x82C67650..0x82C6765C)
	// 82C67650: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C67654: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C67658: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C6765C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C6765C size=8
    let mut pc: u32 = 0x82C6765C;
    'dispatch: loop {
        match pc {
            0x82C6765C => {
    //   block [0x82C6765C..0x82C67664)
	// 82C6765C: 4BFF89FC  b 0x82c60058
	sub_82C60058(ctx, base);
	return;
	// 82C67660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67668 size=36
    let mut pc: u32 = 0x82C67668;
    'dispatch: loop {
        match pc {
            0x82C67668 => {
    //   block [0x82C67668..0x82C6768C)
	// 82C67668: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C6766C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82C67670: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C67674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82C67678: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C6767C: 41980044  blt cr6, 0x82c676c0
	if ctx.cr[6].lt {
		sub_82C6768C(ctx, base);
		return;
	}
	// 82C67680: 81030014  lwz r8, 0x14(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C67684: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82C67688: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C6768C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C6768C size=152
    let mut pc: u32 = 0x82C6768C;
    'dispatch: loop {
        match pc {
            0x82C6768C => {
    //   block [0x82C6768C..0x82C67724)
	// 82C6768C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C67690: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C67694: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C67698: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C6769C: 41980008  blt cr6, 0x82c676a4
	if ctx.cr[6].lt {
	pc = 0x82C676A4; continue 'dispatch;
	}
	// 82C676A0: 90E30004  stw r7, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82C676A4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C676A8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C676AC: 5566003E  slwi r6, r11, 0
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82C676B0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C676B4: 80A30010  lwz r5, 0x10(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C676B8: 7F062840  cmplw cr6, r6, r5
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82C676BC: 4098FFC8  bge cr6, 0x82c67684
	if !ctx.cr[6].lt {
		sub_82C67668(ctx, base);
		return;
	}
	// 82C676C0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C676C4: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C676C8: 81030018  lwz r8, 0x18(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C676CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C676D0: 7CCB49D6  mullw r6, r11, r9
	ctx.r[6].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82C676D4: 54C9103A  slwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C676D8: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82C676DC: 4099001C  ble cr6, 0x82c676f8
	if !ctx.cr[6].gt {
	pc = 0x82C676F8; continue 'dispatch;
	}
	// 82C676E0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C676E4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C676E8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82C676EC: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C676F0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82C676F4: 4181FFEC  bgt 0x82c676e0
	if ctx.cr[0].gt {
	pc = 0x82C676E0; continue 'dispatch;
	}
	// 82C676F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C676FC: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C67700: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C67704: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C67708: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C6770C: 41980008  blt cr6, 0x82c67714
	if ctx.cr[6].lt {
	pc = 0x82C67714; continue 'dispatch;
	}
	// 82C67710: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82C67714: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C67718: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C6771C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C67720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67728 size=128
    let mut pc: u32 = 0x82C67728;
    'dispatch: loop {
        match pc {
            0x82C67728 => {
    //   block [0x82C67728..0x82C677A8)
	// 82C67728: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C6772C: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 82C67730: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C67734: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C67738: 40990070  ble cr6, 0x82c677a8
	if !ctx.cr[6].gt {
		sub_82C677A8(ctx, base);
		return;
	}
	// 82C6773C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C67740: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C67744: 80EB0018  lwz r7, 0x18(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C67748: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C6774C: 7CC951D6  mullw r6, r9, r10
	ctx.r[6].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82C67750: 54C9103A  slwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C67754: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 82C67758: 4099001C  ble cr6, 0x82c67774
	if !ctx.cr[6].gt {
	pc = 0x82C67774; continue 'dispatch;
	}
	// 82C6775C: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67760: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C67764: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82C67768: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82C6776C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82C67770: 4181FFEC  bgt 0x82c6775c
	if ctx.cr[0].gt {
	pc = 0x82C6775C; continue 'dispatch;
	}
	// 82C67774: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C67778: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C6777C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C67780: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C67784: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C67788: 4198000C  blt cr6, 0x82c67794
	if ctx.cr[6].lt {
	pc = 0x82C67794; continue 'dispatch;
	}
	// 82C6778C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C67790: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C67794: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C67798: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C6779C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C677A0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C677A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C677A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C677A8 size=8
    let mut pc: u32 = 0x82C677A8;
    'dispatch: loop {
        match pc {
            0x82C677A8 => {
    //   block [0x82C677A8..0x82C677B0)
	// 82C677A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C677AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C677B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C677B0 size=20
    let mut pc: u32 = 0x82C677B0;
    'dispatch: loop {
        match pc {
            0x82C677B0 => {
    //   block [0x82C677B0..0x82C677C4)
	// 82C677B0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C677B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C677B8: 409A00B0  bne cr6, 0x82c67868
	if !ctx.cr[6].eq {
		sub_82C67868(ctx, base);
		return;
	}
	// 82C677BC: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82C677C0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C677C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C677C4 size=112
    let mut pc: u32 = 0x82C677C4;
    'dispatch: loop {
        match pc {
            0x82C677C4 => {
    //   block [0x82C677C4..0x82C67834)
	// 82C677C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C677C8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C677CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C677D0: C96BDE38  lfd f11, -0x21c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-8648 as u32) ) };
	// 82C677D4: C18A0C18  lfs f12, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C677D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C677DC: FDA06090  fmr f13, f12
	ctx.f[13].f64 = ctx.f[12].f64;
	// 82C677E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C677E4: 40990038  ble cr6, 0x82c6781c
	if !ctx.cr[6].gt {
	pc = 0x82C6781C; continue 'dispatch;
	}
	// 82C677E8: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C677EC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82C677F0: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C677F4: 7C09442E  lfsx f0, r9, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C677F8: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82C677FC: 40980008  bge cr6, 0x82c67804
	if !ctx.cr[6].lt {
	pc = 0x82C67804; continue 'dispatch;
	}
	// 82C67800: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82C67804: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C67808: 40990008  ble cr6, 0x82c67810
	if !ctx.cr[6].gt {
	pc = 0x82C67810; continue 'dispatch;
	}
	// 82C6780C: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 82C67810: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C67814: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C67818: 4082FFD8  bne 0x82c677f0
	if !ctx.cr[0].eq {
	pc = 0x82C677F0; continue 'dispatch;
	}
	// 82C6781C: C003001C  lfs f0, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C67820: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C67824: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C67828: 4099000C  ble cr6, 0x82c67834
	if !ctx.cr[6].gt {
		sub_82C67834(ctx, base);
		return;
	}
	// 82C6782C: C1430010  lfs f10, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82C67830: 48000008  b 0x82c67838
	sub_82C67834(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67834(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C67834 size=52
    let mut pc: u32 = 0x82C67834;
    'dispatch: loop {
        match pc {
            0x82C67834 => {
    //   block [0x82C67834..0x82C67868)
	// 82C67834: C1430018  lfs f10, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82C67838: ED2D02BA  fmadds f9, f13, f10, f0
	ctx.f[9].f64 = (((ctx.f[13].f64 * ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64);
	// 82C6783C: D123001C  stfs f9, 0x1c(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82C67840: FC004890  fmr f0, f9
	ctx.f[0].f64 = ctx.f[9].f64;
	// 82C67844: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82C67848: 40980008  bge cr6, 0x82c67850
	if !ctx.cr[6].lt {
	pc = 0x82C67850; continue 'dispatch;
	}
	// 82C6784C: D183001C  stfs f12, 0x1c(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82C67850: C003001C  lfs f0, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C67854: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82C67858: 7C092D2E  stfsx f0, r9, r5
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 82C6785C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82C67860: 4082FF78  bne 0x82c677d8
	if !ctx.cr[0].eq {
		sub_82C677C4(ctx, base);
		return;
	}
	// 82C67864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67868 size=16
    let mut pc: u32 = 0x82C67868;
    'dispatch: loop {
        match pc {
            0x82C67868 => {
    //   block [0x82C67868..0x82C67878)
	// 82C67868: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82C6786C: 409A00A4  bne cr6, 0x82c67910
	if !ctx.cr[6].eq {
		sub_82C67910(ctx, base);
		return;
	}
	// 82C67870: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82C67874: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C67878 size=100
    let mut pc: u32 = 0x82C67878;
    'dispatch: loop {
        match pc {
            0x82C67878 => {
    //   block [0x82C67878..0x82C678DC)
	// 82C67878: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C6787C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C67880: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C67884: C96BDE38  lfd f11, -0x21c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-8648 as u32) ) };
	// 82C67888: C18A0C18  lfs f12, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C6788C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67890: FC006090  fmr f0, f12
	ctx.f[0].f64 = ctx.f[12].f64;
	// 82C67894: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C67898: 4099002C  ble cr6, 0x82c678c4
	if !ctx.cr[6].gt {
	pc = 0x82C678C4; continue 'dispatch;
	}
	// 82C6789C: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C678A0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82C678A4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C678A8: 7DA9442E  lfsx f13, r9, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C678AC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C678B0: 40990008  ble cr6, 0x82c678b8
	if !ctx.cr[6].gt {
	pc = 0x82C678B8; continue 'dispatch;
	}
	// 82C678B4: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82C678B8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C678BC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C678C0: 4082FFE4  bne 0x82c678a4
	if !ctx.cr[0].eq {
	pc = 0x82C678A4; continue 'dispatch;
	}
	// 82C678C4: C1A3001C  lfs f13, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C678C8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C678CC: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82C678D0: 4099000C  ble cr6, 0x82c678dc
	if !ctx.cr[6].gt {
		sub_82C678DC(ctx, base);
		return;
	}
	// 82C678D4: C1430010  lfs f10, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82C678D8: 48000008  b 0x82c678e0
	sub_82C678DC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C678DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C678DC size=52
    let mut pc: u32 = 0x82C678DC;
    'dispatch: loop {
        match pc {
            0x82C678DC => {
    //   block [0x82C678DC..0x82C67910)
	// 82C678DC: C1430018  lfs f10, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82C678E0: ED206ABA  fmadds f9, f0, f10, f13
	ctx.f[9].f64 = (((ctx.f[0].f64 * ctx.f[10].f64 + ctx.f[13].f64) as f32) as f64);
	// 82C678E4: D123001C  stfs f9, 0x1c(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82C678E8: FC004890  fmr f0, f9
	ctx.f[0].f64 = ctx.f[9].f64;
	// 82C678EC: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82C678F0: 40980008  bge cr6, 0x82c678f8
	if !ctx.cr[6].lt {
	pc = 0x82C678F8; continue 'dispatch;
	}
	// 82C678F4: D183001C  stfs f12, 0x1c(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82C678F8: C003001C  lfs f0, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C678FC: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82C67900: 7C092D2E  stfsx f0, r9, r5
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 82C67904: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82C67908: 4082FF84  bne 0x82c6788c
	if !ctx.cr[0].eq {
		sub_82C67878(ctx, base);
		return;
	}
	// 82C6790C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67910 size=8
    let mut pc: u32 = 0x82C67910;
    'dispatch: loop {
        match pc {
            0x82C67910 => {
    //   block [0x82C67910..0x82C67918)
	// 82C67910: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82C67914: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67918 size=44
    let mut pc: u32 = 0x82C67918;
    'dispatch: loop {
        match pc {
            0x82C67918 => {
    //   block [0x82C67918..0x82C67944)
	// 82C67918: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6791C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C67920: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82C67924: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 82C67928: F921FFF0  std r9, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[9].u64 ) };
	// 82C6792C: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C67930: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82C67934: C80A0CB8  lfd f0, 0xcb8(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(3256 as u32) ) };
	// 82C67938: FD806824  fdiv f12, f0, f13
	ctx.f[12].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 82C6793C: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82C67940: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67944(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C67944 size=96
    let mut pc: u32 = 0x82C67944;
    'dispatch: loop {
        match pc {
            0x82C67944 => {
    //   block [0x82C67944..0x82C679A4)
	// 82C67944: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C67948: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C6794C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C67950: C94BDE38  lfd f10, -0x21c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-8648 as u32) ) };
	// 82C67954: C16A0C18  lfs f11, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82C67958: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C6795C: FC005890  fmr f0, f11
	ctx.f[0].f64 = ctx.f[11].f64;
	// 82C67960: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C67964: 40990024  ble cr6, 0x82c67988
	if !ctx.cr[6].gt {
	pc = 0x82C67988; continue 'dispatch;
	}
	// 82C67968: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C6796C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82C67970: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C67974: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C67978: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82C6797C: 7DA84C2E  lfsx f13, r8, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C67980: EC0D037A  fmadds f0, f13, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82C67984: 4082FFEC  bne 0x82c67970
	if !ctx.cr[0].eq {
	pc = 0x82C67970; continue 'dispatch;
	}
	// 82C67988: EDAC0032  fmuls f13, f12, f0
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C6798C: C003001C  lfs f0, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C67990: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C67994: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C67998: 4099000C  ble cr6, 0x82c679a4
	if !ctx.cr[6].gt {
		sub_82C679A4(ctx, base);
		return;
	}
	// 82C6799C: C1230010  lfs f9, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82C679A0: 48000008  b 0x82c679a8
	sub_82C679A4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C679A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C679A4 size=52
    let mut pc: u32 = 0x82C679A4;
    'dispatch: loop {
        match pc {
            0x82C679A4 => {
    //   block [0x82C679A4..0x82C679D8)
	// 82C679A4: C1230018  lfs f9, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82C679A8: ED0D027A  fmadds f8, f13, f9, f0
	ctx.f[8].f64 = (((ctx.f[13].f64 * ctx.f[9].f64 + ctx.f[0].f64) as f32) as f64);
	// 82C679AC: D103001C  stfs f8, 0x1c(r3)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82C679B0: FC004090  fmr f0, f8
	ctx.f[0].f64 = ctx.f[8].f64;
	// 82C679B4: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 82C679B8: 40980008  bge cr6, 0x82c679c0
	if !ctx.cr[6].lt {
	pc = 0x82C679C0; continue 'dispatch;
	}
	// 82C679BC: D163001C  stfs f11, 0x1c(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82C679C0: C003001C  lfs f0, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C679C4: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82C679C8: 7C092D2E  stfsx f0, r9, r5
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 82C679CC: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82C679D0: 4082FF88  bne 0x82c67958
	if !ctx.cr[0].eq {
		sub_82C67944(ctx, base);
		return;
	}
	// 82C679D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C679D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C679D8 size=52
    let mut pc: u32 = 0x82C679D8;
    'dispatch: loop {
        match pc {
            0x82C679D8 => {
    //   block [0x82C679D8..0x82C67A0C)
	// 82C679D8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82C679DC: C1630004  lfs f11, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82C679E0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C679E4: D023000C  stfs f1, 0xc(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C679E8: C00BBE14  lfs f0, -0x41ec(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16876 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C679EC: C18A0C14  lfs f12, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C679F0: EDA10032  fmuls f13, f1, f0
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C679F4: EC0C5824  fdivs f0, f12, f11
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[11].f64) as f32) as f64;
	// 82C679F8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C679FC: 40980010  bge cr6, 0x82c67a0c
	if !ctx.cr[6].lt {
		sub_82C67A0C(ctx, base);
		return;
	}
	// 82C67A00: EC000024  fdivs f0, f0, f0
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[0].f64) as f32) as f64;
	// 82C67A04: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82C67A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67A0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C67A0C size=12
    let mut pc: u32 = 0x82C67A0C;
    'dispatch: loop {
        match pc {
            0x82C67A0C => {
    //   block [0x82C67A0C..0x82C67A18)
	// 82C67A0C: EC006824  fdivs f0, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82C67A10: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82C67A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C67A18 size=52
    let mut pc: u32 = 0x82C67A18;
    'dispatch: loop {
        match pc {
            0x82C67A18 => {
    //   block [0x82C67A18..0x82C67A4C)
	// 82C67A18: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82C67A1C: C1630004  lfs f11, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82C67A20: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C67A24: D0230014  stfs f1, 0x14(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82C67A28: C00BBE14  lfs f0, -0x41ec(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16876 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C67A2C: C18A0C14  lfs f12, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C67A30: EDA10032  fmuls f13, f1, f0
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C67A34: EC0C5824  fdivs f0, f12, f11
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[11].f64) as f32) as f64;
	// 82C67A38: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C67A3C: 40980010  bge cr6, 0x82c67a4c
	if !ctx.cr[6].lt {
		sub_82C67A4C(ctx, base);
		return;
	}
	// 82C67A40: EC000024  fdivs f0, f0, f0
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[0].f64) as f32) as f64;
	// 82C67A44: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82C67A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67A4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C67A4C size=12
    let mut pc: u32 = 0x82C67A4C;
    'dispatch: loop {
        match pc {
            0x82C67A4C => {
    //   block [0x82C67A4C..0x82C67A58)
	// 82C67A4C: EC006824  fdivs f0, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82C67A50: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82C67A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67A58 size=8
    let mut pc: u32 = 0x82C67A58;
    'dispatch: loop {
        match pc {
            0x82C67A58 => {
    //   block [0x82C67A58..0x82C67A60)
	// 82C67A58: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82C67A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C67A60 size=12
    let mut pc: u32 = 0x82C67A60;
    'dispatch: loop {
        match pc {
            0x82C67A60 => {
    //   block [0x82C67A60..0x82C67A6C)
	// 82C67A60: 7C8B0034  cntlzw r11, r4
	ctx.r[11].u64 = if ctx.r[4].u32 == 0 { 32 } else { ctx.r[4].u32.leading_zeros() as u64 };
	// 82C67A64: 5564DFFE  rlwinm r4, r11, 0x1b, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82C67A68: 4BFFFFF0  b 0x82c67a58
	sub_82C67A58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C67A70 size=8
    let mut pc: u32 = 0x82C67A70;
    'dispatch: loop {
        match pc {
            0x82C67A70 => {
    //   block [0x82C67A70..0x82C67A78)
	// 82C67A70: D023001C  stfs f1, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82C67A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C67A78 size=116
    let mut pc: u32 = 0x82C67A78;
    'dispatch: loop {
        match pc {
            0x82C67A78 => {
    //   block [0x82C67A78..0x82C67AEC)
	// 82C67A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C67A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C67A80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C67A84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C67A88: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82C67A8C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C67A90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C67A94: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C67A98: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82C67A9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C67AA0: 4BFFC499  bl 0x82c63f38
	ctx.lr = 0x82C67AA4;
	sub_82C63F38(ctx, base);
	// 82C67AA4: D3FF0004  stfs f31, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C67AA8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C67AAC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C67AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C67AB4: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C67AB8: ED40F824  fdivs f10, f0, f31
	ctx.f[10].f64 = ((ctx.f[0].f64 / ctx.f[31].f64) as f32) as f64;
	// 82C67ABC: FC205090  fmr f1, f10
	ctx.f[1].f64 = ctx.f[10].f64;
	// 82C67AC0: 4BFFFF19  bl 0x82c679d8
	ctx.lr = 0x82C67AC4;
	sub_82C679D8(ctx, base);
	// 82C67AC4: 4BFFFF55  bl 0x82c67a18
	ctx.lr = 0x82C67AC8;
	sub_82C67A18(ctx, base);
	// 82C67AC8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C67ACC: 4BFFFF8D  bl 0x82c67a58
	ctx.lr = 0x82C67AD0;
	sub_82C67A58(ctx, base);
	// 82C67AD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C67AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C67AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C67ADC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82C67AE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C67AE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C67AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C67AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C67AF0 size=88
    let mut pc: u32 = 0x82C67AF0;
    'dispatch: loop {
        match pc {
            0x82C67AF0 => {
    //   block [0x82C67AF0..0x82C67B48)
	// 82C67AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C67AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C67AF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C67AFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C67B00: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82C67B04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C67B08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C67B0C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C67B10: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82C67B14: 4BFFC455  bl 0x82c63f68
	ctx.lr = 0x82C67B18;
	sub_82C63F68(ctx, base);
	// 82C67B18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C67B1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C67B20: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C67B24: 4BFFFF55  bl 0x82c67a78
	ctx.lr = 0x82C67B28;
	sub_82C67A78(ctx, base);
	// 82C67B28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C67B2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C67B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C67B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C67B38: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82C67B3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C67B40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C67B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


