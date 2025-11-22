pub fn sub_824FB848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB848 size=8
    let mut pc: u32 = 0x824FB848;
    'dispatch: loop {
        match pc {
            0x824FB848 => {
    //   block [0x824FB848..0x824FB850)
	// 824FB848: 38630050  addi r3, r3, 0x50
	ctx.r[3].s64 = ctx.r[3].s64 + 80;
	// 824FB84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FB850 size=44
    let mut pc: u32 = 0x824FB850;
    'dispatch: loop {
        match pc {
            0x824FB850 => {
    //   block [0x824FB850..0x824FB87C)
	// 824FB850: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FB854: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824FB858: C1AB0000  lfs f13, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FB85C: D1A40000  stfs f13, 0(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824FB860: C1AB0010  lfs f13, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FB864: D1A40004  stfs f13, 4(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824FB868: C1AB0020  lfs f13, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FB86C: C00A1FF8  lfs f0, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB870: D1A40008  stfs f13, 8(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824FB874: D004000C  stfs f0, 0xc(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824FB878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FB880 size=400
    let mut pc: u32 = 0x824FB880;
    'dispatch: loop {
        match pc {
            0x824FB880 => {
    //   block [0x824FB880..0x824FBA10)
	// 824FB880: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824FB884: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824FB888: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 824FB88C: 810A004C  lwz r8, 0x4c(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 824FB890: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FB894: 38A8FFFF  addi r5, r8, -1
	ctx.r[5].s64 = ctx.r[8].s64 + -1;
	// 824FB898: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 824FB89C: 419800F4  blt cr6, 0x824fb990
	if ctx.cr[6].lt {
	pc = 0x824FB990; continue 'dispatch;
	}
	// 824FB8A0: 39050001  addi r8, r5, 1
	ctx.r[8].s64 = ctx.r[5].s64 + 1;
	// 824FB8A4: 5508F0BE  srwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FB8A8: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FB8AC: 7CA72850  subf r5, r7, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[7].s64;
	// 824FB8B0: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB8B4: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 824FB8B8: D001FFC0  stfs f0, -0x40(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 824FB8BC: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 824FB8C0: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB8C4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 824FB8C8: D001FFC4  stfs f0, -0x3c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 824FB8CC: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	// 824FB8D0: C00B0020  lfs f0, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB8D4: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 824FB8D8: D001FFC8  stfs f0, -0x38(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), tmp.u32 ) };
	// 824FB8DC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824FB8E0: C00A0010  lfs f0, 0x10(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB8E4: D001FFCC  stfs f0, -0x34(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-52 as u32), tmp.u32 ) };
	// 824FB8E8: 3881FFC0  addi r4, r1, -0x40
	ctx.r[4].s64 = ctx.r[1].s64 + -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FBA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FBA10 size=48
    let mut pc: u32 = 0x824FBA10;
    'dispatch: loop {
        match pc {
            0x824FBA10 => {
    //   block [0x824FBA10..0x824FBA40)
	// 824FBA10: C00B0008  lfs f0, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBA14: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 824FBA18: C00B0018  lfs f0, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBA1C: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 824FBA20: C00B0028  lfs f0, 0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBA24: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 824FBA28: C00A0010  lfs f0, 0x10(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBA2C: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 824FBA30: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FBA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FBA40 size=236
    let mut pc: u32 = 0x824FBA40;
    'dispatch: loop {
        match pc {
            0x824FBA40 => {
    //   block [0x824FBA40..0x824FBB2C)
	// 824FBA40: 3D20829A  lis r9, -0x7d66
	ctx.r[9].s64 = -2103836672;
	// 824FBA44: 80C30040  lwz r6, 0x40(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FBA48: 81492250  lwz r10, 0x2250(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8784 as u32) ) } as u64;
	// 824FBA4C: 554B07FE  clrlwi r11, r10, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 824FBA50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FBA54: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 824FBA58: 396B2240  addi r11, r11, 0x2240
	ctx.r[11].s64 = ctx.r[11].s64 + 8768;
	// 824FBA5C: 409A0024  bne cr6, 0x824fba80
	if !ctx.cr[6].eq {
	pc = 0x824FBA80; continue 'dispatch;
	}
	// 824FBA60: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 824FBA64: 91492250  stw r10, 0x2250(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8784 as u32), ctx.r[10].u32 ) };
	// 824FBA68: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FBA6C: C00A0DA0  lfs f0, 0xda0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3488 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBA70: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824FBA74: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824FBA78: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824FBA7C: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FBB2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FBB2C size=196
    let mut pc: u32 = 0x824FBB2C;
    'dispatch: loop {
        match pc {
            0x824FBB2C => {
    //   block [0x824FBB2C..0x824FBBF0)
	// 824FBB2C: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FBBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FBBF0 size=520
    let mut pc: u32 = 0x824FBBF0;
    'dispatch: loop {
        match pc {
            0x824FBBF0 => {
    //   block [0x824FBBF0..0x824FBDF8)
	// 824FBBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FBBF4: 480394C5  bl 0x825350b8
	ctx.lr = 0x824FBBF8;
	sub_82535080(ctx, base);
	// 824FBBF8: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 824FBBFC: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 824FBC00: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 824FBC04: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 824FBC08: 4198017C  blt cr6, 0x824fbd84
	if ctx.cr[6].lt {
	pc = 0x824FBD84; continue 'dispatch;
	}
	// 824FBC0C: 5547F0BE  srwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FBC10: 39660018  addi r11, r6, 0x18
	ctx.r[11].s64 = ctx.r[6].s64 + 24;
	// 824FBC14: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBC18: 7FCAF050  subf r30, r10, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[10].s64;
	// 824FBC1C: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FBC20: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 824FBC24: 80A30040  lwz r5, 0x40(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FBC28: 7D481670  srawi r8, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 824FBC2C: 554907BE  clrlwi r9, r10, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 824FBC30: 655F3F00  oris r31, r10, 0x3f00
	ctx.r[31].u64 = ctx.r[10].u64 | 1056964608;
	// 824FBC34: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBC38: 3BA90004  addi r29, r9, 4
	ctx.r[29].s64 = ctx.r[9].s64 + 4;
	// 824FBC3C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824FBC40: 553C103A  slwi r28, r9, 2
	ctx.r[28].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 824FBC44: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBC48: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FBC4C: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 824FBC50: 57A8103A  slwi r8, r29, 2
	ctx.r[8].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FBC54: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FBC58: 7C1C542E  lfsx f0, r28, r10
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBC5C: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824FBC60: 7C08542E  lfsx f0, r8, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBC64: D00BFFEC  stfs f0, -0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 824FBC68: 7C09542E  lfsx f0, r9, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBC6C: D00BFFF0  stfs f0, -0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 824FBC70: 93EBFFF4  stw r31, -0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-12 as u32), ctx.r[31].u32 ) };
	// 824FBC74: A1440002  lhz r10, 2(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 824FBC78: 80A30040  lwz r5, 0x40(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FBC7C: 7D481670  srawi r8, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 824FBC80: 554907BE  clrlwi r9, r10, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 824FBC84: 655F3F00  oris r31, r10, 0x3f00
	ctx.r[31].u64 = ctx.r[10].u64 | 1056964608;
	// 824FBC88: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBC8C: 3BA90004  addi r29, r9, 4
	ctx.r[29].s64 = ctx.r[9].s64 + 4;
	// 824FBC90: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824FBC94: 553C103A  slwi r28, r9, 2
	ctx.r[28].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 824FBC98: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBC9C: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FBCA0: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 824FBCA4: 57A8103A  slwi r8, r29, 2
	ctx.r[8].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FBCA8: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FBCAC: 7C1C542E  lfsx f0, r28, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBCB0: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 824FBCB4: 7C08542E  lfsx f0, r8, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBCB8: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 824FBCBC: 7C09542E  lfsx f0, r9, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBCC0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824FBCC4: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824FBCC8: A1440004  lhz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FBCCC: 80A30040  lwz r5, 0x40(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FBCD0: 7D481670  srawi r8, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 824FBCD4: 554907BE  clrlwi r9, r10, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 824FBCD8: 655F3F00  oris r31, r10, 0x3f00
	ctx.r[31].u64 = ctx.r[10].u64 | 1056964608;
	// 824FBCDC: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBCE0: 3BA90004  addi r29, r9, 4
	ctx.r[29].s64 = ctx.r[9].s64 + 4;
	// 824FBCE4: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824FBCE8: 553C103A  slwi r28, r9, 2
	ctx.r[28].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 824FBCEC: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBCF0: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FBCF4: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 824FBCF8: 57A8103A  slwi r8, r29, 2
	ctx.r[8].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FBCFC: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FBD00: 7C1C542E  lfsx f0, r28, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBD04: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824FBD08: 7C08542E  lfsx f0, r8, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBD0C: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824FBD10: 7C09542E  lfsx f0, r9, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBD14: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824FBD18: 93EB0014  stw r31, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 824FBD1C: A1440006  lhz r10, 6(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FBD20: 38840008  addi r4, r4, 8
	ctx.r[4].s64 = ctx.r[4].s64 + 8;
	// 824FBD24: 80A30040  lwz r5, 0x40(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FBD28: 7D481670  srawi r8, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 824FBD2C: 554907BE  clrlwi r9, r10, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 824FBD30: 551F083C  slwi r31, r8, 1
	ctx.r[31].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824FBD34: 553D103A  slwi r29, r9, 2
	ctx.r[29].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 824FBD38: 7D08FA14  add r8, r8, r31
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 824FBD3C: 654A3F00  oris r10, r10, 0x3f00
	ctx.r[10].u64 = ctx.r[10].u64 | 1056964608;
	// 824FBD40: 55082036  slwi r8, r8, 4
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FBD44: 7D082A14  add r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[5].u64;
	// 824FBD48: 38A90004  addi r5, r9, 4
	ctx.r[5].s64 = ctx.r[9].s64 + 4;
	// 824FBD4C: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FBD50: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FBD54: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FBD58: 7C1D442E  lfsx f0, r29, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBD5C: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824FBD60: 7C05442E  lfsx f0, r5, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBD64: 38C60040  addi r6, r6, 0x40
	ctx.r[6].s64 = ctx.r[6].s64 + 64;
	// 824FBD68: D00B001C  stfs f0, 0x1c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824FBD6C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 824FBD70: 7C09442E  lfsx f0, r9, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBD74: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824FBD78: 914B0024  stw r10, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 824FBD7C: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 824FBD80: 409AFE9C  bne cr6, 0x824fbc1c
	if !ctx.cr[6].eq {
	pc = 0x824FBC1C; continue 'dispatch;
	}
	// 824FBD84: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824FBD88: 4198006C  blt cr6, 0x824fbdf4
	if ctx.cr[6].lt {
	pc = 0x824FBDF4; continue 'dispatch;
	}
	// 824FBD8C: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FBD90: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824FBD94: 81030040  lwz r8, 0x40(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FBD98: 38840002  addi r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 2;
	// 824FBD9C: 7D691670  srawi r9, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 824FBDA0: 556A07BE  clrlwi r10, r11, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 824FBDA4: 65673F00  oris r7, r11, 0x3f00
	ctx.r[7].u64 = ctx.r[11].u64 | 1056964608;
	// 824FBDA8: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FBDAC: 38AA0004  addi r5, r10, 4
	ctx.r[5].s64 = ctx.r[10].s64 + 4;
	// 824FBDB0: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824FBDB4: 555F103A  slwi r31, r10, 2
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824FBDB8: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FBDBC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824FBDC0: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824FBDC4: 54A9103A  slwi r9, r5, 2
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FBDC8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBDCC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824FBDD0: 7C1F5C2E  lfsx f0, r31, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBDD4: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824FBDD8: 7C095C2E  lfsx f0, r9, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBDDC: D0060004  stfs f0, 4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824FBDE0: 7C0A5C2E  lfsx f0, r10, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBDE4: D0060008  stfs f0, 8(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824FBDE8: 90E6000C  stw r7, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 824FBDEC: 38C60010  addi r6, r6, 0x10
	ctx.r[6].s64 = ctx.r[6].s64 + 16;
	// 824FBDF0: 4098FF9C  bge cr6, 0x824fbd8c
	if !ctx.cr[6].lt {
	pc = 0x824FBD8C; continue 'dispatch;
	}
	// 824FBDF4: 48039314  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FBDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FBDF8 size=16
    let mut pc: u32 = 0x824FBDF8;
    'dispatch: loop {
        match pc {
            0x824FBDF8 => {
    //   block [0x824FBDF8..0x824FBE08)
	// 824FBDF8: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FBE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FBE08 size=212
    let mut pc: u32 = 0x824FBE08;
    'dispatch: loop {
        match pc {
            0x824FBE08 => {
    //   block [0x824FBE08..0x824FBEDC)
	// 824FBE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FBE0C: 480392A5  bl 0x825350b0
	ctx.lr = 0x824FBE10;
	sub_82535080(ctx, base);
	// 824FBE10: 39250003  addi r9, r5, 3
	ctx.r[9].s64 = ctx.r[5].s64 + 3;
	// 824FBE14: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824FBE18: 3FC08206  lis r30, -0x7dfa
	ctx.r[30].s64 = -2113536000;
	// 824FBE1C: 5529003A  rlwinm r9, r9, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 824FBE20: 3BDE3804  addi r30, r30, 0x3804
	ctx.r[30].s64 = ctx.r[30].s64 + 14340;
	// 824FBE24: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 824FBE28: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824FBE2C: 3B400007  li r26, 7
	ctx.r[26].s64 = 7;
	// 824FBE30: 7D291670  srawi r9, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 2) as i64;
	// 824FBE34: 3F808000  lis r28, -0x8000
	ctx.r[28].s64 = -2147483648;
	// 824FBE38: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 824FBE3C: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824FBE40: B3630006  sth r27, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[27].u16 ) };
	// 824FBE44: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 824FBE48: 7CFEE378  or r30, r7, r28
	ctx.r[30].u64 = ctx.r[7].u64 | ctx.r[28].u64;
	// 824FBE4C: 9343000C  stw r26, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 824FBE50: 3BE80010  addi r31, r8, 0x10
	ctx.r[31].s64 = ctx.r[8].s64 + 16;
	// 824FBE54: 90830040  stw r4, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[4].u32 ) };
	// 824FBE58: 7D24E378  or r4, r9, r28
	ctx.r[4].u64 = ctx.r[9].u64 | ctx.r[28].u64;
	// 824FBE5C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824FBE60: 91230044  stw r9, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 824FBE64: 3B61FFC0  addi r27, r1, -0x40
	ctx.r[27].s64 = ctx.r[1].s64 + -64;
	// 824FBE68: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 824FBE6C: 3B41FFC4  addi r26, r1, -0x3c
	ctx.r[26].s64 = ctx.r[1].s64 + -60;
	// 824FBE70: 90830048  stw r4, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[4].u32 ) };
	// 824FBE74: 90A3004C  stw r5, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[5].u32 ) };
	// 824FBE78: C00ABFFC  lfs f0, -0x4004(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBE7C: 90C30050  stw r6, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 824FBE80: D001FFC0  stfs f0, -0x40(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 824FBE84: 90E30054  stw r7, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FBEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FBEE0 size=232
    let mut pc: u32 = 0x824FBEE0;
    'dispatch: loop {
        match pc {
            0x824FBEE0 => {
    //   block [0x824FBEE0..0x824FBFC8)
	// 824FBEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FBEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FBEE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FBEEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FBEF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FBEF4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FBEF8: 396B3804  addi r11, r11, 0x3804
	ctx.r[11].s64 = ctx.r[11].s64 + 14340;
	// 824FBEFC: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 824FBF00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FBF04: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FBF08: 419A003C  beq cr6, 0x824fbf44
	if ctx.cr[6].eq {
	pc = 0x824FBF44; continue 'dispatch;
	}
	// 824FBF0C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FBF10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FBF14: 419A0030  beq cr6, 0x824fbf44
	if ctx.cr[6].eq {
	pc = 0x824FBF44; continue 'dispatch;
	}
	// 824FBF18: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FBF1C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824FBF20: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824FBF24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FBF28: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824FBF2C: 409A0018  bne cr6, 0x824fbf44
	if !ctx.cr[6].eq {
	pc = 0x824FBF44; continue 'dispatch;
	}
	// 824FBF30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FBF34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FBF38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FBF3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FBF40: 4E800421  bctrl
	ctx.lr = 0x824FBF44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FBF44: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 824FBF48: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FBF4C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FBF50: 409A0020  bne cr6, 0x824fbf70
	if !ctx.cr[6].eq {
	pc = 0x824FBF70; continue 'dispatch;
	}
	// 824FBF54: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FBF58: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FBF5C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FBF60: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 824FBF64: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FBF68: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FBF6C: 4BF6814D  bl 0x824640b8
	ctx.lr = 0x824FBF70;
	sub_824640B8(ctx, base);
	// 824FBF70: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824FBF74: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FBF78: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FBF7C: 409A002C  bne cr6, 0x824fbfa8
	if !ctx.cr[6].eq {
	pc = 0x824FBFA8; continue 'dispatch;
	}
	// 824FBF80: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FBF84: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FBF88: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FBF8C: 809F0040  lwz r4, 0x40(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FBF90: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FBF94: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FBF98: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBF9C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824FBFA0: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FBFA4: 4BF68115  bl 0x824640b8
	ctx.lr = 0x824FBFA8;
	sub_824640B8(ctx, base);
	// 824FBFA8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824FBFAC: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824FBFB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FBFB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FBFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FBFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FBFC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FBFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FBFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824FBFC8 size=228
    let mut pc: u32 = 0x824FBFC8;
    'dispatch: loop {
        match pc {
            0x824FBFC8 => {
    //   block [0x824FBFC8..0x824FC0AC)
	// 824FBFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FBFCC: 480390F1  bl 0x825350bc
	ctx.lr = 0x824FBFD0;
	sub_82535080(ctx, base);
	// 824FBFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FBFD4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FBFD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FBFDC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FBFE0: 83BF004C  lwz r29, 0x4c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824FBFE4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FBFE8: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824FBFEC: 40980024  bge cr6, 0x824fc010
	if !ctx.cr[6].lt {
	pc = 0x824FC010; continue 'dispatch;
	}
	// 824FBFF0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FBFF4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FBFF8: 41980008  blt cr6, 0x824fc000
	if ctx.cr[6].lt {
	pc = 0x824FC000; continue 'dispatch;
	}
	// 824FBFFC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 824FC000: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 824FC004: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824FC008: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824FC00C: 4BF722BD  bl 0x8246e2c8
	ctx.lr = 0x824FC010;
	sub_8246E2C8(ctx, base);
	// 824FC010: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824FC014: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FC018: 815F004C  lwz r10, 0x4c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824FC01C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FC020: 40990084  ble cr6, 0x824fc0a4
	if !ctx.cr[6].gt {
	pc = 0x824FC0A4; continue 'dispatch;
	}
	// 824FC024: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824FC028: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824FC02C: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC030: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 824FC034: 7D691670  srawi r9, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 824FC038: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FC03C: 556707BE  clrlwi r7, r11, 0x1e
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 824FC040: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC044: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824FC048: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FC04C: 5526083C  slwi r6, r9, 1
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC050: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 824FC054: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FC058: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 824FC05C: 38E90004  addi r7, r9, 4
	ctx.r[7].s64 = ctx.r[9].s64 + 4;
	// 824FC060: 38C90008  addi r6, r9, 8
	ctx.r[6].s64 = ctx.r[9].s64 + 8;
	// 824FC064: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FC068: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC06C: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC070: 7C09542E  lfsx f0, r9, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC074: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824FC078: 7C07542E  lfsx f0, r7, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC07C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 824FC080: 7C06542E  lfsx f0, r6, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC084: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 824FC088: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FC0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824FC0B0 size=1200
    let mut pc: u32 = 0x824FC0B0;
    'dispatch: loop {
        match pc {
            0x824FC0B0 => {
    //   block [0x824FC0B0..0x824FC560)
	// 824FC0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FC0B4: 48038FE5  bl 0x82535098
	ctx.lr = 0x824FC0B8;
	sub_82535080(ctx, base);
	// 824FC0B8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FC0BC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824FC0C0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824FC0C4: 397C0003  addi r11, r28, 3
	ctx.r[11].s64 = ctx.r[28].s64 + 3;
	// 824FC0C8: 3BFA0040  addi r31, r26, 0x40
	ctx.r[31].s64 = ctx.r[26].s64 + 64;
	// 824FC0CC: 557B003A  rlwinm r27, r11, 0, 0, 0x1d
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FC0D0: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 824FC0D4: 7F6B1670  srawi r11, r27, 2
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[27].s32 >> 2) as i64;
	// 824FC0D8: 939A004C  stw r28, 0x4c(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(76 as u32), ctx.r[28].u32 ) };
	// 824FC0DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824FC0E0: 7FAB0194  addze r29, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[29].s64 = tmp.s64;
	// 824FC0E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FC0E8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FC0EC: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824FC0F0: 40980024  bge cr6, 0x824fc114
	if !ctx.cr[6].lt {
	pc = 0x824FC114; continue 'dispatch;
	}
	// 824FC0F4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FC0F8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FC0FC: 41980008  blt cr6, 0x824fc104
	if ctx.cr[6].lt {
	pc = 0x824FC104; continue 'dispatch;
	}
	// 824FC100: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 824FC104: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 824FC108: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824FC10C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FC110: 4BF721B9  bl 0x8246e2c8
	ctx.lr = 0x824FC114;
	sub_8246E2C8(ctx, base);
	// 824FC114: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 824FC118: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824FC11C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FC120: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 824FC124: 4198017C  blt cr6, 0x824fc2a0
	if ctx.cr[6].lt {
	pc = 0x824FC2A0; continue 'dispatch;
	}
	// 824FC128: 387CFFFD  addi r3, r28, -3
	ctx.r[3].s64 = ctx.r[28].s64 + -3;
	// 824FC12C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FC130: 5568F0BE  srwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC134: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC138: 388A0001  addi r4, r10, 1
	ctx.r[4].s64 = ctx.r[10].s64 + 1;
	// 824FC13C: C0090000  lfs f0, 0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC140: 5506083C  slwi r6, r8, 1
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC144: 5547F0BE  srwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC148: 7D083214  add r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 824FC14C: 5486F0BE  srwi r6, r4, 2
	ctx.r[6].u32 = ctx.r[4].u32.wrapping_shr(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC150: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC154: 556507BE  clrlwi r5, r11, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 824FC158: 54E4083C  slwi r4, r7, 1
	ctx.r[4].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC15C: 3AEA0002  addi r23, r10, 2
	ctx.r[23].s64 = ctx.r[10].s64 + 2;
	// 824FC160: 7D082A14  add r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[5].u64;
	// 824FC164: 7C872214  add r4, r7, r4
	ctx.r[4].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 824FC168: 56E7F0BE  srwi r7, r23, 2
	ctx.r[7].u32 = ctx.r[23].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC16C: 5517103A  slwi r23, r8, 2
	ctx.r[23].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 824FC170: 3AA80004  addi r21, r8, 4
	ctx.r[21].s64 = ctx.r[8].s64 + 4;
	// 824FC174: 5484103A  slwi r4, r4, 2
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC178: 56B5103A  slwi r21, r21, 2
	ctx.r[21].u32 = ctx.r[21].u32.wrapping_shl(2);
	ctx.r[21].u64 = ctx.r[21].u32 as u64;
	// 824FC17C: 554507BE  clrlwi r5, r10, 0x1e
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 824FC180: 7C17ED2E  stfsx f0, r23, r29
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[23].u32.wrapping_add(ctx.r[29].u32), tmp.u32) };
	// 824FC184: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC188: 3A880008  addi r20, r8, 8
	ctx.r[20].s64 = ctx.r[8].s64 + 8;
	// 824FC18C: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC190: 7D042A14  add r8, r4, r5
	ctx.r[8].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 824FC194: 5684103A  slwi r4, r20, 2
	ctx.r[4].u32 = ctx.r[20].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC198: 3B0A0001  addi r24, r10, 1
	ctx.r[24].s64 = ctx.r[10].s64 + 1;
	// 824FC19C: 7C15ED2E  stfsx f0, r21, r29
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[21].u32.wrapping_add(ctx.r[29].u32), tmp.u32) };
	// 824FC1A0: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC1A4: C0090008  lfs f0, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC1A8: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 824FC1AC: 570507BE  clrlwi r5, r24, 0x1e
	ctx.r[5].u64 = ctx.r[24].u32 as u64 & 0x00000003u64;
	// 824FC1B0: 3B080004  addi r24, r8, 4
	ctx.r[24].s64 = ctx.r[8].s64 + 4;
	// 824FC1B4: 3AE80008  addi r23, r8, 8
	ctx.r[23].s64 = ctx.r[8].s64 + 8;
	// 824FC1B8: 7C04ED2E  stfsx f0, r4, r29
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[29].u32), tmp.u32) };
	// 824FC1BC: 54C4083C  slwi r4, r6, 1
	ctx.r[4].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC1C0: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC1C4: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC1C8: 7CC62214  add r6, r6, r4
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 824FC1CC: 5504103A  slwi r4, r8, 2
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC1D0: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC1D4: 5718103A  slwi r24, r24, 2
	ctx.r[24].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 824FC1D8: 7D062A14  add r8, r6, r5
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 824FC1DC: 3ACAFFFE  addi r22, r10, -2
	ctx.r[22].s64 = ctx.r[10].s64 + -2;
	// 824FC1E0: 7C04ED2E  stfsx f0, r4, r29
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[29].u32), tmp.u32) };
	// 824FC1E4: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC1E8: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC1EC: 56E4103A  slwi r4, r23, 2
	ctx.r[4].u32 = ctx.r[23].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC1F0: 3BA80004  addi r29, r8, 4
	ctx.r[29].s64 = ctx.r[8].s64 + 4;
	// 824FC1F4: 56C607BE  clrlwi r6, r22, 0x1e
	ctx.r[6].u64 = ctx.r[22].u32 as u64 & 0x00000003u64;
	// 824FC1F8: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 824FC1FC: 7C182D2E  stfsx f0, r24, r5
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[24].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 824FC200: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC204: C0090008  lfs f0, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC208: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 824FC20C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824FC210: 7C042D2E  stfsx f0, r4, r5
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 824FC214: 54E5083C  slwi r5, r7, 1
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FC218: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC21C: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC220: 7CE72A14  add r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 824FC224: 5505103A  slwi r5, r8, 2
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FC228: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 824FC22C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC230: 5518103A  slwi r24, r8, 2
	ctx.r[24].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 824FC234: 7D073214  add r8, r7, r6
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 824FC238: 7C05252E  stfsx f0, r5, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 824FC23C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC240: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC244: 38A80004  addi r5, r8, 4
	ctx.r[5].s64 = ctx.r[8].s64 + 4;
	// 824FC248: 5506103A  slwi r6, r8, 2
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC24C: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 824FC250: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FC254: 7C1D3D2E  stfsx f0, r29, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC258: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC25C: C0090008  lfs f0, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC260: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 824FC264: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC268: 7C183D2E  stfsx f0, r24, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[24].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC26C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC270: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC274: 7C063D2E  stfsx f0, r6, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC278: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC27C: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC280: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824FC284: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 824FC288: 7C053D2E  stfsx f0, r5, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC28C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC290: C0090008  lfs f0, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC294: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 824FC298: 7C083D2E  stfsx f0, r8, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC29C: 4198FE94  blt cr6, 0x824fc130
	if ctx.cr[6].lt {
	pc = 0x824FC130; continue 'dispatch;
	}
	// 824FC2A0: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 824FC2A4: 40980064  bge cr6, 0x824fc308
	if !ctx.cr[6].lt {
	pc = 0x824FC308; continue 'dispatch;
	}
	// 824FC2A8: 556AF0BE  srwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FC2AC: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC2B0: 556807BE  clrlwi r8, r11, 0x1e
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 824FC2B4: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC2B8: 5547083C  slwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC2BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FC2C0: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 824FC2C4: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 824FC2C8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FC2CC: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 824FC2D0: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC2D4: 38EA0004  addi r7, r10, 4
	ctx.r[7].s64 = ctx.r[10].s64 + 4;
	// 824FC2D8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824FC2DC: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC2E0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FC2E4: 7C08352E  stfsx f0, r8, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 824FC2E8: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC2EC: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC2F0: 7C07452E  stfsx f0, r7, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 824FC2F4: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC2F8: C0090008  lfs f0, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC2FC: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 824FC300: 7C0A452E  stfsx f0, r10, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 824FC304: 4198FFA4  blt cr6, 0x824fc2a8
	if ctx.cr[6].lt {
	pc = 0x824FC2A8; continue 'dispatch;
	}
	// 824FC308: 7D0BD850  subf r8, r11, r27
	ctx.r[8].s64 = ctx.r[27].s64 - ctx.r[11].s64;
	// 824FC30C: 7D5E4850  subf r10, r30, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 824FC310: 2F080004  cmpwi cr6, r8, 4
	ctx.cr[6].compare_i32(ctx.r[8].s32, 4, &mut ctx.xer);
	// 824FC314: 4198016C  blt cr6, 0x824fc480
	if ctx.cr[6].lt {
	pc = 0x824FC480; continue 'dispatch;
	}
	// 824FC318: 3BBBFFFD  addi r29, r27, -3
	ctx.r[29].s64 = ctx.r[27].s64 + -3;
	// 824FC31C: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 824FC320: 7D681670  srawi r8, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 824FC324: 831F0000  lwz r24, 0(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC328: 7D271670  srawi r7, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[9].s32 >> 2) as i64;
	// 824FC32C: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC330: 5504083C  slwi r4, r8, 1
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC334: 556507BE  clrlwi r5, r11, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 824FC338: 7D082214  add r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 824FC33C: 54E3083C  slwi r3, r7, 1
	ctx.r[3].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824FC340: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC344: 7CE71A14  add r7, r7, r3
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[3].u64;
	// 824FC348: 7D082A14  add r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[5].u64;
	// 824FC34C: 38C90001  addi r6, r9, 1
	ctx.r[6].s64 = ctx.r[9].s64 + 1;
	// 824FC350: 5503103A  slwi r3, r8, 2
	ctx.r[3].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824FC354: 3AE90002  addi r23, r9, 2
	ctx.r[23].s64 = ctx.r[9].s64 + 2;
	// 824FC358: 7CC61670  srawi r6, r6, 2
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[6].s32 >> 2) as i64;
	// 824FC35C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC360: 7EE51670  srawi r5, r23, 2
	ctx.xer.ca = (ctx.r[23].s32 < 0) && ((ctx.r[23].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[23].s32 >> 2) as i64;
	// 824FC364: 552407BE  clrlwi r4, r9, 0x1e
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0x00000003u64;
	// 824FC368: 7C03C52E  stfsx f0, r3, r24
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[24].u32), tmp.u32) };
	// 824FC36C: 3AE80004  addi r23, r8, 4
	ctx.r[23].s64 = ctx.r[8].s64 + 4;
	// 824FC370: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC374: 3A880008  addi r20, r8, 8
	ctx.r[20].s64 = ctx.r[8].s64 + 8;
	// 824FC378: 7D072214  add r8, r7, r4
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 824FC37C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC380: 56E3103A  slwi r3, r23, 2
	ctx.r[3].u32 = ctx.r[23].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824FC384: 5698103A  slwi r24, r20, 2
	ctx.r[24].u32 = ctx.r[20].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 824FC388: 3AC90001  addi r22, r9, 1
	ctx.r[22].s64 = ctx.r[9].s64 + 1;
	// 824FC38C: 3AE80008  addi r23, r8, 8
	ctx.r[23].s64 = ctx.r[8].s64 + 8;
	// 824FC390: 56C707BE  clrlwi r7, r22, 0x1e
	ctx.r[7].u64 = ctx.r[22].u32 as u64 & 0x00000003u64;
	// 824FC394: 7C03252E  stfsx f0, r3, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 824FC398: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC39C: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC3A0: 3AA9FFFE  addi r21, r9, -2
	ctx.r[21].s64 = ctx.r[9].s64 + -2;
	// 824FC3A4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824FC3A8: 7C18252E  stfsx f0, r24, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[24].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 824FC3AC: 54C4083C  slwi r4, r6, 1
	ctx.r[4].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC3B0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC3B4: 3B080004  addi r24, r8, 4
	ctx.r[24].s64 = ctx.r[8].s64 + 4;
	// 824FC3B8: 7CC62214  add r6, r6, r4
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 824FC3BC: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC3C0: 5504103A  slwi r4, r8, 2
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC3C4: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC3C8: 7D063A14  add r8, r6, r7
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 824FC3CC: 56A707BE  clrlwi r7, r21, 0x1e
	ctx.r[7].u64 = ctx.r[21].u32 as u64 & 0x00000003u64;
	// 824FC3D0: 7C041D2E  stfsx f0, r4, r3
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 824FC3D4: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC3D8: 5704103A  slwi r4, r24, 2
	ctx.r[4].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC3DC: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC3E0: 56E3103A  slwi r3, r23, 2
	ctx.r[3].u32 = ctx.r[23].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824FC3E4: 3B080008  addi r24, r8, 8
	ctx.r[24].s64 = ctx.r[8].s64 + 8;
	// 824FC3E8: 7C04352E  stfsx f0, r4, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 824FC3EC: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC3F0: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC3F4: 7C03352E  stfsx f0, r3, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 824FC3F8: 54A6083C  slwi r6, r5, 1
	ctx.r[6].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC3FC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC400: 38680004  addi r3, r8, 4
	ctx.r[3].s64 = ctx.r[8].s64 + 4;
	// 824FC404: 7CC53214  add r6, r5, r6
	ctx.r[6].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 824FC408: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC40C: 5505103A  slwi r5, r8, 2
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FC410: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC414: 7D063A14  add r8, r6, r7
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 824FC418: 5467103A  slwi r7, r3, 2
	ctx.r[7].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC41C: 7C05252E  stfsx f0, r5, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 824FC420: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC424: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC428: 5705103A  slwi r5, r24, 2
	ctx.r[5].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FC42C: 7C07352E  stfsx f0, r7, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 824FC430: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC434: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC438: 7C053D2E  stfsx f0, r5, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC43C: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC440: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC444: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC448: 38A80004  addi r5, r8, 4
	ctx.r[5].s64 = ctx.r[8].s64 + 4;
	// 824FC44C: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 824FC450: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FC454: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC458: 7C07352E  stfsx f0, r7, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 824FC45C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC460: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC464: 7C053D2E  stfsx f0, r5, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC468: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC46C: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC470: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 824FC474: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824FC478: 7C083D2E  stfsx f0, r8, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC47C: 4198FEA4  blt cr6, 0x824fc320
	if ctx.cr[6].lt {
	pc = 0x824FC320; continue 'dispatch;
	}
	// 824FC480: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 824FC484: 40980060  bge cr6, 0x824fc4e4
	if !ctx.cr[6].lt {
	pc = 0x824FC4E4; continue 'dispatch;
	}
	// 824FC488: 7D691670  srawi r9, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 824FC48C: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC490: 556807BE  clrlwi r8, r11, 0x1e
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 824FC494: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC498: 5527083C  slwi r7, r9, 1
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC49C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FC4A0: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 824FC4A4: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 824FC4A8: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FC4AC: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 824FC4B0: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC4B4: 38E90004  addi r7, r9, 4
	ctx.r[7].s64 = ctx.r[9].s64 + 4;
	// 824FC4B8: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FC4BC: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC4C0: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FC4C4: 7C08352E  stfsx f0, r8, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 824FC4C8: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC4CC: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC4D0: 7C07452E  stfsx f0, r7, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 824FC4D4: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC4D8: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC4DC: 7C09452E  stfsx f0, r9, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 824FC4E0: 4198FFA8  blt cr6, 0x824fc488
	if ctx.cr[6].lt {
	pc = 0x824FC488; continue 'dispatch;
	}
	// 824FC4E4: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 824FC4E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824FC4EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824FC4F0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 824FC4F4: 4800488D  bl 0x82500d80
	ctx.lr = 0x824FC4F8;
	sub_82500D80(ctx, base);
	// 824FC4F8: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 824FC4FC: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 824FC500: 397A0030  addi r11, r26, 0x30
	ctx.r[11].s64 = ctx.r[26].s64 + 48;
	// 824FC504: 395A0020  addi r10, r26, 0x20
	ctx.r[10].s64 = ctx.r[26].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FC560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FC560 size=32
    let mut pc: u32 = 0x824FC560;
    'dispatch: loop {
        match pc {
            0x824FC560 => {
    //   block [0x824FC560..0x824FC580)
	// 824FC560: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824FC564: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824FC568: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 824FC56C: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
	// 824FC570: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 824FC574: C00B0010  lfs f0, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC578: EC20082A  fadds f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 824FC57C: 480003D4  b 0x824fc950
	sub_824FC950(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FC580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FC580 size=444
    let mut pc: u32 = 0x824FC580;
    'dispatch: loop {
        match pc {
            0x824FC580 => {
    //   block [0x824FC580..0x824FC73C)
	// 824FC580: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 824FC584: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC588: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 824FC58C: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824FC590: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC594: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FC598: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FC59C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824FC5A0: 40980020  bge cr6, 0x824fc5c0
	if !ctx.cr[6].lt {
	pc = 0x824FC5C0; continue 'dispatch;
	}
	// 824FC5A4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FC5A8: 39293FC0  addi r9, r9, 0x3fc0
	ctx.r[9].s64 = ctx.r[9].s64 + 16320;
	// 824FC5AC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FC5B0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824FC5B4: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 824FC5B8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824FC5BC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 824FC5C0: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 824FC5C4: 81640054  lwz r11, 0x54(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(84 as u32) ) } as u64;
	// 824FC5C8: C1860010  lfs f12, 0x10(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824FC5CC: 38E60010  addi r7, r6, 0x10
	ctx.r[7].s64 = ctx.r[6].s64 + 16;
	// 824FC5D0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824FC5D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FC740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FC740 size=232
    let mut pc: u32 = 0x824FC740;
    'dispatch: loop {
        match pc {
            0x824FC740 => {
    //   block [0x824FC740..0x824FC828)
	// 824FC740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FC744: 48038975  bl 0x825350b8
	ctx.lr = 0x824FC748;
	sub_82535080(ctx, base);
	// 824FC748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FC74C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FC750: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FC754: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FC758: 388B3FD8  addi r4, r11, 0x3fd8
	ctx.r[4].s64 = ctx.r[11].s64 + 16344;
	// 824FC75C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 824FC760: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC764: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FC768: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824FC76C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FC770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FC774: 4E800421  bctrl
	ctx.lr = 0x824FC778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FC778: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824FC77C: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 824FC780: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FC784: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FC788: 3BAB3FD0  addi r29, r11, 0x3fd0
	ctx.r[29].s64 = ctx.r[11].s64 + 16336;
	// 824FC78C: 409A0044  bne cr6, 0x824fc7d0
	if !ctx.cr[6].eq {
	pc = 0x824FC7D0; continue 'dispatch;
	}
	// 824FC790: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC794: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824FC798: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 824FC79C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FC7A0: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC7A4: 80DF0040  lwz r6, 0x40(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FC7A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824FC7AC: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 824FC7B0: 83890008  lwz r28, 8(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FC7B4: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FC7B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824FC7BC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824FC7C0: 55482036  slwi r8, r10, 4
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC7C4: 55672036  slwi r7, r11, 4
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC7C8: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 824FC7CC: 4E800421  bctrl
	ctx.lr = 0x824FC7D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FC7D0: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 824FC7D4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FC7D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FC7DC: 409A0030  bne cr6, 0x824fc80c
	if !ctx.cr[6].eq {
	pc = 0x824FC80C; continue 'dispatch;
	}
	// 824FC7E0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC7E4: 55682036  slwi r8, r11, 4
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC7E8: 813F0054  lwz r9, 0x54(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824FC7EC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FC7F0: 80DF0050  lwz r6, 0x50(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 824FC7F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824FC7F8: 55272036  slwi r7, r9, 4
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC7FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824FC800: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FC804: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FC808: 4E800421  bctrl
	ctx.lr = 0x824FC80C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FC80C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC810: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824FC814: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FC818: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FC81C: 4E800421  bctrl
	ctx.lr = 0x824FC820;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FC820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FC824: 480388E4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FC828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824FC828 size=296
    let mut pc: u32 = 0x824FC828;
    'dispatch: loop {
        match pc {
            0x824FC828 => {
    //   block [0x824FC828..0x824FC950)
	// 824FC828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FC82C: 4803888D  bl 0x825350b8
	ctx.lr = 0x824FC830;
	sub_82535080(ctx, base);
	// 824FC830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FC834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FC838: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824FC83C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FC840: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 824FC844: 394B3804  addi r10, r11, 0x3804
	ctx.r[10].s64 = ctx.r[11].s64 + 14340;
	// 824FC848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FC84C: D03F0010  stfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824FC850: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824FC854: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 824FC858: 3BDF0050  addi r30, r31, 0x50
	ctx.r[30].s64 = ctx.r[31].s64 + 80;
	// 824FC85C: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824FC860: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824FC864: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FC868: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824FC86C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FC870: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 824FC874: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 824FC878: 913F0048  stw r9, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[9].u32 ) };
	// 824FC87C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FC880: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824FC884: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824FC888: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 824FC88C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FC890: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FC894: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FC898: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824FC89C: 40980060  bge cr6, 0x824fc8fc
	if !ctx.cr[6].lt {
	pc = 0x824FC8FC; continue 'dispatch;
	}
	// 824FC8A0: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FC8A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FC8A8: 409A0020  bne cr6, 0x824fc8c8
	if !ctx.cr[6].eq {
	pc = 0x824FC8C8; continue 'dispatch;
	}
	// 824FC8AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC8B0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FC8B4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FC8B8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC8BC: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FC8C0: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FC8C4: 4BF677F5  bl 0x824640b8
	ctx.lr = 0x824FC8C8;
	sub_824640B8(ctx, base);
	// 824FC8C8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC8CC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FC8D0: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FC8D4: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 824FC8D8: 55242036  slwi r4, r9, 4
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC8DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FC8E0: 4BF67759  bl 0x82464038
	ctx.lr = 0x824FC8E4;
	sub_82464038(ctx, base);
	// 824FC8E4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FC8E8: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824FC8EC: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FC8F0: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FC8F4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824FC8F8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FC8FC: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FC900: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC904: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FC908: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824FC90C: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC910: 40990020  ble cr6, 0x824fc930
	if !ctx.cr[6].gt {
	pc = 0x824FC930; continue 'dispatch;
	}
	// 824FC914: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 824FC918: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FC950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FC950 size=176
    let mut pc: u32 = 0x824FC950;
    'dispatch: loop {
        match pc {
            0x824FC950 => {
    //   block [0x824FC950..0x824FCA00)
	// 824FC950: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FCA00 size=124
    let mut pc: u32 = 0x824FCA00;
    'dispatch: loop {
        match pc {
            0x824FCA00 => {
    //   block [0x824FCA00..0x824FCA7C)
	// 824FCA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FCA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FCA08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FCA0C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FCA10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FCA14: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824FCA18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FCA1C: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 824FCA20: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCA24: 48002705  bl 0x824ff128
	ctx.lr = 0x824FCA28;
	sub_824FF128(ctx, base);
	// 824FCA28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCA2C: 48019B2D  bl 0x82516558
	ctx.lr = 0x824FCA30;
	sub_82516558(ctx, base);
	// 824FCA30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCA34: 480171F5  bl 0x82513c28
	ctx.lr = 0x824FCA38;
	sub_82513C28(ctx, base);
	// 824FCA38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCA3C: 48016495  bl 0x82512ed0
	ctx.lr = 0x824FCA40;
	sub_82512ED0(ctx, base);
	// 824FCA40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCA44: 48015945  bl 0x82512388
	ctx.lr = 0x824FCA48;
	sub_82512388(ctx, base);
	// 824FCA48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCA4C: 480B7245  bl 0x825b3c90
	ctx.lr = 0x824FCA50;
	sub_825B3C90(ctx, base);
	// 824FCA50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FCA54: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824FCA58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCA5C: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 824FCA60: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCA64: 480026C5  bl 0x824ff128
	ctx.lr = 0x824FCA68;
	sub_824FF128(ctx, base);
	// 824FCA68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FCA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FCA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FCA74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FCA78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FCA80 size=76
    let mut pc: u32 = 0x824FCA80;
    'dispatch: loop {
        match pc {
            0x824FCA80 => {
    //   block [0x824FCA80..0x824FCACC)
	// 824FCA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FCA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FCA88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FCA8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FCA90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FCA94: 4801D955  bl 0x8251a3e8
	ctx.lr = 0x824FCA98;
	sub_8251A3E8(ctx, base);
	// 824FCA98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCA9C: 4801C6CD  bl 0x82519168
	ctx.lr = 0x824FCAA0;
	sub_82519168(ctx, base);
	// 824FCAA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCAA4: 480B7D9D  bl 0x825b4840
	ctx.lr = 0x824FCAA8;
	sub_825B4840(ctx, base);
	// 824FCAA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCAAC: 4801AA95  bl 0x82517540
	ctx.lr = 0x824FCAB0;
	sub_82517540(ctx, base);
	// 824FCAB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCAB4: 480B75CD  bl 0x825b4080
	ctx.lr = 0x824FCAB8;
	sub_825B4080(ctx, base);
	// 824FCAB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FCABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FCAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FCAC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FCAC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FCAD0 size=164
    let mut pc: u32 = 0x824FCAD0;
    'dispatch: loop {
        match pc {
            0x824FCAD0 => {
    //   block [0x824FCAD0..0x824FCB74)
	// 824FCAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FCAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FCAD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FCADC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FCAE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FCAE4: 48026A55  bl 0x82523538
	ctx.lr = 0x824FCAE8;
	sub_82523538(ctx, base);
	// 824FCAE8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FCAEC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FCAF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCAF4: 480B9625  bl 0x825b6118
	ctx.lr = 0x824FCAF8;
	sub_825B6118(ctx, base);
	// 824FCAF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCAFC: 480B8D35  bl 0x825b5830
	ctx.lr = 0x824FCB00;
	sub_825B5830(ctx, base);
	// 824FCB00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB04: 48026895  bl 0x82523398
	ctx.lr = 0x824FCB08;
	sub_82523398(ctx, base);
	// 824FCB08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB0C: 480BA2D5  bl 0x825b6de0
	ctx.lr = 0x824FCB10;
	sub_825B6DE0(ctx, base);
	// 824FCB10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB14: 48025FCD  bl 0x82522ae0
	ctx.lr = 0x824FCB18;
	sub_82522AE0(ctx, base);
	// 824FCB18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB1C: 480251B5  bl 0x82521cd0
	ctx.lr = 0x824FCB20;
	sub_82521CD0(ctx, base);
	// 824FCB20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB24: 480240ED  bl 0x82520c10
	ctx.lr = 0x824FCB28;
	sub_82520C10(ctx, base);
	// 824FCB28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB2C: 48023395  bl 0x8251fec0
	ctx.lr = 0x824FCB30;
	sub_8251FEC0(ctx, base);
	// 824FCB30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB34: 480223B5  bl 0x8251eee8
	ctx.lr = 0x824FCB38;
	sub_8251EEE8(ctx, base);
	// 824FCB38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB3C: 4802111D  bl 0x8251dc58
	ctx.lr = 0x824FCB40;
	sub_8251DC58(ctx, base);
	// 824FCB40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB44: 480B9DAD  bl 0x825b68f0
	ctx.lr = 0x824FCB48;
	sub_825B68F0(ctx, base);
	// 824FCB48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB4C: 4801F935  bl 0x8251c480
	ctx.lr = 0x824FCB50;
	sub_8251C480(ctx, base);
	// 824FCB50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB54: 4801E605  bl 0x8251b158
	ctx.lr = 0x824FCB58;
	sub_8251B158(ctx, base);
	// 824FCB58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB5C: 48016445  bl 0x82512fa0
	ctx.lr = 0x824FCB60;
	sub_82512FA0(ctx, base);
	// 824FCB60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FCB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FCB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FCB6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FCB70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FCB78 size=220
    let mut pc: u32 = 0x824FCB78;
    'dispatch: loop {
        match pc {
            0x824FCB78 => {
    //   block [0x824FCB78..0x824FCC54)
	// 824FCB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FCB7C: 48038541  bl 0x825350bc
	ctx.lr = 0x824FCB80;
	sub_82535080(ctx, base);
	// 824FCB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FCB84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FCB88: 4802BB61  bl 0x825286e8
	ctx.lr = 0x824FCB8C;
	sub_825286E8(ctx, base);
	// 824FCB8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB90: 4802B6B1  bl 0x82528240
	ctx.lr = 0x824FCB94;
	sub_82528240(ctx, base);
	// 824FCB94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB98: 4802A089  bl 0x82526c20
	ctx.lr = 0x824FCB9C;
	sub_82526C20(ctx, base);
	// 824FCB9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBA0: 4BFFFE61  bl 0x824fca00
	ctx.lr = 0x824FCBA4;
	sub_824FCA00(ctx, base);
	// 824FCBA4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824FCBA8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824FCBAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBB0: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 824FCBB4: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCBB8: 48002571  bl 0x824ff128
	ctx.lr = 0x824FCBBC;
	sub_824FF128(ctx, base);
	// 824FCBBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBC0: 48029469  bl 0x82526028
	ctx.lr = 0x824FCBC4;
	sub_82526028(ctx, base);
	// 824FCBC4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 824FCBC8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824FCBCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBD0: 9BC10050  stb r30, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u8 ) };
	// 824FCBD4: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCBD8: 48002551  bl 0x824ff128
	ctx.lr = 0x824FCBDC;
	sub_824FF128(ctx, base);
	// 824FCBDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBE0: 4801D809  bl 0x8251a3e8
	ctx.lr = 0x824FCBE4;
	sub_8251A3E8(ctx, base);
	// 824FCBE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBE8: 4801C581  bl 0x82519168
	ctx.lr = 0x824FCBEC;
	sub_82519168(ctx, base);
	// 824FCBEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBF0: 480B7C51  bl 0x825b4840
	ctx.lr = 0x824FCBF4;
	sub_825B4840(ctx, base);
	// 824FCBF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBF8: 4801A949  bl 0x82517540
	ctx.lr = 0x824FCBFC;
	sub_82517540(ctx, base);
	// 824FCBFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC00: 480B7481  bl 0x825b4080
	ctx.lr = 0x824FCC04;
	sub_825B4080(ctx, base);
	// 824FCC04: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824FCC08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC0C: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 824FCC10: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCC14: 48002515  bl 0x824ff128
	ctx.lr = 0x824FCC18;
	sub_824FF128(ctx, base);
	// 824FCC18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC1C: 480B650D  bl 0x825b3128
	ctx.lr = 0x824FCC20;
	sub_825B3128(ctx, base);
	// 824FCC20: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824FCC24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC28: 9BC10050  stb r30, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u8 ) };
	// 824FCC2C: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCC30: 480024F9  bl 0x824ff128
	ctx.lr = 0x824FCC34;
	sub_824FF128(ctx, base);
	// 824FCC34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC38: 48027CF9  bl 0x82524930
	ctx.lr = 0x824FCC3C;
	sub_82524930(ctx, base);
	// 824FCC3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC40: 480272D1  bl 0x82523f10
	ctx.lr = 0x824FCC44;
	sub_82523F10(ctx, base);
	// 824FCC44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC48: 4BFFFE89  bl 0x824fcad0
	ctx.lr = 0x824FCC4C;
	sub_824FCAD0(ctx, base);
	// 824FCC4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FCC50: 480384BC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FCC58 size=96
    let mut pc: u32 = 0x824FCC58;
    'dispatch: loop {
        match pc {
            0x824FCC58 => {
    //   block [0x824FCC58..0x824FCCB8)
	// 824FCC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FCC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FCC60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FCC64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FCC68: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824FCC6C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FCC70: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 824FCC74: 388B40A0  addi r4, r11, 0x40a0
	ctx.r[4].s64 = ctx.r[11].s64 + 16544;
	// 824FCC78: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FCC7C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCC80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC84: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FCC88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FCC8C: 4E800421  bctrl
	ctx.lr = 0x824FCC90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FCC90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCC94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC98: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FCC9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FCCA0: 4E800421  bctrl
	ctx.lr = 0x824FCCA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FCCA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FCCA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FCCAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FCCB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FCCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCCB8 size=8
    let mut pc: u32 = 0x824FCCB8;
    'dispatch: loop {
        match pc {
            0x824FCCB8 => {
    //   block [0x824FCCB8..0x824FCCC0)
	// 824FCCB8: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 824FCCBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCCC0 size=16
    let mut pc: u32 = 0x824FCCC0;
    'dispatch: loop {
        match pc {
            0x824FCCC0 => {
    //   block [0x824FCCC0..0x824FCCD0)
	// 824FCCC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824FCCC4: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824FCCC8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FCCCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCCD0 size=56
    let mut pc: u32 = 0x824FCCD0;
    'dispatch: loop {
        match pc {
            0x824FCCD0 => {
    //   block [0x824FCCD0..0x824FCD08)
	// 824FCCD0: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FCD08 size=8
    let mut pc: u32 = 0x824FCD08;
    'dispatch: loop {
        match pc {
            0x824FCD08 => {
    //   block [0x824FCD08..0x824FCD10)
	// 824FCD08: D003002C  stfs f0, 0x2c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824FCD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCD10 size=16
    let mut pc: u32 = 0x824FCD10;
    'dispatch: loop {
        match pc {
            0x824FCD10 => {
    //   block [0x824FCD10..0x824FCD20)
	// 824FCD10: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCD20 size=156
    let mut pc: u32 = 0x824FCD20;
    'dispatch: loop {
        match pc {
            0x824FCD20 => {
    //   block [0x824FCD20..0x824FCDBC)
	// 824FCD20: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCDC0 size=204
    let mut pc: u32 = 0x824FCDC0;
    'dispatch: loop {
        match pc {
            0x824FCDC0 => {
    //   block [0x824FCDC0..0x824FCE8C)
	// 824FCDC0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCE90 size=16
    let mut pc: u32 = 0x824FCE90;
    'dispatch: loop {
        match pc {
            0x824FCE90 => {
    //   block [0x824FCE90..0x824FCEA0)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCEA0 size=12
    let mut pc: u32 = 0x824FCEA0;
    'dispatch: loop {
        match pc {
            0x824FCEA0 => {
    //   block [0x824FCEA0..0x824FCEAC)
	// 824FCEA0: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 824FCEA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FCEA8: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCEAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCEAC size=68
    let mut pc: u32 = 0x824FCEAC;
    'dispatch: loop {
        match pc {
            0x824FCEAC => {
    //   block [0x824FCEAC..0x824FCEF0)
	// 824FCEAC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FCEB0: 39030020  addi r8, r3, 0x20
	ctx.r[8].s64 = ctx.r[3].s64 + 32;
	// 824FCEB4: 392A4020  addi r9, r10, 0x4020
	ctx.r[9].s64 = ctx.r[10].s64 + 16416;
	// 824FCEB8: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FCEF0 size=220
    let mut pc: u32 = 0x824FCEF0;
    'dispatch: loop {
        match pc {
            0x824FCEF0 => {
    //   block [0x824FCEF0..0x824FCFCC)
	// 824FCEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FCEF4: 480381C1  bl 0x825350b4
	ctx.lr = 0x824FCEF8;
	sub_82535080(ctx, base);
	// 824FCEF8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FCEFC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824FCF00: 396B4020  addi r11, r11, 0x4020
	ctx.r[11].s64 = ctx.r[11].s64 + 16416;
	// 824FCF04: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FCF08: 390A0020  addi r8, r10, 0x20
	ctx.r[8].s64 = ctx.r[10].s64 + 32;
	// 824FCF0C: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 824FCF10: C00A0010  lfs f0, 0x10(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FCF14: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FCFD0 size=120
    let mut pc: u32 = 0x824FCFD0;
    'dispatch: loop {
        match pc {
            0x824FCFD0 => {
    //   block [0x824FCFD0..0x824FD048)
	// 824FCFD0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FCFD4: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824FCFD8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FCFDC: 396B40B4  addi r11, r11, 0x40b4
	ctx.r[11].s64 = ctx.r[11].s64 + 16564;
	// 824FCFE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824FCFE4: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 824FCFE8: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824FCFEC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FCFF0: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824FCFF4: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824FCFF8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCFFC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824FD000: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FD004: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 824FD008: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FD00C: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 824FD010: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD014: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 824FD018: C0030020  lfs f0, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FD01C: C1A30024  lfs f13, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FD020: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 824FD024: 41980008  blt cr6, 0x824fd02c
	if ctx.cr[6].lt {
	pc = 0x824FD02C; continue 'dispatch;
	}
	// 824FD028: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 824FD02C: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FD030: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824FD034: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824FD038: 40980008  bge cr6, 0x824fd040
	if !ctx.cr[6].lt {
	pc = 0x824FD040; continue 'dispatch;
	}
	// 824FD03C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 824FD040: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824FD044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FD048 size=908
    let mut pc: u32 = 0x824FD048;
    'dispatch: loop {
        match pc {
            0x824FD048 => {
    //   block [0x824FD048..0x824FD3D4)
	// 824FD048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FD04C: 48038069  bl 0x825350b4
	ctx.lr = 0x824FD050;
	sub_82535080(ctx, base);
	// 824FD050: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD054: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 824FD058: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 824FD05C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FD060: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD064: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824FD068: 40980020  bge cr6, 0x824fd088
	if !ctx.cr[6].lt {
	pc = 0x824FD088; continue 'dispatch;
	}
	// 824FD06C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FD070: 392940F4  addi r9, r9, 0x40f4
	ctx.r[9].s64 = ctx.r[9].s64 + 16628;
	// 824FD074: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FD078: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824FD07C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 824FD080: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824FD084: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824FD088: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824FD08C: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FD090: D001FF80  stfs f0, -0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), tmp.u32 ) };
	// 824FD094: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 824FD098: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD3D8 size=8
    let mut pc: u32 = 0x824FD3D8;
    'dispatch: loop {
        match pc {
            0x824FD3D8 => {
    //   block [0x824FD3D8..0x824FD3E0)
	// 824FD3D8: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 824FD3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD3E0 size=20
    let mut pc: u32 = 0x824FD3E0;
    'dispatch: loop {
        match pc {
            0x824FD3E0 => {
    //   block [0x824FD3E0..0x824FD3F4)
	// 824FD3E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD3E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FD3E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD3EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD3F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD3F8 size=8
    let mut pc: u32 = 0x824FD3F8;
    'dispatch: loop {
        match pc {
            0x824FD3F8 => {
    //   block [0x824FD3F8..0x824FD400)
	// 824FD3F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FD3FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD400 size=32
    let mut pc: u32 = 0x824FD400;
    'dispatch: loop {
        match pc {
            0x824FD400 => {
    //   block [0x824FD400..0x824FD420)
	// 824FD400: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FD404: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FD408: 396B40B4  addi r11, r11, 0x40b4
	ctx.r[11].s64 = ctx.r[11].s64 + 16564;
	// 824FD40C: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 824FD410: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824FD414: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FD418: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FD41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD420 size=12
    let mut pc: u32 = 0x824FD420;
    'dispatch: loop {
        match pc {
            0x824FD420 => {
    //   block [0x824FD420..0x824FD42C)
	// 824FD420: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FD424: 386B40B4  addi r3, r11, 0x40b4
	ctx.r[3].s64 = ctx.r[11].s64 + 16564;
	// 824FD428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD430 size=20
    let mut pc: u32 = 0x824FD430;
    'dispatch: loop {
        match pc {
            0x824FD430 => {
    //   block [0x824FD430..0x824FD444)
	// 824FD430: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD434: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FD438: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD43C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD440: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD448 size=8
    let mut pc: u32 = 0x824FD448;
    'dispatch: loop {
        match pc {
            0x824FD448 => {
    //   block [0x824FD448..0x824FD450)
	// 824FD448: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FD44C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD450 size=44
    let mut pc: u32 = 0x824FD450;
    'dispatch: loop {
        match pc {
            0x824FD450 => {
    //   block [0x824FD450..0x824FD47C)
	// 824FD450: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FD454: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FD458: 396B417C  addi r11, r11, 0x417c
	ctx.r[11].s64 = ctx.r[11].s64 + 16764;
	// 824FD45C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824FD460: 394A2A68  addi r10, r10, 0x2a68
	ctx.r[10].s64 = ctx.r[10].s64 + 10856;
	// 824FD464: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 824FD468: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FD46C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824FD470: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824FD474: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824FD478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD480 size=12
    let mut pc: u32 = 0x824FD480;
    'dispatch: loop {
        match pc {
            0x824FD480 => {
    //   block [0x824FD480..0x824FD48C)
	// 824FD480: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FD484: 386B417C  addi r3, r11, 0x417c
	ctx.r[3].s64 = ctx.r[11].s64 + 16764;
	// 824FD488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FD490 size=116
    let mut pc: u32 = 0x824FD490;
    'dispatch: loop {
        match pc {
            0x824FD490 => {
    //   block [0x824FD490..0x824FD504)
	// 824FD490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FD494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FD498: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FD49C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FD4A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FD4A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FD4A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FD4AC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 824FD4B0: 4BFFC241  bl 0x824f96f0
	ctx.lr = 0x824FD4B4;
	sub_824F96F0(ctx, base);
	// 824FD4B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824FD4B8: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FD4BC: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824FD4C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824FD4C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FD4C8: 419A0020  beq cr6, 0x824fd4e8
	if ctx.cr[6].eq {
	pc = 0x824FD4E8; continue 'dispatch;
	}
	// 824FD4CC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD4D0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FD4D4: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FD4D8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FD4DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FD4E0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FD4E4: 4BF66BD5  bl 0x824640b8
	ctx.lr = 0x824FD4E8;
	sub_824640B8(ctx, base);
	// 824FD4E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FD4EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FD4F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FD4F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FD4F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FD4FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FD500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD508 size=60
    let mut pc: u32 = 0x824FD508;
    'dispatch: loop {
        match pc {
            0x824FD508 => {
    //   block [0x824FD508..0x824FD544)
	// 824FD508: 7CAB3278  xor r11, r5, r6
	ctx.r[11].u64 = ctx.r[5].u64 ^ ctx.r[6].u64;
	// 824FD50C: 556B001E  rlwinm r11, r11, 0, 0, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD510: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD514: 409A0050  bne cr6, 0x824fd564
	if !ctx.cr[6].eq {
		sub_824FD564(ctx, base);
		return;
	}
	// 824FD518: 54AB001E  rlwinm r11, r5, 0, 0, 0xf
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD51C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD520: 419A0044  beq cr6, 0x824fd564
	if ctx.cr[6].eq {
		sub_824FD564(ctx, base);
		return;
	}
	// 824FD524: 54CBD97E  srwi r11, r6, 5
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FD528: 7D6B2A78  xor r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[5].u64;
	// 824FD52C: 556B05B4  rlwinm r11, r11, 0, 0x16, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD530: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD534: 409A0010  bne cr6, 0x824fd544
	if !ctx.cr[6].eq {
		sub_824FD544(ctx, base);
		return;
	}
	// 824FD538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FD53C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824FD540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD544(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD544 size=32
    let mut pc: u32 = 0x824FD544;
    'dispatch: loop {
        match pc {
            0x824FD544 => {
    //   block [0x824FD544..0x824FD564)
	// 824FD544: 54ABD97E  srwi r11, r5, 5
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FD548: 7D6B3278  xor r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[6].u64;
	// 824FD54C: 556B05B4  rlwinm r11, r11, 0, 0x16, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD550: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD554: 419AFFE4  beq cr6, 0x824fd538
	if ctx.cr[6].eq {
		sub_824FD508(ctx, base);
		return;
	}
	// 824FD558: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824FD55C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824FD560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD564(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD564 size=56
    let mut pc: u32 = 0x824FD564;
    'dispatch: loop {
        match pc {
            0x824FD564 => {
    //   block [0x824FD564..0x824FD59C)
	// 824FD564: 54CA06FE  clrlwi r10, r6, 0x1b
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x0000001Fu64;
	// 824FD568: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824FD56C: 54AB06FE  clrlwi r11, r5, 0x1b
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x0000001Fu64;
	// 824FD570: 396B000D  addi r11, r11, 0xd
	ctx.r[11].s64 = ctx.r[11].s64 + 13;
	// 824FD574: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FD578: 7D6B202E  lwzx r11, r11, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 824FD57C: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 824FD580: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 824FD584: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 824FD588: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824FD58C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824FD590: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 824FD594: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824FD598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FD5A0 size=104
    let mut pc: u32 = 0x824FD5A0;
    'dispatch: loop {
        match pc {
            0x824FD5A0 => {
    //   block [0x824FD5A0..0x824FD608)
	// 824FD5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FD5A4: 48037B15  bl 0x825350b8
	ctx.lr = 0x824FD5A8;
	sub_82535080(ctx, base);
	// 824FD5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FD5AC: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD5B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FD5B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824FD5B8: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 824FD5BC: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 824FD5C0: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 824FD5C4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD5C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD5CC: 4E800421  bctrl
	ctx.lr = 0x824FD5D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FD5D0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD5D4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824FD5D8: 808100D4  lwz r4, 0xd4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 824FD5DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824FD5E0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD5E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD5E8: 4E800421  bctrl
	ctx.lr = 0x824FD5EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FD5EC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 824FD5F0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824FD5F4: 389DFFF4  addi r4, r29, -0xc
	ctx.r[4].s64 = ctx.r[29].s64 + -12;
	// 824FD5F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FD5FC: 4BFFFF0D  bl 0x824fd508
	ctx.lr = 0x824FD600;
	sub_824FD508(ctx, base);
	// 824FD600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FD604: 48037B04  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FD608 size=76
    let mut pc: u32 = 0x824FD608;
    'dispatch: loop {
        match pc {
            0x824FD608 => {
    //   block [0x824FD608..0x824FD654)
	// 824FD608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FD60C: 48037AB1  bl 0x825350bc
	ctx.lr = 0x824FD610;
	sub_82535080(ctx, base);
	// 824FD610: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FD614: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD618: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FD61C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FD620: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824FD624: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 824FD628: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 824FD62C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD630: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD634: 4E800421  bctrl
	ctx.lr = 0x824FD638;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FD638: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 824FD63C: 389EFFF0  addi r4, r30, -0x10
	ctx.r[4].s64 = ctx.r[30].s64 + -16;
	// 824FD640: 80BD0020  lwz r5, 0x20(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FD644: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FD648: 4BFFFEC1  bl 0x824fd508
	ctx.lr = 0x824FD64C;
	sub_824FD508(ctx, base);
	// 824FD64C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FD650: 48037ABC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD658 size=84
    let mut pc: u32 = 0x824FD658;
    'dispatch: loop {
        match pc {
            0x824FD658 => {
    //   block [0x824FD658..0x824FD6AC)
	// 824FD658: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824FD65C: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	// 824FD660: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 824FD664: 7D0A4830  slw r10, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 824FD668: 7D472038  and r7, r10, r4
	ctx.r[7].u64 = ctx.r[10].u64 & ctx.r[4].u64;
	// 824FD66C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 824FD670: 419A0010  beq cr6, 0x824fd680
	if ctx.cr[6].eq {
	pc = 0x824FD680; continue 'dispatch;
	}
	// 824FD674: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD678: 7CE72B78  or r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[5].u64;
	// 824FD67C: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824FD680: 7D4A2838  and r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[5].u64;
	// 824FD684: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824FD688: 419A0010  beq cr6, 0x824fd698
	if ctx.cr[6].eq {
	pc = 0x824FD698; continue 'dispatch;
	}
	// 824FD68C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD690: 7D4A2378  or r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[4].u64;
	// 824FD694: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FD698: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824FD69C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824FD6A0: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 824FD6A4: 4198FFC0  blt cr6, 0x824fd664
	if ctx.cr[6].lt {
	pc = 0x824FD664; continue 'dispatch;
	}
	// 824FD6A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD6B0 size=56
    let mut pc: u32 = 0x824FD6B0;
    'dispatch: loop {
        match pc {
            0x824FD6B0 => {
    //   block [0x824FD6B0..0x824FD6E8)
	// 824FD6B0: 3964000D  addi r11, r4, 0xd
	ctx.r[11].s64 = ctx.r[4].s64 + 13;
	// 824FD6B4: 3925000D  addi r9, r5, 0xd
	ctx.r[9].s64 = ctx.r[5].s64 + 13;
	// 824FD6B8: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FD6BC: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FD6C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824FD6C4: 7CEA182E  lwzx r7, r10, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 824FD6C8: 7D282830  slw r8, r9, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[9].u32) << ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 824FD6CC: 7D292030  slw r9, r9, r4
	if (ctx.r[4].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[4].u8 & 0x1F) as u32)) as u64;
	}
	// 824FD6D0: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 824FD6D4: 7D0A192E  stwx r8, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[8].u32) };
	// 824FD6D8: 7D4B182E  lwzx r10, r11, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 824FD6DC: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 824FD6E0: 7D4B192E  stwx r10, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	// 824FD6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD6E8 size=56
    let mut pc: u32 = 0x824FD6E8;
    'dispatch: loop {
        match pc {
            0x824FD6E8 => {
    //   block [0x824FD6E8..0x824FD720)
	// 824FD6E8: 3964000D  addi r11, r4, 0xd
	ctx.r[11].s64 = ctx.r[4].s64 + 13;
	// 824FD6EC: 3925000D  addi r9, r5, 0xd
	ctx.r[9].s64 = ctx.r[5].s64 + 13;
	// 824FD6F0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FD6F4: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FD6F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824FD6FC: 7CEA182E  lwzx r7, r10, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 824FD700: 7D282830  slw r8, r9, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[9].u32) << ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 824FD704: 7D292030  slw r9, r9, r4
	if (ctx.r[4].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[4].u8 & 0x1F) as u32)) as u64;
	}
	// 824FD708: 7CE84078  andc r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 & !ctx.r[8].u64;
	// 824FD70C: 7D0A192E  stwx r8, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[8].u32) };
	// 824FD710: 7D4B182E  lwzx r10, r11, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 824FD714: 7D4A4878  andc r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[9].u64;
	// 824FD718: 7D4B192E  stwx r10, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	// 824FD71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD720 size=84
    let mut pc: u32 = 0x824FD720;
    'dispatch: loop {
        match pc {
            0x824FD720 => {
    //   block [0x824FD720..0x824FD774)
	// 824FD720: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824FD724: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	// 824FD728: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 824FD72C: 7D0A4830  slw r10, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 824FD730: 7D472038  and r7, r10, r4
	ctx.r[7].u64 = ctx.r[10].u64 & ctx.r[4].u64;
	// 824FD734: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 824FD738: 419A0010  beq cr6, 0x824fd748
	if ctx.cr[6].eq {
	pc = 0x824FD748; continue 'dispatch;
	}
	// 824FD73C: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD740: 7CE72878  andc r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 & !ctx.r[5].u64;
	// 824FD744: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824FD748: 7D4A2838  and r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[5].u64;
	// 824FD74C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824FD750: 419A0010  beq cr6, 0x824fd760
	if ctx.cr[6].eq {
	pc = 0x824FD760; continue 'dispatch;
	}
	// 824FD754: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD758: 7D4A2078  andc r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[4].u64;
	// 824FD75C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FD760: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824FD764: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824FD768: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 824FD76C: 4198FFC0  blt cr6, 0x824fd72c
	if ctx.cr[6].lt {
	pc = 0x824FD72C; continue 'dispatch;
	}
	// 824FD770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FD778 size=152
    let mut pc: u32 = 0x824FD778;
    'dispatch: loop {
        match pc {
            0x824FD778 => {
    //   block [0x824FD778..0x824FD810)
	// 824FD778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FD77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FD780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FD784: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FD788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FD78C: 48001035  bl 0x824fe7c0
	ctx.lr = 0x824FD790;
	sub_824FE7C0(ctx, base);
	// 824FD790: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FD794: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FD798: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FD79C: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824FD7A0: 3CC08206  lis r6, -0x7dfa
	ctx.r[6].s64 = -2113536000;
	// 824FD7A4: 394A29E4  addi r10, r10, 0x29e4
	ctx.r[10].s64 = ctx.r[10].s64 + 10724;
	// 824FD7A8: 392929D8  addi r9, r9, 0x29d8
	ctx.r[9].s64 = ctx.r[9].s64 + 10712;
	// 824FD7AC: 390829C4  addi r8, r8, 0x29c4
	ctx.r[8].s64 = ctx.r[8].s64 + 10692;
	// 824FD7B0: 38E729B8  addi r7, r7, 0x29b8
	ctx.r[7].s64 = ctx.r[7].s64 + 10680;
	// 824FD7B4: 38C629AC  addi r6, r6, 0x29ac
	ctx.r[6].s64 = ctx.r[6].s64 + 10668;
	// 824FD7B8: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 824FD7BC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FD7C0: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824FD7C4: 397F0034  addi r11, r31, 0x34
	ctx.r[11].s64 = ctx.r[31].s64 + 52;
	// 824FD7C8: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 824FD7CC: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824FD7D0: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 824FD7D4: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 824FD7D8: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 824FD7DC: 90BF0020  stw r5, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[5].u32 ) };
	// 824FD7E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 824FD7E4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FD7E8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824FD7EC: 4200FFF8  bdnz 0x824fd7e4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824FD7E4; continue 'dispatch;
	}
	// 824FD7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FD7F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FD7F8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 824FD7FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FD800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FD804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FD808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FD80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD810 size=64
    let mut pc: u32 = 0x824FD810;
    'dispatch: loop {
        match pc {
            0x824FD810 => {
    //   block [0x824FD810..0x824FD850)
	// 824FD810: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FD814: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FD818: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FD81C: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FD820: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824FD824: 396B29E4  addi r11, r11, 0x29e4
	ctx.r[11].s64 = ctx.r[11].s64 + 10724;
	// 824FD828: 394A29D8  addi r10, r10, 0x29d8
	ctx.r[10].s64 = ctx.r[10].s64 + 10712;
	// 824FD82C: 392929C4  addi r9, r9, 0x29c4
	ctx.r[9].s64 = ctx.r[9].s64 + 10692;
	// 824FD830: 390829B8  addi r8, r8, 0x29b8
	ctx.r[8].s64 = ctx.r[8].s64 + 10680;
	// 824FD834: 38E729AC  addi r7, r7, 0x29ac
	ctx.r[7].s64 = ctx.r[7].s64 + 10668;
	// 824FD838: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FD83C: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824FD840: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FD844: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 824FD848: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 824FD84C: 4BF95424  b 0x82492c70
	sub_82492C70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD850 size=16
    let mut pc: u32 = 0x824FD850;
    'dispatch: loop {
        match pc {
            0x824FD850 => {
    //   block [0x824FD850..0x824FD860)
	// 824FD850: 80C6001C  lwz r6, 0x1c(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28 as u32) ) } as u64;
	// 824FD854: 3884FFF8  addi r4, r4, -8
	ctx.r[4].s64 = ctx.r[4].s64 + -8;
	// 824FD858: 80A5001C  lwz r5, 0x1c(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 824FD85C: 4BFFFCAC  b 0x824fd508
	sub_824FD508(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD860 size=16
    let mut pc: u32 = 0x824FD860;
    'dispatch: loop {
        match pc {
            0x824FD860 => {
    //   block [0x824FD860..0x824FD870)
	// 824FD860: 80C6001C  lwz r6, 0x1c(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28 as u32) ) } as u64;
	// 824FD864: 3884FFEC  addi r4, r4, -0x14
	ctx.r[4].s64 = ctx.r[4].s64 + -20;
	// 824FD868: 80A50024  lwz r5, 0x24(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) } as u64;
	// 824FD86C: 4BFFFC9C  b 0x824fd508
	sub_824FD508(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FD870 size=348
    let mut pc: u32 = 0x824FD870;
    'dispatch: loop {
        match pc {
            0x824FD870 => {
    //   block [0x824FD870..0x824FD9CC)
	// 824FD870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FD874: 48037841  bl 0x825350b4
	ctx.lr = 0x824FD878;
	sub_82535080(ctx, base);
	// 824FD878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FD87C: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD880: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824FD884: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824FD888: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824FD88C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 824FD890: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 824FD894: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD898: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 824FD89C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD8A0: 4E800421  bctrl
	ctx.lr = 0x824FD8A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FD8A4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD8A8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824FD8AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FD8B0: 409A0028  bne cr6, 0x824fd8d8
	if !ctx.cr[6].eq {
	pc = 0x824FD8D8; continue 'dispatch;
	}
	// 824FD8B4: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FD8B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD8BC: 419A0014  beq cr6, 0x824fd8d0
	if ctx.cr[6].eq {
	pc = 0x824FD8D0; continue 'dispatch;
	}
	// 824FD8C0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 824FD8C4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD8C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD8CC: 409AFFF4  bne cr6, 0x824fd8c0
	if !ctx.cr[6].eq {
	pc = 0x824FD8C0; continue 'dispatch;
	}
	// 824FD8D0: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824FD8D4: 48000068  b 0x824fd93c
	pc = 0x824FD93C; continue 'dispatch;
	// 824FD8D8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD8DC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824FD8E0: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 824FD8E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD8E8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD8EC: 396B0044  addi r11, r11, 0x44
	ctx.r[11].s64 = ctx.r[11].s64 + 68;
	// 824FD8F0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FD8F4: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FD8F8: 556905EE  rlwinm r9, r11, 0, 0x17, 0x17
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD8FC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824FD900: 409A0058  bne cr6, 0x824fd958
	if !ctx.cr[6].eq {
	pc = 0x824FD958; continue 'dispatch;
	}
	// 824FD904: 556905AC  rlwinm r9, r11, 0, 0x16, 0x16
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD908: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824FD90C: 409A006C  bne cr6, 0x824fd978
	if !ctx.cr[6].eq {
	pc = 0x824FD978; continue 'dispatch;
	}
	// 824FD910: 556902D6  rlwinm r9, r11, 0, 0xb, 0xb
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD914: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824FD918: 409A007C  bne cr6, 0x824fd994
	if !ctx.cr[6].eq {
	pc = 0x824FD994; continue 'dispatch;
	}
	// 824FD91C: 556B0294  rlwinm r11, r11, 0, 0xa, 0xa
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD920: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FD924: 409A0094  bne cr6, 0x824fd9b8
	if !ctx.cr[6].eq {
	pc = 0x824FD9B8; continue 'dispatch;
	}
	// 824FD928: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824FD92C: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD930: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FD934: 409AFFB0  bne cr6, 0x824fd8e4
	if !ctx.cr[6].eq {
	pc = 0x824FD8E4; continue 'dispatch;
	}
	// 824FD938: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 824FD93C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824FD940: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 824FD944: 389CFFF4  addi r4, r28, -0xc
	ctx.r[4].s64 = ctx.r[28].s64 + -12;
	// 824FD948: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824FD94C: 4BFFFBBD  bl 0x824fd508
	ctx.lr = 0x824FD950;
	sub_824FD508(ctx, base);
	// 824FD950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FD954: 480377B0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 824FD958: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD95C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FD960: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 824FD964: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD968: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD96C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD970: 4E800421  bctrl
	ctx.lr = 0x824FD974;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FD974: 4BFFFFC8  b 0x824fd93c
	pc = 0x824FD93C; continue 'dispatch;
	// 824FD978: 480B5CD1  bl 0x825b3648
	ctx.lr = 0x824FD97C;
	sub_825B3648(ctx, base);
	// 824FD97C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD980: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FD984: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD988: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD98C: 4E800421  bctrl
	ctx.lr = 0x824FD990;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FD990: 4BFFFFAC  b 0x824fd93c
	pc = 0x824FD93C; continue 'dispatch;
	// 824FD994: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD99C: 419A0014  beq cr6, 0x824fd9b0
	if ctx.cr[6].eq {
	pc = 0x824FD9B0; continue 'dispatch;
	}
	// 824FD9A0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 824FD9A4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD9A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD9AC: 409AFFF4  bne cr6, 0x824fd9a0
	if !ctx.cr[6].eq {
	pc = 0x824FD9A0; continue 'dispatch;
	}
	// 824FD9B0: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824FD9B4: 4BFFFF88  b 0x824fd93c
	pc = 0x824FD93C; continue 'dispatch;
	// 824FD9B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824FD9BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824FD9C0: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824FD9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FD9C8: 4803773C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FD9D0 size=320
    let mut pc: u32 = 0x824FD9D0;
    'dispatch: loop {
        match pc {
            0x824FD9D0 => {
    //   block [0x824FD9D0..0x824FDB10)
	// 824FD9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FD9D4: 480376D9  bl 0x825350ac
	ctx.lr = 0x824FD9D8;
	sub_82535080(ctx, base);
	// 824FD9D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FD9DC: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 824FD9E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FD9E4: 7D690774  extsb r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	// 824FD9E8: 7FC92214  add r30, r9, r4
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 824FD9EC: 811F0010  lwz r8, 0x10(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD9F0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD9F4: 7D6829D6  mullw r11, r8, r5
	ctx.r[11].s64 = (ctx.r[8].s32 as i64) * (ctx.r[5].s32 as i64);
	// 824FD9F8: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD9FC: 7F6B5214  add r27, r11, r10
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824FDA00: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 824FDA04: 419A0100  beq cr6, 0x824fdb04
	if ctx.cr[6].eq {
	pc = 0x824FDB04; continue 'dispatch;
	}
	// 824FDA08: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FDA0C: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FDA10: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FDA14: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824FDA18: 7D655A14  add r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 824FDA1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824FDA20: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FDA24: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FDA28: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824FDA2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824FDA30: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FDA34: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824FDA38: 4E800421  bctrl
	ctx.lr = 0x824FDA3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FDA3C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FDA40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FDA44: 419A00C0  beq cr6, 0x824fdb04
	if ctx.cr[6].eq {
	pc = 0x824FDB04; continue 'dispatch;
	}
	// 824FDA48: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FDA4C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 824FDA50: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FDA54: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 824FDA58: 394A9F50  addi r10, r10, -0x60b0
	ctx.r[10].s64 = ctx.r[10].s64 + -24752;
	// 824FDA5C: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 824FDA60: 38EB0020  addi r7, r11, 0x20
	ctx.r[7].s64 = ctx.r[11].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FDB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FDB10 size=300
    let mut pc: u32 = 0x824FDB10;
    'dispatch: loop {
        match pc {
            0x824FDB10 => {
    //   block [0x824FDB10..0x824FDC3C)
	// 824FDB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FDB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FDB18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FDB1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FDB20: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FDB24: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FDB28: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 824FDB2C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 824FDB30: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824FDB34: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FDB38: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FDB3C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FDB40: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824FDB44: 40980020  bge cr6, 0x824fdb64
	if !ctx.cr[6].lt {
	pc = 0x824FDB64; continue 'dispatch;
	}
	// 824FDB48: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FDB4C: 39084280  addi r8, r8, 0x4280
	ctx.r[8].s64 = ctx.r[8].s64 + 17024;
	// 824FDB50: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824FDB54: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 824FDB58: 388A000C  addi r4, r10, 0xc
	ctx.r[4].s64 = ctx.r[10].s64 + 12;
	// 824FDB5C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824FDB60: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 824FDB64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FDB68: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824FDB6C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FDB70: 90E9000C  stw r7, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 824FDB74: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 824FDB78: 91490010  stw r10, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824FDB7C: 409A0008  bne cr6, 0x824fdb84
	if !ctx.cr[6].eq {
	pc = 0x824FDB84; continue 'dispatch;
	}
	// 824FDB80: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 824FDB84: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FDB88: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FDB8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FDB90: 419A001C  beq cr6, 0x824fdbac
	if ctx.cr[6].eq {
	pc = 0x824FDBAC; continue 'dispatch;
	}
	// 824FDB94: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FDB98: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 824FDB9C: 409A0008  bne cr6, 0x824fdba4
	if !ctx.cr[6].eq {
	pc = 0x824FDBA4; continue 'dispatch;
	}
	// 824FDBA0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 824FDBA4: 91690044  stw r11, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 824FDBA8: 48000008  b 0x824fdbb0
	pc = 0x824FDBB0; continue 'dispatch;
	// 824FDBAC: 91490044  stw r10, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 824FDBB0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FDC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FDC40 size=276
    let mut pc: u32 = 0x824FDC40;
    'dispatch: loop {
        match pc {
            0x824FDC40 => {
    //   block [0x824FDC40..0x824FDD54)
	// 824FDC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FDC44: 48037479  bl 0x825350bc
	ctx.lr = 0x824FDC48;
	sub_82535080(ctx, base);
	// 824FDC48: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FDC4C: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FDC50: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 824FDC54: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 824FDC58: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824FDC5C: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FDC60: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FDC64: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FDC68: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824FDC6C: 40980020  bge cr6, 0x824fdc8c
	if !ctx.cr[6].lt {
	pc = 0x824FDC8C; continue 'dispatch;
	}
	// 824FDC70: 3C808206  lis r4, -0x7dfa
	ctx.r[4].s64 = -2113536000;
	// 824FDC74: 3884428C  addi r4, r4, 0x428c
	ctx.r[4].s64 = ctx.r[4].s64 + 17036;
	// 824FDC78: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824FDC7C: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 824FDC80: 3BAA000C  addi r29, r10, 0xc
	ctx.r[29].s64 = ctx.r[10].s64 + 12;
	// 824FDC84: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 824FDC88: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824FDC8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FDC90: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824FDC94: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FDC98: 9109000C  stw r8, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824FDC9C: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 824FDCA0: 91490010  stw r10, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824FDCA4: 409A0008  bne cr6, 0x824fdcac
	if !ctx.cr[6].eq {
	pc = 0x824FDCAC; continue 'dispatch;
	}
	// 824FDCA8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 824FDCAC: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FDCB0: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FDCB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FDCB8: 419A001C  beq cr6, 0x824fdcd4
	if ctx.cr[6].eq {
	pc = 0x824FDCD4; continue 'dispatch;
	}
	// 824FDCBC: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FDCC0: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 824FDCC4: 409A0008  bne cr6, 0x824fdccc
	if !ctx.cr[6].eq {
	pc = 0x824FDCCC; continue 'dispatch;
	}
	// 824FDCC8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 824FDCCC: 91690044  stw r11, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 824FDCD0: 48000008  b 0x824fdcd8
	pc = 0x824FDCD8; continue 'dispatch;
	// 824FDCD4: 91490044  stw r10, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 824FDCD8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FDD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FDD58 size=472
    let mut pc: u32 = 0x824FDD58;
    'dispatch: loop {
        match pc {
            0x824FDD58 => {
    //   block [0x824FDD58..0x824FDF30)
	// 824FDD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FDD5C: 48037341  bl 0x8253509c
	ctx.lr = 0x824FDD60;
	sub_82535080(ctx, base);
	// 824FDD60: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FDD64: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FDD68: 3AA00014  li r21, 0x14
	ctx.r[21].s64 = 20;
	// 824FDD6C: 7D374B78  mr r23, r9
	ctx.r[23].u64 = ctx.r[9].u64;
	// 824FDD70: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 824FDD74: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824FDD78: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824FDD7C: 7D7AA82E  lwzx r11, r26, r21
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 824FDD80: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 824FDD84: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 824FDD88: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 824FDD8C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FDD90: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FDD94: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824FDD98: 40980020  bge cr6, 0x824fddb8
	if !ctx.cr[6].lt {
	pc = 0x824FDDB8; continue 'dispatch;
	}
	// 824FDD9C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FDDA0: 3929429C  addi r9, r9, 0x429c
	ctx.r[9].s64 = ctx.r[9].s64 + 17052;
	// 824FDDA4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FDDA8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824FDDAC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 824FDDB0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824FDDB4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824FDDB8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FDF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FDF30 size=260
    let mut pc: u32 = 0x824FDF30;
    'dispatch: loop {
        match pc {
            0x824FDF30 => {
    //   block [0x824FDF30..0x824FE034)
	// 824FDF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FDF34: 48037181  bl 0x825350b4
	ctx.lr = 0x824FDF38;
	sub_82535080(ctx, base);
	// 824FDF38: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FDF3C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FDF40: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 824FDF44: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824FDF48: 7D7EE82E  lwzx r11, r30, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824FDF4C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FDF50: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FDF54: 7F04F840  cmplw cr6, r4, r31
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[31].u32, &mut ctx.xer);
	// 824FDF58: 40980020  bge cr6, 0x824fdf78
	if !ctx.cr[6].lt {
	pc = 0x824FDF78; continue 'dispatch;
	}
	// 824FDF5C: 3FE08206  lis r31, -0x7dfa
	ctx.r[31].s64 = -2113536000;
	// 824FDF60: 3BFF42AC  addi r31, r31, 0x42ac
	ctx.r[31].s64 = ctx.r[31].s64 + 17068;
	// 824FDF64: 93E40000  stw r31, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 824FDF68: 7FEC42E6  mftb r31, 0x10c
	ctx.r[31].u64 = crate::rt::rdtsc_u64();
	// 824FDF6C: 3B64000C  addi r27, r4, 0xc
	ctx.r[27].s64 = ctx.r[4].s64 + 12;
	// 824FDF70: 93E40004  stw r31, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824FDF74: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 824FDF78: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 824FDF7C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824FDF80: 39670014  addi r11, r7, 0x14
	ctx.r[11].s64 = ctx.r[7].s64 + 20;
	// 824FDF84: 409A0008  bne cr6, 0x824fdf8c
	if !ctx.cr[6].eq {
	pc = 0x824FDF8C; continue 'dispatch;
	}
	// 824FDF88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FDF8C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FDF90: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FDF94: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824FDF98: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FDF9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FDFA0: 419A0010  beq cr6, 0x824fdfb0
	if ctx.cr[6].eq {
	pc = 0x824FDFB0; continue 'dispatch;
	}
	// 824FDFA4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 824FDFA8: 39670010  addi r11, r7, 0x10
	ctx.r[11].s64 = ctx.r[7].s64 + 16;
	// 824FDFAC: 409A0008  bne cr6, 0x824fdfb4
	if !ctx.cr[6].eq {
	pc = 0x824FDFB4; continue 'dispatch;
	}
	// 824FDFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FDFB4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 824FDFB8: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE038 size=8
    let mut pc: u32 = 0x824FE038;
    'dispatch: loop {
        match pc {
            0x824FE038 => {
    //   block [0x824FE038..0x824FE040)
	// 824FE038: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 824FE03C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE040 size=236
    let mut pc: u32 = 0x824FE040;
    'dispatch: loop {
        match pc {
            0x824FE040 => {
    //   block [0x824FE040..0x824FE12C)
	// 824FE040: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FE044: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FE048: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FE04C: 39630100  addi r11, r3, 0x100
	ctx.r[11].s64 = ctx.r[3].s64 + 256;
	// 824FE050: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FE054: 394AFAC4  addi r10, r10, -0x53c
	ctx.r[10].s64 = ctx.r[10].s64 + -1340;
	// 824FE058: 392942D0  addi r9, r9, 0x42d0
	ctx.r[9].s64 = ctx.r[9].s64 + 17104;
	// 824FE05C: 390842C0  addi r8, r8, 0x42c0
	ctx.r[8].s64 = ctx.r[8].s64 + 17088;
	// 824FE060: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 824FE064: B0AB0006  sth r5, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 824FE068: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 824FE06C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824FE070: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FE074: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 824FE078: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE07C: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE080: 409A0008  bne cr6, 0x824fe088
	if !ctx.cr[6].eq {
	pc = 0x824FE088; continue 'dispatch;
	}
	// 824FE084: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FE088: 9147FFF8  stw r10, -8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-8 as u32), ctx.r[10].u32 ) };
	// 824FE08C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE090: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE094: 409A0008  bne cr6, 0x824fe09c
	if !ctx.cr[6].eq {
	pc = 0x824FE09C; continue 'dispatch;
	}
	// 824FE098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FE09C: 9147FFFC  stw r10, -4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-4 as u32), ctx.r[10].u32 ) };
	// 824FE0A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE0A4: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE0A8: 409A0008  bne cr6, 0x824fe0b0
	if !ctx.cr[6].eq {
	pc = 0x824FE0B0; continue 'dispatch;
	}
	// 824FE0AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FE0B0: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FE0B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE0B8: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE0BC: 409A0008  bne cr6, 0x824fe0c4
	if !ctx.cr[6].eq {
	pc = 0x824FE0C4; continue 'dispatch;
	}
	// 824FE0C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FE0C4: 91470004  stw r10, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824FE0C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE0CC: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE0D0: 409A0008  bne cr6, 0x824fe0d8
	if !ctx.cr[6].eq {
	pc = 0x824FE0D8; continue 'dispatch;
	}
	// 824FE0D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FE0D8: 91470008  stw r10, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824FE0DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE0E0: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE0E4: 409A0008  bne cr6, 0x824fe0ec
	if !ctx.cr[6].eq {
	pc = 0x824FE0EC; continue 'dispatch;
	}
	// 824FE0E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FE0EC: 9147000C  stw r10, 0xc(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 824FE0F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE0F4: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE0F8: 409A0008  bne cr6, 0x824fe100
	if !ctx.cr[6].eq {
	pc = 0x824FE100; continue 'dispatch;
	}
	// 824FE0FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FE100: 91470010  stw r10, 0x10(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824FE104: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE108: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE10C: 409A0008  bne cr6, 0x824fe114
	if !ctx.cr[6].eq {
	pc = 0x824FE114; continue 'dispatch;
	}
	// 824FE110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FE114: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 824FE118: 91470014  stw r10, 0x14(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 824FE11C: 38E70020  addi r7, r7, 0x20
	ctx.r[7].s64 = ctx.r[7].s64 + 32;
	// 824FE120: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FE124: 409AFF54  bne cr6, 0x824fe078
	if !ctx.cr[6].eq {
	pc = 0x824FE078; continue 'dispatch;
	}
	// 824FE128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE130 size=48
    let mut pc: u32 = 0x824FE130;
    'dispatch: loop {
        match pc {
            0x824FE130 => {
    //   block [0x824FE130..0x824FE160)
	// 824FE130: 39230100  addi r9, r3, 0x100
	ctx.r[9].s64 = ctx.r[3].s64 + 256;
	// 824FE134: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824FE138: 39090008  addi r8, r9, 8
	ctx.r[8].s64 = ctx.r[9].s64 + 8;
	// 824FE13C: 409A0008  bne cr6, 0x824fe144
	if !ctx.cr[6].eq {
	pc = 0x824FE144; continue 'dispatch;
	}
	// 824FE140: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824FE144: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FE148: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824FE14C: 396BFAC4  addi r11, r11, -0x53c
	ctx.r[11].s64 = ctx.r[11].s64 + -1340;
	// 824FE150: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 824FE154: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FE158: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FE15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FE160 size=188
    let mut pc: u32 = 0x824FE160;
    'dispatch: loop {
        match pc {
            0x824FE160 => {
    //   block [0x824FE160..0x824FE21C)
	// 824FE160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FE164: 48036F55  bl 0x825350b8
	ctx.lr = 0x824FE168;
	sub_82535080(ctx, base);
	// 824FE168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FE16C: 3B85FFFF  addi r28, r5, -1
	ctx.r[28].s64 = ctx.r[5].s64 + -1;
	// 824FE170: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824FE174: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824FE178: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 824FE17C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824FE180: 41980094  blt cr6, 0x824fe214
	if ctx.cr[6].lt {
	pc = 0x824FE214; continue 'dispatch;
	}
	// 824FE184: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE188: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824FE18C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE190: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824FE194: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE198: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE19C: 892B0005  lbz r9, 5(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 824FE1A0: 88CA0005  lbz r6, 5(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(5 as u32) ) } as u64;
	// 824FE1A4: 7D280774  extsb r8, r9
	ctx.r[8].s64 = ctx.r[9].s8 as i64;
	// 824FE1A8: 7CC90774  extsb r9, r6
	ctx.r[9].s64 = ctx.r[6].s8 as i64;
	// 824FE1AC: 7CA85A14  add r5, r8, r11
	ctx.r[5].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 824FE1B0: 7CC95214  add r6, r9, r10
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 824FE1B4: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 824FE1B8: 4E800421  bctrl
	ctx.lr = 0x824FE1BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FE1BC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE1C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE1C4: 419A0040  beq cr6, 0x824fe204
	if ctx.cr[6].eq {
	pc = 0x824FE204; continue 'dispatch;
	}
	// 824FE1C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE1CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FE1D0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE1D4: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE1D8: 894A0004  lbz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE1DC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824FE1E0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 824FE1E4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE1E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824FE1EC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE1F0: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824FE1F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE1F8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE1FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FE200: 4E800421  bctrl
	ctx.lr = 0x824FE204;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FE204: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 824FE208: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 824FE20C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824FE210: 4098FF74  bge cr6, 0x824fe184
	if !ctx.cr[6].lt {
	pc = 0x824FE184; continue 'dispatch;
	}
	// 824FE214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FE218: 48036EF0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FE220 size=116
    let mut pc: u32 = 0x824FE220;
    'dispatch: loop {
        match pc {
            0x824FE220 => {
    //   block [0x824FE220..0x824FE294)
	// 824FE220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FE224: 48036E99  bl 0x825350bc
	ctx.lr = 0x824FE228;
	sub_82535080(ctx, base);
	// 824FE228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FE22C: 3BC5FFFF  addi r30, r5, -1
	ctx.r[30].s64 = ctx.r[5].s64 + -1;
	// 824FE230: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824FE234: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824FE238: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824FE23C: 41980050  blt cr6, 0x824fe28c
	if ctx.cr[6].lt {
	pc = 0x824FE28C; continue 'dispatch;
	}
	// 824FE240: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE244: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FE248: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE24C: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE250: 894A0004  lbz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE254: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824FE258: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 824FE25C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE260: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824FE264: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE268: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824FE26C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE270: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FE274: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FE278: 4E800421  bctrl
	ctx.lr = 0x824FE27C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FE27C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824FE280: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 824FE284: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824FE288: 4098FFB8  bge cr6, 0x824fe240
	if !ctx.cr[6].lt {
	pc = 0x824FE240; continue 'dispatch;
	}
	// 824FE28C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FE290: 48036E7C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FE298 size=1172
    let mut pc: u32 = 0x824FE298;
    'dispatch: loop {
        match pc {
            0x824FE298 => {
    //   block [0x824FE298..0x824FE72C)
	// 824FE298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FE29C: 48036E01  bl 0x8253509c
	ctx.lr = 0x824FE2A0;
	sub_82535080(ctx, base);
	// 824FE2A0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FE2A4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824FE2A8: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 824FE2AC: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE2B0: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE2B4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824FE2B8: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE2BC: 41980008  blt cr6, 0x824fe2c4
	if ctx.cr[6].lt {
	pc = 0x824FE2C4; continue 'dispatch;
	}
	// 824FE2C0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824FE2C4: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 824FE2C8: 40980154  bge cr6, 0x824fe41c
	if !ctx.cr[6].lt {
	pc = 0x824FE41C; continue 'dispatch;
	}
	// 824FE2CC: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 824FE2D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FE2D4: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 824FE2D8: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 824FE2DC: 40990104  ble cr6, 0x824fe3e0
	if !ctx.cr[6].gt {
	pc = 0x824FE3E0; continue 'dispatch;
	}
	// 824FE2E0: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 824FE2E4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 824FE2E8: 80DA0004  lwz r6, 4(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE2EC: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 824FE2F0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 824FE2F4: 4099005C  ble cr6, 0x824fe350
	if !ctx.cr[6].gt {
	pc = 0x824FE350; continue 'dispatch;
	}
	// 824FE2F8: 81380000  lwz r9, 0(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE2FC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE300: 7D054A14  add r8, r5, r9
	ctx.r[8].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 824FE304: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE308: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE30C: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 824FE310: 409A0014  bne cr6, 0x824fe324
	if !ctx.cr[6].eq {
	pc = 0x824FE324; continue 'dispatch;
	}
	// 824FE314: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE318: 83A80004  lwz r29, 4(r8)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE31C: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 824FE320: 419A0030  beq cr6, 0x824fe350
	if ctx.cr[6].eq {
	pc = 0x824FE350; continue 'dispatch;
	}
	// 824FE324: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE328: 7F1E3840  cmplw cr6, r30, r7
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[7].u32, &mut ctx.xer);
	// 824FE32C: 409A0010  bne cr6, 0x824fe33c
	if !ctx.cr[6].eq {
	pc = 0x824FE33C; continue 'dispatch;
	}
	// 824FE330: 83C80004  lwz r30, 4(r8)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE334: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 824FE338: 419A0018  beq cr6, 0x824fe350
	if ctx.cr[6].eq {
	pc = 0x824FE350; continue 'dispatch;
	}
	// 824FE33C: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE340: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824FE344: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824FE348: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824FE34C: 4198FFBC  blt cr6, 0x824fe308
	if ctx.cr[6].lt {
	pc = 0x824FE308; continue 'dispatch;
	}
	// 824FE350: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 824FE354: 409A0034  bne cr6, 0x824fe388
	if !ctx.cr[6].eq {
	pc = 0x824FE388; continue 'dispatch;
	}
	// 824FE358: 7F1F2000  cmpw cr6, r31, r4
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[4].s32, &mut ctx.xer);
	// 824FE35C: 419A0020  beq cr6, 0x824fe37c
	if ctx.cr[6].eq {
	pc = 0x824FE37C; continue 'dispatch;
	}
	// 824FE360: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE364: 7D455A14  add r10, r5, r11
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 824FE368: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 824FE36C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE370: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FE374: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE378: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824FE37C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824FE380: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 824FE384: 48000048  b 0x824fe3cc
	pc = 0x824FE3CC; continue 'dispatch;
	// 824FE388: 3966FFFF  addi r11, r6, -1
	ctx.r[11].s64 = ctx.r[6].s64 + -1;
	// 824FE38C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE390: 917A0004  stw r11, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824FE394: 40980038  bge cr6, 0x824fe3cc
	if !ctx.cr[6].lt {
	pc = 0x824FE3CC; continue 'dispatch;
	}
	// 824FE398: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FE39C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE3A0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824FE3A4: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824FE3A8: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FE3AC: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 824FE3B0: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE3B4: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824FE3B8: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE3BC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824FE3C0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE3C4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE3C8: 4198FFD4  blt cr6, 0x824fe39c
	if ctx.cr[6].lt {
	pc = 0x824FE39C; continue 'dispatch;
	}
	// 824FE3CC: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE3D0: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 824FE3D4: 38A50008  addi r5, r5, 8
	ctx.r[5].s64 = ctx.r[5].s64 + 8;
	// 824FE3D8: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE3DC: 4198FF0C  blt cr6, 0x824fe2e8
	if ctx.cr[6].lt {
	pc = 0x824FE2E8; continue 'dispatch;
	}
	// 824FE3E0: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FE3E4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FE3E8: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824FE3EC: 40980024  bge cr6, 0x824fe410
	if !ctx.cr[6].lt {
	pc = 0x824FE410; continue 'dispatch;
	}
	// 824FE3F0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE3F4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE3F8: 41980008  blt cr6, 0x824fe400
	if ctx.cr[6].lt {
	pc = 0x824FE400; continue 'dispatch;
	}
	// 824FE3FC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824FE400: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824FE404: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824FE408: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 824FE40C: 4BF6FEBD  bl 0x8246e2c8
	ctx.lr = 0x824FE410;
	sub_8246E2C8(ctx, base);
	// 824FE410: 93F80004  stw r31, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824FE414: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824FE418: 48036CD4  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
	// 824FE41C: 4BF6EE45  bl 0x8246d260
	ctx.lr = 0x824FE420;
	sub_8246D260(ctx, base);
	// 824FE420: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE424: 3AE00010  li r23, 0x10
	ctx.r[23].s64 = 16;
	// 824FE428: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FE42C: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 824FE430: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824FE434: 55640036  rlwinm r4, r11, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FE438: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FE43C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 824FE440: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824FE444: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824FE448: 41990010  bgt cr6, 0x824fe458
	if ctx.cr[6].gt {
	pc = 0x824FE458; continue 'dispatch;
	}
	// 824FE44C: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 824FE450: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824FE454: 48000018  b 0x824fe46c
	pc = 0x824FE46C; continue 'dispatch;
	// 824FE458: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE45C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824FE460: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FE464: 4E800421  bctrl
	ctx.lr = 0x824FE468;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FE468: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 824FE46C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824FE470: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 824FE474: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824FE478: 4BF6EDA1  bl 0x8246d218
	ctx.lr = 0x824FE47C;
	sub_8246D218(ctx, base);
	// 824FE47C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE480: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 824FE484: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FE488: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 824FE48C: 409900B0  ble cr6, 0x824fe53c
	if !ctx.cr[6].gt {
	pc = 0x824FE53C; continue 'dispatch;
	}
	// 824FE490: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 824FE494: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE498: 7FDF582A  ldx r30, r31, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	// 824FE49C: FBC10050  std r30, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u64 ) };
	// 824FE4A0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824FE4A4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824FE4A8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824FE4AC: 40990010  ble cr6, 0x824fe4bc
	if !ctx.cr[6].gt {
	pc = 0x824FE4BC; continue 'dispatch;
	}
	// 824FE4B0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 824FE4B4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824FE4B8: EBC10050  ld r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824FE4BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824FE4C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824FE4C4: 4BF6F49D  bl 0x8246d960
	ctx.lr = 0x824FE4C8;
	sub_8246D960(ctx, base);
	// 824FE4C8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824FE4CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FE4D0: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE4D4: 40990008  ble cr6, 0x824fe4dc
	if !ctx.cr[6].gt {
	pc = 0x824FE4DC; continue 'dispatch;
	}
	// 824FE4D8: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 824FE4DC: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 824FE4E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FE4E4: 419A002C  beq cr6, 0x824fe510
	if ctx.cr[6].eq {
	pc = 0x824FE510; continue 'dispatch;
	}
	// 824FE4E8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824FE4EC: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824FE4F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FE4F4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE4F8: 7D2B502A  ldx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) };
	// 824FE4FC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824FE500: 7D2B512A  stdx r9, r11, r10
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u64) };
	// 824FE504: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE508: 7EBF592E  stwx r21, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[21].u32) };
	// 824FE50C: 4800001C  b 0x824fe528
	pc = 0x824FE528; continue 'dispatch;
	// 824FE510: 57AB402E  slwi r11, r29, 8
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE514: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824FE518: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824FE51C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824FE520: 61650001  ori r5, r11, 1
	ctx.r[5].u64 = ctx.r[11].u64 | 1;
	// 824FE524: 4BF6F35D  bl 0x8246d880
	ctx.lr = 0x824FE528;
	sub_8246D880(ctx, base);
	// 824FE528: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE52C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824FE530: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 824FE534: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE538: 4198FF5C  blt cr6, 0x824fe494
	if ctx.cr[6].lt {
	pc = 0x824FE494; continue 'dispatch;
	}
	// 824FE53C: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE540: 7EBCAB78  mr r28, r21
	ctx.r[28].u64 = ctx.r[21].u64;
	// 824FE544: 7EBBAB78  mr r27, r21
	ctx.r[27].u64 = ctx.r[21].u64;
	// 824FE548: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FE54C: 409900E0  ble cr6, 0x824fe62c
	if !ctx.cr[6].gt {
	pc = 0x824FE62C; continue 'dispatch;
	}
	// 824FE550: 7EBEAB78  mr r30, r21
	ctx.r[30].u64 = ctx.r[21].u64;
	// 824FE554: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 824FE558: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE55C: 7D7E582A  ldx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) };
	// 824FE560: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 824FE564: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824FE568: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824FE56C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824FE570: 4099000C  ble cr6, 0x824fe57c
	if !ctx.cr[6].gt {
	pc = 0x824FE57C; continue 'dispatch;
	}
	// 824FE574: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 824FE578: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824FE57C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824FE580: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824FE584: 4BF6F3DD  bl 0x8246d960
	ctx.lr = 0x824FE588;
	sub_8246D960(ctx, base);
	// 824FE588: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824FE58C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824FE590: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824FE594: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824FE598: 40990008  ble cr6, 0x824fe5a0
	if !ctx.cr[6].gt {
	pc = 0x824FE5A0; continue 'dispatch;
	}
	// 824FE59C: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 824FE5A0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824FE5A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FE5A8: 419A004C  beq cr6, 0x824fe5f4
	if ctx.cr[6].eq {
	pc = 0x824FE5F4; continue 'dispatch;
	}
	// 824FE5AC: 7D6A2214  add r11, r10, r4
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 824FE5B0: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824FE5B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FE5B8: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FE5BC: 7D6A482A  ldx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 824FE5C0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 824FE5C4: 57E8063E  clrlwi r8, r31, 0x18
	ctx.r[8].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 824FE5C8: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 824FE5CC: 40990010  ble cr6, 0x824fe5dc
	if !ctx.cr[6].gt {
	pc = 0x824FE5DC; continue 'dispatch;
	}
	// 824FE5D0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824FE5D4: 7D6A492A  stdx r11, r10, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u64) };
	// 824FE5D8: 48000040  b 0x824fe618
	pc = 0x824FE618; continue 'dispatch;
	// 824FE5DC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824FE5E0: 4BF6F3E1  bl 0x8246d9c0
	ctx.lr = 0x824FE5E4;
	sub_8246D9C0(ctx, base);
	// 824FE5E4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE5E8: 57EAD978  rlwinm r10, r31, 0x1b, 5, 0x1c
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x0000001Fu64;
	// 824FE5EC: 7EAA592E  stwx r21, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[21].u32) };
	// 824FE5F0: 48000028  b 0x824fe618
	pc = 0x824FE618; continue 'dispatch;
	// 824FE5F4: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE5F8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824FE5FC: 7D5E5A14  add r10, r30, r11
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 824FE600: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 824FE604: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 824FE608: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE60C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FE610: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE614: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824FE618: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE61C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 824FE620: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 824FE624: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE628: 4198FF30  blt cr6, 0x824fe558
	if ctx.cr[6].lt {
	pc = 0x824FE558; continue 'dispatch;
	}
	// 824FE62C: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FE630: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FE634: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 824FE638: 40980024  bge cr6, 0x824fe65c
	if !ctx.cr[6].lt {
	pc = 0x824FE65C; continue 'dispatch;
	}
	// 824FE63C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE640: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE644: 41980008  blt cr6, 0x824fe64c
	if ctx.cr[6].lt {
	pc = 0x824FE64C; continue 'dispatch;
	}
	// 824FE648: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 824FE64C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824FE650: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824FE654: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 824FE658: 4BF6FC71  bl 0x8246e2c8
	ctx.lr = 0x824FE65C;
	sub_8246E2C8(ctx, base);
	// 824FE65C: 93980004  stw r28, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 824FE660: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 824FE664: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE668: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 824FE66C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FE670: 40990050  ble cr6, 0x824fe6c0
	if !ctx.cr[6].gt {
	pc = 0x824FE6C0; continue 'dispatch;
	}
	// 824FE674: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 824FE678: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 824FE67C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE680: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824FE684: 7CC9582E  lwzx r6, r9, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FE688: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FE68C: 419A0020  beq cr6, 0x824fe6ac
	if ctx.cr[6].eq {
	pc = 0x824FE6AC; continue 'dispatch;
	}
	// 824FE690: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE694: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 824FE698: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824FE69C: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 824FE6A0: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 824FE6A4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE6A8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824FE6AC: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE6B0: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 824FE6B4: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FE6B8: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE6BC: 4198FFC0  blt cr6, 0x824fe67c
	if ctx.cr[6].lt {
	pc = 0x824FE67C; continue 'dispatch;
	}
	// 824FE6C0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FE6C4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FE6C8: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824FE6CC: 40980024  bge cr6, 0x824fe6f0
	if !ctx.cr[6].lt {
	pc = 0x824FE6F0; continue 'dispatch;
	}
	// 824FE6D0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE6D4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE6D8: 41980008  blt cr6, 0x824fe6e0
	if ctx.cr[6].lt {
	pc = 0x824FE6E0; continue 'dispatch;
	}
	// 824FE6DC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824FE6E0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824FE6E4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824FE6E8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824FE6EC: 4BF6FBDD  bl 0x8246e2c8
	ctx.lr = 0x824FE6F0;
	sub_8246E2C8(ctx, base);
	// 824FE6F0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824FE6F4: 93FA0004  stw r31, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824FE6F8: 4BF6F151  bl 0x8246d848
	ctx.lr = 0x824FE6FC;
	sub_8246D848(ctx, base);
	// 824FE6FC: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824FE700: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824FE704: 93230020  stw r25, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[25].u32 ) };
	// 824FE708: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824FE70C: 409A0018  bne cr6, 0x824fe724
	if !ctx.cr[6].eq {
	pc = 0x824FE724; continue 'dispatch;
	}
	// 824FE710: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE714: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 824FE718: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FE71C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FE720: 4E800421  bctrl
	ctx.lr = 0x824FE724;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FE724: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824FE728: 480369C4  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE730 size=8
    let mut pc: u32 = 0x824FE730;
    'dispatch: loop {
        match pc {
            0x824FE730 => {
    //   block [0x824FE730..0x824FE738)
	// 824FE730: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 824FE734: 48000004  b 0x824fe738
	sub_824FE738(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FE738 size=124
    let mut pc: u32 = 0x824FE738;
    'dispatch: loop {
        match pc {
            0x824FE738 => {
    //   block [0x824FE738..0x824FE7B4)
	// 824FE738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FE73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FE740: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FE744: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FE748: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FE74C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824FE750: 393F0008  addi r9, r31, 8
	ctx.r[9].s64 = ctx.r[31].s64 + 8;
	// 824FE754: 409A0008  bne cr6, 0x824fe75c
	if !ctx.cr[6].eq {
	pc = 0x824FE75C; continue 'dispatch;
	}
	// 824FE758: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824FE75C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FE760: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824FE764: 396BFAC4  addi r11, r11, -0x53c
	ctx.r[11].s64 = ctx.r[11].s64 + -1340;
	// 824FE768: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 824FE76C: 548807FE  clrlwi r8, r4, 0x1f
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 824FE770: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824FE774: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FE778: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FE77C: 419A0020  beq cr6, 0x824fe79c
	if ctx.cr[6].eq {
	pc = 0x824FE79C; continue 'dispatch;
	}
	// 824FE780: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE784: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FE788: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FE78C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE790: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FE794: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FE798: 4BF65921  bl 0x824640b8
	ctx.lr = 0x824FE79C;
	sub_824640B8(ctx, base);
	// 824FE79C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FE7A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FE7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FE7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FE7AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FE7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE7B8 size=8
    let mut pc: u32 = 0x824FE7B8;
    'dispatch: loop {
        match pc {
            0x824FE7B8 => {
    //   block [0x824FE7B8..0x824FE7C0)
	// 824FE7B8: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 824FE7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE7C0 size=128
    let mut pc: u32 = 0x824FE7C0;
    'dispatch: loop {
        match pc {
            0x824FE7C0 => {
    //   block [0x824FE7C0..0x824FE840)
	// 824FE7C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FE7C4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 824FE7C8: 396BFAB8  addi r11, r11, -0x548
	ctx.r[11].s64 = ctx.r[11].s64 + -1352;
	// 824FE7CC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FE7D0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FE7D4: 394AFAE0  addi r10, r10, -0x520
	ctx.r[10].s64 = ctx.r[10].s64 + -1312;
	// 824FE7D8: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 824FE7DC: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FE7E0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FE7E4: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824FE7E8: 3CC08206  lis r6, -0x7dfa
	ctx.r[6].s64 = -2113536000;
	// 824FE7EC: 3CA08206  lis r5, -0x7dfa
	ctx.r[5].s64 = -2113536000;
	// 824FE7F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FE7F4: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 824FE7F8: 3C808206  lis r4, -0x7dfa
	ctx.r[4].s64 = -2113536000;
	// 824FE7FC: 3929FAF4  addi r9, r9, -0x50c
	ctx.r[9].s64 = ctx.r[9].s64 + -1292;
	// 824FE800: 3908FAD4  addi r8, r8, -0x52c
	ctx.r[8].s64 = ctx.r[8].s64 + -1324;
	// 824FE804: 38E74318  addi r7, r7, 0x4318
	ctx.r[7].s64 = ctx.r[7].s64 + 17176;
	// 824FE808: 38C6430C  addi r6, r6, 0x430c
	ctx.r[6].s64 = ctx.r[6].s64 + 17164;
	// 824FE80C: 38A542F8  addi r5, r5, 0x42f8
	ctx.r[5].s64 = ctx.r[5].s64 + 17144;
	// 824FE810: 396B42EC  addi r11, r11, 0x42ec
	ctx.r[11].s64 = ctx.r[11].s64 + 17132;
	// 824FE814: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 824FE818: 388442E0  addi r4, r4, 0x42e0
	ctx.r[4].s64 = ctx.r[4].s64 + 17120;
	// 824FE81C: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 824FE820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FE824: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824FE828: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 824FE82C: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 824FE830: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824FE834: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 824FE838: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824FE83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE840 size=8
    let mut pc: u32 = 0x824FE840;
    'dispatch: loop {
        match pc {
            0x824FE840 => {
    //   block [0x824FE840..0x824FE848)
	// 824FE840: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 824FE844: 4800000C  b 0x824fe850
	sub_824FE850(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE848 size=8
    let mut pc: u32 = 0x824FE848;
    'dispatch: loop {
        match pc {
            0x824FE848 => {
    //   block [0x824FE848..0x824FE850)
	// 824FE848: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 824FE84C: 48000004  b 0x824fe850
	sub_824FE850(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FE850 size=100
    let mut pc: u32 = 0x824FE850;
    'dispatch: loop {
        match pc {
            0x824FE850 => {
    //   block [0x824FE850..0x824FE8B4)
	// 824FE850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FE854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FE858: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FE85C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FE860: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FE864: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FE868: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FE86C: 4BF94405  bl 0x82492c70
	ctx.lr = 0x824FE870;
	sub_82492C70(ctx, base);
	// 824FE870: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FE874: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE878: 419A0020  beq cr6, 0x824fe898
	if ctx.cr[6].eq {
	pc = 0x824FE898; continue 'dispatch;
	}
	// 824FE87C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE880: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FE884: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FE888: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE88C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FE890: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FE894: 4BF65825  bl 0x824640b8
	ctx.lr = 0x824FE898;
	sub_824640B8(ctx, base);
	// 824FE898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FE89C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FE8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FE8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FE8A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FE8AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FE8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE8B8 size=64
    let mut pc: u32 = 0x824FE8B8;
    'dispatch: loop {
        match pc {
            0x824FE8B8 => {
    //   block [0x824FE8B8..0x824FE8F8)
	// 824FE8B8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FE8BC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FE8C0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FE8C4: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FE8C8: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824FE8CC: 396BFBA4  addi r11, r11, -0x45c
	ctx.r[11].s64 = ctx.r[11].s64 + -1116;
	// 824FE8D0: 394AFB98  addi r10, r10, -0x468
	ctx.r[10].s64 = ctx.r[10].s64 + -1128;
	// 824FE8D4: 3929FB84  addi r9, r9, -0x47c
	ctx.r[9].s64 = ctx.r[9].s64 + -1148;
	// 824FE8D8: 3908FB78  addi r8, r8, -0x488
	ctx.r[8].s64 = ctx.r[8].s64 + -1160;
	// 824FE8DC: 38E7FB6C  addi r7, r7, -0x494
	ctx.r[7].s64 = ctx.r[7].s64 + -1172;
	// 824FE8E0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FE8E4: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824FE8E8: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FE8EC: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 824FE8F0: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 824FE8F4: 4BF9437C  b 0x82492c70
	sub_82492C70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE8F8 size=32
    let mut pc: u32 = 0x824FE8F8;
    'dispatch: loop {
        match pc {
            0x824FE8F8 => {
    //   block [0x824FE8F8..0x824FE918)
	// 824FE8F8: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE8FC: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE900: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FE904: 396B0044  addi r11, r11, 0x44
	ctx.r[11].s64 = ctx.r[11].s64 + 68;
	// 824FE908: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE90C: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FE910: 5563BFFE  rlwinm r3, r11, 0x17, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 824FE914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FE918 size=376
    let mut pc: u32 = 0x824FE918;
    'dispatch: loop {
        match pc {
            0x824FE918 => {
    //   block [0x824FE918..0x824FEA90)
	// 824FE918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FE91C: 48036795  bl 0x825350b0
	ctx.lr = 0x824FE920;
	sub_82535080(ctx, base);
	// 824FE920: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FE924: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 824FE928: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FE92C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824FE930: 7F6B2214  add r27, r11, r4
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824FE934: 835B0000  lwz r26, 0(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE938: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 824FE93C: 419A0144  beq cr6, 0x824fea80
	if ctx.cr[6].eq {
	pc = 0x824FEA80; continue 'dispatch;
	}
	// 824FE940: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FE944: 54AA083C  slwi r10, r5, 1
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FE948: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE94C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 824FE950: 7D455214  add r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[10].u64;
	// 824FE954: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 824FE958: 555D2036  slwi r29, r10, 4
	ctx.r[29].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 824FE95C: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE960: 7CABEA14  add r5, r11, r29
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 824FE964: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE968: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 824FE96C: 4E800421  bctrl
	ctx.lr = 0x824FE970;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FE970: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE974: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE978: 419A0108  beq cr6, 0x824fea80
	if ctx.cr[6].eq {
	pc = 0x824FEA80; continue 'dispatch;
	}
	// 824FE97C: 83DB0008  lwz r30, 8(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FE980: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 824FE984: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE988: 38BF0010  addi r5, r31, 0x10
	ctx.r[5].s64 = ctx.r[31].s64 + 16;
	// 824FE98C: 393E0030  addi r9, r30, 0x30
	ctx.r[9].s64 = ctx.r[30].s64 + 48;
	// 824FE990: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FE994: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 824FE998: 3BBE0010  addi r29, r30, 0x10
	ctx.r[29].s64 = ctx.r[30].s64 + 16;
	// 824FE99C: 3B9E0020  addi r28, r30, 0x20
	ctx.r[28].s64 = ctx.r[30].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FEA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FEA90 size=296
    let mut pc: u32 = 0x824FEA90;
    'dispatch: loop {
        match pc {
            0x824FEA90 => {
    //   block [0x824FEA90..0x824FEBB8)
	// 824FEA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FEA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FEA98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FEA9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FEAA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FEAA4: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FEAA8: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 824FEAAC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 824FEAB0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824FEAB4: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FEAB8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FEABC: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FEAC0: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824FEAC4: 40980020  bge cr6, 0x824feae4
	if !ctx.cr[6].lt {
	pc = 0x824FEAE4; continue 'dispatch;
	}
	// 824FEAC8: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FEACC: 39084324  addi r8, r8, 0x4324
	ctx.r[8].s64 = ctx.r[8].s64 + 17188;
	// 824FEAD0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824FEAD4: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 824FEAD8: 388A000C  addi r4, r10, 0xc
	ctx.r[4].s64 = ctx.r[10].s64 + 12;
	// 824FEADC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824FEAE0: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 824FEAE4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FEAE8: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824FEAEC: 90E9000C  stw r7, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 824FEAF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FEAF4: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 824FEAF8: 409A0008  bne cr6, 0x824feb00
	if !ctx.cr[6].eq {
	pc = 0x824FEB00; continue 'dispatch;
	}
	// 824FEAFC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 824FEB00: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FEB04: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FEB08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FEB0C: 419A001C  beq cr6, 0x824feb28
	if ctx.cr[6].eq {
	pc = 0x824FEB28; continue 'dispatch;
	}
	// 824FEB10: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FEB14: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 824FEB18: 409A0008  bne cr6, 0x824feb20
	if !ctx.cr[6].eq {
	pc = 0x824FEB20; continue 'dispatch;
	}
	// 824FEB1C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 824FEB20: 91690034  stw r11, 0x34(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 824FEB24: 48000008  b 0x824feb2c
	pc = 0x824FEB2C; continue 'dispatch;
	// 824FEB28: 91490034  stw r10, 0x34(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 824FEB2C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FEBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FEBB8 size=260
    let mut pc: u32 = 0x824FEBB8;
    'dispatch: loop {
        match pc {
            0x824FEBB8 => {
    //   block [0x824FEBB8..0x824FECBC)
	// 824FEBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FEBBC: 48036501  bl 0x825350bc
	ctx.lr = 0x824FEBC0;
	sub_82535080(ctx, base);
	// 824FEBC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FEBC4: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FEBC8: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 824FEBCC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 824FEBD0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824FEBD4: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FEBD8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FEBDC: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FEBE0: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824FEBE4: 40980020  bge cr6, 0x824fec04
	if !ctx.cr[6].lt {
	pc = 0x824FEC04; continue 'dispatch;
	}
	// 824FEBE8: 3C808206  lis r4, -0x7dfa
	ctx.r[4].s64 = -2113536000;
	// 824FEBEC: 38844334  addi r4, r4, 0x4334
	ctx.r[4].s64 = ctx.r[4].s64 + 17204;
	// 824FEBF0: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824FEBF4: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 824FEBF8: 3BAA000C  addi r29, r10, 0xc
	ctx.r[29].s64 = ctx.r[10].s64 + 12;
	// 824FEBFC: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 824FEC00: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824FEC04: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FEC08: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824FEC0C: 9109000C  stw r8, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824FEC10: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 824FEC14: 409A0008  bne cr6, 0x824fec1c
	if !ctx.cr[6].eq {
	pc = 0x824FEC1C; continue 'dispatch;
	}
	// 824FEC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FEC1C: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FEC20: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FEC24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FEC28: 419A0010  beq cr6, 0x824fec38
	if ctx.cr[6].eq {
	pc = 0x824FEC38; continue 'dispatch;
	}
	// 824FEC2C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FEC30: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 824FEC34: 409A0008  bne cr6, 0x824fec3c
	if !ctx.cr[6].eq {
	pc = 0x824FEC3C; continue 'dispatch;
	}
	// 824FEC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FEC3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FEC40: 91690034  stw r11, 0x34(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FECC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FECC0 size=468
    let mut pc: u32 = 0x824FECC0;
    'dispatch: loop {
        match pc {
            0x824FECC0 => {
    //   block [0x824FECC0..0x824FEE94)
	// 824FECC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FECC4: 480363DD  bl 0x825350a0
	ctx.lr = 0x824FECC8;
	sub_82535080(ctx, base);
	// 824FECC8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FECCC: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FECD0: 3AC00014  li r22, 0x14
	ctx.r[22].s64 = 20;
	// 824FECD4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 824FECD8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824FECDC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824FECE0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 824FECE4: 7D79B02E  lwzx r11, r25, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824FECE8: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 824FECEC: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 824FECF0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FECF4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FECF8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824FECFC: 40980020  bge cr6, 0x824fed1c
	if !ctx.cr[6].lt {
	pc = 0x824FED1C; continue 'dispatch;
	}
	// 824FED00: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FED04: 39294344  addi r9, r9, 0x4344
	ctx.r[9].s64 = ctx.r[9].s64 + 17220;
	// 824FED08: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FED0C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824FED10: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 824FED14: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824FED18: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824FED1C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FEE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FEE98 size=256
    let mut pc: u32 = 0x824FEE98;
    'dispatch: loop {
        match pc {
            0x824FEE98 => {
    //   block [0x824FEE98..0x824FEF98)
	// 824FEE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FEE9C: 4803621D  bl 0x825350b8
	ctx.lr = 0x824FEEA0;
	sub_82535080(ctx, base);
	// 824FEEA0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FEEA4: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FEEA8: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 824FEEAC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824FEEB0: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FEEB4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FEEB8: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FEEBC: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824FEEC0: 40980020  bge cr6, 0x824feee0
	if !ctx.cr[6].lt {
	pc = 0x824FEEE0; continue 'dispatch;
	}
	// 824FEEC4: 3C808206  lis r4, -0x7dfa
	ctx.r[4].s64 = -2113536000;
	// 824FEEC8: 38844354  addi r4, r4, 0x4354
	ctx.r[4].s64 = ctx.r[4].s64 + 17236;
	// 824FEECC: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824FEED0: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 824FEED4: 3B8A000C  addi r28, r10, 0xc
	ctx.r[28].s64 = ctx.r[10].s64 + 12;
	// 824FEED8: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 824FEEDC: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 824FEEE0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 824FEEE4: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824FEEE8: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FEEEC: 39670014  addi r11, r7, 0x14
	ctx.r[11].s64 = ctx.r[7].s64 + 20;
	// 824FEEF0: 409A0008  bne cr6, 0x824feef8
	if !ctx.cr[6].eq {
	pc = 0x824FEEF8; continue 'dispatch;
	}
	// 824FEEF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FEEF8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FEEFC: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FEF00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FEF04: 419A0010  beq cr6, 0x824fef14
	if ctx.cr[6].eq {
	pc = 0x824FEF14; continue 'dispatch;
	}
	// 824FEF08: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 824FEF0C: 39670010  addi r11, r7, 0x10
	ctx.r[11].s64 = ctx.r[7].s64 + 16;
	// 824FEF10: 409A0008  bne cr6, 0x824fef18
	if !ctx.cr[6].eq {
	pc = 0x824FEF18; continue 'dispatch;
	}
	// 824FEF14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FEF18: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 824FEF1C: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FEF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FEF98 size=16
    let mut pc: u32 = 0x824FEF98;
    'dispatch: loop {
        match pc {
            0x824FEF98 => {
    //   block [0x824FEF98..0x824FEFA8)
	// 824FEF98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FEF9C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 824FEFA0: 99640000  stb r11, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824FEFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FEFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FEFA8 size=4
    let mut pc: u32 = 0x824FEFA8;
    'dispatch: loop {
        match pc {
            0x824FEFA8 => {
    //   block [0x824FEFA8..0x824FEFAC)
	// 824FEFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FEFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FEFB0 size=8
    let mut pc: u32 = 0x824FEFB0;
    'dispatch: loop {
        match pc {
            0x824FEFB0 => {
    //   block [0x824FEFB0..0x824FEFB8)
	// 824FEFB0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 824FEFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FEFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FEFB8 size=368
    let mut pc: u32 = 0x824FEFB8;
    'dispatch: loop {
        match pc {
            0x824FEFB8 => {
    //   block [0x824FEFB8..0x824FF128)
	// 824FEFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FEFBC: 48036101  bl 0x825350bc
	ctx.lr = 0x824FEFC0;
	sub_82535080(ctx, base);
	// 824FEFC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FEFC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FEFC8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 824FEFCC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824FEFD0: 395F0EB0  addi r10, r31, 0xeb0
	ctx.r[10].s64 = ctx.r[31].s64 + 3760;
	// 824FEFD4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 824FEFD8: 3BA00064  li r29, 0x64
	ctx.r[29].s64 = 100;
	// 824FEFDC: 90FF0190  stw r7, 0x190(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[7].u32 ) };
	// 824FEFE0: 90FF0EA0  stw r7, 0xea0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3744 as u32), ctx.r[7].u32 ) };
	// 824FEFE4: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 824FEFE8: 813F1E30  lwz r9, 0x1e30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7728 as u32) ) } as u64;
	// 824FEFEC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824FEFF0: 419A0030  beq cr6, 0x824ff020
	if ctx.cr[6].eq {
	pc = 0x824FF020; continue 'dispatch;
	}
	// 824FEFF4: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824FEFF8: 9BA90002  stb r29, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 824FEFFC: 813F1E34  lwz r9, 0x1e34(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7732 as u32) ) } as u64;
	// 824FF000: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824FF004: 9BA90002  stb r29, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 824FF008: 813F1E38  lwz r9, 0x1e38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 824FF00C: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824FF010: 9BA90002  stb r29, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 824FF014: 813F1E3C  lwz r9, 0x1e3c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 824FF018: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824FF01C: 9BA90002  stb r29, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 824FF020: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 824FF024: 9BCAF2F0  stb r30, -0xd10(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-3344 as u32), ctx.r[30].u8 ) };
	// 824FF028: 9BCA0000  stb r30, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 824FF02C: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 824FF030: 9BCAF6F0  stb r30, -0x910(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-2320 as u32), ctx.r[30].u8 ) };
	// 824FF034: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824FF038: 9BCA0400  stb r30, 0x400(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1024 as u32), ctx.r[30].u8 ) };
	// 824FF03C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824FF040: 409AFFA8  bne cr6, 0x824fefe8
	if !ctx.cr[6].eq {
	pc = 0x824FEFE8; continue 'dispatch;
	}
	// 824FF044: 2F0B0C00  cmpwi cr6, r11, 0xc00
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3072, &mut ctx.xer);
	// 824FF048: 4198FF9C  blt cr6, 0x824fefe4
	if ctx.cr[6].lt {
	pc = 0x824FEFE4; continue 'dispatch;
	}
	// 824FF04C: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 824FF050: 3D408250  lis r10, -0x7db0
	ctx.r[10].s64 = -2108686336;
	// 824FF054: 3D608250  lis r11, -0x7db0
	ctx.r[11].s64 = -2108686336;
	// 824FF058: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 824FF05C: 38C90B28  addi r6, r9, 0xb28
	ctx.r[6].s64 = ctx.r[9].s64 + 2856;
	// 824FF060: 38AA0B20  addi r5, r10, 0xb20
	ctx.r[5].s64 = ctx.r[10].s64 + 2848;
	// 824FF064: 388B0B30  addi r4, r11, 0xb30
	ctx.r[4].s64 = ctx.r[11].s64 + 2864;
	// 824FF068: 39080B38  addi r8, r8, 0xb38
	ctx.r[8].s64 = ctx.r[8].s64 + 2872;
	// 824FF06C: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 824FF070: 3D408250  lis r10, -0x7db0
	ctx.r[10].s64 = -2108686336;
	// 824FF074: 90DF09A4  stw r6, 0x9a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2468 as u32), ctx.r[6].u32 ) };
	// 824FF078: 3D608250  lis r11, -0x7db0
	ctx.r[11].s64 = -2108686336;
	// 824FF07C: 90BF09A8  stw r5, 0x9a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2472 as u32), ctx.r[5].u32 ) };
	// 824FF080: 3929EF98  addi r9, r9, -0x1068
	ctx.r[9].s64 = ctx.r[9].s64 + -4200;
	// 824FF084: 909F09AC  stw r4, 0x9ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2476 as u32), ctx.r[4].u32 ) };
	// 824FF088: 394AEFA8  addi r10, r10, -0x1058
	ctx.r[10].s64 = ctx.r[10].s64 + -4184;
	// 824FF08C: 911F09A0  stw r8, 0x9a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2464 as u32), ctx.r[8].u32 ) };
	// 824FF090: 396BEFB0  addi r11, r11, -0x1050
	ctx.r[11].s64 = ctx.r[11].s64 + -4176;
	// 824FF094: 9BDF09B0  stb r30, 0x9b0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2480 as u32), ctx.r[30].u8 ) };
	// 824FF098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FF09C: 98FF09B1  stb r7, 0x9b1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2481 as u32), ctx.r[7].u8 ) };
	// 824FF0A0: 93DF16B8  stw r30, 0x16b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5816 as u32), ctx.r[30].u32 ) };
	// 824FF0A4: 913F16B0  stw r9, 0x16b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5808 as u32), ctx.r[9].u32 ) };
	// 824FF0A8: 915F16B4  stw r10, 0x16b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5812 as u32), ctx.r[10].u32 ) };
	// 824FF0AC: 93DF16BC  stw r30, 0x16bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5820 as u32), ctx.r[30].u32 ) };
	// 824FF0B0: 93DF16C0  stw r30, 0x16c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5824 as u32), ctx.r[30].u32 ) };
	// 824FF0B4: 93DF16C4  stw r30, 0x16c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5828 as u32), ctx.r[30].u32 ) };
	// 824FF0B8: 93DF16C8  stw r30, 0x16c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5832 as u32), ctx.r[30].u32 ) };
	// 824FF0BC: 93DF16CC  stw r30, 0x16cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5836 as u32), ctx.r[30].u32 ) };
	// 824FF0C0: 93DF16D8  stw r30, 0x16d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5848 as u32), ctx.r[30].u32 ) };
	// 824FF0C4: 93DF16D0  stw r30, 0x16d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5840 as u32), ctx.r[30].u32 ) };
	// 824FF0C8: 93DF16D4  stw r30, 0x16d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5844 as u32), ctx.r[30].u32 ) };
	// 824FF0CC: 917F16DC  stw r11, 0x16dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5852 as u32), ctx.r[11].u32 ) };
	// 824FF0D0: 93DF16F0  stw r30, 0x16f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5872 as u32), ctx.r[30].u32 ) };
	// 824FF0D4: 4802A665  bl 0x82529738
	ctx.lr = 0x824FF0D8;
	sub_82529738(ctx, base);
	// 824FF0D8: 9BDF1E20  stb r30, 0x1e20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7712 as u32), ctx.r[30].u8 ) };
	// 824FF0DC: 817F1E38  lwz r11, 0x1e38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 824FF0E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FF0E4: 419A003C  beq cr6, 0x824ff120
	if ctx.cr[6].eq {
	pc = 0x824FF120; continue 'dispatch;
	}
	// 824FF0E8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 824FF0EC: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 824FF0F0: 813F1E38  lwz r9, 0x1e38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 824FF0F4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824FF0F8: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824FF0FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824FF100: 9BA90002  stb r29, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 824FF104: 813F1E3C  lwz r9, 0x1e3c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 824FF108: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824FF10C: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 824FF110: 9BA90002  stb r29, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 824FF114: 409AFFDC  bne cr6, 0x824ff0f0
	if !ctx.cr[6].eq {
	pc = 0x824FF0F0; continue 'dispatch;
	}
	// 824FF118: 2F0B0C00  cmpwi cr6, r11, 0xc00
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3072, &mut ctx.xer);
	// 824FF11C: 4198FFD0  blt cr6, 0x824ff0ec
	if ctx.cr[6].lt {
	pc = 0x824FF0EC; continue 'dispatch;
	}
	// 824FF120: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FF124: 48035FE8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FF128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FF128 size=8
    let mut pc: u32 = 0x824FF128;
    'dispatch: loop {
        match pc {
            0x824FF128 => {
    //   block [0x824FF128..0x824FF130)
	// 824FF128: 98831E23  stb r4, 0x1e23(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7715 as u32), ctx.r[4].u8 ) };
	// 824FF12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FF130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FF130 size=244
    let mut pc: u32 = 0x824FF130;
    'dispatch: loop {
        match pc {
            0x824FF130 => {
    //   block [0x824FF130..0x824FF224)
	// 824FF130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FF134: 48035F81  bl 0x825350b4
	ctx.lr = 0x824FF138;
	sub_82535080(ctx, base);
	// 824FF138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FF13C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824FF140: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824FF144: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824FF148: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 824FF14C: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FF150: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FF154: 419A0010  beq cr6, 0x824ff164
	if ctx.cr[6].eq {
	pc = 0x824FF164; continue 'dispatch;
	}
	// 824FF158: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FF15C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FF160: B17D0006  sth r11, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824FF164: 576B1838  slwi r11, r27, 3
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FF168: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 824FF16C: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 824FF170: 557F103A  slwi r31, r11, 2
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824FF174: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FF178: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FF17C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FF180: 419A0030  beq cr6, 0x824ff1b0
	if ctx.cr[6].eq {
	pc = 0x824FF1B0; continue 'dispatch;
	}
	// 824FF184: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FF188: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824FF18C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824FF190: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FF194: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824FF198: 409A0018  bne cr6, 0x824ff1b0
	if !ctx.cr[6].eq {
	pc = 0x824FF1B0; continue 'dispatch;
	}
	// 824FF19C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF1A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FF1A4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF1A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FF1AC: 4E800421  bctrl
	ctx.lr = 0x824FF1B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FF1B0: 7FBFF12E  stwx r29, r31, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32), ctx.r[29].u32) };
	// 824FF1B4: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FF1B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FF1BC: 419A0010  beq cr6, 0x824ff1cc
	if ctx.cr[6].eq {
	pc = 0x824FF1CC; continue 'dispatch;
	}
	// 824FF1C0: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FF1C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FF1C8: B17D0006  sth r11, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824FF1CC: 578B1838  slwi r11, r28, 3
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FF1D0: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 824FF1D4: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 824FF1D8: 557F103A  slwi r31, r11, 2
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824FF1DC: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FF1E0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FF1E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FF1E8: 419A0030  beq cr6, 0x824ff218
	if ctx.cr[6].eq {
	pc = 0x824FF218; continue 'dispatch;
	}
	// 824FF1EC: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FF1F0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824FF1F4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824FF1F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FF1FC: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824FF200: 409A0018  bne cr6, 0x824ff218
	if !ctx.cr[6].eq {
	pc = 0x824FF218; continue 'dispatch;
	}
	// 824FF204: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF208: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FF20C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF210: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FF214: 4E800421  bctrl
	ctx.lr = 0x824FF218;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FF218: 7FBFF12E  stwx r29, r31, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32), ctx.r[29].u32) };
	// 824FF21C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FF220: 48035EE4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FF228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FF228 size=248
    let mut pc: u32 = 0x824FF228;
    'dispatch: loop {
        match pc {
            0x824FF228 => {
    //   block [0x824FF228..0x824FF320)
	// 824FF228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FF22C: 48035E8D  bl 0x825350b8
	ctx.lr = 0x824FF230;
	sub_82535080(ctx, base);
	// 824FF230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FF234: 54AB2834  slwi r11, r5, 5
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FF238: 39450003  addi r10, r5, 3
	ctx.r[10].s64 = ctx.r[5].s64 + 3;
	// 824FF23C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824FF240: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FF244: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824FF248: 3BAB000C  addi r29, r11, 0xc
	ctx.r[29].s64 = ctx.r[11].s64 + 12;
	// 824FF24C: 7FCA1A14  add r30, r10, r3
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 824FF250: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 824FF254: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FF258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FF25C: 419A0010  beq cr6, 0x824ff26c
	if ctx.cr[6].eq {
	pc = 0x824FF26C; continue 'dispatch;
	}
	// 824FF260: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FF264: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FF268: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824FF26C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF270: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FF274: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FF278: 419A0030  beq cr6, 0x824ff2a8
	if ctx.cr[6].eq {
	pc = 0x824FF2A8; continue 'dispatch;
	}
	// 824FF27C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FF280: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824FF284: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824FF288: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FF28C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824FF290: 409A0018  bne cr6, 0x824ff2a8
	if !ctx.cr[6].eq {
	pc = 0x824FF2A8; continue 'dispatch;
	}
	// 824FF294: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF298: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FF29C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF2A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FF2A4: 4E800421  bctrl
	ctx.lr = 0x824FF2A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FF2A8: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 824FF2AC: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FF2B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FF2B4: 419A0010  beq cr6, 0x824ff2c4
	if ctx.cr[6].eq {
	pc = 0x824FF2C4; continue 'dispatch;
	}
	// 824FF2B8: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FF2BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FF2C0: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824FF2C4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF2C8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FF2CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FF2D0: 419A0030  beq cr6, 0x824ff300
	if ctx.cr[6].eq {
	pc = 0x824FF300; continue 'dispatch;
	}
	// 824FF2D4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FF2D8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824FF2DC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824FF2E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FF2E4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824FF2E8: 409A0018  bne cr6, 0x824ff300
	if !ctx.cr[6].eq {
	pc = 0x824FF300; continue 'dispatch;
	}
	// 824FF2EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF2F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FF2F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF2F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FF2FC: 4E800421  bctrl
	ctx.lr = 0x824FF300;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FF300: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 824FF304: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 824FF308: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 824FF30C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824FF310: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 824FF314: 409AFF40  bne cr6, 0x824ff254
	if !ctx.cr[6].eq {
	pc = 0x824FF254; continue 'dispatch;
	}
	// 824FF318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FF31C: 48035DEC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FF320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FF320 size=108
    let mut pc: u32 = 0x824FF320;
    'dispatch: loop {
        match pc {
            0x824FF320 => {
    //   block [0x824FF320..0x824FF38C)
	// 824FF320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FF324: 48035D99  bl 0x825350bc
	ctx.lr = 0x824FF328;
	sub_82535080(ctx, base);
	// 824FF328: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FF32C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FF330: 809F1E30  lwz r4, 0x1e30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7728 as u32) ) } as u64;
	// 824FF334: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824FF338: 419A004C  beq cr6, 0x824ff384
	if ctx.cr[6].eq {
	pc = 0x824FF384; continue 'dispatch;
	}
	// 824FF33C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF340: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 824FF344: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824FF348: 4BF64F79  bl 0x824642c0
	ctx.lr = 0x824FF34C;
	sub_824642C0(ctx, base);
	// 824FF34C: 809F1E34  lwz r4, 0x1e34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7732 as u32) ) } as u64;
	// 824FF350: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824FF354: 4BF64F6D  bl 0x824642c0
	ctx.lr = 0x824FF358;
	sub_824642C0(ctx, base);
	// 824FF358: 809F1E38  lwz r4, 0x1e38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 824FF35C: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824FF360: 4BF64F61  bl 0x824642c0
	ctx.lr = 0x824FF364;
	sub_824642C0(ctx, base);
	// 824FF364: 809F1E3C  lwz r4, 0x1e3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 824FF368: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824FF36C: 4BF64F55  bl 0x824642c0
	ctx.lr = 0x824FF370;
	sub_824642C0(ctx, base);
	// 824FF370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FF374: 917F1E30  stw r11, 0x1e30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7728 as u32), ctx.r[11].u32 ) };
	// 824FF378: 917F1E34  stw r11, 0x1e34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7732 as u32), ctx.r[11].u32 ) };
	// 824FF37C: 917F1E38  stw r11, 0x1e38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7736 as u32), ctx.r[11].u32 ) };
	// 824FF380: 917F1E3C  stw r11, 0x1e3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7740 as u32), ctx.r[11].u32 ) };
	// 824FF384: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FF388: 48035D84  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FF390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FF390 size=540
    let mut pc: u32 = 0x824FF390;
    'dispatch: loop {
        match pc {
            0x824FF390 => {
    //   block [0x824FF390..0x824FF5AC)
	// 824FF390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FF394: 48035CED  bl 0x82535080
	ctx.lr = 0x824FF398;
	sub_82535080(ctx, base);
	// 824FF398: 9421FB00  stwu r1, -0x500(r1)
	ea = ctx.r[1].u32.wrapping_add(-1280 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FF39C: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 824FF3A0: 83C10554  lwz r30, 0x554(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1364 as u32) ) } as u64;
	// 824FF3A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824FF3A8: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 824FF3AC: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 824FF3B0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824FF3B4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 824FF3B8: 99741E20  stb r11, 0x1e20(r20)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[20].u32.wrapping_add(7712 as u32), ctx.r[11].u8 ) };
	// 824FF3BC: 7CF53B78  mr r21, r7
	ctx.r[21].u64 = ctx.r[7].u64;
	// 824FF3C0: 81741E28  lwz r11, 0x1e28(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(7720 as u32) ) } as u64;
	// 824FF3C4: 7D0E4378  mr r14, r8
	ctx.r[14].u64 = ctx.r[8].u64;
	// 824FF3C8: 92E10524  stw r23, 0x524(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1316 as u32), ctx.r[23].u32 ) };
	// 824FF3CC: 7D535378  mr r19, r10
	ctx.r[19].u64 = ctx.r[10].u64;
	// 824FF3D0: 93210544  stw r25, 0x544(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1348 as u32), ctx.r[25].u32 ) };
	// 824FF3D4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 824FF3D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FF3DC: 40990094  ble cr6, 0x824ff470
	if !ctx.cr[6].gt {
	pc = 0x824FF470; continue 'dispatch;
	}
	// 824FF3E0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824FF3E4: 81741E24  lwz r11, 0x1e24(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(7716 as u32) ) } as u64;
	// 824FF3E8: 7FEBE214  add r31, r11, r28
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 824FF3EC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FF3F0: 7F0BD000  cmpw cr6, r11, r26
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[26].s32, &mut ctx.xer);
	// 824FF3F4: 409A0030  bne cr6, 0x824ff424
	if !ctx.cr[6].eq {
	pc = 0x824FF424; continue 'dispatch;
	}
	// 824FF3F8: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 824FF3FC: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF400: 7E6A9B78  mr r10, r19
	ctx.r[10].u64 = ctx.r[19].u64;
	// 824FF404: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 824FF408: 7DC87378  mr r8, r14
	ctx.r[8].u64 = ctx.r[14].u64;
	// 824FF40C: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 824FF410: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 824FF414: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824FF418: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824FF41C: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 824FF420: 4BFFFF71  bl 0x824ff390
	ctx.lr = 0x824FF424;
	sub_824FF390(ctx, base);
	// 824FF424: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FF428: 7F0BA800  cmpw cr6, r11, r21
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[21].s32, &mut ctx.xer);
	// 824FF42C: 409A0030  bne cr6, 0x824ff45c
	if !ctx.cr[6].eq {
	pc = 0x824FF45C; continue 'dispatch;
	}
	// 824FF430: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 824FF434: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF438: 7E6A9B78  mr r10, r19
	ctx.r[10].u64 = ctx.r[19].u64;
	// 824FF43C: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 824FF440: 7DC87378  mr r8, r14
	ctx.r[8].u64 = ctx.r[14].u64;
	// 824FF444: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 824FF448: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 824FF44C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824FF450: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824FF454: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 824FF458: 4BFFFF39  bl 0x824ff390
	ctx.lr = 0x824FF45C;
	sub_824FF390(ctx, base);
	// 824FF45C: 81741E28  lwz r11, 0x1e28(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(7720 as u32) ) } as u64;
	// 824FF460: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 824FF464: 3B9C0008  addi r28, r28, 8
	ctx.r[28].s64 = ctx.r[28].s64 + 8;
	// 824FF468: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FF46C: 4198FF78  blt cr6, 0x824ff3e4
	if ctx.cr[6].lt {
	pc = 0x824FF3E4; continue 'dispatch;
	}
	// 824FF470: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 824FF474: 7EB0AB78  mr r16, r21
	ctx.r[16].u64 = ctx.r[21].u64;
	// 824FF478: 393A0001  addi r9, r26, 1
	ctx.r[9].s64 = ctx.r[26].s64 + 1;
	// 824FF47C: 39F50001  addi r15, r21, 1
	ctx.r[15].s64 = ctx.r[21].s64 + 1;
	// 824FF480: 7FD6F378  mr r22, r30
	ctx.r[22].u64 = ctx.r[30].u64;
	// 824FF484: 2F1AFFFF  cmpwi cr6, r26, -1
	ctx.cr[6].compare_i32(ctx.r[26].s32, -1, &mut ctx.xer);
	// 824FF488: 409A0010  bne cr6, 0x824ff498
	if !ctx.cr[6].eq {
	pc = 0x824FF498; continue 'dispatch;
	}
	// 824FF48C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FF490: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 824FF494: 3ADE0001  addi r22, r30, 1
	ctx.r[22].s64 = ctx.r[30].s64 + 1;
	// 824FF498: 2F15FFFF  cmpwi cr6, r21, -1
	ctx.cr[6].compare_i32(ctx.r[21].s32, -1, &mut ctx.xer);
	// 824FF49C: 409A0010  bne cr6, 0x824ff4ac
	if !ctx.cr[6].eq {
	pc = 0x824FF4AC; continue 'dispatch;
	}
	// 824FF4A0: 3A000001  li r16, 1
	ctx.r[16].s64 = 1;
	// 824FF4A4: 39E00020  li r15, 0x20
	ctx.r[15].s64 = 32;
	// 824FF4A8: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 824FF4AC: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824FF4B0: 409800F4  bge cr6, 0x824ff5a4
	if !ctx.cr[6].lt {
	pc = 0x824FF5A4; continue 'dispatch;
	}
	// 824FF4B4: 554B2834  slwi r11, r10, 5
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FF4B8: 7E2A4850  subf r17, r10, r9
	ctx.r[17].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 824FF4BC: 7D4B8214  add r10, r11, r16
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[16].u64;
	// 824FF4C0: 7F0BEA14  add r24, r11, r29
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 824FF4C4: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FF4C8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824FF4CC: 7D6B9A14  add r11, r11, r19
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[19].u64;
	// 824FF4D0: 3A4B0001  addi r18, r11, 1
	ctx.r[18].s64 = ctx.r[11].s64 + 1;
	// 824FF4D4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FF4D8: 3B2B4380  addi r25, r11, 0x4380
	ctx.r[25].s64 = ctx.r[11].s64 + 17280;
	// 824FF4DC: 7E1B8378  mr r27, r16
	ctx.r[27].u64 = ctx.r[16].u64;
	// 824FF4E0: 7F107800  cmpw cr6, r16, r15
	ctx.cr[6].compare_i32(ctx.r[16].s32, ctx.r[15].s32, &mut ctx.xer);
	// 824FF4E4: 409800AC  bge cr6, 0x824ff590
	if !ctx.cr[6].lt {
	pc = 0x824FF590; continue 'dispatch;
	}
	// 824FF4E8: 56F7063E  clrlwi r23, r23, 0x18
	ctx.r[23].u64 = ctx.r[23].u32 as u64 & 0x000000FFu64;
	// 824FF4EC: 7E5F9378  mr r31, r18
	ctx.r[31].u64 = ctx.r[18].u64;
	// 824FF4F0: 2B130000  cmplwi cr6, r19, 0
	ctx.cr[6].compare_u32(ctx.r[19].u32, 0 as u32, &mut ctx.xer);
	// 824FF4F4: 7EF8D9AE  stbx r23, r24, r27
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[24].u32.wrapping_add(ctx.r[27].u32), ctx.r[23].u8) };
	// 824FF4F8: 419A0084  beq cr6, 0x824ff57c
	if ctx.cr[6].eq {
	pc = 0x824FF57C; continue 'dispatch;
	}
	// 824FF4FC: 89741E23  lbz r11, 0x1e23(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[20].u32.wrapping_add(7715 as u32) ) } as u64;
	// 824FF500: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FF504: 419A0068  beq cr6, 0x824ff56c
	if ctx.cr[6].eq {
	pc = 0x824FF56C; continue 'dispatch;
	}
	// 824FF508: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 824FF50C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824FF510: 7F165800  cmpw cr6, r22, r11
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FF514: 40990058  ble cr6, 0x824ff56c
	if !ctx.cr[6].gt {
	pc = 0x824FF56C; continue 'dispatch;
	}
	// 824FF518: 897FFFFF  lbz r11, -1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(-1 as u32) ) } as u64;
	// 824FF51C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 824FF520: 480292F1  bl 0x82528810
	ctx.lr = 0x824FF524;
	sub_82528810(ctx, base);
	// 824FF524: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF528: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824FF52C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 824FF530: 480292E1  bl 0x82528810
	ctx.lr = 0x824FF534;
	sub_82528810(ctx, base);
	// 824FF534: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824FF538: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824FF53C: 480292D5  bl 0x82528810
	ctx.lr = 0x824FF540;
	sub_82528810(ctx, base);
	// 824FF540: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824FF544: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 824FF548: 480292C9  bl 0x82528810
	ctx.lr = 0x824FF54C;
	sub_82528810(ctx, base);
	// 824FF54C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 824FF550: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 824FF554: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 824FF558: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 824FF55C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 824FF560: 388003E8  li r4, 0x3e8
	ctx.r[4].s64 = 1000;
	// 824FF564: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824FF568: 4BF6A9D9  bl 0x82469f40
	ctx.lr = 0x824FF56C;
	sub_82469F40(ctx, base);
	// 824FF56C: 81210544  lwz r9, 0x544(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1348 as u32) ) } as u64;
	// 824FF570: 9ADF0001  stb r22, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[22].u8 ) };
	// 824FF574: 99DFFFFF  stb r14, -1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-1 as u32), ctx.r[14].u8 ) };
	// 824FF578: 993F0000  stb r9, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 824FF57C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 824FF580: 3BFF0003  addi r31, r31, 3
	ctx.r[31].s64 = ctx.r[31].s64 + 3;
	// 824FF584: 7F1B7800  cmpw cr6, r27, r15
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[15].s32, &mut ctx.xer);
	// 824FF588: 4198FF68  blt cr6, 0x824ff4f0
	if ctx.cr[6].lt {
	pc = 0x824FF4F0; continue 'dispatch;
	}
	// 824FF58C: 82E10524  lwz r23, 0x524(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1316 as u32) ) } as u64;
	// 824FF590: 3A31FFFF  addi r17, r17, -1
	ctx.r[17].s64 = ctx.r[17].s64 + -1;
	// 824FF594: 3A520060  addi r18, r18, 0x60
	ctx.r[18].s64 = ctx.r[18].s64 + 96;
	// 824FF598: 3B180020  addi r24, r24, 0x20
	ctx.r[24].s64 = ctx.r[24].s64 + 32;
	// 824FF59C: 2B110000  cmplwi cr6, r17, 0
	ctx.cr[6].compare_u32(ctx.r[17].u32, 0 as u32, &mut ctx.xer);
	// 824FF5A0: 409AFF3C  bne cr6, 0x824ff4dc
	if !ctx.cr[6].eq {
	pc = 0x824FF4DC; continue 'dispatch;
	}
	// 824FF5A4: 38210500  addi r1, r1, 0x500
	ctx.r[1].s64 = ctx.r[1].s64 + 1280;
	// 824FF5A8: 48035B28  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FF5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FF5B0 size=140
    let mut pc: u32 = 0x824FF5B0;
    'dispatch: loop {
        match pc {
            0x824FF5B0 => {
    //   block [0x824FF5B0..0x824FF63C)
	// 824FF5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FF5B4: 48035B01  bl 0x825350b4
	ctx.lr = 0x824FF5B8;
	sub_82535080(ctx, base);
	// 824FF5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FF5BC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824FF5C0: 39450044  addi r10, r5, 0x44
	ctx.r[10].s64 = ctx.r[5].s64 + 68;
	// 824FF5C4: 397D0044  addi r11, r29, 0x44
	ctx.r[11].s64 = ctx.r[29].s64 + 68;
	// 824FF5C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FF5CC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FF5D0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FF5D4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 824FF5D8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824FF5DC: 7D2BF82E  lwzx r9, r11, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824FF5E0: 7D4AF82E  lwzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824FF5E4: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 824FF5E8: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 824FF5EC: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 824FF5F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FF5F4: 40990040  ble cr6, 0x824ff634
	if !ctx.cr[6].gt {
	pc = 0x824FF634; continue 'dispatch;
	}
	// 824FF5F8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824FF5FC: 817F1E24  lwz r11, 0x1e24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7716 as u32) ) } as u64;
	// 824FF600: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824FF604: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FF608: 7F05E800  cmpw cr6, r5, r29
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824FF60C: 409A0014  bne cr6, 0x824ff620
	if !ctx.cr[6].eq {
	pc = 0x824FF620; continue 'dispatch;
	}
	// 824FF610: 38DB0001  addi r6, r27, 1
	ctx.r[6].s64 = ctx.r[27].s64 + 1;
	// 824FF614: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FF618: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FF61C: 4BFFFF95  bl 0x824ff5b0
	ctx.lr = 0x824FF620;
	sub_824FF5B0(ctx, base);
	// 824FF620: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 824FF624: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824FF628: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 824FF62C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FF630: 4198FFCC  blt cr6, 0x824ff5fc
	if ctx.cr[6].lt {
	pc = 0x824FF5FC; continue 'dispatch;
	}
	// 824FF634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FF638: 48035ACC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FF640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FF640 size=1220
    let mut pc: u32 = 0x824FF640;
    'dispatch: loop {
        match pc {
            0x824FF640 => {
    //   block [0x824FF640..0x824FFB04)
	// 824FF640: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 824FF644: C0040008  lfs f0, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FF648: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824FF64C: D0031E44  stfs f0, 0x1e44(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(7748 as u32), tmp.u32 ) };
	// 824FF650: 39631C60  addi r11, r3, 0x1c60
	ctx.r[11].s64 = ctx.r[3].s64 + 7264;
	// 824FF654: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FF658: D0031E40  stfs f0, 0x1e40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(7744 as u32), tmp.u32 ) };
	// 824FF65C: 8924001B  lbz r9, 0x1b(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(27 as u32) ) } as u64;
	// 824FF660: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FF664: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 824FF668: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FF66C: 992B003A  stb r9, 0x3a(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(58 as u32), ctx.r[9].u8 ) };
	// 824FF670: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824FF674: C00ABFFC  lfs f0, -0x4004(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FF678: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 824FF67C: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FF680: D1AB0000  stfs f13, 0(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824FF684: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FF688: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824FF68C: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FF690: D1AB000C  stfs f13, 0xc(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824FF694: 89440019  lbz r10, 0x19(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(25 as u32) ) } as u64;
	// 824FF698: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824FF69C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824FF6A0: C10A24DC  lfs f8, 0x24dc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9436 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 824FF6A4: 419A000C  beq cr6, 0x824ff6b0
	if ctx.cr[6].eq {
	pc = 0x824FF6B0; continue 'dispatch;
	}
	// 824FF6A8: EDA00232  fmuls f13, f0, f8
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[8].f64) as f32) as f64);
	// 824FF6AC: D1AB000C  stfs f13, 0xc(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824FF6B0: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FF6B4: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824FF6B8: 8944001A  lbz r10, 0x1a(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(26 as u32) ) } as u64;
	// 824FF6BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824FF6C0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824FF6C4: C12A2074  lfs f9, 0x2074(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8308 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 824FF6C8: 419A000C  beq cr6, 0x824ff6d4
	if ctx.cr[6].eq {
	pc = 0x824FF6D4; continue 'dispatch;
	}
	// 824FF6CC: EC000272  fmuls f0, f0, f9
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[9].f64) as f32) as f64);
	// 824FF6D0: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824FF6D4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FF6D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824FF6DC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824FF6E0: 38A31DA0  addi r5, r3, 0x1da0
	ctx.r[5].s64 = ctx.r[3].s64 + 7584;
	// 824FF6E4: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 824FF6E8: C1894478  lfs f12, 0x4478(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(17528 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824FF6EC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FF6F0: C00A8CB4  lfs f0, -0x734c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FF6F4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FF6F8: D00B0030  stfs f0, 0x30(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 824FF6FC: 93EB0010  stw r31, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 824FF700: A0E40010  lhz r7, 0x10(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FF704: D18B0024  stfs f12, 0x24(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824FF708: D18B0028  stfs f12, 0x28(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824FF70C: C8094470  lfd f0, 0x4470(r9)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(17520 as u32) ) };
	// 824FF710: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 824FF714: FD600210  fabs f11, f0
	ctx.f[11].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 824FF718: C1AA4468  lfs f13, 0x4468(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(17512 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FF71C: D1AB0014  stfs f13, 0x14(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824FF720: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 824FF724: D1AB0018  stfs f13, 0x18(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824FF728: B0EB0038  sth r7, 0x38(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[7].u16 ) };
	// 824FF72C: D16B002C  stfs f11, 0x2c(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824FF730: C0091850  lfs f0, 0x1850(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FF734: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 824FF738: D00B001C  stfs f0, 0x1c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824FF73C: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824FF740: D00B0034  stfs f0, 0x34(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 824FF744: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 824FF748: E9480000  ld r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 824FF74C: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 824FF750: F9490000  std r10, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 824FF754: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FF758: 4200FFF0  bdnz 0x824ff748
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824FF748; continue 'dispatch;
	}
	// 824FF75C: 38E31CA0  addi r7, r3, 0x1ca0
	ctx.r[7].s64 = ctx.r[3].s64 + 7328;
	// 824FF760: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824FF764: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 824FF768: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 824FF76C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 824FF770: E90A0000  ld r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 824FF774: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824FF778: F9090000  std r8, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 824FF77C: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FF780: 4200FFF0  bdnz 0x824ff770
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824FF770; continue 'dispatch;
	}
	// 824FF784: 39031CE0  addi r8, r3, 0x1ce0
	ctx.r[8].s64 = ctx.r[3].s64 + 7392;
	// 824FF788: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824FF78C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 824FF790: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 824FF794: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 824FF798: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 824FF79C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824FF7A0: F8C90000  std r6, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 824FF7A4: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FF7A8: 4200FFF0  bdnz 0x824ff798
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824FF798; continue 'dispatch;
	}
	// 824FF7AC: 39231D20  addi r9, r3, 0x1d20
	ctx.r[9].s64 = ctx.r[3].s64 + 7456;
	// 824FF7B0: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 824FF7B4: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 824FF7B8: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 824FF7BC: E8CB0000  ld r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 824FF7C0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824FF7C4: F8CA0000  std r6, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 824FF7C8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824FF7CC: 4200FFF0  bdnz 0x824ff7bc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824FF7BC; continue 'dispatch;
	}
	// 824FF7D0: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FF7D4: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 824FF7D8: D005000C  stfs f0, 0xc(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824FF7DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FF7E0: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FF7E4: D0050004  stfs f0, 4(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824FF7E8: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FF7EC: D0050008  stfs f0, 8(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824FF7F0: 89640018  lbz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FF7F4: C1A62090  lfs f13, 0x2090(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8336 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FF7F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FF7FC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824FF800: C00B0828  lfs f0, 0x828(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2088 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FF804: 419A0080  beq cr6, 0x824ff884
	if ctx.cr[6].eq {
	pc = 0x824FF884; continue 'dispatch;
	}
	// 824FF808: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824FF80C: C16B27B4  lfs f11, 0x27b4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10164 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF810: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FF814: D1670014  stfs f11, 0x14(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824FF818: C18B4464  lfs f12, 0x4464(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17508 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824FF81C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824FF820: D1870018  stfs f12, 0x18(r7)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824FF824: C16B3260  lfs f11, 0x3260(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12896 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF828: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FF82C: D1670024  stfs f11, 0x24(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824FF830: C14B4460  lfs f10, 0x4460(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17504 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824FF834: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FF838: D1470028  stfs f10, 0x28(r7)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824FF83C: C16B445C  lfs f11, 0x445c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17500 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF840: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824FF844: D167002C  stfs f11, 0x2c(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824FF848: C14B235C  lfs f10, 0x235c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9052 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824FF84C: D1470030  stfs f10, 0x30(r7)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 824FF850: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF854: ED605824  fdivs f11, f0, f11
	ctx.f[11].f64 = ((ctx.f[0].f64 / ctx.f[11].f64) as f32) as f64;
	// 824FF858: D167001C  stfs f11, 0x1c(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824FF85C: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF860: ED6D5824  fdivs f11, f13, f11
	ctx.f[11].f64 = ((ctx.f[13].f64 / ctx.f[11].f64) as f32) as f64;
	// 824FF864: D1670020  stfs f11, 0x20(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824FF868: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF86C: ED8C5824  fdivs f12, f12, f11
	ctx.f[12].f64 = ((ctx.f[12].f64 / ctx.f[11].f64) as f32) as f64;
	// 824FF870: ED8C0232  fmuls f12, f12, f8
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[8].f64) as f32) as f64);
	// 824FF874: D1870034  stfs f12, 0x34(r7)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 824FF878: A1640012  lhz r11, 0x12(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(18 as u32) ) } as u64;
	// 824FF87C: 91470010  stw r10, 0x10(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824FF880: B1670038  sth r11, 0x38(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(56 as u32), ctx.r[11].u16 ) };
	// 824FF884: 89640018  lbz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FF888: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FF88C: 419A0080  beq cr6, 0x824ff90c
	if ctx.cr[6].eq {
	pc = 0x824FF90C; continue 'dispatch;
	}
	// 824FF890: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824FF894: C16B2048  lfs f11, 0x2048(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8264 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF898: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824FF89C: D1680014  stfs f11, 0x14(r8)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824FF8A0: C18B280C  lfs f12, 0x280c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10252 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824FF8A4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FF8A8: D1880018  stfs f12, 0x18(r8)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824FF8AC: C16B4458  lfs f11, 0x4458(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17496 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF8B0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FF8B4: D1680024  stfs f11, 0x24(r8)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824FF8B8: C14B4454  lfs f10, 0x4454(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17492 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824FF8BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824FF8C0: D1480028  stfs f10, 0x28(r8)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824FF8C4: C16B21B8  lfs f11, 0x21b8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8632 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF8C8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824FF8CC: D168002C  stfs f11, 0x2c(r8)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824FF8D0: C14B2120  lfs f10, 0x2120(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8480 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824FF8D4: D1480030  stfs f10, 0x30(r8)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 824FF8D8: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF8DC: ED605824  fdivs f11, f0, f11
	ctx.f[11].f64 = ((ctx.f[0].f64 / ctx.f[11].f64) as f32) as f64;
	// 824FF8E0: D168001C  stfs f11, 0x1c(r8)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824FF8E4: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF8E8: ED6D5824  fdivs f11, f13, f11
	ctx.f[11].f64 = ((ctx.f[13].f64 / ctx.f[11].f64) as f32) as f64;
	// 824FF8EC: D1680020  stfs f11, 0x20(r8)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824FF8F0: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF8F4: ED8C5824  fdivs f12, f12, f11
	ctx.f[12].f64 = ((ctx.f[12].f64 / ctx.f[11].f64) as f32) as f64;
	// 824FF8F8: ED8C0272  fmuls f12, f12, f9
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[9].f64) as f32) as f64);
	// 824FF8FC: D1880034  stfs f12, 0x34(r8)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 824FF900: A1640014  lhz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 824FF904: 91480010  stw r10, 0x10(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824FF908: B1680038  sth r11, 0x38(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(56 as u32), ctx.r[11].u16 ) };
	// 824FF90C: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824FF910: D1890000  stfs f12, 0(r9)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824FF914: 89640018  lbz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FF918: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FF91C: 419A0080  beq cr6, 0x824ff99c
	if ctx.cr[6].eq {
	pc = 0x824FF99C; continue 'dispatch;
	}
	// 824FF920: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824FF924: C16B2500  lfs f11, 0x2500(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9472 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF928: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FF92C: D1690014  stfs f11, 0x14(r9)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824FF930: C18B4450  lfs f12, 0x4450(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17488 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824FF934: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824FF938: D1890018  stfs f12, 0x18(r9)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824FF93C: C16B29BC  lfs f11, 0x29bc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10684 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF940: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FF944: D1690024  stfs f11, 0x24(r9)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824FF948: C14B444C  lfs f10, 0x444c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17484 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824FF94C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FF950: D1490028  stfs f10, 0x28(r9)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824FF954: C16B4448  lfs f11, 0x4448(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17480 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF958: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FF95C: D169002C  stfs f11, 0x2c(r9)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824FF960: C14B4444  lfs f10, 0x4444(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17476 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824FF964: D1490030  stfs f10, 0x30(r9)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 824FF968: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824FF96C: EC005824  fdivs f0, f0, f11
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[11].f64) as f32) as f64;
	// 824FF970: D009001C  stfs f0, 0x1c(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824FF974: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FF978: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 824FF97C: D0090020  stfs f0, 0x20(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824FF980: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FF984: EC0C0024  fdivs f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[0].f64) as f32) as f64;
	// 824FF988: EC000272  fmuls f0, f0, f9
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[9].f64) as f32) as f64);
	// 824FF98C: D0090034  stfs f0, 0x34(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 824FF990: A1640016  lhz r11, 0x16(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(22 as u32) ) } as u64;
	// 824FF994: 91490010  stw r10, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824FF998: B1690038  sth r11, 0x38(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(56 as u32), ctx.r[11].u16 ) };
	// 824FF99C: 39631D60  addi r11, r3, 0x1d60
	ctx.r[11].s64 = ctx.r[3].s64 + 7520;
	// 824FF9A0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 824FF9A4: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 824FF9A8: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 824FF9AC: E8E90000  ld r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 824FF9B0: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FF9B4: F8E80000  std r7, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 824FF9B8: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 824FF9BC: 4200FFF0  bdnz 0x824ff9ac
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824FF9AC; continue 'dispatch;
	}
	// 824FF9C0: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FF9C4: 39231BB0  addi r9, r3, 0x1bb0
	ctx.r[9].s64 = ctx.r[3].s64 + 7088;
	// 824FF9C8: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824FF9CC: 39000064  li r8, 0x64
	ctx.r[8].s64 = 100;
	// 824FF9D0: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FF9D4: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824FF9D8: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 824FF9DC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 824FF9E0: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 824FF9E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FF9E8: 4200FFF8  bdnz 0x824ff9e0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824FF9E0; continue 'dispatch;
	}
	// 824FF9EC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 824FF9F0: 9BE31BBB  stb r31, 0x1bbb(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7099 as u32), ctx.r[31].u8 ) };
	// 824FF9F4: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 824FF9F8: 9BE31BBC  stb r31, 0x1bbc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7100 as u32), ctx.r[31].u8 ) };
	// 824FF9FC: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 824FFA00: 99431BBD  stb r10, 0x1bbd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7101 as u32), ctx.r[10].u8 ) };
	// 824FFA04: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 824FFA08: 9BE31BC5  stb r31, 0x1bc5(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7109 as u32), ctx.r[31].u8 ) };
	// 824FFA0C: 9BE31BC6  stb r31, 0x1bc6(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7110 as u32), ctx.r[31].u8 ) };
	// 824FFA10: 99631BC3  stb r11, 0x1bc3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7107 as u32), ctx.r[11].u8 ) };
	// 824FFA14: 99231BBE  stb r9, 0x1bbe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7102 as u32), ctx.r[9].u8 ) };
	// 824FFA18: 98E31BBF  stb r7, 0x1bbf(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7103 as u32), ctx.r[7].u8 ) };
	// 824FFA1C: 99231BC0  stb r9, 0x1bc0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7104 as u32), ctx.r[9].u8 ) };
	// 824FFA20: 99031BC2  stb r8, 0x1bc2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7106 as u32), ctx.r[8].u8 ) };
	// 824FFA24: 99631BCD  stb r11, 0x1bcd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7117 as u32), ctx.r[11].u8 ) };
	// 824FFA28: 99431BC7  stb r10, 0x1bc7(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7111 as u32), ctx.r[10].u8 ) };
	// 824FFA2C: 99631BC8  stb r11, 0x1bc8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7112 as u32), ctx.r[11].u8 ) };
	// 824FFA30: 99631BC9  stb r11, 0x1bc9(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7113 as u32), ctx.r[11].u8 ) };
	// 824FFA34: 99631BCA  stb r11, 0x1bca(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7114 as u32), ctx.r[11].u8 ) };
	// 824FFA38: 99631BCC  stb r11, 0x1bcc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7116 as u32), ctx.r[11].u8 ) };
	// 824FFA3C: 99631C0B  stb r11, 0x1c0b(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7179 as u32), ctx.r[11].u8 ) };
	// 824FFA40: 99631C0C  stb r11, 0x1c0c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7180 as u32), ctx.r[11].u8 ) };
	// 824FFA44: 99631C13  stb r11, 0x1c13(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7187 as u32), ctx.r[11].u8 ) };
	// 824FFA48: 99431C0D  stb r10, 0x1c0d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7181 as u32), ctx.r[10].u8 ) };
	// 824FFA4C: 99631C0E  stb r11, 0x1c0e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7182 as u32), ctx.r[11].u8 ) };
	// 824FFA50: 99631C0F  stb r11, 0x1c0f(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7183 as u32), ctx.r[11].u8 ) };
	// 824FFA54: 99631C10  stb r11, 0x1c10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7184 as u32), ctx.r[11].u8 ) };
	// 824FFA58: 99631C12  stb r11, 0x1c12(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7186 as u32), ctx.r[11].u8 ) };
	// 824FFA5C: 99431BCF  stb r10, 0x1bcf(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7119 as u32), ctx.r[10].u8 ) };
	// 824FFA60: 99431BD0  stb r10, 0x1bd0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7120 as u32), ctx.r[10].u8 ) };
	// 824FFA64: 99431BD7  stb r10, 0x1bd7(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7127 as u32), ctx.r[10].u8 ) };
	// 824FFA68: 99431BD1  stb r10, 0x1bd1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7121 as u32), ctx.r[10].u8 ) };
	// 824FFA6C: 99431BD2  stb r10, 0x1bd2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7122 as u32), ctx.r[10].u8 ) };
	// 824FFA70: 99431BD3  stb r10, 0x1bd3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7123 as u32), ctx.r[10].u8 ) };
	// 824FFA74: 99631BD4  stb r11, 0x1bd4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7124 as u32), ctx.r[11].u8 ) };
	// 824FFA78: 99431BD6  stb r10, 0x1bd6(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7126 as u32), ctx.r[10].u8 ) };
	// 824FFA7C: 99231BD9  stb r9, 0x1bd9(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7129 as u32), ctx.r[9].u8 ) };
	// 824FFA80: 99631BDA  stb r11, 0x1bda(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7130 as u32), ctx.r[11].u8 ) };
	// 824FFA84: 99631BE1  stb r11, 0x1be1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7137 as u32), ctx.r[11].u8 ) };
	// 824FFA88: 99431BDB  stb r10, 0x1bdb(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7131 as u32), ctx.r[10].u8 ) };
	// 824FFA8C: 99431BDC  stb r10, 0x1bdc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7132 as u32), ctx.r[10].u8 ) };
	// 824FFA90: 99631BDD  stb r11, 0x1bdd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7133 as u32), ctx.r[11].u8 ) };
	// 824FFA94: 99631BDE  stb r11, 0x1bde(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7134 as u32), ctx.r[11].u8 ) };
	// 824FFA98: 99631BE0  stb r11, 0x1be0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7136 as u32), ctx.r[11].u8 ) };
	// 824FFA9C: 98E31BE3  stb r7, 0x1be3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7139 as u32), ctx.r[7].u8 ) };
	// 824FFAA0: 99631BE4  stb r11, 0x1be4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7140 as u32), ctx.r[11].u8 ) };
	// 824FFAA4: 99631BEB  stb r11, 0x1beb(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7147 as u32), ctx.r[11].u8 ) };
	// 824FFAA8: 99431BE5  stb r10, 0x1be5(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7141 as u32), ctx.r[10].u8 ) };
	// 824FFAAC: 99631BE6  stb r11, 0x1be6(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7142 as u32), ctx.r[11].u8 ) };
	// 824FFAB0: 99631BE7  stb r11, 0x1be7(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7143 as u32), ctx.r[11].u8 ) };
	// 824FFAB4: 99631BE8  stb r11, 0x1be8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7144 as u32), ctx.r[11].u8 ) };
	// 824FFAB8: 99631BEA  stb r11, 0x1bea(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7146 as u32), ctx.r[11].u8 ) };
	// 824FFABC: 99231BED  stb r9, 0x1bed(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7149 as u32), ctx.r[9].u8 ) };
	// 824FFAC0: 99631BEE  stb r11, 0x1bee(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7150 as u32), ctx.r[11].u8 ) };
	// 824FFAC4: 99631BF5  stb r11, 0x1bf5(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7157 as u32), ctx.r[11].u8 ) };
	// 824FFAC8: 99631BEF  stb r11, 0x1bef(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7151 as u32), ctx.r[11].u8 ) };
	// 824FFACC: 99631BF0  stb r11, 0x1bf0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7152 as u32), ctx.r[11].u8 ) };
	// 824FFAD0: 99631BF1  stb r11, 0x1bf1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7153 as u32), ctx.r[11].u8 ) };
	// 824FFAD4: 99631BF2  stb r11, 0x1bf2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7154 as u32), ctx.r[11].u8 ) };
	// 824FFAD8: 99631BF4  stb r11, 0x1bf4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7156 as u32), ctx.r[11].u8 ) };
	// 824FFADC: 99031C01  stb r8, 0x1c01(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7169 as u32), ctx.r[8].u8 ) };
	// 824FFAE0: 99631C02  stb r11, 0x1c02(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7170 as u32), ctx.r[11].u8 ) };
	// 824FFAE4: 99631C09  stb r11, 0x1c09(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7177 as u32), ctx.r[11].u8 ) };
	// 824FFAE8: 99431C03  stb r10, 0x1c03(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7171 as u32), ctx.r[10].u8 ) };
	// 824FFAEC: 99631C04  stb r11, 0x1c04(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7172 as u32), ctx.r[11].u8 ) };
	// 824FFAF0: 99631C05  stb r11, 0x1c05(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7173 as u32), ctx.r[11].u8 ) };
	// 824FFAF4: 99631C06  stb r11, 0x1c06(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7174 as u32), ctx.r[11].u8 ) };
	// 824FFAF8: 99631C08  stb r11, 0x1c08(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7176 as u32), ctx.r[11].u8 ) };
	// 824FFAFC: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 824FFB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FFB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FFB08 size=344
    let mut pc: u32 = 0x824FFB08;
    'dispatch: loop {
        match pc {
            0x824FFB08 => {
    //   block [0x824FFB08..0x824FFC60)
	// 824FFB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FFB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FFB10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FFB14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FFB18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FFB1C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FFB20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FFB24: 392A4480  addi r9, r10, 0x4480
	ctx.r[9].s64 = ctx.r[10].s64 + 17536;
	// 824FFB28: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 824FFB2C: 397F09A0  addi r11, r31, 0x9a0
	ctx.r[11].s64 = ctx.r[31].s64 + 2464;
	// 824FFB30: 3940003F  li r10, 0x3f
	ctx.r[10].s64 = 63;
	// 824FFB34: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 824FFB38: 396B0011  addi r11, r11, 0x11
	ctx.r[11].s64 = ctx.r[11].s64 + 17;
	// 824FFB3C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FFB40: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824FFB44: B0FF0006  sth r7, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 824FFB48: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824FFB4C: 9BCBFFFF  stb r30, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[30].u8 ) };
	// 824FFB50: 9BCB0000  stb r30, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 824FFB54: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 824FFB58: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FFB5C: 4098FFEC  bge cr6, 0x824ffb48
	if !ctx.cr[6].lt {
	pc = 0x824FFB48; continue 'dispatch;
	}
	// 824FFB60: 397F16B0  addi r11, r31, 0x16b0
	ctx.r[11].s64 = ctx.r[31].s64 + 5808;
	// 824FFB64: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 824FFB68: 396B0031  addi r11, r11, 0x31
	ctx.r[11].s64 = ctx.r[11].s64 + 49;
	// 824FFB6C: 9BCBFFFF  stb r30, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[30].u8 ) };
	// 824FFB70: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824FFB74: 9BCB0000  stb r30, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 824FFB78: 9BCB0001  stb r30, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[30].u8 ) };
	// 824FFB7C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FFB80: 93CBFFE7  stw r30, -0x19(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-25 as u32), ctx.r[30].u32 ) };
	// 824FFB84: 93CBFFEB  stw r30, -0x15(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-21 as u32), ctx.r[30].u32 ) };
	// 824FFB88: 93CBFFEF  stw r30, -0x11(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-17 as u32), ctx.r[30].u32 ) };
	// 824FFB8C: 93CBFFF3  stw r30, -0xd(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-13 as u32), ctx.r[30].u32 ) };
	// 824FFB90: 93CBFFF7  stw r30, -9(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-9 as u32), ctx.r[30].u32 ) };
	// 824FFB94: 396B0050  addi r11, r11, 0x50
	ctx.r[11].s64 = ctx.r[11].s64 + 80;
	// 824FFB98: 4098FFD4  bge cr6, 0x824ffb6c
	if !ctx.cr[6].lt {
	pc = 0x824FFB6C; continue 'dispatch;
	}
	// 824FFB9C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 824FFBA0: 93DF1E24  stw r30, 0x1e24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7716 as u32), ctx.r[30].u32 ) };
	// 824FFBA4: 93DF1E28  stw r30, 0x1e28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7720 as u32), ctx.r[30].u32 ) };
	// 824FFBA8: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 824FFBAC: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 824FFBB0: 915F1E2C  stw r10, 0x1e2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7724 as u32), ctx.r[10].u32 ) };
	// 824FFBB4: 93DF1E30  stw r30, 0x1e30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7728 as u32), ctx.r[30].u32 ) };
	// 824FFBB8: 93DF1E34  stw r30, 0x1e34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7732 as u32), ctx.r[30].u32 ) };
	// 824FFBBC: 93DF1E38  stw r30, 0x1e38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7736 as u32), ctx.r[30].u32 ) };
	// 824FFBC0: 93DF1E3C  stw r30, 0x1e3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7740 as u32), ctx.r[30].u32 ) };
	// 824FFBC4: 9BDF1E20  stb r30, 0x1e20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7712 as u32), ctx.r[30].u8 ) };
	// 824FFBC8: 98FF1E23  stb r7, 0x1e23(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7715 as u32), ctx.r[7].u8 ) };
	// 824FFBCC: 93DF0EA0  stw r30, 0xea0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3744 as u32), ctx.r[30].u32 ) };
	// 824FFBD0: 9BDF1E22  stb r30, 0x1e22(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7714 as u32), ctx.r[30].u8 ) };
	// 824FFBD4: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 824FFBD8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 824FFBDC: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 824FFBE0: 419A001C  beq cr6, 0x824ffbfc
	if ctx.cr[6].eq {
	pc = 0x824FFBFC; continue 'dispatch;
	}
	// 824FFBE4: A1450004  lhz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FFBE8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824FFBEC: 419A0010  beq cr6, 0x824ffbfc
	if ctx.cr[6].eq {
	pc = 0x824FFBFC; continue 'dispatch;
	}
	// 824FFBF0: A1450006  lhz r10, 6(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FFBF4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824FFBF8: B1450006  sth r10, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824FFBFC: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 824FFC00: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824FFC04: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824FFC08: 409AFFD0  bne cr6, 0x824ffbd8
	if !ctx.cr[6].eq {
	pc = 0x824FFBD8; continue 'dispatch;
	}
	// 824FFC0C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 824FFC10: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824FFC14: 409AFFC0  bne cr6, 0x824ffbd4
	if !ctx.cr[6].eq {
	pc = 0x824FFBD4; continue 'dispatch;
	}
	// 824FFC18: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 824FFC1C: 395F0110  addi r10, r31, 0x110
	ctx.r[10].s64 = ctx.r[31].s64 + 272;
	// 824FFC20: 7CE95830  slw r9, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 824FFC24: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FFC28: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 824FFC2C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FFC30: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824FFC34: 4198FFEC  blt cr6, 0x824ffc20
	if ctx.cr[6].lt {
	pc = 0x824FFC20; continue 'dispatch;
	}
	// 824FFC38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FFC3C: 4BFFF37D  bl 0x824fefb8
	ctx.lr = 0x824FFC40;
	sub_824FEFB8(ctx, base);
	// 824FFC40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FFC44: 9BDF1E21  stb r30, 0x1e21(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7713 as u32), ctx.r[30].u8 ) };
	// 824FFC48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FFC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FFC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FFC54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FFC58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FFC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FFC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FFC60 size=292
    let mut pc: u32 = 0x824FFC60;
    'dispatch: loop {
        match pc {
            0x824FFC60 => {
    //   block [0x824FFC60..0x824FFD84)
	// 824FFC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FFC64: 48035455  bl 0x825350b8
	ctx.lr = 0x824FFC68;
	sub_82535080(ctx, base);
	// 824FFC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FFC6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824FFC70: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FFC74: 396B4480  addi r11, r11, 0x4480
	ctx.r[11].s64 = ctx.r[11].s64 + 17536;
	// 824FFC78: 809D1E30  lwz r4, 0x1e30(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(7728 as u32) ) } as u64;
	// 824FFC7C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824FFC80: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FFC84: 419A004C  beq cr6, 0x824ffcd0
	if ctx.cr[6].eq {
	pc = 0x824FFCD0; continue 'dispatch;
	}
	// 824FFC88: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FFC8C: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 824FFC90: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FFC94: 4BF6462D  bl 0x824642c0
	ctx.lr = 0x824FFC98;
	sub_824642C0(ctx, base);
	// 824FFC98: 809D1E34  lwz r4, 0x1e34(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(7732 as u32) ) } as u64;
	// 824FFC9C: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FFCA0: 4BF64621  bl 0x824642c0
	ctx.lr = 0x824FFCA4;
	sub_824642C0(ctx, base);
	// 824FFCA4: 809D1E38  lwz r4, 0x1e38(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(7736 as u32) ) } as u64;
	// 824FFCA8: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FFCAC: 4BF64615  bl 0x824642c0
	ctx.lr = 0x824FFCB0;
	sub_824642C0(ctx, base);
	// 824FFCB0: 809D1E3C  lwz r4, 0x1e3c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(7740 as u32) ) } as u64;
	// 824FFCB4: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FFCB8: 4BF64609  bl 0x824642c0
	ctx.lr = 0x824FFCBC;
	sub_824642C0(ctx, base);
	// 824FFCBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FFCC0: 917D1E30  stw r11, 0x1e30(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(7728 as u32), ctx.r[11].u32 ) };
	// 824FFCC4: 917D1E34  stw r11, 0x1e34(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(7732 as u32), ctx.r[11].u32 ) };
	// 824FFCC8: 917D1E38  stw r11, 0x1e38(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(7736 as u32), ctx.r[11].u32 ) };
	// 824FFCCC: 917D1E3C  stw r11, 0x1e3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(7740 as u32), ctx.r[11].u32 ) };
	// 824FFCD0: 397D000C  addi r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 + 12;
	// 824FFCD4: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 824FFCD8: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 824FFCDC: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 824FFCE0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FFCE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FFCE8: 419A003C  beq cr6, 0x824ffd24
	if ctx.cr[6].eq {
	pc = 0x824FFD24; continue 'dispatch;
	}
	// 824FFCEC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FFCF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FFCF4: 419A0030  beq cr6, 0x824ffd24
	if ctx.cr[6].eq {
	pc = 0x824FFD24; continue 'dispatch;
	}
	// 824FFCF8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FFCFC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824FFD00: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824FFD04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FFD08: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824FFD0C: 409A0018  bne cr6, 0x824ffd24
	if !ctx.cr[6].eq {
	pc = 0x824FFD24; continue 'dispatch;
	}
	// 824FFD10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FFD14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FFD18: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FFD1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FFD20: 4E800421  bctrl
	ctx.lr = 0x824FFD24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FFD24: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824FFD28: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824FFD2C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824FFD30: 409AFFB0  bne cr6, 0x824ffce0
	if !ctx.cr[6].eq {
	pc = 0x824FFCE0; continue 'dispatch;
	}
	// 824FFD34: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 824FFD38: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824FFD3C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 824FFD40: 409AFF98  bne cr6, 0x824ffcd8
	if !ctx.cr[6].eq {
	pc = 0x824FFCD8; continue 'dispatch;
	}
	// 824FFD44: 817D1E2C  lwz r11, 0x1e2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(7724 as u32) ) } as u64;
	// 824FFD48: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FFD4C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FFD50: 409A0020  bne cr6, 0x824ffd70
	if !ctx.cr[6].eq {
	pc = 0x824FFD70; continue 'dispatch;
	}
	// 824FFD54: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FFD58: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FFD5C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FFD60: 809D1E24  lwz r4, 0x1e24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(7716 as u32) ) } as u64;
	// 824FFD64: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FFD68: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FFD6C: 4BF6434D  bl 0x824640b8
	ctx.lr = 0x824FFD70;
	sub_824640B8(ctx, base);
	// 824FFD70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824FFD74: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824FFD78: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FFD7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FFD80: 48035388  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FFD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FFD88 size=280
    let mut pc: u32 = 0x824FFD88;
    'dispatch: loop {
        match pc {
            0x824FFD88 => {
    //   block [0x824FFD88..0x824FFEA0)
	// 824FFD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FFD8C: 48035329  bl 0x825350b4
	ctx.lr = 0x824FFD90;
	sub_82535080(ctx, base);
	// 824FFD90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FFD94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FFD98: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824FFD9C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824FFDA0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 824FFDA4: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 824FFDA8: 817F0190  lwz r11, 0x190(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 824FFDAC: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 824FFDB0: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FFDB4: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824FFDB8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FFDBC: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 824FFDC0: 396B09A0  addi r11, r11, 0x9a0
	ctx.r[11].s64 = ctx.r[11].s64 + 2464;
	// 824FFDC4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824FFDC8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FFDCC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824FFDD0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FFDD4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824FFDD8: 4200FFF0  bdnz 0x824ffdc8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824FFDC8; continue 'dispatch;
	}
	// 824FFDDC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 824FFDE0: 815F1E38  lwz r10, 0x1e38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 824FFDE4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 824FFDE8: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 824FFDEC: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 824FFDF0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824FFDF4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FFDF8: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 824FFDFC: 389F0EB0  addi r4, r31, 0xeb0
	ctx.r[4].s64 = ctx.r[31].s64 + 3760;
	// 824FFE00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FFE04: 4BFFF58D  bl 0x824ff390
	ctx.lr = 0x824FFE08;
	sub_824FF390(ctx, base);
	// 824FFE08: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 824FFE0C: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 824FFE10: 815F1E30  lwz r10, 0x1e30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7728 as u32) ) } as u64;
	// 824FFE14: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 824FFE18: 80BF0190  lwz r5, 0x190(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 824FFE1C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824FFE20: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 824FFE24: 389F01A0  addi r4, r31, 0x1a0
	ctx.r[4].s64 = ctx.r[31].s64 + 416;
	// 824FFE28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FFE2C: 4BFFF565  bl 0x824ff390
	ctx.lr = 0x824FFE30;
	sub_824FF390(ctx, base);
	// 824FFE30: 897C0011  lbz r11, 0x11(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(17 as u32) ) } as u64;
	// 824FFE34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FFE38: 419A0054  beq cr6, 0x824ffe8c
	if ctx.cr[6].eq {
	pc = 0x824FFE8C; continue 'dispatch;
	}
	// 824FFE3C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 824FFE40: 815F1E3C  lwz r10, 0x1e3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 824FFE44: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 824FFE48: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 824FFE4C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 824FFE50: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824FFE54: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FFE58: 389F12B0  addi r4, r31, 0x12b0
	ctx.r[4].s64 = ctx.r[31].s64 + 4784;
	// 824FFE5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FFE60: 4BFFF531  bl 0x824ff390
	ctx.lr = 0x824FFE64;
	sub_824FF390(ctx, base);
	// 824FFE64: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 824FFE68: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 824FFE6C: 815F1E34  lwz r10, 0x1e34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7732 as u32) ) } as u64;
	// 824FFE70: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 824FFE74: 80BF0190  lwz r5, 0x190(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 824FFE78: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824FFE7C: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 824FFE80: 389F05A0  addi r4, r31, 0x5a0
	ctx.r[4].s64 = ctx.r[31].s64 + 1440;
	// 824FFE84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FFE88: 4BFFF509  bl 0x824ff390
	ctx.lr = 0x824FFE8C;
	sub_824FF390(ctx, base);
	// 824FFE8C: 817F0190  lwz r11, 0x190(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 824FFE90: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FFE94: 917F0190  stw r11, 0x190(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[11].u32 ) };
	// 824FFE98: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824FFE9C: 48035268  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FFEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FFEA0 size=200
    let mut pc: u32 = 0x824FFEA0;
    'dispatch: loop {
        match pc {
            0x824FFEA0 => {
    //   block [0x824FFEA0..0x824FFF68)
	// 824FFEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FFEA4: 48035211  bl 0x825350b4
	ctx.lr = 0x824FFEA8;
	sub_82535080(ctx, base);
	// 824FFEA8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FFEAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FFEB0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824FFEB4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824FFEB8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 824FFEBC: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 824FFEC0: 817F0190  lwz r11, 0x190(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 824FFEC4: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 824FFEC8: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FFECC: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824FFED0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FFED4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 824FFED8: 396B09A0  addi r11, r11, 0x9a0
	ctx.r[11].s64 = ctx.r[11].s64 + 2464;
	// 824FFEDC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824FFEE0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FFEE4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824FFEE8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FFEEC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824FFEF0: 4200FFF0  bdnz 0x824ffee0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824FFEE0; continue 'dispatch;
	}
	// 824FFEF4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 824FFEF8: 815F1E30  lwz r10, 0x1e30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7728 as u32) ) } as u64;
	// 824FFEFC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 824FFF00: 80BF0190  lwz r5, 0x190(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 824FFF04: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 824FFF08: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 824FFF0C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824FFF10: 389F01A0  addi r4, r31, 0x1a0
	ctx.r[4].s64 = ctx.r[31].s64 + 416;
	// 824FFF14: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 824FFF18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FFF1C: 4BFFF475  bl 0x824ff390
	ctx.lr = 0x824FFF20;
	sub_824FF390(ctx, base);
	// 824FFF20: 897C0011  lbz r11, 0x11(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(17 as u32) ) } as u64;
	// 824FFF24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FFF28: 419A002C  beq cr6, 0x824fff54
	if ctx.cr[6].eq {
	pc = 0x824FFF54; continue 'dispatch;
	}
	// 824FFF2C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 824FFF30: 815F1E34  lwz r10, 0x1e34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7732 as u32) ) } as u64;
	// 824FFF34: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 824FFF38: 80BF0190  lwz r5, 0x190(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 824FFF3C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 824FFF40: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 824FFF44: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824FFF48: 389F05A0  addi r4, r31, 0x5a0
	ctx.r[4].s64 = ctx.r[31].s64 + 1440;
	// 824FFF4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FFF50: 4BFFF441  bl 0x824ff390
	ctx.lr = 0x824FFF54;
	sub_824FF390(ctx, base);
	// 824FFF54: 817F0190  lwz r11, 0x190(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 824FFF58: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FFF5C: 917F0190  stw r11, 0x190(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[11].u32 ) };
	// 824FFF60: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824FFF64: 480351A0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FFF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FFF68 size=468
    let mut pc: u32 = 0x824FFF68;
    'dispatch: loop {
        match pc {
            0x824FFF68 => {
    //   block [0x824FFF68..0x8250013C)
	// 824FFF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FFF6C: 48035141  bl 0x825350ac
	ctx.lr = 0x824FFF70;
	sub_82535080(ctx, base);
	// 824FFF70: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FFF74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FFF78: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 824FFF7C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FFF80: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824FFF84: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824FFF88: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 824FFF8C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 824FFF90: 9B5F1E21  stb r26, 0x1e21(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7713 as u32), ctx.r[26].u8 ) };
	// 824FFF94: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 824FFF98: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824FFF9C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 824FFFA0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824FFFA4: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 824FFFA8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824FFFAC: 4200FFF0  bdnz 0x824fff9c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824FFF9C; continue 'dispatch;
	}
	// 824FFFB0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 824FFFB4: 8B210090  lbz r25, 0x90(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 824FFFB8: 7F1DE000  cmpw cr6, r29, r28
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[28].s32, &mut ctx.xer);
	// 824FFFBC: 936100A0  stw r27, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[27].u32 ) };
	// 824FFFC0: 419A00BC  beq cr6, 0x8250007c
	if ctx.cr[6].eq {
	pc = 0x8250007C; continue 'dispatch;
	}
	// 824FFFC4: 897E0031  lbz r11, 0x31(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(49 as u32) ) } as u64;
	// 824FFFC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FFFCC: 409A00B0  bne cr6, 0x8250007c
	if !ctx.cr[6].eq {
	pc = 0x8250007C; continue 'dispatch;
	}
	// 824FFFD0: 817F0EA0  lwz r11, 0xea0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3744 as u32) ) } as u64;
	// 824FFFD4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 824FFFD8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 824FFFDC: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FFFE0: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824FFFE4: 914100A0  stw r10, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 824FFFE8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 824FFFEC: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FFFF0: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 824FFFF4: 396B16B0  addi r11, r11, 0x16b0
	ctx.r[11].s64 = ctx.r[11].s64 + 5808;
	// 824FFFF8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824FFFFC: E92A0000  ld r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82500000: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82500004: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82500008: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8250000C: 4200FFF0  bdnz 0x824ffffc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824FFFFC; continue 'dispatch;
	}
	// 82500010: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82500014: 815F1E38  lwz r10, 0x1e38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 82500018: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 8250001C: 80BF0EA0  lwz r5, 0xea0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3744 as u32) ) } as u64;
	// 82500020: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82500024: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82500028: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8250002C: 389F0EB0  addi r4, r31, 0xeb0
	ctx.r[4].s64 = ctx.r[31].s64 + 3760;
	// 82500030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82500034: 4BFFF35D  bl 0x824ff390
	ctx.lr = 0x82500038;
	sub_824FF390(ctx, base);
	// 82500038: 7F2B0774  extsb r11, r25
	ctx.r[11].s64 = ctx.r[25].s8 as i64;
	// 8250003C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82500040: 419A002C  beq cr6, 0x8250006c
	if ctx.cr[6].eq {
	pc = 0x8250006C; continue 'dispatch;
	}
	// 82500044: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82500048: 815F1E3C  lwz r10, 0x1e3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 8250004C: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82500050: 80BF0EA0  lwz r5, 0xea0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3744 as u32) ) } as u64;
	// 82500054: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82500058: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 8250005C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82500060: 389F12B0  addi r4, r31, 0x12b0
	ctx.r[4].s64 = ctx.r[31].s64 + 4784;
	// 82500064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82500068: 4BFFF329  bl 0x824ff390
	ctx.lr = 0x8250006C;
	sub_824FF390(ctx, base);
	// 8250006C: 817F0EA0  lwz r11, 0xea0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3744 as u32) ) } as u64;
	// 82500070: 934100A0  stw r26, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[26].u32 ) };
	// 82500074: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82500078: 917F0EA0  stw r11, 0xea0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3744 as u32), ctx.r[11].u32 ) };
	// 8250007C: 897E0032  lbz r11, 0x32(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(50 as u32) ) } as u64;
	// 82500080: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82500084: 409A0048  bne cr6, 0x825000cc
	if !ctx.cr[6].eq {
	pc = 0x825000CC; continue 'dispatch;
	}
	// 82500088: 83DF0EA0  lwz r30, 0xea0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3744 as u32) ) } as u64;
	// 8250008C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82500090: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82500094: 395E0001  addi r10, r30, 1
	ctx.r[10].s64 = ctx.r[30].s64 + 1;
	// 82500098: 915F0EA0  stw r10, 0xea0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3744 as u32), ctx.r[10].u32 ) };
	// 8250009C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825000A0: 7D5E5214  add r10, r30, r10
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[10].u64;
	// 825000A4: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825000A8: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 825000AC: 394A16B0  addi r10, r10, 0x16b0
	ctx.r[10].s64 = ctx.r[10].s64 + 5808;
	// 825000B0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 825000B4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 825000B8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 825000BC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 825000C0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 825000C4: 4200FFF0  bdnz 0x825000b4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825000B4; continue 'dispatch;
	}
	// 825000C8: 4800000C  b 0x825000d4
	pc = 0x825000D4; continue 'dispatch;
	// 825000CC: 817F0EA0  lwz r11, 0xea0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3744 as u32) ) } as u64;
	// 825000D0: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 825000D4: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 825000D8: 815F1E38  lwz r10, 0x1e38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 825000DC: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 825000E0: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 825000E4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 825000E8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 825000EC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825000F0: 389F0EB0  addi r4, r31, 0xeb0
	ctx.r[4].s64 = ctx.r[31].s64 + 3760;
	// 825000F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825000F8: 4BFFF299  bl 0x824ff390
	ctx.lr = 0x825000FC;
	sub_824FF390(ctx, base);
	// 825000FC: 7F2B0774  extsb r11, r25
	ctx.r[11].s64 = ctx.r[25].s8 as i64;
	// 82500100: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82500104: 419A002C  beq cr6, 0x82500130
	if ctx.cr[6].eq {
	pc = 0x82500130; continue 'dispatch;
	}
	// 82500108: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 8250010C: 815F1E3C  lwz r10, 0x1e3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 82500110: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82500114: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82500118: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8250011C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82500120: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82500124: 389F12B0  addi r4, r31, 0x12b0
	ctx.r[4].s64 = ctx.r[31].s64 + 4784;
	// 82500128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250012C: 4BFFF265  bl 0x824ff390
	ctx.lr = 0x82500130;
	sub_824FF390(ctx, base);
	// 82500130: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82500134: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82500138: 48034FC4  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82500140 size=280
    let mut pc: u32 = 0x82500140;
    'dispatch: loop {
        match pc {
            0x82500140 => {
    //   block [0x82500140..0x82500258)
	// 82500140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82500144: 48034F79  bl 0x825350bc
	ctx.lr = 0x82500148;
	sub_82535080(ctx, base);
	// 82500148: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250014C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82500150: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82500154: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82500158: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8250015C: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 82500160: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82500164: 40990094  ble cr6, 0x825001f8
	if !ctx.cr[6].gt {
	pc = 0x825001F8; continue 'dispatch;
	}
	// 82500168: 391F1E24  addi r8, r31, 0x1e24
	ctx.r[8].s64 = ctx.r[31].s64 + 7716;
	// 8250016C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82500170: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82500174: 7D665A14  add r11, r6, r11
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 82500178: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250017C: 7F0AF000  cmpw cr6, r10, r30
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82500180: 409A0064  bne cr6, 0x825001e4
	if !ctx.cr[6].eq {
	pc = 0x825001E4; continue 'dispatch;
	}
	// 82500184: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82500188: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8250018C: 409A0058  bne cr6, 0x825001e4
	if !ctx.cr[6].eq {
	pc = 0x825001E4; continue 'dispatch;
	}
	// 82500190: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82500194: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82500198: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8250019C: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825001A0: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825001A4: 40980038  bge cr6, 0x825001dc
	if !ctx.cr[6].lt {
	pc = 0x825001DC; continue 'dispatch;
	}
	// 825001A8: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 825001AC: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 825001B0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825001B4: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 825001B8: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 825001BC: 38AB0008  addi r5, r11, 8
	ctx.r[5].s64 = ctx.r[11].s64 + 8;
	// 825001C0: 80850000  lwz r4, 0(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 825001C4: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 825001C8: 80A50004  lwz r5, 4(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 825001CC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 825001D0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 825001D4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825001D8: 4198FFD4  blt cr6, 0x825001ac
	if ctx.cr[6].lt {
	pc = 0x825001AC; continue 'dispatch;
	}
	// 825001DC: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 825001E0: 38C6FFF8  addi r6, r6, -8
	ctx.r[6].s64 = ctx.r[6].s64 + -8;
	// 825001E4: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 825001E8: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 825001EC: 38C60008  addi r6, r6, 8
	ctx.r[6].s64 = ctx.r[6].s64 + 8;
	// 825001F0: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825001F4: 4198FF7C  blt cr6, 0x82500170
	if ctx.cr[6].lt {
	pc = 0x82500170; continue 'dispatch;
	}
	// 825001F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825001FC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82500200: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82500204: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82500208: 4BFFF3A9  bl 0x824ff5b0
	ctx.lr = 0x8250020C;
	sub_824FF5B0(ctx, base);
	// 8250020C: 3BFF1E24  addi r31, r31, 0x1e24
	ctx.r[31].s64 = ctx.r[31].s64 + 7716;
	// 82500210: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82500214: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82500218: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8250021C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82500220: 409A0010  bne cr6, 0x82500230
	if !ctx.cr[6].eq {
	pc = 0x82500230; continue 'dispatch;
	}
	// 82500224: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82500228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250022C: 4BF6E125  bl 0x8246e350
	ctx.lr = 0x82500230;
	sub_8246E350(ctx, base);
	// 82500230: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82500234: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82500238: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250023C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82500240: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82500244: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82500248: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8250024C: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82500250: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82500254: 48034EB8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82500258 size=320
    let mut pc: u32 = 0x82500258;
    'dispatch: loop {
        match pc {
            0x82500258 => {
    //   block [0x82500258..0x82500398)
	// 82500258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250025C: 48034E59  bl 0x825350b4
	ctx.lr = 0x82500260;
	sub_82535080(ctx, base);
	// 82500260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82500264: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82500268: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250026C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82500270: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82500274: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82500278: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250027C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82500280: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82500284: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82500288: 4E800421  bctrl
	ctx.lr = 0x8250028C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250028C: 80DF1E30  lwz r6, 0x1e30(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7728 as u32) ) } as u64;
	// 82500290: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82500294: 419A0058  beq cr6, 0x825002ec
	if ctx.cr[6].eq {
	pc = 0x825002EC; continue 'dispatch;
	}
	// 82500298: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250029C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 825002A0: 3BCB44B8  addi r30, r11, 0x44b8
	ctx.r[30].s64 = ctx.r[11].s64 + 17592;
	// 825002A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825002A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825002AC: 4BF63775  bl 0x82463a20
	ctx.lr = 0x825002B0;
	sub_82463A20(ctx, base);
	// 825002B0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 825002B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825002B8: 80DF1E34  lwz r6, 0x1e34(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7732 as u32) ) } as u64;
	// 825002BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825002C0: 4BF63761  bl 0x82463a20
	ctx.lr = 0x825002C4;
	sub_82463A20(ctx, base);
	// 825002C4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 825002C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825002CC: 80DF1E38  lwz r6, 0x1e38(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 825002D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825002D4: 4BF6374D  bl 0x82463a20
	ctx.lr = 0x825002D8;
	sub_82463A20(ctx, base);
	// 825002D8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 825002DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825002E0: 80DF1E3C  lwz r6, 0x1e3c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 825002E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825002E8: 4BF63739  bl 0x82463a20
	ctx.lr = 0x825002EC;
	sub_82463A20(ctx, base);
	// 825002EC: 815F1E2C  lwz r10, 0x1e2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7724 as u32) ) } as u64;
	// 825002F0: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 825002F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825002F8: 409A0034  bne cr6, 0x8250032c
	if !ctx.cr[6].eq {
	pc = 0x8250032C; continue 'dispatch;
	}
	// 825002FC: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82500300: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82500304: 80FF1E28  lwz r7, 0x1e28(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 82500308: 55481838  slwi r8, r10, 3
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8250030C: 388B44A0  addi r4, r11, 0x44a0
	ctx.r[4].s64 = ctx.r[11].s64 + 17568;
	// 82500310: 80DF1E24  lwz r6, 0x1e24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7716 as u32) ) } as u64;
	// 82500314: 54E71838  slwi r7, r7, 3
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82500318: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8250031C: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82500320: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82500324: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82500328: 4E800421  bctrl
	ctx.lr = 0x8250032C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250032C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82500330: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82500334: 3B600008  li r27, 8
	ctx.r[27].s64 = 8;
	// 82500338: 3B8B448C  addi r28, r11, 0x448c
	ctx.r[28].s64 = ctx.r[11].s64 + 17548;
	// 8250033C: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 82500340: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82500344: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82500348: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250034C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82500350: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82500354: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82500358: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250035C: 4E800421  bctrl
	ctx.lr = 0x82500360;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82500360: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82500364: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82500368: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8250036C: 409AFFD4  bne cr6, 0x82500340
	if !ctx.cr[6].eq {
	pc = 0x82500340; continue 'dispatch;
	}
	// 82500370: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 82500374: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82500378: 409AFFC4  bne cr6, 0x8250033c
	if !ctx.cr[6].eq {
	pc = 0x8250033C; continue 'dispatch;
	}
	// 8250037C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82500380: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82500384: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82500388: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250038C: 4E800421  bctrl
	ctx.lr = 0x82500390;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82500390: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82500394: 48034D70  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82500398 size=1156
    let mut pc: u32 = 0x82500398;
    'dispatch: loop {
        match pc {
            0x82500398 => {
    //   block [0x82500398..0x8250081C)
	// 82500398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250039C: 48034CF5  bl 0x82535090
	ctx.lr = 0x825003A0;
	sub_82535080(ctx, base);
	// 825003A0: 9421FBB0  stwu r1, -0x450(r1)
	ea = ctx.r[1].u32.wrapping_add(-1104 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825003A4: 3F208293  lis r25, -0x7d6d
	ctx.r[25].s64 = -2104295424;
	// 825003A8: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 825003AC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825003B0: 3C805E43  lis r4, 0x5e43
	ctx.r[4].s64 = 1581449216;
	// 825003B4: 38AB4534  addi r5, r11, 0x4534
	ctx.r[5].s64 = ctx.r[11].s64 + 17716;
	// 825003B8: 80799190  lwz r3, -0x6e70(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 825003BC: 608445E4  ori r4, r4, 0x45e4
	ctx.r[4].u64 = ctx.r[4].u64 | 17892;
	// 825003C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825003C4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825003C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825003CC: 4E800421  bctrl
	ctx.lr = 0x825003D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825003D0: 81731E30  lwz r11, 0x1e30(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(7728 as u32) ) } as u64;
	// 825003D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825003D8: 419A043C  beq cr6, 0x82500814
	if ctx.cr[6].eq {
	pc = 0x82500814; continue 'dispatch;
	}
	// 825003DC: 81731E34  lwz r11, 0x1e34(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(7732 as u32) ) } as u64;
	// 825003E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825003E4: 419A0430  beq cr6, 0x82500814
	if ctx.cr[6].eq {
	pc = 0x82500814; continue 'dispatch;
	}
	// 825003E8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825003EC: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 825003F0: 3AAB4520  addi r21, r11, 0x4520
	ctx.r[21].s64 = ctx.r[11].s64 + 17696;
	// 825003F4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825003F8: 7E96A378  mr r22, r20
	ctx.r[22].u64 = ctx.r[20].u64;
	// 825003FC: 3B4B44F8  addi r26, r11, 0x44f8
	ctx.r[26].s64 = ctx.r[11].s64 + 17656;
	// 82500400: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82500404: 3AE00003  li r23, 3
	ctx.r[23].s64 = 3;
	// 82500408: 3B0B44DC  addi r24, r11, 0x44dc
	ctx.r[24].s64 = ctx.r[11].s64 + 17628;
	// 8250040C: 3E408000  lis r18, -0x8000
	ctx.r[18].s64 = -2147483648;
	// 82500410: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82500414: 92810060  stw r20, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[20].u32 ) };
	// 82500418: 92810064  stw r20, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[20].u32 ) };
	// 8250041C: 92410068  stw r18, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[18].u32 ) };
	// 82500420: 4BF69E01  bl 0x8246a220
	ctx.lr = 0x82500424;
	sub_8246A220(ctx, base);
	// 82500424: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82500428: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8250042C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82500430: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82500434: 40980024  bge cr6, 0x82500458
	if !ctx.cr[6].lt {
	pc = 0x82500458; continue 'dispatch;
	}
	// 82500438: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250043C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82500440: 41980008  blt cr6, 0x82500448
	if ctx.cr[6].lt {
	pc = 0x82500448; continue 'dispatch;
	}
	// 82500444: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82500448: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250044C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82500450: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82500454: 4BF6DE75  bl 0x8246e2c8
	ctx.lr = 0x82500458;
	sub_8246E2C8(ctx, base);
	// 82500458: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8250045C: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82500460: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82500464: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82500468: 4BF69EC1  bl 0x8246a328
	ctx.lr = 0x8250046C;
	sub_8246A328(ctx, base);
	// 8250046C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82500470: 480283A1  bl 0x82528810
	ctx.lr = 0x82500474;
	sub_82528810(ctx, base);
	// 82500474: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82500478: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8250047C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82500480: 4BF6A7D1  bl 0x8246ac50
	ctx.lr = 0x82500484;
	sub_8246AC50(ctx, base);
	// 82500484: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82500488: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8250048C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82500490: 409A0020  bne cr6, 0x825004b0
	if !ctx.cr[6].eq {
	pc = 0x825004B0; continue 'dispatch;
	}
	// 82500494: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82500498: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8250049C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825004A0: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 825004A4: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825004A8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825004AC: 4BF63C0D  bl 0x824640b8
	ctx.lr = 0x825004B0;
	sub_824640B8(ctx, base);
	// 825004B0: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 825004B4: 388101D0  addi r4, r1, 0x1d0
	ctx.r[4].s64 = ctx.r[1].s64 + 464;
	// 825004B8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 825004BC: 4BF6893D  bl 0x82468df8
	ctx.lr = 0x825004C0;
	sub_82468DF8(ctx, base);
	// 825004C0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 825004C4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 825004C8: 4BF68531  bl 0x824689f8
	ctx.lr = 0x825004CC;
	sub_824689F8(ctx, base);
	// 825004CC: 80799190  lwz r3, -0x6e70(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 825004D0: 39000222  li r8, 0x222
	ctx.r[8].s64 = 546;
	// 825004D4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 825004D8: 38C101D0  addi r6, r1, 0x1d0
	ctx.r[6].s64 = ctx.r[1].s64 + 464;
	// 825004DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 825004E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825004E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825004E8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825004EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825004F0: 4E800421  bctrl
	ctx.lr = 0x825004F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825004F4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 825004F8: 4BF683E9  bl 0x824688e0
	ctx.lr = 0x825004FC;
	sub_824688E0(ctx, base);
	// 825004FC: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82500500: 7EFBBB78  mr r27, r23
	ctx.r[27].u64 = ctx.r[23].u64;
	// 82500504: 81731E34  lwz r11, 0x1e34(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(7732 as u32) ) } as u64;
	// 82500508: 7FFB5A14  add r31, r27, r11
	ctx.r[31].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 8250050C: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82500510: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82500514: 2F0B0064  cmpwi cr6, r11, 0x64
	ctx.cr[6].compare_i32(ctx.r[11].s32, 100, &mut ctx.xer);
	// 82500518: 4098009C  bge cr6, 0x825005b4
	if !ctx.cr[6].lt {
	pc = 0x825005B4; continue 'dispatch;
	}
	// 8250051C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82500520: 480282F1  bl 0x82528810
	ctx.lr = 0x82500524;
	sub_82528810(ctx, base);
	// 82500524: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82500528: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250052C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82500530: 480282E1  bl 0x82528810
	ctx.lr = 0x82500534;
	sub_82528810(ctx, base);
	// 82500534: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82500538: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8250053C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82500540: 480282D1  bl 0x82528810
	ctx.lr = 0x82500544;
	sub_82528810(ctx, base);
	// 82500544: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82500548: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8250054C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82500550: 7D670774  extsb r7, r11
	ctx.r[7].s64 = ctx.r[11].s8 as i64;
	// 82500554: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82500558: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 8250055C: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 82500560: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82500564: 4BF699DD  bl 0x82469f40
	ctx.lr = 0x82500568;
	sub_82469F40(ctx, base);
	// 82500568: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 8250056C: 388101D0  addi r4, r1, 0x1d0
	ctx.r[4].s64 = ctx.r[1].s64 + 464;
	// 82500570: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82500574: 4BF68885  bl 0x82468df8
	ctx.lr = 0x82500578;
	sub_82468DF8(ctx, base);
	// 82500578: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
	// 8250057C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82500580: 4BF67D21  bl 0x824682a0
	ctx.lr = 0x82500584;
	sub_824682A0(ctx, base);
	// 82500584: 80799190  lwz r3, -0x6e70(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82500588: 39000231  li r8, 0x231
	ctx.r[8].s64 = 561;
	// 8250058C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82500590: 38C101D0  addi r6, r1, 0x1d0
	ctx.r[6].s64 = ctx.r[1].s64 + 464;
	// 82500594: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82500598: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250059C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825005A0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825005A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825005A8: 4E800421  bctrl
	ctx.lr = 0x825005AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825005AC: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 825005B0: 4BF68331  bl 0x824688e0
	ctx.lr = 0x825005B4;
	sub_824688E0(ctx, base);
	// 825005B4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 825005B8: 3B7B0003  addi r27, r27, 3
	ctx.r[27].s64 = ctx.r[27].s64 + 3;
	// 825005BC: 2F1C0020  cmpwi cr6, r28, 0x20
	ctx.cr[6].compare_i32(ctx.r[28].s32, 32, &mut ctx.xer);
	// 825005C0: 4198FF44  blt cr6, 0x82500504
	if ctx.cr[6].lt {
	pc = 0x82500504; continue 'dispatch;
	}
	// 825005C4: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 825005C8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825005CC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825005D0: 409A0020  bne cr6, 0x825005f0
	if !ctx.cr[6].eq {
	pc = 0x825005F0; continue 'dispatch;
	}
	// 825005D4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825005D8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 825005DC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825005E0: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 825005E4: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825005E8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825005EC: 4BF63ACD  bl 0x824640b8
	ctx.lr = 0x825005F0;
	sub_824640B8(ctx, base);
	// 825005F0: 3AF70060  addi r23, r23, 0x60
	ctx.r[23].s64 = ctx.r[23].s64 + 96;
	// 825005F4: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 825005F8: 2F170C03  cmpwi cr6, r23, 0xc03
	ctx.cr[6].compare_i32(ctx.r[23].s32, 3075, &mut ctx.xer);
	// 825005FC: 4198FE14  blt cr6, 0x82500410
	if ctx.cr[6].lt {
	pc = 0x82500410; continue 'dispatch;
	}
	// 82500600: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82500604: 7E96A378  mr r22, r20
	ctx.r[22].u64 = ctx.r[20].u64;
	// 82500608: 3B000003  li r24, 3
	ctx.r[24].s64 = 3;
	// 8250060C: 3AEB44C4  addi r23, r11, 0x44c4
	ctx.r[23].s64 = ctx.r[11].s64 + 17604;
	// 82500610: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82500614: 92810050  stw r20, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[20].u32 ) };
	// 82500618: 92810054  stw r20, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[20].u32 ) };
	// 8250061C: 92410058  stw r18, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[18].u32 ) };
	// 82500620: 4BF69C01  bl 0x8246a220
	ctx.lr = 0x82500624;
	sub_8246A220(ctx, base);
	// 82500624: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82500628: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8250062C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82500630: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82500634: 40980024  bge cr6, 0x82500658
	if !ctx.cr[6].lt {
	pc = 0x82500658; continue 'dispatch;
	}
	// 82500638: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250063C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82500640: 41980008  blt cr6, 0x82500648
	if ctx.cr[6].lt {
	pc = 0x82500648; continue 'dispatch;
	}
	// 82500644: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82500648: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250064C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82500650: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82500654: 4BF6DC75  bl 0x8246e2c8
	ctx.lr = 0x82500658;
	sub_8246E2C8(ctx, base);
	// 82500658: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8250065C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82500660: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82500664: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82500668: 4BF69CC1  bl 0x8246a328
	ctx.lr = 0x8250066C;
	sub_8246A328(ctx, base);
	// 8250066C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82500670: 480281A1  bl 0x82528810
	ctx.lr = 0x82500674;
	sub_82528810(ctx, base);
	// 82500674: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82500678: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8250067C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82500680: 4BF6A5D1  bl 0x8246ac50
	ctx.lr = 0x82500684;
	sub_8246AC50(ctx, base);
	// 82500684: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82500688: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8250068C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82500690: 409A0020  bne cr6, 0x825006b0
	if !ctx.cr[6].eq {
	pc = 0x825006B0; continue 'dispatch;
	}
	// 82500694: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82500698: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8250069C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825006A0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825006A4: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825006A8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825006AC: 4BF63A0D  bl 0x824640b8
	ctx.lr = 0x825006B0;
	sub_824640B8(ctx, base);
	// 825006B0: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 825006B4: 388101D0  addi r4, r1, 0x1d0
	ctx.r[4].s64 = ctx.r[1].s64 + 464;
	// 825006B8: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825006BC: 4BF6873D  bl 0x82468df8
	ctx.lr = 0x825006C0;
	sub_82468DF8(ctx, base);
	// 825006C0: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 825006C4: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825006C8: 4BF68331  bl 0x824689f8
	ctx.lr = 0x825006CC;
	sub_824689F8(ctx, base);
	// 825006CC: 80799190  lwz r3, -0x6e70(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 825006D0: 39000239  li r8, 0x239
	ctx.r[8].s64 = 569;
	// 825006D4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 825006D8: 38C101D0  addi r6, r1, 0x1d0
	ctx.r[6].s64 = ctx.r[1].s64 + 464;
	// 825006DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 825006E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825006E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825006E8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825006EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825006F0: 4E800421  bctrl
	ctx.lr = 0x825006F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825006F4: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825006F8: 4BF681E9  bl 0x824688e0
	ctx.lr = 0x825006FC;
	sub_824688E0(ctx, base);
	// 825006FC: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82500700: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 82500704: 81731E30  lwz r11, 0x1e30(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(7728 as u32) ) } as u64;
	// 82500708: 7FFB5A14  add r31, r27, r11
	ctx.r[31].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 8250070C: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82500710: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82500714: 2F0B0064  cmpwi cr6, r11, 0x64
	ctx.cr[6].compare_i32(ctx.r[11].s32, 100, &mut ctx.xer);
	// 82500718: 4098009C  bge cr6, 0x825007b4
	if !ctx.cr[6].lt {
	pc = 0x825007B4; continue 'dispatch;
	}
	// 8250071C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82500720: 480280F1  bl 0x82528810
	ctx.lr = 0x82500724;
	sub_82528810(ctx, base);
	// 82500724: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82500728: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250072C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82500730: 480280E1  bl 0x82528810
	ctx.lr = 0x82500734;
	sub_82528810(ctx, base);
	// 82500734: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82500738: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8250073C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82500740: 480280D1  bl 0x82528810
	ctx.lr = 0x82500744;
	sub_82528810(ctx, base);
	// 82500744: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82500748: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8250074C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82500750: 7D670774  extsb r7, r11
	ctx.r[7].s64 = ctx.r[11].s8 as i64;
	// 82500754: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82500758: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 8250075C: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 82500760: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82500764: 4BF697DD  bl 0x82469f40
	ctx.lr = 0x82500768;
	sub_82469F40(ctx, base);
	// 82500768: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 8250076C: 388101D0  addi r4, r1, 0x1d0
	ctx.r[4].s64 = ctx.r[1].s64 + 464;
	// 82500770: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82500774: 4BF68685  bl 0x82468df8
	ctx.lr = 0x82500778;
	sub_82468DF8(ctx, base);
	// 82500778: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
	// 8250077C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82500780: 4BF67B21  bl 0x824682a0
	ctx.lr = 0x82500784;
	sub_824682A0(ctx, base);
	// 82500784: 80799190  lwz r3, -0x6e70(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82500788: 39000248  li r8, 0x248
	ctx.r[8].s64 = 584;
	// 8250078C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82500790: 38C101D0  addi r6, r1, 0x1d0
	ctx.r[6].s64 = ctx.r[1].s64 + 464;
	// 82500794: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82500798: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250079C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825007A0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825007A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825007A8: 4E800421  bctrl
	ctx.lr = 0x825007AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825007AC: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825007B0: 4BF68131  bl 0x824688e0
	ctx.lr = 0x825007B4;
	sub_824688E0(ctx, base);
	// 825007B4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 825007B8: 3B7B0003  addi r27, r27, 3
	ctx.r[27].s64 = ctx.r[27].s64 + 3;
	// 825007BC: 2F1C0020  cmpwi cr6, r28, 0x20
	ctx.cr[6].compare_i32(ctx.r[28].s32, 32, &mut ctx.xer);
	// 825007C0: 4198FF44  blt cr6, 0x82500704
	if ctx.cr[6].lt {
	pc = 0x82500704; continue 'dispatch;
	}
	// 825007C4: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 825007C8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825007CC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825007D0: 409A0020  bne cr6, 0x825007f0
	if !ctx.cr[6].eq {
	pc = 0x825007F0; continue 'dispatch;
	}
	// 825007D4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825007D8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 825007DC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825007E0: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 825007E4: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825007E8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825007EC: 4BF638CD  bl 0x824640b8
	ctx.lr = 0x825007F0;
	sub_824640B8(ctx, base);
	// 825007F0: 3B180060  addi r24, r24, 0x60
	ctx.r[24].s64 = ctx.r[24].s64 + 96;
	// 825007F4: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 825007F8: 2F180C03  cmpwi cr6, r24, 0xc03
	ctx.cr[6].compare_i32(ctx.r[24].s32, 3075, &mut ctx.xer);
	// 825007FC: 4198FE14  blt cr6, 0x82500610
	if ctx.cr[6].lt {
	pc = 0x82500610; continue 'dispatch;
	}
	// 82500800: 80799190  lwz r3, -0x6e70(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82500804: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82500808: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250080C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82500810: 4E800421  bctrl
	ctx.lr = 0x82500814;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82500814: 38210450  addi r1, r1, 0x450
	ctx.r[1].s64 = ctx.r[1].s64 + 1104;
	// 82500818: 480348C8  b 0x825350e0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82500820 size=100
    let mut pc: u32 = 0x82500820;
    'dispatch: loop {
        match pc {
            0x82500820 => {
    //   block [0x82500820..0x82500884)
	// 82500820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82500824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82500828: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250082C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82500830: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82500834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82500838: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250083C: 4BFFF425  bl 0x824ffc60
	ctx.lr = 0x82500840;
	sub_824FFC60(ctx, base);
	// 82500840: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82500844: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82500848: 419A0020  beq cr6, 0x82500868
	if ctx.cr[6].eq {
	pc = 0x82500868; continue 'dispatch;
	}
	// 8250084C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82500850: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82500854: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82500858: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250085C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82500860: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82500864: 4BF63855  bl 0x824640b8
	ctx.lr = 0x82500868;
	sub_824640B8(ctx, base);
	// 82500868: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250086C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82500870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82500874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82500878: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250087C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82500880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82500888 size=184
    let mut pc: u32 = 0x82500888;
    'dispatch: loop {
        match pc {
            0x82500888 => {
    //   block [0x82500888..0x82500940)
	// 82500888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250088C: 48034831  bl 0x825350bc
	ctx.lr = 0x82500890;
	sub_82535080(ctx, base);
	// 82500890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82500894: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82500898: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250089C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 825008A0: 7FCB2214  add r30, r11, r4
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 825008A4: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825008A8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825008AC: 419A0084  beq cr6, 0x82500930
	if ctx.cr[6].eq {
	pc = 0x82500930; continue 'dispatch;
	}
	// 825008B0: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 825008B4: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 825008B8: 419A0078  beq cr6, 0x82500930
	if ctx.cr[6].eq {
	pc = 0x82500930; continue 'dispatch;
	}
	// 825008BC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825008C0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825008C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825008C8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825008CC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825008D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825008D4: 4E800421  bctrl
	ctx.lr = 0x825008D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825008D8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825008DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825008E0: 419A0050  beq cr6, 0x82500930
	if ctx.cr[6].eq {
	pc = 0x82500930; continue 'dispatch;
	}
	// 825008E4: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825008E8: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 825008EC: 813D000C  lwz r9, 0xc(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 825008F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825008F4: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 825008F8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 825008FC: 80FF0010  lwz r7, 0x10(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82500900: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82500904: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82500908: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250090C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82500910: 7D4A58AE  lbzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82500914: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82500918: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8250091C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82500920: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82500924: 816B09AC  lwz r11, 0x9ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2476 as u32) ) } as u64;
	// 82500928: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250092C: 4E800421  bctrl
	ctx.lr = 0x82500930;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82500930: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82500934: C02B0004  lfs f1, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82500938: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250093C: 480347D0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82500940 size=476
    let mut pc: u32 = 0x82500940;
    'dispatch: loop {
        match pc {
            0x82500940 => {
    //   block [0x82500940..0x82500B1C)
	// 82500940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82500944: 48034759  bl 0x8253509c
	ctx.lr = 0x82500948;
	sub_82535080(ctx, base);
	// 82500948: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82500B20 size=4
    let mut pc: u32 = 0x82500B20;
    'dispatch: loop {
        match pc {
            0x82500B20 => {
    //   block [0x82500B20..0x82500B24)
	// 82500B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82500B28 size=4
    let mut pc: u32 = 0x82500B28;
    'dispatch: loop {
        match pc {
            0x82500B28 => {
    //   block [0x82500B28..0x82500B2C)
	// 82500B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82500B30 size=4
    let mut pc: u32 = 0x82500B30;
    'dispatch: loop {
        match pc {
            0x82500B30 => {
    //   block [0x82500B30..0x82500B34)
	// 82500B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82500B38 size=12
    let mut pc: u32 = 0x82500B38;
    'dispatch: loop {
        match pc {
            0x82500B38 => {
    //   block [0x82500B38..0x82500B44)
	// 82500B38: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82500B3C: 386BD24C  addi r3, r11, -0x2db4
	ctx.r[3].s64 = ctx.r[11].s64 + -11700;
	// 82500B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82500B48 size=12
    let mut pc: u32 = 0x82500B48;
    'dispatch: loop {
        match pc {
            0x82500B48 => {
    //   block [0x82500B48..0x82500B54)
	// 82500B48: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82500B4C: 386BD24C  addi r3, r11, -0x2db4
	ctx.r[3].s64 = ctx.r[11].s64 + -11700;
	// 82500B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82500B58 size=32
    let mut pc: u32 = 0x82500B58;
    'dispatch: loop {
        match pc {
            0x82500B58 => {
    //   block [0x82500B58..0x82500B78)
	// 82500B58: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82500B5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82500B60: 396B4560  addi r11, r11, 0x4560
	ctx.r[11].s64 = ctx.r[11].s64 + 17760;
	// 82500B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82500B68: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82500B6C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82500B70: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82500B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82500B78 size=368
    let mut pc: u32 = 0x82500B78;
    'dispatch: loop {
        match pc {
            0x82500B78 => {
    //   block [0x82500B78..0x82500CE8)
	// 82500B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82500B7C: 48034541  bl 0x825350bc
	ctx.lr = 0x82500B80;
	sub_82535080(ctx, base);
	// 82500B80: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82500B84: 48035451  bl 0x82535fd4
	ctx.lr = 0x82500B88;
	sub_82535FB0(ctx, base);
	// 82500B88: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82500B8C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82500B90: FF400890  fmr f26, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[26].f64 = ctx.f[1].f64;
	// 82500B94: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 82500B98: FF601090  fmr f27, f2
	ctx.f[27].f64 = ctx.f[2].f64;
	// 82500B9C: 3BAB2380  addi r29, r11, 0x2380
	ctx.r[29].s64 = ctx.r[11].s64 + 9088;
	// 82500BA0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82500BA4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82500BA8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82500BAC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82500BB0: C2E72074  lfs f23, 0x2074(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8308 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 82500BB4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82500BB8: C3281850  lfs f25, 0x1850(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(6224 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 82500BBC: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 82500BC0: C3891FF8  lfs f28, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82500BC4: C3CA2930  lfs f30, 0x2930(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(10544 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82500BC8: C30B4598  lfs f24, 0x4598(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17816 as u32) ) };
	ctx.f[24].f64 = (tmp.f32 as f64);
	// 82500BCC: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 82500BD0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82500BD4: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82500BD8: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82500BDC: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82500BE0: EFE0F638  fmsubs f31, f0, f24, f30
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[24].f64 - ctx.f[30].f64) as f32) as f64);
	// 82500BE4: FF1FE000  fcmpu cr6, f31, f28
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[28].f64);
	// 82500BE8: 40980070  bge cr6, 0x82500c58
	if !ctx.cr[6].lt {
	pc = 0x82500C58; continue 'dispatch;
	}
	// 82500BEC: FC00FA10  fabs f0, f31
	ctx.f[0].u64 = ctx.f[31].u64 & !0x8000_0000_0000_0000u64;
	// 82500BF0: D33FFFFC  stfs f25, -4(r31)
	tmp.f32 = (ctx.f[25].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82500BF4: D39F0000  stfs f28, 0(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82500BF8: EFBE0028  fsubs f29, f30, f0
	ctx.f[29].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 82500BFC: EFFDD028  fsubs f31, f29, f26
	ctx.f[31].f64 = (((ctx.f[29].f64 - ctx.f[26].f64) as f32) as f64);
	// 82500C00: FF1FD800  fcmpu cr6, f31, f27
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[27].f64);
	// 82500C04: 4099000C  ble cr6, 0x82500c10
	if !ctx.cr[6].gt {
	pc = 0x82500C10; continue 'dispatch;
	}
	// 82500C08: FC20D890  fmr f1, f27
	ctx.f[1].f64 = ctx.f[27].f64;
	// 82500C0C: 48000008  b 0x82500c14
	pc = 0x82500C14; continue 'dispatch;
	// 82500C10: FC3FE7EE  fsel f1, f31, f31, f28
	ctx.f[1].f64 = if ctx.f[31].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[28].f64 };
	// 82500C14: 48035715  bl 0x82536328
	ctx.lr = 0x82500C18;
	sub_82536328(ctx, base);
	// 82500C18: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82500C1C: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82500C20: FC3FE7EE  fsel f1, f31, f31, f28
	ctx.f[1].f64 = if ctx.f[31].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[28].f64 };
	// 82500C24: 48035705  bl 0x82536328
	ctx.lr = 0x82500C28;
	sub_82536328(ctx, base);
	// 82500C28: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82500C2C: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82500C30: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82500C34: 480356F5  bl 0x82536328
	ctx.lr = 0x82500C38;
	sub_82536328(ctx, base);
	// 82500C38: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 82500C3C: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82500C40: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82500C44: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82500C48: 48035611  bl 0x82536258
	ctx.lr = 0x82500C4C;
	sub_82536258(ctx, base);
	// 82500C4C: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82500C50: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82500C54: 48000070  b 0x82500cc4
	pc = 0x82500CC4; continue 'dispatch;
	// 82500C58: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82500C5C: 480356CD  bl 0x82536328
	ctx.lr = 0x82500C60;
	sub_82536328(ctx, base);
	// 82500C60: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 82500C64: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82500C68: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82500C6C: D01FFFFC  stfs f0, -4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82500C70: 480355E9  bl 0x82536258
	ctx.lr = 0x82500C74;
	sub_82536258(ctx, base);
	// 82500C74: EFFFD02A  fadds f31, f31, f26
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ((ctx.f[31].f64 + ctx.f[26].f64) as f32) as f64;
	// 82500C78: FC000818  frsp f0, f1
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82500C7C: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82500C80: EC1FF028  fsubs f0, f31, f30
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[30].f64) as f32) as f64);
	// 82500C84: FC20FFAE  fsel f1, f0, f30, f31
	ctx.f[1].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[30].f64 } else { ctx.f[31].f64 };
	// 82500C88: 480356A1  bl 0x82536328
	ctx.lr = 0x82500C8C;
	sub_82536328(ctx, base);
	// 82500C8C: EC1ED828  fsubs f0, f30, f27
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[30].f64 - ctx.f[27].f64) as f32) as f64);
	// 82500C90: FDA00818  frsp f13, f1
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82500C94: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82500C98: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82500C9C: 4098000C  bge cr6, 0x82500ca8
	if !ctx.cr[6].lt {
	pc = 0x82500CA8; continue 'dispatch;
	}
	// 82500CA0: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 82500CA4: 4800000C  b 0x82500cb0
	pc = 0x82500CB0; continue 'dispatch;
	// 82500CA8: EC1FF028  fsubs f0, f31, f30
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[30].f64) as f32) as f64);
	// 82500CAC: FC20FFAE  fsel f1, f0, f30, f31
	ctx.f[1].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[30].f64 } else { ctx.f[31].f64 };
	// 82500CB0: 48035679  bl 0x82536328
	ctx.lr = 0x82500CB4;
	sub_82536328(ctx, base);
	// 82500CB4: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82500CB8: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82500CBC: D2FF000C  stfs f23, 0xc(r31)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82500CC0: D39F0010  stfs f28, 0x10(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82500CC4: 3BFF0018  addi r31, r31, 0x18
	ctx.r[31].s64 = ctx.r[31].s64 + 24;
	// 82500CC8: 397D02EC  addi r11, r29, 0x2ec
	ctx.r[11].s64 = ctx.r[29].s64 + 748;
	// 82500CCC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82500CD0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82500CD4: 4198FEF8  blt cr6, 0x82500bcc
	if ctx.cr[6].lt {
	pc = 0x82500BCC; continue 'dispatch;
	}
	// 82500CD8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82500CDC: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82500CE0: 48035341  bl 0x82536020
	ctx.lr = 0x82500CE4;
	sub_82535FFC(ctx, base);
	// 82500CE4: 48034428  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82500CE8 size=28
    let mut pc: u32 = 0x82500CE8;
    'dispatch: loop {
        match pc {
            0x82500CE8 => {
    //   block [0x82500CE8..0x82500D04)
	// 82500CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82500CEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82500CF0: B1630020  sth r11, 0x20(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u16 ) };
	// 82500CF4: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82500CF8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82500CFC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82500D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82500D08 size=120
    let mut pc: u32 = 0x82500D08;
    'dispatch: loop {
        match pc {
            0x82500D08 => {
    //   block [0x82500D08..0x82500D80)
	// 82500D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82500D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82500D10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82500D14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82500D18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82500D1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82500D20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82500D24: 419A0048  beq cr6, 0x82500d6c
	if ctx.cr[6].eq {
	pc = 0x82500D6C; continue 'dispatch;
	}
	// 82500D28: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82500D2C: 2F0A000A  cmpwi cr6, r10, 0xa
	ctx.cr[6].compare_i32(ctx.r[10].s32, 10, &mut ctx.xer);
	// 82500D30: 409A0018  bne cr6, 0x82500d48
	if !ctx.cr[6].eq {
	pc = 0x82500D48; continue 'dispatch;
	}
	// 82500D34: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82500D38: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82500D3C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82500D40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82500D44: 4E800421  bctrl
	ctx.lr = 0x82500D48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82500D48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82500D4C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82500D50: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 82500D54: 409A0018  bne cr6, 0x82500d6c
	if !ctx.cr[6].eq {
	pc = 0x82500D6C; continue 'dispatch;
	}
	// 82500D58: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82500D5C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82500D60: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82500D64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82500D68: 4E800421  bctrl
	ctx.lr = 0x82500D6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82500D6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82500D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82500D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82500D78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82500D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82500D80 size=448
    let mut pc: u32 = 0x82500D80;
    'dispatch: loop {
        match pc {
            0x82500D80 => {
    //   block [0x82500D80..0x82500F40)
	// 82500D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82500D84: 4803432D  bl 0x825350b0
	ctx.lr = 0x82500D88;
	sub_82535080(ctx, base);
	// 82500D88: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82500D8C: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82500D90: D001FFB0  stfs f0, -0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), tmp.u32 ) };
	// 82500D94: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82500D98: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82500D9C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82500DA0: D001FFB4  stfs f0, -0x4c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-76 as u32), tmp.u32 ) };
	// 82500DA4: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82500DA8: D001FFB8  stfs f0, -0x48(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), tmp.u32 ) };
	// 82500DAC: 3961FFB0  addi r11, r1, -0x50
	ctx.r[11].s64 = ctx.r[1].s64 + -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82500F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82500F40 size=1084
    let mut pc: u32 = 0x82500F40;
    'dispatch: loop {
        match pc {
            0x82500F40 => {
    //   block [0x82500F40..0x8250137C)
	// 82500F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82500F44: 4803415D  bl 0x825350a0
	ctx.lr = 0x82500F48;
	sub_82535080(ctx, base);
	// 82500F48: 3981FFA8  addi r12, r1, -0x58
	ctx.r[12].s64 = ctx.r[1].s64 + -88;
	// 82500F4C: 48035091  bl 0x82535fdc
	ctx.lr = 0x82500F50;
	sub_82535FB0(ctx, base);
	// 82500F50: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82501380 size=4
    let mut pc: u32 = 0x82501380;
    'dispatch: loop {
        match pc {
            0x82501380 => {
    //   block [0x82501380..0x82501384)
	// 82501380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82501388 size=84
    let mut pc: u32 = 0x82501388;
    'dispatch: loop {
        match pc {
            0x82501388 => {
    //   block [0x82501388..0x825013DC)
	// 82501388: 7C6B1E70  srawi r11, r3, 3
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 3) as i64;
	// 8250138C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82501390: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82501394: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82501398: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8250139C: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825013A0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825013A4: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825013A8: C00B8E30  lfs f0, -0x71d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825013AC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825013B0: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 825013B4: C00B45C0  lfs f0, 0x45c0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17856 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825013B8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 825013BC: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825013C0: D00BD258  stfs f0, -0x2da8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-11688 as u32), tmp.u32 ) };
	// 825013C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825013C8: C1AB1850  lfs f13, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825013CC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 825013D0: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 825013D4: D00BD25C  stfs f0, -0x2da4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-11684 as u32), tmp.u32 ) };
	// 825013D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825013E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825013E0 size=8
    let mut pc: u32 = 0x825013E0;
    'dispatch: loop {
        match pc {
            0x825013E0 => {
    //   block [0x825013E0..0x825013E8)
	// 825013E0: C0230014  lfs f1, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825013E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825013E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825013E8 size=8
    let mut pc: u32 = 0x825013E8;
    'dispatch: loop {
        match pc {
            0x825013E8 => {
    //   block [0x825013E8..0x825013F0)
	// 825013E8: 38600012  li r3, 0x12
	ctx.r[3].s64 = 18;
	// 825013EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825013F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825013F0 size=96
    let mut pc: u32 = 0x825013F0;
    'dispatch: loop {
        match pc {
            0x825013F0 => {
    //   block [0x825013F0..0x82501450)
	// 825013F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825013F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825013F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825013FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82501400: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82501404: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82501408: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8250140C: 388B45C4  addi r4, r11, 0x45c4
	ctx.r[4].s64 = ctx.r[11].s64 + 17860;
	// 82501410: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82501414: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82501418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250141C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82501420: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82501424: 4E800421  bctrl
	ctx.lr = 0x82501428;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82501428: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250142C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82501430: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82501434: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82501438: 4E800421  bctrl
	ctx.lr = 0x8250143C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250143C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82501440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82501444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82501448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250144C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82501450 size=48
    let mut pc: u32 = 0x82501450;
    'dispatch: loop {
        match pc {
            0x82501450 => {
    //   block [0x82501450..0x82501480)
	// 82501450: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82501454: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82501458: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8250145C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82501460: C1AA2278  lfs f13, 0x2278(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8824 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82501464: C00B20B0  lfs f0, 0x20b0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8368 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82501468: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 8250146C: FD80081E  fctiwz f12, f1
	ctx.f[12].s64 = if ctx.f[1].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[1].f64.trunc() as i32 as i64 };
	// 82501470: 7D805FAE  stfiwx f12, 0, r11
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 82501474: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82501478: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250147C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82501480 size=24
    let mut pc: u32 = 0x82501480;
    'dispatch: loop {
        match pc {
            0x82501480 => {
    //   block [0x82501480..0x82501498)
	// 82501480: EC21002A  fadds f1, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 82501484: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 82501488: 4198FFE0  blt cr6, 0x82501468
	if ctx.cr[6].lt {
		sub_82501450(ctx, base);
		return;
	}
	// 8250148C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82501490: C02B1850  lfs f1, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82501494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82501498 size=292
    let mut pc: u32 = 0x82501498;
    'dispatch: loop {
        match pc {
            0x82501498 => {
    //   block [0x82501498..0x825015BC)
	// 82501498: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8250149C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 825014A0: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 825014A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825014A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825015C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825015C0 size=32
    let mut pc: u32 = 0x825015C0;
    'dispatch: loop {
        match pc {
            0x825015C0 => {
    //   block [0x825015C0..0x825015E0)
	// 825015C0: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825015C4: EC00082A  fadds f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 825015C8: D0230014  stfs f1, 0x14(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825015CC: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825015D0: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825015D4: EC00082A  fadds f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 825015D8: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 825015DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825015E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825015E0 size=152
    let mut pc: u32 = 0x825015E0;
    'dispatch: loop {
        match pc {
            0x825015E0 => {
    //   block [0x825015E0..0x82501678)
	// 825015E0: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82501678 size=40
    let mut pc: u32 = 0x82501678;
    'dispatch: loop {
        match pc {
            0x82501678 => {
    //   block [0x82501678..0x825016A0)
	// 82501678: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8250167C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82501680: 396B1FF8  addi r11, r11, 0x1ff8
	ctx.r[11].s64 = ctx.r[11].s64 + 8184;
	// 82501684: C14B0000  lfs f10, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82501688: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8250168C: FDA05090  fmr f13, f10
	ctx.f[13].f64 = ctx.f[10].f64;
	// 82501690: 396B1850  addi r11, r11, 0x1850
	ctx.r[11].s64 = ctx.r[11].s64 + 6224;
	// 82501694: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82501698: FC006090  fmr f0, f12
	ctx.f[0].f64 = ctx.f[12].f64;
	// 8250169C: 48000008  b 0x825016a4
	sub_825016A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825016A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825016A0 size=244
    let mut pc: u32 = 0x825016A0;
    'dispatch: loop {
        match pc {
            0x825016A0 => {
    //   block [0x825016A0..0x82501794)
	// 825016A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825016A4: FF005000  fcmpu cr6, f0, f10
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 825016A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825016AC: 40980008  bge cr6, 0x825016b4
	if !ctx.cr[6].lt {
	pc = 0x825016B4; continue 'dispatch;
	}
	// 825016B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825016B4: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 825016B8: FD600210  fabs f11, f0
	ctx.f[11].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 825016BC: FF0D5800  fcmpu cr6, f13, f11
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[11].f64);
	// 825016C0: 419900D4  bgt cr6, 0x82501794
	if ctx.cr[6].gt {
		sub_82501794(ctx, base);
		return;
	}
	// 825016C4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825016C8: FD606890  fmr f11, f13
	ctx.f[11].f64 = ctx.f[13].f64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501794(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82501794 size=8
    let mut pc: u32 = 0x82501794;
    'dispatch: loop {
        match pc {
            0x82501794 => {
    //   block [0x82501794..0x8250179C)
	// 82501794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82501798: 4BFFFF34  b 0x825016cc
	sub_825016A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250179C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8250179C size=124
    let mut pc: u32 = 0x8250179C;
    'dispatch: loop {
        match pc {
            0x8250179C => {
    //   block [0x8250179C..0x82501818)
	// 8250179C: FD800090  fmr f12, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = ctx.f[0].f64;
	// 825017A0: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 825017A4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825017A8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 825017AC: D181FFF8  stfs f12, -8(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 825017B0: 409A000C  bne cr6, 0x825017bc
	if !ctx.cr[6].eq {
	pc = 0x825017BC; continue 'dispatch;
	}
	// 825017B4: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 825017B8: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825017BC: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 825017C0: 409A000C  bne cr6, 0x825017cc
	if !ctx.cr[6].eq {
	pc = 0x825017CC; continue 'dispatch;
	}
	// 825017C4: FC006050  fneg f0, f12
	ctx.f[0].u64 = ctx.f[12].u64 ^ 0x8000_0000_0000_0000u64;
	// 825017C8: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 825017CC: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 825017D0: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 825017D4: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 825017D8: 21290003  subfic r9, r9, 3
	ctx.xer.ca = ctx.r[9].u32 <= 3 as u32;
	ctx.r[9].s64 = (3 as i64) - ctx.r[9].s64;
	// 825017DC: 654A3F00  oris r10, r10, 0x3f00
	ctx.r[10].u64 = ctx.r[10].u64 | 1056964608;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82501818 size=276
    let mut pc: u32 = 0x82501818;
    'dispatch: loop {
        match pc {
            0x82501818 => {
    //   block [0x82501818..0x8250192C)
	// 82501818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250181C: 48033899  bl 0x825350b4
	ctx.lr = 0x82501820;
	sub_82535080(ctx, base);
	// 82501820: 3945FFFF  addi r10, r5, -1
	ctx.r[10].s64 = ctx.r[5].s64 + -1;
	// 82501824: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82501828: 41980100  blt cr6, 0x82501928
	if ctx.cr[6].lt {
	pc = 0x82501928; continue 'dispatch;
	}
	// 8250182C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82501830: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82501834: 38A30014  addi r5, r3, 0x14
	ctx.r[5].s64 = ctx.r[3].s64 + 20;
	// 82501838: 3BE30050  addi r31, r3, 0x50
	ctx.r[31].s64 = ctx.r[3].s64 + 80;
	// 8250183C: 3BC30040  addi r30, r3, 0x40
	ctx.r[30].s64 = ctx.r[3].s64 + 64;
	// 82501840: C1491850  lfs f10, 0x1850(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(6224 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82501844: 3FA08282  lis r29, -0x7d7e
	ctx.r[29].s64 = -2105409536;
	// 82501848: C16BBFFC  lfs f11, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8250184C: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82501850: 557C073E  clrlwi r28, r11, 0x1c
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82501854: 557B06F6  rlwinm r27, r11, 0, 0x1b, 0x1b
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82501858: 5569CFFE  rlwinm r9, r11, 0x19, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 8250185C: 5568D7FE  rlwinm r8, r11, 0x1a, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82501860: 5567DFFE  rlwinm r7, r11, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82501864: FB81FFC8  std r28, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[28].u64 ) };
	// 82501868: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8250186C: C801FFC8  lfd f0, -0x38(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82501870: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82501874: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82501878: EDA0582A  fadds f13, f0, f11
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64;
	// 8250187C: C01DD25C  lfs f0, -0x2da4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-11684 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82501880: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82501884: EDA0503C  fnmsubs f13, f0, f0, f10
	ctx.f[13].f64 = -(((ctx.f[0].f64 * ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 82501888: EDA0682C  fsqrts f13, f13
	ctx.f[13].f64 = ((ctx.f[13].f64).sqrt() as f32) as f64;
	// 8250188C: 419A000C  beq cr6, 0x82501898
	if ctx.cr[6].eq {
	pc = 0x82501898; continue 'dispatch;
	}
	// 82501890: FD806890  fmr f12, f13
	ctx.f[12].f64 = ctx.f[13].f64;
	// 82501894: 4800000C  b 0x825018a0
	pc = 0x825018A0; continue 'dispatch;
	// 82501898: FD800090  fmr f12, f0
	ctx.f[12].f64 = ctx.f[0].f64;
	// 8250189C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 825018A0: D001FFC0  stfs f0, -0x40(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 825018A4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 825018A8: D181FFC4  stfs f12, -0x3c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 825018AC: 409A000C  bne cr6, 0x825018b8
	if !ctx.cr[6].eq {
	pc = 0x825018B8; continue 'dispatch;
	}
	// 825018B0: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 825018B4: D001FFC0  stfs f0, -0x40(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 825018B8: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 825018BC: 409A000C  bne cr6, 0x825018c8
	if !ctx.cr[6].eq {
	pc = 0x825018C8; continue 'dispatch;
	}
	// 825018C0: FC006050  fneg f0, f12
	ctx.f[0].u64 = ctx.f[12].u64 ^ 0x8000_0000_0000_0000u64;
	// 825018C4: D001FFC4  stfs f0, -0x3c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 825018C8: 3901FFC0  addi r8, r1, -0x40
	ctx.r[8].s64 = ctx.r[1].s64 + -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82501930 size=56
    let mut pc: u32 = 0x82501930;
    'dispatch: loop {
        match pc {
            0x82501930 => {
    //   block [0x82501930..0x82501968)
	// 82501930: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82501934: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82501938: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 8250193C: C00BBFFC  lfs f0, -0x4004(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82501940: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82501968 size=16
    let mut pc: u32 = 0x82501968;
    'dispatch: loop {
        match pc {
            0x82501968 => {
    //   block [0x82501968..0x82501978)
	// 82501968: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82501978 size=592
    let mut pc: u32 = 0x82501978;
    'dispatch: loop {
        match pc {
            0x82501978 => {
    //   block [0x82501978..0x82501BC8)
	// 82501978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250197C: 48033729  bl 0x825350a4
	ctx.lr = 0x82501980;
	sub_82535080(ctx, base);
	// 82501980: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82501984: C0030014  lfs f0, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82501988: 39230030  addi r9, r3, 0x30
	ctx.r[9].s64 = ctx.r[3].s64 + 48;
	// 8250198C: ED600032  fmuls f11, f0, f0
	ctx.f[11].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82501990: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82501994: D001FF90  stfs f0, -0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82501BC8 size=184
    let mut pc: u32 = 0x82501BC8;
    'dispatch: loop {
        match pc {
            0x82501BC8 => {
    //   block [0x82501BC8..0x82501C80)
	// 82501BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82501BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82501BD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82501BD4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82501BD8: D0430010  stfs f2, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82501BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82501BE0: FD600890  fmr f11, f1
	ctx.f[11].f64 = ctx.f[1].f64;
	// 82501BE4: 396B45D4  addi r11, r11, 0x45d4
	ctx.r[11].s64 = ctx.r[11].s64 + 17876;
	// 82501BE8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82501BEC: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82501BF0: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82501BF4: 3D208282  lis r9, -0x7d7e
	ctx.r[9].s64 = -2105409536;
	// 82501BF8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82501BFC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82501C00: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82501C04: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82501C08: C009D260  lfs f0, -0x2da0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-11680 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82501C0C: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82501C10: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82501C14: 40980018  bge cr6, 0x82501c2c
	if !ctx.cr[6].lt {
	pc = 0x82501C2C; continue 'dispatch;
	}
	// 82501C18: 4BFFF839  bl 0x82501450
	ctx.lr = 0x82501C1C;
	sub_82501450(ctx, base);
	// 82501C1C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82501C20: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82501C24: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82501C28: D009D260  stfs f0, -0x2da0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-11680 as u32), tmp.u32 ) };
	// 82501C2C: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82501C80 size=112
    let mut pc: u32 = 0x82501C80;
    'dispatch: loop {
        match pc {
            0x82501C80 => {
    //   block [0x82501C80..0x82501CF0)
	// 82501C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82501C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82501C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82501C8C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82501C90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82501C94: 396B45D4  addi r11, r11, 0x45d4
	ctx.r[11].s64 = ctx.r[11].s64 + 17876;
	// 82501C98: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82501C9C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82501CA0: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82501CA4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82501CA8: 419A0030  beq cr6, 0x82501cd8
	if ctx.cr[6].eq {
	pc = 0x82501CD8; continue 'dispatch;
	}
	// 82501CAC: 3D208282  lis r9, -0x7d7e
	ctx.r[9].s64 = -2105409536;
	// 82501CB0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82501CB4: C009D260  lfs f0, -0x2da0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-11680 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82501CB8: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82501CBC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82501CC0: 40980018  bge cr6, 0x82501cd8
	if !ctx.cr[6].lt {
	pc = 0x82501CD8; continue 'dispatch;
	}
	// 82501CC4: 4BFFF78D  bl 0x82501450
	ctx.lr = 0x82501CC8;
	sub_82501450(ctx, base);
	// 82501CC8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82501CCC: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82501CD0: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82501CD4: D009D260  stfs f0, -0x2da0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-11680 as u32), tmp.u32 ) };
	// 82501CD8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82501CDC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82501CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82501CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82501CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82501CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82501CF0 size=436
    let mut pc: u32 = 0x82501CF0;
    'dispatch: loop {
        match pc {
            0x82501CF0 => {
    //   block [0x82501CF0..0x82501EA4)
	// 82501CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82501CF4: 480333B5  bl 0x825350a8
	ctx.lr = 0x82501CF8;
	sub_82535080(ctx, base);
	// 82501CF8: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 82501CFC: EB640000  ld r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82501D00: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82501D04: 39240030  addi r9, r4, 0x30
	ctx.r[9].s64 = ctx.r[4].s64 + 48;
	// 82501D08: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82501D0C: 3901FF60  addi r8, r1, -0xa0
	ctx.r[8].s64 = ctx.r[1].s64 + -160;
	// 82501D10: 38E1FF70  addi r7, r1, -0x90
	ctx.r[7].s64 = ctx.r[1].s64 + -144;
	// 82501D14: EB4B0000  ld r26, 0(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82501D18: 38A1FF80  addi r5, r1, -0x80
	ctx.r[5].s64 = ctx.r[1].s64 + -128;
	// 82501D1C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82501D20: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82501D24: EB2A0000  ld r25, 0(r10)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82501D28: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82501D2C: FB680000  std r27, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 82501D30: 3B81FFA0  addi r28, r1, -0x60
	ctx.r[28].s64 = ctx.r[1].s64 + -96;
	// 82501D34: F8880008  std r4, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[4].u64 ) };
	// 82501D38: 3BA30030  addi r29, r3, 0x30
	ctx.r[29].s64 = ctx.r[3].s64 + 48;
	// 82501D3C: FB470000  std r26, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
	// 82501D40: F9670008  std r11, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82501D44: 3961FF60  addi r11, r1, -0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + -160;
	// 82501D48: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 82501D4C: FB250000  std r25, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
	// 82501D50: EB090000  ld r24, 0(r9)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82501D54: E9290008  ld r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82501EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82501EA8 size=1420
    let mut pc: u32 = 0x82501EA8;
    'dispatch: loop {
        match pc {
            0x82501EA8 => {
    //   block [0x82501EA8..0x82502434)
	// 82501EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82501EAC: 48033209  bl 0x825350b4
	ctx.lr = 0x82501EB0;
	sub_82535080(ctx, base);
	// 82501EB0: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 82501EB4: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82501EB8: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82501EBC: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82501EC0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82501EC4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82501EC8: 7FAA4A14  add r29, r10, r9
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82501ECC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82501ED0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82501ED4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82501ED8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82501EDC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82501EE0: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82501EE4: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82501EE8: 40980020  bge cr6, 0x82501f08
	if !ctx.cr[6].lt {
	pc = 0x82501F08; continue 'dispatch;
	}
	// 82501EEC: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82501EF0: 39084614  addi r8, r8, 0x4614
	ctx.r[8].s64 = ctx.r[8].s64 + 17940;
	// 82501EF4: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82501EF8: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82501EFC: 38E9000C  addi r7, r9, 0xc
	ctx.r[7].s64 = ctx.r[9].s64 + 12;
	// 82501F00: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82501F04: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82501F08: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 82501F0C: C00B0014  lfs f0, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82501F10: 392B0050  addi r9, r11, 0x50
	ctx.r[9].s64 = ctx.r[11].s64 + 80;
	// 82501F14: C1AB0010  lfs f13, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82501F18: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82501F1C: EFE0682A  fadds f31, f0, f13
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82501F20: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82501F24: 3B800030  li r28, 0x30
	ctx.r[28].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82502438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82502438 size=28
    let mut pc: u32 = 0x82502438;
    'dispatch: loop {
        match pc {
            0x82502438 => {
    //   block [0x82502438..0x82502454)
	// 82502438: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8250243C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82502440: 419A0014  beq cr6, 0x82502454
	if ctx.cr[6].eq {
		sub_82502454(ctx, base);
		return;
	}
	// 82502444: C004001C  lfs f0, 0x1c(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82502448: C1A3002C  lfs f13, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8250244C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82502450: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82502454(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82502454 size=132
    let mut pc: u32 = 0x82502454;
    'dispatch: loop {
        match pc {
            0x82502454 => {
    //   block [0x82502454..0x825024D8)
	// 82502454: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825024D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825024D8 size=4
    let mut pc: u32 = 0x825024D8;
    'dispatch: loop {
        match pc {
            0x825024D8 => {
    //   block [0x825024D8..0x825024DC)
	// 825024D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825024E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825024E0 size=8
    let mut pc: u32 = 0x825024E0;
    'dispatch: loop {
        match pc {
            0x825024E0 => {
    //   block [0x825024E0..0x825024E8)
	// 825024E0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 825024E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825024E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825024E8 size=260
    let mut pc: u32 = 0x825024E8;
    'dispatch: loop {
        match pc {
            0x825024E8 => {
    //   block [0x825024E8..0x825025EC)
	// 825024E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825024EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825024F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825024F4: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825024F8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 825024FC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82502500: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82502504: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82502508: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825025F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825025F0 size=8
    let mut pc: u32 = 0x825025F0;
    'dispatch: loop {
        match pc {
            0x825025F0 => {
    //   block [0x825025F0..0x825025F8)
	// 825025F0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 825025F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825025F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825025F8 size=108
    let mut pc: u32 = 0x825025F8;
    'dispatch: loop {
        match pc {
            0x825025F8 => {
    //   block [0x825025F8..0x82502664)
	// 825025F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825025FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82502600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82502604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82502608: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250260C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82502610: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82502614: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82502618: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250261C: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82502620: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82502624: 4E800421  bctrl
	ctx.lr = 0x82502628;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82502628: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82502668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82502668 size=128
    let mut pc: u32 = 0x82502668;
    'dispatch: loop {
        match pc {
            0x82502668 => {
    //   block [0x82502668..0x825026E8)
	// 82502668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250266C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82502670: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82502674: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82502678: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 8250267C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82502680: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82502684: 388B2690  addi r4, r11, 0x2690
	ctx.r[4].s64 = ctx.r[11].s64 + 9872;
	// 82502688: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8250268C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82502690: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82502694: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82502698: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250269C: 4E800421  bctrl
	ctx.lr = 0x825026A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825026A0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 825026A4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825026E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825026E8 size=220
    let mut pc: u32 = 0x825026E8;
    'dispatch: loop {
        match pc {
            0x825026E8 => {
    //   block [0x825026E8..0x825027C4)
	// 825026E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825026EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825026F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825026F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825026F8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825026FC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82502700: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82502704: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82502708: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8250270C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82502710: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82502714: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 82502718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8250271C: C01F0004  lfs f0, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82502720: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82502724: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82502728: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8250272C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82502730: 914100A0  stw r10, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 82502734: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82502738: 91410080  stw r10, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 8250273C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82502740: 4E800421  bctrl
	ctx.lr = 0x82502744;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82502744: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82502748: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250274C: 419A0060  beq cr6, 0x825027ac
	if ctx.cr[6].eq {
	pc = 0x825027AC; continue 'dispatch;
	}
	// 82502750: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82502754: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82502758: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8250275C: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82502760: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82502764: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82502768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825027C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825027C8 size=8
    let mut pc: u32 = 0x825027C8;
    'dispatch: loop {
        match pc {
            0x825027C8 => {
    //   block [0x825027C8..0x825027D0)
	// 825027C8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 825027CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825027D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825027D0 size=96
    let mut pc: u32 = 0x825027D0;
    'dispatch: loop {
        match pc {
            0x825027D0 => {
    //   block [0x825027D0..0x82502830)
	// 825027D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825027D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825027D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825027DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825027E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825027E4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825027E8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 825027EC: 388B4650  addi r4, r11, 0x4650
	ctx.r[4].s64 = ctx.r[11].s64 + 18000;
	// 825027F0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 825027F4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825027F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825027FC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82502800: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82502804: 4E800421  bctrl
	ctx.lr = 0x82502808;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82502808: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250280C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82502810: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82502814: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82502818: 4E800421  bctrl
	ctx.lr = 0x8250281C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250281C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82502820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82502824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82502828: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250282C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82502830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82502830 size=76
    let mut pc: u32 = 0x82502830;
    'dispatch: loop {
        match pc {
            0x82502830 => {
    //   block [0x82502830..0x8250287C)
	// 82502830: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250287C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250287C size=20
    let mut pc: u32 = 0x8250287C;
    'dispatch: loop {
        match pc {
            0x8250287C => {
    //   block [0x8250287C..0x82502890)
	// 8250287C: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82502890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82502890 size=12
    let mut pc: u32 = 0x82502890;
    'dispatch: loop {
        match pc {
            0x82502890 => {
    //   block [0x82502890..0x8250289C)
	// 82502890: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 82502894: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82502898: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250289C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250289C size=48
    let mut pc: u32 = 0x8250289C;
    'dispatch: loop {
        match pc {
            0x8250289C => {
    //   block [0x8250289C..0x825028CC)
	// 8250289C: 39230020  addi r9, r3, 0x20
	ctx.r[9].s64 = ctx.r[3].s64 + 32;
	// 825028A0: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825028A4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825028A8: 38840002  addi r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 2;
	// 825028AC: 65483F00  oris r8, r10, 0x3f00
	ctx.r[8].u64 = ctx.r[10].u64 | 1056964608;
	// 825028B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825028D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825028D0 size=56
    let mut pc: u32 = 0x825028D0;
    'dispatch: loop {
        match pc {
            0x825028D0 => {
    //   block [0x825028D0..0x82502908)
	// 825028D0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825028D4: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 825028D8: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 825028DC: C00BBFFC  lfs f0, -0x4004(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825028E0: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82502908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82502908 size=16
    let mut pc: u32 = 0x82502908;
    'dispatch: loop {
        match pc {
            0x82502908 => {
    //   block [0x82502908..0x82502918)
	// 82502908: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82502918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82502918 size=40
    let mut pc: u32 = 0x82502918;
    'dispatch: loop {
        match pc {
            0x82502918 => {
    //   block [0x82502918..0x82502940)
	// 82502918: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250291C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82502920: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82502924: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82502928: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82502940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82502940 size=272
    let mut pc: u32 = 0x82502940;
    'dispatch: loop {
        match pc {
            0x82502940 => {
    //   block [0x82502940..0x82502A50)
	// 82502940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82502944: 48032775  bl 0x825350b8
	ctx.lr = 0x82502948;
	sub_82535080(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82502A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82502A50 size=76
    let mut pc: u32 = 0x82502A50;
    'dispatch: loop {
        match pc {
            0x82502A50 => {
    //   block [0x82502A50..0x82502A9C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82502A9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82502A9C size=20
    let mut pc: u32 = 0x82502A9C;
    'dispatch: loop {
        match pc {
            0x82502A9C => {
    //   block [0x82502A9C..0x82502AB0)
	// 82502A9C: C1A1FFF4  lfs f13, -0xc(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82502AA0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82502AA4: 4198000C  blt cr6, 0x82502ab0
	if ctx.cr[6].lt {
		sub_82502AB0(ctx, base);
		return;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82502AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82502AB0 size=32
    let mut pc: u32 = 0x82502AB0;
    'dispatch: loop {
        match pc {
            0x82502AB0 => {
    //   block [0x82502AB0..0x82502AD0)
	// 82502AB0: 3961FFF4  addi r11, r1, -0xc
	ctx.r[11].s64 = ctx.r[1].s64 + -12;
	// 82502AB4: EC006824  fdivs f0, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82502AB8: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82502AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82502AD0 size=76
    let mut pc: u32 = 0x82502AD0;
    'dispatch: loop {
        match pc {
            0x82502AD0 => {
    //   block [0x82502AD0..0x82502B1C)
	// 82502AD0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82502AD4: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82502AD8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82502ADC: 396B291C  addi r11, r11, 0x291c
	ctx.r[11].s64 = ctx.r[11].s64 + 10524;
	// 82502AE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82502AE4: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82502AE8: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82502AEC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82502AF0: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82502AF4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82502AF8: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82502AFC: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82502B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82502B20 size=276
    let mut pc: u32 = 0x82502B20;
    'dispatch: loop {
        match pc {
            0x82502B20 => {
    //   block [0x82502B20..0x82502C34)
	// 82502B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82502B24: 48032589  bl 0x825350ac
	ctx.lr = 0x82502B28;
	sub_82535080(ctx, base);
	// 82502B28: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82502B2C: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 82502B30: EC00082A  fadds f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82502B34: D001FF60  stfs f0, -0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), tmp.u32 ) };
	// 82502B38: 3B81FF60  addi r28, r1, -0xa0
	ctx.r[28].s64 = ctx.r[1].s64 + -160;
	// 82502B3C: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82502B40: 39240030  addi r9, r4, 0x30
	ctx.r[9].s64 = ctx.r[4].s64 + 48;
	// 82502B44: EB6B0000  ld r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82502B48: 3901FF70  addi r8, r1, -0x90
	ctx.r[8].s64 = ctx.r[1].s64 + -144;
	// 82502B4C: 38E1FF80  addi r7, r1, -0x80
	ctx.r[7].s64 = ctx.r[1].s64 + -128;
	// 82502B50: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82502C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82502C38 size=1092
    let mut pc: u32 = 0x82502C38;
    'dispatch: loop {
        match pc {
            0x82502C38 => {
    //   block [0x82502C38..0x8250307C)
	// 82502C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82502C3C: 4803246D  bl 0x825350a8
	ctx.lr = 0x82502C40;
	sub_82535080(ctx, base);
	// 82502C40: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82502C44: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82502C48: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82502C4C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82502C50: 7F4B5214  add r26, r11, r10
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82502C54: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82502C58: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82502C5C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82502C60: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82502C64: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82502C68: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82502C6C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82502C70: 40980020  bge cr6, 0x82502c90
	if !ctx.cr[6].lt {
	pc = 0x82502C90; continue 'dispatch;
	}
	// 82502C74: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82502C78: 39294660  addi r9, r9, 0x4660
	ctx.r[9].s64 = ctx.r[9].s64 + 18016;
	// 82502C7C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82502C80: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82502C84: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82502C88: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82502C8C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82502C90: 3BBB0030  addi r29, r27, 0x30
	ctx.r[29].s64 = ctx.r[27].s64 + 48;
	// 82502C94: 3BDB0020  addi r30, r27, 0x20
	ctx.r[30].s64 = ctx.r[27].s64 + 32;
	// 82502C98: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82502C9C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82502CA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82502CA4: 4BFFFDAD  bl 0x82502a50
	ctx.lr = 0x82502CA8;
	sub_82502A50(ctx, base);
	// 82502CA8: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82503080 size=8
    let mut pc: u32 = 0x82503080;
    'dispatch: loop {
        match pc {
            0x82503080 => {
    //   block [0x82503080..0x82503088)
	// 82503080: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82503084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82503088 size=116
    let mut pc: u32 = 0x82503088;
    'dispatch: loop {
        match pc {
            0x82503088 => {
    //   block [0x82503088..0x825030FC)
	// 82503088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250308C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82503090: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82503094: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82503098: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250309C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825030A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825030A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825030A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825030AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825030B0: 4E800421  bctrl
	ctx.lr = 0x825030B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825030B4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 825030B8: 419A0028  beq cr6, 0x825030e0
	if ctx.cr[6].eq {
	pc = 0x825030E0; continue 'dispatch;
	}
	// 825030BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825030C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825030C4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825030C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825030CC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825030D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825030D4: 4E800421  bctrl
	ctx.lr = 0x825030D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825030D8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 825030DC: 409AFFE0  bne cr6, 0x825030bc
	if !ctx.cr[6].eq {
	pc = 0x825030BC; continue 'dispatch;
	}
	// 825030E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825030E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825030E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825030EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825030F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825030F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825030F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82503100 size=8
    let mut pc: u32 = 0x82503100;
    'dispatch: loop {
        match pc {
            0x82503100 => {
    //   block [0x82503100..0x82503108)
	// 82503100: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82503108 size=144
    let mut pc: u32 = 0x82503108;
    'dispatch: loop {
        match pc {
            0x82503108 => {
    //   block [0x82503108..0x82503198)
	// 82503108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250310C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82503110: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82503114: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82503118: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250311C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82503120: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82503124: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82503128: 388B4678  addi r4, r11, 0x4678
	ctx.r[4].s64 = ctx.r[11].s64 + 18040;
	// 8250312C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82503130: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503134: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82503138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250313C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503140: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82503144: 4E800421  bctrl
	ctx.lr = 0x82503148;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82503148: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250314C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82503150: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82503154: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82503158: 388B466C  addi r4, r11, 0x466c
	ctx.r[4].s64 = ctx.r[11].s64 + 18028;
	// 8250315C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82503160: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82503164: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82503168: 4E800421  bctrl
	ctx.lr = 0x8250316C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250316C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82503174: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82503178: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250317C: 4E800421  bctrl
	ctx.lr = 0x82503180;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82503180: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82503184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82503188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250318C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82503190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82503194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82503198 size=8
    let mut pc: u32 = 0x82503198;
    'dispatch: loop {
        match pc {
            0x82503198 => {
    //   block [0x82503198..0x825031A0)
	// 82503198: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 8250319C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825031A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825031A0 size=60
    let mut pc: u32 = 0x825031A0;
    'dispatch: loop {
        match pc {
            0x825031A0 => {
    //   block [0x825031A0..0x825031DC)
	// 825031A0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825031A4: 38830030  addi r4, r3, 0x30
	ctx.r[4].s64 = ctx.r[3].s64 + 48;
	// 825031A8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 825031AC: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 825031B0: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825031E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825031E0 size=176
    let mut pc: u32 = 0x825031E0;
    'dispatch: loop {
        match pc {
            0x825031E0 => {
    //   block [0x825031E0..0x82503290)
	// 825031E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825031E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825031E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825031EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825031F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825031F4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 825031F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825031FC: 396B2A8C  addi r11, r11, 0x2a8c
	ctx.r[11].s64 = ctx.r[11].s64 + 10892;
	// 82503200: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82503204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82503208: 38E0001C  li r7, 0x1c
	ctx.r[7].s64 = 28;
	// 8250320C: 394A2A68  addi r10, r10, 0x2a68
	ctx.r[10].s64 = ctx.r[10].s64 + 10856;
	// 82503210: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82503214: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82503218: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8250321C: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82503220: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82503224: 909F0014  stw r4, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82503228: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250322C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82503230: 419A0010  beq cr6, 0x82503240
	if ctx.cr[6].eq {
	pc = 0x82503240; continue 'dispatch;
	}
	// 82503234: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82503238: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250323C: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82503240: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82503290 size=108
    let mut pc: u32 = 0x82503290;
    'dispatch: loop {
        match pc {
            0x82503290 => {
    //   block [0x82503290..0x825032FC)
	// 82503290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82503294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82503298: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250329C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825032A0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825032A4: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825032A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825032AC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825032B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825032B4: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 825032B8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 825032BC: 480A5DDD  bl 0x825a9098
	ctx.lr = 0x825032C0;
	sub_825A9098(ctx, base);
	// 825032C0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 825032C4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825032C8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825032CC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825032D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825032D4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825032D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825032DC: 4E800421  bctrl
	ctx.lr = 0x825032E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825032E0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825032E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825032E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825032EC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 825032F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825032F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825032F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82503300 size=216
    let mut pc: u32 = 0x82503300;
    'dispatch: loop {
        match pc {
            0x82503300 => {
    //   block [0x82503300..0x825033D8)
	// 82503300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82503304: 48031DB5  bl 0x825350b8
	ctx.lr = 0x82503308;
	sub_82535080(ctx, base);
	// 82503308: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250330C: 3BE30030  addi r31, r3, 0x30
	ctx.r[31].s64 = ctx.r[3].s64 + 48;
	// 82503310: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82503314: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82503318: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8250331C: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 82503320: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825033D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825033D8 size=436
    let mut pc: u32 = 0x825033D8;
    'dispatch: loop {
        match pc {
            0x825033D8 => {
    //   block [0x825033D8..0x8250358C)
	// 825033D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825033DC: 48031CD9  bl 0x825350b4
	ctx.lr = 0x825033E0;
	sub_82535080(ctx, base);
	// 825033E0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825033E4: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825033E8: 3B800014  li r28, 0x14
	ctx.r[28].s64 = 20;
	// 825033EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825033F0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 825033F4: 7D7CD82E  lwzx r11, r28, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 825033F8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825033FC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82503400: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82503404: 40980020  bge cr6, 0x82503424
	if !ctx.cr[6].lt {
	pc = 0x82503424; continue 'dispatch;
	}
	// 82503408: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8250340C: 39294688  addi r9, r9, 0x4688
	ctx.r[9].s64 = ctx.r[9].s64 + 18056;
	// 82503410: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82503414: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82503418: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8250341C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82503420: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82503424: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82503428: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8250342C: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82503430: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82503434: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82503438: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8250343C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82503440: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82503444: 4200FFF0  bdnz 0x82503434
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82503434; continue 'dispatch;
	}
	// 82503448: 3BE40030  addi r31, r4, 0x30
	ctx.r[31].s64 = ctx.r[4].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82503590 size=384
    let mut pc: u32 = 0x82503590;
    'dispatch: loop {
        match pc {
            0x82503590 => {
    //   block [0x82503590..0x82503710)
	// 82503590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82503594: 48031B21  bl 0x825350b4
	ctx.lr = 0x82503598;
	sub_82535080(ctx, base);
	// 82503598: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250359C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825035A0: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 825035A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825035A8: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 825035AC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825035B0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 825035B4: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825035B8: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825035BC: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825035C0: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 825035C4: 40980020  bge cr6, 0x825035e4
	if !ctx.cr[6].lt {
	pc = 0x825035E4; continue 'dispatch;
	}
	// 825035C8: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 825035CC: 39084688  addi r8, r8, 0x4688
	ctx.r[8].s64 = ctx.r[8].s64 + 18056;
	// 825035D0: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825035D4: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 825035D8: 38E9000C  addi r7, r9, 0xc
	ctx.r[7].s64 = ctx.r[9].s64 + 12;
	// 825035DC: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 825035E0: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 825035E4: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 825035E8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 825035EC: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825035F0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 825035F4: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 825035F8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 825035FC: F9090000  std r8, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82503600: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82503604: 4200FFF0  bdnz 0x825035f4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825035F4; continue 'dispatch;
	}
	// 82503608: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82503710 size=8
    let mut pc: u32 = 0x82503710;
    'dispatch: loop {
        match pc {
            0x82503710 => {
    //   block [0x82503710..0x82503718)
	// 82503710: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82503714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82503718 size=156
    let mut pc: u32 = 0x82503718;
    'dispatch: loop {
        match pc {
            0x82503718 => {
    //   block [0x82503718..0x825037B4)
	// 82503718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250371C: 48031995  bl 0x825350b0
	ctx.lr = 0x82503720;
	sub_82535080(ctx, base);
	// 82503720: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82503724: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82503728: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8250372C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82503730: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82503734: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82503738: 3B6BFFFF  addi r27, r11, -1
	ctx.r[27].s64 = ctx.r[11].s64 + -1;
	// 8250373C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82503740: 4198004C  blt cr6, 0x8250378c
	if ctx.cr[6].lt {
	pc = 0x8250378C; continue 'dispatch;
	}
	// 82503744: 577F103A  slwi r31, r27, 2
	ctx.r[31].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82503748: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 8250374C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82503750: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82503754: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82503758: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8250375C: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82503760: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503764: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503768: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250376C: 4E800421  bctrl
	ctx.lr = 0x82503770;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82503770: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503774: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82503778: 419A0028  beq cr6, 0x825037a0
	if ctx.cr[6].eq {
	pc = 0x825037A0; continue 'dispatch;
	}
	// 8250377C: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 82503780: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82503784: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82503788: 4098FFC0  bge cr6, 0x82503748
	if !ctx.cr[6].lt {
	pc = 0x82503748; continue 'dispatch;
	}
	// 8250378C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82503790: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82503794: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82503798: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8250379C: 48031964  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 825037A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825037A4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825037A8: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825037AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825037B0: 48031950  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825037B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825037B8 size=180
    let mut pc: u32 = 0x825037B8;
    'dispatch: loop {
        match pc {
            0x825037B8 => {
    //   block [0x825037B8..0x8250386C)
	// 825037B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825037BC: 480318E9  bl 0x825350a4
	ctx.lr = 0x825037C0;
	sub_82535080(ctx, base);
	// 825037C0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825037C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825037C8: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 825037CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825037D0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825037D4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 825037D8: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 825037DC: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 825037E0: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 825037E4: 3B0BFFFF  addi r24, r11, -1
	ctx.r[24].s64 = ctx.r[11].s64 + -1;
	// 825037E8: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 825037EC: 41980058  blt cr6, 0x82503844
	if ctx.cr[6].lt {
	pc = 0x82503844; continue 'dispatch;
	}
	// 825037F0: 571E103A  slwi r30, r24, 2
	ctx.r[30].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 825037F4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 825037F8: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 825037FC: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82503800: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82503804: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82503808: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8250380C: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82503810: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82503814: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82503818: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250381C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503820: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82503824: 4E800421  bctrl
	ctx.lr = 0x82503828;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82503828: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250382C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82503830: 419A0028  beq cr6, 0x82503858
	if ctx.cr[6].eq {
	pc = 0x82503858; continue 'dispatch;
	}
	// 82503834: 3B18FFFF  addi r24, r24, -1
	ctx.r[24].s64 = ctx.r[24].s64 + -1;
	// 82503838: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 8250383C: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82503840: 4098FFB4  bge cr6, 0x825037f4
	if !ctx.cr[6].lt {
	pc = 0x825037F4; continue 'dispatch;
	}
	// 82503844: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82503848: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8250384C: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82503850: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82503854: 480318A0  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 82503858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250385C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82503860: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82503864: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82503868: 4803188C  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82503870 size=172
    let mut pc: u32 = 0x82503870;
    'dispatch: loop {
        match pc {
            0x82503870 => {
    //   block [0x82503870..0x8250391C)
	// 82503870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82503874: 48031835  bl 0x825350a8
	ctx.lr = 0x82503878;
	sub_82535080(ctx, base);
	// 82503878: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250387C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82503880: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82503884: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82503888: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8250388C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82503890: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82503894: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82503898: 3B2BFFFF  addi r25, r11, -1
	ctx.r[25].s64 = ctx.r[11].s64 + -1;
	// 8250389C: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 825038A0: 41980054  blt cr6, 0x825038f4
	if ctx.cr[6].lt {
	pc = 0x825038F4; continue 'dispatch;
	}
	// 825038A4: 573F103A  slwi r31, r25, 2
	ctx.r[31].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 825038A8: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 825038AC: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 825038B0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 825038B4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 825038B8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825038BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825038C0: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 825038C4: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 825038C8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825038CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825038D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825038D4: 4E800421  bctrl
	ctx.lr = 0x825038D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825038D8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825038DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825038E0: 419A0028  beq cr6, 0x82503908
	if ctx.cr[6].eq {
	pc = 0x82503908; continue 'dispatch;
	}
	// 825038E4: 3B39FFFF  addi r25, r25, -1
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	// 825038E8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 825038EC: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 825038F0: 4098FFB8  bge cr6, 0x825038a8
	if !ctx.cr[6].lt {
	pc = 0x825038A8; continue 'dispatch;
	}
	// 825038F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825038F8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 825038FC: 99780000  stb r11, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82503900: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82503904: 480317F4  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82503908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250390C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82503910: 99780000  stb r11, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82503914: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82503918: 480317E0  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82503920 size=196
    let mut pc: u32 = 0x82503920;
    'dispatch: loop {
        match pc {
            0x82503920 => {
    //   block [0x82503920..0x825039E4)
	// 82503920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82503924: 48031779  bl 0x8253509c
	ctx.lr = 0x82503928;
	sub_82535080(ctx, base);
	// 82503928: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250392C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82503930: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82503934: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82503938: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8250393C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82503940: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82503944: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82503948: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 8250394C: 3ACBFFFF  addi r22, r11, -1
	ctx.r[22].s64 = ctx.r[11].s64 + -1;
	// 82503950: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 82503954: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82503958: 41980064  blt cr6, 0x825039bc
	if ctx.cr[6].lt {
	pc = 0x825039BC; continue 'dispatch;
	}
	// 8250395C: 82E10124  lwz r23, 0x124(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 82503960: 56DE103A  slwi r30, r22, 2
	ctx.r[30].u32 = ctx.r[22].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82503964: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82503968: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 8250396C: 92E10054  stw r23, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[23].u32 ) };
	// 82503970: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 82503974: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82503978: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8250397C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82503980: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82503984: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82503988: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250398C: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82503990: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503994: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503998: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250399C: 4E800421  bctrl
	ctx.lr = 0x825039A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825039A0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825039A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825039A8: 419A0028  beq cr6, 0x825039d0
	if ctx.cr[6].eq {
	pc = 0x825039D0; continue 'dispatch;
	}
	// 825039AC: 3AD6FFFF  addi r22, r22, -1
	ctx.r[22].s64 = ctx.r[22].s64 + -1;
	// 825039B0: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 825039B4: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 825039B8: 4098FFAC  bge cr6, 0x82503964
	if !ctx.cr[6].lt {
	pc = 0x82503964; continue 'dispatch;
	}
	// 825039BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825039C0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 825039C4: 99750000  stb r11, 0(r21)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825039C8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 825039CC: 48031720  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
	// 825039D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825039D4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 825039D8: 99750000  stb r11, 0(r21)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825039DC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 825039E0: 4803170C  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825039E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825039E8 size=156
    let mut pc: u32 = 0x825039E8;
    'dispatch: loop {
        match pc {
            0x825039E8 => {
    //   block [0x825039E8..0x82503A84)
	// 825039E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825039EC: 480316C5  bl 0x825350b0
	ctx.lr = 0x825039F0;
	sub_82535080(ctx, base);
	// 825039F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825039F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825039F8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 825039FC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82503A00: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82503A04: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82503A08: 3B6BFFFF  addi r27, r11, -1
	ctx.r[27].s64 = ctx.r[11].s64 + -1;
	// 82503A0C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82503A10: 4198004C  blt cr6, 0x82503a5c
	if ctx.cr[6].lt {
	pc = 0x82503A5C; continue 'dispatch;
	}
	// 82503A14: 577F103A  slwi r31, r27, 2
	ctx.r[31].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82503A18: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82503A1C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82503A20: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82503A24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82503A28: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82503A2C: 388B0014  addi r4, r11, 0x14
	ctx.r[4].s64 = ctx.r[11].s64 + 20;
	// 82503A30: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503A34: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503A38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82503A3C: 4E800421  bctrl
	ctx.lr = 0x82503A40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82503A40: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503A44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82503A48: 419A0028  beq cr6, 0x82503a70
	if ctx.cr[6].eq {
	pc = 0x82503A70; continue 'dispatch;
	}
	// 82503A4C: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 82503A50: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82503A54: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82503A58: 4098FFC0  bge cr6, 0x82503a18
	if !ctx.cr[6].lt {
	pc = 0x82503A18; continue 'dispatch;
	}
	// 82503A5C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82503A60: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82503A64: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82503A68: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82503A6C: 48031694  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 82503A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82503A74: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82503A78: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82503A7C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82503A80: 48031680  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82503A88 size=104
    let mut pc: u32 = 0x82503A88;
    'dispatch: loop {
        match pc {
            0x82503A88 => {
    //   block [0x82503A88..0x82503AF0)
	// 82503A88: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82503A8C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82503A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82503A94: 81090034  lwz r8, 0x34(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 82503A98: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82503A9C: 40990024  ble cr6, 0x82503ac0
	if !ctx.cr[6].gt {
	pc = 0x82503AC0; continue 'dispatch;
	}
	// 82503AA0: 81490030  lwz r10, 0x30(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(48 as u32) ) } as u64;
	// 82503AA4: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503AA8: 7F071840  cmplw cr6, r7, r3
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82503AAC: 419A0018  beq cr6, 0x82503ac4
	if ctx.cr[6].eq {
	pc = 0x82503AC4; continue 'dispatch;
	}
	// 82503AB0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82503AB4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82503AB8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82503ABC: 4198FFE8  blt cr6, 0x82503aa4
	if ctx.cr[6].lt {
	pc = 0x82503AA4; continue 'dispatch;
	}
	// 82503AC0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82503AC4: 81490034  lwz r10, 0x34(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 82503AC8: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82503ACC: 81690030  lwz r11, 0x30(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(48 as u32) ) } as u64;
	// 82503AD0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82503AD4: 5547103A  slwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82503AD8: 91490034  stw r10, 0x34(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82503ADC: 7D47582E  lwzx r10, r7, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82503AE0: 7D48592E  stwx r10, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82503AE4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503AE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82503AEC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82503AF0 size=24
    let mut pc: u32 = 0x82503AF0;
    'dispatch: loop {
        match pc {
            0x82503AF0 => {
    //   block [0x82503AF0..0x82503B08)
	// 82503AF0: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82503AF4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82503AF8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82503AFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82503B00: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82503B04: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82503B08 size=20
    let mut pc: u32 = 0x82503B08;
    'dispatch: loop {
        match pc {
            0x82503B08 => {
    //   block [0x82503B08..0x82503B1C)
	// 82503B08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503B0C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82503B10: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503B14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82503B18: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503B1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82503B1C size=4
    let mut pc: u32 = 0x82503B1C;
    'dispatch: loop {
        match pc {
            0x82503B1C => {
    //   block [0x82503B1C..0x82503B20)
	// 82503B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82503B20 size=364
    let mut pc: u32 = 0x82503B20;
    'dispatch: loop {
        match pc {
            0x82503B20 => {
    //   block [0x82503B20..0x82503C8C)
	// 82503B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82503B24: 48031595  bl 0x825350b8
	ctx.lr = 0x82503B28;
	sub_82535080(ctx, base);
	// 82503B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82503B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82503B30: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82503B34: 4BFFAC8D  bl 0x824fe7c0
	ctx.lr = 0x82503B38;
	sub_824FE7C0(ctx, base);
	// 82503B38: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82503B3C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82503B40: 396B2B04  addi r11, r11, 0x2b04
	ctx.r[11].s64 = ctx.r[11].s64 + 11012;
	// 82503B44: 394A2AF8  addi r10, r10, 0x2af8
	ctx.r[10].s64 = ctx.r[10].s64 + 11000;
	// 82503B48: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82503B4C: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82503B50: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 82503B54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82503B58: 39292AE4  addi r9, r9, 0x2ae4
	ctx.r[9].s64 = ctx.r[9].s64 + 10980;
	// 82503B5C: 39082AD8  addi r8, r8, 0x2ad8
	ctx.r[8].s64 = ctx.r[8].s64 + 10968;
	// 82503B60: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82503B64: 38E72ACC  addi r7, r7, 0x2acc
	ctx.r[7].s64 = ctx.r[7].s64 + 10956;
	// 82503B68: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82503B6C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82503B70: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82503B74: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82503B78: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82503B7C: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82503B80: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82503B84: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82503B88: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82503B8C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82503B90: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82503B94: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82503B98: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503B9C: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82503BA0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82503BA4: 40980060  bge cr6, 0x82503c04
	if !ctx.cr[6].lt {
	pc = 0x82503C04; continue 'dispatch;
	}
	// 82503BA8: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82503BAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82503BB0: 409A0020  bne cr6, 0x82503bd0
	if !ctx.cr[6].eq {
	pc = 0x82503BD0; continue 'dispatch;
	}
	// 82503BB4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503BB8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82503BBC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82503BC0: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503BC4: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82503BC8: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82503BCC: 4BF604ED  bl 0x824640b8
	ctx.lr = 0x82503BD0;
	sub_824640B8(ctx, base);
	// 82503BD0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503BD4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82503BD8: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503BDC: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82503BE0: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82503BE4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82503BE8: 4BF60451  bl 0x82464038
	ctx.lr = 0x82503BEC;
	sub_82464038(ctx, base);
	// 82503BEC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82503BF0: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82503BF4: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503BF8: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82503BFC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82503C00: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82503C04: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503C08: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503C0C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82503C10: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82503C14: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503C18: 40990020  ble cr6, 0x82503c38
	if !ctx.cr[6].gt {
	pc = 0x82503C38; continue 'dispatch;
	}
	// 82503C1C: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82503C20: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82503C24: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82503C28: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82503C2C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82503C30: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82503C34: 409AFFEC  bne cr6, 0x82503c20
	if !ctx.cr[6].eq {
	pc = 0x82503C20; continue 'dispatch;
	}
	// 82503C38: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82503C3C: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82503C40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82503C44: 4099003C  ble cr6, 0x82503c80
	if !ctx.cr[6].gt {
	pc = 0x82503C80; continue 'dispatch;
	}
	// 82503C48: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82503C4C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503C50: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82503C54: A10B0004  lhz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503C58: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82503C5C: 419A0010  beq cr6, 0x82503c6c
	if ctx.cr[6].eq {
	pc = 0x82503C6C; continue 'dispatch;
	}
	// 82503C60: A10B0006  lhz r8, 6(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82503C64: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82503C68: B10B0006  sth r8, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82503C6C: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82503C70: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82503C74: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82503C78: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82503C7C: 4198FFD0  blt cr6, 0x82503c4c
	if ctx.cr[6].lt {
	pc = 0x82503C4C; continue 'dispatch;
	}
	// 82503C80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82503C84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82503C88: 48031480  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82503C90 size=292
    let mut pc: u32 = 0x82503C90;
    'dispatch: loop {
        match pc {
            0x82503C90 => {
    //   block [0x82503C90..0x82503DB4)
	// 82503C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82503C94: 48031429  bl 0x825350bc
	ctx.lr = 0x82503C98;
	sub_82535080(ctx, base);
	// 82503C98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82503C9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82503CA0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82503CA4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82503CA8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82503CAC: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82503CB0: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 82503CB4: 80DF0034  lwz r6, 0x34(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82503CB8: 396B2B04  addi r11, r11, 0x2b04
	ctx.r[11].s64 = ctx.r[11].s64 + 11012;
	// 82503CBC: 394A2AF8  addi r10, r10, 0x2af8
	ctx.r[10].s64 = ctx.r[10].s64 + 11000;
	// 82503CC0: 39292AE4  addi r9, r9, 0x2ae4
	ctx.r[9].s64 = ctx.r[9].s64 + 10980;
	// 82503CC4: 39082AD8  addi r8, r8, 0x2ad8
	ctx.r[8].s64 = ctx.r[8].s64 + 10968;
	// 82503CC8: 38E72ACC  addi r7, r7, 0x2acc
	ctx.r[7].s64 = ctx.r[7].s64 + 10956;
	// 82503CCC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82503CD0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82503CD4: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82503CD8: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82503CDC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82503CE0: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82503CE4: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82503CE8: 4099005C  ble cr6, 0x82503d44
	if !ctx.cr[6].gt {
	pc = 0x82503D44; continue 'dispatch;
	}
	// 82503CEC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82503CF0: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82503CF4: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82503CF8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503CFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82503D00: 419A0030  beq cr6, 0x82503d30
	if ctx.cr[6].eq {
	pc = 0x82503D30; continue 'dispatch;
	}
	// 82503D04: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82503D08: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82503D0C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82503D10: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82503D14: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82503D18: 409A0018  bne cr6, 0x82503d30
	if !ctx.cr[6].eq {
	pc = 0x82503D30; continue 'dispatch;
	}
	// 82503D1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503D20: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82503D24: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503D28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82503D2C: 4E800421  bctrl
	ctx.lr = 0x82503D30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82503D30: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82503D34: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82503D38: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82503D3C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82503D40: 4198FFB0  blt cr6, 0x82503cf0
	if ctx.cr[6].lt {
	pc = 0x82503CF0; continue 'dispatch;
	}
	// 82503D44: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82503D48: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82503D4C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82503D50: 409A0020  bne cr6, 0x82503d70
	if !ctx.cr[6].eq {
	pc = 0x82503D70; continue 'dispatch;
	}
	// 82503D54: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503D58: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82503D5C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82503D60: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82503D64: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82503D68: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82503D6C: 4BF6034D  bl 0x824640b8
	ctx.lr = 0x82503D70;
	sub_824640B8(ctx, base);
	// 82503D70: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82503D74: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82503D78: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82503D7C: 396BFAD4  addi r11, r11, -0x52c
	ctx.r[11].s64 = ctx.r[11].s64 + -1324;
	// 82503D80: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82503D84: 394AFAF4  addi r10, r10, -0x50c
	ctx.r[10].s64 = ctx.r[10].s64 + -1292;
	// 82503D88: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82503D8C: 3929FAE0  addi r9, r9, -0x520
	ctx.r[9].s64 = ctx.r[9].s64 + -1312;
	// 82503D90: 3908FAB8  addi r8, r8, -0x548
	ctx.r[8].s64 = ctx.r[8].s64 + -1352;
	// 82503D94: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82503D98: 38E76DD0  addi r7, r7, 0x6dd0
	ctx.r[7].s64 = ctx.r[7].s64 + 28112;
	// 82503D9C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82503DA0: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82503DA4: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82503DA8: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82503DAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82503DB0: 4803135C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82503DB8 size=128
    let mut pc: u32 = 0x82503DB8;
    'dispatch: loop {
        match pc {
            0x82503DB8 => {
    //   block [0x82503DB8..0x82503E38)
	// 82503DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82503DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82503DC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82503DC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82503DC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82503DCC: 4BFFA9F5  bl 0x824fe7c0
	ctx.lr = 0x82503DD0;
	sub_824FE7C0(ctx, base);
	// 82503DD0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82503DD4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82503DD8: 396B2B04  addi r11, r11, 0x2b04
	ctx.r[11].s64 = ctx.r[11].s64 + 11012;
	// 82503DDC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82503DE0: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82503DE4: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 82503DE8: 394A2AF8  addi r10, r10, 0x2af8
	ctx.r[10].s64 = ctx.r[10].s64 + 11000;
	// 82503DEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82503DF0: 39292AE4  addi r9, r9, 0x2ae4
	ctx.r[9].s64 = ctx.r[9].s64 + 10980;
	// 82503DF4: 39082AD8  addi r8, r8, 0x2ad8
	ctx.r[8].s64 = ctx.r[8].s64 + 10968;
	// 82503DF8: 38E72ACC  addi r7, r7, 0x2acc
	ctx.r[7].s64 = ctx.r[7].s64 + 10956;
	// 82503DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82503E00: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82503E04: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82503E08: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82503E0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82503E10: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82503E14: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82503E18: 90DF0030  stw r6, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[6].u32 ) };
	// 82503E1C: 90DF0034  stw r6, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[6].u32 ) };
	// 82503E20: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82503E24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82503E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82503E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82503E30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82503E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82503E38 size=136
    let mut pc: u32 = 0x82503E38;
    'dispatch: loop {
        match pc {
            0x82503E38 => {
    //   block [0x82503E38..0x82503EC0)
	// 82503E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82503E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82503E40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82503E44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82503E48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82503E4C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82503E50: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503E54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82503E58: 419A0010  beq cr6, 0x82503e68
	if ctx.cr[6].eq {
	pc = 0x82503E68; continue 'dispatch;
	}
	// 82503E5C: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82503E60: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82503E64: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82503E68: 3BE30030  addi r31, r3, 0x30
	ctx.r[31].s64 = ctx.r[3].s64 + 48;
	// 82503E6C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82503E70: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503E74: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82503E78: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82503E7C: 409A0010  bne cr6, 0x82503e8c
	if !ctx.cr[6].eq {
	pc = 0x82503E8C; continue 'dispatch;
	}
	// 82503E80: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82503E84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82503E88: 4BF6A4C9  bl 0x8246e350
	ctx.lr = 0x82503E8C;
	sub_8246E350(ctx, base);
	// 82503E8C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503E90: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503E94: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82503E98: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82503E9C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503EA0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82503EA4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82503EA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82503EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82503EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82503EB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82503EB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82503EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82503EC0 size=16
    let mut pc: u32 = 0x82503EC0;
    'dispatch: loop {
        match pc {
            0x82503EC0 => {
    //   block [0x82503EC0..0x82503ED0)
	// 82503EC0: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82503ED0 size=4
    let mut pc: u32 = 0x82503ED0;
    'dispatch: loop {
        match pc {
            0x82503ED0 => {
    //   block [0x82503ED0..0x82503ED4)
	// 82503ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82503ED8 size=4
    let mut pc: u32 = 0x82503ED8;
    'dispatch: loop {
        match pc {
            0x82503ED8 => {
    //   block [0x82503ED8..0x82503EDC)
	// 82503ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82503EE0 size=216
    let mut pc: u32 = 0x82503EE0;
    'dispatch: loop {
        match pc {
            0x82503EE0 => {
    //   block [0x82503EE0..0x82503FB8)
	// 82503EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82503EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82503EE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82503EEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82503EF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82503EF4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82503EF8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82503EFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82503F00: 388B46BC  addi r4, r11, 0x46bc
	ctx.r[4].s64 = ctx.r[11].s64 + 18108;
	// 82503F04: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82503F08: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503F0C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82503F10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82503F14: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82503F18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82503F1C: 4E800421  bctrl
	ctx.lr = 0x82503F20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82503F20: 80DE0050  lwz r6, 0x50(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 82503F24: 397E0070  addi r11, r30, 0x70
	ctx.r[11].s64 = ctx.r[30].s64 + 112;
	// 82503F28: 7F065840  cmplw cr6, r6, r11
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82503F2C: 419A0030  beq cr6, 0x82503f5c
	if ctx.cr[6].eq {
	pc = 0x82503F5C; continue 'dispatch;
	}
	// 82503F30: 815E0054  lwz r10, 0x54(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 82503F34: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82503F38: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503F3C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82503F40: 55483032  slwi r8, r10, 6
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82503F44: 388B46A8  addi r4, r11, 0x46a8
	ctx.r[4].s64 = ctx.r[11].s64 + 18088;
	// 82503F48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82503F4C: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82503F50: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82503F54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82503F58: 4E800421  bctrl
	ctx.lr = 0x82503F5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82503F5C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82503F60: 815E005C  lwz r10, 0x5c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82503F64: 80DE0058  lwz r6, 0x58(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 82503F68: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82503F6C: 388B4698  addi r4, r11, 0x4698
	ctx.r[4].s64 = ctx.r[11].s64 + 18072;
	// 82503F70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503F74: 55483032  slwi r8, r10, 6
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82503F78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82503F7C: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82503F80: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82503F84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82503F88: 4E800421  bctrl
	ctx.lr = 0x82503F8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82503F8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82503F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82503F94: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82503F98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82503F9C: 4E800421  bctrl
	ctx.lr = 0x82503FA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82503FA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82503FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82503FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82503FAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82503FB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82503FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82503FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82503FB8 size=244
    let mut pc: u32 = 0x82503FB8;
    'dispatch: loop {
        match pc {
            0x82503FB8 => {
    //   block [0x82503FB8..0x825040AC)
	// 82503FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82503FBC: 480310F1  bl 0x825350ac
	ctx.lr = 0x82503FC0;
	sub_82535080(ctx, base);
	// 82503FC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82503FC4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82503FC8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82503FCC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82503FD0: 817A005C  lwz r11, 0x5c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(92 as u32) ) } as u64;
	// 82503FD4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82503FD8: 409900BC  ble cr6, 0x82504094
	if !ctx.cr[6].gt {
	pc = 0x82504094; continue 'dispatch;
	}
	// 82503FDC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82503FE0: 817A0058  lwz r11, 0x58(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(88 as u32) ) } as u64;
	// 82503FE4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82503FE8: 7FFB5A14  add r31, r27, r11
	ctx.r[31].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82503FEC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82503FF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82503FF4: 4099008C  ble cr6, 0x82504080
	if !ctx.cr[6].gt {
	pc = 0x82504080; continue 'dispatch;
	}
	// 82503FF8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82503FFC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82504000: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 82504004: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82504008: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8250400C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82504010: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82504014: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82504018: 4E800421  bctrl
	ctx.lr = 0x8250401C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250401C: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82504020: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82504024: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82504028: 409A0018  bne cr6, 0x82504040
	if !ctx.cr[6].eq {
	pc = 0x82504040; continue 'dispatch;
	}
	// 8250402C: 895F0019  lbz r10, 0x19(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82504030: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82504034: 409A000C  bne cr6, 0x82504040
	if !ctx.cr[6].eq {
	pc = 0x82504040; continue 'dispatch;
	}
	// 82504038: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 8250403C: 48000020  b 0x8250405c
	pc = 0x8250405C; continue 'dispatch;
	// 82504040: 895F0019  lbz r10, 0x19(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82504044: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82504048: 409A0010  bne cr6, 0x82504058
	if !ctx.cr[6].eq {
	pc = 0x82504058; continue 'dispatch;
	}
	// 8250404C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82504050: 396001D0  li r11, 0x1d0
	ctx.r[11].s64 = 464;
	// 82504054: 409A0008  bne cr6, 0x8250405c
	if !ctx.cr[6].eq {
	pc = 0x8250405C; continue 'dispatch;
	}
	// 82504058: 396001A0  li r11, 0x1a0
	ctx.r[11].s64 = 416;
	// 8250405C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82504060: 41980040  blt cr6, 0x825040a0
	if ctx.cr[6].lt {
	pc = 0x825040A0; continue 'dispatch;
	}
	// 82504064: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82504068: 41990038  bgt cr6, 0x825040a0
	if ctx.cr[6].gt {
	pc = 0x825040A0; continue 'dispatch;
	}
	// 8250406C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82504070: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82504074: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82504078: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250407C: 4198FF80  blt cr6, 0x82503ffc
	if ctx.cr[6].lt {
	pc = 0x82503FFC; continue 'dispatch;
	}
	// 82504080: 817A005C  lwz r11, 0x5c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(92 as u32) ) } as u64;
	// 82504084: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82504088: 3B7B0040  addi r27, r27, 0x40
	ctx.r[27].s64 = ctx.r[27].s64 + 64;
	// 8250408C: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82504090: 4198FF50  blt cr6, 0x82503fe0
	if ctx.cr[6].lt {
	pc = 0x82503FE0; continue 'dispatch;
	}
	// 82504094: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 82504098: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8250409C: 48031060  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 825040A0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 825040A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825040A8: 48031054  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825040B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825040B0 size=60
    let mut pc: u32 = 0x825040B0;
    'dispatch: loop {
        match pc {
            0x825040B0 => {
    //   block [0x825040B0..0x825040EC)
	// 825040B0: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825040B4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 825040B8: 54880000  rlwinm r8, r4, 0, 0, 0
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 825040BC: 5489007E  clrlwi r9, r4, 1
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x7FFFFFFFu64;
	// 825040C0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825040C4: 210B0020  subfic r8, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[11].s64;
	// 825040C8: 7D4B5C30  srw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 825040CC: 7D672038  and r7, r11, r4
	ctx.r[7].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 825040D0: 7D2B4430  srw r11, r9, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 825040D4: 409A0020  bne cr6, 0x825040f4
	if !ctx.cr[6].eq {
		sub_825040F4(ctx, base);
		return;
	}
	// 825040D8: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 825040DC: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 825040E0: 409A000C  bne cr6, 0x825040ec
	if !ctx.cr[6].eq {
		sub_825040EC(ctx, base);
		return;
	}
	// 825040E4: 39630070  addi r11, r3, 0x70
	ctx.r[11].s64 = ctx.r[3].s64 + 112;
	// 825040E8: 48000018  b 0x82504100
	sub_825040F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825040EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825040EC size=8
    let mut pc: u32 = 0x825040EC;
    'dispatch: loop {
        match pc {
            0x825040EC => {
    //   block [0x825040EC..0x825040F4)
	// 825040EC: 81430050  lwz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 825040F0: 48000008  b 0x825040f8
	sub_825040F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825040F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825040F4 size=72
    let mut pc: u32 = 0x825040F4;
    'dispatch: loop {
        match pc {
            0x825040F4 => {
    //   block [0x825040F4..0x8250413C)
	// 825040F4: 81430058  lwz r10, 0x58(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 825040F8: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825040FC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82504100: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82504104: A10B0002  lhz r8, 2(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82504108: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250410C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82504110: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 82504114: 419A0038  beq cr6, 0x8250414c
	if ctx.cr[6].eq {
		sub_8250414C(ctx, base);
		return;
	}
	// 82504118: 88AB0001  lbz r5, 1(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8250411C: A0CB0008  lhz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82504120: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 82504124: 7D6639D6  mullw r11, r6, r7
	ctx.r[11].s64 = (ctx.r[6].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82504128: 409A0014  bne cr6, 0x8250413c
	if !ctx.cr[6].eq {
		sub_8250413C(ctx, base);
		return;
	}
	// 8250412C: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82504130: 7D6B41D6  mullw r11, r11, r8
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82504134: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82504138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250413C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250413C size=16
    let mut pc: u32 = 0x8250413C;
    'dispatch: loop {
        match pc {
            0x8250413C => {
    //   block [0x8250413C..0x8250414C)
	// 8250413C: 7D6B522E  lhzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82504140: 7D6B41D6  mullw r11, r11, r8
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82504144: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82504148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250414C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250414C size=8
    let mut pc: u32 = 0x8250414C;
    'dispatch: loop {
        match pc {
            0x8250414C => {
    //   block [0x8250414C..0x82504154)
	// 8250414C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82504150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82504158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82504158 size=68
    let mut pc: u32 = 0x82504158;
    'dispatch: loop {
        match pc {
            0x82504158 => {
    //   block [0x82504158..0x8250419C)
	// 82504158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250415C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82504160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82504164: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82504168: 4BFFFF49  bl 0x825040b0
	ctx.lr = 0x8250416C;
	sub_825040B0(ctx, base);
	// 8250416C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82504170: 419A0018  beq cr6, 0x82504188
	if ctx.cr[6].eq {
	pc = 0x82504188; continue 'dispatch;
	}
	// 82504174: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82504178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250417C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82504180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82504184: 4E800020  blr
	return;
	// 82504188: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250418C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82504190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82504194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82504198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825041A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825041A0 size=168
    let mut pc: u32 = 0x825041A0;
    'dispatch: loop {
        match pc {
            0x825041A0 => {
    //   block [0x825041A0..0x82504248)
	// 825041A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825041A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825041A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825041AC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825041B0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 825041B4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 825041B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825041BC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 825041C0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825041C4: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825041C8: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 825041CC: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825041D0: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825041D4: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825041D8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825041DC: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825041E0: C0060004  lfs f0, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825041E4: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 825041E8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 825041EC: C1AB0008  lfs f13, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825041F0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 825041F4: C0060008  lfs f0, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825041F8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825041FC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82504200: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82504248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82504248 size=268
    let mut pc: u32 = 0x82504248;
    'dispatch: loop {
        match pc {
            0x82504248 => {
    //   block [0x82504248..0x82504354)
	// 82504248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250424C: 48030E61  bl 0x825350ac
	ctx.lr = 0x82504250;
	sub_82535080(ctx, base);
	// 82504250: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82504254: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82504258: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8250425C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82504260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82504264: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82504268: C00B8CB4  lfs f0, -0x734c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250426C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82504270: D01B0000  stfs f0, 0(r27)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82504274: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82504358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82504358 size=244
    let mut pc: u32 = 0x82504358;
    'dispatch: loop {
        match pc {
            0x82504358 => {
    //   block [0x82504358..0x8250444C)
	// 82504358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250435C: 48030D59  bl 0x825350b4
	ctx.lr = 0x82504360;
	sub_82535080(ctx, base);
	// 82504360: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82504364: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82504368: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8250436C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82504370: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82504374: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82504378: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 8250437C: C00B8CB4  lfs f0, -0x734c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82504380: 3B9F0010  addi r28, r31, 0x10
	ctx.r[28].s64 = ctx.r[31].s64 + 16;
	// 82504384: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82504388: 397E0030  addi r11, r30, 0x30
	ctx.r[11].s64 = ctx.r[30].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82504450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82504450 size=408
    let mut pc: u32 = 0x82504450;
    'dispatch: loop {
        match pc {
            0x82504450 => {
    //   block [0x82504450..0x825045E8)
	// 82504450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82504454: 48030C65  bl 0x825350b8
	ctx.lr = 0x82504458;
	sub_82535080(ctx, base);
	// 82504458: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825045E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825045E8 size=388
    let mut pc: u32 = 0x825045E8;
    'dispatch: loop {
        match pc {
            0x825045E8 => {
    //   block [0x825045E8..0x8250476C)
	// 825045E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825045EC: 48030AC5  bl 0x825350b0
	ctx.lr = 0x825045F0;
	sub_82535080(ctx, base);
	// 825045F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825045F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825045F8: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825045FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82504600: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82504604: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82504608: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 8250460C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82504610: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82504614: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82504618: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 8250461C: 577C103A  slwi r28, r27, 2
	ctx.r[28].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82504620: B3BF0008  sth r29, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u16 ) };
	// 82504624: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82504628: B3BF0002  sth r29, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[29].u16 ) };
	// 8250462C: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 82504630: B15F000A  sth r10, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[10].u16 ) };
	// 82504634: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82504638: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 8250463C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82504640: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82504644: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82504648: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250464C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82504650: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82504654: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82504658: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250465C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82504660: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82504664: 4BF5F9D5  bl 0x82464038
	ctx.lr = 0x82504668;
	sub_82464038(ctx, base);
	// 82504668: 5789F0BE  srwi r9, r28, 2
	ctx.r[9].u32 = ctx.r[28].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8250466C: 937F0014  stw r27, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 82504670: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82504674: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82504678: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8250467C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82504680: 40990020  ble cr6, 0x825046a0
	if !ctx.cr[6].gt {
	pc = 0x825046A0; continue 'dispatch;
	}
	// 82504684: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82504688: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8250468C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82504690: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82504694: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82504698: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8250469C: 409AFFE8  bne cr6, 0x82504684
	if !ctx.cr[6].eq {
	pc = 0x82504684; continue 'dispatch;
	}
	// 825046A0: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 825046A4: 40990038  ble cr6, 0x825046dc
	if !ctx.cr[6].gt {
	pc = 0x825046DC; continue 'dispatch;
	}
	// 825046A8: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 825046AC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825046B0: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825046B4: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825046B8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825046BC: 419A0010  beq cr6, 0x825046cc
	if ctx.cr[6].eq {
	pc = 0x825046CC; continue 'dispatch;
	}
	// 825046C0: A12B0006  lhz r9, 6(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 825046C4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 825046C8: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 825046CC: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 825046D0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 825046D4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 825046D8: 409AFFD4  bne cr6, 0x825046ac
	if !ctx.cr[6].eq {
	pc = 0x825046AC; continue 'dispatch;
	}
	// 825046DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825046E0: 117F038C  vspltisw v11, -1
	for i in 0..4 {
		ctx.v[11].u32[i] = 4294967295;
	}
	// 825046E4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 825046E8: C00B2238  lfs f0, 0x2238(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8760 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825046EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 825046F0: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825046F4: 396B9FA0  addi r11, r11, -0x6060
	ctx.r[11].s64 = ctx.r[11].s64 + -24672;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82504770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82504770 size=420
    let mut pc: u32 = 0x82504770;
    'dispatch: loop {
        match pc {
            0x82504770 => {
    //   block [0x82504770..0x82504914)
	// 82504770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82504774: 48030939  bl 0x825350ac
	ctx.lr = 0x82504778;
	sub_82535080(ctx, base);
	// 82504778: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250477C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82504780: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82504784: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82504788: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8250478C: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82504790: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82504794: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82504798: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8250479C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 825047A0: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 825047A4: B3BF0008  sth r29, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u16 ) };
	// 825047A8: B3BF0002  sth r29, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[29].u16 ) };
	// 825047AC: B17F000A  sth r11, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 825047B0: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 825047B4: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 825047B8: 480A3F99  bl 0x825a8750
	ctx.lr = 0x825047BC;
	sub_825A8750(ctx, base);
	// 825047BC: 397A0030  addi r11, r26, 0x30
	ctx.r[11].s64 = ctx.r[26].s64 + 48;
	// 825047C0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825047C4: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 825047C8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 825047CC: 577C103A  slwi r28, r27, 2
	ctx.r[28].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 825047D0: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 825047D4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825047D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825047DC: 911E0000  stw r8, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825047E0: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825047E4: 911E0004  stw r8, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 825047E8: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825047EC: 911E0008  stw r8, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 825047F0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825047F4: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825047F8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825047FC: 4BF5F83D  bl 0x82464038
	ctx.lr = 0x82504800;
	sub_82464038(ctx, base);
	// 82504800: 5789F0BE  srwi r9, r28, 2
	ctx.r[9].u32 = ctx.r[28].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82504804: 937F0014  stw r27, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 82504808: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 8250480C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82504810: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82504814: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82504818: 40990020  ble cr6, 0x82504838
	if !ctx.cr[6].gt {
	pc = 0x82504838; continue 'dispatch;
	}
	// 8250481C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82504820: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82504824: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82504828: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8250482C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82504830: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82504834: 409AFFE8  bne cr6, 0x8250481c
	if !ctx.cr[6].eq {
	pc = 0x8250481C; continue 'dispatch;
	}
	// 82504838: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8250483C: 40990034  ble cr6, 0x82504870
	if !ctx.cr[6].gt {
	pc = 0x82504870; continue 'dispatch;
	}
	// 82504840: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82504844: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82504848: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250484C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82504850: 419A0010  beq cr6, 0x82504860
	if ctx.cr[6].eq {
	pc = 0x82504860; continue 'dispatch;
	}
	// 82504854: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82504858: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250485C: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82504860: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 82504864: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82504868: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8250486C: 409AFFD4  bne cr6, 0x82504840
	if !ctx.cr[6].eq {
	pc = 0x82504840; continue 'dispatch;
	}
	// 82504870: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82504918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82504918 size=160
    let mut pc: u32 = 0x82504918;
    'dispatch: loop {
        match pc {
            0x82504918 => {
    //   block [0x82504918..0x825049B8)
	// 82504918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250491C: 480307A1  bl 0x825350bc
	ctx.lr = 0x82504920;
	sub_82535080(ctx, base);
	// 82504920: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82504924: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82504928: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8250492C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82504930: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82504934: 4099005C  ble cr6, 0x82504990
	if !ctx.cr[6].gt {
	pc = 0x82504990; continue 'dispatch;
	}
	// 82504938: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8250493C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82504940: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82504944: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82504948: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250494C: 419A0030  beq cr6, 0x8250497c
	if ctx.cr[6].eq {
	pc = 0x8250497C; continue 'dispatch;
	}
	// 82504950: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82504954: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82504958: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8250495C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82504960: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82504964: 409A0018  bne cr6, 0x8250497c
	if !ctx.cr[6].eq {
	pc = 0x8250497C; continue 'dispatch;
	}
	// 82504968: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250496C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82504970: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82504974: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82504978: 4E800421  bctrl
	ctx.lr = 0x8250497C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250497C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82504980: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82504984: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82504988: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250498C: 4198FFB0  blt cr6, 0x8250493c
	if ctx.cr[6].lt {
	pc = 0x8250493C; continue 'dispatch;
	}
	// 82504990: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82504994: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82504998: 813E0014  lwz r9, 0x14(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250499C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 825049A0: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 825049A4: 5525103A  slwi r5, r9, 2
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825049A8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825049AC: 4BF5F70D  bl 0x824640b8
	ctx.lr = 0x825049B0;
	sub_824640B8(ctx, base);
	// 825049B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825049B4: 48030758  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825049B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825049B8 size=12
    let mut pc: u32 = 0x825049B8;
    'dispatch: loop {
        match pc {
            0x825049B8 => {
    //   block [0x825049B8..0x825049C4)
	// 825049B8: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 825049BC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 825049C0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825049C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825049C4 size=32
    let mut pc: u32 = 0x825049C4;
    'dispatch: loop {
        match pc {
            0x825049C4 => {
    //   block [0x825049C4..0x825049E4)
	// 825049C4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825049C8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825049CC: 80830050  lwz r4, 0x50(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 825049D0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825049D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825049D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825049DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825049E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825049E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825049E4 size=4
    let mut pc: u32 = 0x825049E4;
    'dispatch: loop {
        match pc {
            0x825049E4 => {
    //   block [0x825049E4..0x825049E8)
	// 825049E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825049E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825049E8 size=32
    let mut pc: u32 = 0x825049E8;
    'dispatch: loop {
        match pc {
            0x825049E8 => {
    //   block [0x825049E8..0x82504A08)
	// 825049E8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825049EC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825049F0: 80830058  lwz r4, 0x58(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 825049F4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825049F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825049FC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82504A00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82504A04: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82504A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82504A08 size=256
    let mut pc: u32 = 0x82504A08;
    'dispatch: loop {
        match pc {
            0x82504A08 => {
    //   block [0x82504A08..0x82504B08)
	// 82504A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82504A0C: 480306B1  bl 0x825350bc
	ctx.lr = 0x82504A10;
	sub_82535080(ctx, base);
	// 82504A10: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82504A14: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82504A18: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82504A1C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82504A20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82504A24: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82504A28: 48006899  bl 0x8250b2c0
	ctx.lr = 0x82504A2C;
	sub_8250B2C0(ctx, base);
	// 82504A2C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82504A30: 395F0070  addi r10, r31, 0x70
	ctx.r[10].s64 = ctx.r[31].s64 + 112;
	// 82504A34: 3C808000  lis r4, -0x8000
	ctx.r[4].s64 = -2147483648;
	// 82504A38: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82504A3C: 38CA0020  addi r6, r10, 0x20
	ctx.r[6].s64 = ctx.r[10].s64 + 32;
	// 82504A40: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82504A44: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82504A48: 3BA00006  li r29, 6
	ctx.r[29].s64 = 6;
	// 82504A4C: 392B46F0  addi r9, r11, 0x46f0
	ctx.r[9].s64 = ctx.r[11].s64 + 18160;
	// 82504A50: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82504A54: 38FF0040  addi r7, r31, 0x40
	ctx.r[7].s64 = ctx.r[31].s64 + 64;
	// 82504A58: 390B46CC  addi r8, r11, 0x46cc
	ctx.r[8].s64 = ctx.r[11].s64 + 18124;
	// 82504A5C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82504A60: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82504A64: 393F0020  addi r9, r31, 0x20
	ctx.r[9].s64 = ctx.r[31].s64 + 32;
	// 82504A68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82504A6C: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82504A70: 391F0030  addi r8, r31, 0x30
	ctx.r[8].s64 = ctx.r[31].s64 + 48;
	// 82504A74: C1AB0DA0  lfs f13, 0xda0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3488 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82504A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82504A7C: 909F0068  stw r4, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[4].u32 ) };
	// 82504A80: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82504A84: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82504A88: 996A0000  stb r11, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82504A8C: 98AA0001  stb r5, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[5].u8 ) };
	// 82504A90: B16A0008  sth r11, 8(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u16 ) };
	// 82504A94: B16A0002  sth r11, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82504A98: B0AA000A  sth r5, 0xa(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[5].u16 ) };
	// 82504A9C: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82504AA0: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82504AA4: 996A0039  stb r11, 0x39(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(57 as u32), ctx.r[11].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82504B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82504B08 size=868
    let mut pc: u32 = 0x82504B08;
    'dispatch: loop {
        match pc {
            0x82504B08 => {
    //   block [0x82504B08..0x82504E6C)
	// 82504B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82504B0C: 480305A1  bl 0x825350ac
	ctx.lr = 0x82504B10;
	sub_82535080(ctx, base);
	// 82504B10: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82504B14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82504B18: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82504B1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82504B20: 480067A1  bl 0x8250b2c0
	ctx.lr = 0x82504B24;
	sub_8250B2C0(ctx, base);
	// 82504B24: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82504B28: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82504B2C: 396B46F0  addi r11, r11, 0x46f0
	ctx.r[11].s64 = ctx.r[11].s64 + 18160;
	// 82504B30: 3BBF0070  addi r29, r31, 0x70
	ctx.r[29].s64 = ctx.r[31].s64 + 112;
	// 82504B34: 394A46CC  addi r10, r10, 0x46cc
	ctx.r[10].s64 = ctx.r[10].s64 + 18124;
	// 82504B38: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82504B3C: 3B7F0060  addi r27, r31, 0x60
	ctx.r[27].s64 = ctx.r[31].s64 + 96;
	// 82504B40: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82504B44: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82504B48: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82504B4C: 397D0020  addi r11, r29, 0x20
	ctx.r[11].s64 = ctx.r[29].s64 + 32;
	// 82504B50: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82504B54: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82504B58: 939B0000  stw r28, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82504B5C: 3B5E0040  addi r26, r30, 0x40
	ctx.r[26].s64 = ctx.r[30].s64 + 64;
	// 82504B60: 939B0004  stw r28, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82504B64: 913B0008  stw r9, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82504B68: 9B9D0000  stb r28, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 82504B6C: 9B3D0001  stb r25, 1(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(1 as u32), ctx.r[25].u8 ) };
	// 82504B70: B39D0008  sth r28, 8(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[28].u16 ) };
	// 82504B74: B39D0002  sth r28, 2(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(2 as u32), ctx.r[28].u16 ) };
	// 82504B78: B33D000A  sth r25, 0xa(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(10 as u32), ctx.r[25].u16 ) };
	// 82504B7C: 939D000C  stw r28, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82504B80: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82504B84: 9B9D0039  stb r28, 0x39(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(57 as u32), ctx.r[28].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82504E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82504E70 size=368
    let mut pc: u32 = 0x82504E70;
    'dispatch: loop {
        match pc {
            0x82504E70 => {
    //   block [0x82504E70..0x82504FE0)
	// 82504E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82504E74: 48030235  bl 0x825350a8
	ctx.lr = 0x82504E78;
	sub_82535080(ctx, base);
	// 82504E78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82504E7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82504E80: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82504E84: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82504E88: 396B46F0  addi r11, r11, 0x46f0
	ctx.r[11].s64 = ctx.r[11].s64 + 18160;
	// 82504E8C: 394A46CC  addi r10, r10, 0x46cc
	ctx.r[10].s64 = ctx.r[10].s64 + 18124;
	// 82504E90: 813E005C  lwz r9, 0x5c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82504E94: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82504E98: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82504E9C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82504EA0: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82504EA4: 409900AC  ble cr6, 0x82504f50
	if !ctx.cr[6].gt {
	pc = 0x82504F50; continue 'dispatch;
	}
	// 82504EA8: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82504EAC: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 82504EB0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82504EB4: 817E0058  lwz r11, 0x58(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 82504EB8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82504EBC: 7FEBDA14  add r31, r11, r27
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82504EC0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82504EC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82504EC8: 4099005C  ble cr6, 0x82504f24
	if !ctx.cr[6].gt {
	pc = 0x82504F24; continue 'dispatch;
	}
	// 82504ECC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82504ED0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82504ED4: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82504ED8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82504EDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82504EE0: 419A0030  beq cr6, 0x82504f10
	if ctx.cr[6].eq {
	pc = 0x82504F10; continue 'dispatch;
	}
	// 82504EE4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82504EE8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82504EEC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82504EF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82504EF4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82504EF8: 409A0018  bne cr6, 0x82504f10
	if !ctx.cr[6].eq {
	pc = 0x82504F10; continue 'dispatch;
	}
	// 82504EFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82504F00: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82504F04: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82504F08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82504F0C: 4E800421  bctrl
	ctx.lr = 0x82504F10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82504F10: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82504F14: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82504F18: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82504F1C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82504F20: 4198FFB0  blt cr6, 0x82504ed0
	if ctx.cr[6].lt {
	pc = 0x82504ED0; continue 'dispatch;
	}
	// 82504F24: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82504F28: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82504F2C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82504F30: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82504F34: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82504F38: 4BF5F181  bl 0x824640b8
	ctx.lr = 0x82504F3C;
	sub_824640B8(ctx, base);
	// 82504F3C: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82504F40: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82504F44: 3B7B0040  addi r27, r27, 0x40
	ctx.r[27].s64 = ctx.r[27].s64 + 64;
	// 82504F48: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82504F4C: 4198FF68  blt cr6, 0x82504eb4
	if ctx.cr[6].lt {
	pc = 0x82504EB4; continue 'dispatch;
	}
	// 82504F50: 817E0054  lwz r11, 0x54(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 82504F54: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82504F58: 40990024  ble cr6, 0x82504f7c
	if !ctx.cr[6].gt {
	pc = 0x82504F7C; continue 'dispatch;
	}
	// 82504F5C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82504F60: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82504F64: 809E0050  lwz r4, 0x50(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 82504F68: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82504F6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82504F70: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82504F74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82504F78: 4E800421  bctrl
	ctx.lr = 0x82504F7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82504F7C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82504F80: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 82504F84: 809E0058  lwz r4, 0x58(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 82504F88: 7C7FE82E  lwzx r3, r31, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82504F8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82504F90: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82504F94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82504F98: 4E800421  bctrl
	ctx.lr = 0x82504F9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82504F9C: 817E0068  lwz r11, 0x68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82504FA0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82504FA4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82504FA8: 409A0018  bne cr6, 0x82504fc0
	if !ctx.cr[6].eq {
	pc = 0x82504FC0; continue 'dispatch;
	}
	// 82504FAC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82504FB0: 809E0060  lwz r4, 0x60(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82504FB4: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82504FB8: 7C7FE82E  lwzx r3, r31, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82504FBC: 4BF5F0FD  bl 0x824640b8
	ctx.lr = 0x82504FC0;
	sub_824640B8(ctx, base);
	// 82504FC0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82504FC4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82504FC8: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 82504FCC: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 82504FD0: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82504FD4: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82504FD8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82504FDC: 4803011C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82504FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82504FE0 size=420
    let mut pc: u32 = 0x82504FE0;
    'dispatch: loop {
        match pc {
            0x82504FE0 => {
    //   block [0x82504FE0..0x82505184)
	// 82504FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82504FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82504FE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82504FEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82504FF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82504FF4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82504FF8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82504FFC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82505000: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82505004: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82505008: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 8250500C: 394A46F0  addi r10, r10, 0x46f0
	ctx.r[10].s64 = ctx.r[10].s64 + 18160;
	// 82505010: 392946CC  addi r9, r9, 0x46cc
	ctx.r[9].s64 = ctx.r[9].s64 + 18124;
	// 82505014: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82505018: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8250501C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82505020: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82505024: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82505028: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 8250502C: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82505030: 419A0130  beq cr6, 0x82505160
	if ctx.cr[6].eq {
	pc = 0x82505160; continue 'dispatch;
	}
	// 82505034: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82505038: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8250503C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82505040: 40990034  ble cr6, 0x82505074
	if !ctx.cr[6].gt {
	pc = 0x82505074; continue 'dispatch;
	}
	// 82505044: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82505048: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250504C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82505050: 88EB0001  lbz r7, 1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82505054: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82505058: 409A0008  bne cr6, 0x82505060
	if !ctx.cr[6].eq {
	pc = 0x82505060; continue 'dispatch;
	}
	// 8250505C: 990B0001  stb r8, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[8].u8 ) };
	// 82505060: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82505064: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82505068: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	// 8250506C: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82505070: 4198FFD8  blt cr6, 0x82505048
	if ctx.cr[6].lt {
	pc = 0x82505048; continue 'dispatch;
	}
	// 82505074: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82505078: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8250507C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82505080: 40990034  ble cr6, 0x825050b4
	if !ctx.cr[6].gt {
	pc = 0x825050B4; continue 'dispatch;
	}
	// 82505084: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82505088: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8250508C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82505090: 88EB0001  lbz r7, 1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82505094: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82505098: 409A0008  bne cr6, 0x825050a0
	if !ctx.cr[6].eq {
	pc = 0x825050A0; continue 'dispatch;
	}
	// 8250509C: 990B0001  stb r8, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[8].u8 ) };
	// 825050A0: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 825050A4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 825050A8: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	// 825050AC: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825050B0: 4198FFD8  blt cr6, 0x82505088
	if ctx.cr[6].lt {
	pc = 0x82505088; continue 'dispatch;
	}
	// 825050B4: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 825050B8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 825050BC: 409A0010  bne cr6, 0x825050cc
	if !ctx.cr[6].eq {
	pc = 0x825050CC; continue 'dispatch;
	}
	// 825050C0: 3BDF0070  addi r30, r31, 0x70
	ctx.r[30].s64 = ctx.r[31].s64 + 112;
	// 825050C4: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 825050C8: 4800003C  b 0x82505104
	pc = 0x82505104; continue 'dispatch;
	// 825050CC: 40990048  ble cr6, 0x82505114
	if !ctx.cr[6].gt {
	pc = 0x82505114; continue 'dispatch;
	}
	// 825050D0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825050D4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 825050D8: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825050DC: 38C00025  li r6, 0x25
	ctx.r[6].s64 = 37;
	// 825050E0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 825050E4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825050E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825050EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825050F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825050F4: 4E800421  bctrl
	ctx.lr = 0x825050F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825050F8: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 825050FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82505100: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82505104: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82505108: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250510C: 4BF6521D  bl 0x8246a328
	ctx.lr = 0x82505110;
	sub_8246A328(ctx, base);
	// 82505110: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82505114: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82505118: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250511C: 40990044  ble cr6, 0x82505160
	if !ctx.cr[6].gt {
	pc = 0x82505160; continue 'dispatch;
	}
	// 82505120: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505124: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82505128: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8250512C: 38C00025  li r6, 0x25
	ctx.r[6].s64 = 37;
	// 82505130: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82505134: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82505138: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250513C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505140: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82505144: 4E800421  bctrl
	ctx.lr = 0x82505148;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82505148: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8250514C: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82505150: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82505154: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82505158: 4BF651D1  bl 0x8246a328
	ctx.lr = 0x8250515C;
	sub_8246A328(ctx, base);
	// 8250515C: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82505160: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82505164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82505168: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8250516C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82505170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82505174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82505178: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250517C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82505180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82505188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82505188 size=364
    let mut pc: u32 = 0x82505188;
    'dispatch: loop {
        match pc {
            0x82505188 => {
    //   block [0x82505188..0x825052F4)
	// 82505188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250518C: 4802FF11  bl 0x8253509c
	ctx.lr = 0x82505190;
	sub_82535080(ctx, base);
	// 82505190: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82505194: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82505198: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 8250519C: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 825051A0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825051A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825051A8: 817D0054  lwz r11, 0x54(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 825051AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825051B0: 40990038  ble cr6, 0x825051e8
	if !ctx.cr[6].gt {
	pc = 0x825051E8; continue 'dispatch;
	}
	// 825051B4: 815D0050  lwz r10, 0x50(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 825051B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825051BC: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825051C0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 825051C4: 93EA003C  stw r31, 0x3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(60 as u32), ctx.r[31].u32 ) };
	// 825051C8: 811D0054  lwz r8, 0x54(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 825051CC: 815D0050  lwz r10, 0x50(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 825051D0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 825051D4: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825051D8: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 825051DC: 81080010  lwz r8, 0x10(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 825051E0: 7FE8FA14  add r31, r8, r31
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 825051E4: 4198FFD8  blt cr6, 0x825051bc
	if ctx.cr[6].lt {
	pc = 0x825051BC; continue 'dispatch;
	}
	// 825051E8: 3AFD0060  addi r23, r29, 0x60
	ctx.r[23].s64 = ctx.r[29].s64 + 96;
	// 825051EC: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 825051F0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825051F4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 825051F8: 40980014  bge cr6, 0x8250520c
	if !ctx.cr[6].lt {
	pc = 0x8250520C; continue 'dispatch;
	}
	// 825051FC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82505200: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82505204: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82505208: 4BF690C1  bl 0x8246e2c8
	ctx.lr = 0x8250520C;
	sub_8246E2C8(ctx, base);
	// 8250520C: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82505210: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82505214: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82505218: 40980024  bge cr6, 0x8250523c
	if !ctx.cr[6].lt {
	pc = 0x8250523C; continue 'dispatch;
	}
	// 8250521C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82505220: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82505224: 41980008  blt cr6, 0x8250522c
	if ctx.cr[6].lt {
	pc = 0x8250522C; continue 'dispatch;
	}
	// 82505228: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8250522C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82505230: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82505234: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82505238: 4BF69091  bl 0x8246e2c8
	ctx.lr = 0x8250523C;
	sub_8246E2C8(ctx, base);
	// 8250523C: 93F70004  stw r31, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82505240: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82505244: 817D0054  lwz r11, 0x54(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82505248: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8250524C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82505250: 40990090  ble cr6, 0x825052e0
	if !ctx.cr[6].gt {
	pc = 0x825052E0; continue 'dispatch;
	}
	// 82505254: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82505258: 817D0050  lwz r11, 0x50(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250525C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82505260: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82505264: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82505268: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250526C: 40990060  ble cr6, 0x825052cc
	if !ctx.cr[6].gt {
	pc = 0x825052CC; continue 'dispatch;
	}
	// 82505270: 3975FFFB  addi r11, r21, -5
	ctx.r[11].s64 = ctx.r[21].s64 + -5;
	// 82505274: 577E083C  slwi r30, r27, 1
	ctx.r[30].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82505278: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8250527C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82505280: 69790001  xori r25, r11, 1
	ctx.r[25].u64 = ctx.r[11].u64 ^ 1;
	// 82505284: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82505288: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8250528C: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505290: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82505294: 214A0020  subfic r10, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[10].s64 = (32 as i64) - ctx.r[10].s64;
	// 82505298: 7F8BF214  add r28, r11, r30
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8250529C: 7F0B5030  slw r11, r24, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[24].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 825052A0: 7D63FB78  or r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 | ctx.r[31].u64;
	// 825052A4: 48024B4D  bl 0x82529df0
	ctx.lr = 0x825052A8;
	sub_82529DF0(ctx, base);
	// 825052A8: B07C0000  sth r3, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 825052AC: 817D0050  lwz r11, 0x50(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 825052B0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 825052B4: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 825052B8: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 825052BC: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 825052C0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825052C4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825052C8: 4198FFBC  blt cr6, 0x82505284
	if ctx.cr[6].lt {
	pc = 0x82505284; continue 'dispatch;
	}
	// 825052CC: 817D0054  lwz r11, 0x54(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 825052D0: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 825052D4: 3B5A0040  addi r26, r26, 0x40
	ctx.r[26].s64 = ctx.r[26].s64 + 64;
	// 825052D8: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825052DC: 4198FF7C  blt cr6, 0x82505258
	if ctx.cr[6].lt {
	pc = 0x82505258; continue 'dispatch;
	}
	// 825052E0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825052E4: 997D0014  stb r11, 0x14(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 825052E8: 9ABD006C  stb r21, 0x6c(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(108 as u32), ctx.r[21].u8 ) };
	// 825052EC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825052F0: 4802FDFC  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825052F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825052F8 size=200
    let mut pc: u32 = 0x825052F8;
    'dispatch: loop {
        match pc {
            0x825052F8 => {
    //   block [0x825052F8..0x825053C0)
	// 825052F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825052FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82505300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82505304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82505308: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250530C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82505310: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82505314: 815F004C  lwz r10, 0x4c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82505318: 7D4A5A15  add. r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8250531C: 4082000C  bne 0x82505328
	if !ctx.cr[0].eq {
	pc = 0x82505328; continue 'dispatch;
	}
	// 82505320: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82505324: 48000084  b 0x825053a8
	pc = 0x825053A8; continue 'dispatch;
	// 82505328: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250532C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82505330: 409A0008  bne cr6, 0x82505338
	if !ctx.cr[6].eq {
	pc = 0x82505338; continue 'dispatch;
	}
	// 82505334: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 82505338: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250533C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82505340: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82505344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82505348: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250534C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82505350: 4E800421  bctrl
	ctx.lr = 0x82505354;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82505354: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82505358: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8250535C: 419A000C  beq cr6, 0x82505368
	if ctx.cr[6].eq {
	pc = 0x82505368; continue 'dispatch;
	}
	// 82505360: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82505364: 48000044  b 0x825053a8
	pc = 0x825053A8; continue 'dispatch;
	// 82505368: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8250536C: 38C30040  addi r6, r3, 0x40
	ctx.r[6].s64 = ctx.r[3].s64 + 64;
	// 82505370: 38A30030  addi r5, r3, 0x30
	ctx.r[5].s64 = ctx.r[3].s64 + 48;
	// 82505374: 38830020  addi r4, r3, 0x20
	ctx.r[4].s64 = ctx.r[3].s64 + 32;
	// 82505378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250537C: C02BE098  lfs f1, -0x1f68(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8040 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82505380: 48024DC1  bl 0x8252a140
	ctx.lr = 0x82505384;
	sub_8252A140(ctx, base);
	// 82505384: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505388: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250538C: 419AFFD4  beq cr6, 0x82505360
	if ctx.cr[6].eq {
	pc = 0x82505360; continue 'dispatch;
	}
	// 82505390: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505394: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82505398: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250539C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825053A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825053A4: 4E800421  bctrl
	ctx.lr = 0x825053A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825053A8: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 825053AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825053B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825053B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825053B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825053BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825053C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825053C0 size=296
    let mut pc: u32 = 0x825053C0;
    'dispatch: loop {
        match pc {
            0x825053C0 => {
    //   block [0x825053C0..0x825054E8)
	// 825053C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825053C4: 4802FCE9  bl 0x825350ac
	ctx.lr = 0x825053C8;
	sub_82535080(ctx, base);
	// 825053C8: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825053CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825053D0: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 825053D4: 548A007E  clrlwi r10, r4, 1
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x7FFFFFFFu64;
	// 825053D8: 549B0000  rlwinm r27, r4, 0, 0, 0
	ctx.r[27].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 825053DC: 3F408282  lis r26, -0x7d7e
	ctx.r[26].s64 = -2105409536;
	// 825053E0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825053E4: 212B0020  subfic r9, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[9].s64 = (32 as i64) - ctx.r[11].s64;
	// 825053E8: 7F2B5C30  srw r11, r25, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[25].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 825053EC: 7D7F2038  and r31, r11, r4
	ctx.r[31].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 825053F0: 7D5E4C30  srw r30, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 825053F4: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 825053F8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 825053FC: 57CB3032  slwi r11, r30, 6
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82505400: 409A0048  bne cr6, 0x82505448
	if !ctx.cr[6].eq {
	pc = 0x82505448; continue 'dispatch;
	}
	// 82505404: 815D0040  lwz r10, 0x40(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82505408: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250540C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82505410: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82505414: 4198005C  blt cr6, 0x82505470
	if ctx.cr[6].lt {
	pc = 0x82505470; continue 'dispatch;
	}
	// 82505418: 817D0044  lwz r11, 0x44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250541C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82505420: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82505424: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82505428: 41980048  blt cr6, 0x82505470
	if ctx.cr[6].lt {
	pc = 0x82505470; continue 'dispatch;
	}
	// 8250542C: 817D004C  lwz r11, 0x4c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 82505430: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82505434: 419A00A8  beq cr6, 0x825054dc
	if ctx.cr[6].eq {
	pc = 0x825054DC; continue 'dispatch;
	}
	// 82505438: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 8250543C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82505440: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82505444: 4BFFFFB0  b 0x825053f4
	pc = 0x825053F4; continue 'dispatch;
	// 82505448: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250544C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82505450: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82505454: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82505458: 41980018  blt cr6, 0x82505470
	if ctx.cr[6].lt {
	pc = 0x82505470; continue 'dispatch;
	}
	// 8250545C: 817D004C  lwz r11, 0x4c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 82505460: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82505464: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82505468: 40980074  bge cr6, 0x825054dc
	if !ctx.cr[6].lt {
	pc = 0x825054DC; continue 'dispatch;
	}
	// 8250546C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82505470: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82505474: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82505478: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250547C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82505480: 216B0020  subfic r11, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[11].s64 = (32 as i64) - ctx.r[11].s64;
	// 82505484: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82505488: 7FCB5830  slw r11, r30, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[30].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8250548C: 7D6BDB78  or r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[27].u64;
	// 82505490: 7D7CFB78  or r28, r11, r31
	ctx.r[28].u64 = ctx.r[11].u64 | ctx.r[31].u64;
	// 82505494: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82505498: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8250549C: 4E800421  bctrl
	ctx.lr = 0x825054A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825054A0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825054A4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 825054A8: 409A0028  bne cr6, 0x825054d0
	if !ctx.cr[6].eq {
	pc = 0x825054D0; continue 'dispatch;
	}
	// 825054AC: 38C30040  addi r6, r3, 0x40
	ctx.r[6].s64 = ctx.r[3].s64 + 64;
	// 825054B0: C03AE098  lfs f1, -0x1f68(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-8040 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825054B4: 38A30030  addi r5, r3, 0x30
	ctx.r[5].s64 = ctx.r[3].s64 + 48;
	// 825054B8: 38830020  addi r4, r3, 0x20
	ctx.r[4].s64 = ctx.r[3].s64 + 32;
	// 825054BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825054C0: 48024C81  bl 0x8252a140
	ctx.lr = 0x825054C4;
	sub_8252A140(ctx, base);
	// 825054C4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825054C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825054CC: 409AFF28  bne cr6, 0x825053f4
	if !ctx.cr[6].eq {
	pc = 0x825053F4; continue 'dispatch;
	}
	// 825054D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825054D4: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 825054D8: 4802FC24  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 825054DC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 825054E0: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 825054E4: 4802FC18  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825054E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825054E8 size=960
    let mut pc: u32 = 0x825054E8;
    'dispatch: loop {
        match pc {
            0x825054E8 => {
    //   block [0x825054E8..0x825058A8)
	// 825054E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825054EC: 4802FBC5  bl 0x825350b0
	ctx.lr = 0x825054F0;
	sub_82535080(ctx, base);
	// 825054F0: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825054F4: 38C3FFF0  addi r6, r3, -0x10
	ctx.r[6].s64 = ctx.r[3].s64 + -16;
	// 825054F8: 548B0000  rlwinm r11, r4, 0, 0, 0
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 825054FC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82505500: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82505504: 5489007E  clrlwi r9, r4, 1
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x7FFFFFFFu64;
	// 82505508: 81660018  lwz r11, 0x18(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250550C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82505510: 210B0020  subfic r8, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[11].s64;
	// 82505514: 7D4B5C30  srw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82505518: 7D7B2038  and r27, r11, r4
	ctx.r[27].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 8250551C: 7D2B4430  srw r11, r9, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82505520: 409A0230  bne cr6, 0x82505750
	if !ctx.cr[6].eq {
	pc = 0x82505750; continue 'dispatch;
	}
	// 82505524: 81460054  lwz r10, 0x54(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(84 as u32) ) } as u64;
	// 82505528: 3921009F  addi r9, r1, 0x9f
	ctx.r[9].s64 = ctx.r[1].s64 + 159;
	// 8250552C: 3B800004  li r28, 4
	ctx.r[28].s64 = 4;
	// 82505530: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82505534: 55270032  rlwinm r7, r9, 0, 0, 0x19
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82505538: 409A000C  bne cr6, 0x82505544
	if !ctx.cr[6].eq {
	pc = 0x82505544; continue 'dispatch;
	}
	// 8250553C: 38E60070  addi r7, r6, 0x70
	ctx.r[7].s64 = ctx.r[6].s64 + 112;
	// 82505540: 4800005C  b 0x8250559c
	pc = 0x8250559C; continue 'dispatch;
	// 82505544: 81460050  lwz r10, 0x50(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) } as u64;
	// 82505548: 55683032  slwi r8, r11, 6
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8250554C: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82505550: 7D685214  add r11, r8, r10
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82505554: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82505558: 7D055850  subf r8, r5, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 8250555C: 7D4A4050  subf r10, r10, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82505560: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82505564: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82505568: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250556C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82505570: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82505574: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82505578: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8250557C: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82505580: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82505584: 910AFFF8  stw r8, -8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), ctx.r[8].u32 ) };
	// 82505588: 90AAFFFC  stw r5, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[5].u32 ) };
	// 8250558C: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82505590: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82505594: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82505598: 4199FFD0  bgt cr6, 0x82505568
	if ctx.cr[6].gt {
	pc = 0x82505568; continue 'dispatch;
	}
	// 8250559C: 89670039  lbz r11, 0x39(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(57 as u32) ) } as u64;
	// 825055A0: 81470034  lwz r10, 0x34(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(52 as u32) ) } as u64;
	// 825055A4: 7D680774  extsb r8, r11
	ctx.r[8].s64 = ctx.r[11].s8 as i64;
	// 825055A8: 81270030  lwz r9, 0x30(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) } as u64;
	// 825055AC: 7D6AD9D6  mullw r11, r10, r27
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[27].s32 as i64);
	// 825055B0: 88A70038  lbz r5, 0x38(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(56 as u32) ) } as u64;
	// 825055B4: 7D0AD838  and r10, r8, r27
	ctx.r[10].u64 = ctx.r[8].u64 & ctx.r[27].u64;
	// 825055B8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825055BC: 69490001  xori r9, r10, 1
	ctx.r[9].u64 = ctx.r[10].u64 ^ 1;
	// 825055C0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825055C4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 825055C8: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 825055CC: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 825055D0: 7D290734  extsh r9, r9
	ctx.r[9].s64 = ctx.r[9].s16 as i64;
	// 825055D4: 409A001C  bne cr6, 0x825055f0
	if !ctx.cr[6].eq {
	pc = 0x825055F0; continue 'dispatch;
	}
	// 825055D8: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825055DC: A08B0000  lhz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825055E0: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825055E4: 7FCA5A2E  lhzx r30, r10, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825055E8: 7FA95A2E  lhzx r29, r9, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825055EC: 48000018  b 0x82505604
	pc = 0x82505604; continue 'dispatch;
	// 825055F0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825055F4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825055F8: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825055FC: 7FCA582E  lwzx r30, r10, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82505600: 7FA9582E  lwzx r29, r9, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82505604: 81070014  lwz r8, 0x14(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 82505608: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250560C: 81270018  lwz r9, 0x18(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 82505610: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82505614: 419A0054  beq cr6, 0x82505668
	if ctx.cr[6].eq {
	pc = 0x82505668; continue 'dispatch;
	}
	// 82505618: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8250561C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82505620: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82505624: 394A3134  addi r10, r10, 0x3134
	ctx.r[10].s64 = ctx.r[10].s64 + 12596;
	// 82505628: 3B400006  li r26, 6
	ctx.r[26].s64 = 6;
	// 8250562C: C00BD284  lfs f0, -0x2d7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11644 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82505630: 397F0050  addi r11, r31, 0x50
	ctx.r[11].s64 = ctx.r[31].s64 + 80;
	// 82505634: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82505638: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 8250563C: B07F0006  sth r3, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[3].u16 ) };
	// 82505640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82505644: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82505648: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250564C: B0BF0014  sth r5, 0x14(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[5].u16 ) };
	// 82505650: 9B5F0016  stb r26, 0x16(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(22 as u32), ctx.r[26].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825058A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825058A8 size=24
    let mut pc: u32 = 0x825058A8;
    'dispatch: loop {
        match pc {
            0x825058A8 => {
    //   block [0x825058A8..0x825058C0)
	// 825058A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825058AC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825058B0: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 825058B4: 38AB0040  addi r5, r11, 0x40
	ctx.r[5].s64 = ctx.r[11].s64 + 64;
	// 825058B8: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 825058BC: 4BFF7094  b 0x824fc950
	sub_824FC950(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825058C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825058C0 size=188
    let mut pc: u32 = 0x825058C0;
    'dispatch: loop {
        match pc {
            0x825058C0 => {
    //   block [0x825058C0..0x8250597C)
	// 825058C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825058C4: 4802F7F5  bl 0x825350b8
	ctx.lr = 0x825058C8;
	sub_82535080(ctx, base);
	// 825058C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825058CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825058D0: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 825058D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825058D8: 409A001C  bne cr6, 0x825058f4
	if !ctx.cr[6].eq {
	pc = 0x825058F4; continue 'dispatch;
	}
	// 825058DC: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 825058E0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825058E4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 825058E8: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825058EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825058F0: 4802F818  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 825058F4: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825058F8: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 825058FC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82505900: 38C00025  li r6, 0x25
	ctx.r[6].s64 = 37;
	// 82505904: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82505908: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8250590C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82505910: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505914: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505918: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250591C: 4E800421  bctrl
	ctx.lr = 0x82505920;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82505920: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82505924: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82505928: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250592C: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82505930: 4BF649F9  bl 0x8246a328
	ctx.lr = 0x82505934;
	sub_8246A328(ctx, base);
	// 82505934: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82505938: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8250593C: 4099001C  ble cr6, 0x82505958
	if !ctx.cr[6].gt {
	pc = 0x82505958; continue 'dispatch;
	}
	// 82505940: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82505944: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82505948: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250594C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82505950: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82505954: 4E800421  bctrl
	ctx.lr = 0x82505958;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82505958: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8250595C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82505960: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82505964: 556A3032  slwi r10, r11, 6
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82505968: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 8250596C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82505970: 386AFFC0  addi r3, r10, -0x40
	ctx.r[3].s64 = ctx.r[10].s64 + -64;
	// 82505974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82505978: 4802F790  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82505980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82505980 size=144
    let mut pc: u32 = 0x82505980;
    'dispatch: loop {
        match pc {
            0x82505980 => {
    //   block [0x82505980..0x82505A10)
	// 82505980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82505984: 4802F735  bl 0x825350b8
	ctx.lr = 0x82505988;
	sub_82535080(ctx, base);
	// 82505988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250598C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82505990: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505994: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 82505998: 38C00025  li r6, 0x25
	ctx.r[6].s64 = 37;
	// 8250599C: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 825059A0: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 825059A4: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825059A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825059AC: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825059B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825059B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825059B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825059BC: 4E800421  bctrl
	ctx.lr = 0x825059C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825059C0: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 825059C4: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 825059C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825059CC: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825059D0: 4BF64959  bl 0x8246a328
	ctx.lr = 0x825059D4;
	sub_8246A328(ctx, base);
	// 825059D4: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825059D8: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 825059DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825059E0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825059E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825059E8: 4E800421  bctrl
	ctx.lr = 0x825059EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825059EC: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 825059F0: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 825059F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825059F8: 556A3032  slwi r10, r11, 6
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825059FC: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82505A00: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82505A04: 386AFFC0  addi r3, r10, -0x40
	ctx.r[3].s64 = ctx.r[10].s64 + -64;
	// 82505A08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82505A0C: 4802F6FC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82505A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82505A10 size=344
    let mut pc: u32 = 0x82505A10;
    'dispatch: loop {
        match pc {
            0x82505A10 => {
    //   block [0x82505A10..0x82505B68)
	// 82505A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82505A14: 4802F6A5  bl 0x825350b8
	ctx.lr = 0x82505A18;
	sub_82535080(ctx, base);
	// 82505A18: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82505B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82505B68 size=444
    let mut pc: u32 = 0x82505B68;
    'dispatch: loop {
        match pc {
            0x82505B68 => {
    //   block [0x82505B68..0x82505D24)
	// 82505B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82505B6C: 4802F551  bl 0x825350bc
	ctx.lr = 0x82505B70;
	sub_82535080(ctx, base);
	// 82505B70: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82505D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82505D28 size=8
    let mut pc: u32 = 0x82505D28;
    'dispatch: loop {
        match pc {
            0x82505D28 => {
    //   block [0x82505D28..0x82505D30)
	// 82505D28: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82505D2C: 48000004  b 0x82505d30
	sub_82505D30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82505D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82505D30 size=100
    let mut pc: u32 = 0x82505D30;
    'dispatch: loop {
        match pc {
            0x82505D30 => {
    //   block [0x82505D30..0x82505D94)
	// 82505D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82505D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82505D38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82505D3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82505D40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82505D44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82505D48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82505D4C: 4BFFF125  bl 0x82504e70
	ctx.lr = 0x82505D50;
	sub_82504E70(ctx, base);
	// 82505D50: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82505D54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82505D58: 419A0020  beq cr6, 0x82505d78
	if ctx.cr[6].eq {
	pc = 0x82505D78; continue 'dispatch;
	}
	// 82505D5C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505D60: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82505D64: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82505D68: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82505D6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82505D70: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82505D74: 4BF5E345  bl 0x824640b8
	ctx.lr = 0x82505D78;
	sub_824640B8(ctx, base);
	// 82505D78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82505D7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82505D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82505D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82505D88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82505D8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82505D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82505D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82505D98 size=180
    let mut pc: u32 = 0x82505D98;
    'dispatch: loop {
        match pc {
            0x82505D98 => {
    //   block [0x82505D98..0x82505E4C)
	// 82505D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82505D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82505DA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82505DA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82505DA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82505DAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82505DB0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82505DB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82505DB8: 388B4774  addi r4, r11, 0x4774
	ctx.r[4].s64 = ctx.r[11].s64 + 18292;
	// 82505DBC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82505DC0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505DC4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82505DC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82505DCC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82505DD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82505DD4: 4E800421  bctrl
	ctx.lr = 0x82505DD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82505DD8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505DDC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82505DE0: 80DE0034  lwz r6, 0x34(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82505DE4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82505DE8: 388B476C  addi r4, r11, 0x476c
	ctx.r[4].s64 = ctx.r[11].s64 + 18284;
	// 82505DEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82505DF0: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82505DF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82505DF8: 4E800421  bctrl
	ctx.lr = 0x82505DFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82505DFC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505E00: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82505E04: 80DE0010  lwz r6, 0x10(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82505E08: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82505E0C: 388B4764  addi r4, r11, 0x4764
	ctx.r[4].s64 = ctx.r[11].s64 + 18276;
	// 82505E10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82505E14: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82505E18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82505E1C: 4E800421  bctrl
	ctx.lr = 0x82505E20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82505E20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505E24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82505E28: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82505E2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82505E30: 4E800421  bctrl
	ctx.lr = 0x82505E34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82505E34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82505E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82505E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82505E40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82505E44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82505E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82505E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82505E50 size=56
    let mut pc: u32 = 0x82505E50;
    'dispatch: loop {
        match pc {
            0x82505E50 => {
    //   block [0x82505E50..0x82505E88)
	// 82505E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82505E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82505E58: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82505E5C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82505E60: 80830010  lwz r4, 0x10(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82505E64: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82505E68: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82505E6C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82505E70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82505E74: 480B1ABD  bl 0x825b7930
	ctx.lr = 0x82505E78;
	sub_825B7930(ctx, base);
	// 82505E78: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82505E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82505E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82505E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82505E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82505E88 size=132
    let mut pc: u32 = 0x82505E88;
    'dispatch: loop {
        match pc {
            0x82505E88 => {
    //   block [0x82505E88..0x82505F0C)
	// 82505E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82505E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82505E90: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82505E94: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82505E98: 80830010  lwz r4, 0x10(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82505E9C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82505EA0: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82505EA4: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82505EA8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82505F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82505F10 size=68
    let mut pc: u32 = 0x82505F10;
    'dispatch: loop {
        match pc {
            0x82505F10 => {
    //   block [0x82505F10..0x82505F54)
	// 82505F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82505F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82505F18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82505F1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82505F20: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82505F24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82505F28: 396B2D50  addi r11, r11, 0x2d50
	ctx.r[11].s64 = ctx.r[11].s64 + 11600;
	// 82505F2C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82505F30: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82505F34: 4BFF37BD  bl 0x824f96f0
	ctx.lr = 0x82505F38;
	sub_824F96F0(ctx, base);
	// 82505F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82505F3C: 4BFF3B75  bl 0x824f9ab0
	ctx.lr = 0x82505F40;
	sub_824F9AB0(ctx, base);
	// 82505F40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82505F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82505F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82505F4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82505F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82505F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82505F58 size=20
    let mut pc: u32 = 0x82505F58;
    'dispatch: loop {
        match pc {
            0x82505F58 => {
    //   block [0x82505F58..0x82505F6C)
	// 82505F58: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82505F5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505F60: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82505F64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82505F68: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82505F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82505F70 size=160
    let mut pc: u32 = 0x82505F70;
    'dispatch: loop {
        match pc {
            0x82505F70 => {
    //   block [0x82505F70..0x82506010)
	// 82505F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82505F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82505F78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82505F7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82505F80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82505F84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82505F88: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82505F8C: 896B002C  lbz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82505F90: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82505F94: 409A000C  bne cr6, 0x82505fa0
	if !ctx.cr[6].eq {
	pc = 0x82505FA0; continue 'dispatch;
	}
	// 82505F98: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82505F9C: 4800005C  b 0x82505ff8
	pc = 0x82505FF8; continue 'dispatch;
	// 82505FA0: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82505FA4: 3BC5FFC0  addi r30, r5, -0x40
	ctx.r[30].s64 = ctx.r[5].s64 + -64;
	// 82505FA8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82505FAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82505FB0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82505FB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82505FB8: 4E800421  bctrl
	ctx.lr = 0x82505FBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82505FBC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82505FC0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82505FC4: 4198FFD4  blt cr6, 0x82505f98
	if ctx.cr[6].lt {
	pc = 0x82505F98; continue 'dispatch;
	}
	// 82505FC8: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82505FCC: 4199FFCC  bgt cr6, 0x82505f98
	if ctx.cr[6].gt {
	pc = 0x82505F98; continue 'dispatch;
	}
	// 82505FD0: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82505FD4: 393F0040  addi r9, r31, 0x40
	ctx.r[9].s64 = ctx.r[31].s64 + 64;
	// 82505FD8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82505FDC: 409A0014  bne cr6, 0x82505ff0
	if !ctx.cr[6].eq {
	pc = 0x82505FF0; continue 'dispatch;
	}
	// 82505FE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82505FE4: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 82505FE8: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82505FEC: 4800000C  b 0x82505ff8
	pc = 0x82505FF8; continue 'dispatch;
	// 82505FF0: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82505FF4: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82505FF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82505FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82506000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82506004: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82506008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250600C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82506010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82506010 size=96
    let mut pc: u32 = 0x82506010;
    'dispatch: loop {
        match pc {
            0x82506010 => {
    //   block [0x82506010..0x82506070)
	// 82506010: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82506014: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82506018: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8250601C: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82506020: 396B2D1C  addi r11, r11, 0x2d1c
	ctx.r[11].s64 = ctx.r[11].s64 + 11548;
	// 82506024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82506028: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8250602C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82506030: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82506034: A1650004  lhz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506038: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250603C: 419A0010  beq cr6, 0x8250604c
	if ctx.cr[6].eq {
	pc = 0x8250604C; continue 'dispatch;
	}
	// 82506040: A1650006  lhz r11, 6(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(6 as u32) ) } as u64;
	// 82506044: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82506048: B1650006  sth r11, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8250604C: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82506050: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82506070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82506070 size=136
    let mut pc: u32 = 0x82506070;
    'dispatch: loop {
        match pc {
            0x82506070 => {
    //   block [0x82506070..0x825060F8)
	// 82506070: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82506074: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82506078: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8250607C: 396B2D1C  addi r11, r11, 0x2d1c
	ctx.r[11].s64 = ctx.r[11].s64 + 11548;
	// 82506080: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82506084: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82506088: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8250608C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82506090: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82506094: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82506098: A1650004  lhz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250609C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825060A0: 419A0010  beq cr6, 0x825060b0
	if ctx.cr[6].eq {
	pc = 0x825060B0; continue 'dispatch;
	}
	// 825060A4: A1650006  lhz r11, 6(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(6 as u32) ) } as u64;
	// 825060A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825060AC: B1650006  sth r11, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 825060B0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 825060B4: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 825060B8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825060BC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 825060C0: 396B2D50  addi r11, r11, 0x2d50
	ctx.r[11].s64 = ctx.r[11].s64 + 11600;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825060F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825060F8 size=16
    let mut pc: u32 = 0x825060F8;
    'dispatch: loop {
        match pc {
            0x825060F8 => {
    //   block [0x825060F8..0x82506108)
	// 825060F8: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 825060FC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82506100: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82506104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82506108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82506108 size=284
    let mut pc: u32 = 0x82506108;
    'dispatch: loop {
        match pc {
            0x82506108 => {
    //   block [0x82506108..0x82506224)
	// 82506108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250610C: 4802EFA9  bl 0x825350b4
	ctx.lr = 0x82506110;
	sub_82535080(ctx, base);
	// 82506110: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82506114: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506118: 3B800014  li r28, 0x14
	ctx.r[28].s64 = 20;
	// 8250611C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82506120: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 82506124: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82506128: 7D7CD82E  lwzx r11, r28, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 8250612C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506130: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82506134: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82506138: 40980020  bge cr6, 0x82506158
	if !ctx.cr[6].lt {
	pc = 0x82506158; continue 'dispatch;
	}
	// 8250613C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82506140: 39294780  addi r9, r9, 0x4780
	ctx.r[9].s64 = ctx.r[9].s64 + 18304;
	// 82506144: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82506148: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8250614C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82506150: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82506154: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82506158: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8250615C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82506160: 394100CF  addi r10, r1, 0xcf
	ctx.r[10].s64 = ctx.r[1].s64 + 207;
	// 82506164: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82506168: 55460036  rlwinm r6, r10, 0, 0, 0x1b
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8250616C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82506170: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82506174: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82506228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82506228 size=228
    let mut pc: u32 = 0x82506228;
    'dispatch: loop {
        match pc {
            0x82506228 => {
    //   block [0x82506228..0x8250630C)
	// 82506228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250622C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82506230: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82506234: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82506238: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250623C: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506240: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 82506244: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 82506248: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8250624C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506250: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82506254: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82506258: 40980020  bge cr6, 0x82506278
	if !ctx.cr[6].lt {
	pc = 0x82506278; continue 'dispatch;
	}
	// 8250625C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82506260: 39294780  addi r9, r9, 0x4780
	ctx.r[9].s64 = ctx.r[9].s64 + 18304;
	// 82506264: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82506268: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8250626C: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 82506270: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82506274: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82506278: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250627C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82506280: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82506284: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82506288: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 8250628C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82506290: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82506294: 914100A0  stw r10, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 82506298: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 8250629C: 914100A4  stw r10, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 825062A0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 825062A4: 409A0008  bne cr6, 0x825062ac
	if !ctx.cr[6].eq {
	pc = 0x825062AC; continue 'dispatch;
	}
	// 825062A8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 825062AC: 80A30010  lwz r5, 0x10(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825062B0: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 825062B4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 825062B8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825062BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825062C0: 480B2C71  bl 0x825b8f30
	ctx.lr = 0x825062C4;
	sub_825B8F30(ctx, base);
	// 825062C4: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 825062C8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825062CC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825062D0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825062D4: 40980020  bge cr6, 0x825062f4
	if !ctx.cr[6].lt {
	pc = 0x825062F4; continue 'dispatch;
	}
	// 825062D8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 825062DC: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 825062E0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825062E4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825062E8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 825062EC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825062F0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 825062F4: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 825062F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825062FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82506300: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82506304: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82506308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82506310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82506310 size=260
    let mut pc: u32 = 0x82506310;
    'dispatch: loop {
        match pc {
            0x82506310 => {
    //   block [0x82506310..0x82506414)
	// 82506310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82506314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82506318: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250631C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82506320: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82506324: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82506418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82506418 size=28
    let mut pc: u32 = 0x82506418;
    'dispatch: loop {
        match pc {
            0x82506418 => {
    //   block [0x82506418..0x82506434)
	// 82506418: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8250641C: 419A0020  beq cr6, 0x8250643c
	if ctx.cr[6].eq {
		sub_8250643C(ctx, base);
		return;
	}
	// 82506420: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82506424: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82506428: 419A000C  beq cr6, 0x82506434
	if ctx.cr[6].eq {
		sub_82506434(ctx, base);
		return;
	}
	// 8250642C: 7C645A14  add r3, r4, r11
	ctx.r[3].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82506430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82506434(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82506434 size=8
    let mut pc: u32 = 0x82506434;
    'dispatch: loop {
        match pc {
            0x82506434 => {
    //   block [0x82506434..0x8250643C)
	// 82506434: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82506438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250643C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250643C size=8
    let mut pc: u32 = 0x8250643C;
    'dispatch: loop {
        match pc {
            0x8250643C => {
    //   block [0x8250643C..0x82506444)
	// 8250643C: 38640002  addi r3, r4, 2
	ctx.r[3].s64 = ctx.r[4].s64 + 2;
	// 82506440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82506448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82506448 size=92
    let mut pc: u32 = 0x82506448;
    'dispatch: loop {
        match pc {
            0x82506448 => {
    //   block [0x82506448..0x825064A4)
	// 82506448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250644C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82506450: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82506454: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82506458: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250645C: 4800899D  bl 0x8250edf8
	ctx.lr = 0x82506460;
	sub_8250EDF8(ctx, base);
	// 82506460: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82506464: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82506468: 396B47B4  addi r11, r11, 0x47b4
	ctx.r[11].s64 = ctx.r[11].s64 + 18356;
	// 8250646C: 394A4790  addi r10, r10, 0x4790
	ctx.r[10].s64 = ctx.r[10].s64 + 18320;
	// 82506470: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82506474: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82506478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250647C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82506480: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82506484: 913F0060  stw r9, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 82506488: 913F0064  stw r9, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 8250648C: 911F0068  stw r8, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[8].u32 ) };
	// 82506490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82506494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82506498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250649C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825064A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825064A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825064A8 size=204
    let mut pc: u32 = 0x825064A8;
    'dispatch: loop {
        match pc {
            0x825064A8 => {
    //   block [0x825064A8..0x82506574)
	// 825064A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825064AC: 4802EC11  bl 0x825350bc
	ctx.lr = 0x825064B0;
	sub_82535080(ctx, base);
	// 825064B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825064B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825064B8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825064BC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 825064C0: 396B47B4  addi r11, r11, 0x47b4
	ctx.r[11].s64 = ctx.r[11].s64 + 18356;
	// 825064C4: 394A4790  addi r10, r10, 0x4790
	ctx.r[10].s64 = ctx.r[10].s64 + 18320;
	// 825064C8: 813F0064  lwz r9, 0x64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 825064CC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825064D0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825064D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825064D8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825064DC: 4099005C  ble cr6, 0x82506538
	if !ctx.cr[6].gt {
	pc = 0x82506538; continue 'dispatch;
	}
	// 825064E0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825064E4: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 825064E8: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 825064EC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825064F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825064F4: 419A0030  beq cr6, 0x82506524
	if ctx.cr[6].eq {
	pc = 0x82506524; continue 'dispatch;
	}
	// 825064F8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 825064FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82506500: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82506504: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82506508: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8250650C: 409A0018  bne cr6, 0x82506524
	if !ctx.cr[6].eq {
	pc = 0x82506524; continue 'dispatch;
	}
	// 82506510: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506514: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82506518: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250651C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82506520: 4E800421  bctrl
	ctx.lr = 0x82506524;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82506524: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82506528: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8250652C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82506530: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82506534: 4198FFB0  blt cr6, 0x825064e4
	if ctx.cr[6].lt {
	pc = 0x825064E4; continue 'dispatch;
	}
	// 82506538: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8250653C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82506540: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82506544: 409A0020  bne cr6, 0x82506564
	if !ctx.cr[6].eq {
	pc = 0x82506564; continue 'dispatch;
	}
	// 82506548: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250654C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82506550: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82506554: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82506558: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8250655C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82506560: 4BF5DB59  bl 0x824640b8
	ctx.lr = 0x82506564;
	sub_824640B8(ctx, base);
	// 82506564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82506568: 4BFF48E1  bl 0x824fae48
	ctx.lr = 0x8250656C;
	sub_824FAE48(ctx, base);
	// 8250656C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82506570: 4802EB9C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82506578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82506578 size=44
    let mut pc: u32 = 0x82506578;
    'dispatch: loop {
        match pc {
            0x82506578 => {
    //   block [0x82506578..0x825065A4)
	// 82506578: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250657C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82506580: 396B47E0  addi r11, r11, 0x47e0
	ctx.r[11].s64 = ctx.r[11].s64 + 18400;
	// 82506584: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82506588: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 8250658C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82506590: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82506594: 91430044  stw r10, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82506598: 91430048  stw r10, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 8250659C: 9103004C  stw r8, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[8].u32 ) };
	// 825065A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825065A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825065A8 size=228
    let mut pc: u32 = 0x825065A8;
    'dispatch: loop {
        match pc {
            0x825065A8 => {
    //   block [0x825065A8..0x8250668C)
	// 825065A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825065AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825065B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825065B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825065B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825065BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825065C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825065C4: 480088E5  bl 0x8250eea8
	ctx.lr = 0x825065C8;
	sub_8250EEA8(ctx, base);
	// 825065C8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825065CC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 825065D0: 396B47B4  addi r11, r11, 0x47b4
	ctx.r[11].s64 = ctx.r[11].s64 + 18356;
	// 825065D4: 394A4790  addi r10, r10, 0x4790
	ctx.r[10].s64 = ctx.r[10].s64 + 18320;
	// 825065D8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825065DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825065E0: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825065E4: 419A008C  beq cr6, 0x82506670
	if ctx.cr[6].eq {
	pc = 0x82506670; continue 'dispatch;
	}
	// 825065E8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825065EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825065F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825065F4: 4099007C  ble cr6, 0x82506670
	if !ctx.cr[6].gt {
	pc = 0x82506670; continue 'dispatch;
	}
	// 825065F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825065FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82506600: 815F0060  lwz r10, 0x60(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82506604: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82506608: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8250660C: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82506610: 88CB0010  lbz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82506614: 80AA0008  lwz r5, 8(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82506618: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 8250661C: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82506620: 409A000C  bne cr6, 0x8250662c
	if !ctx.cr[6].eq {
	pc = 0x8250662C; continue 'dispatch;
	}
	// 82506624: 80CA0014  lwz r6, 0x14(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82506628: 48000008  b 0x82506630
	pc = 0x82506630; continue 'dispatch;
	// 8250662C: 80CA0020  lwz r6, 0x20(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82506630: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82506634: 88CB0011  lbz r6, 0x11(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 82506638: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 8250663C: 409A000C  bne cr6, 0x82506648
	if !ctx.cr[6].eq {
	pc = 0x82506648; continue 'dispatch;
	}
	// 82506640: 80CA002C  lwz r6, 0x2c(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82506644: 48000008  b 0x8250664c
	pc = 0x8250664C; continue 'dispatch;
	// 82506648: 80CA0044  lwz r6, 0x44(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250664C: 90CB0020  stw r6, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 82506650: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82506654: 814A0038  lwz r10, 0x38(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 82506658: 39290038  addi r9, r9, 0x38
	ctx.r[9].s64 = ctx.r[9].s64 + 56;
	// 8250665C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82506660: 914B0028  stw r10, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82506664: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82506668: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250666C: 4198FF94  blt cr6, 0x82506600
	if ctx.cr[6].lt {
	pc = 0x82506600; continue 'dispatch;
	}
	// 82506670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82506674: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82506678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250667C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82506680: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82506684: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82506688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82506690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82506690 size=1880
    let mut pc: u32 = 0x82506690;
    'dispatch: loop {
        match pc {
            0x82506690 => {
    //   block [0x82506690..0x82506DE8)
	// 82506690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82506694: 4802EA15  bl 0x825350a8
	ctx.lr = 0x82506698;
	sub_82535080(ctx, base);
	// 82506698: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250669C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825066A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825066A4: 3BFD0034  addi r31, r29, 0x34
	ctx.r[31].s64 = ctx.r[29].s64 + 52;
	// 825066A8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825066AC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825066B0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825066B4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825066B8: 409A0010  bne cr6, 0x825066c8
	if !ctx.cr[6].eq {
	pc = 0x825066C8; continue 'dispatch;
	}
	// 825066BC: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 825066C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825066C4: 4BF67C8D  bl 0x8246e350
	ctx.lr = 0x825066C8;
	sub_8246E350(ctx, base);
	// 825066C8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825066CC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 825066D0: 810D0000  lwz r8, 0(r13)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825066D4: 38A00025  li r5, 0x25
	ctx.r[5].s64 = 37;
	// 825066D8: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 825066DC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825066E0: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 * 56;
	// 825066E4: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 825066E8: 7C69402E  lwzx r3, r9, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 825066EC: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 825066F0: 7F6B5214  add r27, r11, r10
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825066F4: 4BF5D945  bl 0x82464038
	ctx.lr = 0x825066F8;
	sub_82464038(ctx, base);
	// 825066F8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825066FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82506700: 394B47E0  addi r10, r11, 0x47e0
	ctx.r[10].s64 = ctx.r[11].s64 + 18400;
	// 82506704: 39200050  li r9, 0x50
	ctx.r[9].s64 = 80;
	// 82506708: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 8250670C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82506710: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82506714: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82506718: 3BBD0060  addi r29, r29, 0x60
	ctx.r[29].s64 = ctx.r[29].s64 + 96;
	// 8250671C: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82506720: B31F0006  sth r24, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[24].u16 ) };
	// 82506724: 935F0008  stw r26, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 82506728: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8250672C: 935F000C  stw r26, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 82506730: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82506734: 935F0014  stw r26, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 82506738: 935F0018  stw r26, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[26].u32 ) };
	// 8250673C: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82506740: 935F0020  stw r26, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[26].u32 ) };
	// 82506744: 935F0024  stw r26, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[26].u32 ) };
	// 82506748: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8250674C: 935F002C  stw r26, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[26].u32 ) };
	// 82506750: 935F0030  stw r26, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[26].u32 ) };
	// 82506754: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82506758: 935F0038  stw r26, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[26].u32 ) };
	// 8250675C: 935F003C  stw r26, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[26].u32 ) };
	// 82506760: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82506764: 935F0044  stw r26, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[26].u32 ) };
	// 82506768: 935F0048  stw r26, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[26].u32 ) };
	// 8250676C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82506770: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506774: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82506778: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250677C: 409A0010  bne cr6, 0x8250678c
	if !ctx.cr[6].eq {
	pc = 0x8250678C; continue 'dispatch;
	}
	// 82506780: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82506784: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82506788: 4BF67BC9  bl 0x8246e350
	ctx.lr = 0x8250678C;
	sub_8246E350(ctx, base);
	// 8250678C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506790: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506794: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82506798: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 8250679C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825067A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825067A4: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825067A8: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825067AC: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825067B0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825067B4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825067B8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 825067BC: 832AFFFC  lwz r25, -4(r10)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 825067C0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825067C4: 3BF90008  addi r31, r25, 8
	ctx.r[31].s64 = ctx.r[25].s64 + 8;
	// 825067C8: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825067CC: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825067D0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825067D4: 7FAAE214  add r29, r10, r28
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 825067D8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825067DC: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 825067E0: 40980024  bge cr6, 0x82506804
	if !ctx.cr[6].lt {
	pc = 0x82506804; continue 'dispatch;
	}
	// 825067E4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825067E8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825067EC: 41980008  blt cr6, 0x825067f4
	if ctx.cr[6].lt {
	pc = 0x825067F4; continue 'dispatch;
	}
	// 825067F0: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 825067F4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 825067F8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825067FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82506800: 4BF67AC9  bl 0x8246e2c8
	ctx.lr = 0x82506804;
	sub_8246E2C8(ctx, base);
	// 82506804: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506808: 578B103A  slwi r11, r28, 2
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250680C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82506810: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82506814: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82506818: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250681C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506820: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82506824: 40990038  ble cr6, 0x8250685c
	if !ctx.cr[6].gt {
	pc = 0x8250685C; continue 'dispatch;
	}
	// 82506828: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250682C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82506830: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82506834: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82506838: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8250683C: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82506840: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82506844: 811E0004  lwz r8, 4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506848: 80FE0008  lwz r7, 8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250684C: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82506850: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82506854: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82506858: 4198FFD0  blt cr6, 0x82506828
	if ctx.cr[6].lt {
	pc = 0x82506828; continue 'dispatch;
	}
	// 8250685C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506860: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82506864: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82506868: 915B0004  stw r10, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8250686C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82506870: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82506874: 897E0010  lbz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82506878: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8250687C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82506880: 409A00F0  bne cr6, 0x82506970
	if !ctx.cr[6].eq {
	pc = 0x82506970; continue 'dispatch;
	}
	// 82506884: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82506888: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 8250688C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82506890: 40990008  ble cr6, 0x82506898
	if !ctx.cr[6].gt {
	pc = 0x82506898; continue 'dispatch;
	}
	// 82506894: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82506898: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250689C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 825068A0: 917B0014  stw r11, 0x14(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 825068A4: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 825068A8: 419A0024  beq cr6, 0x825068cc
	if ctx.cr[6].eq {
	pc = 0x825068CC; continue 'dispatch;
	}
	// 825068AC: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 825068B0: 419A0010  beq cr6, 0x825068c0
	if ctx.cr[6].eq {
	pc = 0x825068C0; continue 'dispatch;
	}
	// 825068B4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825068B8: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825068BC: 48000014  b 0x825068d0
	pc = 0x825068D0; continue 'dispatch;
	// 825068C0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825068C4: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 825068C8: 48000008  b 0x825068d0
	pc = 0x825068D0; continue 'dispatch;
	// 825068CC: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 825068D0: 3BF90014  addi r31, r25, 0x14
	ctx.r[31].s64 = ctx.r[25].s64 + 20;
	// 825068D4: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825068D8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825068DC: 7FBC5214  add r29, r28, r10
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[10].u64;
	// 825068E0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825068E4: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 825068E8: 40980024  bge cr6, 0x8250690c
	if !ctx.cr[6].lt {
	pc = 0x8250690C; continue 'dispatch;
	}
	// 825068EC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825068F0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825068F4: 41980008  blt cr6, 0x825068fc
	if ctx.cr[6].lt {
	pc = 0x825068FC; continue 'dispatch;
	}
	// 825068F8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 825068FC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82506900: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82506904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82506908: 4BF679C1  bl 0x8246e2c8
	ctx.lr = 0x8250690C;
	sub_8246E2C8(ctx, base);
	// 8250690C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506910: 578B083C  slwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82506914: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82506918: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 8250691C: 811E001C  lwz r8, 0x1c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82506920: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82506924: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82506928: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8250692C: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82506930: 40990128  ble cr6, 0x82506a58
	if !ctx.cr[6].gt {
	pc = 0x82506A58; continue 'dispatch;
	}
	// 82506934: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506938: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8250693C: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82506940: A10A0002  lhz r8, 2(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 82506944: B10B0002  sth r8, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 82506948: A10A0004  lhz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250694C: B10B0004  sth r8, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82506950: 80FB0014  lwz r7, 0x14(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82506954: 811E0014  lwz r8, 0x14(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82506958: 80BE001C  lwz r5, 0x1c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250695C: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82506960: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82506964: 7F092800  cmpw cr6, r9, r5
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82506968: 4198FFCC  blt cr6, 0x82506934
	if ctx.cr[6].lt {
	pc = 0x82506934; continue 'dispatch;
	}
	// 8250696C: 480000EC  b 0x82506a58
	pc = 0x82506A58; continue 'dispatch;
	// 82506970: 556AF0BE  srwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82506974: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 82506978: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8250697C: 40990008  ble cr6, 0x82506984
	if !ctx.cr[6].gt {
	pc = 0x82506984; continue 'dispatch;
	}
	// 82506980: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82506984: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82506988: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8250698C: 917B0014  stw r11, 0x14(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82506990: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82506994: 419A0024  beq cr6, 0x825069b8
	if ctx.cr[6].eq {
	pc = 0x825069B8; continue 'dispatch;
	}
	// 82506998: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 8250699C: 419A0010  beq cr6, 0x825069ac
	if ctx.cr[6].eq {
	pc = 0x825069AC; continue 'dispatch;
	}
	// 825069A0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825069A4: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825069A8: 48000014  b 0x825069bc
	pc = 0x825069BC; continue 'dispatch;
	// 825069AC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825069B0: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 825069B4: 48000008  b 0x825069bc
	pc = 0x825069BC; continue 'dispatch;
	// 825069B8: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 825069BC: 3BF90020  addi r31, r25, 0x20
	ctx.r[31].s64 = ctx.r[25].s64 + 32;
	// 825069C0: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825069C4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825069C8: 7FBC5214  add r29, r28, r10
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[10].u64;
	// 825069CC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825069D0: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 825069D4: 40980024  bge cr6, 0x825069f8
	if !ctx.cr[6].lt {
	pc = 0x825069F8; continue 'dispatch;
	}
	// 825069D8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825069DC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825069E0: 41980008  blt cr6, 0x825069e8
	if ctx.cr[6].lt {
	pc = 0x825069E8; continue 'dispatch;
	}
	// 825069E4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 825069E8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 825069EC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825069F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825069F4: 4BF678D5  bl 0x8246e2c8
	ctx.lr = 0x825069F8;
	sub_8246E2C8(ctx, base);
	// 825069F8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825069FC: 578B103A  slwi r11, r28, 2
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82506A00: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82506A04: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82506A08: 811E001C  lwz r8, 0x1c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82506A0C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82506A10: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82506A14: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82506A18: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82506A1C: 4099003C  ble cr6, 0x82506a58
	if !ctx.cr[6].gt {
	pc = 0x82506A58; continue 'dispatch;
	}
	// 82506A20: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506A24: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82506A28: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82506A2C: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506A30: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82506A34: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82506A38: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82506A3C: 80FB0014  lwz r7, 0x14(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82506A40: 811E0014  lwz r8, 0x14(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82506A44: 80BE001C  lwz r5, 0x1c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82506A48: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82506A4C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82506A50: 7F092800  cmpw cr6, r9, r5
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82506A54: 4198FFCC  blt cr6, 0x82506a20
	if ctx.cr[6].lt {
	pc = 0x82506A20; continue 'dispatch;
	}
	// 82506A58: 897E0010  lbz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82506A5C: 997B0010  stb r11, 0x10(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82506A60: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82506A64: 917B001C  stw r11, 0x1c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82506A68: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82506A6C: 90DB000C  stw r6, 0xc(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82506A70: 917B0018  stw r11, 0x18(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82506A74: 897E0011  lbz r11, 0x11(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(17 as u32) ) } as u64;
	// 82506A78: 997B0011  stb r11, 0x11(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82506A7C: 83BE0020  lwz r29, 0x20(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82506A80: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82506A84: 419A0210  beq cr6, 0x82506c94
	if ctx.cr[6].eq {
	pc = 0x82506C94; continue 'dispatch;
	}
	// 82506A88: 897E0011  lbz r11, 0x11(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(17 as u32) ) } as u64;
	// 82506A8C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82506A90: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82506A94: 409A00E0  bne cr6, 0x82506b74
	if !ctx.cr[6].eq {
	pc = 0x82506B74; continue 'dispatch;
	}
	// 82506A98: 3BF9002C  addi r31, r25, 0x2c
	ctx.r[31].s64 = ctx.r[25].s64 + 44;
	// 82506A9C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82506AA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82506AA4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82506AA8: 409A0050  bne cr6, 0x82506af8
	if !ctx.cr[6].eq {
	pc = 0x82506AF8; continue 'dispatch;
	}
	// 82506AAC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506AB0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82506AB4: 409A0010  bne cr6, 0x82506ac4
	if !ctx.cr[6].eq {
	pc = 0x82506AC4; continue 'dispatch;
	}
	// 82506AB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82506ABC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82506AC0: 4BF67891  bl 0x8246e350
	ctx.lr = 0x82506AC4;
	sub_8246E350(ctx, base);
	// 82506AC4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506AC8: 895D0000  lbz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506ACC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506AD0: 7D4959AE  stbx r10, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 82506AD4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506AD8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82506ADC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82506AE0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82506AE4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506AE8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82506AEC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82506AF0: 917B0020  stw r11, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82506AF4: 48000168  b 0x82506c5c
	pc = 0x82506C5C; continue 'dispatch;
	// 82506AF8: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82506AFC: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506B00: 7FBC5214  add r29, r28, r10
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[10].u64;
	// 82506B04: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82506B08: 40980024  bge cr6, 0x82506b2c
	if !ctx.cr[6].lt {
	pc = 0x82506B2C; continue 'dispatch;
	}
	// 82506B0C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82506B10: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82506B14: 41980008  blt cr6, 0x82506b1c
	if ctx.cr[6].lt {
	pc = 0x82506B1C; continue 'dispatch;
	}
	// 82506B18: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82506B1C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82506B20: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82506B24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82506B28: 4BF677A1  bl 0x8246e2c8
	ctx.lr = 0x82506B2C;
	sub_8246E2C8(ctx, base);
	// 82506B2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506B30: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82506B34: 7D3C5A14  add r9, r28, r11
	ctx.r[9].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 82506B38: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82506B3C: 913B0020  stw r9, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82506B40: 811E001C  lwz r8, 0x1c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82506B44: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82506B48: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82506B4C: 40990110  ble cr6, 0x82506c5c
	if !ctx.cr[6].gt {
	pc = 0x82506C5C; continue 'dispatch;
	}
	// 82506B50: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506B54: 7D0B49AE  stbx r8, r11, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u8) };
	// 82506B58: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82506B5C: 811E0024  lwz r8, 0x24(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82506B60: 80FE001C  lwz r7, 0x1c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82506B64: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82506B68: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82506B6C: 4198FFE4  blt cr6, 0x82506b50
	if ctx.cr[6].lt {
	pc = 0x82506B50; continue 'dispatch;
	}
	// 82506B70: 480000EC  b 0x82506c5c
	pc = 0x82506C5C; continue 'dispatch;
	// 82506B74: 3BF90044  addi r31, r25, 0x44
	ctx.r[31].s64 = ctx.r[25].s64 + 68;
	// 82506B78: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82506B7C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82506B80: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82506B84: 409A0058  bne cr6, 0x82506bdc
	if !ctx.cr[6].eq {
	pc = 0x82506BDC; continue 'dispatch;
	}
	// 82506B88: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506B8C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82506B90: 409A0010  bne cr6, 0x82506ba0
	if !ctx.cr[6].eq {
	pc = 0x82506BA0; continue 'dispatch;
	}
	// 82506B94: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82506B98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82506B9C: 4BF677B5  bl 0x8246e350
	ctx.lr = 0x82506BA0;
	sub_8246E350(ctx, base);
	// 82506BA0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506BA4: A15D0000  lhz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506BA8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82506BAC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506BB0: 7D4B4B2E  sthx r10, r11, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u16) };
	// 82506BB4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506BB8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82506BBC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82506BC0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82506BC4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506BC8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82506BCC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82506BD0: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82506BD4: 917B0020  stw r11, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82506BD8: 48000084  b 0x82506c5c
	pc = 0x82506C5C; continue 'dispatch;
	// 82506BDC: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82506BE0: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506BE4: 7FBC5214  add r29, r28, r10
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[10].u64;
	// 82506BE8: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82506BEC: 40980024  bge cr6, 0x82506c10
	if !ctx.cr[6].lt {
	pc = 0x82506C10; continue 'dispatch;
	}
	// 82506BF0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82506BF4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82506BF8: 41980008  blt cr6, 0x82506c00
	if ctx.cr[6].lt {
	pc = 0x82506C00; continue 'dispatch;
	}
	// 82506BFC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82506C00: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82506C04: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82506C08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82506C0C: 4BF676BD  bl 0x8246e2c8
	ctx.lr = 0x82506C10;
	sub_8246E2C8(ctx, base);
	// 82506C10: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506C14: 578B083C  slwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82506C18: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82506C1C: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82506C20: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82506C24: 913B0020  stw r9, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82506C28: 811E001C  lwz r8, 0x1c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82506C2C: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82506C30: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82506C34: 40990028  ble cr6, 0x82506c5c
	if !ctx.cr[6].gt {
	pc = 0x82506C5C; continue 'dispatch;
	}
	// 82506C38: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506C3C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82506C40: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82506C44: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82506C48: 811E0024  lwz r8, 0x24(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82506C4C: 80FE001C  lwz r7, 0x1c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82506C50: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82506C54: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82506C58: 4198FFE0  blt cr6, 0x82506c38
	if ctx.cr[6].lt {
	pc = 0x82506C38; continue 'dispatch;
	}
	// 82506C5C: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82506C60: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82506C64: 917B0024  stw r11, 0x24(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82506C68: 419A0034  beq cr6, 0x82506c9c
	if ctx.cr[6].eq {
	pc = 0x82506C9C; continue 'dispatch;
	}
	// 82506C6C: 897B0011  lbz r11, 0x11(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(17 as u32) ) } as u64;
	// 82506C70: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82506C74: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82506C78: 419A0014  beq cr6, 0x82506c8c
	if ctx.cr[6].eq {
	pc = 0x82506C8C; continue 'dispatch;
	}
	// 82506C7C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82506C80: 409A001C  bne cr6, 0x82506c9c
	if !ctx.cr[6].eq {
	pc = 0x82506C9C; continue 'dispatch;
	}
	// 82506C84: 917B0024  stw r11, 0x24(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82506C88: 48000014  b 0x82506c9c
	pc = 0x82506C9C; continue 'dispatch;
	// 82506C8C: 931B0024  stw r24, 0x24(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(36 as u32), ctx.r[24].u32 ) };
	// 82506C90: 4800000C  b 0x82506c9c
	pc = 0x82506C9C; continue 'dispatch;
	// 82506C94: 935B0020  stw r26, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[26].u32 ) };
	// 82506C98: 935B0024  stw r26, 0x24(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(36 as u32), ctx.r[26].u32 ) };
	// 82506C9C: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82506CA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82506CA4: 419A011C  beq cr6, 0x82506dc0
	if ctx.cr[6].eq {
	pc = 0x82506DC0; continue 'dispatch;
	}
	// 82506CA8: 3BF90038  addi r31, r25, 0x38
	ctx.r[31].s64 = ctx.r[25].s64 + 56;
	// 82506CAC: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82506CB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82506CB4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82506CB8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82506CBC: 409A0060  bne cr6, 0x82506d1c
	if !ctx.cr[6].eq {
	pc = 0x82506D1C; continue 'dispatch;
	}
	// 82506CC0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506CC4: 83BE0028  lwz r29, 0x28(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82506CC8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82506CCC: 409A0010  bne cr6, 0x82506cdc
	if !ctx.cr[6].eq {
	pc = 0x82506CDC; continue 'dispatch;
	}
	// 82506CD0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82506CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82506CD8: 4BF67679  bl 0x8246e350
	ctx.lr = 0x82506CDC;
	sub_8246E350(ctx, base);
	// 82506CDC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506CE0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506CE4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506CE8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82506CEC: 7D4B492E  stwx r10, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 82506CF0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506CF4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82506CF8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82506CFC: 935B002C  stw r26, 0x2c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(44 as u32), ctx.r[26].u32 ) };
	// 82506D00: 931B0030  stw r24, 0x30(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(48 as u32), ctx.r[24].u32 ) };
	// 82506D04: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506D08: 917B0028  stw r11, 0x28(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82506D0C: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82506D10: 917B0034  stw r11, 0x34(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82506D14: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82506D18: 4802E3E0  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82506D1C: 815E0030  lwz r10, 0x30(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82506D20: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82506D24: 7FBC5214  add r29, r28, r10
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[10].u64;
	// 82506D28: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82506D2C: 40980024  bge cr6, 0x82506d50
	if !ctx.cr[6].lt {
	pc = 0x82506D50; continue 'dispatch;
	}
	// 82506D30: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82506D34: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82506D38: 41980008  blt cr6, 0x82506d40
	if ctx.cr[6].lt {
	pc = 0x82506D40; continue 'dispatch;
	}
	// 82506D3C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82506D40: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82506D44: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82506D48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82506D4C: 4BF6757D  bl 0x8246e2c8
	ctx.lr = 0x82506D50;
	sub_8246E2C8(ctx, base);
	// 82506D50: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506D54: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82506D58: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82506D5C: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82506D60: 811E0030  lwz r8, 0x30(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82506D64: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82506D68: 815E0028  lwz r10, 0x28(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82506D6C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82506D70: 40990028  ble cr6, 0x82506d98
	if !ctx.cr[6].gt {
	pc = 0x82506D98; continue 'dispatch;
	}
	// 82506D74: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506D78: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82506D7C: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82506D80: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82506D84: 811E002C  lwz r8, 0x2c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82506D88: 80FE0030  lwz r7, 0x30(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82506D8C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82506D90: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82506D94: 4198FFE0  blt cr6, 0x82506d74
	if ctx.cr[6].lt {
	pc = 0x82506D74; continue 'dispatch;
	}
	// 82506D98: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82506D9C: 917B002C  stw r11, 0x2c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82506DA0: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82506DA4: 917B0030  stw r11, 0x30(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82506DA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506DAC: 917B0028  stw r11, 0x28(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82506DB0: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82506DB4: 917B0034  stw r11, 0x34(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82506DB8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82506DBC: 4802E33C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82506DC0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82506DC4: 931B0030  stw r24, 0x30(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(48 as u32), ctx.r[24].u32 ) };
	// 82506DC8: 935B002C  stw r26, 0x2c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(44 as u32), ctx.r[26].u32 ) };
	// 82506DCC: 396B9F50  addi r11, r11, -0x60b0
	ctx.r[11].s64 = ctx.r[11].s64 + -24752;
	// 82506DD0: 917B0020  stw r11, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82506DD4: 917B0028  stw r11, 0x28(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82506DD8: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82506DDC: 917B0034  stw r11, 0x34(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82506DE0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82506DE4: 4802E314  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82506DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82506DE8 size=348
    let mut pc: u32 = 0x82506DE8;
    'dispatch: loop {
        match pc {
            0x82506DE8 => {
    //   block [0x82506DE8..0x82506F44)
	// 82506DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82506DEC: 4802E2CD  bl 0x825350b8
	ctx.lr = 0x82506DF0;
	sub_82535080(ctx, base);
	// 82506DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82506DF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82506DF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82506DFC: 80BE0030  lwz r5, 0x30(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82506E00: C03E0050  lfs f1, 0x50(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82506E04: 48007FF5  bl 0x8250edf8
	ctx.lr = 0x82506E08;
	sub_8250EDF8(ctx, base);
	// 82506E08: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82506E0C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82506E10: 396B47B4  addi r11, r11, 0x47b4
	ctx.r[11].s64 = ctx.r[11].s64 + 18356;
	// 82506E14: 394A4790  addi r10, r10, 0x4790
	ctx.r[10].s64 = ctx.r[10].s64 + 18320;
	// 82506E18: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82506E1C: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82506E20: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82506E24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82506E28: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82506E2C: 93BF0060  stw r29, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82506E30: 93BF0064  stw r29, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82506E34: 911F0068  stw r8, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[8].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82506F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82506F48 size=8
    let mut pc: u32 = 0x82506F48;
    'dispatch: loop {
        match pc {
            0x82506F48 => {
    //   block [0x82506F48..0x82506F50)
	// 82506F48: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82506F4C: 480001B4  b 0x82507100
	sub_82507100(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82506F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82506F50 size=328
    let mut pc: u32 = 0x82506F50;
    'dispatch: loop {
        match pc {
            0x82506F50 => {
    //   block [0x82506F50..0x82507098)
	// 82506F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82506F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82506F58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82506F5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82506F60: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82506F64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82506F68: 396B47E0  addi r11, r11, 0x47e0
	ctx.r[11].s64 = ctx.r[11].s64 + 18400;
	// 82506F6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82506F70: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82506F74: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82506F78: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82506F7C: 409A0020  bne cr6, 0x82506f9c
	if !ctx.cr[6].eq {
	pc = 0x82506F9C; continue 'dispatch;
	}
	// 82506F80: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506F84: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82506F88: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82506F8C: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82506F90: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82506F94: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82506F98: 4BF5D121  bl 0x824640b8
	ctx.lr = 0x82506F9C;
	sub_824640B8(ctx, base);
	// 82506F9C: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82506FA0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82506FA4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82506FA8: 409A0020  bne cr6, 0x82506fc8
	if !ctx.cr[6].eq {
	pc = 0x82506FC8; continue 'dispatch;
	}
	// 82506FAC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506FB0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82506FB4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82506FB8: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82506FBC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82506FC0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82506FC4: 4BF5D0F5  bl 0x824640b8
	ctx.lr = 0x82506FC8;
	sub_824640B8(ctx, base);
	// 82506FC8: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82506FCC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82506FD0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82506FD4: 409A0020  bne cr6, 0x82506ff4
	if !ctx.cr[6].eq {
	pc = 0x82506FF4; continue 'dispatch;
	}
	// 82506FD8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82506FDC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82506FE0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82506FE4: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82506FE8: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82506FEC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82506FF0: 4BF5D0C9  bl 0x824640b8
	ctx.lr = 0x82506FF4;
	sub_824640B8(ctx, base);
	// 82506FF4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82506FF8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82506FFC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82507000: 409A0020  bne cr6, 0x82507020
	if !ctx.cr[6].eq {
	pc = 0x82507020; continue 'dispatch;
	}
	// 82507004: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507008: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8250700C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82507010: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82507014: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82507018: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8250701C: 4BF5D09D  bl 0x824640b8
	ctx.lr = 0x82507020;
	sub_824640B8(ctx, base);
	// 82507020: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82507024: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82507028: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8250702C: 409A0020  bne cr6, 0x8250704c
	if !ctx.cr[6].eq {
	pc = 0x8250704C; continue 'dispatch;
	}
	// 82507030: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507034: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82507038: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8250703C: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82507040: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82507044: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82507048: 4BF5D071  bl 0x824640b8
	ctx.lr = 0x8250704C;
	sub_824640B8(ctx, base);
	// 8250704C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82507050: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82507054: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82507058: 409A0020  bne cr6, 0x82507078
	if !ctx.cr[6].eq {
	pc = 0x82507078; continue 'dispatch;
	}
	// 8250705C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507060: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82507064: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82507068: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250706C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82507070: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82507074: 4BF5D045  bl 0x824640b8
	ctx.lr = 0x82507078;
	sub_824640B8(ctx, base);
	// 82507078: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8250707C: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82507080: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82507084: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82507088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250708C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82507090: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82507094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82507098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82507098 size=100
    let mut pc: u32 = 0x82507098;
    'dispatch: loop {
        match pc {
            0x82507098 => {
    //   block [0x82507098..0x825070FC)
	// 82507098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250709C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825070A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825070A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825070A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825070AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825070B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825070B4: 4BFFFE9D  bl 0x82506f50
	ctx.lr = 0x825070B8;
	sub_82506F50(ctx, base);
	// 825070B8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 825070BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825070C0: 419A0020  beq cr6, 0x825070e0
	if ctx.cr[6].eq {
	pc = 0x825070E0; continue 'dispatch;
	}
	// 825070C4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825070C8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825070CC: 38C00025  li r6, 0x25
	ctx.r[6].s64 = 37;
	// 825070D0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825070D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825070D8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825070DC: 4BF5CFDD  bl 0x824640b8
	ctx.lr = 0x825070E0;
	sub_824640B8(ctx, base);
	// 825070E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825070E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825070E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825070EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825070F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825070F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825070F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


