pub fn sub_832B9758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9758 size=2060
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B9780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9780 size=2060
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -6436;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B97A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B97A8 size=2060
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832BAC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BAC00 size=3092
	ctx.lr = 0x832BAC04;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203A08);
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BAD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BAD00 size=6156
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832BAD1C; continue 'dispatch;
	}
	ctx.lr = 0x832BAD0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203D20);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].s64 = -2093875200;
	ctx.r[3].s64 = ctx.r[11].s64 + -22104;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203990);
	return;
}

pub fn sub_832BAFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BAFF8 size=5140
	ctx.lr = 0x832BAFFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82AA9058);
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	ctx.lr = 0x832BB004;
	crate::recompiler::externs::call(&mut ctx, base, 0x82AA9058);
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BB00C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BB130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BB130 size=7176
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BB138;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].u64 = ctx.r[30].u64;
	if !ctx.cr[0].eq {
			crate::recompiler::externs::call(&mut ctx, base, 0x832BB12C);
		return;
	}
	ctx.lr = 0x832BB14C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BB288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BB288 size=3092
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[10].u64;
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	return;
}

pub fn sub_832BB2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BB2F8 size=5132
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	if ctx.cr[0].eq {
			pc = 0x832BB318; continue 'dispatch;
	}
	ctx.lr = 0x832BB318;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BB390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BB390 size=3092
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -1444;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832BB4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BB4B8 size=12
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
}

pub fn sub_832BC598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BC598 size=2060
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + 17328;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832BC6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BC6C0 size=3084
	ctx.r[11].u64 = ctx.r[24].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[25].u32) };
	if ctx.cr[0].eq {
			pc = 0x832BC6E4; continue 'dispatch;
	}
	ctx.lr = 0x832BC6DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203D20);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[24].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[24].u32) };
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	if !ctx.cr[0].eq {
			pc = 0x832BC6F4; continue 'dispatch;
	}
	ctx.r[11].u64 = ctx.r[24].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	ctx.r[3].u64 = ctx.r[30].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[26].u32) };
	ctx.lr = 0x832BC704;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203A08);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[27].u32) };
	ctx.r[3].u64 = ctx.r[30].u64;
	ctx.lr = 0x832BC710;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203A08);
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200684208].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E48);
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + 17396;
	crate::recompiler::externs::call(&mut ctx, base, 0x822A5498);
	return;
}

pub fn sub_832BC7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BC7F0 size=3092
	ctx.r[11].s64 = -2112946176;
	ctx.r[30].s64 = ctx.r[31].s64 + -21496;
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	ctx.r[3].u64 = ctx.r[30].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832BC808;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203A08);
	ctx.r[11].s64 = -2112946176;
	ctx.r[3].u64 = ctx.r[30].u64;
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832BC81C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203A08);
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BC9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832BC9E0 size=3084
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 1;
	ctx.r[11].s64 = ctx.r[11].s64 + 19980;
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BCA00;
	crate::recompiler::externs::call(&mut ctx, base, 0x821BF518);
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200685044].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 1;
	ctx.r[11].s64 = ctx.r[11].s64 + 19996;
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BCA50;
	crate::recompiler::externs::call(&mut ctx, base, 0x821BF518);
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200685124].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 1;
	ctx.r[11].s64 = ctx.r[11].s64 + 20012;
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BCAA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821BF518);
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200685204].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 1;
	ctx.r[11].s64 = ctx.r[11].s64 + 20028;
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BCAF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821BF518);
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200685284].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 1;
	ctx.r[11].s64 = ctx.r[11].s64 + 20044;
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BCB40;
	crate::recompiler::externs::call(&mut ctx, base, 0x821BF518);
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200685364].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 1;
	ctx.r[11].s64 = ctx.r[11].s64 + 20060;
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BCB90;
	crate::recompiler::externs::call(&mut ctx, base, 0x821BF518);
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200685444].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].s64 = -2093875200;
	ctx.r[11].s64 = -2112946176;
	ctx.r[30].s64 = ctx.r[31].s64 + -21480;
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	ctx.r[3].u64 = ctx.r[30].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832BCBE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203A08);
	ctx.r[11].s64 = -2112946176;
	ctx.r[3].u64 = ctx.r[30].u64;
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832BCBF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203A08);
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BCB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832BCB30 size=5132
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BCB40;
	crate::recompiler::externs::call(&mut ctx, base, 0x821BF518);
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200685364].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 1;
	ctx.r[11].s64 = ctx.r[11].s64 + 20060;
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BCB90;
	crate::recompiler::externs::call(&mut ctx, base, 0x821BF518);
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200685444].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].s64 = -2093875200;
	ctx.r[11].s64 = -2112946176;
	ctx.r[30].s64 = ctx.r[31].s64 + -21480;
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	ctx.r[3].u64 = ctx.r[30].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832BCBE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203A08);
	ctx.r[11].s64 = -2112946176;
	ctx.r[3].u64 = ctx.r[30].u64;
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832BCBF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203A08);
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BCDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832BCDE8 size=4108
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].s64 = -2092302336;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832BCE58; continue 'dispatch;
	}
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200686108].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832BCE50; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[10].u64;
	ctx.lr = 0x832BCE50;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BD018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BD018 size=7180
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BD218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BD218 size=2060
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BD220;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A9EAE0);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BD228;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BD440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BD440 size=4108
	crate::recompiler::externs::call(&mut ctx, base, 0x82AA11E8);
	return;
}

pub fn sub_832BD710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BD710 size=4108
	if ctx.cr[0].eq {
			pc = 0x832BD724; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BD71C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B7FDA0);
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BD724;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BD8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BD8E0 size=7180
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832BD8F0; continue 'dispatch;
	}
	ctx.lr = 0x832BD8F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BDAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BDAD0 size=4108
	crate::recompiler::externs::call(&mut ctx, base, 0x82B1FE88);
	return;
}

pub fn sub_832BDCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BDCA0 size=6160
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + 23104;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832BDE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BDE38 size=1036
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832BDEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BDEA0 size=12
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
}

pub fn sub_832BE098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BE098 size=7180
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq { return; }
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[10].u64;
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	return;
}

pub fn sub_832BE238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832BE238 size=7180
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].s64 = -2093875200;
	ctx.r[11].s64 = -2112880640;
	ctx.r[30].s64 = ctx.r[31].s64 + -20276;
	ctx.r[11].s64 = ctx.r[11].s64 + -21020;
	ctx.r[3].u64 = ctx.r[30].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832BE260;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203A08);
	ctx.r[11].s64 = -2112946176;
	ctx.r[3].u64 = ctx.r[30].u64;
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832BE274;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203A08);
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BE2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BE2E8 size=16
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + 24508;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B75910);
	return;
}

pub fn sub_832BE498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832BE498 size=7180
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].s64 = -2093875200;
	ctx.r[11].s64 = -2112946176;
	ctx.r[30].s64 = ctx.r[31].s64 + -20168;
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	ctx.r[3].u64 = ctx.r[30].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832BE4D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203A08);
	ctx.r[11].s64 = -2112946176;
	ctx.r[3].u64 = ctx.r[30].u64;
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832BE4E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82203A08);
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BE558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832BE558 size=20
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093875200;
}

pub fn sub_832BE5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BE5F8 size=7184
	ctx.r[11].s64 = -2092171264;
	ctx.r[3].s64 = ctx.r[11].s64 + -8548;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B8C658);
	return;
}

pub fn sub_832BE698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BE698 size=4108
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832BE980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BE980 size=2060
	ctx.r[11].s64 = ctx.r[11].s64 + -20052;
	ctx.r[9].s64 = ctx.r[10].s64 + -25608;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.cr[0].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832BE9A8; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	if !ctx.cr[0].eq { return; }
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B62308);
	return;
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B5F6E0);
	return;
	return;
}

pub fn sub_832BEA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BEA28 size=12
	ctx.r[11].s64 = -2092171264;
	ctx.r[3].s64 = ctx.r[11].s64 + -7068;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832BEBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BEBA8 size=2068
	crate::recompiler::externs::call(&mut ctx, base, 0x82BFD880);
	return;
}

pub fn sub_832BEC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BEC50 size=6156
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832BED88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BED88 size=2068
	ctx.lr = 0x832BED8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x823298C0);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BED94;
	crate::recompiler::externs::call(&mut ctx, base, 0x8284D1B0);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].s64 = -2093809664;
	ctx.r[3].s64 = ctx.r[11].s64 + 19416;
	crate::recompiler::externs::call(&mut ctx, base, 0x82C11610);
	return;
}

pub fn sub_832BEDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BEDF0 size=2068
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].s64 = -2093809664;
	ctx.r[3].s64 = ctx.r[11].s64 + 19552;
	crate::recompiler::externs::call(&mut ctx, base, 0x82C11610);
	return;
}

pub fn sub_832BEE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832BEE98 size=5140
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	ctx.r[31].s64 = ctx.r[11].s64 + 19656;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[4].u64 = ctx.r[31].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[6].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[5].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.lr = 0x832BEEDC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82C142D0);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BEEE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8284D1B0);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	ctx.r[31].s64 = ctx.r[11].s64 + 19668;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[4].u64 = ctx.r[31].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[6].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[5].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.lr = 0x832BEF4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82C143B8);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BEF54;
	crate::recompiler::externs::call(&mut ctx, base, 0x8284D1B0);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	ctx.r[31].s64 = ctx.r[11].s64 + 19680;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[4].u64 = ctx.r[31].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[6].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[5].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.lr = 0x832BEFBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82C144A0);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BEFC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8284D1B0);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[30].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + 19760;
	ctx.r[31].s64 = ctx.r[11].s64 + 140;
	ctx.r[31].s64 = ctx.r[31].s64 + -28;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BF018;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B9E878);
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200694796].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[30].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + 19900;
	ctx.r[31].s64 = ctx.r[11].s64 + 180;
	ctx.r[31].s64 = ctx.r[31].s64 + -36;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BF068;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B453D8);
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200694876].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832BF090;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[29].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + 19700;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if ctx.cr[0].eq {
			pc = 0x832BF0E8; continue 'dispatch;
	}
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BF0D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8284D1B0);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = ctx.r[30].u64;
	if !ctx.cr[0].eq {
			pc = 0x832BF0D0; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BF0F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8284D1B0);
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200694952].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
}

pub fn sub_832BEF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832BEF10 size=4112
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	ctx.r[31].s64 = ctx.r[11].s64 + 19668;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[4].u64 = ctx.r[31].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[6].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[5].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.lr = 0x832BEF4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82C143B8);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BEF54;
	crate::recompiler::externs::call(&mut ctx, base, 0x8284D1B0);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	ctx.r[31].s64 = ctx.r[11].s64 + 19680;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[4].u64 = ctx.r[31].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[6].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[5].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.lr = 0x832BEFBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82C144A0);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BEFC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8284D1B0);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[30].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + 19760;
	ctx.r[31].s64 = ctx.r[11].s64 + 140;
	ctx.r[31].s64 = ctx.r[31].s64 + -28;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BF018;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B9E878);
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200694796].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[30].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + 19900;
	ctx.r[31].s64 = ctx.r[11].s64 + 180;
	ctx.r[31].s64 = ctx.r[31].s64 + -36;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BF068;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B453D8);
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200694876].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832BF090;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[29].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + 19700;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if ctx.cr[0].eq {
			pc = 0x832BF0E8; continue 'dispatch;
	}
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BF0D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8284D1B0);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = ctx.r[30].u64;
	if !ctx.cr[0].eq {
			pc = 0x832BF0D0; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BF0F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8284D1B0);
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200694952].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
}

pub fn sub_832BEF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832BEF80 size=5132
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	ctx.r[31].s64 = ctx.r[11].s64 + 19680;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[4].u64 = ctx.r[31].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[6].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[5].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.lr = 0x832BEFBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82C144A0);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BEFC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8284D1B0);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[30].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + 19760;
	ctx.r[31].s64 = ctx.r[11].s64 + 140;
	ctx.r[31].s64 = ctx.r[31].s64 + -28;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BF018;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B9E878);
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200694796].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[30].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + 19900;
	ctx.r[31].s64 = ctx.r[11].s64 + 180;
	ctx.r[31].s64 = ctx.r[31].s64 + -36;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832BF068;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B453D8);
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200694876].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832BF090;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093809664;
	ctx.r[29].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + 19700;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if ctx.cr[0].eq {
			pc = 0x832BF0E8; continue 'dispatch;
	}
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BF0D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8284D1B0);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = ctx.r[30].u64;
	if !ctx.cr[0].eq {
			pc = 0x832BF0D0; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832BF0F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8284D1B0);
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200694952].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
}

pub fn sub_832BF198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BF198 size=7180
	ctx.r[11].s64 = -2093809664;
	ctx.r[3].s64 = ctx.r[11].s64 + 20244;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832BF2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BF2B8 size=1048
	ctx.r[11].s64 = -2093809664;
	ctx.r[3].s64 = ctx.r[11].s64 + 20316;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832BF5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BF5C0 size=2080
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].s64 = -2093809664;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq { return; }
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[10].u64;
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	return;
}

pub fn sub_832BFBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BFBA8 size=7192
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	return;
}

pub fn sub_832BFEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BFEC8 size=4108
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	return;
}

pub fn sub_832BFFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BFFF8 size=3084
	ctx.r[11].s64 = -2112946176;
	ctx.r[10].s64 = -2093744128;
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	return;
}

pub fn sub_832C0108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C0108 size=2060
	ctx.r[11].s64 = ctx.r[11].s64 + 25880;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	return;
}

pub fn sub_832C01D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C01D0 size=1036
	return;
}

pub fn sub_832C05A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C05A0 size=3084
	return;
}

pub fn sub_832C0630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C0630 size=2064
	return;
}

pub fn sub_832C0698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C0698 size=2060
	ctx.r[11].s64 = -2113667072;
	ctx.r[10].s64 = -2093678592;
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	return;
}

pub fn sub_832C0900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C0900 size=16
	return;
}

pub fn sub_832C09C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C09C0 size=5132
	return;
}

pub fn sub_832C0AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C0AB8 size=12
	return;
}

pub fn sub_832C0BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C0BB0 size=1036
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832C13B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C13B8 size=6156
}

pub fn sub_832C16B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C16B0 size=5132
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
}

pub fn sub_832C1968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C1968 size=7180
}

pub fn sub_832C1C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C1C08 size=3084
}

pub fn sub_832C1E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C1E38 size=4104
}

pub fn sub_832C2028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C2028 size=4108
}

pub fn sub_832C22B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C22B8 size=7176
}

pub fn sub_832C2618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C2618 size=4108
	ctx.r[8].u64 = ctx.r[3].u64;
	ctx.r[31].s64 = ctx.r[11].s64 + -15252;
	ctx.r[7].s64 = ctx.r[4].s64 + 31;
	ctx.r[30].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	ctx.lr = 0x832C2644;
	crate::recompiler::externs::call(&mut ctx, base, 0x82BF1790);
	ctx.r[11].s64 = 0;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	if ctx.cr[0].eq {
			pc = 0x832C268C; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C268C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	if !ctx.cr[2200708720].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832C3F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C3F98 size=32
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C3FA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E0C);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.r[30].s64 = 0;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			crate::recompiler::externs::call(&mut ctx, base, 0x832C3FD0);
		return;
	}
}

pub fn sub_832C40C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C40C0 size=68
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C40C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E0C);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C40E4; continue 'dispatch;
	}
	ctx.r[3].s64 = 0;
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].eq {
			pc = 0x832C40D8; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].s64 = 1;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			crate::recompiler::externs::call(&mut ctx, base, 0x832C4108);
		return;
	}
}

pub fn sub_832C41F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C41F0 size=492
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	if !ctx.cr[2200715812].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4274; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	if !ctx.cr[0].gt {
			pc = 0x832C425C; continue 'dispatch;
	}
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	pc = 0x832C4274; continue 'dispatch;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].gt {
			pc = 0x832C4274; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[30].s64 = 0;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[4].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32), ctx.r[6].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[10].u32) };
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32), ctx.r[5].u32) };
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32), ctx.r[10].u32) };
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[10].u32), ctx.r[5].u32) };
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[5].u32), ctx.r[7].u32) };
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C4344; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = ctx.r[11].s64 * 1000;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].u64 = ctx.r[8].u64 / ctx.r[9].u64;
	ctx.r[3].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32), ctx.r[3].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[4].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	pc = 0x832C4358; continue 'dispatch;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4358; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C4358;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C2700);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	if ctx.cr[0].eq {
			pc = 0x832C4370; continue 'dispatch;
	}
	ctx.r[11].s64 = -1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	if !ctx.cr[0].eq {
			pc = 0x832C43A0; continue 'dispatch;
	}
	ctx.lr = 0x832C438C;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].s64 = -1;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832C43E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C43E0 size=80
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C43E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0DF0);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].u64 = ctx.r[6].u64;
	ctx.r[22].u64 = ctx.r[7].u64;
	ctx.r[6].s64 = ctx.r[11].s64 + 4;
	ctx.r[23].u64 = ctx.r[8].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	ctx.r[25].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			crate::recompiler::externs::call(&mut ctx, base, 0x832C4524);
		return;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[4].s64 * 388;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[26].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200716352].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
}

pub fn sub_832C4530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C4530 size=48
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C4538;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0DEC);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.lr = 0x832C4544;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[27].u64 = ctx.r[3].u64;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.r[4].u64 = ctx.r[27].u64;
	ctx.lr = 0x832C4554;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C40C0);
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			crate::recompiler::externs::call(&mut ctx, base, 0x832C4568);
		return;
	}
}

pub fn sub_832C49E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C49E8 size=2072
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = 1;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[25].s64;
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	if !ctx.cr[0].gt {
			pc = 0x832C4A10; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[26].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E38);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C4A20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E0C);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.r[29].s64 = 0;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	ctx.r[30].u64 = ctx.r[29].u64;
	if ctx.cr[0].eq {
			pc = 0x832C4C0C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C4C0C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C4C0C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C4A64; continue 'dispatch;
	}
	ctx.r[4].s64 = 0;
	ctx.lr = 0x832C4A64;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C3F98);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4A80; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	pc = 0x832C4AEC; continue 'dispatch;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].u64 = ctx.r[29].u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].gt {
			pc = 0x832C4BA0; continue 'dispatch;
	}
	ctx.r[10].u64 = ctx.r[29].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4AD0; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	if !ctx.cr[0].gt {
			pc = 0x832C4AD0; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].gt {
			pc = 0x832C4AD0; continue 'dispatch;
	}
	ctx.r[30].s64 = 1;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	ctx.r[10].s64 = ctx.r[10].s64 + 388;
	if ctx.cr[0].lt {
			pc = 0x832C4A94; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4BA0; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C4B04;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C6380);
	ctx.r[4].s64 = 0;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C4B10;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C54D0);
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = ctx.r[10].s64 * 29;
	if !ctx.cr[0].lt {
			pc = 0x832C4B5C; continue 'dispatch;
	}
	ctx.r[30].s64 = ctx.r[31].s64 + 336;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = ctx.r[30].u64;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C4B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4B5C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[11].s64 * 29;
	if ctx.cr[0].lt {
			pc = 0x832C4B2C; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	ctx.r[4].s64 = 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C4B70;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C54D0);
	ctx.xer.ca = (ctx.r[31].u32 > (!(852 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + 852;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	if ctx.cr[2200718216].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4B88; continue 'dispatch;
	}
	ctx.lr = 0x832C4B88;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.xer.ca = (ctx.r[31].u32 > (!(596 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + 596;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	if ctx.cr[2200718240].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4BA0; continue 'dispatch;
	}
	ctx.lr = 0x832C4BA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].s64 = 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4BBC; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C4BBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C2700);
	ctx.lr = 0x832C4BC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4BE8; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4BE8; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].s64 = 1;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C4C00; continue 'dispatch;
	}
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C4C08;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C29F8);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
}

pub fn sub_832C4DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C4DB0 size=7180
	if ctx.cr[0].eq {
			pc = 0x832C4DF4; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4DD4; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].eq {
			pc = 0x832C4DD4; continue 'dispatch;
	}
	ctx.r[29].u64 = ctx.r[24].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[25].u32) };
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C4DDC;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C4530);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[25].u32) };
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C4DE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C4A18);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].eq {
			pc = 0x832C4DB8; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[24].u32) };
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[24].u32) };
	if ctx.cr[0].eq {
			pc = 0x832C4E08; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[25].u32) };
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4E20; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4E20; continue 'dispatch;
	}
	ctx.lr = 0x832C4E20;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4E34; continue 'dispatch;
	}
	ctx.r[4].s64 = 1;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C4E34;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C54D0);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[24].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E48);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C4E48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E0C);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5108; continue 'dispatch;
	}
	ctx.r[4].s64 = 1;
	ctx.lr = 0x832C4E60;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C52E0);
	ctx.r[11].s64 = -2091909120;
	ctx.r[30].s64 = ctx.r[11].s64 + -15192;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C4E8C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C4E8C; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	pc = 0x832C4E90; continue 'dispatch;
	ctx.r[3].s64 = 0;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4EA4; continue 'dispatch;
	}
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	pc = 0x832C4EA8; continue 'dispatch;
	ctx.r[5].s64 = 0;
	ctx.r[7].s64 = 1;
	ctx.r[6].s64 = ctx.r[31].s64 + 852;
	ctx.r[4].s64 = ctx.r[31].s64 + 596;
	ctx.lr = 0x832C4EB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C6980);
	ctx.cr[0].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4ED8; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4F40; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C4EE0; continue 'dispatch;
	}
	ctx.r[10].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].s64 = 0;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].gt {
			pc = 0x832C4FA0; continue 'dispatch;
	}
	ctx.r[30].s64 = 0;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C4F08;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4F50; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C4F4C; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[9].u64;
	ctx.lr = 0x832C4F3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	pc = 0x832C4F50; continue 'dispatch;
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	pc = 0x832C4EE0; continue 'dispatch;
	ctx.lr = 0x832C4F50;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB3B40);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4F8C; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C4F88; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[9].u64;
	ctx.lr = 0x832C4F84;
	crate::rt::call_indirect(ctx.ctr.u32);
	pc = 0x832C4F8C; continue 'dispatch;
	ctx.lr = 0x832C4F8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB3B40);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	ctx.r[30].s64 = ctx.r[30].s64 + 388;
	if ctx.cr[0].lt {
			pc = 0x832C4EF4; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C4FE0; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5024; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C5020; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[9].u64;
	ctx.lr = 0x832C4FDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	pc = 0x832C5024; continue 'dispatch;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[31].s64 + 336;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C4FF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5024; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C5020; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[9].u64;
	ctx.lr = 0x832C501C;
	crate::rt::call_indirect(ctx.ctr.u32);
	pc = 0x832C5024; continue 'dispatch;
	ctx.lr = 0x832C5024;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB3B40);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5058; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C5054; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[9].u64;
	ctx.lr = 0x832C5050;
	crate::rt::call_indirect(ctx.ctr.u32);
	pc = 0x832C5058; continue 'dispatch;
	ctx.lr = 0x832C5058;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB3B40);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C508C; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C5088; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[9].u64;
	ctx.lr = 0x832C5084;
	crate::rt::call_indirect(ctx.ctr.u32);
	pc = 0x832C508C; continue 'dispatch;
	ctx.lr = 0x832C508C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB3B40);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C50CC; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].eq {
			pc = 0x832C50CC; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C50C8; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[9].u64;
	ctx.lr = 0x832C50C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	pc = 0x832C50CC; continue 'dispatch;
	ctx.lr = 0x832C50CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB3B40);
	ctx.r[5].s64 = 924;
	ctx.r[4].s64 = 0;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C50DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB13B0);
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C5104; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[10].u64;
	ctx.lr = 0x832C50FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.lr = 0x832C5108;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB3B40);
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C5118;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E0C);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5134; continue 'dispatch;
	}
	ctx.r[3].s64 = 0;
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C514C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5128; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5128; continue 'dispatch;
	}
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[30].s64 = 0;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C518C; continue 'dispatch;
	}
	ctx.cr[0].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5188; continue 'dispatch;
	}
	ctx.lr = 0x832C5174;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C51A0; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C51A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C2700);
	ctx.lr = 0x832C51A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C51CC; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C51CC; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5244; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C51F0; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5244; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5128; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].u64 = ctx.r[6].u64 & 0x00000000FFFFFFFFu64;
	ctx.r[11].s64 = ctx.r[4].s64 * ctx.r[7].s64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = ctx.r[11].s64 * 1000;
	ctx.r[7].u64 = ctx.r[8].u64 / ctx.r[9].u64;
	ctx.r[5].s64 = ctx.r[6].s64 * ctx.r[10].s64;
	ctx.r[4].u64 = ctx.r[5].u64 & 0x000000000000FFFFu64;
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[29].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200719656].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5284; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].lt {
			pc = 0x832C5274; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.r[30].s64 = ctx.r[31].s64 + 596;
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52AC; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52AC; continue 'dispatch;
	}
	ctx.r[4].s64 = 0;
	ctx.lr = 0x832C52A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[31].s64 + 336;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C52BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.lr = 0x832C52D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[3].s64 = 1;
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C52E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E00);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5300; continue 'dispatch;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E50);
	return;
	ctx.lr = 0x832C5304;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[28].s64 = 0;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5330; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5330; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[27].s64 = ctx.r[31].s64 + 852;
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	if ctx.cr[0].eq {
			pc = 0x832C5360; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5360; continue 'dispatch;
	}
	ctx.r[4].s64 = -1;
	ctx.lr = 0x832C5360;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].u64 = ctx.r[28].u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].gt {
			pc = 0x832C53A0; continue 'dispatch;
	}
	ctx.r[30].u64 = ctx.r[28].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].s64 = 1;
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C538C;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	ctx.r[30].s64 = ctx.r[30].s64 + 388;
	if ctx.cr[0].lt {
			pc = 0x832C5374; continue 'dispatch;
	}
	ctx.lr = 0x832C53A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8226DAB0);
	ctx.r[11].s64 = -2091909120;
	ctx.r[10].s64 = ctx.r[11].s64 + -15872;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C53C0; continue 'dispatch;
	}
	ctx.r[11].u64 = ctx.r[3].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	ctx.r[8].s64 = -1073741824;
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	if ctx.cr[0].gt {
			pc = 0x832C53E0; continue 'dispatch;
	}
	ctx.r[9].u64 = ctx.r[11].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5404; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5400; continue 'dispatch;
	}
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	pc = 0x832C5404; continue 'dispatch;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C541C; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C541C; continue 'dispatch;
	}
	ctx.lr = 0x832C541C;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5430; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C5430;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C2700);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E50);
	return;
}

pub fn sub_832C4FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C4FB0 size=4116
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5024; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C5020; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[9].u64;
	ctx.lr = 0x832C4FDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	pc = 0x832C5024; continue 'dispatch;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[31].s64 + 336;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C4FF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5024; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C5020; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[9].u64;
	ctx.lr = 0x832C501C;
	crate::rt::call_indirect(ctx.ctr.u32);
	pc = 0x832C5024; continue 'dispatch;
	ctx.lr = 0x832C5024;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB3B40);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5058; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C5054; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[9].u64;
	ctx.lr = 0x832C5050;
	crate::rt::call_indirect(ctx.ctr.u32);
	pc = 0x832C5058; continue 'dispatch;
	ctx.lr = 0x832C5058;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB3B40);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C508C; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C5088; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[9].u64;
	ctx.lr = 0x832C5084;
	crate::rt::call_indirect(ctx.ctr.u32);
	pc = 0x832C508C; continue 'dispatch;
	ctx.lr = 0x832C508C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB3B40);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C50CC; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].eq {
			pc = 0x832C50CC; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C50C8; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[9].u64;
	ctx.lr = 0x832C50C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	pc = 0x832C50CC; continue 'dispatch;
	ctx.lr = 0x832C50CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB3B40);
	ctx.r[5].s64 = 924;
	ctx.r[4].s64 = 0;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C50DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB13B0);
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C5104; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[10].u64;
	ctx.lr = 0x832C50FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.lr = 0x832C5108;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB3B40);
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C5118;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E0C);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5134; continue 'dispatch;
	}
	ctx.r[3].s64 = 0;
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C514C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5128; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5128; continue 'dispatch;
	}
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[30].s64 = 0;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C518C; continue 'dispatch;
	}
	ctx.cr[0].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5188; continue 'dispatch;
	}
	ctx.lr = 0x832C5174;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C51A0; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C51A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C2700);
	ctx.lr = 0x832C51A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C51CC; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C51CC; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5244; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C51F0; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5244; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5128; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].u64 = ctx.r[6].u64 & 0x00000000FFFFFFFFu64;
	ctx.r[11].s64 = ctx.r[4].s64 * ctx.r[7].s64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = ctx.r[11].s64 * 1000;
	ctx.r[7].u64 = ctx.r[8].u64 / ctx.r[9].u64;
	ctx.r[5].s64 = ctx.r[6].s64 * ctx.r[10].s64;
	ctx.r[4].u64 = ctx.r[5].u64 & 0x000000000000FFFFu64;
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[29].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200719656].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5284; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].lt {
			pc = 0x832C5274; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.r[30].s64 = ctx.r[31].s64 + 596;
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52AC; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52AC; continue 'dispatch;
	}
	ctx.r[4].s64 = 0;
	ctx.lr = 0x832C52A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[31].s64 + 336;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C52BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.lr = 0x832C52D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[3].s64 = 1;
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C52E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E00);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5300; continue 'dispatch;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E50);
	return;
	ctx.lr = 0x832C5304;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[28].s64 = 0;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5330; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5330; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[27].s64 = ctx.r[31].s64 + 852;
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	if ctx.cr[0].eq {
			pc = 0x832C5360; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5360; continue 'dispatch;
	}
	ctx.r[4].s64 = -1;
	ctx.lr = 0x832C5360;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].u64 = ctx.r[28].u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].gt {
			pc = 0x832C53A0; continue 'dispatch;
	}
	ctx.r[30].u64 = ctx.r[28].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].s64 = 1;
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C538C;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	ctx.r[30].s64 = ctx.r[30].s64 + 388;
	if ctx.cr[0].lt {
			pc = 0x832C5374; continue 'dispatch;
	}
	ctx.lr = 0x832C53A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8226DAB0);
	ctx.r[11].s64 = -2091909120;
	ctx.r[10].s64 = ctx.r[11].s64 + -15872;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C53C0; continue 'dispatch;
	}
	ctx.r[11].u64 = ctx.r[3].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	ctx.r[8].s64 = -1073741824;
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	if ctx.cr[0].gt {
			pc = 0x832C53E0; continue 'dispatch;
	}
	ctx.r[9].u64 = ctx.r[11].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5404; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5400; continue 'dispatch;
	}
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	pc = 0x832C5404; continue 'dispatch;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C541C; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C541C; continue 'dispatch;
	}
	ctx.lr = 0x832C541C;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5430; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C5430;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C2700);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E50);
	return;
}

pub fn sub_832C51A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C51A0 size=3096
	ctx.lr = 0x832C51A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C51CC; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C51CC; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5244; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C51F0; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5244; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			crate::recompiler::externs::call(&mut ctx, base, 0x832C5128);
		return;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].u64 = ctx.r[6].u64 & 0x00000000FFFFFFFFu64;
	ctx.r[11].s64 = ctx.r[4].s64 * ctx.r[7].s64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = ctx.r[11].s64 * 1000;
	ctx.r[7].u64 = ctx.r[8].u64 / ctx.r[9].u64;
	ctx.r[5].s64 = ctx.r[6].s64 * ctx.r[10].s64;
	ctx.r[4].u64 = ctx.r[5].u64 & 0x000000000000FFFFu64;
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[29].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200719656].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5284; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].lt {
			pc = 0x832C5274; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.r[30].s64 = ctx.r[31].s64 + 596;
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52AC; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52AC; continue 'dispatch;
	}
	ctx.r[4].s64 = 0;
	ctx.lr = 0x832C52A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[31].s64 + 336;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C52BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.lr = 0x832C52D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[3].s64 = 1;
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C52E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E00);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5300; continue 'dispatch;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E50);
	return;
	ctx.lr = 0x832C5304;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[28].s64 = 0;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5330; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5330; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[27].s64 = ctx.r[31].s64 + 852;
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	if ctx.cr[0].eq {
			pc = 0x832C5360; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5360; continue 'dispatch;
	}
	ctx.r[4].s64 = -1;
	ctx.lr = 0x832C5360;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].u64 = ctx.r[28].u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].gt {
			pc = 0x832C53A0; continue 'dispatch;
	}
	ctx.r[30].u64 = ctx.r[28].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].s64 = 1;
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C538C;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	ctx.r[30].s64 = ctx.r[30].s64 + 388;
	if ctx.cr[0].lt {
			pc = 0x832C5374; continue 'dispatch;
	}
	ctx.lr = 0x832C53A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8226DAB0);
	ctx.r[11].s64 = -2091909120;
	ctx.r[10].s64 = ctx.r[11].s64 + -15872;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C53C0; continue 'dispatch;
	}
	ctx.r[11].u64 = ctx.r[3].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	ctx.r[8].s64 = -1073741824;
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	if ctx.cr[0].gt {
			pc = 0x832C53E0; continue 'dispatch;
	}
	ctx.r[9].u64 = ctx.r[11].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5404; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5400; continue 'dispatch;
	}
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	pc = 0x832C5404; continue 'dispatch;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C541C; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C541C; continue 'dispatch;
	}
	ctx.lr = 0x832C541C;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5430; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C5430;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C2700);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E50);
	return;
}

pub fn sub_832C5290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C5290 size=2068
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52AC; continue 'dispatch;
	}
	ctx.r[4].s64 = 0;
	ctx.lr = 0x832C52A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[31].s64 + 336;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C52BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C52D4; continue 'dispatch;
	}
	ctx.lr = 0x832C52D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[3].s64 = 1;
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C52E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E00);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5300; continue 'dispatch;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E50);
	return;
	ctx.lr = 0x832C5304;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[28].s64 = 0;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5330; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5330; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[27].s64 = ctx.r[31].s64 + 852;
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	if ctx.cr[0].eq {
			pc = 0x832C5360; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5360; continue 'dispatch;
	}
	ctx.r[4].s64 = -1;
	ctx.lr = 0x832C5360;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].u64 = ctx.r[28].u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].gt {
			pc = 0x832C53A0; continue 'dispatch;
	}
	ctx.r[30].u64 = ctx.r[28].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].s64 = 1;
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C538C;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	ctx.r[30].s64 = ctx.r[30].s64 + 388;
	if ctx.cr[0].lt {
			pc = 0x832C5374; continue 'dispatch;
	}
	ctx.lr = 0x832C53A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8226DAB0);
	ctx.r[11].s64 = -2091909120;
	ctx.r[10].s64 = ctx.r[11].s64 + -15872;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C53C0; continue 'dispatch;
	}
	ctx.r[11].u64 = ctx.r[3].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	ctx.r[8].s64 = -1073741824;
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	if ctx.cr[0].gt {
			pc = 0x832C53E0; continue 'dispatch;
	}
	ctx.r[9].u64 = ctx.r[11].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5404; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5400; continue 'dispatch;
	}
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	pc = 0x832C5404; continue 'dispatch;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C541C; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C541C; continue 'dispatch;
	}
	ctx.lr = 0x832C541C;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C5430; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C5430;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C2700);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E50);
	return;
}

pub fn sub_832C55D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C55D8 size=8
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			crate::recompiler::externs::call(&mut ctx, base, 0x832C5594);
		return;
	}
}

pub fn sub_832C5AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C5AF8 size=4112
	ctx.lr = 0x832C5AFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CE1100);
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C5B08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CE10A8);
	ctx.r[4].s64 = 0;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C5B14;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CE0F48);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.lr = 0x832C5B1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	ctx.r[25].u64 = ctx.r[3].u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C5B2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CE1050);
	ctx.r[11].s64 = -2091909120;
	ctx.r[26].u64 = ctx.r[28].u64;
	ctx.r[27].s64 = -1073741824;
	ctx.r[30].s64 = ctx.r[11].s64 + -15872;
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C5B50;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CE1050);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	if ctx.cr[0].eq {
			pc = 0x832C5B68; continue 'dispatch;
	}
	ctx.r[26].s64 = 1;
	pc = 0x832C5C14; continue 'dispatch;
	ctx.cr[0].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5C14; continue 'dispatch;
	}
	ctx.lr = 0x832C5B74;
	crate::recompiler::externs::call(&mut ctx, base, 0x8226DAB0);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5B88; continue 'dispatch;
	}
	ctx.r[11].u64 = ctx.r[3].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if ctx.cr[0].gt {
			pc = 0x832C5BA4; continue 'dispatch;
	}
	ctx.r[10].u64 = ctx.r[11].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[25].s64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 50 as u32, &mut ctx.xer);
	if !ctx.cr[0].gt {
			pc = 0x832C5C14; continue 'dispatch;
	}
	ctx.r[4].s64 = 0;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C5BBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CE0F48);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.lr = 0x832C5BC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8226DAB0);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C5BD8; continue 'dispatch;
	}
	ctx.r[11].u64 = ctx.r[3].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if !ctx.cr[0].gt {
			pc = 0x832C5BF4; continue 'dispatch;
	}
	ctx.r[25].u64 = ctx.r[10].u64;
	pc = 0x832C5BFC; continue 'dispatch;
	ctx.r[25].u64 = ctx.r[11].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C5C0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CE1050);
	ctx.r[26].u64 = ctx.r[28].u64;
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].gt {
			pc = 0x832C5B40; continue 'dispatch;
	}
	ctx.lr = 0x832C5C28;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E4C);
	return;
}

pub fn sub_832C61E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C61E8 size=6156
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.cr[0].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	if !ctx.cr[0].eq {
			pc = 0x832C6204; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].s64 = 32767;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.cr[0].compare_i32(ctx.r[27].s32, 16, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	if !ctx.cr[0].eq {
			pc = 0x832C6228; continue 'dispatch;
	}
	ctx.r[11].u64 = ctx.r[28].u64;
	pc = 0x832C6230; continue 'dispatch;
	ctx.r[11].s64 = -2139095040;
	ctx.r[11].u64 = ctx.r[11].u64 | 32896;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C623C;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C56A0);
	ctx.r[4].s64 = 0;
	ctx.r[3].u64 = ctx.r[26].u64;
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C624C;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C5838);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[0].lt {
			pc = 0x832C6264; continue 'dispatch;
	}
	ctx.r[11].u64 = ctx.r[30].u64;
	ctx.r[10].s64 = -2091909120;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[9].s64 = -2094268416;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[8].s64 = -2094268416;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.r[7].s64 = -2094268416;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[25].u32) };
	ctx.r[9].s64 = ctx.r[9].s64 + -8544;
	ctx.r[8].s64 = ctx.r[8].s64 + -8096;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].s64 = ctx.r[7].s64 + -7624;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[3].s64 = -2094268416;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	ctx.r[6].s64 = -2094268416;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	ctx.r[5].s64 = -2094268416;
	ctx.r[4].s64 = -2094268416;
	ctx.r[30].s64 = -2094268416;
	ctx.r[29].s64 = -2094268416;
	ctx.r[28].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[3].s64 + -7016;
	ctx.r[6].s64 = ctx.r[6].s64 + -7496;
	ctx.r[5].s64 = ctx.r[5].s64 + -7472;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[4].s64 = ctx.r[4].s64 + -6824;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	ctx.r[9].s64 = ctx.r[30].s64 + -6664;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[5].u32) };
	ctx.r[8].s64 = ctx.r[29].s64 + -7064;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[4].u32) };
	ctx.r[7].s64 = ctx.r[28].s64 + -7448;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	ctx.r[3].s64 = 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E4C);
	return;
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -6504;
	return;
}

pub fn sub_832C62C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C62C0 size=4108
	ctx.r[6].s64 = ctx.r[6].s64 + -7496;
	ctx.r[5].s64 = ctx.r[5].s64 + -7472;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[4].s64 = ctx.r[4].s64 + -6824;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	ctx.r[9].s64 = ctx.r[30].s64 + -6664;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[5].u32) };
	ctx.r[8].s64 = ctx.r[29].s64 + -7064;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[4].u32) };
	ctx.r[7].s64 = ctx.r[28].s64 + -7448;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	ctx.r[3].s64 = 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E4C);
	return;
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -6504;
	return;
}

pub fn sub_832C6390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C6390 size=3084
	ctx.r[31].u64 = ctx.r[4].u64;
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C63B8; continue 'dispatch;
	}
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C6420; continue 'dispatch;
	}
	ctx.r[4].s64 = -1;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C63B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C63D4; continue 'dispatch;
	}
	ctx.r[4].s64 = -1;
	ctx.r[3].u64 = ctx.r[30].u64;
	ctx.lr = 0x832C63CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	ctx.r[29].s64 = -1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[5].s64 = 0;
	ctx.r[6].u64 = ctx.r[29].u64;
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	ctx.r[3].s64 = 2;
	ctx.lr = 0x832C63F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82181D10);
	ctx.cr[0].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	if ctx.cr[0].lt {
			pc = 0x832C6414; continue 'dispatch;
	}
	if !ctx.cr[0].eq {
			pc = 0x832C6420; continue 'dispatch;
	}
	ctx.r[4].u64 = ctx.r[29].u64;
	ctx.r[3].u64 = ctx.r[30].u64;
	ctx.lr = 0x832C640C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.r[4].u64 = ctx.r[29].u64;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C6420;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C6430;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[28].u64 = ctx.r[3].u64;
	ctx.r[29].s64 = 0;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[30].u64 = ctx.r[29].u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C64FC; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C65A0; continue 'dispatch;
	}
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C6478;
	crate::rt::call_indirect(ctx.ctr.u32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C64F0; continue 'dispatch;
	}
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C6498; continue 'dispatch;
	}
	ctx.r[30].u64 = ctx.r[31].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	pc = 0x832C64F0; continue 'dispatch;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].lt {
			pc = 0x832C64B0; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	ctx.r[30].u64 = ctx.r[31].u64;
	pc = 0x832C64F0; continue 'dispatch;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].s64 = ctx.r[30].s64 + 8;
	ctx.r[9].u64 = ctx.r[30].u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C64E8; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C6594; continue 'dispatch;
	}
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].s64 = ctx.r[11].s64 + 8;
	ctx.r[9].u64 = ctx.r[11].u64;
	ctx.cr[0].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C64C4; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C6458; continue 'dispatch;
	}
	ctx.r[11].u64 = ctx.r[30].u64;
	ctx.r[31].u64 = ctx.r[29].u64;
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C65A0; continue 'dispatch;
	}
	ctx.r[10].u64 = ctx.r[29].u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 64 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C6540; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32), ctx.r[11].u32) };
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C6510; continue 'dispatch;
	}
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C65A0; continue 'dispatch;
	}
	ctx.r[6].s64 = 1;
	ctx.r[5].s64 = 0;
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C655C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82181D10);
	if !ctx.cr[0].lt {
			pc = 0x832C6448; continue 'dispatch;
	}
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	ctx.r[3].u64 = ctx.r[10].u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[9].u64;
	ctx.lr = 0x832C6584;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	ctx.lr = 0x832C6590;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	pc = 0x832C6448; continue 'dispatch;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	pc = 0x832C64EC; continue 'dispatch;
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C65CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C6600; continue 'dispatch;
	}
	ctx.r[30].s64 = -1;
	ctx.r[4].u64 = ctx.r[30].u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C65E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C661C; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C6630; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C6600;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[3].s64 = 0;
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C6630; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C6630;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C6428);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C6638;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C6644;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C65D8; continue 'dispatch;
	}
	pc = 0x832C6600; continue 'dispatch;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C6658;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2091909120;
	ctx.r[28].s64 = 0;
	ctx.r[29].s64 = ctx.r[10].s64 + -15244;
	ctx.r[11].u64 = ctx.r[28].u64;
	ctx.r[10].u64 = ctx.r[29].u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C6698; continue 'dispatch;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	if ctx.cr[0].lt {
			pc = 0x832C6670; continue 'dispatch;
	}
	ctx.r[3].s64 = 0;
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[10].s64 = -2092367872;
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	ctx.r[30].s64 = ctx.r[10].s64 + 26544;
	ctx.r[8].s64 = 1;
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[29].u32), ctx.r[8].u32) };
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C6744; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C66D0; continue 'dispatch;
	}
	ctx.r[3].s64 = -ctx.r[3].s64;
	ctx.r[11].s64 = 1000;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.r[10].s64 = -2094268416;
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[3].s32;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.r[4].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[8].s64 = ctx.r[31].s64 + 16;
	ctx.r[7].s64 = 4;
	ctx.r[6].u64 = ctx.r[31].u64;
	ctx.r[5].s64 = ctx.r[10].s64 + -5208;
	ctx.r[4].u64 = ctx.r[4].u64 | 32768;
	ctx.r[3].s64 = 0;
	ctx.lr = 0x832C6710;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCB618);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[4].s64 = 5;
	ctx.lr = 0x832C671C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCA070);
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C6750; continue 'dispatch;
	}
	ctx.r[11].s64 = 36;
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	ctx.r[11].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C6744; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u32) };
	ctx.r[3].s64 = 0;
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[5].s64 = 0;
	ctx.r[4].s64 = 0;
	ctx.r[3].s64 = 0;
	ctx.lr = 0x832C6760;
	crate::recompiler::externs::call(&mut ctx, base, 0x83013338);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = 0;
	ctx.r[4].s64 = 0;
	ctx.r[3].s64 = 0;
	ctx.lr = 0x832C6778;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCA6E0);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[30].u64 = ctx.r[4].u64;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.r[4].s64 = 0;
	ctx.r[3].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[5].u32) };
	ctx.r[5].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	ctx.lr = 0x832C67BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83013338);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.r[30].u64 = ctx.r[4].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	if !ctx.cr[0].eq {
			pc = 0x832C6844; continue 'dispatch;
	}
	ctx.lr = 0x832C6828;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C6898);
	ctx.r[4].s64 = -1;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C6834;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C683C;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C6910);
	ctx.r[31].s64 = 1;
	pc = 0x832C686C; continue 'dispatch;
	ctx.r[11].s64 = 4;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].s64 = -1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832C6858;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[10].s64 = 1;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.lr = 0x832C6868;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[31].s64 = 0;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C6874;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCA1B0);
	ctx.r[11].s64 = 0;
	ctx.r[3].u64 = ctx.r[31].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C68BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x822D0D88);
	ctx.r[11].s64 = 3;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].s64 = 15;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832C68D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC9EC0);
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C68EC; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C68F4; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C68E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	pc = 0x832C68F4; continue 'dispatch;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C68F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCF868);
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832C65C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C65C0 size=5132
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C65CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C6600; continue 'dispatch;
	}
	ctx.r[30].s64 = -1;
	ctx.r[4].u64 = ctx.r[30].u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C65E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C661C; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C6630; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C6600;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[3].s64 = 0;
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C6630; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C6630;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C6428);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C6638;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C6644;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C65D8; continue 'dispatch;
	}
	pc = 0x832C6600; continue 'dispatch;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C6658;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2091909120;
	ctx.r[28].s64 = 0;
	ctx.r[29].s64 = ctx.r[10].s64 + -15244;
	ctx.r[11].u64 = ctx.r[28].u64;
	ctx.r[10].u64 = ctx.r[29].u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C6698; continue 'dispatch;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	if ctx.cr[0].lt {
			pc = 0x832C6670; continue 'dispatch;
	}
	ctx.r[3].s64 = 0;
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[10].s64 = -2092367872;
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	ctx.r[30].s64 = ctx.r[10].s64 + 26544;
	ctx.r[8].s64 = 1;
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[29].u32), ctx.r[8].u32) };
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C6744; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C66D0; continue 'dispatch;
	}
	ctx.r[3].s64 = -ctx.r[3].s64;
	ctx.r[11].s64 = 1000;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.r[10].s64 = -2094268416;
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[3].s32;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	ctx.r[4].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[8].s64 = ctx.r[31].s64 + 16;
	ctx.r[7].s64 = 4;
	ctx.r[6].u64 = ctx.r[31].u64;
	ctx.r[5].s64 = ctx.r[10].s64 + -5208;
	ctx.r[4].u64 = ctx.r[4].u64 | 32768;
	ctx.r[3].s64 = 0;
	ctx.lr = 0x832C6710;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCB618);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[4].s64 = 5;
	ctx.lr = 0x832C671C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCA070);
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C6750; continue 'dispatch;
	}
	ctx.r[11].s64 = 36;
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	ctx.r[11].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C6744; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u32) };
	ctx.r[3].s64 = 0;
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[5].s64 = 0;
	ctx.r[4].s64 = 0;
	ctx.r[3].s64 = 0;
	ctx.lr = 0x832C6760;
	crate::recompiler::externs::call(&mut ctx, base, 0x83013338);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = 0;
	ctx.r[4].s64 = 0;
	ctx.r[3].s64 = 0;
	ctx.lr = 0x832C6778;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCA6E0);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[30].u64 = ctx.r[4].u64;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.r[4].s64 = 0;
	ctx.r[3].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[5].u32) };
	ctx.r[5].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	ctx.lr = 0x832C67BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83013338);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.r[30].u64 = ctx.r[4].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	if !ctx.cr[0].eq {
			pc = 0x832C6844; continue 'dispatch;
	}
	ctx.lr = 0x832C6828;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C6898);
	ctx.r[4].s64 = -1;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C6834;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C683C;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C6910);
	ctx.r[31].s64 = 1;
	pc = 0x832C686C; continue 'dispatch;
	ctx.r[11].s64 = 4;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].s64 = -1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832C6858;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[10].s64 = 1;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.lr = 0x832C6868;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[31].s64 = 0;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C6874;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCA1B0);
	ctx.r[11].s64 = 0;
	ctx.r[3].u64 = ctx.r[31].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C68BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x822D0D88);
	ctx.r[11].s64 = 3;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].s64 = 15;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832C68D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC9EC0);
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C68EC; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C68F4; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C68E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	pc = 0x832C68F4; continue 'dispatch;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C68F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCF868);
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832C6818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C6818 size=2068
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	if !ctx.cr[0].eq {
			pc = 0x832C6844; continue 'dispatch;
	}
	ctx.lr = 0x832C6828;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C6898);
	ctx.r[4].s64 = -1;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C6834;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C683C;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C6910);
	ctx.r[31].s64 = 1;
	pc = 0x832C686C; continue 'dispatch;
	ctx.r[11].s64 = 4;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].s64 = -1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832C6858;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219E3C8);
	ctx.r[10].s64 = 1;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.lr = 0x832C6868;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	ctx.r[31].s64 = 0;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C6874;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCA1B0);
	ctx.r[11].s64 = 0;
	ctx.r[3].u64 = ctx.r[31].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C68BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x822D0D88);
	ctx.r[11].s64 = 3;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].s64 = 15;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832C68D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC9EC0);
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C68EC; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C68F4; continue 'dispatch;
	}
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C68E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x830133C0);
	pc = 0x832C68F4; continue 'dispatch;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C68F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCF868);
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832C6BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C6BA0 size=4112
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2091909120;
	ctx.r[26].u64 = ctx.r[3].u64;
	ctx.r[31].s64 = ctx.r[11].s64 + -15844;
	ctx.r[25].u64 = ctx.r[4].u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].eq {
			pc = 0x832C6C64; continue 'dispatch;
	}
	ctx.lr = 0x832C6BCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82BF1790);
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].u64 = ctx.r[3].u64;
	ctx.r[5].u64 = ctx.r[24].u64;
	ctx.r[4].u64 = ctx.r[28].u64;
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	ctx.lr = 0x832C6BF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E80);
	ctx.r[5].u64 = ctx.r[24].u64;
	ctx.r[3].u64 = ctx.r[27].u64;
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C6C00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E80);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[11].u32, 64 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C6C50; continue 'dispatch;
	}
	ctx.cr[0].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C6C50; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	if !ctx.cr[0].eq {
			pc = 0x832C6C4C; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.ctr.u64 = ctx.r[10].u64;
	ctx.lr = 0x832C6C38;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[8].u64 = ctx.r[27].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	pc = 0x832C6C6C; continue 'dispatch;
	ctx.lr = 0x832C6C50;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB3B40);
	ctx.r[8].u64 = ctx.r[27].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	pc = 0x832C6C6C; continue 'dispatch;
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[25].s64 + 31;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.r[7].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[3].s64 = ctx.r[7].s64 + 1;
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[8].u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[29].u32), ctx.r[26].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E48);
	return;
}

pub fn sub_832C6FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C6FD0 size=12
	ctx.r[12].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[12].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.r[12].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
}

pub fn sub_832C7070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832C7070 size=2060
	ctx.cr[0].compare_u32(ctx.r[11].u32, 29 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C70A8; continue 'dispatch;
	}
	if !ctx.cr[0].lt {
			pc = 0x832C740C; continue 'dispatch;
	}
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 29 as u32;
	ctx.r[7].s64 = (29 as i64) - ctx.r[11].s64;
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[5].u64 = ctx.r[6].u64 | ctx.r[9].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x1FFFFFFFu64;
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	pc = 0x832C70B4; continue 'dispatch;
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x1FFFFFFFu64;
	ctx.r[11].s64 = ctx.r[11].s64 + -29;
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x3FFFFFFFu64;
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[4].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[5].u64 = ctx.r[5].u64 & 0x00000000FFFFFFFFu64;
	ctx.cr[0].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[5].u64) };
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	if ctx.cr[0].eq {
			pc = 0x832C70EC; continue 'dispatch;
	}
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	ctx.cr[0].compare_u32(ctx.r[11].u32, 29 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C7128; continue 'dispatch;
	}
	if !ctx.cr[0].lt {
			pc = 0x832C740C; continue 'dispatch;
	}
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 29 as u32;
	ctx.r[7].s64 = (29 as i64) - ctx.r[11].s64;
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[5].u64 = ctx.r[6].u64 | ctx.r[9].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x1FFFFFFFu64;
	pc = 0x832C7134; continue 'dispatch;
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x1FFFFFFFu64;
	ctx.r[11].s64 = ctx.r[11].s64 + -29;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x3FFFFFFFu64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[4].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[5].u64 = ctx.r[5].u64 & 0x00000000FFFFFFFFu64;
	ctx.cr[0].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[5].u64) };
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	ctx.f[0].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	if ctx.cr[0].eq {
			pc = 0x832C7174; continue 'dispatch;
	}
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	ctx.r[5].s64 = 0;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	if ctx.cr[0].lt {
			pc = 0x832C7330; continue 'dispatch;
	}
	ctx.r[6].s64 = ctx.r[1].s64 + 116;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C71BC; continue 'dispatch;
	}
	if !ctx.cr[0].lt {
			pc = 0x832C73B8; continue 'dispatch;
	}
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[7].s64 = (8 as i64) - ctx.r[11].s64;
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[9].u64;
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[8].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	pc = 0x832C71C8; continue 'dispatch;
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[8].u32, 95 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	if !ctx.cr[0].gt {
			pc = 0x832C71DC; continue 'dispatch;
	}
	ctx.r[8].s64 = 95;
	ctx.r[9].s64 = ctx.r[31].s64 + -384;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	if !ctx.cr[0].lt {
			pc = 0x832C7220; continue 'dispatch;
	}
	if !ctx.cr[0].lt {
			pc = 0x832C73B8; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[8].s64 = (8 as i64) - ctx.r[11].s64;
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[7].u64;
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	pc = 0x832C722C; continue 'dispatch;
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[9].u32, 95 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	if !ctx.cr[0].gt {
			pc = 0x832C7240; continue 'dispatch;
	}
	ctx.r[9].s64 = 95;
	ctx.r[7].s64 = ctx.r[31].s64 + -384;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	if !ctx.cr[0].lt {
			pc = 0x832C7284; continue 'dispatch;
	}
	if !ctx.cr[0].lt {
			pc = 0x832C73B8; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[7].s64 = (8 as i64) - ctx.r[11].s64;
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[8].u64;
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[9].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	pc = 0x832C7290; continue 'dispatch;
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[9].u32, 95 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	if !ctx.cr[0].gt {
			pc = 0x832C72A4; continue 'dispatch;
	}
	ctx.r[9].s64 = 95;
	ctx.r[8].s64 = ctx.r[31].s64 + -384;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	if !ctx.cr[0].lt {
			pc = 0x832C72E8; continue 'dispatch;
	}
	if !ctx.cr[0].lt {
			pc = 0x832C73B8; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[8].s64 = (8 as i64) - ctx.r[11].s64;
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[7].u64;
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	pc = 0x832C72F4; continue 'dispatch;
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	ctx.r[8].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[8].u32, 95 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	if !ctx.cr[0].gt {
			pc = 0x832C7308; continue 'dispatch;
	}
	ctx.r[8].s64 = 95;
	ctx.r[7].s64 = ctx.r[31].s64 + -384;
	ctx.r[5].s64 = ctx.r[5].s64 + 4;
	ctx.r[3].s64 = ctx.r[29].s64 + -3;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	ctx.r[6].s64 = ctx.r[6].s64 + 16;
	if ctx.cr[0].lt {
			pc = 0x832C7188; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	if !ctx.cr[0].lt {
			pc = 0x832C73BC; continue 'dispatch;
	}
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C7378; continue 'dispatch;
	}
	if !ctx.cr[0].lt {
			pc = 0x832C73B0; continue 'dispatch;
	}
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[6].s64 = (8 as i64) - ctx.r[11].s64;
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[9].u64;
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) >> ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	pc = 0x832C7384; continue 'dispatch;
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	ctx.cr[0].compare_u32(ctx.r[8].u32, 95 as u32, &mut ctx.xer);
	if !ctx.cr[0].gt {
			pc = 0x832C7390; continue 'dispatch;
	}
	ctx.r[8].s64 = 95;
	ctx.r[6].s64 = ctx.r[31].s64 + -384;
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[6].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	if ctx.cr[0].lt {
			pc = 0x832C7344; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[8].u64 = ctx.r[30].u64;
	ctx.r[7].u64 = ctx.r[23].u64;
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	ctx.r[4].u64 = ctx.r[27].u64;
	ctx.r[3].u64 = ctx.r[28].u64;
	ctx.lr = 0x832C73D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C6CB8);
	ctx.cr[0].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C73F4; continue 'dispatch;
	}
	ctx.r[6].u64 = ctx.r[21].u64;
	ctx.r[5].u64 = ctx.r[22].u64;
	ctx.r[4].u64 = ctx.r[28].u64;
	ctx.r[3].u64 = ctx.r[27].u64;
	ctx.lr = 0x832C73F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x832CC7B0);
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	if ctx.cr[0].lt {
			crate::recompiler::externs::call(&mut ctx, base, 0x832C7068);
		return;
	}
	ctx.cr[0].compare_u32(ctx.r[24].u32, 1 as u32, &mut ctx.xer);
	ctx.r[11].s64 = ctx.r[1].s64 + 224;
	if !ctx.cr[0].eq {
			pc = 0x832C74D0; continue 'dispatch;
	}
	ctx.r[9].u64 = ctx.r[27].u64;
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C74A0; continue 'dispatch;
	}
	ctx.r[7].s64 = -2113273856;
	ctx.r[6].s64 = -2112880640;
	ctx.r[5].s64 = -2112880640;
	ctx.r[8].s64 = -2112880640;
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	ctx.r[8].s64 = ctx.r[8].s64 + 3688;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	ctx.f[10].f64 = (((ctx.f[11].f64 * ctx.f[31].f64) as f32) as f64);
	ctx.f[9].f64 = ((ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64;
	ctx.cr[0].compare_f64(ctx.f[9].f64, ctx.f[13].f64);
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[7].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	ctx.r[4].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	ctx.f[7].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[10].f64 } else { ctx.f[12].f64 };
	ctx.f[6].f64 = (((ctx.f[7].f64 - ctx.f[0].f64) as f32) as f64);
	ctx.cr[0].compare_f64(ctx.f[6].f64, ctx.f[13].f64);
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[3].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0x0000001Fu64;
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x00000003u64;
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	ctx.f[4].f64 = if ctx.f[5].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[7].f64 };
	ctx.f[3].s64 = if ctx.f[4].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[4].f64.trunc() as i32 as i64 };
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	if !ctx.cr[2200728644].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[9].s64 = ctx.r[1].s64 + 224;
	ctx.r[8].u64 = ctx.r[20].u64;
	ctx.r[11].u64 = ctx.r[27].u64;
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7590; continue 'dispatch;
	}
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[7].u16) };
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	if !ctx.cr[2200728756].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	pc = 0x832C7590; continue 'dispatch;
	ctx.cr[0].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7558; continue 'dispatch;
	}
	ctx.r[7].s64 = -2113273856;
	ctx.r[6].s64 = -2112880640;
	ctx.r[5].s64 = -2112880640;
	ctx.r[8].s64 = -2112880640;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	ctx.r[8].s64 = ctx.r[8].s64 + 3688;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	ctx.f[10].f64 = (((ctx.f[31].f64 * ctx.f[11].f64) as f32) as f64);
	ctx.f[9].f64 = ((ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64;
	ctx.cr[0].compare_f64(ctx.f[9].f64, ctx.f[13].f64);
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[7].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	ctx.r[4].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	ctx.f[7].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[10].f64 } else { ctx.f[12].f64 };
	ctx.f[6].f64 = (((ctx.f[7].f64 - ctx.f[0].f64) as f32) as f64);
	ctx.cr[0].compare_f64(ctx.f[6].f64, ctx.f[13].f64);
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[3].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0x0000001Fu64;
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x00000003u64;
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	ctx.f[4].f64 = if ctx.f[5].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[7].f64 };
	ctx.f[3].s64 = if ctx.f[4].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[4].f64.trunc() as i32 as i64 };
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	if !ctx.cr[2200728828].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[11].s64 = ctx.r[1].s64 + 224;
	ctx.r[9].u64 = ctx.r[27].u64;
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7590; continue 'dispatch;
	}
	ctx.r[7].s64 = ctx.r[1].s64 + 224;
	ctx.r[7].s64 = ctx.r[20].s64 - ctx.r[7].s64;
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[6].u64 & 0xFFFFFFFF0000FFFF);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[6].u32) };
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	if !ctx.cr[2200728948].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x1FFFFFFFu64;
	ctx.r[1].s64 = ctx.r[1].s64 + 16720;
	ctx.f[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E38);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C75B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0DE0);
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.f[29].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.f[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.f[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[21].u64 = ctx.r[4].u64;
	ctx.r[19].u64 = ctx.r[5].u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 44100 as u32, &mut ctx.xer);
	if ctx.cr[0].lt {
			pc = 0x832C75D8; continue 'dispatch;
	}
	ctx.r[26].s64 = 2048;
	pc = 0x832C75EC; continue 'dispatch;
	ctx.r[11].s64 = 22050;
	ctx.xer.ca = ctx.r[3].u32 >= ctx.r[11].u32;
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	tmp.u8 = ((~ctx.r[10].u32).wrapping_add(ctx.r[10].u32) < (~ctx.r[10].u32)) as u8 | ((~ctx.r[10].u32).wrapping_add(ctx.r[10].u32).wrapping_add(ctx.xer.ca as u32) < ctx.xer.ca as u32) as u8;
	ctx.r[9].u64 = (!ctx.r[10].u64).wrapping_add(ctx.r[10].u64).wrapping_add(ctx.xer.ca as u64);
	ctx.xer.ca = (tmp.u8 != 0);
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[26].s64 = ctx.r[11].s64 + 1024;
	ctx.r[11].s64 = (ctx.r[26].s32 as i64) * (ctx.r[21].s32 as i64);
	ctx.r[10].u64 = ctx.r[19].u32 as u64 & 0x00000001u64;
	ctx.r[18].s64 = 1;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C7610; continue 'dispatch;
	}
	ctx.r[3].s64 = (ctx.r[3].s32 as i64) * (ctx.r[21].s32 as i64);
	ctx.r[26].u64 = ctx.r[11].u64;
	ctx.r[21].u64 = ctx.r[18].u64;
	ctx.r[11].s64 = -2091909120;
	ctx.r[9].s64 = ctx.r[3].s64 + 1;
	ctx.r[20].s64 = ctx.r[11].s64 + -15840;
	ctx.r[27].s64 = 0;
	ctx.r[11].s64 = ctx.r[20].s64 + 4;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C7698; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C767C; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C7684; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C768C; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C7694; continue 'dispatch;
	}
	ctx.r[27].s64 = ctx.r[27].s64 + 5;
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	ctx.cr[0].compare_u32(ctx.r[27].u32, 25 as u32, &mut ctx.xer);
	if ctx.cr[0].lt {
			pc = 0x832C762C; continue 'dispatch;
	}
	pc = 0x832C7698; continue 'dispatch;
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	pc = 0x832C7698; continue 'dispatch;
	ctx.r[27].s64 = ctx.r[27].s64 + 2;
	pc = 0x832C7698; continue 'dispatch;
	ctx.r[27].s64 = ctx.r[27].s64 + 3;
	pc = 0x832C7698; continue 'dispatch;
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	ctx.r[9].u64 = ctx.r[22].u64 & 0x00000000FFFFFFFFu64;
	ctx.r[11].s64 = -2113273856;
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[9].u64) };
	ctx.f[0].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	ctx.r[25].s64 = ctx.r[11].s64 + -19328;
	ctx.f[0].f64 = (ctx.f[13].f64 as f32) as f64;
	ctx.r[8].s64 = -2113929216;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	ctx.r[7].s64 = -2113929216;
	ctx.r[6].s64 = ctx.r[11].s64 + 175;
	ctx.r[28].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	ctx.f[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.cr[0].compare_f64(ctx.f[0].f64, ctx.f[29].f64);
	if !ctx.cr[0].gt {
			pc = 0x832C7718; continue 'dispatch;
	}
	ctx.f[12].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	ctx.f[11].f64 = -(ctx.f[12].f64 * ctx.f[13].f64 - ctx.f[31].f64);
	ctx.f[10].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	ctx.f[9].f64 = ctx.f[10].f64 * ctx.f[30].f64;
	ctx.f[8].f64 = ctx.f[0].f64 * ctx.f[9].f64;
	ctx.f[7].f64 = -(ctx.f[8].f64 * ctx.f[9].f64 - ctx.f[31].f64);
	ctx.f[6].f64 = ctx.f[7].f64 * ctx.f[9].f64;
	ctx.f[5].f64 = ctx.f[6].f64 * ctx.f[30].f64;
	ctx.f[4].f64 = ctx.f[0].f64 * ctx.f[5].f64;
	ctx.f[3].f64 = -(ctx.f[4].f64 * ctx.f[5].f64 - ctx.f[31].f64);
	ctx.f[2].f64 = ctx.f[3].f64 * ctx.f[5].f64;
	ctx.f[1].f64 = ctx.f[2].f64 * ctx.f[30].f64;
	ctx.f[0].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.f[0].u64) };
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	ctx.r[10].s64 = ctx.r[11].s64 + 23;
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	if ctx.cr[0].eq {
			pc = 0x832C7758; continue 'dispatch;
	}
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	ctx.r[10].s64 = ctx.r[11].s64 + 15;
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	pc = 0x832C7768; continue 'dispatch;
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[24].u64;
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.lr = 0x832C7778;
	crate::recompiler::externs::call(&mut ctx, base, 0x82BF1790);
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C7798; continue 'dispatch;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E30);
	return;
	ctx.r[5].s64 = 152;
	ctx.r[4].s64 = 0;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C77A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB13B0);
	ctx.r[11].u64 = ctx.r[26].u64 & 0x00000000FFFFFFFFu64;
	ctx.r[10].s64 = ctx.r[31].s64 + 160;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[11].u64) };
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	ctx.r[9].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	ctx.f[0].f64 = (ctx.f[13].f64 as f32) as f64;
	ctx.r[8].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	ctx.r[7].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[19].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[21].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[27].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[26].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[24].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	ctx.cr[0].compare_f64(ctx.f[0].f64, ctx.f[29].f64);
	if !ctx.cr[0].gt {
			pc = 0x832C7840; continue 'dispatch;
	}
	ctx.f[12].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	ctx.f[11].f64 = -(ctx.f[12].f64 * ctx.f[13].f64 - ctx.f[31].f64);
	ctx.f[10].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	ctx.f[9].f64 = ctx.f[10].f64 * ctx.f[30].f64;
	ctx.f[8].f64 = ctx.f[0].f64 * ctx.f[9].f64;
	ctx.f[7].f64 = -(ctx.f[8].f64 * ctx.f[9].f64 - ctx.f[31].f64);
	ctx.f[6].f64 = ctx.f[7].f64 * ctx.f[9].f64;
	ctx.f[5].f64 = ctx.f[6].f64 * ctx.f[30].f64;
	ctx.f[4].f64 = ctx.f[0].f64 * ctx.f[5].f64;
	ctx.f[3].f64 = -(ctx.f[4].f64 * ctx.f[5].f64 - ctx.f[31].f64);
	ctx.f[2].f64 = ctx.f[3].f64 * ctx.f[5].f64;
	ctx.f[1].f64 = ctx.f[2].f64 * ctx.f[30].f64;
	ctx.f[0].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	ctx.r[8].s64 = 0;
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			crate::recompiler::externs::call(&mut ctx, base, 0x832C7890);
		return;
	}
	ctx.r[10].u64 = ctx.r[20].u64;
	ctx.r[11].s64 = ctx.r[31].s64 + 52;
	ctx.r[9].u64 = ctx.r[27].u64;
	ctx.r[8].u64 = ctx.r[27].u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].s64 = (ctx.r[22].s32 as i64) * (ctx.r[7].s32 as i64);
	ctx.r[5].u32 = ctx.r[6].u32 / ctx.r[23].u32;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[5].u32) };
	if !ctx.cr[2200729728].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
}

pub fn sub_832C7118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832C7118 size=3088
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x1FFFFFFFu64;
	pc = 0x832C7134; continue 'dispatch;
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x1FFFFFFFu64;
	ctx.r[11].s64 = ctx.r[11].s64 + -29;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x3FFFFFFFu64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[4].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[5].u64 = ctx.r[5].u64 & 0x00000000FFFFFFFFu64;
	ctx.cr[0].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[5].u64) };
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	ctx.f[0].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	if ctx.cr[0].eq {
			pc = 0x832C7174; continue 'dispatch;
	}
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	ctx.r[5].s64 = 0;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	if ctx.cr[0].lt {
			pc = 0x832C7330; continue 'dispatch;
	}
	ctx.r[6].s64 = ctx.r[1].s64 + 116;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C71BC; continue 'dispatch;
	}
	if !ctx.cr[0].lt {
			pc = 0x832C73B8; continue 'dispatch;
	}
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[7].s64 = (8 as i64) - ctx.r[11].s64;
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[9].u64;
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[8].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	pc = 0x832C71C8; continue 'dispatch;
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[8].u32, 95 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	if !ctx.cr[0].gt {
			pc = 0x832C71DC; continue 'dispatch;
	}
	ctx.r[8].s64 = 95;
	ctx.r[9].s64 = ctx.r[31].s64 + -384;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	if !ctx.cr[0].lt {
			pc = 0x832C7220; continue 'dispatch;
	}
	if !ctx.cr[0].lt {
			pc = 0x832C73B8; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[8].s64 = (8 as i64) - ctx.r[11].s64;
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[7].u64;
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	pc = 0x832C722C; continue 'dispatch;
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[9].u32, 95 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	if !ctx.cr[0].gt {
			pc = 0x832C7240; continue 'dispatch;
	}
	ctx.r[9].s64 = 95;
	ctx.r[7].s64 = ctx.r[31].s64 + -384;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	if !ctx.cr[0].lt {
			pc = 0x832C7284; continue 'dispatch;
	}
	if !ctx.cr[0].lt {
			pc = 0x832C73B8; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[7].s64 = (8 as i64) - ctx.r[11].s64;
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[8].u64;
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[9].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	pc = 0x832C7290; continue 'dispatch;
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[9].u32, 95 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	if !ctx.cr[0].gt {
			pc = 0x832C72A4; continue 'dispatch;
	}
	ctx.r[9].s64 = 95;
	ctx.r[8].s64 = ctx.r[31].s64 + -384;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	if !ctx.cr[0].lt {
			pc = 0x832C72E8; continue 'dispatch;
	}
	if !ctx.cr[0].lt {
			pc = 0x832C73B8; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[8].s64 = (8 as i64) - ctx.r[11].s64;
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[7].u64;
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	pc = 0x832C72F4; continue 'dispatch;
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	ctx.r[8].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[8].u32, 95 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	if !ctx.cr[0].gt {
			pc = 0x832C7308; continue 'dispatch;
	}
	ctx.r[8].s64 = 95;
	ctx.r[7].s64 = ctx.r[31].s64 + -384;
	ctx.r[5].s64 = ctx.r[5].s64 + 4;
	ctx.r[3].s64 = ctx.r[29].s64 + -3;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	ctx.r[6].s64 = ctx.r[6].s64 + 16;
	if ctx.cr[0].lt {
			pc = 0x832C7188; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	if !ctx.cr[0].lt {
			pc = 0x832C73BC; continue 'dispatch;
	}
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C7378; continue 'dispatch;
	}
	if !ctx.cr[0].lt {
			pc = 0x832C73B0; continue 'dispatch;
	}
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[6].s64 = (8 as i64) - ctx.r[11].s64;
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[9].u64;
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) >> ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	pc = 0x832C7384; continue 'dispatch;
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	ctx.cr[0].compare_u32(ctx.r[8].u32, 95 as u32, &mut ctx.xer);
	if !ctx.cr[0].gt {
			pc = 0x832C7390; continue 'dispatch;
	}
	ctx.r[8].s64 = 95;
	ctx.r[6].s64 = ctx.r[31].s64 + -384;
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[6].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	if ctx.cr[0].lt {
			pc = 0x832C7344; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[8].u64 = ctx.r[30].u64;
	ctx.r[7].u64 = ctx.r[23].u64;
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	ctx.r[4].u64 = ctx.r[27].u64;
	ctx.r[3].u64 = ctx.r[28].u64;
	ctx.lr = 0x832C73D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C6CB8);
	ctx.cr[0].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C73F4; continue 'dispatch;
	}
	ctx.r[6].u64 = ctx.r[21].u64;
	ctx.r[5].u64 = ctx.r[22].u64;
	ctx.r[4].u64 = ctx.r[28].u64;
	ctx.r[3].u64 = ctx.r[27].u64;
	ctx.lr = 0x832C73F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x832CC7B0);
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	if ctx.cr[0].lt {
			crate::recompiler::externs::call(&mut ctx, base, 0x832C7068);
		return;
	}
	ctx.cr[0].compare_u32(ctx.r[24].u32, 1 as u32, &mut ctx.xer);
	ctx.r[11].s64 = ctx.r[1].s64 + 224;
	if !ctx.cr[0].eq {
			pc = 0x832C74D0; continue 'dispatch;
	}
	ctx.r[9].u64 = ctx.r[27].u64;
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C74A0; continue 'dispatch;
	}
	ctx.r[7].s64 = -2113273856;
	ctx.r[6].s64 = -2112880640;
	ctx.r[5].s64 = -2112880640;
	ctx.r[8].s64 = -2112880640;
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	ctx.r[8].s64 = ctx.r[8].s64 + 3688;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	ctx.f[10].f64 = (((ctx.f[11].f64 * ctx.f[31].f64) as f32) as f64);
	ctx.f[9].f64 = ((ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64;
	ctx.cr[0].compare_f64(ctx.f[9].f64, ctx.f[13].f64);
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[7].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	ctx.r[4].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	ctx.f[7].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[10].f64 } else { ctx.f[12].f64 };
	ctx.f[6].f64 = (((ctx.f[7].f64 - ctx.f[0].f64) as f32) as f64);
	ctx.cr[0].compare_f64(ctx.f[6].f64, ctx.f[13].f64);
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[3].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0x0000001Fu64;
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x00000003u64;
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	ctx.f[4].f64 = if ctx.f[5].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[7].f64 };
	ctx.f[3].s64 = if ctx.f[4].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[4].f64.trunc() as i32 as i64 };
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	if !ctx.cr[2200728644].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[9].s64 = ctx.r[1].s64 + 224;
	ctx.r[8].u64 = ctx.r[20].u64;
	ctx.r[11].u64 = ctx.r[27].u64;
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7590; continue 'dispatch;
	}
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[7].u16) };
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	if !ctx.cr[2200728756].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	pc = 0x832C7590; continue 'dispatch;
	ctx.cr[0].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7558; continue 'dispatch;
	}
	ctx.r[7].s64 = -2113273856;
	ctx.r[6].s64 = -2112880640;
	ctx.r[5].s64 = -2112880640;
	ctx.r[8].s64 = -2112880640;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	ctx.r[8].s64 = ctx.r[8].s64 + 3688;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	ctx.f[10].f64 = (((ctx.f[31].f64 * ctx.f[11].f64) as f32) as f64);
	ctx.f[9].f64 = ((ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64;
	ctx.cr[0].compare_f64(ctx.f[9].f64, ctx.f[13].f64);
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[7].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	ctx.r[4].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	ctx.f[7].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[10].f64 } else { ctx.f[12].f64 };
	ctx.f[6].f64 = (((ctx.f[7].f64 - ctx.f[0].f64) as f32) as f64);
	ctx.cr[0].compare_f64(ctx.f[6].f64, ctx.f[13].f64);
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[3].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0x0000001Fu64;
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x00000003u64;
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	ctx.f[4].f64 = if ctx.f[5].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[7].f64 };
	ctx.f[3].s64 = if ctx.f[4].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[4].f64.trunc() as i32 as i64 };
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	if !ctx.cr[2200728828].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[11].s64 = ctx.r[1].s64 + 224;
	ctx.r[9].u64 = ctx.r[27].u64;
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7590; continue 'dispatch;
	}
	ctx.r[7].s64 = ctx.r[1].s64 + 224;
	ctx.r[7].s64 = ctx.r[20].s64 - ctx.r[7].s64;
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[6].u64 & 0xFFFFFFFF0000FFFF);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[6].u32) };
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	if !ctx.cr[2200728948].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x1FFFFFFFu64;
	ctx.r[1].s64 = ctx.r[1].s64 + 16720;
	ctx.f[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E38);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C75B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0DE0);
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.f[29].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.f[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.f[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[21].u64 = ctx.r[4].u64;
	ctx.r[19].u64 = ctx.r[5].u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 44100 as u32, &mut ctx.xer);
	if ctx.cr[0].lt {
			pc = 0x832C75D8; continue 'dispatch;
	}
	ctx.r[26].s64 = 2048;
	pc = 0x832C75EC; continue 'dispatch;
	ctx.r[11].s64 = 22050;
	ctx.xer.ca = ctx.r[3].u32 >= ctx.r[11].u32;
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	tmp.u8 = ((~ctx.r[10].u32).wrapping_add(ctx.r[10].u32) < (~ctx.r[10].u32)) as u8 | ((~ctx.r[10].u32).wrapping_add(ctx.r[10].u32).wrapping_add(ctx.xer.ca as u32) < ctx.xer.ca as u32) as u8;
	ctx.r[9].u64 = (!ctx.r[10].u64).wrapping_add(ctx.r[10].u64).wrapping_add(ctx.xer.ca as u64);
	ctx.xer.ca = (tmp.u8 != 0);
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[26].s64 = ctx.r[11].s64 + 1024;
	ctx.r[11].s64 = (ctx.r[26].s32 as i64) * (ctx.r[21].s32 as i64);
	ctx.r[10].u64 = ctx.r[19].u32 as u64 & 0x00000001u64;
	ctx.r[18].s64 = 1;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C7610; continue 'dispatch;
	}
	ctx.r[3].s64 = (ctx.r[3].s32 as i64) * (ctx.r[21].s32 as i64);
	ctx.r[26].u64 = ctx.r[11].u64;
	ctx.r[21].u64 = ctx.r[18].u64;
	ctx.r[11].s64 = -2091909120;
	ctx.r[9].s64 = ctx.r[3].s64 + 1;
	ctx.r[20].s64 = ctx.r[11].s64 + -15840;
	ctx.r[27].s64 = 0;
	ctx.r[11].s64 = ctx.r[20].s64 + 4;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C7698; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C767C; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C7684; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C768C; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C7694; continue 'dispatch;
	}
	ctx.r[27].s64 = ctx.r[27].s64 + 5;
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	ctx.cr[0].compare_u32(ctx.r[27].u32, 25 as u32, &mut ctx.xer);
	if ctx.cr[0].lt {
			pc = 0x832C762C; continue 'dispatch;
	}
	pc = 0x832C7698; continue 'dispatch;
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	pc = 0x832C7698; continue 'dispatch;
	ctx.r[27].s64 = ctx.r[27].s64 + 2;
	pc = 0x832C7698; continue 'dispatch;
	ctx.r[27].s64 = ctx.r[27].s64 + 3;
	pc = 0x832C7698; continue 'dispatch;
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	ctx.r[9].u64 = ctx.r[22].u64 & 0x00000000FFFFFFFFu64;
	ctx.r[11].s64 = -2113273856;
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[9].u64) };
	ctx.f[0].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	ctx.r[25].s64 = ctx.r[11].s64 + -19328;
	ctx.f[0].f64 = (ctx.f[13].f64 as f32) as f64;
	ctx.r[8].s64 = -2113929216;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	ctx.r[7].s64 = -2113929216;
	ctx.r[6].s64 = ctx.r[11].s64 + 175;
	ctx.r[28].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	ctx.f[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.cr[0].compare_f64(ctx.f[0].f64, ctx.f[29].f64);
	if !ctx.cr[0].gt {
			pc = 0x832C7718; continue 'dispatch;
	}
	ctx.f[12].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	ctx.f[11].f64 = -(ctx.f[12].f64 * ctx.f[13].f64 - ctx.f[31].f64);
	ctx.f[10].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	ctx.f[9].f64 = ctx.f[10].f64 * ctx.f[30].f64;
	ctx.f[8].f64 = ctx.f[0].f64 * ctx.f[9].f64;
	ctx.f[7].f64 = -(ctx.f[8].f64 * ctx.f[9].f64 - ctx.f[31].f64);
	ctx.f[6].f64 = ctx.f[7].f64 * ctx.f[9].f64;
	ctx.f[5].f64 = ctx.f[6].f64 * ctx.f[30].f64;
	ctx.f[4].f64 = ctx.f[0].f64 * ctx.f[5].f64;
	ctx.f[3].f64 = -(ctx.f[4].f64 * ctx.f[5].f64 - ctx.f[31].f64);
	ctx.f[2].f64 = ctx.f[3].f64 * ctx.f[5].f64;
	ctx.f[1].f64 = ctx.f[2].f64 * ctx.f[30].f64;
	ctx.f[0].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.f[0].u64) };
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	ctx.r[10].s64 = ctx.r[11].s64 + 23;
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	if ctx.cr[0].eq {
			pc = 0x832C7758; continue 'dispatch;
	}
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	ctx.r[10].s64 = ctx.r[11].s64 + 15;
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	pc = 0x832C7768; continue 'dispatch;
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[24].u64;
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.lr = 0x832C7778;
	crate::recompiler::externs::call(&mut ctx, base, 0x82BF1790);
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C7798; continue 'dispatch;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E30);
	return;
	ctx.r[5].s64 = 152;
	ctx.r[4].s64 = 0;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C77A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB13B0);
	ctx.r[11].u64 = ctx.r[26].u64 & 0x00000000FFFFFFFFu64;
	ctx.r[10].s64 = ctx.r[31].s64 + 160;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[11].u64) };
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	ctx.r[9].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	ctx.f[0].f64 = (ctx.f[13].f64 as f32) as f64;
	ctx.r[8].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	ctx.r[7].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[19].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[21].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[27].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[26].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[24].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	ctx.cr[0].compare_f64(ctx.f[0].f64, ctx.f[29].f64);
	if !ctx.cr[0].gt {
			pc = 0x832C7840; continue 'dispatch;
	}
	ctx.f[12].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	ctx.f[11].f64 = -(ctx.f[12].f64 * ctx.f[13].f64 - ctx.f[31].f64);
	ctx.f[10].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	ctx.f[9].f64 = ctx.f[10].f64 * ctx.f[30].f64;
	ctx.f[8].f64 = ctx.f[0].f64 * ctx.f[9].f64;
	ctx.f[7].f64 = -(ctx.f[8].f64 * ctx.f[9].f64 - ctx.f[31].f64);
	ctx.f[6].f64 = ctx.f[7].f64 * ctx.f[9].f64;
	ctx.f[5].f64 = ctx.f[6].f64 * ctx.f[30].f64;
	ctx.f[4].f64 = ctx.f[0].f64 * ctx.f[5].f64;
	ctx.f[3].f64 = -(ctx.f[4].f64 * ctx.f[5].f64 - ctx.f[31].f64);
	ctx.f[2].f64 = ctx.f[3].f64 * ctx.f[5].f64;
	ctx.f[1].f64 = ctx.f[2].f64 * ctx.f[30].f64;
	ctx.f[0].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	ctx.r[8].s64 = 0;
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7890; continue 'dispatch;
	}
	ctx.r[10].u64 = ctx.r[20].u64;
	ctx.r[11].s64 = ctx.r[31].s64 + 52;
	ctx.r[9].u64 = ctx.r[27].u64;
	ctx.r[8].u64 = ctx.r[27].u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].s64 = (ctx.r[22].s32 as i64) * (ctx.r[7].s32 as i64);
	ctx.r[5].u32 = ctx.r[6].u32 / ctx.r[23].u32;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[5].u32) };
	if !ctx.cr[2200729728].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[18].u32) };
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	if !ctx.cr[2200729704].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[11].s64 = ctx.r[8].s64 + 13;
	ctx.r[10].s64 = 0;
	ctx.r[3].u64 = ctx.r[31].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[22].u32) };
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[18].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	ctx.f[29].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E30);
	return;
}

pub fn sub_832C7528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832C7528 size=6160
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[6].f64 = (((ctx.f[7].f64 - ctx.f[0].f64) as f32) as f64);
	ctx.cr[0].compare_f64(ctx.f[6].f64, ctx.f[13].f64);
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[3].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[3].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0x0000001Fu64;
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x00000003u64;
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	ctx.f[4].f64 = if ctx.f[5].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[7].f64 };
	ctx.f[3].s64 = if ctx.f[4].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[4].f64.trunc() as i32 as i64 };
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	if !ctx.cr[2200728828].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[11].s64 = ctx.r[1].s64 + 224;
	ctx.r[9].u64 = ctx.r[27].u64;
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7590; continue 'dispatch;
	}
	ctx.r[7].s64 = ctx.r[1].s64 + 224;
	ctx.r[7].s64 = ctx.r[20].s64 - ctx.r[7].s64;
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[6].u64 & 0xFFFFFFFF0000FFFF);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[6].u32) };
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	if !ctx.cr[2200728948].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x1FFFFFFFu64;
	ctx.r[1].s64 = ctx.r[1].s64 + 16720;
	ctx.f[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E38);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C75B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0DE0);
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.f[29].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.f[30].u64) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.f[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[21].u64 = ctx.r[4].u64;
	ctx.r[19].u64 = ctx.r[5].u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 44100 as u32, &mut ctx.xer);
	if ctx.cr[0].lt {
			pc = 0x832C75D8; continue 'dispatch;
	}
	ctx.r[26].s64 = 2048;
	pc = 0x832C75EC; continue 'dispatch;
	ctx.r[11].s64 = 22050;
	ctx.xer.ca = ctx.r[3].u32 >= ctx.r[11].u32;
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	tmp.u8 = ((~ctx.r[10].u32).wrapping_add(ctx.r[10].u32) < (~ctx.r[10].u32)) as u8 | ((~ctx.r[10].u32).wrapping_add(ctx.r[10].u32).wrapping_add(ctx.xer.ca as u32) < ctx.xer.ca as u32) as u8;
	ctx.r[9].u64 = (!ctx.r[10].u64).wrapping_add(ctx.r[10].u64).wrapping_add(ctx.xer.ca as u64);
	ctx.xer.ca = (tmp.u8 != 0);
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[26].s64 = ctx.r[11].s64 + 1024;
	ctx.r[11].s64 = (ctx.r[26].s32 as i64) * (ctx.r[21].s32 as i64);
	ctx.r[10].u64 = ctx.r[19].u32 as u64 & 0x00000001u64;
	ctx.r[18].s64 = 1;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C7610; continue 'dispatch;
	}
	ctx.r[3].s64 = (ctx.r[3].s32 as i64) * (ctx.r[21].s32 as i64);
	ctx.r[26].u64 = ctx.r[11].u64;
	ctx.r[21].u64 = ctx.r[18].u64;
	ctx.r[11].s64 = -2091909120;
	ctx.r[9].s64 = ctx.r[3].s64 + 1;
	ctx.r[20].s64 = ctx.r[11].s64 + -15840;
	ctx.r[27].s64 = 0;
	ctx.r[11].s64 = ctx.r[20].s64 + 4;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C7698; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C767C; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C7684; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C768C; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C7694; continue 'dispatch;
	}
	ctx.r[27].s64 = ctx.r[27].s64 + 5;
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	ctx.cr[0].compare_u32(ctx.r[27].u32, 25 as u32, &mut ctx.xer);
	if ctx.cr[0].lt {
			pc = 0x832C762C; continue 'dispatch;
	}
	pc = 0x832C7698; continue 'dispatch;
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	pc = 0x832C7698; continue 'dispatch;
	ctx.r[27].s64 = ctx.r[27].s64 + 2;
	pc = 0x832C7698; continue 'dispatch;
	ctx.r[27].s64 = ctx.r[27].s64 + 3;
	pc = 0x832C7698; continue 'dispatch;
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	ctx.r[9].u64 = ctx.r[22].u64 & 0x00000000FFFFFFFFu64;
	ctx.r[11].s64 = -2113273856;
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[9].u64) };
	ctx.f[0].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	ctx.r[25].s64 = ctx.r[11].s64 + -19328;
	ctx.f[0].f64 = (ctx.f[13].f64 as f32) as f64;
	ctx.r[8].s64 = -2113929216;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	ctx.r[7].s64 = -2113929216;
	ctx.r[6].s64 = ctx.r[11].s64 + 175;
	ctx.r[28].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	ctx.f[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.cr[0].compare_f64(ctx.f[0].f64, ctx.f[29].f64);
	if !ctx.cr[0].gt {
			pc = 0x832C7718; continue 'dispatch;
	}
	ctx.f[12].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	ctx.f[11].f64 = -(ctx.f[12].f64 * ctx.f[13].f64 - ctx.f[31].f64);
	ctx.f[10].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	ctx.f[9].f64 = ctx.f[10].f64 * ctx.f[30].f64;
	ctx.f[8].f64 = ctx.f[0].f64 * ctx.f[9].f64;
	ctx.f[7].f64 = -(ctx.f[8].f64 * ctx.f[9].f64 - ctx.f[31].f64);
	ctx.f[6].f64 = ctx.f[7].f64 * ctx.f[9].f64;
	ctx.f[5].f64 = ctx.f[6].f64 * ctx.f[30].f64;
	ctx.f[4].f64 = ctx.f[0].f64 * ctx.f[5].f64;
	ctx.f[3].f64 = -(ctx.f[4].f64 * ctx.f[5].f64 - ctx.f[31].f64);
	ctx.f[2].f64 = ctx.f[3].f64 * ctx.f[5].f64;
	ctx.f[1].f64 = ctx.f[2].f64 * ctx.f[30].f64;
	ctx.f[0].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.f[0].u64) };
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	ctx.r[10].s64 = ctx.r[11].s64 + 23;
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	if ctx.cr[0].eq {
			pc = 0x832C7758; continue 'dispatch;
	}
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	ctx.r[10].s64 = ctx.r[11].s64 + 15;
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	pc = 0x832C7768; continue 'dispatch;
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[24].u64;
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.lr = 0x832C7778;
	crate::recompiler::externs::call(&mut ctx, base, 0x82BF1790);
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C7798; continue 'dispatch;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E30);
	return;
	ctx.r[5].s64 = 152;
	ctx.r[4].s64 = 0;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C77A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB13B0);
	ctx.r[11].u64 = ctx.r[26].u64 & 0x00000000FFFFFFFFu64;
	ctx.r[10].s64 = ctx.r[31].s64 + 160;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[11].u64) };
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	ctx.r[9].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	ctx.f[0].f64 = (ctx.f[13].f64 as f32) as f64;
	ctx.r[8].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	ctx.r[7].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[19].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[21].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[27].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[26].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[24].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	ctx.cr[0].compare_f64(ctx.f[0].f64, ctx.f[29].f64);
	if !ctx.cr[0].gt {
			pc = 0x832C7840; continue 'dispatch;
	}
	ctx.f[12].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	ctx.f[11].f64 = -(ctx.f[12].f64 * ctx.f[13].f64 - ctx.f[31].f64);
	ctx.f[10].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	ctx.f[9].f64 = ctx.f[10].f64 * ctx.f[30].f64;
	ctx.f[8].f64 = ctx.f[0].f64 * ctx.f[9].f64;
	ctx.f[7].f64 = -(ctx.f[8].f64 * ctx.f[9].f64 - ctx.f[31].f64);
	ctx.f[6].f64 = ctx.f[7].f64 * ctx.f[9].f64;
	ctx.f[5].f64 = ctx.f[6].f64 * ctx.f[30].f64;
	ctx.f[4].f64 = ctx.f[0].f64 * ctx.f[5].f64;
	ctx.f[3].f64 = -(ctx.f[4].f64 * ctx.f[5].f64 - ctx.f[31].f64);
	ctx.f[2].f64 = ctx.f[3].f64 * ctx.f[5].f64;
	ctx.f[1].f64 = ctx.f[2].f64 * ctx.f[30].f64;
	ctx.f[0].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	ctx.r[8].s64 = 0;
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7890; continue 'dispatch;
	}
	ctx.r[10].u64 = ctx.r[20].u64;
	ctx.r[11].s64 = ctx.r[31].s64 + 52;
	ctx.r[9].u64 = ctx.r[27].u64;
	ctx.r[8].u64 = ctx.r[27].u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].s64 = (ctx.r[22].s32 as i64) * (ctx.r[7].s32 as i64);
	ctx.r[5].u32 = ctx.r[6].u32 / ctx.r[23].u32;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[5].u32) };
	if !ctx.cr[2200729728].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[18].u32) };
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	if !ctx.cr[2200729704].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[11].s64 = ctx.r[8].s64 + 13;
	ctx.r[10].s64 = 0;
	ctx.r[3].u64 = ctx.r[31].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[22].u32) };
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[18].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	ctx.f[29].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	ctx.f[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E30);
	return;
}

pub fn sub_832C7A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C7A40 size=2064
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = 0;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	ctx.lr = 0x832C7A54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCF338);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	ctx.r[7].s64 = 0;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	ctx.r[5].u64 = ctx.r[29].u64;
	ctx.r[4].u64 = ctx.r[28].u64;
	ctx.lr = 0x832C7A70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCB088);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].eq {
			pc = 0x832C7A84; continue 'dispatch;
	}
	ctx.r[11].s64 = 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	if ctx.cr[0].lt {
			pc = 0x832C7AB4; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7AD4; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.ctr.u64 = ctx.r[10].u64;
	ctx.lr = 0x832C7AD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[3].s64 + 3;
	ctx.r[10].u64 = ctx.r[28].u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7AFC; continue 'dispatch;
	}
	ctx.r[9].u64 = (unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32) }).swap_bytes() as u64;
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	if !ctx.cr[2200730344].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
}

pub fn sub_832C7BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C7BC8 size=4112
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C7BD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0DF0);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.r[28].u64 = ctx.r[6].u64;
	ctx.r[25].s64 = 0;
	ctx.r[30].u64 = ctx.r[5].u64;
	ctx.r[27].u64 = ctx.r[7].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[22].u64 = ctx.r[28].u64;
	ctx.r[23].u64 = ctx.r[25].u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7C10; continue 'dispatch;
	}
	ctx.r[3].s64 = 0;
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E40);
	return;
	ctx.lr = 0x832C7C14;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[24].u64 = ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7CF4; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].eq {
			pc = 0x832C7CF4; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[23].s64 = 1;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7C48; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C7C48;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].gt {
			pc = 0x832C7CB8; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].gt {
			pc = 0x832C7CB8; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].gt {
			pc = 0x832C7CF4; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	pc = 0x832C7CF4; continue 'dispatch;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = 0;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	ctx.lr = 0x832C7CD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCF338);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[25].u32) };
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[26].s64 = ctx.r[31].s64 + 76;
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7E00; continue 'dispatch;
	}
	if !ctx.cr[0].gt {
			pc = 0x832C7D10; continue 'dispatch;
	}
	ctx.r[30].u64 = ctx.r[27].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[27].s64 = ctx.r[27].s64 - ctx.r[30].s64;
	ctx.r[25].u64 = ctx.r[30].u64 + ctx.r[25].u64;
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	if ctx.cr[0].gt {
			pc = 0x832C7DA0; continue 'dispatch;
	}
	ctx.r[5].u64 = ctx.r[29].u64;
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = ctx.r[28].u64;
	ctx.lr = 0x832C7D48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E80);
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[28].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	ctx.r[8].s64 = -ctx.r[29].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	ctx.r[9].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[26].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	let addr = ctx.r[26].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200730972].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[6].s64 = ctx.r[31].s64 + 100;
	ctx.r[3].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[6].u32) as usize) as *const u32) };
	ctx.r[5].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[4].u64 = ctx.r[29].u64 + ctx.r[5].u64;
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200731004].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7E00; continue 'dispatch;
	}
	ctx.r[5].u64 = ctx.r[30].u64;
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = ctx.r[28].u64;
	ctx.lr = 0x832C7DB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E80);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[28].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	ctx.r[6].s64 = -ctx.r[30].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	ctx.r[8].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[26].u32) as usize) as *const u32) };
	ctx.r[10].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[9].u64 = ctx.r[6].u64 + ctx.r[10].u64;
	let addr = ctx.r[26].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200731076].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[5].s64 = ctx.r[31].s64 + 100;
	ctx.r[11].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[5].u32) as usize) as *const u32) };
	ctx.r[4].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[4].u64;
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[11].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200731108].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.cr[0].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7ECC; continue 'dispatch;
	}
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C7E30; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[23].s64 = 1;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7CF8; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C7E2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	pc = 0x832C7CF8; continue 'dispatch;
	ctx.lr = 0x832C7E34;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	ctx.r[5].u64 = ctx.r[27].u64;
	ctx.r[4].u64 = ctx.r[28].u64;
	ctx.r[30].u64 = ctx.r[3].u64;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832C7E50;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCB088);
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C7E64; continue 'dispatch;
	}
	ctx.r[11].s64 = 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[25].u64 = ctx.r[4].u64 + ctx.r[25].u64;
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7EA4; continue 'dispatch;
	}
	ctx.r[5].u64 = ctx.r[30].u64;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C7EA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C7B08);
	ctx.lr = 0x832C7EA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[30].s64;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[24].s64;
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	pc = 0x832C7EE0; continue 'dispatch;
	ctx.lr = 0x832C7ED0;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C84B8);
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[24].s64;
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	if ctx.cr[0].lt {
			pc = 0x832C7F00; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[11].s64 + 131072;
	if !ctx.cr[0].gt {
			pc = 0x832C7F20; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7F40; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7F40; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C7F40;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[11].s64 = ctx.r[25].s64 + 3;
	ctx.r[10].u64 = ctx.r[22].u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7F68; continue 'dispatch;
	}
	ctx.r[9].u64 = (unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32) }).swap_bytes() as u64;
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	if !ctx.cr[2200731476].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[3].u64 = ctx.r[25].u64;
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E40);
	return;
}

pub fn sub_832C7D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C7D98 size=8
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			crate::recompiler::externs::call(&mut ctx, base, 0x832C7E00);
		return;
	}
}

pub fn sub_832C7F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C7F98 size=7176
	ctx.r[3].u64 = ctx.r[11].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C7FA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E04);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.r[30].u64 = ctx.r[4].u64;
	ctx.r[29].u64 = ctx.r[5].u64;
	ctx.r[28].u64 = ctx.r[6].u64;
	ctx.r[27].u64 = ctx.r[7].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C7FD4; continue 'dispatch;
	}
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C7FD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[11].s64 = ctx.r[30].s64 + 4095;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[8].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.cr[0].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[27].u32) };
	if ctx.cr[0].eq {
			pc = 0x832C8024; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.ctr.u64 = ctx.r[9].u64;
	ctx.lr = 0x832C8024;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E54);
	return;
}

pub fn sub_832C82B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C82B8 size=6152
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C82F8; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C82D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E48);
	return;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C82F4; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C82F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[3].s64 = -1;
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E48);
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C8374; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C8334; continue 'dispatch;
	}
	ctx.r[11].s64 = 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C83A8; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C8358; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C8358;
	crate::rt::call_indirect(ctx.ctr.u32);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C83A8; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.ctr.u64 = ctx.r[11].u64;
	ctx.lr = 0x832C8370;
	crate::rt::call_indirect(ctx.ctr.u32);
	pc = 0x832C83A8; continue 'dispatch;
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C83A8; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C8394; continue 'dispatch;
	}
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C83A8; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832C83A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C8098);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C83C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E0C);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[30].u64 = ctx.r[4].u64;
	ctx.r[29].u64 = ctx.r[5].u64;
	ctx.r[5].s64 = 324;
	ctx.r[4].s64 = 0;
	ctx.r[31].u64 = ctx.r[3].u64;
	ctx.lr = 0x832C83E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB13B0);
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C8474; continue 'dispatch;
	}
	ctx.r[11].s64 = 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	ctx.r[6].s64 = 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[5].s64 = 0;
	ctx.r[4].s64 = 0;
	ctx.r[3].u64 = ctx.r[30].u64;
	ctx.lr = 0x832C8410;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCF338);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[11].s64 = -2094268416;
	ctx.r[10].s64 = -2094268416;
	ctx.r[9].s64 = -2094268416;
	ctx.r[8].s64 = -2094268416;
	ctx.r[7].s64 = -2094268416;
	ctx.r[4].s64 = ctx.r[11].s64 + -8;
	ctx.r[6].s64 = -2094268416;
	ctx.r[5].s64 = -2094268416;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[4].u32) };
	ctx.r[3].s64 = ctx.r[10].s64 + 464;
	ctx.r[11].s64 = ctx.r[9].s64 + 1400;
	ctx.r[10].s64 = ctx.r[8].s64 + 1440;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[9].s64 = ctx.r[7].s64 + 1688;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[8].s64 = ctx.r[6].s64 + 1584;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[7].s64 = ctx.r[5].s64 + 2304;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	ctx.r[3].s64 = 1;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
	ctx.r[8].s64 = 134217728;
	ctx.r[9].s64 = 0;
	ctx.r[8].u64 = ctx.r[8].u64 | 128;
	ctx.r[7].s64 = 3;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = 1;
	ctx.r[4].s64 = -2147483648;
	ctx.r[3].u64 = ctx.r[30].u64;
	ctx.lr = 0x832C8498;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CCAE90);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C8414; continue 'dispatch;
	}
	ctx.r[3].s64 = 0;
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E5C);
	return;
}

pub fn sub_832C8630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C8630 size=6152
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	if !ctx.cr[0].eq {
			pc = 0x832C8660; continue 'dispatch;
	}
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[11].u8) };
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	if ctx.cr[0].lt {
			pc = 0x832C863C; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E54);
	return;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C867C; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	ctx.r[11].s64 = 31;
	pc = 0x832C8688; continue 'dispatch;
	ctx.r[9].u64 = ctx.r[30].u64;
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C8CD8; continue 'dispatch;
	}
	ctx.cr[0].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C86C0; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 2 as u32;
	ctx.r[7].s64 = (2 as i64) - ctx.r[11].s64;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[5].u64 = ctx.r[6].u64 | ctx.r[10].u64;
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[9].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[29].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	ctx.r[31].s64 = ctx.r[11].s64 + 30;
	pc = 0x832C86CC; continue 'dispatch;
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.r[31].s64 = ctx.r[11].s64 + -2;
	ctx.cr[0].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	ctx.r[11].s64 = 1;
	if !ctx.cr[0].eq {
			pc = 0x832C874C; continue 'dispatch;
	}
	ctx.r[9].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C86F8; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	ctx.r[31].s64 = 31;
	pc = 0x832C8704; continue 'dispatch;
	ctx.r[10].u64 = ctx.r[30].u64;
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	ctx.r[10].s64 = ctx.r[11].s64 + 255;
	if ctx.cr[0].eq {
			pc = 0x832C8720; continue 'dispatch;
	}
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[11].u8) };
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[10].u8) };
	pc = 0x832C8728; continue 'dispatch;
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[11].u8) };
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 16 as u32, &mut ctx.xer);
	if ctx.cr[0].lt {
			pc = 0x832C86DC; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E54);
	return;
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	ctx.r[9].s64 = 0;
	ctx.r[7].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C8774; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	ctx.r[31].s64 = 31;
	pc = 0x832C8780; continue 'dispatch;
	ctx.r[10].u64 = ctx.r[30].u64;
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	if ctx.cr[0].eq {
			pc = 0x832C879C; continue 'dispatch;
	}
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[11].u8) };
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u8) };
	pc = 0x832C87A4; continue 'dispatch;
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 17 as u32, &mut ctx.xer);
	if ctx.cr[0].lt {
			pc = 0x832C8758; continue 'dispatch;
	}
	ctx.cr[0].compare_u32(ctx.r[29].u32, 1 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C8960; continue 'dispatch;
	}
	ctx.cr[0].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C87D8; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	pc = 0x832C87DC; continue 'dispatch;
	ctx.r[3].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	ctx.r[7].s64 = 2;
	ctx.r[6].s64 = ctx.r[1].s64 + 82;
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	ctx.r[4].u64 = ctx.r[28].u64;
	ctx.lr = 0x832C87F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C8518);
	if !ctx.cr[0].lt {
			pc = 0x832C8814; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	pc = 0x832C881C; continue 'dispatch;
	if (ctx.r[3].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[3].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C8838; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	pc = 0x832C883C; continue 'dispatch;
	ctx.r[3].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	ctx.r[7].s64 = 2;
	ctx.r[6].s64 = ctx.r[1].s64 + 86;
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	ctx.r[4].s64 = ctx.r[28].s64 + 4;
	ctx.lr = 0x832C8850;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C8518);
	if !ctx.cr[0].lt {
			pc = 0x832C8874; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	pc = 0x832C887C; continue 'dispatch;
	if (ctx.r[3].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[3].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C8898; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	pc = 0x832C889C; continue 'dispatch;
	ctx.r[3].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	ctx.r[7].s64 = 2;
	ctx.r[6].s64 = ctx.r[1].s64 + 90;
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	ctx.r[4].s64 = ctx.r[28].s64 + 8;
	ctx.lr = 0x832C88B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C8518);
	if !ctx.cr[0].lt {
			pc = 0x832C88D4; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	pc = 0x832C88DC; continue 'dispatch;
	if (ctx.r[3].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[3].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C88F8; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	pc = 0x832C88FC; continue 'dispatch;
	ctx.r[3].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	ctx.r[7].s64 = 2;
	ctx.r[6].s64 = ctx.r[1].s64 + 94;
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	ctx.r[4].s64 = ctx.r[28].s64 + 12;
	ctx.lr = 0x832C8910;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C8518);
	if !ctx.cr[0].lt {
			pc = 0x832C8944; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E54);
	return;
	if (ctx.r[3].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[3].u8 & 0x1F) as u32)) as u64;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E54);
	return;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C897C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	pc = 0x832C8980; continue 'dispatch;
	ctx.r[3].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	ctx.r[7].s64 = 2;
	ctx.r[6].s64 = ctx.r[1].s64 + 82;
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	ctx.lr = 0x832C8994;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C8518);
	if !ctx.cr[0].lt {
			pc = 0x832C89B8; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	pc = 0x832C89C0; continue 'dispatch;
	if (ctx.r[3].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[3].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C89DC; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	pc = 0x832C89E0; continue 'dispatch;
	ctx.r[3].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	ctx.r[7].s64 = 2;
	ctx.r[6].s64 = ctx.r[1].s64 + 86;
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	ctx.lr = 0x832C89F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C8518);
	if !ctx.cr[0].lt {
			pc = 0x832C8A18; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	pc = 0x832C8A20; continue 'dispatch;
	if (ctx.r[3].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[3].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C8A3C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	pc = 0x832C8A40; continue 'dispatch;
	ctx.r[3].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	ctx.r[7].s64 = 2;
	ctx.r[6].s64 = ctx.r[1].s64 + 90;
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	ctx.lr = 0x832C8A54;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C8518);
	if !ctx.cr[0].lt {
			pc = 0x832C8A78; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	pc = 0x832C8A80; continue 'dispatch;
	if (ctx.r[3].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[3].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C8A9C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	pc = 0x832C8AA0; continue 'dispatch;
	ctx.r[3].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	ctx.r[7].s64 = 2;
	ctx.r[6].s64 = ctx.r[1].s64 + 94;
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	ctx.r[4].s64 = ctx.r[1].s64 + 108;
	ctx.lr = 0x832C8AB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C8518);
	if !ctx.cr[0].lt {
			pc = 0x832C8AD8; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	pc = 0x832C8AE0; continue 'dispatch;
	if (ctx.r[3].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[3].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_u32(ctx.r[29].u32, 2 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C8BB0; continue 'dispatch;
	}
	ctx.cr[0].compare_u32(ctx.r[31].u32, 7 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C8B04; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x0000007Fu64;
	pc = 0x832C8B08; continue 'dispatch;
	ctx.r[3].u64 = ctx.r[30].u32 as u64 & 0x0000007Fu64;
	ctx.r[7].s64 = 4;
	ctx.r[6].s64 = ctx.r[1].s64 + 100;
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	ctx.r[4].u64 = ctx.r[28].u64;
	ctx.lr = 0x832C8B1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C8518);
	if !ctx.cr[0].lt {
			pc = 0x832C8B40; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	pc = 0x832C8B48; continue 'dispatch;
	if (ctx.r[3].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[3].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 7 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C8B64; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x0000007Fu64;
	pc = 0x832C8B68; continue 'dispatch;
	ctx.r[3].u64 = ctx.r[30].u32 as u64 & 0x0000007Fu64;
	ctx.r[7].s64 = 4;
	ctx.r[6].s64 = ctx.r[1].s64 + 108;
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	ctx.r[4].s64 = ctx.r[28].s64 + 8;
	ctx.lr = 0x832C8B7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C8518);
	if !ctx.cr[0].lt {
			pc = 0x832C8944; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E54);
	return;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 7 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C8BCC; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x0000007Fu64;
	pc = 0x832C8BD0; continue 'dispatch;
	ctx.r[3].u64 = ctx.r[30].u32 as u64 & 0x0000007Fu64;
	ctx.r[7].s64 = 4;
	ctx.r[6].s64 = ctx.r[1].s64 + 100;
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	ctx.lr = 0x832C8BE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C8518);
	if !ctx.cr[0].lt {
			pc = 0x832C8C08; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	pc = 0x832C8C10; continue 'dispatch;
	if (ctx.r[3].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[3].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 7 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C8C2C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x0000007Fu64;
	pc = 0x832C8C30; continue 'dispatch;
	ctx.r[3].u64 = ctx.r[30].u32 as u64 & 0x0000007Fu64;
	ctx.r[7].s64 = 4;
	ctx.r[6].s64 = ctx.r[1].s64 + 108;
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	ctx.lr = 0x832C8C44;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C8518);
	if !ctx.cr[0].lt {
			pc = 0x832C8C68; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	pc = 0x832C8C70; continue 'dispatch;
	if (ctx.r[3].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[3].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 15 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C8C8C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x00007FFFu64;
	pc = 0x832C8C90; continue 'dispatch;
	ctx.r[3].u64 = ctx.r[30].u32 as u64 & 0x00007FFFu64;
	ctx.r[7].s64 = 8;
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	ctx.r[4].u64 = ctx.r[28].u64;
	ctx.lr = 0x832C8CA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C8518);
	if !ctx.cr[0].lt {
			pc = 0x832C8944; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E54);
	return;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C8D04; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 3 as u32;
	ctx.r[7].s64 = (3 as i64) - ctx.r[11].s64;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[5].u64 = ctx.r[6].u64 | ctx.r[10].u64;
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[9].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0x00000007u64;
	ctx.r[31].s64 = ctx.r[11].s64 + 29;
	pc = 0x832C8D10; continue 'dispatch;
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	ctx.r[31].s64 = ctx.r[11].s64 + -3;
	ctx.r[7].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[7].u64 = ctx.r[7].u64 | 65535;
	ctx.r[6].s64 = 1;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C8D4C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[31].u32 <= 4 as u32;
	ctx.r[5].s64 = (4 as i64) - ctx.r[31].s64;
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[30].u64;
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[11].u32) >> ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	pc = 0x832C8D58; continue 'dispatch;
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x0000000Fu64;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32), ctx.r[11].u8) };
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[6].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	ctx.r[7].u64 = ctx.r[7].u64 & !ctx.r[4].u64;
	if !ctx.cr[0].gt {
			pc = 0x832C8D20; continue 'dispatch;
	}
	ctx.r[11].s64 = 0;
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C8D90; continue 'dispatch;
	}
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	ctx.cr[0].compare_u32(ctx.r[9].u32, 15 as u32, &mut ctx.xer);
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), ctx.r[11].u8) };
	if ctx.cr[0].eq {
			pc = 0x832C8DA0; continue 'dispatch;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C8D74; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E54);
	return;
}

pub fn sub_832C89A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C89A8 size=8
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
}

pub fn sub_832C9488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C9488 size=7176
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0x0000000Fu64;
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[30].u32) >> ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[28].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[5].s64;
	pc = 0x832C9518; continue 'dispatch;
	if !ctx.cr[0].lt {
			pc = 0x832C95E0; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = ctx.r[5].s64 + 80;
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[7].s64 = (32 as i64) - ctx.r[11].s64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[10].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[9].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[30].u64;
	ctx.r[7].u64 = ctx.r[7].u64 & ctx.r[4].u64;
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	ctx.r[7].u64 = ctx.r[5].u32 as u64 & 0x0000000Fu64;
	ctx.r[4].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	ctx.r[28].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	if ctx.cr[0].lt {
			pc = 0x832C9504; continue 'dispatch;
	}
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[30].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	pc = 0x832C9518; continue 'dispatch;
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	ctx.r[5].u64 = ctx.r[28].u64;
	ctx.xer.ca = ctx.r[26].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[26].s64;
	if ctx.cr[0].lt {
			pc = 0x832C954C; continue 'dispatch;
	}
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[4].u64 = ctx.r[7].u64 & ctx.r[10].u64;
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000000Fu64;
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[4].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	ctx.r[31].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	pc = 0x832C9598; continue 'dispatch;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[10].u64;
	ctx.r[8].u64 = ctx.r[8].u64 & ctx.r[4].u64;
	ctx.r[4].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	ctx.r[31].u64 = ctx.r[4].u32 as u64 & 0x0000000Fu64;
	ctx.r[4].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	if ctx.cr[0].lt {
			pc = 0x832C9584; continue 'dispatch;
	}
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	pc = 0x832C9598; continue 'dispatch;
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[7].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[10].u8) };
	ctx.r[21].s64 = ctx.r[21].s64 + 1;
	if ctx.cr[0].gt {
			crate::recompiler::externs::call(&mut ctx, base, 0x832C9458);
		return;
	}
	ctx.cr[0].compare_i32(ctx.r[23].s32, -22, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C95C8; continue 'dispatch;
	}
	ctx.xer.ca = ctx.r[23].u32 <= -21 as u32;
	ctx.r[5].s64 = (-21 as i64) - ctx.r[23].s64;
	ctx.r[4].u64 = unsafe { crate::rt::load_u8(base as *const u8, 0u32) } as u64;
	ctx.r[3].u64 = ctx.r[22].u64;
	ctx.lr = 0x832C95C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB13B0);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E38);
	return;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E38);
	return;
}

pub fn sub_832C99A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C99A8 size=5132
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	if ctx.cr[0].lt {
			pc = 0x832C99CC; continue 'dispatch;
	}
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	pc = 0x832C99E0; continue 'dispatch;
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[7].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	ctx.cr[0].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[27].u64;
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[8].u8) };
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	if !ctx.cr[0].eq {
			crate::recompiler::externs::call(&mut ctx, base, 0x832C98E4);
		return;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E4C);
	return;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = ctx.r[9].s64 + 4;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E4C);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C9A30;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0DFC);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[27].u64 = ctx.r[4].u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[26].u64 = ctx.r[5].u64;
	if !ctx.cr[0].eq {
			pc = 0x832C9CB0; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = -1;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C9A94; continue 'dispatch;
	}
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[6].s64 = (32 as i64) - ctx.r[11].s64;
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = ((ctx.r[7].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.r[9].u64 = ctx.r[5].u64 | ctx.r[9].u64;
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[8].u32) >> ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	if (ctx.r[4].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[7].u32) >> ((ctx.r[4].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	pc = 0x832C9AA4; continue 'dispatch;
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[7].s64 = (32 as i64) - ctx.r[11].s64;
	ctx.r[31].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[8].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[9].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[5].u64 = ctx.r[6].u64 & ctx.r[9].u64;
	ctx.cr[0].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9C94; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].gt {
			pc = 0x832C9C94; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	if !ctx.cr[0].eq {
			pc = 0x832C9AE8; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	ctx.r[31].s64 = 31;
	pc = 0x832C9AF4; continue 'dispatch;
	ctx.r[11].u64 = ctx.r[30].u64;
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C9C00; continue 'dispatch;
	}
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[28].u64 = ctx.r[10].u64;
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].s64 = ctx.r[3].s64 + 16;
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	if ctx.cr[0].lt {
			pc = 0x832C9B44; continue 'dispatch;
	}
	ctx.xer.ca = ctx.r[7].u32 <= 32 as u32;
	ctx.r[11].s64 = (32 as i64) - ctx.r[7].s64;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[10].u64 & ctx.r[30].u64;
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	pc = 0x832C9B9C; continue 'dispatch;
	if !ctx.cr[0].lt {
			pc = 0x832C9C94; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[7].u32 <= 32 as u32;
	ctx.r[11].s64 = (32 as i64) - ctx.r[7].s64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[10].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[30].u64;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[8].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	ctx.r[25].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	if ctx.cr[0].lt {
			pc = 0x832C9B88; continue 'dispatch;
	}
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	pc = 0x832C9B9C; continue 'dispatch;
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	if (ctx.r[30].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[30].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9BDC; continue 'dispatch;
	}
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C9BC0; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	ctx.r[31].s64 = 31;
	pc = 0x832C9BCC; continue 'dispatch;
	ctx.r[11].u64 = ctx.r[30].u64;
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9BDC; continue 'dispatch;
	}
	ctx.r[9].s64 = -ctx.r[9].s64;
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[9].u8) };
	ctx.cr[0].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	if !ctx.cr[0].eq {
			pc = 0x832C9B10; continue 'dispatch;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E4C);
	return;
	ctx.cr[0].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C9C2C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[31].u32 <= 4 as u32;
	ctx.r[9].s64 = (4 as i64) - ctx.r[31].s64;
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[7].u64 = ctx.r[8].u64 | ctx.r[30].u64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[11].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[4].u64 = ctx.r[7].u32 as u64 & 0x0000000Fu64;
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	pc = 0x832C9C38; continue 'dispatch;
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000000Fu64;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9C78; continue 'dispatch;
	}
	ctx.cr[0].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C9C5C; continue 'dispatch;
	}
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	ctx.r[31].s64 = 31;
	pc = 0x832C9C68; continue 'dispatch;
	ctx.r[11].u64 = ctx.r[30].u64;
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9C78; continue 'dispatch;
	}
	ctx.r[4].s64 = -ctx.r[4].s64;
	ctx.r[3].u64 = ctx.r[10].u64;
	ctx.lr = 0x832C9C80;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB13B0);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E4C);
	return;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E4C);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832C9CC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0DF8);
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].eq {
			pc = 0x832CA010; continue 'dispatch;
	}
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[25].s64 = -1;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if !ctx.cr[0].lt {
			pc = 0x832C9D1C; continue 'dispatch;
	}
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[29].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[6].s64 = (32 as i64) - ctx.r[10].s64;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[31].u64 = 0;
	} else {
		ctx.r[31].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.r[10].u64 = ctx.r[31].u64 | ctx.r[9].u64;
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[25].u32) >> ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[30].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	if (ctx.r[29].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) >> ((ctx.r[29].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	pc = 0x832C9D30; continue 'dispatch;
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[10].s64;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[25].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[30].u64 = ctx.r[6].u64 & ctx.r[9].u64;
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	ctx.cr[0].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9FF4; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].gt {
			pc = 0x832C9FF4; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	if ctx.cr[0].eq {
			pc = 0x832C9DF8; continue 'dispatch;
	}
	ctx.r[8].s64 = ctx.r[9].s64 + -1;
	if !ctx.cr[0].lt {
			pc = 0x832C9D9C; continue 'dispatch;
	}
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[9].u32 <= 33 as u32;
	ctx.r[29].s64 = (33 as i64) - ctx.r[9].s64;
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[28].u64 = 0;
	} else {
		ctx.r[28].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[10].u64 = ctx.r[28].u64 | ctx.r[10].u64;
	if (ctx.r[29].u8 & 0x20) != 0 {
		ctx.r[29].u64 = 0;
	} else {
		ctx.r[29].u64 = ((ctx.r[25].u32) >> ((ctx.r[29].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	ctx.r[27].u64 = ctx.r[29].u64 & ctx.r[10].u64;
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 33;
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	pc = 0x832C9DB4; continue 'dispatch;
	ctx.xer.ca = ctx.r[9].u32 <= 33 as u32;
	ctx.r[31].s64 = (33 as i64) - ctx.r[9].s64;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[25].u32) >> ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[27].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9E44; continue 'dispatch;
	}
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C9DD8; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	ctx.r[11].s64 = 31;
	pc = 0x832C9DE4; continue 'dispatch;
	ctx.r[9].u64 = ctx.r[10].u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9E44; continue 'dispatch;
	}
	ctx.r[27].s64 = -ctx.r[27].s64;
	pc = 0x832C9E44; continue 'dispatch;
	if !ctx.cr[0].lt {
			pc = 0x832C9E30; continue 'dispatch;
	}
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[9].u32 <= 32 as u32;
	ctx.r[31].s64 = (32 as i64) - ctx.r[9].s64;
	ctx.r[29].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[28].u64 = 0;
	} else {
		ctx.r[28].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[10].u64 = ctx.r[28].u64 | ctx.r[10].u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[31].u64 = 0;
	} else {
		ctx.r[31].u64 = ((ctx.r[25].u32) >> ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	ctx.r[27].u64 = ctx.r[31].u64 & ctx.r[10].u64;
	if (ctx.r[29].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) >> ((ctx.r[29].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	pc = 0x832C9E44; continue 'dispatch;
	ctx.xer.ca = ctx.r[9].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[9].s64;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[25].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[27].u64 = ctx.r[8].u64 & ctx.r[10].u64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	ctx.r[31].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[24].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[31].u16) };
	ctx.r[28].s64 = ctx.r[6].s64 + 2;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	if ctx.cr[2200739844].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.cr[0].compare_u32(ctx.r[24].u32, 8 as u32, &mut ctx.xer);
	ctx.r[26].s64 = 8;
	if ctx.cr[0].gt {
			pc = 0x832C9E78; continue 'dispatch;
	}
	ctx.r[26].u64 = ctx.r[24].u64;
	ctx.cr[0].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	if !ctx.cr[0].lt {
			pc = 0x832C9EAC; continue 'dispatch;
	}
	if !ctx.cr[0].lt {
			pc = 0x832C9FF4; continue 'dispatch;
	}
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.xer.ca = ctx.r[11].u32 <= 4 as u32;
	ctx.r[8].s64 = (4 as i64) - ctx.r[11].s64;
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[10].u64;
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0x0000000Fu64;
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	pc = 0x832C9EB8; continue 'dispatch;
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	ctx.cr[0].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9F70; continue 'dispatch;
	}
	ctx.r[29].u64 = ctx.r[26].u64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9FDC; continue 'dispatch;
	}
	ctx.xer.ca = ctx.r[9].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[9].s64;
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[25].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	if !ctx.cr[0].lt {
			pc = 0x832C9F08; continue 'dispatch;
	}
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[31].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[6].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[6].u32) >> ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[6].u64 = ctx.r[11].u64 & ctx.r[30].u64;
	ctx.r[11].s64 = ctx.r[8].s64 + 32;
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	pc = 0x832C9F14; continue 'dispatch;
	ctx.r[6].u64 = ctx.r[30].u64 & ctx.r[10].u64;
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9F54; continue 'dispatch;
	}
	ctx.cr[0].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	if !ctx.cr[0].eq {
			pc = 0x832C9F38; continue 'dispatch;
	}
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	ctx.r[11].s64 = 31;
	pc = 0x832C9F44; continue 'dispatch;
	ctx.r[8].u64 = ctx.r[10].u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9F54; continue 'dispatch;
	}
	ctx.r[6].s64 = -ctx.r[6].s64;
	ctx.r[27].u64 = ctx.r[6].u64 + ctx.r[27].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	ctx.r[31].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[31].u16) };
	ctx.r[28].s64 = ctx.r[28].s64 + 2;
	if !ctx.cr[0].eq {
			pc = 0x832C9ED4; continue 'dispatch;
	}
	pc = 0x832C9FDC; continue 'dispatch;
	ctx.r[9].u64 = ctx.r[31].u32 as u64 & 0x0000FFFFu64;
	ctx.r[6].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.r[29].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	ctx.r[30].u64 = ctx.r[26].u64;
	ctx.r[9].u64 = ctx.r[28].u64;
	ctx.cr[0].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9F9C; continue 'dispatch;
	}
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[31].u16) };
	ctx.r[9].s64 = ctx.r[28].s64 + 2;
	ctx.r[30].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9FC4; continue 'dispatch;
	}
	ctx.r[8].u64 = ctx.r[9].u64;
	ctx.ctr.u64 = ctx.r[6].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x832C9FB0; continue 'dispatch;
	}
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	ctx.r[8].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832C9FD4; continue 'dispatch;
	}
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[29].u16) };
	ctx.r[28].u64 = ctx.r[9].u64 + ctx.r[28].u64;
	ctx.r[24].s64 = ctx.r[24].s64 - ctx.r[26].s64;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	if !ctx.cr[2200739432].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E48);
	return;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = ctx.r[9].s64 + 4;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E48);
	return;
}

