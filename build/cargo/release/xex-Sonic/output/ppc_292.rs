pub fn sub_831F8428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831F8428 size=560
    let mut pc: u32 = 0x831F8428;
    'dispatch: loop {
        match pc {
            0x831F8428 => {
    //   block [0x831F8428..0x831F8658)
	// 831F8428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F842C: 4BFAFD3D  bl 0x831a8168
	ctx.lr = 0x831F8430;
	sub_831A8130(ctx, base);
	// 831F8430: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F8434: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F8438: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F843C: 3BC30034  addi r30, r3, 0x34
	ctx.r[30].s64 = ctx.r[3].s64 + 52;
	// 831F8440: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831F8444: 80A30008  lwz r5, 8(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F8448: 8883000D  lbz r4, 0xd(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F844C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831F8450: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F8454: 7D0429D6  mullw r8, r4, r5
	ctx.r[8].s64 = (ctx.r[4].s32 as i64) * (ctx.r[5].s32 as i64);
	// 831F8458: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F845C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F8460: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 831F8464: 7D455050  subf r10, r5, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[5].s64;
	// 831F8468: 5508083C  slwi r8, r8, 1
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831F846C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 831F8470: 7CE63A14  add r7, r6, r7
	ctx.r[7].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 831F8474: 7CA84A14  add r5, r8, r9
	ctx.r[5].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 831F8478: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831F847C: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 831F8480: 41980008  blt cr6, 0x831f8488
	if ctx.cr[6].lt {
	pc = 0x831F8488; continue 'dispatch;
	}
	// 831F8484: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 831F8488: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 831F848C: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F8490: ED6D0028  fsubs f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831F8494: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 831F8498: F961FFD0  std r11, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.r[11].u64 ) };
	// 831F849C: C941FFD0  lfd f10, -0x30(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 831F84A0: FD20569C  fcfid f9, f10
	ctx.f[9].f64 = (ctx.f[10].s64 as f64);
	// 831F84A4: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 831F84A8: FD004818  frsp f8, f9
	ctx.f[8].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F84AC: C1AA9524  lfs f13, -0x6adc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F84B0: C1897490  lfs f12, 0x7490(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(29840 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831F84B4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831F84B8: ED4B4024  fdivs f10, f11, f8
	ctx.f[10].f64 = ((ctx.f[11].f64 / ctx.f[8].f64) as f32) as f64;
	// 831F84BC: ED2A0372  fmuls f9, f10, f13
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F84C0: 419A0040  beq cr6, 0x831f8500
	if ctx.cr[6].eq {
	pc = 0x831F8500; continue 'dispatch;
	}
	// 831F84C4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 831F84C8: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 831F84CC: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F84D0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831F84D4: 5528C63E  rlwinm r8, r9, 0x18, 0x18, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 831F84D8: 5128442E  rlwimi r8, r9, 8, 0x10, 0x17
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[8].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F84DC: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 831F84E0: F901FFD0  std r8, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.r[8].u64 ) };
	// 831F84E4: C9A1FFD0  lfd f13, -0x30(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 831F84E8: FD606E9C  fcfid f11, f13
	ctx.f[11].f64 = (ctx.f[13].s64 as f64);
	// 831F84EC: FD005818  frsp f8, f11
	ctx.f[8].f64 = (ctx.f[11].f64 as f32) as f64;
	// 831F84F0: ECE80332  fmuls f7, f8, f12
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F84F4: D0EB0000  stfs f7, 0(r11)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831F84F8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831F84FC: 4082FFD0  bne 0x831f84cc
	if !ctx.cr[0].eq {
	pc = 0x831F84CC; continue 'dispatch;
	}
	// 831F8500: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 831F8504: 549F083C  slwi r31, r4, 1
	ctx.r[31].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 831F8508: C16B9450  lfs f11, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 831F850C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831F8510: 419A0070  beq cr6, 0x831f8580
	if ctx.cr[6].eq {
	pc = 0x831F8580; continue 'dispatch;
	}
	// 831F8514: EDAA002A  fadds f13, f10, f0
	ctx.f[13].f64 = ((ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64;
	// 831F8518: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 831F851C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 831F8520: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 831F8524: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 831F8528: A3A80000  lhz r29, 0(r8)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F852C: C10B0000  lfs f8, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 831F8530: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F8534: 57BCC63E  rlwinm r28, r29, 0x18, 0x18, 0x1f
	ctx.r[28].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 831F8538: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 831F853C: 53BC442E  rlwimi r28, r29, 8, 0x10, 0x17
	ctx.r[28].u64 = (((ctx.r[29].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[28].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8540: 7F9D0734  extsh r29, r28
	ctx.r[29].s64 = ctx.r[28].s16 as i64;
	// 831F8544: FBA1FFD0  std r29, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.r[29].u64 ) };
	// 831F8548: C8E1FFD0  lfd f7, -0x30(r1)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 831F854C: FCC03E9C  fcfid f6, f7
	ctx.f[6].f64 = (ctx.f[7].s64 as f64);
	// 831F8550: FCA03018  frsp f5, f6
	ctx.f[5].f64 = (ctx.f[6].f64 as f32) as f64;
	// 831F8554: EC850332  fmuls f4, f5, f12
	ctx.f[4].f64 = (((ctx.f[5].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F8558: D08B0000  stfs f4, 0(r11)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831F855C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831F8560: EC64402A  fadds f3, f4, f8
	ctx.f[3].f64 = ((ctx.f[4].f64 + ctx.f[8].f64) as f32) as f64;
	// 831F8564: EC4D0132  fmuls f2, f13, f4
	ctx.f[2].f64 = (((ctx.f[13].f64 * ctx.f[4].f64) as f32) as f64);
	// 831F8568: EC230032  fmuls f1, f3, f0
	ctx.f[1].f64 = (((ctx.f[3].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F856C: ED0102F2  fmuls f8, f1, f11
	ctx.f[8].f64 = (((ctx.f[1].f64 * ctx.f[11].f64) as f32) as f64);
	// 831F8570: D10A0000  stfs f8, 0(r10)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831F8574: D04A0004  stfs f2, 4(r10)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 831F8578: 394A0400  addi r10, r10, 0x400
	ctx.r[10].s64 = ctx.r[10].s64 + 1024;
	// 831F857C: 4082FFAC  bne 0x831f8528
	if !ctx.cr[0].eq {
	pc = 0x831F8528; continue 'dispatch;
	}
	// 831F8580: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831F8584: EC09002A  fadds f0, f9, f0
	ctx.f[0].f64 = ((ctx.f[9].f64 + ctx.f[0].f64) as f32) as f64;
	// 831F8588: 7CBF2A14  add r5, r31, r5
	ctx.r[5].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 831F858C: 38E70008  addi r7, r7, 8
	ctx.r[7].s64 = ctx.r[7].s64 + 8;
	// 831F8590: 4082FF7C  bne 0x831f850c
	if !ctx.cr[0].eq {
	pc = 0x831F850C; continue 'dispatch;
	}
	// 831F8594: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F8598: 8943000D  lbz r10, 0xd(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F859C: 7D2B2850  subf r9, r11, r5
	ctx.r[9].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 831F85A0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F85A4: 5548083E  rotlwi r8, r10, 1
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 831F85A8: 7D494396  divwu r10, r9, r8
	ctx.r[10].u32 = ctx.r[9].u32 / ctx.r[8].u32;
	// 831F85AC: 0CC80000  twi 6, r8, 0
	// 831F85B0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831F85B4: 41980008  blt cr6, 0x831f85bc
	if ctx.cr[6].lt {
	pc = 0x831F85BC; continue 'dispatch;
	}
	// 831F85B8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 831F85BC: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F85C0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 831F85C4: 7D4B3850  subf r10, r11, r7
	ctx.r[10].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 831F85C8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F85CC: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F85D0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831F85D4: 40980008  bge cr6, 0x831f85dc
	if !ctx.cr[6].lt {
	pc = 0x831F85DC; continue 'dispatch;
	}
	// 831F85D8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831F85DC: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 831F85E0: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 831F85E4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831F85E8: 419A006C  beq cr6, 0x831f8654
	if ctx.cr[6].eq {
	pc = 0x831F8654; continue 'dispatch;
	}
	// 831F85EC: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 831F85F0: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 831F85F4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 831F85F8: 39000080  li r8, 0x80
	ctx.r[8].s64 = 128;
	// 831F85FC: 6129FF7F  ori r9, r9, 0xff7f
	ctx.r[9].u64 = ctx.r[9].u64 | 65407;
	// 831F8600: C00A6AB0  lfs f0, 0x6ab0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(27312 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F8604: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F8608: ED8D0032  fmuls f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F860C: FD60601E  fctiwz f11, f12
	ctx.f[11].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 831F8610: D961FFD0  stfd f11, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[11].u64 ) };
	// 831F8614: 8141FFD4  lwz r10, -0x2c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-44 as u32) ) } as u64;
	// 831F8618: 2F0A7FFF  cmpwi cr6, r10, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32767, &mut ctx.xer);
	// 831F861C: 4198000C  blt cr6, 0x831f8628
	if ctx.cr[6].lt {
	pc = 0x831F8628; continue 'dispatch;
	}
	// 831F8620: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 831F8624: 48000024  b 0x831f8648
	pc = 0x831F8648; continue 'dispatch;
	// 831F8628: 2F0A8000  cmpwi cr6, r10, -0x8000
	ctx.cr[6].compare_i32(ctx.r[10].s32, -32768, &mut ctx.xer);
	// 831F862C: 4199000C  bgt cr6, 0x831f8638
	if ctx.cr[6].gt {
	pc = 0x831F8638; continue 'dispatch;
	}
	// 831F8630: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831F8634: 48000014  b 0x831f8648
	pc = 0x831F8648; continue 'dispatch;
	// 831F8638: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 831F863C: 5547C63E  rlwinm r7, r10, 0x18, 0x18, 0x1f
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 831F8640: 5147442E  rlwimi r7, r10, 8, 0x10, 0x17
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8644: B0EB0000  sth r7, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 831F8648: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831F864C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831F8650: 4082FFB4  bne 0x831f8604
	if !ctx.cr[0].eq {
	pc = 0x831F8604; continue 'dispatch;
	}
	// 831F8654: 4BFAFB64  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F8658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831F8658 size=444
    let mut pc: u32 = 0x831F8658;
    'dispatch: loop {
        match pc {
            0x831F8658 => {
    //   block [0x831F8658..0x831F8814)
	// 831F8658: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F865C: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F8660: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F8664: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F8668: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831F866C: 8903000D  lbz r8, 0xd(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F8670: 7CAB5050  subf r5, r11, r10
	ctx.r[5].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831F8674: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F8678: 7D4849D6  mullw r10, r8, r9
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[9].s32 as i64);
	// 831F867C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F8680: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F8684: 7CA50E70  srawi r5, r5, 1
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[5].s32 >> 1) as i64;
	// 831F8688: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831F868C: 7D292050  subf r9, r9, r4
	ctx.r[9].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 831F8690: 7D450194  addze r10, r5
	tmp.s64 = ctx.r[5].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[5].u32);
	ctx.r[10].s64 = tmp.s64;
	// 831F8694: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 831F8698: 7D063A14  add r8, r6, r7
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 831F869C: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831F86A0: 41980008  blt cr6, 0x831f86a8
	if ctx.cr[6].lt {
	pc = 0x831F86A8; continue 'dispatch;
	}
	// 831F86A4: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 831F86A8: 7D4707B4  extsw r7, r10
	ctx.r[7].s64 = ctx.r[10].s32 as i64;
	// 831F86AC: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F86B0: ED6D0028  fsubs f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831F86B4: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F86B8: F8E1FFF0  std r7, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[7].u64 ) };
	// 831F86BC: C941FFF0  lfd f10, -0x10(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831F86C0: FD20569C  fcfid f9, f10
	ctx.f[9].f64 = (ctx.f[10].s64 as f64);
	// 831F86C4: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 831F86C8: FD004818  frsp f8, f9
	ctx.f[8].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F86CC: 38AA007F  addi r5, r10, 0x7f
	ctx.r[5].s64 = ctx.r[10].s64 + 127;
	// 831F86D0: C1869524  lfs f12, -0x6adc(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831F86D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831F86D8: 54A7C9FE  srwi r7, r5, 7
	ctx.r[7].u32 = ctx.r[5].u32.wrapping_shr(7);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831F86DC: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 831F86E0: EDAB4024  fdivs f13, f11, f8
	ctx.f[13].f64 = ((ctx.f[11].f64 / ctx.f[8].f64) as f32) as f64;
	// 831F86E4: ED8D0332  fmuls f12, f13, f12
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F86E8: 419A0018  beq cr6, 0x831f8700
	if ctx.cr[6].eq {
	pc = 0x831F8700; continue 'dispatch;
	}
	// 831F86EC: 55463830  slwi r6, r10, 7
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831F86F0: 7C065A2C  dcbt r6, r11
	// 831F86F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831F86F8: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 831F86FC: 4198FFF0  blt cr6, 0x831f86ec
	if ctx.cr[6].lt {
	pc = 0x831F86EC; continue 'dispatch;
	}
	// 831F8700: A1430034  lhz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 831F8704: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 831F8708: 5546C63E  rlwinm r6, r10, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 831F870C: 5146442E  rlwimi r6, r10, 8, 0x10, 0x17
	ctx.r[6].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8710: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 831F8714: C1477490  lfs f10, 0x7490(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(29840 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 831F8718: 7CC40734  extsh r4, r6
	ctx.r[4].s64 = ctx.r[6].s16 as i64;
	// 831F871C: F881FFF0  std r4, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[4].u64 ) };
	// 831F8720: C961FFF0  lfd f11, -0x10(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831F8724: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F8728: FD004818  frsp f8, f9
	ctx.f[8].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F872C: C16A9450  lfs f11, -0x6bb0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 831F8730: ECE802B2  fmuls f7, f8, f10
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[10].f64) as f32) as f64);
	// 831F8734: D0E30034  stfs f7, 0x34(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 831F8738: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F873C: C1230034  lfs f9, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 831F8740: ED0D002A  fadds f8, f13, f0
	ctx.f[8].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 831F8744: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F8748: 5547C63E  rlwinm r7, r10, 0x18, 0x18, 0x1f
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 831F874C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831F8750: 5147442E  rlwimi r7, r10, 8, 0x10, 0x17
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8754: 7CE50734  extsh r5, r7
	ctx.r[5].s64 = ctx.r[7].s16 as i64;
	// 831F8758: F8A1FFF0  std r5, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[5].u64 ) };
	// 831F875C: C8E1FFF0  lfd f7, -0x10(r1)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831F8760: FCC03E9C  fcfid f6, f7
	ctx.f[6].f64 = (ctx.f[7].s64 as f64);
	// 831F8764: FCA03018  frsp f5, f6
	ctx.f[5].f64 = (ctx.f[6].f64 as f32) as f64;
	// 831F8768: EC8502B2  fmuls f4, f5, f10
	ctx.f[4].f64 = (((ctx.f[5].f64 * ctx.f[10].f64) as f32) as f64);
	// 831F876C: D0830034  stfs f4, 0x34(r3)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 831F8770: EC64482A  fadds f3, f4, f9
	ctx.f[3].f64 = ((ctx.f[4].f64 + ctx.f[9].f64) as f32) as f64;
	// 831F8774: EC480132  fmuls f2, f8, f4
	ctx.f[2].f64 = (((ctx.f[8].f64 * ctx.f[4].f64) as f32) as f64);
	// 831F8778: EC230032  fmuls f1, f3, f0
	ctx.f[1].f64 = (((ctx.f[3].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F877C: EC0C002A  fadds f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 831F8780: ED2102F2  fmuls f9, f1, f11
	ctx.f[9].f64 = (((ctx.f[1].f64 * ctx.f[11].f64) as f32) as f64);
	// 831F8784: D1280000  stfs f9, 0(r8)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831F8788: D0480004  stfs f2, 4(r8)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 831F878C: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 831F8790: 4082FFA8  bne 0x831f8738
	if !ctx.cr[0].eq {
	pc = 0x831F8738; continue 'dispatch;
	}
	// 831F8794: 8943000D  lbz r10, 0xd(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F8798: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F879C: 5547083E  rotlwi r7, r10, 1
	ctx.r[7].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 831F87A0: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F87A4: 7CC95850  subf r6, r9, r11
	ctx.r[6].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 831F87A8: 0CC70000  twi 6, r7, 0
	// 831F87AC: 7D663B96  divwu r11, r6, r7
	ctx.r[11].u32 = ctx.r[6].u32 / ctx.r[7].u32;
	// 831F87B0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831F87B4: 40980008  bge cr6, 0x831f87bc
	if !ctx.cr[6].lt {
	pc = 0x831F87BC; continue 'dispatch;
	}
	// 831F87B8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 831F87BC: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F87C0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 831F87C4: 7D4B4050  subf r10, r11, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 831F87C8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F87CC: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F87D0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831F87D4: 40980008  bge cr6, 0x831f87dc
	if !ctx.cr[6].lt {
	pc = 0x831F87DC; continue 'dispatch;
	}
	// 831F87D8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831F87DC: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 831F87E0: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 831F87E4: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 831F87E8: C1A30034  lfs f13, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F87EC: C00A6AB0  lfs f0, 0x6ab0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(27312 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F87F0: ED8D0032  fmuls f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F87F4: FD60601E  fctiwz f11, f12
	ctx.f[11].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 831F87F8: D961FFF0  stfd f11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[11].u64 ) };
	// 831F87FC: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 831F8800: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 831F8804: 41980010  blt cr6, 0x831f8814
	if ctx.cr[6].lt {
		sub_831F8814(ctx, base);
		return;
	}
	// 831F8808: 3960FF7F  li r11, -0x81
	ctx.r[11].s64 = -129;
	// 831F880C: B1630034  sth r11, 0x34(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u16 ) };
	// 831F8810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F8814(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831F8814 size=20
    let mut pc: u32 = 0x831F8814;
    'dispatch: loop {
        match pc {
            0x831F8814 => {
    //   block [0x831F8814..0x831F8828)
	// 831F8814: 2F0B8000  cmpwi cr6, r11, -0x8000
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32768, &mut ctx.xer);
	// 831F8818: 41990010  bgt cr6, 0x831f8828
	if ctx.cr[6].gt {
		sub_831F8828(ctx, base);
		return;
	}
	// 831F881C: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 831F8820: B1630034  sth r11, 0x34(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u16 ) };
	// 831F8824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F8828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831F8828 size=20
    let mut pc: u32 = 0x831F8828;
    'dispatch: loop {
        match pc {
            0x831F8828 => {
    //   block [0x831F8828..0x831F883C)
	// 831F8828: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 831F882C: 556AC63E  rlwinm r10, r11, 0x18, 0x18, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 831F8830: 516A442E  rlwimi r10, r11, 8, 0x10, 0x17
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8834: B1430034  sth r10, 0x34(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[10].u16 ) };
	// 831F8838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F8840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831F8840 size=608
    let mut pc: u32 = 0x831F8840;
    'dispatch: loop {
        match pc {
            0x831F8840 => {
    //   block [0x831F8840..0x831F8AA0)
	// 831F8840: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 831F8844: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F8848: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F884C: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F8850: 38A30034  addi r5, r3, 0x34
	ctx.r[5].s64 = ctx.r[3].s64 + 52;
	// 831F8854: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F8858: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831F885C: 8903000D  lbz r8, 0xd(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F8860: 7C8B5050  subf r4, r11, r10
	ctx.r[4].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831F8864: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F8868: 7D4849D6  mullw r10, r8, r9
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[9].s32 as i64);
	// 831F886C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F8870: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F8874: 7C840E70  srawi r4, r4, 1
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[4].s32 >> 1) as i64;
	// 831F8878: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F887C: 7D09F850  subf r8, r9, r31
	ctx.r[8].s64 = ctx.r[31].s64 - ctx.r[9].s64;
	// 831F8880: 7D240194  addze r9, r4
	tmp.s64 = ctx.r[4].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[4].u32);
	ctx.r[9].s64 = tmp.s64;
	// 831F8884: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F8888: 7D463A14  add r10, r6, r7
	ctx.r[10].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 831F888C: 7F084800  cmpw cr6, r8, r9
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831F8890: 41980008  blt cr6, 0x831f8898
	if ctx.cr[6].lt {
	pc = 0x831F8898; continue 'dispatch;
	}
	// 831F8894: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 831F8898: 7D2707B4  extsw r7, r9
	ctx.r[7].s64 = ctx.r[9].s32 as i64;
	// 831F889C: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F88A0: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831F88A4: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831F88A8: F8E1FFE0  std r7, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[7].u64 ) };
	// 831F88AC: C961FFE0  lfd f11, -0x20(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 831F88B0: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 831F88B4: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 831F88B8: FD205018  frsp f9, f10
	ctx.f[9].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F88BC: 3889007F  addi r4, r9, 0x7f
	ctx.r[4].s64 = ctx.r[9].s64 + 127;
	// 831F88C0: C1A69524  lfs f13, -0x6adc(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F88C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831F88C8: 5487C9FE  srwi r7, r4, 7
	ctx.r[7].u32 = ctx.r[4].u32.wrapping_shr(7);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831F88CC: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 831F88D0: ED6C4824  fdivs f11, f12, f9
	ctx.f[11].f64 = ((ctx.f[12].f64 / ctx.f[9].f64) as f32) as f64;
	// 831F88D4: ED4B0372  fmuls f10, f11, f13
	ctx.f[10].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F88D8: 419A0018  beq cr6, 0x831f88f0
	if ctx.cr[6].eq {
	pc = 0x831F88F0; continue 'dispatch;
	}
	// 831F88DC: 55263830  slwi r6, r9, 7
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831F88E0: 7C065A2C  dcbt r6, r11
	// 831F88E4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831F88E8: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 831F88EC: 4198FFF0  blt cr6, 0x831f88dc
	if ctx.cr[6].lt {
	pc = 0x831F88DC; continue 'dispatch;
	}
	// 831F88F0: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 831F88F4: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 831F88F8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 831F88FC: C1A67490  lfs f13, 0x7490(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(29840 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F8900: A0C90000  lhz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F8904: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 831F8908: 54C4C63E  rlwinm r4, r6, 0x18, 0x18, 0x1f
	ctx.r[4].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 831F890C: 50C4442E  rlwimi r4, r6, 8, 0x10, 0x17
	ctx.r[4].u64 = (((ctx.r[6].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[4].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8910: 7C840734  extsh r4, r4
	ctx.r[4].s64 = ctx.r[4].s16 as i64;
	// 831F8914: F881FFE0  std r4, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[4].u64 ) };
	// 831F8918: C981FFE0  lfd f12, -0x20(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 831F891C: FD20669C  fcfid f9, f12
	ctx.f[9].f64 = (ctx.f[12].s64 as f64);
	// 831F8920: FD004818  frsp f8, f9
	ctx.f[8].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F8924: ECE80372  fmuls f7, f8, f13
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F8928: D0E90000  stfs f7, 0(r9)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831F892C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 831F8930: 4082FFD0  bne 0x831f8900
	if !ctx.cr[0].eq {
	pc = 0x831F8900; continue 'dispatch;
	}
	// 831F8934: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 831F8938: C1899450  lfs f12, -0x6bb0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831F893C: A12B0002  lhz r9, 2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 831F8940: C1250004  lfs f9, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 831F8944: ED0B002A  fadds f8, f11, f0
	ctx.f[8].f64 = ((ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64;
	// 831F8948: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831F894C: 5527C63E  rlwinm r7, r9, 0x18, 0x18, 0x1f
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 831F8950: 5127442E  rlwimi r7, r9, 8, 0x10, 0x17
	ctx.r[7].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8954: 7CE40734  extsh r4, r7
	ctx.r[4].s64 = ctx.r[7].s16 as i64;
	// 831F8958: F881FFE0  std r4, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[4].u64 ) };
	// 831F895C: C8E1FFE0  lfd f7, -0x20(r1)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 831F8960: FCC03E9C  fcfid f6, f7
	ctx.f[6].f64 = (ctx.f[7].s64 as f64);
	// 831F8964: FCA03018  frsp f5, f6
	ctx.f[5].f64 = (ctx.f[6].f64 as f32) as f64;
	// 831F8968: EC850372  fmuls f4, f5, f13
	ctx.f[4].f64 = (((ctx.f[5].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F896C: D0850004  stfs f4, 4(r5)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 831F8970: EC64482A  fadds f3, f4, f9
	ctx.f[3].f64 = ((ctx.f[4].f64 + ctx.f[9].f64) as f32) as f64;
	// 831F8974: EC480132  fmuls f2, f8, f4
	ctx.f[2].f64 = (((ctx.f[8].f64 * ctx.f[4].f64) as f32) as f64);
	// 831F8978: EC230032  fmuls f1, f3, f0
	ctx.f[1].f64 = (((ctx.f[3].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F897C: ED210332  fmuls f9, f1, f12
	ctx.f[9].f64 = (((ctx.f[1].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F8980: D12A0400  stfs f9, 0x400(r10)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(1024 as u32), tmp.u32 ) };
	// 831F8984: D04A0404  stfs f2, 0x404(r10)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(1028 as u32), tmp.u32 ) };
	// 831F8988: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F898C: 5527C63E  rlwinm r7, r9, 0x18, 0x18, 0x1f
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 831F8990: C0E50000  lfs f7, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 831F8994: 5127442E  rlwimi r7, r9, 8, 0x10, 0x17
	ctx.r[7].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8998: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831F899C: 7CE40734  extsh r4, r7
	ctx.r[4].s64 = ctx.r[7].s16 as i64;
	// 831F89A0: F881FFE8  std r4, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[4].u64 ) };
	// 831F89A4: C8C1FFE8  lfd f6, -0x18(r1)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831F89A8: FCA0369C  fcfid f5, f6
	ctx.f[5].f64 = (ctx.f[6].s64 as f64);
	// 831F89AC: FC802818  frsp f4, f5
	ctx.f[4].f64 = (ctx.f[5].f64 as f32) as f64;
	// 831F89B0: EC640372  fmuls f3, f4, f13
	ctx.f[3].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F89B4: D0650000  stfs f3, 0(r5)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831F89B8: EC43382A  fadds f2, f3, f7
	ctx.f[2].f64 = ((ctx.f[3].f64 + ctx.f[7].f64) as f32) as f64;
	// 831F89BC: EC2800F2  fmuls f1, f8, f3
	ctx.f[1].f64 = (((ctx.f[8].f64 * ctx.f[3].f64) as f32) as f64);
	// 831F89C0: ED220032  fmuls f9, f2, f0
	ctx.f[9].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F89C4: EC0A002A  fadds f0, f10, f0
	ctx.f[0].f64 = ((ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64;
	// 831F89C8: ED090332  fmuls f8, f9, f12
	ctx.f[8].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F89CC: D10A0000  stfs f8, 0(r10)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831F89D0: D02A0004  stfs f1, 4(r10)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 831F89D4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 831F89D8: 4082FF64  bne 0x831f893c
	if !ctx.cr[0].eq {
	pc = 0x831F893C; continue 'dispatch;
	}
	// 831F89DC: 8923000D  lbz r9, 0xd(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F89E0: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F89E4: 5527083E  rotlwi r7, r9, 1
	ctx.r[7].u64 = ((ctx.r[9].u32).rotate_left(1)) as u64;
	// 831F89E8: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F89EC: 7CC85850  subf r6, r8, r11
	ctx.r[6].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 831F89F0: 0CC70000  twi 6, r7, 0
	// 831F89F4: 7D663B96  divwu r11, r6, r7
	ctx.r[11].u32 = ctx.r[6].u32 / ctx.r[7].u32;
	// 831F89F8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 831F89FC: 40980008  bge cr6, 0x831f8a04
	if !ctx.cr[6].lt {
	pc = 0x831F8A04; continue 'dispatch;
	}
	// 831F8A00: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 831F8A04: 81030014  lwz r8, 0x14(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F8A08: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F8A0C: 7CE85050  subf r7, r8, r10
	ctx.r[7].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 831F8A10: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 831F8A14: 54EAF0BE  srwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F8A18: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831F8A1C: 40980008  bge cr6, 0x831f8a24
	if !ctx.cr[6].lt {
	pc = 0x831F8A24; continue 'dispatch;
	}
	// 831F8A20: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831F8A24: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 831F8A28: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 831F8A2C: 3D000000  lis r8, 0
	ctx.r[8].s64 = 0;
	// 831F8A30: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 831F8A34: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 831F8A38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 831F8A3C: 38E00080  li r7, 0x80
	ctx.r[7].s64 = 128;
	// 831F8A40: C00A6AB0  lfs f0, 0x6ab0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(27312 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F8A44: 6108FF7F  ori r8, r8, 0xff7f
	ctx.r[8].u64 = ctx.r[8].u64 | 65407;
	// 831F8A48: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F8A4C: ED8D0032  fmuls f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F8A50: FD60601E  fctiwz f11, f12
	ctx.f[11].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 831F8A54: D961FFE8  stfd f11, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[11].u64 ) };
	// 831F8A58: 8141FFEC  lwz r10, -0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-20 as u32) ) } as u64;
	// 831F8A5C: 2F0A7FFF  cmpwi cr6, r10, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32767, &mut ctx.xer);
	// 831F8A60: 4198000C  blt cr6, 0x831f8a6c
	if ctx.cr[6].lt {
	pc = 0x831F8A6C; continue 'dispatch;
	}
	// 831F8A64: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831F8A68: 48000024  b 0x831f8a8c
	pc = 0x831F8A8C; continue 'dispatch;
	// 831F8A6C: 2F0A8000  cmpwi cr6, r10, -0x8000
	ctx.cr[6].compare_i32(ctx.r[10].s32, -32768, &mut ctx.xer);
	// 831F8A70: 4199000C  bgt cr6, 0x831f8a7c
	if ctx.cr[6].gt {
	pc = 0x831F8A7C; continue 'dispatch;
	}
	// 831F8A74: B0EB0000  sth r7, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 831F8A78: 48000014  b 0x831f8a8c
	pc = 0x831F8A8C; continue 'dispatch;
	// 831F8A7C: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 831F8A80: 5546C63E  rlwinm r6, r10, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 831F8A84: 5146442E  rlwimi r6, r10, 8, 0x10, 0x17
	ctx.r[6].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8A88: B0CB0000  sth r6, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 831F8A8C: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F8A90: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831F8A94: 4082FFB4  bne 0x831f8a48
	if !ctx.cr[0].eq {
	pc = 0x831F8A48; continue 'dispatch;
	}
	// 831F8A98: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831F8A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F8AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831F8AA0 size=740
    let mut pc: u32 = 0x831F8AA0;
    'dispatch: loop {
        match pc {
            0x831F8AA0 => {
    //   block [0x831F8AA0..0x831F8D84)
	// 831F8AA0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 831F8AA4: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F8AA8: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F8AAC: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F8AB0: 39230034  addi r9, r3, 0x34
	ctx.r[9].s64 = ctx.r[3].s64 + 52;
	// 831F8AB4: 81030008  lwz r8, 8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F8AB8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831F8ABC: 88E3000D  lbz r7, 0xd(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F8AC0: 7C8B5050  subf r4, r11, r10
	ctx.r[4].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831F8AC4: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F8AC8: 7D4741D6  mullw r10, r7, r8
	ctx.r[10].s64 = (ctx.r[7].s32 as i64) * (ctx.r[8].s32 as i64);
	// 831F8ACC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F8AD0: 80C30014  lwz r6, 0x14(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F8AD4: 7C840E70  srawi r4, r4, 1
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[4].s32 >> 1) as i64;
	// 831F8AD8: 7CE8F850  subf r7, r8, r31
	ctx.r[7].s64 = ctx.r[31].s64 - ctx.r[8].s64;
	// 831F8ADC: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F8AE0: 7D040194  addze r8, r4
	tmp.s64 = ctx.r[4].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[4].u32);
	ctx.r[8].s64 = tmp.s64;
	// 831F8AE4: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F8AE8: 7D653214  add r11, r5, r6
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 831F8AEC: 7F074000  cmpw cr6, r7, r8
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831F8AF0: 41980008  blt cr6, 0x831f8af8
	if ctx.cr[6].lt {
	pc = 0x831F8AF8; continue 'dispatch;
	}
	// 831F8AF4: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 831F8AF8: 7D0607B4  extsw r6, r8
	ctx.r[6].s64 = ctx.r[8].s32 as i64;
	// 831F8AFC: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F8B00: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831F8B04: 54E81838  slwi r8, r7, 3
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831F8B08: F8C1FFD0  std r6, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.r[6].u64 ) };
	// 831F8B0C: C961FFD0  lfd f11, -0x30(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 831F8B10: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 831F8B14: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 831F8B18: FD205018  frsp f9, f10
	ctx.f[9].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F8B1C: 3888007F  addi r4, r8, 0x7f
	ctx.r[4].s64 = ctx.r[8].s64 + 127;
	// 831F8B20: C1A59524  lfs f13, -0x6adc(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F8B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 831F8B28: 5486C9FE  srwi r6, r4, 7
	ctx.r[6].u32 = ctx.r[4].u32.wrapping_shr(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831F8B2C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 831F8B30: ED6C4824  fdivs f11, f12, f9
	ctx.f[11].f64 = ((ctx.f[12].f64 / ctx.f[9].f64) as f32) as f64;
	// 831F8B34: ED4B0372  fmuls f10, f11, f13
	ctx.f[10].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F8B38: 419A0018  beq cr6, 0x831f8b50
	if ctx.cr[6].eq {
	pc = 0x831F8B50; continue 'dispatch;
	}
	// 831F8B3C: 55053830  slwi r5, r8, 7
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831F8B40: 7C05522C  dcbt r5, r10
	// 831F8B44: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 831F8B48: 7F083040  cmplw cr6, r8, r6
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[6].u32, &mut ctx.xer);
	// 831F8B4C: 4198FFF0  blt cr6, 0x831f8b3c
	if ctx.cr[6].lt {
	pc = 0x831F8B3C; continue 'dispatch;
	}
	// 831F8B50: 3CA0820C  lis r5, -0x7df4
	ctx.r[5].s64 = -2113142784;
	// 831F8B54: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 831F8B58: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 831F8B5C: C1A57490  lfs f13, 0x7490(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(29840 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F8B60: A0A80000  lhz r5, 0(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F8B64: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831F8B68: 54A4C63E  rlwinm r4, r5, 0x18, 0x18, 0x1f
	ctx.r[4].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 831F8B6C: 50A4442E  rlwimi r4, r5, 8, 0x10, 0x17
	ctx.r[4].u64 = (((ctx.r[5].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[4].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8B70: 7C840734  extsh r4, r4
	ctx.r[4].s64 = ctx.r[4].s16 as i64;
	// 831F8B74: F881FFD0  std r4, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.r[4].u64 ) };
	// 831F8B78: C981FFD0  lfd f12, -0x30(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 831F8B7C: FD20669C  fcfid f9, f12
	ctx.f[9].f64 = (ctx.f[12].s64 as f64);
	// 831F8B80: FD004818  frsp f8, f9
	ctx.f[8].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F8B84: ECE80372  fmuls f7, f8, f13
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F8B88: D0E80000  stfs f7, 0(r8)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831F8B8C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 831F8B90: 4082FFD0  bne 0x831f8b60
	if !ctx.cr[0].eq {
	pc = 0x831F8B60; continue 'dispatch;
	}
	// 831F8B94: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 831F8B98: C1889450  lfs f12, -0x6bb0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831F8B9C: A10A0006  lhz r8, 6(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 831F8BA0: C129000C  lfs f9, 0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 831F8BA4: ED0B002A  fadds f8, f11, f0
	ctx.f[8].f64 = ((ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64;
	// 831F8BA8: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 831F8BAC: 5506C63E  rlwinm r6, r8, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 831F8BB0: 5106442E  rlwimi r6, r8, 8, 0x10, 0x17
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8BB4: 7CC40734  extsh r4, r6
	ctx.r[4].s64 = ctx.r[6].s16 as i64;
	// 831F8BB8: F881FFD0  std r4, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.r[4].u64 ) };
	// 831F8BBC: C8E1FFD0  lfd f7, -0x30(r1)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 831F8BC0: FCC03E9C  fcfid f6, f7
	ctx.f[6].f64 = (ctx.f[7].s64 as f64);
	// 831F8BC4: FCA03018  frsp f5, f6
	ctx.f[5].f64 = (ctx.f[6].f64 as f32) as f64;
	// 831F8BC8: EC850372  fmuls f4, f5, f13
	ctx.f[4].f64 = (((ctx.f[5].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F8BCC: D089000C  stfs f4, 0xc(r9)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 831F8BD0: EC64482A  fadds f3, f4, f9
	ctx.f[3].f64 = ((ctx.f[4].f64 + ctx.f[9].f64) as f32) as f64;
	// 831F8BD4: EC480132  fmuls f2, f8, f4
	ctx.f[2].f64 = (((ctx.f[8].f64 * ctx.f[4].f64) as f32) as f64);
	// 831F8BD8: EC230032  fmuls f1, f3, f0
	ctx.f[1].f64 = (((ctx.f[3].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F8BDC: ED210332  fmuls f9, f1, f12
	ctx.f[9].f64 = (((ctx.f[1].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F8BE0: D12B0C00  stfs f9, 0xc00(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3072 as u32), tmp.u32 ) };
	// 831F8BE4: D04B0C04  stfs f2, 0xc04(r11)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3076 as u32), tmp.u32 ) };
	// 831F8BE8: A10A0004  lhz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F8BEC: 5506C63E  rlwinm r6, r8, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 831F8BF0: C0E90008  lfs f7, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 831F8BF4: 5106442E  rlwimi r6, r8, 8, 0x10, 0x17
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8BF8: 7CC40734  extsh r4, r6
	ctx.r[4].s64 = ctx.r[6].s16 as i64;
	// 831F8BFC: F881FFD8  std r4, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.r[4].u64 ) };
	// 831F8C00: C8C1FFD8  lfd f6, -0x28(r1)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 831F8C04: FCA0369C  fcfid f5, f6
	ctx.f[5].f64 = (ctx.f[6].s64 as f64);
	// 831F8C08: FC802818  frsp f4, f5
	ctx.f[4].f64 = (ctx.f[5].f64 as f32) as f64;
	// 831F8C0C: EC640372  fmuls f3, f4, f13
	ctx.f[3].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F8C10: D0690008  stfs f3, 8(r9)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 831F8C14: EC43382A  fadds f2, f3, f7
	ctx.f[2].f64 = ((ctx.f[3].f64 + ctx.f[7].f64) as f32) as f64;
	// 831F8C18: EC2800F2  fmuls f1, f8, f3
	ctx.f[1].f64 = (((ctx.f[8].f64 * ctx.f[3].f64) as f32) as f64);
	// 831F8C1C: ED220032  fmuls f9, f2, f0
	ctx.f[9].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F8C20: ECE90332  fmuls f7, f9, f12
	ctx.f[7].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F8C24: D0EB0800  stfs f7, 0x800(r11)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2048 as u32), tmp.u32 ) };
	// 831F8C28: D02B0804  stfs f1, 0x804(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2052 as u32), tmp.u32 ) };
	// 831F8C2C: A10A0002  lhz r8, 2(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 831F8C30: 5506C63E  rlwinm r6, r8, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 831F8C34: C0C90004  lfs f6, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 831F8C38: 5106442E  rlwimi r6, r8, 8, 0x10, 0x17
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8C3C: 7CC40734  extsh r4, r6
	ctx.r[4].s64 = ctx.r[6].s16 as i64;
	// 831F8C40: F881FFE0  std r4, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[4].u64 ) };
	// 831F8C44: C8A1FFE0  lfd f5, -0x20(r1)
	ctx.f[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 831F8C48: FC802E9C  fcfid f4, f5
	ctx.f[4].f64 = (ctx.f[5].s64 as f64);
	// 831F8C4C: FC602018  frsp f3, f4
	ctx.f[3].f64 = (ctx.f[4].f64 as f32) as f64;
	// 831F8C50: EC430372  fmuls f2, f3, f13
	ctx.f[2].f64 = (((ctx.f[3].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F8C54: D0490004  stfs f2, 4(r9)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 831F8C58: EC22302A  fadds f1, f2, f6
	ctx.f[1].f64 = ((ctx.f[2].f64 + ctx.f[6].f64) as f32) as f64;
	// 831F8C5C: ED2800B2  fmuls f9, f8, f2
	ctx.f[9].f64 = (((ctx.f[8].f64 * ctx.f[2].f64) as f32) as f64);
	// 831F8C60: ECE10032  fmuls f7, f1, f0
	ctx.f[7].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F8C64: ECC70332  fmuls f6, f7, f12
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F8C68: D0CB0400  stfs f6, 0x400(r11)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), tmp.u32 ) };
	// 831F8C6C: D12B0404  stfs f9, 0x404(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1028 as u32), tmp.u32 ) };
	// 831F8C70: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F8C74: 5506C63E  rlwinm r6, r8, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 831F8C78: C0A90000  lfs f5, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 831F8C7C: 5106442E  rlwimi r6, r8, 8, 0x10, 0x17
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8C80: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 831F8C84: 7CC40734  extsh r4, r6
	ctx.r[4].s64 = ctx.r[6].s16 as i64;
	// 831F8C88: F881FFE8  std r4, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[4].u64 ) };
	// 831F8C8C: C881FFE8  lfd f4, -0x18(r1)
	ctx.f[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831F8C90: FC60269C  fcfid f3, f4
	ctx.f[3].f64 = (ctx.f[4].s64 as f64);
	// 831F8C94: FC401818  frsp f2, f3
	ctx.f[2].f64 = (ctx.f[3].f64 as f32) as f64;
	// 831F8C98: EC220372  fmuls f1, f2, f13
	ctx.f[1].f64 = (((ctx.f[2].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F8C9C: D0290000  stfs f1, 0(r9)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831F8CA0: ED21282A  fadds f9, f1, f5
	ctx.f[9].f64 = ((ctx.f[1].f64 + ctx.f[5].f64) as f32) as f64;
	// 831F8CA4: ED080072  fmuls f8, f8, f1
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[1].f64) as f32) as f64);
	// 831F8CA8: ECE90032  fmuls f7, f9, f0
	ctx.f[7].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F8CAC: EC0A002A  fadds f0, f10, f0
	ctx.f[0].f64 = ((ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64;
	// 831F8CB0: ECC70332  fmuls f6, f7, f12
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F8CB4: D0CB0000  stfs f6, 0(r11)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831F8CB8: D10B0004  stfs f8, 4(r11)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 831F8CBC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831F8CC0: 4082FEDC  bne 0x831f8b9c
	if !ctx.cr[0].eq {
	pc = 0x831F8B9C; continue 'dispatch;
	}
	// 831F8CC4: 8903000D  lbz r8, 0xd(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F8CC8: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F8CCC: 5506083E  rotlwi r6, r8, 1
	ctx.r[6].u64 = ((ctx.r[8].u32).rotate_left(1)) as u64;
	// 831F8CD0: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F8CD4: 7CA75050  subf r5, r7, r10
	ctx.r[5].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 831F8CD8: 0CC60000  twi 6, r6, 0
	// 831F8CDC: 7D453396  divwu r10, r5, r6
	ctx.r[10].u32 = ctx.r[5].u32 / ctx.r[6].u32;
	// 831F8CE0: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 831F8CE4: 40980008  bge cr6, 0x831f8cec
	if !ctx.cr[6].lt {
	pc = 0x831F8CEC; continue 'dispatch;
	}
	// 831F8CE8: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 831F8CEC: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F8CF0: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F8CF4: 7CC75850  subf r6, r7, r11
	ctx.r[6].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 831F8CF8: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 831F8CFC: 54CBF0BE  srwi r11, r6, 2
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831F8D00: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831F8D04: 41980008  blt cr6, 0x831f8d0c
	if ctx.cr[6].lt {
	pc = 0x831F8D0C; continue 'dispatch;
	}
	// 831F8D08: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831F8D0C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 831F8D10: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 831F8D14: 3D000000  lis r8, 0
	ctx.r[8].s64 = 0;
	// 831F8D18: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 831F8D1C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 831F8D20: 38E00080  li r7, 0x80
	ctx.r[7].s64 = 128;
	// 831F8D24: 6108FF7F  ori r8, r8, 0xff7f
	ctx.r[8].u64 = ctx.r[8].u64 | 65407;
	// 831F8D28: C00B6AB0  lfs f0, 0x6ab0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27312 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F8D2C: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F8D30: ED8D0032  fmuls f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F8D34: FD60601E  fctiwz f11, f12
	ctx.f[11].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 831F8D38: D961FFE8  stfd f11, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[11].u64 ) };
	// 831F8D3C: 8161FFEC  lwz r11, -0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-20 as u32) ) } as u64;
	// 831F8D40: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 831F8D44: 4198000C  blt cr6, 0x831f8d50
	if ctx.cr[6].lt {
	pc = 0x831F8D50; continue 'dispatch;
	}
	// 831F8D48: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831F8D4C: 48000024  b 0x831f8d70
	pc = 0x831F8D70; continue 'dispatch;
	// 831F8D50: 2F0B8000  cmpwi cr6, r11, -0x8000
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32768, &mut ctx.xer);
	// 831F8D54: 4199000C  bgt cr6, 0x831f8d60
	if ctx.cr[6].gt {
	pc = 0x831F8D60; continue 'dispatch;
	}
	// 831F8D58: B0E90000  sth r7, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 831F8D5C: 48000014  b 0x831f8d70
	pc = 0x831F8D70; continue 'dispatch;
	// 831F8D60: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 831F8D64: 5566C63E  rlwinm r6, r11, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 831F8D68: 5166442E  rlwimi r6, r11, 8, 0x10, 0x17
	ctx.r[6].u64 = (((ctx.r[11].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8D6C: B0C90000  sth r6, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 831F8D70: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831F8D74: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 831F8D78: 4082FFB4  bne 0x831f8d2c
	if !ctx.cr[0].eq {
	pc = 0x831F8D2C; continue 'dispatch;
	}
	// 831F8D7C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831F8D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F8D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831F8D88 size=884
    let mut pc: u32 = 0x831F8D88;
    'dispatch: loop {
        match pc {
            0x831F8D88 => {
    //   block [0x831F8D88..0x831F90FC)
	// 831F8D88: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 831F8D8C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F8D90: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F8D94: 81230018  lwz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F8D98: 39430034  addi r10, r3, 0x34
	ctx.r[10].s64 = ctx.r[3].s64 + 52;
	// 831F8D9C: 81030008  lwz r8, 8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F8DA0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831F8DA4: 88E3000D  lbz r7, 0xd(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F8DA8: 7C8B4850  subf r4, r11, r9
	ctx.r[4].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 831F8DAC: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F8DB0: 7D2741D6  mullw r9, r7, r8
	ctx.r[9].s64 = (ctx.r[7].s32 as i64) * (ctx.r[8].s32 as i64);
	// 831F8DB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F8DB8: 80C30014  lwz r6, 0x14(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F8DBC: 7C840E70  srawi r4, r4, 1
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[4].s32 >> 1) as i64;
	// 831F8DC0: 7CE8F850  subf r7, r8, r31
	ctx.r[7].s64 = ctx.r[31].s64 - ctx.r[8].s64;
	// 831F8DC4: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831F8DC8: 7D040194  addze r8, r4
	tmp.s64 = ctx.r[4].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[4].u32);
	ctx.r[8].s64 = tmp.s64;
	// 831F8DCC: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 831F8DD0: 7D653214  add r11, r5, r6
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 831F8DD4: 7F074000  cmpw cr6, r7, r8
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831F8DD8: 41980008  blt cr6, 0x831f8de0
	if ctx.cr[6].lt {
	pc = 0x831F8DE0; continue 'dispatch;
	}
	// 831F8DDC: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 831F8DE0: 7D0607B4  extsw r6, r8
	ctx.r[6].s64 = ctx.r[8].s32 as i64;
	// 831F8DE4: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F8DE8: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831F8DEC: 54E8083C  slwi r8, r7, 1
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831F8DF0: F8C1FFC0  std r6, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[6].u64 ) };
	// 831F8DF4: C961FFC0  lfd f11, -0x40(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 831F8DF8: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 831F8DFC: 7CA74214  add r5, r7, r8
	ctx.r[5].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 831F8E00: FD205018  frsp f9, f10
	ctx.f[9].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F8E04: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831F8E08: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 831F8E0C: 38C8007F  addi r6, r8, 0x7f
	ctx.r[6].s64 = ctx.r[8].s64 + 127;
	// 831F8E10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 831F8E14: 54C6C9FE  srwi r6, r6, 7
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shr(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831F8E18: C1A49524  lfs f13, -0x6adc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F8E1C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 831F8E20: ED6C4824  fdivs f11, f12, f9
	ctx.f[11].f64 = ((ctx.f[12].f64 / ctx.f[9].f64) as f32) as f64;
	// 831F8E24: ED4B0372  fmuls f10, f11, f13
	ctx.f[10].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F8E28: 419A0018  beq cr6, 0x831f8e40
	if ctx.cr[6].eq {
	pc = 0x831F8E40; continue 'dispatch;
	}
	// 831F8E2C: 55053830  slwi r5, r8, 7
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831F8E30: 7C054A2C  dcbt r5, r9
	// 831F8E34: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 831F8E38: 7F083040  cmplw cr6, r8, r6
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[6].u32, &mut ctx.xer);
	// 831F8E3C: 4198FFF0  blt cr6, 0x831f8e2c
	if ctx.cr[6].lt {
	pc = 0x831F8E2C; continue 'dispatch;
	}
	// 831F8E40: 3CA0820C  lis r5, -0x7df4
	ctx.r[5].s64 = -2113142784;
	// 831F8E44: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 831F8E48: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 831F8E4C: C1A57490  lfs f13, 0x7490(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(29840 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F8E50: A0A80000  lhz r5, 0(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F8E54: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831F8E58: 54A4C63E  rlwinm r4, r5, 0x18, 0x18, 0x1f
	ctx.r[4].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 831F8E5C: 50A4442E  rlwimi r4, r5, 8, 0x10, 0x17
	ctx.r[4].u64 = (((ctx.r[5].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[4].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8E60: 7C840734  extsh r4, r4
	ctx.r[4].s64 = ctx.r[4].s16 as i64;
	// 831F8E64: F881FFC0  std r4, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[4].u64 ) };
	// 831F8E68: C981FFC0  lfd f12, -0x40(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 831F8E6C: FD20669C  fcfid f9, f12
	ctx.f[9].f64 = (ctx.f[12].s64 as f64);
	// 831F8E70: FD004818  frsp f8, f9
	ctx.f[8].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F8E74: ECE80372  fmuls f7, f8, f13
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F8E78: D0E80000  stfs f7, 0(r8)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831F8E7C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 831F8E80: 4082FFD0  bne 0x831f8e50
	if !ctx.cr[0].eq {
	pc = 0x831F8E50; continue 'dispatch;
	}
	// 831F8E84: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 831F8E88: C1889450  lfs f12, -0x6bb0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831F8E8C: A109000A  lhz r8, 0xa(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(10 as u32) ) } as u64;
	// 831F8E90: C12A0014  lfs f9, 0x14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 831F8E94: ED0B002A  fadds f8, f11, f0
	ctx.f[8].f64 = ((ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64;
	// 831F8E98: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 831F8E9C: 5506C63E  rlwinm r6, r8, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 831F8EA0: 5106442E  rlwimi r6, r8, 8, 0x10, 0x17
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8EA4: 7CC40734  extsh r4, r6
	ctx.r[4].s64 = ctx.r[6].s16 as i64;
	// 831F8EA8: F881FFC0  std r4, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[4].u64 ) };
	// 831F8EAC: C8E1FFC0  lfd f7, -0x40(r1)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 831F8EB0: FCC03E9C  fcfid f6, f7
	ctx.f[6].f64 = (ctx.f[7].s64 as f64);
	// 831F8EB4: FCA03018  frsp f5, f6
	ctx.f[5].f64 = (ctx.f[6].f64 as f32) as f64;
	// 831F8EB8: EC850372  fmuls f4, f5, f13
	ctx.f[4].f64 = (((ctx.f[5].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F8EBC: D08A0014  stfs f4, 0x14(r10)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 831F8EC0: EC64482A  fadds f3, f4, f9
	ctx.f[3].f64 = ((ctx.f[4].f64 + ctx.f[9].f64) as f32) as f64;
	// 831F8EC4: EC480132  fmuls f2, f8, f4
	ctx.f[2].f64 = (((ctx.f[8].f64 * ctx.f[4].f64) as f32) as f64);
	// 831F8EC8: EC230032  fmuls f1, f3, f0
	ctx.f[1].f64 = (((ctx.f[3].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F8ECC: ED210332  fmuls f9, f1, f12
	ctx.f[9].f64 = (((ctx.f[1].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F8ED0: D12B1400  stfs f9, 0x1400(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5120 as u32), tmp.u32 ) };
	// 831F8ED4: D04B1404  stfs f2, 0x1404(r11)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5124 as u32), tmp.u32 ) };
	// 831F8ED8: A1090008  lhz r8, 8(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F8EDC: 5506C63E  rlwinm r6, r8, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 831F8EE0: C0EA0010  lfs f7, 0x10(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 831F8EE4: 5106442E  rlwimi r6, r8, 8, 0x10, 0x17
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8EE8: 7CC40734  extsh r4, r6
	ctx.r[4].s64 = ctx.r[6].s16 as i64;
	// 831F8EEC: F881FFC8  std r4, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[4].u64 ) };
	// 831F8EF0: C8C1FFC8  lfd f6, -0x38(r1)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 831F8EF4: FCA0369C  fcfid f5, f6
	ctx.f[5].f64 = (ctx.f[6].s64 as f64);
	// 831F8EF8: FC802818  frsp f4, f5
	ctx.f[4].f64 = (ctx.f[5].f64 as f32) as f64;
	// 831F8EFC: EC640372  fmuls f3, f4, f13
	ctx.f[3].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F8F00: D06A0010  stfs f3, 0x10(r10)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 831F8F04: EC43382A  fadds f2, f3, f7
	ctx.f[2].f64 = ((ctx.f[3].f64 + ctx.f[7].f64) as f32) as f64;
	// 831F8F08: EC2800F2  fmuls f1, f8, f3
	ctx.f[1].f64 = (((ctx.f[8].f64 * ctx.f[3].f64) as f32) as f64);
	// 831F8F0C: ED220032  fmuls f9, f2, f0
	ctx.f[9].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F8F10: ECE90332  fmuls f7, f9, f12
	ctx.f[7].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F8F14: D0EB1000  stfs f7, 0x1000(r11)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4096 as u32), tmp.u32 ) };
	// 831F8F18: D02B1004  stfs f1, 0x1004(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4100 as u32), tmp.u32 ) };
	// 831F8F1C: A1090006  lhz r8, 6(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(6 as u32) ) } as u64;
	// 831F8F20: 5506C63E  rlwinm r6, r8, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 831F8F24: C0CA000C  lfs f6, 0xc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 831F8F28: 5106442E  rlwimi r6, r8, 8, 0x10, 0x17
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8F2C: 7CC40734  extsh r4, r6
	ctx.r[4].s64 = ctx.r[6].s16 as i64;
	// 831F8F30: F881FFD0  std r4, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.r[4].u64 ) };
	// 831F8F34: C8A1FFD0  lfd f5, -0x30(r1)
	ctx.f[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 831F8F38: FC802E9C  fcfid f4, f5
	ctx.f[4].f64 = (ctx.f[5].s64 as f64);
	// 831F8F3C: FC602018  frsp f3, f4
	ctx.f[3].f64 = (ctx.f[4].f64 as f32) as f64;
	// 831F8F40: EC430372  fmuls f2, f3, f13
	ctx.f[2].f64 = (((ctx.f[3].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F8F44: D04A000C  stfs f2, 0xc(r10)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 831F8F48: EC22302A  fadds f1, f2, f6
	ctx.f[1].f64 = ((ctx.f[2].f64 + ctx.f[6].f64) as f32) as f64;
	// 831F8F4C: ED2800B2  fmuls f9, f8, f2
	ctx.f[9].f64 = (((ctx.f[8].f64 * ctx.f[2].f64) as f32) as f64);
	// 831F8F50: ECE10032  fmuls f7, f1, f0
	ctx.f[7].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F8F54: ECC70332  fmuls f6, f7, f12
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F8F58: D0CB0C00  stfs f6, 0xc00(r11)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3072 as u32), tmp.u32 ) };
	// 831F8F5C: D12B0C04  stfs f9, 0xc04(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3076 as u32), tmp.u32 ) };
	// 831F8F60: A1090004  lhz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F8F64: 5506C63E  rlwinm r6, r8, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 831F8F68: C0AA0008  lfs f5, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 831F8F6C: 5106442E  rlwimi r6, r8, 8, 0x10, 0x17
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8F70: 7CC40734  extsh r4, r6
	ctx.r[4].s64 = ctx.r[6].s16 as i64;
	// 831F8F74: F881FFD8  std r4, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.r[4].u64 ) };
	// 831F8F78: C881FFD8  lfd f4, -0x28(r1)
	ctx.f[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 831F8F7C: FC60269C  fcfid f3, f4
	ctx.f[3].f64 = (ctx.f[4].s64 as f64);
	// 831F8F80: FC401818  frsp f2, f3
	ctx.f[2].f64 = (ctx.f[3].f64 as f32) as f64;
	// 831F8F84: EC220372  fmuls f1, f2, f13
	ctx.f[1].f64 = (((ctx.f[2].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F8F88: D02A0008  stfs f1, 8(r10)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 831F8F8C: ED21282A  fadds f9, f1, f5
	ctx.f[9].f64 = ((ctx.f[1].f64 + ctx.f[5].f64) as f32) as f64;
	// 831F8F90: ECE80072  fmuls f7, f8, f1
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[1].f64) as f32) as f64);
	// 831F8F94: ECC90032  fmuls f6, f9, f0
	ctx.f[6].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F8F98: ECA60332  fmuls f5, f6, f12
	ctx.f[5].f64 = (((ctx.f[6].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F8F9C: D0AB0800  stfs f5, 0x800(r11)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2048 as u32), tmp.u32 ) };
	// 831F8FA0: D0EB0804  stfs f7, 0x804(r11)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2052 as u32), tmp.u32 ) };
	// 831F8FA4: A1090002  lhz r8, 2(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(2 as u32) ) } as u64;
	// 831F8FA8: 5506C63E  rlwinm r6, r8, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 831F8FAC: C08A0004  lfs f4, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 831F8FB0: 5106442E  rlwimi r6, r8, 8, 0x10, 0x17
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8FB4: 7CC40734  extsh r4, r6
	ctx.r[4].s64 = ctx.r[6].s16 as i64;
	// 831F8FB8: F881FFE0  std r4, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[4].u64 ) };
	// 831F8FBC: C861FFE0  lfd f3, -0x20(r1)
	ctx.f[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 831F8FC0: FC401E9C  fcfid f2, f3
	ctx.f[2].f64 = (ctx.f[3].s64 as f64);
	// 831F8FC4: FC201018  frsp f1, f2
	ctx.f[1].f64 = (ctx.f[2].f64 as f32) as f64;
	// 831F8FC8: ED210372  fmuls f9, f1, f13
	ctx.f[9].f64 = (((ctx.f[1].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F8FCC: D12A0004  stfs f9, 4(r10)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 831F8FD0: ECE9202A  fadds f7, f9, f4
	ctx.f[7].f64 = ((ctx.f[9].f64 + ctx.f[4].f64) as f32) as f64;
	// 831F8FD4: ECC80272  fmuls f6, f8, f9
	ctx.f[6].f64 = (((ctx.f[8].f64 * ctx.f[9].f64) as f32) as f64);
	// 831F8FD8: ECA70032  fmuls f5, f7, f0
	ctx.f[5].f64 = (((ctx.f[7].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F8FDC: EC850332  fmuls f4, f5, f12
	ctx.f[4].f64 = (((ctx.f[5].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F8FE0: D08B0400  stfs f4, 0x400(r11)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), tmp.u32 ) };
	// 831F8FE4: D0CB0404  stfs f6, 0x404(r11)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1028 as u32), tmp.u32 ) };
	// 831F8FE8: C06A0000  lfs f3, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 831F8FEC: A1090000  lhz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F8FF0: 5506C63E  rlwinm r6, r8, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 831F8FF4: 5106442E  rlwimi r6, r8, 8, 0x10, 0x17
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F8FF8: 3929000C  addi r9, r9, 0xc
	ctx.r[9].s64 = ctx.r[9].s64 + 12;
	// 831F8FFC: 7CC40734  extsh r4, r6
	ctx.r[4].s64 = ctx.r[6].s16 as i64;
	// 831F9000: F881FFE8  std r4, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[4].u64 ) };
	// 831F9004: C841FFE8  lfd f2, -0x18(r1)
	ctx.f[2].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831F9008: FC20169C  fcfid f1, f2
	ctx.f[1].f64 = (ctx.f[2].s64 as f64);
	// 831F900C: FD200818  frsp f9, f1
	ctx.f[9].f64 = (ctx.f[1].f64 as f32) as f64;
	// 831F9010: ECE90372  fmuls f7, f9, f13
	ctx.f[7].f64 = (((ctx.f[9].f64 * ctx.f[13].f64) as f32) as f64);
	// 831F9014: D0EA0000  stfs f7, 0(r10)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831F9018: ECC7182A  fadds f6, f7, f3
	ctx.f[6].f64 = ((ctx.f[7].f64 + ctx.f[3].f64) as f32) as f64;
	// 831F901C: ECA801F2  fmuls f5, f8, f7
	ctx.f[5].f64 = (((ctx.f[8].f64 * ctx.f[7].f64) as f32) as f64);
	// 831F9020: EC860032  fmuls f4, f6, f0
	ctx.f[4].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F9024: EC0A002A  fadds f0, f10, f0
	ctx.f[0].f64 = ((ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64;
	// 831F9028: EC640332  fmuls f3, f4, f12
	ctx.f[3].f64 = (((ctx.f[4].f64 * ctx.f[12].f64) as f32) as f64);
	// 831F902C: D06B0000  stfs f3, 0(r11)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831F9030: D0AB0004  stfs f5, 4(r11)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 831F9034: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831F9038: 4082FE54  bne 0x831f8e8c
	if !ctx.cr[0].eq {
	pc = 0x831F8E8C; continue 'dispatch;
	}
	// 831F903C: 8903000D  lbz r8, 0xd(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F9040: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F9044: 5506083E  rotlwi r6, r8, 1
	ctx.r[6].u64 = ((ctx.r[8].u32).rotate_left(1)) as u64;
	// 831F9048: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F904C: 7CA74850  subf r5, r7, r9
	ctx.r[5].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	// 831F9050: 0CC60000  twi 6, r6, 0
	// 831F9054: 7D253396  divwu r9, r5, r6
	ctx.r[9].u32 = ctx.r[5].u32 / ctx.r[6].u32;
	// 831F9058: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 831F905C: 40980008  bge cr6, 0x831f9064
	if !ctx.cr[6].lt {
	pc = 0x831F9064; continue 'dispatch;
	}
	// 831F9060: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 831F9064: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F9068: 81230018  lwz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F906C: 7CC75850  subf r6, r7, r11
	ctx.r[6].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 831F9070: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 831F9074: 54CBF0BE  srwi r11, r6, 2
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831F9078: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 831F907C: 41980008  blt cr6, 0x831f9084
	if ctx.cr[6].lt {
	pc = 0x831F9084; continue 'dispatch;
	}
	// 831F9080: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 831F9084: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 831F9088: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 831F908C: 3D000000  lis r8, 0
	ctx.r[8].s64 = 0;
	// 831F9090: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 831F9094: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 831F9098: 38E00080  li r7, 0x80
	ctx.r[7].s64 = 128;
	// 831F909C: 6108FF7F  ori r8, r8, 0xff7f
	ctx.r[8].u64 = ctx.r[8].u64 | 65407;
	// 831F90A0: C00B6AB0  lfs f0, 0x6ab0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27312 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F90A4: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F90A8: ED8D0032  fmuls f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 831F90AC: FD60601E  fctiwz f11, f12
	ctx.f[11].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 831F90B0: D961FFE8  stfd f11, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[11].u64 ) };
	// 831F90B4: 8161FFEC  lwz r11, -0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-20 as u32) ) } as u64;
	// 831F90B8: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 831F90BC: 4198000C  blt cr6, 0x831f90c8
	if ctx.cr[6].lt {
	pc = 0x831F90C8; continue 'dispatch;
	}
	// 831F90C0: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831F90C4: 48000024  b 0x831f90e8
	pc = 0x831F90E8; continue 'dispatch;
	// 831F90C8: 2F0B8000  cmpwi cr6, r11, -0x8000
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32768, &mut ctx.xer);
	// 831F90CC: 4199000C  bgt cr6, 0x831f90d8
	if ctx.cr[6].gt {
	pc = 0x831F90D8; continue 'dispatch;
	}
	// 831F90D0: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 831F90D4: 48000014  b 0x831f90e8
	pc = 0x831F90E8; continue 'dispatch;
	// 831F90D8: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 831F90DC: 5566C63E  rlwinm r6, r11, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 831F90E0: 5166442E  rlwimi r6, r11, 8, 0x10, 0x17
	ctx.r[6].u64 = (((ctx.r[11].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF00FF);
	// 831F90E4: B0CA0000  sth r6, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 831F90E8: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F90EC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831F90F0: 4082FFB4  bne 0x831f90a4
	if !ctx.cr[0].eq {
	pc = 0x831F90A4; continue 'dispatch;
	}
	// 831F90F4: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831F90F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F9100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F9100 size=216
    let mut pc: u32 = 0x831F9100;
    'dispatch: loop {
        match pc {
            0x831F9100 => {
    //   block [0x831F9100..0x831F91D8)
	// 831F9100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F9104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F9108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F910C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F9110: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F9114: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F9118: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F911C: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F9120: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9124: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F9128: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F912C: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F9130: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9134: 54CA07BE  clrlwi r10, r6, 0x1e
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x00000003u64;
	// 831F9138: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F913C: 7D2321D6  mullw r9, r3, r4
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F9140: 81070004  lwz r8, 4(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F9144: 7D432B78  or r3, r10, r5
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 831F9148: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F914C: 5469073E  clrlwi r9, r3, 0x1c
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 831F9150: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9154: 7C844050  subf r4, r4, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 831F9158: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F915C: 409A0064  bne cr6, 0x831f91c0
	if !ctx.cr[6].eq {
	pc = 0x831F91C0; continue 'dispatch;
	}
	// 831F9160: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F9164: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9168: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F916C: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F9170: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F9174: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9178: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F917C: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9180: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F9184: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F9188: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F918C: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F9190: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F9194: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F9198: 40990028  ble cr6, 0x831f91c0
	if !ctx.cr[6].gt {
	pc = 0x831F91C0; continue 'dispatch;
	}
	// 831F919C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F91A0: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F91A4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F91A8: 41980018  blt cr6, 0x831f91c0
	if ctx.cr[6].lt {
	pc = 0x831F91C0; continue 'dispatch;
	}
	// 831F91AC: 4BFF33CD  bl 0x831ec578
	ctx.lr = 0x831F91B0;
	sub_831EC578(ctx, base);
	// 831F91B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F91B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F91B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F91BC: 4E800020  blr
	return;
	// 831F91C0: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F91C4: 4BFF6435  bl 0x831ef5f8
	ctx.lr = 0x831F91C8;
	sub_831EF5F8(ctx, base);
	// 831F91C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F91CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F91D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F91D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F91D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F91D8 size=216
    let mut pc: u32 = 0x831F91D8;
    'dispatch: loop {
        match pc {
            0x831F91D8 => {
    //   block [0x831F91D8..0x831F92B0)
	// 831F91D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F91DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F91E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F91E4: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F91E8: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F91EC: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F91F0: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F91F4: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F91F8: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F91FC: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F9200: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F9204: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F9208: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F920C: 54CA07BE  clrlwi r10, r6, 0x1e
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x00000003u64;
	// 831F9210: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F9214: 7D2321D6  mullw r9, r3, r4
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F9218: 81070004  lwz r8, 4(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F921C: 7D432B78  or r3, r10, r5
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 831F9220: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9224: 5469073E  clrlwi r9, r3, 0x1c
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 831F9228: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F922C: 7C844050  subf r4, r4, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 831F9230: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F9234: 409A0064  bne cr6, 0x831f9298
	if !ctx.cr[6].eq {
	pc = 0x831F9298; continue 'dispatch;
	}
	// 831F9238: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F923C: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9240: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F9244: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F9248: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F924C: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9250: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F9254: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9258: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F925C: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F9260: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F9264: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F9268: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F926C: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F9270: 40990028  ble cr6, 0x831f9298
	if !ctx.cr[6].gt {
	pc = 0x831F9298; continue 'dispatch;
	}
	// 831F9274: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F9278: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F927C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F9280: 41980018  blt cr6, 0x831f9298
	if ctx.cr[6].lt {
	pc = 0x831F9298; continue 'dispatch;
	}
	// 831F9284: 4BFF34F5  bl 0x831ec778
	ctx.lr = 0x831F9288;
	sub_831EC778(ctx, base);
	// 831F9288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F928C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9294: 4E800020  blr
	return;
	// 831F9298: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F929C: 4BFF663D  bl 0x831ef8d8
	ctx.lr = 0x831F92A0;
	sub_831EF8D8(ctx, base);
	// 831F92A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F92A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F92A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F92AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F92B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F92B0 size=216
    let mut pc: u32 = 0x831F92B0;
    'dispatch: loop {
        match pc {
            0x831F92B0 => {
    //   block [0x831F92B0..0x831F9388)
	// 831F92B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F92B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F92B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F92BC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F92C0: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F92C4: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F92C8: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F92CC: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F92D0: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F92D4: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F92D8: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F92DC: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F92E0: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F92E4: 54CA07BE  clrlwi r10, r6, 0x1e
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x00000003u64;
	// 831F92E8: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F92EC: 7D2321D6  mullw r9, r3, r4
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F92F0: 81070004  lwz r8, 4(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F92F4: 7D432B78  or r3, r10, r5
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 831F92F8: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F92FC: 5469073E  clrlwi r9, r3, 0x1c
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 831F9300: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9304: 7C844050  subf r4, r4, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 831F9308: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F930C: 409A0064  bne cr6, 0x831f9370
	if !ctx.cr[6].eq {
	pc = 0x831F9370; continue 'dispatch;
	}
	// 831F9310: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F9314: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9318: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F931C: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F9320: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F9324: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9328: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F932C: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9330: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F9334: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F9338: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F933C: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F9340: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F9344: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F9348: 40990028  ble cr6, 0x831f9370
	if !ctx.cr[6].gt {
	pc = 0x831F9370; continue 'dispatch;
	}
	// 831F934C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F9350: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9354: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F9358: 41980018  blt cr6, 0x831f9370
	if ctx.cr[6].lt {
	pc = 0x831F9370; continue 'dispatch;
	}
	// 831F935C: 4BFF364D  bl 0x831ec9a8
	ctx.lr = 0x831F9360;
	sub_831EC9A8(ctx, base);
	// 831F9360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F936C: 4E800020  blr
	return;
	// 831F9370: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F9374: 4BFF688D  bl 0x831efc00
	ctx.lr = 0x831F9378;
	sub_831EFC00(ctx, base);
	// 831F9378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F937C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F9388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F9388 size=216
    let mut pc: u32 = 0x831F9388;
    'dispatch: loop {
        match pc {
            0x831F9388 => {
    //   block [0x831F9388..0x831F9460)
	// 831F9388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F938C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F9390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F9394: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F9398: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F939C: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F93A0: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F93A4: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F93A8: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F93AC: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F93B0: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F93B4: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F93B8: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F93BC: 54CA07BE  clrlwi r10, r6, 0x1e
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x00000003u64;
	// 831F93C0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F93C4: 7D2321D6  mullw r9, r3, r4
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F93C8: 81070004  lwz r8, 4(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F93CC: 7D432B78  or r3, r10, r5
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 831F93D0: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F93D4: 5469073E  clrlwi r9, r3, 0x1c
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 831F93D8: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F93DC: 7C844050  subf r4, r4, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 831F93E0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F93E4: 409A0064  bne cr6, 0x831f9448
	if !ctx.cr[6].eq {
	pc = 0x831F9448; continue 'dispatch;
	}
	// 831F93E8: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F93EC: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F93F0: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F93F4: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F93F8: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F93FC: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9400: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F9404: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9408: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F940C: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F9410: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F9414: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F9418: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F941C: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F9420: 40990028  ble cr6, 0x831f9448
	if !ctx.cr[6].gt {
	pc = 0x831F9448; continue 'dispatch;
	}
	// 831F9424: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F9428: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F942C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F9430: 41980018  blt cr6, 0x831f9448
	if ctx.cr[6].lt {
	pc = 0x831F9448; continue 'dispatch;
	}
	// 831F9434: 4BFF380D  bl 0x831ecc40
	ctx.lr = 0x831F9438;
	sub_831ECC40(ctx, base);
	// 831F9438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F943C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9444: 4E800020  blr
	return;
	// 831F9448: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F944C: 4BFF6BAD  bl 0x831efff8
	ctx.lr = 0x831F9450;
	sub_831EFFF8(ctx, base);
	// 831F9450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F945C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F9460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F9460 size=216
    let mut pc: u32 = 0x831F9460;
    'dispatch: loop {
        match pc {
            0x831F9460 => {
    //   block [0x831F9460..0x831F9538)
	// 831F9460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F9464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F9468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F946C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F9470: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F9474: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F9478: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F947C: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F9480: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9484: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F9488: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F948C: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F9490: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9494: 54CA077E  clrlwi r10, r6, 0x1d
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x00000007u64;
	// 831F9498: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F949C: 7D2321D6  mullw r9, r3, r4
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F94A0: 81070004  lwz r8, 4(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F94A4: 7D432B78  or r3, r10, r5
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 831F94A8: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F94AC: 5469073E  clrlwi r9, r3, 0x1c
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 831F94B0: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F94B4: 7C844050  subf r4, r4, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 831F94B8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F94BC: 409A0064  bne cr6, 0x831f9520
	if !ctx.cr[6].eq {
	pc = 0x831F9520; continue 'dispatch;
	}
	// 831F94C0: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F94C4: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F94C8: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F94CC: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F94D0: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F94D4: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F94D8: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F94DC: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F94E0: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F94E4: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F94E8: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F94EC: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F94F0: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F94F4: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F94F8: 40990028  ble cr6, 0x831f9520
	if !ctx.cr[6].gt {
	pc = 0x831F9520; continue 'dispatch;
	}
	// 831F94FC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F9500: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9504: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F9508: 41980018  blt cr6, 0x831f9520
	if ctx.cr[6].lt {
	pc = 0x831F9520; continue 'dispatch;
	}
	// 831F950C: 4BFF2135  bl 0x831eb640
	ctx.lr = 0x831F9510;
	sub_831EB640(ctx, base);
	// 831F9510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F951C: 4E800020  blr
	return;
	// 831F9520: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F9524: 4BFF737D  bl 0x831f08a0
	ctx.lr = 0x831F9528;
	sub_831F08A0(ctx, base);
	// 831F9528: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F952C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F9538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F9538 size=216
    let mut pc: u32 = 0x831F9538;
    'dispatch: loop {
        match pc {
            0x831F9538 => {
    //   block [0x831F9538..0x831F9610)
	// 831F9538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F953C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F9540: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F9544: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F9548: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F954C: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F9550: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F9554: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F9558: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F955C: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F9560: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F9564: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F9568: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F956C: 54CA077E  clrlwi r10, r6, 0x1d
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x00000007u64;
	// 831F9570: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F9574: 7D2321D6  mullw r9, r3, r4
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F9578: 81070004  lwz r8, 4(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F957C: 7D432B78  or r3, r10, r5
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 831F9580: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9584: 5469073E  clrlwi r9, r3, 0x1c
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 831F9588: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F958C: 7C844050  subf r4, r4, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 831F9590: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F9594: 409A0064  bne cr6, 0x831f95f8
	if !ctx.cr[6].eq {
	pc = 0x831F95F8; continue 'dispatch;
	}
	// 831F9598: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F959C: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F95A0: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F95A4: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F95A8: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F95AC: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F95B0: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F95B4: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F95B8: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F95BC: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F95C0: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F95C4: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F95C8: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F95CC: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F95D0: 40990028  ble cr6, 0x831f95f8
	if !ctx.cr[6].gt {
	pc = 0x831F95F8; continue 'dispatch;
	}
	// 831F95D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F95D8: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F95DC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F95E0: 41980018  blt cr6, 0x831f95f8
	if ctx.cr[6].lt {
	pc = 0x831F95F8; continue 'dispatch;
	}
	// 831F95E4: 4BFF22C5  bl 0x831eb8a8
	ctx.lr = 0x831F95E8;
	sub_831EB8A8(ctx, base);
	// 831F95E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F95EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F95F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F95F4: 4E800020  blr
	return;
	// 831F95F8: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F95FC: 4BFF75BD  bl 0x831f0bb8
	ctx.lr = 0x831F9600;
	sub_831F0BB8(ctx, base);
	// 831F9600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F960C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F9610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F9610 size=216
    let mut pc: u32 = 0x831F9610;
    'dispatch: loop {
        match pc {
            0x831F9610 => {
    //   block [0x831F9610..0x831F96E8)
	// 831F9610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F9614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F9618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F961C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F9620: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F9624: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F9628: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F962C: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F9630: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9634: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F9638: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F963C: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F9640: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9644: 54CA077E  clrlwi r10, r6, 0x1d
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x00000007u64;
	// 831F9648: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F964C: 7D2321D6  mullw r9, r3, r4
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F9650: 81070004  lwz r8, 4(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F9654: 7D432B78  or r3, r10, r5
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 831F9658: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F965C: 5469073E  clrlwi r9, r3, 0x1c
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 831F9660: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9664: 7C844050  subf r4, r4, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 831F9668: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F966C: 409A0064  bne cr6, 0x831f96d0
	if !ctx.cr[6].eq {
	pc = 0x831F96D0; continue 'dispatch;
	}
	// 831F9670: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F9674: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9678: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F967C: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F9680: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F9684: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9688: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F968C: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9690: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F9694: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F9698: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F969C: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F96A0: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F96A4: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F96A8: 40990028  ble cr6, 0x831f96d0
	if !ctx.cr[6].gt {
	pc = 0x831F96D0; continue 'dispatch;
	}
	// 831F96AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F96B0: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F96B4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F96B8: 41980018  blt cr6, 0x831f96d0
	if ctx.cr[6].lt {
	pc = 0x831F96D0; continue 'dispatch;
	}
	// 831F96BC: 4BFF24BD  bl 0x831ebb78
	ctx.lr = 0x831F96C0;
	sub_831EBB78(ctx, base);
	// 831F96C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F96C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F96C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F96CC: 4E800020  blr
	return;
	// 831F96D0: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F96D4: 4BFF787D  bl 0x831f0f50
	ctx.lr = 0x831F96D8;
	sub_831F0F50(ctx, base);
	// 831F96D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F96DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F96E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F96E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F96E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F96E8 size=216
    let mut pc: u32 = 0x831F96E8;
    'dispatch: loop {
        match pc {
            0x831F96E8 => {
    //   block [0x831F96E8..0x831F97C0)
	// 831F96E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F96EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F96F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F96F4: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F96F8: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F96FC: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F9700: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F9704: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F9708: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F970C: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F9710: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F9714: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F9718: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F971C: 54CA077E  clrlwi r10, r6, 0x1d
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x00000007u64;
	// 831F9720: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F9724: 7D2321D6  mullw r9, r3, r4
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F9728: 81070004  lwz r8, 4(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F972C: 7D432B78  or r3, r10, r5
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 831F9730: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9734: 5469073E  clrlwi r9, r3, 0x1c
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 831F9738: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F973C: 7C844050  subf r4, r4, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 831F9740: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F9744: 409A0064  bne cr6, 0x831f97a8
	if !ctx.cr[6].eq {
	pc = 0x831F97A8; continue 'dispatch;
	}
	// 831F9748: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F974C: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9750: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F9754: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F9758: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F975C: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9760: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F9764: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9768: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F976C: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F9770: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F9774: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F9778: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F977C: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F9780: 40990028  ble cr6, 0x831f97a8
	if !ctx.cr[6].gt {
	pc = 0x831F97A8; continue 'dispatch;
	}
	// 831F9784: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F9788: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F978C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F9790: 41980018  blt cr6, 0x831f97a8
	if ctx.cr[6].lt {
	pc = 0x831F97A8; continue 'dispatch;
	}
	// 831F9794: 4BFF2785  bl 0x831ebf18
	ctx.lr = 0x831F9798;
	sub_831EBF18(ctx, base);
	// 831F9798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F979C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F97A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F97A4: 4E800020  blr
	return;
	// 831F97A8: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F97AC: 4BFF7C6D  bl 0x831f1418
	ctx.lr = 0x831F97B0;
	sub_831F1418(ctx, base);
	// 831F97B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F97B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F97B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F97BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F97C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F97C0 size=208
    let mut pc: u32 = 0x831F97C0;
    'dispatch: loop {
        match pc {
            0x831F97C0 => {
    //   block [0x831F97C0..0x831F9890)
	// 831F97C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F97C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F97C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F97CC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F97D0: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F97D4: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F97D8: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F97DC: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F97E0: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F97E4: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F97E8: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F97EC: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F97F0: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F97F4: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F97F8: 7D4321D6  mullw r10, r3, r4
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F97FC: 81270004  lwz r9, 4(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F9800: 7CC82B78  or r8, r6, r5
	ctx.r[8].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 831F9804: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9808: 550B073E  clrlwi r11, r8, 0x1c
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000000Fu64;
	// 831F980C: 7C844850  subf r4, r4, r9
	ctx.r[4].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 831F9810: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831F9814: 409A0064  bne cr6, 0x831f9878
	if !ctx.cr[6].eq {
	pc = 0x831F9878; continue 'dispatch;
	}
	// 831F9818: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F981C: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9820: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F9824: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F9828: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F982C: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9830: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F9834: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9838: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F983C: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F9840: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F9844: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F9848: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F984C: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F9850: 40990028  ble cr6, 0x831f9878
	if !ctx.cr[6].gt {
	pc = 0x831F9878; continue 'dispatch;
	}
	// 831F9854: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F9858: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F985C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F9860: 41980018  blt cr6, 0x831f9878
	if ctx.cr[6].lt {
	pc = 0x831F9878; continue 'dispatch;
	}
	// 831F9864: 4BFF36ED  bl 0x831ecf50
	ctx.lr = 0x831F9868;
	sub_831ECF50(ctx, base);
	// 831F9868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F986C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9874: 4E800020  blr
	return;
	// 831F9878: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F987C: 4BFF856D  bl 0x831f1de8
	ctx.lr = 0x831F9880;
	sub_831F1DE8(ctx, base);
	// 831F9880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F988C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F9890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F9890 size=208
    let mut pc: u32 = 0x831F9890;
    'dispatch: loop {
        match pc {
            0x831F9890 => {
    //   block [0x831F9890..0x831F9960)
	// 831F9890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F9894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F9898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F989C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F98A0: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F98A4: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F98A8: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F98AC: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F98B0: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F98B4: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F98B8: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F98BC: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F98C0: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F98C4: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F98C8: 7D4321D6  mullw r10, r3, r4
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F98CC: 81270004  lwz r9, 4(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F98D0: 7CC82B78  or r8, r6, r5
	ctx.r[8].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 831F98D4: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F98D8: 550B073E  clrlwi r11, r8, 0x1c
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000000Fu64;
	// 831F98DC: 7C844850  subf r4, r4, r9
	ctx.r[4].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 831F98E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831F98E4: 409A0064  bne cr6, 0x831f9948
	if !ctx.cr[6].eq {
	pc = 0x831F9948; continue 'dispatch;
	}
	// 831F98E8: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F98EC: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F98F0: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F98F4: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F98F8: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F98FC: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9900: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F9904: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9908: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F990C: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F9910: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F9914: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F9918: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F991C: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F9920: 40990028  ble cr6, 0x831f9948
	if !ctx.cr[6].gt {
	pc = 0x831F9948; continue 'dispatch;
	}
	// 831F9924: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F9928: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F992C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F9930: 41980018  blt cr6, 0x831f9948
	if ctx.cr[6].lt {
	pc = 0x831F9948; continue 'dispatch;
	}
	// 831F9934: 4BFF3935  bl 0x831ed268
	ctx.lr = 0x831F9938;
	sub_831ED268(ctx, base);
	// 831F9938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F993C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9944: 4E800020  blr
	return;
	// 831F9948: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F994C: 4BFF87D5  bl 0x831f2120
	ctx.lr = 0x831F9950;
	sub_831F2120(ctx, base);
	// 831F9950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F995C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F9960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F9960 size=208
    let mut pc: u32 = 0x831F9960;
    'dispatch: loop {
        match pc {
            0x831F9960 => {
    //   block [0x831F9960..0x831F9A30)
	// 831F9960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F9964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F9968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F996C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F9970: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F9974: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F9978: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F997C: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F9980: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9984: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F9988: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F998C: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9990: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F9994: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F9998: 7D4321D6  mullw r10, r3, r4
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F999C: 81270004  lwz r9, 4(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F99A0: 7CC82B78  or r8, r6, r5
	ctx.r[8].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 831F99A4: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F99A8: 550B073E  clrlwi r11, r8, 0x1c
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000000Fu64;
	// 831F99AC: 7C844850  subf r4, r4, r9
	ctx.r[4].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 831F99B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831F99B4: 409A0064  bne cr6, 0x831f9a18
	if !ctx.cr[6].eq {
	pc = 0x831F9A18; continue 'dispatch;
	}
	// 831F99B8: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F99BC: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F99C0: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F99C4: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F99C8: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F99CC: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F99D0: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F99D4: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F99D8: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F99DC: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F99E0: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F99E4: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F99E8: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F99EC: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F99F0: 40990028  ble cr6, 0x831f9a18
	if !ctx.cr[6].gt {
	pc = 0x831F9A18; continue 'dispatch;
	}
	// 831F99F4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F99F8: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F99FC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F9A00: 41980018  blt cr6, 0x831f9a18
	if ctx.cr[6].lt {
	pc = 0x831F9A18; continue 'dispatch;
	}
	// 831F9A04: 4BFF3C55  bl 0x831ed658
	ctx.lr = 0x831F9A08;
	sub_831ED658(ctx, base);
	// 831F9A08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9A14: 4E800020  blr
	return;
	// 831F9A18: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F9A1C: 4BFF8ADD  bl 0x831f24f8
	ctx.lr = 0x831F9A20;
	sub_831F24F8(ctx, base);
	// 831F9A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F9A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F9A30 size=208
    let mut pc: u32 = 0x831F9A30;
    'dispatch: loop {
        match pc {
            0x831F9A30 => {
    //   block [0x831F9A30..0x831F9B00)
	// 831F9A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F9A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F9A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F9A3C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F9A40: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F9A44: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F9A48: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F9A4C: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F9A50: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9A54: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F9A58: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F9A5C: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9A60: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F9A64: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F9A68: 7D4321D6  mullw r10, r3, r4
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F9A6C: 81270004  lwz r9, 4(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F9A70: 7CC82B78  or r8, r6, r5
	ctx.r[8].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 831F9A74: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9A78: 550B073E  clrlwi r11, r8, 0x1c
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000000Fu64;
	// 831F9A7C: 7C844850  subf r4, r4, r9
	ctx.r[4].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 831F9A80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831F9A84: 409A0064  bne cr6, 0x831f9ae8
	if !ctx.cr[6].eq {
	pc = 0x831F9AE8; continue 'dispatch;
	}
	// 831F9A88: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F9A8C: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9A90: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F9A94: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F9A98: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F9A9C: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9AA0: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F9AA4: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9AA8: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F9AAC: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F9AB0: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F9AB4: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F9AB8: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F9ABC: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F9AC0: 40990028  ble cr6, 0x831f9ae8
	if !ctx.cr[6].gt {
	pc = 0x831F9AE8; continue 'dispatch;
	}
	// 831F9AC4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F9AC8: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9ACC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F9AD0: 41980018  blt cr6, 0x831f9ae8
	if ctx.cr[6].lt {
	pc = 0x831F9AE8; continue 'dispatch;
	}
	// 831F9AD4: 4BFF4175  bl 0x831edc48
	ctx.lr = 0x831F9AD8;
	sub_831EDC48(ctx, base);
	// 831F9AD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9AE4: 4E800020  blr
	return;
	// 831F9AE8: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F9AEC: 4BFF8F2D  bl 0x831f2a18
	ctx.lr = 0x831F9AF0;
	sub_831F2A18(ctx, base);
	// 831F9AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F9B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F9B00 size=216
    let mut pc: u32 = 0x831F9B00;
    'dispatch: loop {
        match pc {
            0x831F9B00 => {
    //   block [0x831F9B00..0x831F9BD8)
	// 831F9B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F9B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F9B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F9B0C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F9B10: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F9B14: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F9B18: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F9B1C: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F9B20: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9B24: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F9B28: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F9B2C: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F9B30: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9B34: 54CA077E  clrlwi r10, r6, 0x1d
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x00000007u64;
	// 831F9B38: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F9B3C: 7D2321D6  mullw r9, r3, r4
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F9B40: 81070004  lwz r8, 4(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F9B44: 7D432B78  or r3, r10, r5
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 831F9B48: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9B4C: 5469073E  clrlwi r9, r3, 0x1c
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 831F9B50: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9B54: 7C844050  subf r4, r4, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 831F9B58: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F9B5C: 409A0064  bne cr6, 0x831f9bc0
	if !ctx.cr[6].eq {
	pc = 0x831F9BC0; continue 'dispatch;
	}
	// 831F9B60: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F9B64: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9B68: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F9B6C: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F9B70: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F9B74: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9B78: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F9B7C: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9B80: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F9B84: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F9B88: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F9B8C: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F9B90: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F9B94: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F9B98: 40990028  ble cr6, 0x831f9bc0
	if !ctx.cr[6].gt {
	pc = 0x831F9BC0; continue 'dispatch;
	}
	// 831F9B9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F9BA0: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9BA4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F9BA8: 41980018  blt cr6, 0x831f9bc0
	if ctx.cr[6].lt {
	pc = 0x831F9BC0; continue 'dispatch;
	}
	// 831F9BAC: 4BFF48ED  bl 0x831ee498
	ctx.lr = 0x831F9BB0;
	sub_831EE498(ctx, base);
	// 831F9BB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9BBC: 4E800020  blr
	return;
	// 831F9BC0: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F9BC4: 4BFF98AD  bl 0x831f3470
	ctx.lr = 0x831F9BC8;
	sub_831F3470(ctx, base);
	// 831F9BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F9BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F9BD8 size=216
    let mut pc: u32 = 0x831F9BD8;
    'dispatch: loop {
        match pc {
            0x831F9BD8 => {
    //   block [0x831F9BD8..0x831F9CB0)
	// 831F9BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F9BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F9BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F9BE4: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F9BE8: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F9BEC: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F9BF0: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F9BF4: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F9BF8: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9BFC: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F9C00: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F9C04: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F9C08: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9C0C: 54CA077E  clrlwi r10, r6, 0x1d
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x00000007u64;
	// 831F9C10: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F9C14: 7D2321D6  mullw r9, r3, r4
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F9C18: 81070004  lwz r8, 4(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F9C1C: 7D432B78  or r3, r10, r5
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 831F9C20: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9C24: 5469073E  clrlwi r9, r3, 0x1c
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 831F9C28: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9C2C: 7C844050  subf r4, r4, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 831F9C30: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F9C34: 409A0064  bne cr6, 0x831f9c98
	if !ctx.cr[6].eq {
	pc = 0x831F9C98; continue 'dispatch;
	}
	// 831F9C38: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F9C3C: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9C40: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F9C44: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F9C48: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F9C4C: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9C50: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F9C54: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9C58: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F9C5C: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F9C60: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F9C64: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F9C68: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F9C6C: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F9C70: 40990028  ble cr6, 0x831f9c98
	if !ctx.cr[6].gt {
	pc = 0x831F9C98; continue 'dispatch;
	}
	// 831F9C74: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F9C78: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9C7C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F9C80: 41980018  blt cr6, 0x831f9c98
	if ctx.cr[6].lt {
	pc = 0x831F9C98; continue 'dispatch;
	}
	// 831F9C84: 4BFF4A9D  bl 0x831ee720
	ctx.lr = 0x831F9C88;
	sub_831EE720(ctx, base);
	// 831F9C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9C94: 4E800020  blr
	return;
	// 831F9C98: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F9C9C: 4BFF9B05  bl 0x831f37a0
	ctx.lr = 0x831F9CA0;
	sub_831F37A0(ctx, base);
	// 831F9CA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F9CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F9CB0 size=216
    let mut pc: u32 = 0x831F9CB0;
    'dispatch: loop {
        match pc {
            0x831F9CB0 => {
    //   block [0x831F9CB0..0x831F9D88)
	// 831F9CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F9CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F9CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F9CBC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F9CC0: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F9CC4: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F9CC8: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F9CCC: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F9CD0: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9CD4: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F9CD8: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F9CDC: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F9CE0: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9CE4: 54CA077E  clrlwi r10, r6, 0x1d
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x00000007u64;
	// 831F9CE8: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F9CEC: 7D2321D6  mullw r9, r3, r4
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F9CF0: 81070004  lwz r8, 4(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F9CF4: 7D432B78  or r3, r10, r5
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 831F9CF8: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9CFC: 5469073E  clrlwi r9, r3, 0x1c
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 831F9D00: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9D04: 7C844050  subf r4, r4, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 831F9D08: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F9D0C: 409A0064  bne cr6, 0x831f9d70
	if !ctx.cr[6].eq {
	pc = 0x831F9D70; continue 'dispatch;
	}
	// 831F9D10: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F9D14: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9D18: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F9D1C: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F9D20: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F9D24: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9D28: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F9D2C: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9D30: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F9D34: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F9D38: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F9D3C: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F9D40: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F9D44: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F9D48: 40990028  ble cr6, 0x831f9d70
	if !ctx.cr[6].gt {
	pc = 0x831F9D70; continue 'dispatch;
	}
	// 831F9D4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F9D50: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9D54: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F9D58: 41980018  blt cr6, 0x831f9d70
	if ctx.cr[6].lt {
	pc = 0x831F9D70; continue 'dispatch;
	}
	// 831F9D5C: 4BFF4CAD  bl 0x831eea08
	ctx.lr = 0x831F9D60;
	sub_831EEA08(ctx, base);
	// 831F9D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9D6C: 4E800020  blr
	return;
	// 831F9D70: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F9D74: 4BFF9DED  bl 0x831f3b60
	ctx.lr = 0x831F9D78;
	sub_831F3B60(ctx, base);
	// 831F9D78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F9D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F9D88 size=216
    let mut pc: u32 = 0x831F9D88;
    'dispatch: loop {
        match pc {
            0x831F9D88 => {
    //   block [0x831F9D88..0x831F9E60)
	// 831F9D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F9D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831F9D90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F9D94: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831F9D98: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F9D9C: C1A70030  lfs f13, 0x30(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F9DA0: 81070018  lwz r8, 0x18(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F9DA4: 81670014  lwz r11, 0x14(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F9DA8: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9DAC: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 831F9DB0: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F9DB4: 8867000D  lbz r3, 0xd(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F9DB8: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9DBC: 54CA077E  clrlwi r10, r6, 0x1d
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x00000007u64;
	// 831F9DC0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F9DC4: 7D2321D6  mullw r9, r3, r4
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	// 831F9DC8: 81070004  lwz r8, 4(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F9DCC: 7D432B78  or r3, r10, r5
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 831F9DD0: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831F9DD4: 5469073E  clrlwi r9, r3, 0x1c
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 831F9DD8: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831F9DDC: 7C844050  subf r4, r4, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 831F9DE0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831F9DE4: 409A0064  bne cr6, 0x831f9e48
	if !ctx.cr[6].eq {
	pc = 0x831F9E48; continue 'dispatch;
	}
	// 831F9DE8: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831F9DEC: C007002C  lfs f0, 0x2c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9DF0: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F9DF4: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 831F9DF8: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831F9DFC: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9E00: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831F9E04: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831F9E08: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 831F9E0C: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 831F9E10: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 831F9E14: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 831F9E18: ECC06A3A  fmadds f6, f0, f8, f13
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 831F9E1C: FF073000  fcmpu cr6, f7, f6
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[6].f64);
	// 831F9E20: 40990028  ble cr6, 0x831f9e48
	if !ctx.cr[6].gt {
	pc = 0x831F9E48; continue 'dispatch;
	}
	// 831F9E24: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831F9E28: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9E2C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831F9E30: 41980018  blt cr6, 0x831f9e48
	if ctx.cr[6].lt {
	pc = 0x831F9E48; continue 'dispatch;
	}
	// 831F9E34: 4BFF4F95  bl 0x831eedc8
	ctx.lr = 0x831F9E38;
	sub_831EEDC8(ctx, base);
	// 831F9E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9E44: 4E800020  blr
	return;
	// 831F9E48: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831F9E4C: 4BFFA22D  bl 0x831f4078
	ctx.lr = 0x831F9E50;
	sub_831F4078(ctx, base);
	// 831F9E50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831F9E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831F9E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831F9E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831F9E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831F9E60 size=448
    let mut pc: u32 = 0x831F9E60;
    'dispatch: loop {
        match pc {
            0x831F9E60 => {
    //   block [0x831F9E60..0x831FA020)
	// 831F9E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831F9E64: 4BFAE305  bl 0x831a8168
	ctx.lr = 0x831F9E68;
	sub_831A8130(ctx, base);
	// 831F9E68: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831F9E6C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831F9E70: 8BC3000D  lbz r30, 0xd(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831F9E74: 8103001C  lwz r8, 0x1c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831F9E78: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831F9E7C: 7CFE51D6  mullw r7, r30, r10
	ctx.r[7].s64 = (ctx.r[30].s32 as i64) * (ctx.r[10].s32 as i64);
	// 831F9E80: 83A30018  lwz r29, 0x18(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831F9E84: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831F9E88: 80830014  lwz r4, 0x14(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831F9E8C: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831F9E90: 7D4AF850  subf r10, r10, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[10].s64;
	// 831F9E94: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831F9E98: 7CC8E850  subf r6, r8, r29
	ctx.r[6].s64 = ctx.r[29].s64 - ctx.r[8].s64;
	// 831F9E9C: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 831F9EA0: 7D292214  add r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 831F9EA4: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 831F9EA8: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 831F9EAC: 41980008  blt cr6, 0x831f9eb4
	if ctx.cr[6].lt {
	pc = 0x831F9EB4; continue 'dispatch;
	}
	// 831F9EB0: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 831F9EB4: 54EA077E  clrlwi r10, r7, 0x1d
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0x00000007u64;
	// 831F9EB8: 7D484B78  or r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 831F9EBC: 7D0A5B78  or r10, r8, r11
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 831F9EC0: 5548073E  clrlwi r8, r10, 0x1c
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 831F9EC4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831F9EC8: 419A0010  beq cr6, 0x831f9ed8
	if ctx.cr[6].eq {
	pc = 0x831F9ED8; continue 'dispatch;
	}
	// 831F9ECC: 4BFFA955  bl 0x831f4820
	ctx.lr = 0x831F9ED0;
	sub_831F4820(ctx, base);
	// 831F9ED0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831F9ED4: 4BFAE2E4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 831F9ED8: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831F9EDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831F9EE0: 3908007F  addi r8, r8, 0x7f
	ctx.r[8].s64 = ctx.r[8].s64 + 127;
	// 831F9EE4: 5508C9FE  srwi r8, r8, 7
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831F9EE8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831F9EEC: 419A0018  beq cr6, 0x831f9f04
	if ctx.cr[6].eq {
	pc = 0x831F9F04; continue 'dispatch;
	}
	// 831F9EF0: 555C3830  slwi r28, r10, 7
	ctx.r[28].u32 = ctx.r[10].u32.wrapping_shl(7);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 831F9EF4: 7C1C5A2C  dcbt r28, r11
	// 831F9EF8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831F9EFC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 831F9F00: 4198FFF0  blt cr6, 0x831f9ef0
	if ctx.cr[6].lt {
	pc = 0x831F9EF0; continue 'dispatch;
	}
	// 831F9F04: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 831F9F08: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831F9F0C: 54E8083C  slwi r8, r7, 1
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831F9F10: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831F9F14: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 831F9F18: C9610058  lfd f11, 0x58(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831F9F1C: 7D0607B4  extsw r6, r8
	ctx.r[6].s64 = ctx.r[8].s32 as i64;
	// 831F9F20: ED4D0028  fsubs f10, f13, f0
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831F9F24: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 831F9F28: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 831F9F2C: F8C10058  std r6, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u64 ) };
	// 831F9F30: C9210058  lfd f9, 0x58(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831F9F34: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 831F9F38: 3B810050  addi r28, r1, 0x50
	ctx.r[28].s64 = ctx.r[1].s64 + 80;
	// 831F9F3C: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 831F9F40: C18AE830  lfs f12, -0x17d0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-6096 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831F9F44: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831F9F48: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 831F9F4C: 394A2990  addi r10, r10, 0x2990
	ctx.r[10].s64 = ctx.r[10].s64 + 10640;
	// 831F9F50: 13E0E407  vcmpneb. (lvlx128) v31, v0, v28
	tmp.u32 = ctx.r[28].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FA020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FA020 size=528
    let mut pc: u32 = 0x831FA020;
    'dispatch: loop {
        match pc {
            0x831FA020 => {
    //   block [0x831FA020..0x831FA230)
	// 831FA020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FA024: 4BFAE141  bl 0x831a8164
	ctx.lr = 0x831FA028;
	sub_831A8130(ctx, base);
	// 831FA028: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FA02C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FA030: 8B83000D  lbz r28, 0xd(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831FA034: 83A30004  lwz r29, 4(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FA038: 8103001C  lwz r8, 0x1c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FA03C: 7CFC51D6  mullw r7, r28, r10
	ctx.r[7].s64 = (ctx.r[28].s32 as i64) * (ctx.r[10].s32 as i64);
	// 831FA040: 83630018  lwz r27, 0x18(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831FA044: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FA048: 83C30014  lwz r30, 0x14(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FA04C: 7D2AE850  subf r9, r10, r29
	ctx.r[9].s64 = ctx.r[29].s64 - ctx.r[10].s64;
	// 831FA050: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FA054: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FA058: 7CC8D850  subf r6, r8, r27
	ctx.r[6].s64 = ctx.r[27].s64 - ctx.r[8].s64;
	// 831FA05C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 831FA060: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 831FA064: 7F093000  cmpw cr6, r9, r6
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[6].s32, &mut ctx.xer);
	// 831FA068: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 831FA06C: 41980008  blt cr6, 0x831fa074
	if ctx.cr[6].lt {
	pc = 0x831FA074; continue 'dispatch;
	}
	// 831FA070: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 831FA074: 54E9077E  clrlwi r9, r7, 0x1d
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x00000007u64;
	// 831FA078: 7D285378  or r8, r9, r10
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 831FA07C: 7D055B78  or r5, r8, r11
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 831FA080: 54A4073E  clrlwi r4, r5, 0x1c
	ctx.r[4].u64 = ctx.r[5].u32 as u64 & 0x0000000Fu64;
	// 831FA084: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831FA088: 419A0010  beq cr6, 0x831fa098
	if ctx.cr[6].eq {
	pc = 0x831FA098; continue 'dispatch;
	}
	// 831FA08C: 4BFFA8B5  bl 0x831f4940
	ctx.lr = 0x831FA090;
	sub_831F4940(ctx, base);
	// 831FA090: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831FA094: 4BFAE120  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831FA098: 54E81838  slwi r8, r7, 3
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FA09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831FA0A0: 3908007F  addi r8, r8, 0x7f
	ctx.r[8].s64 = ctx.r[8].s64 + 127;
	// 831FA0A4: 5508C9FE  srwi r8, r8, 7
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FA0A8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831FA0AC: 419A0018  beq cr6, 0x831fa0c4
	if ctx.cr[6].eq {
	pc = 0x831FA0C4; continue 'dispatch;
	}
	// 831FA0B0: 55253830  slwi r5, r9, 7
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831FA0B4: 7C055A2C  dcbt r5, r11
	// 831FA0B8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831FA0BC: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 831FA0C0: 4198FFF0  blt cr6, 0x831fa0b0
	if ctx.cr[6].lt {
	pc = 0x831FA0B0; continue 'dispatch;
	}
	// 831FA0C4: 7CC907B4  extsw r9, r6
	ctx.r[9].s64 = ctx.r[6].s32 as i64;
	// 831FA0C8: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831FA0CC: 54E8083C  slwi r8, r7, 1
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FA0D0: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831FA0D4: F9210058  std r9, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u64 ) };
	// 831FA0D8: C9610058  lfd f11, 0x58(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FA0DC: 7D0607B4  extsw r6, r8
	ctx.r[6].s64 = ctx.r[8].s32 as i64;
	// 831FA0E0: ED4D0028  fsubs f10, f13, f0
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831FA0E4: 3CA08208  lis r5, -0x7df8
	ctx.r[5].s64 = -2113404928;
	// 831FA0E8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 831FA0EC: F8C10058  std r6, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u64 ) };
	// 831FA0F0: C9210058  lfd f9, 0x58(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FA0F4: 39210058  addi r9, r1, 0x58
	ctx.r[9].s64 = ctx.r[1].s64 + 88;
	// 831FA0F8: 3CC0821A  lis r6, -0x7de6
	ctx.r[6].s64 = -2112225280;
	// 831FA0FC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 831FA100: C185E830  lfs f12, -0x17d0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-6096 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831FA104: 38A62990  addi r5, r6, 0x2990
	ctx.r[5].s64 = ctx.r[6].s64 + 10640;
	// 831FA108: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 831FA10C: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 831FA110: 13E04407  vcmpneb. (lvlx128) v31, v0, v8
	tmp.u32 = ctx.r[8].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FA230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FA230 size=832
    let mut pc: u32 = 0x831FA230;
    'dispatch: loop {
        match pc {
            0x831FA230 => {
    //   block [0x831FA230..0x831FA570)
	// 831FA230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FA234: 4BFADF09  bl 0x831a813c
	ctx.lr = 0x831FA238;
	sub_831A8130(ctx, base);
	// 831FA238: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FA23C: 80C30008  lwz r6, 8(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FA240: 8923000D  lbz r9, 0xd(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831FA244: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FA248: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831FA24C: 7D2931D6  mullw r9, r9, r6
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[6].s32 as i64);
	// 831FA250: 8083001C  lwz r4, 0x1c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FA254: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FA258: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FA25C: 906100F4  stw r3, 0xf4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), ctx.r[3].u32 ) };
	// 831FA260: 7D064050  subf r8, r6, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[6].s64;
	// 831FA264: 7CA43850  subf r5, r4, r7
	ctx.r[5].s64 = ctx.r[7].s64 - ctx.r[4].s64;
	// 831FA268: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FA26C: 5486103A  slwi r6, r4, 2
	ctx.r[6].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831FA270: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 831FA274: 7CE65214  add r7, r6, r10
	ctx.r[7].u64 = ctx.r[6].u64 + ctx.r[10].u64;
	// 831FA278: 7F082800  cmpw cr6, r8, r5
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[5].s32, &mut ctx.xer);
	// 831FA27C: 41980008  blt cr6, 0x831fa284
	if ctx.cr[6].lt {
	pc = 0x831FA284; continue 'dispatch;
	}
	// 831FA280: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 831FA284: 550B077E  clrlwi r11, r8, 0x1d
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x00000007u64;
	// 831FA288: 7D6A3B78  or r10, r11, r7
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[7].u64;
	// 831FA28C: 7D464B78  or r6, r10, r9
	ctx.r[6].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 831FA290: 54C4073E  clrlwi r4, r6, 0x1c
	ctx.r[4].u64 = ctx.r[6].u32 as u64 & 0x0000000Fu64;
	// 831FA294: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831FA298: 419A0010  beq cr6, 0x831fa2a8
	if ctx.cr[6].eq {
	pc = 0x831FA2A8; continue 'dispatch;
	}
	// 831FA29C: 4BFFA90D  bl 0x831f4ba8
	ctx.lr = 0x831FA2A0;
	sub_831F4BA8(ctx, base);
	// 831FA2A0: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 831FA2A4: 4BFADEE8  b 0x831a818c
	sub_831A8180(ctx, base);
	return;
	// 831FA2A8: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FA2AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FA2B0: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 831FA2B4: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FA2B8: 38CA007F  addi r6, r10, 0x7f
	ctx.r[6].s64 = ctx.r[10].s64 + 127;
	// 831FA2BC: 54CAC9FE  srwi r10, r6, 7
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shr(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FA2C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831FA2C4: 419A0018  beq cr6, 0x831fa2dc
	if ctx.cr[6].eq {
	pc = 0x831FA2DC; continue 'dispatch;
	}
	// 831FA2C8: 55663830  slwi r6, r11, 7
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831FA2CC: 7C064A2C  dcbt r6, r9
	// 831FA2D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FA2D4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831FA2D8: 4198FFF0  blt cr6, 0x831fa2c8
	if ctx.cr[6].lt {
	pc = 0x831FA2C8; continue 'dispatch;
	}
	// 831FA2DC: 7CAB07B4  extsw r11, r5
	ctx.r[11].s64 = ctx.r[5].s32 as i64;
	// 831FA2E0: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831FA2E4: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FA2E8: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831FA2EC: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 831FA2F0: C9610058  lfd f11, 0x58(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FA2F4: 7D4607B4  extsw r6, r10
	ctx.r[6].s64 = ctx.r[10].s32 as i64;
	// 831FA2F8: ED4D0028  fsubs f10, f13, f0
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831FA2FC: 3CA08208  lis r5, -0x7df8
	ctx.r[5].s64 = -2113404928;
	// 831FA300: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 831FA304: F8C10058  std r6, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u64 ) };
	// 831FA308: C9210058  lfd f9, 0x58(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FA30C: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 831FA310: 3CC0821A  lis r6, -0x7de6
	ctx.r[6].s64 = -2112225280;
	// 831FA314: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 831FA318: C185E830  lfs f12, -0x17d0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-6096 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831FA31C: 38A62990  addi r5, r6, 0x2990
	ctx.r[5].s64 = ctx.r[6].s64 + 10640;
	// 831FA320: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 831FA324: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831FA328: 13E05407  vcmpneb. (lvlx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FA570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FA570 size=520
    let mut pc: u32 = 0x831FA570;
    'dispatch: loop {
        match pc {
            0x831FA570 => {
    //   block [0x831FA570..0x831FA778)
	// 831FA570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FA574: 4BFADBED  bl 0x831a8160
	ctx.lr = 0x831FA578;
	sub_831A8130(ctx, base);
	// 831FA578: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FA57C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FA580: 8BA3000D  lbz r29, 0xd(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831FA584: 80E3001C  lwz r7, 0x1c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FA588: 7CDD51D6  mullw r6, r29, r10
	ctx.r[6].s64 = (ctx.r[29].s32 as i64) * (ctx.r[10].s32 as i64);
	// 831FA58C: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FA590: 83830018  lwz r28, 0x18(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831FA594: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FA598: 83E30014  lwz r31, 0x14(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FA59C: 54CB083C  slwi r11, r6, 1
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FA5A0: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FA5A4: 7D2AF050  subf r9, r10, r30
	ctx.r[9].s64 = ctx.r[30].s64 - ctx.r[10].s64;
	// 831FA5A8: 7CE7E050  subf r7, r7, r28
	ctx.r[7].s64 = ctx.r[28].s64 - ctx.r[7].s64;
	// 831FA5AC: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 831FA5B0: 7D68FA14  add r11, r8, r31
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 831FA5B4: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 831FA5B8: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	// 831FA5BC: 41980008  blt cr6, 0x831fa5c4
	if ctx.cr[6].lt {
	pc = 0x831FA5C4; continue 'dispatch;
	}
	// 831FA5C0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 831FA5C4: 7CC95B78  or r9, r6, r11
	ctx.r[9].u64 = ctx.r[6].u64 | ctx.r[11].u64;
	// 831FA5C8: 7D285378  or r8, r9, r10
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 831FA5CC: 5505073E  clrlwi r5, r8, 0x1c
	ctx.r[5].u64 = ctx.r[8].u32 as u64 & 0x0000000Fu64;
	// 831FA5D0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 831FA5D4: 419A0010  beq cr6, 0x831fa5e4
	if ctx.cr[6].eq {
	pc = 0x831FA5E4; continue 'dispatch;
	}
	// 831FA5D8: 4BFFA869  bl 0x831f4e40
	ctx.lr = 0x831FA5DC;
	sub_831F4E40(ctx, base);
	// 831FA5DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831FA5E0: 4BFADBD0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 831FA5E4: 54C5083C  slwi r5, r6, 1
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831FA5E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831FA5EC: 3905007F  addi r8, r5, 0x7f
	ctx.r[8].s64 = ctx.r[5].s64 + 127;
	// 831FA5F0: 5508C9FE  srwi r8, r8, 7
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FA5F4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831FA5F8: 419A0018  beq cr6, 0x831fa610
	if ctx.cr[6].eq {
	pc = 0x831FA610; continue 'dispatch;
	}
	// 831FA5FC: 553B3830  slwi r27, r9, 7
	ctx.r[27].u32 = ctx.r[9].u32.wrapping_shl(7);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 831FA600: 7C1B522C  dcbt r27, r10
	// 831FA604: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831FA608: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 831FA60C: 4198FFF0  blt cr6, 0x831fa5fc
	if ctx.cr[6].lt {
	pc = 0x831FA5FC; continue 'dispatch;
	}
	// 831FA610: 7CE907B4  extsw r9, r7
	ctx.r[9].s64 = ctx.r[7].s32 as i64;
	// 831FA614: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831FA618: 7CA807B4  extsw r8, r5
	ctx.r[8].s64 = ctx.r[5].s32 as i64;
	// 831FA61C: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831FA620: F9210058  std r9, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u64 ) };
	// 831FA624: C9610058  lfd f11, 0x58(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FA628: F9010058  std r8, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u64 ) };
	// 831FA62C: C9410058  lfd f10, 0x58(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FA630: ED2D0028  fsubs f9, f13, f0
	ctx.f[9].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831FA634: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 831FA638: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 831FA63C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 831FA640: 3CA0821A  lis r5, -0x7de6
	ctx.r[5].s64 = -2112225280;
	// 831FA644: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 831FA648: 390529A0  addi r8, r5, 0x29a0
	ctx.r[8].s64 = ctx.r[5].s64 + 10656;
	// 831FA64C: 13E0DC07  vcmpneb. (lvlx128) v31, v0, v27
	tmp.u32 = ctx.r[27].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831FA650: 38A92990  addi r5, r9, 0x2990
	ctx.r[5].s64 = ctx.r[9].s64 + 10640;
	// 831FA654: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FA778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FA778 size=524
    let mut pc: u32 = 0x831FA778;
    'dispatch: loop {
        match pc {
            0x831FA778 => {
    //   block [0x831FA778..0x831FA984)
	// 831FA778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FA77C: 4BFAD9E9  bl 0x831a8164
	ctx.lr = 0x831FA780;
	sub_831A8130(ctx, base);
	// 831FA780: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FA784: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FA788: 8BC3000D  lbz r30, 0xd(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831FA78C: 80E3001C  lwz r7, 0x1c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FA790: 7CDE51D6  mullw r6, r30, r10
	ctx.r[6].s64 = (ctx.r[30].s32 as i64) * (ctx.r[10].s32 as i64);
	// 831FA794: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FA798: 83A30018  lwz r29, 0x18(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831FA79C: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FA7A0: 80830014  lwz r4, 0x14(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FA7A4: 54CB083C  slwi r11, r6, 1
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FA7A8: 7D2AF850  subf r9, r10, r31
	ctx.r[9].s64 = ctx.r[31].s64 - ctx.r[10].s64;
	// 831FA7AC: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FA7B0: 7CC7E850  subf r6, r7, r29
	ctx.r[6].s64 = ctx.r[29].s64 - ctx.r[7].s64;
	// 831FA7B4: 7D4B2A14  add r10, r11, r5
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 831FA7B8: 7D682214  add r11, r8, r4
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 831FA7BC: 7F093000  cmpw cr6, r9, r6
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[6].s32, &mut ctx.xer);
	// 831FA7C0: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 831FA7C4: 41980008  blt cr6, 0x831fa7cc
	if ctx.cr[6].lt {
	pc = 0x831FA7CC; continue 'dispatch;
	}
	// 831FA7C8: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 831FA7CC: 54E9077E  clrlwi r9, r7, 0x1d
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x00000007u64;
	// 831FA7D0: 7D285B78  or r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 831FA7D4: 7D095378  or r9, r8, r10
	ctx.r[9].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 831FA7D8: 5528073E  clrlwi r8, r9, 0x1c
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 831FA7DC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831FA7E0: 419A0010  beq cr6, 0x831fa7f0
	if ctx.cr[6].eq {
	pc = 0x831FA7F0; continue 'dispatch;
	}
	// 831FA7E4: 4BFFA79D  bl 0x831f4f80
	ctx.lr = 0x831FA7E8;
	sub_831F4F80(ctx, base);
	// 831FA7E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831FA7EC: 4BFAD9C8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831FA7F0: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FA7F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831FA7F8: 3908007F  addi r8, r8, 0x7f
	ctx.r[8].s64 = ctx.r[8].s64 + 127;
	// 831FA7FC: 5508C9FE  srwi r8, r8, 7
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FA800: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831FA804: 419A0018  beq cr6, 0x831fa81c
	if ctx.cr[6].eq {
	pc = 0x831FA81C; continue 'dispatch;
	}
	// 831FA808: 553C3830  slwi r28, r9, 7
	ctx.r[28].u32 = ctx.r[9].u32.wrapping_shl(7);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 831FA80C: 7C1C522C  dcbt r28, r10
	// 831FA810: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831FA814: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 831FA818: 4198FFF0  blt cr6, 0x831fa808
	if ctx.cr[6].lt {
	pc = 0x831FA808; continue 'dispatch;
	}
	// 831FA81C: 7CC907B4  extsw r9, r6
	ctx.r[9].s64 = ctx.r[6].s32 as i64;
	// 831FA820: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831FA824: 54E8083C  slwi r8, r7, 1
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FA828: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831FA82C: F9210058  std r9, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u64 ) };
	// 831FA830: C9610058  lfd f11, 0x58(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FA834: 7D0607B4  extsw r6, r8
	ctx.r[6].s64 = ctx.r[8].s32 as i64;
	// 831FA838: ED4D0028  fsubs f10, f13, f0
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831FA83C: 3B810050  addi r28, r1, 0x50
	ctx.r[28].s64 = ctx.r[1].s64 + 80;
	// 831FA840: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 831FA844: F8C10058  std r6, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u64 ) };
	// 831FA848: C9210058  lfd f9, 0x58(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FA84C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 831FA850: 3D00821A  lis r8, -0x7de6
	ctx.r[8].s64 = -2112225280;
	// 831FA854: 3CC08208  lis r6, -0x7df8
	ctx.r[6].s64 = -2113404928;
	// 831FA858: 13E0E407  vcmpneb. (lvlx128) v31, v0, v28
	tmp.u32 = ctx.r[28].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831FA85C: 39082990  addi r8, r8, 0x2990
	ctx.r[8].s64 = ctx.r[8].s64 + 10640;
	// 831FA860: 3B610058  addi r27, r1, 0x58
	ctx.r[27].s64 = ctx.r[1].s64 + 88;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FA988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FA988 size=844
    let mut pc: u32 = 0x831FA988;
    'dispatch: loop {
        match pc {
            0x831FA988 => {
    //   block [0x831FA988..0x831FACD4)
	// 831FA988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FA98C: 4BFAD7BD  bl 0x831a8148
	ctx.lr = 0x831FA990;
	sub_831A8130(ctx, base);
	// 831FA990: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FA994: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FA998: 8963000D  lbz r11, 0xd(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831FA99C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FA9A0: 7D0B51D6  mullw r8, r11, r10
	ctx.r[8].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 831FA9A4: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831FA9A8: 80A3001C  lwz r5, 0x1c(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FA9AC: 82A30000  lwz r21, 0(r3)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FA9B0: 82830014  lwz r20, 0x14(r3)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FA9B4: 7D6A4850  subf r11, r10, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 831FA9B8: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FA9BC: 7CC53850  subf r6, r5, r7
	ctx.r[6].s64 = ctx.r[7].s64 - ctx.r[5].s64;
	// 831FA9C0: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FA9C4: 7D2AAA14  add r9, r10, r21
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[21].u64;
	// 831FA9C8: 7CE8A214  add r7, r8, r20
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[20].u64;
	// 831FA9CC: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 831FA9D0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 831FA9D4: 41980008  blt cr6, 0x831fa9dc
	if ctx.cr[6].lt {
	pc = 0x831FA9DC; continue 'dispatch;
	}
	// 831FA9D8: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 831FA9DC: 554B077E  clrlwi r11, r10, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 831FA9E0: 7D683B78  or r8, r11, r7
	ctx.r[8].u64 = ctx.r[11].u64 | ctx.r[7].u64;
	// 831FA9E4: 7D054B78  or r5, r8, r9
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 831FA9E8: 54A4073E  clrlwi r4, r5, 0x1c
	ctx.r[4].u64 = ctx.r[5].u32 as u64 & 0x0000000Fu64;
	// 831FA9EC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831FA9F0: 419A0010  beq cr6, 0x831faa00
	if ctx.cr[6].eq {
	pc = 0x831FAA00; continue 'dispatch;
	}
	// 831FA9F4: 4BFFA895  bl 0x831f5288
	ctx.lr = 0x831FA9F8;
	sub_831F5288(ctx, base);
	// 831FA9F8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 831FA9FC: 4BFAD79C  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
	// 831FAA00: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FAA04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FAA08: 7D0A4214  add r8, r10, r8
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 831FAA0C: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FAA10: 38A8007F  addi r5, r8, 0x7f
	ctx.r[5].s64 = ctx.r[8].s64 + 127;
	// 831FAA14: 54A8C9FE  srwi r8, r5, 7
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shr(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FAA18: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831FAA1C: 419A0018  beq cr6, 0x831faa34
	if ctx.cr[6].eq {
	pc = 0x831FAA34; continue 'dispatch;
	}
	// 831FAA20: 55653830  slwi r5, r11, 7
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831FAA24: 7C054A2C  dcbt r5, r9
	// 831FAA28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FAA2C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 831FAA30: 4198FFF0  blt cr6, 0x831faa20
	if ctx.cr[6].lt {
	pc = 0x831FAA20; continue 'dispatch;
	}
	// 831FAA34: 7CCB07B4  extsw r11, r6
	ctx.r[11].s64 = ctx.r[6].s32 as i64;
	// 831FAA38: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831FAA3C: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FAA40: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831FAA44: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 831FAA48: C9610058  lfd f11, 0x58(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FAA4C: 7D0607B4  extsw r6, r8
	ctx.r[6].s64 = ctx.r[8].s32 as i64;
	// 831FAA50: ED4D0028  fsubs f10, f13, f0
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831FAA54: 3C80821A  lis r4, -0x7de6
	ctx.r[4].s64 = -2112225280;
	// 831FAA58: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 831FAA5C: F8C10058  std r6, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u64 ) };
	// 831FAA60: C9210058  lfd f9, 0x58(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FAA64: 39042990  addi r8, r4, 0x2990
	ctx.r[8].s64 = ctx.r[4].s64 + 10640;
	// 831FAA68: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 831FAA6C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 831FAA70: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831FAA74: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 831FAA78: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831FAA7C: C18BE830  lfs f12, -0x17d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6096 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831FAA80: 13E03407  vcmpneb. (lvlx128) v31, v0, v6
	tmp.u32 = ctx.r[6].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FACD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FACD8 size=680
    let mut pc: u32 = 0x831FACD8;
    'dispatch: loop {
        match pc {
            0x831FACD8 => {
    //   block [0x831FACD8..0x831FAF80)
	// 831FACD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FACDC: 4BFAD47D  bl 0x831a8158
	ctx.lr = 0x831FACE0;
	sub_831A8130(ctx, base);
	// 831FACE0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FACE4: 8103001C  lwz r8, 0x1c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FACE8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FACEC: 8B43000D  lbz r26, 0xd(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831FACF0: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FACF4: 83630004  lwz r27, 4(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FACF8: 83230018  lwz r25, 0x18(r3)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831FACFC: 7D5A59D6  mullw r10, r26, r11
	ctx.r[10].s64 = (ctx.r[26].s32 as i64) * (ctx.r[11].s32 as i64);
	// 831FAD00: 83830014  lwz r28, 0x14(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FAD04: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FAD08: 7D6BD850  subf r11, r11, r27
	ctx.r[11].s64 = ctx.r[27].s64 - ctx.r[11].s64;
	// 831FAD0C: 7CE8C850  subf r7, r8, r25
	ctx.r[7].s64 = ctx.r[25].s64 - ctx.r[8].s64;
	// 831FAD10: 7D09E214  add r8, r9, r28
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[28].u64;
	// 831FAD14: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 831FAD18: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 831FAD1C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 831FAD20: 41980008  blt cr6, 0x831fad28
	if ctx.cr[6].lt {
	pc = 0x831FAD28; continue 'dispatch;
	}
	// 831FAD24: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 831FAD28: 7D0B5378  or r11, r8, r10
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 831FAD2C: 5566073E  clrlwi r6, r11, 0x1c
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 831FAD30: 7CC54B78  or r5, r6, r9
	ctx.r[5].u64 = ctx.r[6].u64 | ctx.r[9].u64;
	// 831FAD34: 54A406FE  clrlwi r4, r5, 0x1b
	ctx.r[4].u64 = ctx.r[5].u32 as u64 & 0x0000001Fu64;
	// 831FAD38: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831FAD3C: 419A0010  beq cr6, 0x831fad4c
	if ctx.cr[6].eq {
	pc = 0x831FAD4C; continue 'dispatch;
	}
	// 831FAD40: 4BFFA871  bl 0x831f55b0
	ctx.lr = 0x831FAD44;
	sub_831F55B0(ctx, base);
	// 831FAD44: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831FAD48: 4BFAD460  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 831FAD4C: 38C9007F  addi r6, r9, 0x7f
	ctx.r[6].s64 = ctx.r[9].s64 + 127;
	// 831FAD50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FAD54: 54C6C9FE  srwi r6, r6, 7
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shr(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831FAD58: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 831FAD5C: 419A0018  beq cr6, 0x831fad74
	if ctx.cr[6].eq {
	pc = 0x831FAD74; continue 'dispatch;
	}
	// 831FAD60: 55653830  slwi r5, r11, 7
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831FAD64: 7C05522C  dcbt r5, r10
	// 831FAD68: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FAD6C: 7F0B3040  cmplw cr6, r11, r6
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[6].u32, &mut ctx.xer);
	// 831FAD70: 4198FFF0  blt cr6, 0x831fad60
	if ctx.cr[6].lt {
	pc = 0x831FAD60; continue 'dispatch;
	}
	// 831FAD74: 7CEB07B4  extsw r11, r7
	ctx.r[11].s64 = ctx.r[7].s32 as i64;
	// 831FAD78: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831FAD7C: 5527083C  slwi r7, r9, 1
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FAD80: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831FAD84: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 831FAD88: C9610058  lfd f11, 0x58(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FAD8C: 7CE607B4  extsw r6, r7
	ctx.r[6].s64 = ctx.r[7].s32 as i64;
	// 831FAD90: ED4D0028  fsubs f10, f13, f0
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831FAD94: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 831FAD98: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 831FAD9C: F8C10058  std r6, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u64 ) };
	// 831FADA0: C9210058  lfd f9, 0x58(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FADA4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831FADA8: 3C80821A  lis r4, -0x7de6
	ctx.r[4].s64 = -2112225280;
	// 831FADAC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831FADB0: 3CE0821A  lis r7, -0x7de6
	ctx.r[7].s64 = -2112225280;
	// 831FADB4: 13E0F407  vcmpneb. (lvlx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831FADB8: 3CC0821A  lis r6, -0x7de6
	ctx.r[6].s64 = -2112225280;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FAF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FAF80 size=568
    let mut pc: u32 = 0x831FAF80;
    'dispatch: loop {
        match pc {
            0x831FAF80 => {
    //   block [0x831FAF80..0x831FB1B8)
	// 831FAF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FAF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FAF88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FAF8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FAF90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FAF94: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FAF98: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831FAF9C: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FAFA0: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831FAFA4: 8903000D  lbz r8, 0xd(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831FAFA8: 7CAB5050  subf r5, r11, r10
	ctx.r[5].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831FAFAC: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FAFB0: 7D4849D6  mullw r10, r8, r9
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[9].s32 as i64);
	// 831FAFB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FAFB8: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FAFBC: 7CBF0E70  srawi r31, r5, 1
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[5].s32 >> 1) as i64;
	// 831FAFC0: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FAFC4: 7D492050  subf r10, r9, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 831FAFC8: 7D3F0194  addze r9, r31
	tmp.s64 = ctx.r[31].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[31].u32);
	ctx.r[9].s64 = tmp.s64;
	// 831FAFCC: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 831FAFD0: 7D063A14  add r8, r6, r7
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 831FAFD4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831FAFD8: 40980008  bge cr6, 0x831fafe0
	if !ctx.cr[6].lt {
	pc = 0x831FAFE0; continue 'dispatch;
	}
	// 831FAFDC: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 831FAFE0: 552A07BE  clrlwi r10, r9, 0x1e
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x00000003u64;
	// 831FAFE4: 7D474378  or r7, r10, r8
	ctx.r[7].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 831FAFE8: 7CE65B78  or r6, r7, r11
	ctx.r[6].u64 = ctx.r[7].u64 | ctx.r[11].u64;
	// 831FAFEC: 54C4073E  clrlwi r4, r6, 0x1c
	ctx.r[4].u64 = ctx.r[6].u32 as u64 & 0x0000000Fu64;
	// 831FAFF0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831FAFF4: 419A000C  beq cr6, 0x831fb000
	if ctx.cr[6].eq {
	pc = 0x831FB000; continue 'dispatch;
	}
	// 831FAFF8: 4BFFB561  bl 0x831f6558
	ctx.lr = 0x831FAFFC;
	sub_831F6558(ctx, base);
	// 831FAFFC: 480001A4  b 0x831fb1a0
	pc = 0x831FB1A0; continue 'dispatch;
	// 831FB000: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FB004: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831FB008: 38E7007F  addi r7, r7, 0x7f
	ctx.r[7].s64 = ctx.r[7].s64 + 127;
	// 831FB00C: 54E7C9FE  srwi r7, r7, 7
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(7);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FB010: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 831FB014: 419A0018  beq cr6, 0x831fb02c
	if ctx.cr[6].eq {
	pc = 0x831FB02C; continue 'dispatch;
	}
	// 831FB018: 55463830  slwi r6, r10, 7
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831FB01C: 7C065A2C  dcbt r6, r11
	// 831FB020: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831FB024: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 831FB028: 4198FFF0  blt cr6, 0x831fb018
	if ctx.cr[6].lt {
	pc = 0x831FB018; continue 'dispatch;
	}
	// 831FB02C: 7CAA07B4  extsw r10, r5
	ctx.r[10].s64 = ctx.r[5].s32 as i64;
	// 831FB030: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831FB034: 5527083C  slwi r7, r9, 1
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FB038: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831FB03C: F9410060  std r10, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 831FB040: C9810060  lfd f12, 0x60(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 831FB044: 7CE607B4  extsw r6, r7
	ctx.r[6].s64 = ctx.r[7].s32 as i64;
	// 831FB048: ED6D0028  fsubs f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831FB04C: 3C80821A  lis r4, -0x7de6
	ctx.r[4].s64 = -2112225280;
	// 831FB050: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 831FB054: F8C10060  std r6, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u64 ) };
	// 831FB058: C9410060  lfd f10, 0x60(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 831FB05C: 38E429A0  addi r7, r4, 0x29a0
	ctx.r[7].s64 = ctx.r[4].s64 + 10656;
	// 831FB060: C1230034  lfs f9, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 831FB064: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FB068: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831FB06C: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831FB070: 3FE08201  lis r31, -0x7dff
	ctx.r[31].s64 = -2113863680;
	// 831FB074: 38CA2990  addi r6, r10, 0x2990
	ctx.r[6].s64 = ctx.r[10].s64 + 10640;
	// 831FB078: 13E02407  vcmpneb. (lvlx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831FB07C: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FB1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FB1B8 size=616
    let mut pc: u32 = 0x831FB1B8;
    'dispatch: loop {
        match pc {
            0x831FB1B8 => {
    //   block [0x831FB1B8..0x831FB420)
	// 831FB1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FB1BC: 4BFACFB1  bl 0x831a816c
	ctx.lr = 0x831FB1C0;
	sub_831A8130(ctx, base);
	// 831FB1C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FB1C4: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FB1C8: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831FB1CC: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FB1D0: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831FB1D4: 8903000D  lbz r8, 0xd(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831FB1D8: 7CAB5050  subf r5, r11, r10
	ctx.r[5].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831FB1DC: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FB1E0: 7D4849D6  mullw r10, r8, r9
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[9].s32 as i64);
	// 831FB1E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FB1E8: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FB1EC: 7CA80E70  srawi r8, r5, 1
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[5].s32 >> 1) as i64;
	// 831FB1F0: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FB1F4: 7D292050  subf r9, r9, r4
	ctx.r[9].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 831FB1F8: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 831FB1FC: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831FB200: 7D663A14  add r11, r6, r7
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 831FB204: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831FB208: 40980008  bge cr6, 0x831fb210
	if !ctx.cr[6].lt {
	pc = 0x831FB210; continue 'dispatch;
	}
	// 831FB20C: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 831FB210: 5509077E  clrlwi r9, r8, 0x1d
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0x00000007u64;
	// 831FB214: 7D275B78  or r7, r9, r11
	ctx.r[7].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 831FB218: 7CE65378  or r6, r7, r10
	ctx.r[6].u64 = ctx.r[7].u64 | ctx.r[10].u64;
	// 831FB21C: 54C4073E  clrlwi r4, r6, 0x1c
	ctx.r[4].u64 = ctx.r[6].u32 as u64 & 0x0000000Fu64;
	// 831FB220: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831FB224: 419A0010  beq cr6, 0x831fb234
	if ctx.cr[6].eq {
	pc = 0x831FB234; continue 'dispatch;
	}
	// 831FB228: 4BFFBC31  bl 0x831f6e58
	ctx.lr = 0x831FB22C;
	sub_831F6E58(ctx, base);
	// 831FB22C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831FB230: 4BFACF8C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831FB234: 5506083C  slwi r6, r8, 1
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831FB238: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831FB23C: 38E6007F  addi r7, r6, 0x7f
	ctx.r[7].s64 = ctx.r[6].s64 + 127;
	// 831FB240: 54E7C9FE  srwi r7, r7, 7
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(7);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FB244: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 831FB248: 419A0018  beq cr6, 0x831fb260
	if ctx.cr[6].eq {
	pc = 0x831FB260; continue 'dispatch;
	}
	// 831FB24C: 55243830  slwi r4, r9, 7
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(7);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 831FB250: 7C04522C  dcbt r4, r10
	// 831FB254: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831FB258: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 831FB25C: 4198FFF0  blt cr6, 0x831fb24c
	if ctx.cr[6].lt {
	pc = 0x831FB24C; continue 'dispatch;
	}
	// 831FB260: 7CA907B4  extsw r9, r5
	ctx.r[9].s64 = ctx.r[5].s32 as i64;
	// 831FB264: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831FB268: 7CC707B4  extsw r7, r6
	ctx.r[7].s64 = ctx.r[6].s32 as i64;
	// 831FB26C: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831FB270: F9210058  std r9, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u64 ) };
	// 831FB274: C9610058  lfd f11, 0x58(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FB278: F8E10058  std r7, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u64 ) };
	// 831FB27C: C9410058  lfd f10, 0x58(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FB280: ED2D0028  fsubs f9, f13, f0
	ctx.f[9].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831FB284: A1230034  lhz r9, 0x34(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 831FB288: 3CA0821A  lis r5, -0x7de6
	ctx.r[5].s64 = -2112225280;
	// 831FB28C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 831FB290: 3C80821A  lis r4, -0x7de6
	ctx.r[4].s64 = -2112225280;
	// 831FB294: 3FC0821A  lis r30, -0x7de6
	ctx.r[30].s64 = -2112225280;
	// 831FB298: 3CE0821A  lis r7, -0x7de6
	ctx.r[7].s64 = -2112225280;
	// 831FB29C: B121006E  sth r9, 0x6e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(110 as u32), ctx.r[9].u16 ) };
	// 831FB2A0: 39210058  addi r9, r1, 0x58
	ctx.r[9].s64 = ctx.r[1].s64 + 88;
	// 831FB2A4: 38A529B0  addi r5, r5, 0x29b0
	ctx.r[5].s64 = ctx.r[5].s64 + 10672;
	// 831FB2A8: 388429A0  addi r4, r4, 0x29a0
	ctx.r[4].s64 = ctx.r[4].s64 + 10656;
	// 831FB2AC: 3BDE29C0  addi r30, r30, 0x29c0
	ctx.r[30].s64 = ctx.r[30].s64 + 10688;
	// 831FB2B0: 38E72990  addi r7, r7, 0x2990
	ctx.r[7].s64 = ctx.r[7].s64 + 10640;
	// 831FB2B4: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 831FB2B8: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 831FB2BC: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 831FB2C0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831FB2C4: C1869594  lfs f12, -0x6a6c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-27244 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831FB2C8: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 831FB2CC: 13E0FC07  vcmpneb. (lvlx128) v31, v0, v31
	tmp.u32 = ctx.r[31].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FB420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FB420 size=796
    let mut pc: u32 = 0x831FB420;
    'dispatch: loop {
        match pc {
            0x831FB420 => {
    //   block [0x831FB420..0x831FB73C)
	// 831FB420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FB424: 4BFACD3D  bl 0x831a8160
	ctx.lr = 0x831FB428;
	sub_831A8130(ctx, base);
	// 831FB428: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FB42C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FB430: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831FB434: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FB438: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831FB43C: 7CAB5050  subf r5, r11, r10
	ctx.r[5].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831FB440: 8903000D  lbz r8, 0xd(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831FB444: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FB448: 7CBF0E70  srawi r31, r5, 1
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[5].s32 >> 1) as i64;
	// 831FB44C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FB450: 7D4849D6  mullw r10, r8, r9
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[9].s32 as i64);
	// 831FB454: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FB458: 7D292050  subf r9, r9, r4
	ctx.r[9].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 831FB45C: 7D1F0194  addze r8, r31
	tmp.s64 = ctx.r[31].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[31].u32);
	ctx.r[8].s64 = tmp.s64;
	// 831FB460: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831FB464: 7D663A14  add r11, r6, r7
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 831FB468: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831FB46C: 40980008  bge cr6, 0x831fb474
	if !ctx.cr[6].lt {
	pc = 0x831FB474; continue 'dispatch;
	}
	// 831FB470: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 831FB474: 7D095B78  or r9, r8, r11
	ctx.r[9].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 831FB478: 7D275378  or r7, r9, r10
	ctx.r[7].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 831FB47C: 54E6073E  clrlwi r6, r7, 0x1c
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000000Fu64;
	// 831FB480: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831FB484: 419A0010  beq cr6, 0x831fb494
	if ctx.cr[6].eq {
	pc = 0x831FB494; continue 'dispatch;
	}
	// 831FB488: 4BFFC5C1  bl 0x831f7a48
	ctx.lr = 0x831FB48C;
	sub_831F7A48(ctx, base);
	// 831FB48C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831FB490: 4BFACD20  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 831FB494: 38E8007F  addi r7, r8, 0x7f
	ctx.r[7].s64 = ctx.r[8].s64 + 127;
	// 831FB498: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831FB49C: 54E7C9FE  srwi r7, r7, 7
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(7);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FB4A0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 831FB4A4: 419A0018  beq cr6, 0x831fb4bc
	if ctx.cr[6].eq {
	pc = 0x831FB4BC; continue 'dispatch;
	}
	// 831FB4A8: 55263830  slwi r6, r9, 7
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831FB4AC: 7C06522C  dcbt r6, r10
	// 831FB4B0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831FB4B4: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 831FB4B8: 4198FFF0  blt cr6, 0x831fb4a8
	if ctx.cr[6].lt {
	pc = 0x831FB4A8; continue 'dispatch;
	}
	// 831FB4BC: 7CA907B4  extsw r9, r5
	ctx.r[9].s64 = ctx.r[5].s32 as i64;
	// 831FB4C0: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831FB4C4: 5507083C  slwi r7, r8, 1
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FB4C8: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831FB4CC: F9210058  std r9, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u64 ) };
	// 831FB4D0: C9610058  lfd f11, 0x58(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FB4D4: 7CE607B4  extsw r6, r7
	ctx.r[6].s64 = ctx.r[7].s32 as i64;
	// 831FB4D8: ED4D0028  fsubs f10, f13, f0
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 831FB4DC: 88830034  lbz r4, 0x34(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 831FB4E0: 3B810060  addi r28, r1, 0x60
	ctx.r[28].s64 = ctx.r[1].s64 + 96;
	// 831FB4E4: F8C10058  std r6, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u64 ) };
	// 831FB4E8: C9210058  lfd f9, 0x58(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831FB4EC: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 831FB4F0: 1007030C  vspltisb v0, 7
	for i in 0..16 {
		ctx.v[0].u8[i] = 7 as u8;
	}
	// 831FB4F4: 3FA0821A  lis r29, -0x7de6
	ctx.r[29].s64 = -2112225280;
	// 831FB4F8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 831FB4FC: 3CE0821A  lis r7, -0x7de6
	ctx.r[7].s64 = -2112225280;
	// 831FB500: 9881006F  stb r4, 0x6f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(111 as u32), ctx.r[4].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FB740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FB740 size=296
    let mut pc: u32 = 0x831FB740;
    'dispatch: loop {
        match pc {
            0x831FB740 => {
    //   block [0x831FB740..0x831FB868)
	// 831FB740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FB744: 4BFACA19  bl 0x831a815c
	ctx.lr = 0x831FB748;
	sub_831A8130(ctx, base);
	// 831FB748: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FB74C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FB750: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FB754: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 831FB758: 4BBF8689  bl 0x82df3de0
	ctx.lr = 0x831FB75C;
	sub_82DF3DE0(ctx, base);
	// 831FB75C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 831FB760: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 831FB764: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 831FB768: 933F0010  stw r25, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[25].u32 ) };
	// 831FB76C: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 831FB770: 3B8A7688  addi r28, r10, 0x7688
	ctx.r[28].s64 = ctx.r[10].s64 + 30344;
	// 831FB774: 3B4B9FD8  addi r26, r11, -0x6028
	ctx.r[26].s64 = ctx.r[11].s64 + -24616;
	// 831FB778: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 831FB77C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831FB780: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FB784: 4BBF8D55  bl 0x82df44d8
	ctx.lr = 0x831FB788;
	sub_82DF44D8(ctx, base);
	// 831FB788: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831FB78C: 408100A0  ble 0x831fb82c
	if !ctx.cr[0].gt {
	pc = 0x831FB82C; continue 'dispatch;
	}
	// 831FB790: 7CDEE850  subf r6, r30, r29
	ctx.r[6].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 831FB794: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831FB798: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FB79C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 831FB7A0: 4BBF8DC9  bl 0x82df4568
	ctx.lr = 0x831FB7A4;
	sub_82DF4568(ctx, base);
	// 831FB7A4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FB7A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 831FB7AC: 396B001E  addi r11, r11, 0x1e
	ctx.r[11].s64 = ctx.r[11].s64 + 30;
	// 831FB7B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FB7B4: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 831FB7B8: 4BBF8419  bl 0x82df3bd0
	ctx.lr = 0x831FB7BC;
	sub_82DF3BD0(ctx, base);
	// 831FB7BC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 831FB7C0: 4BBF7C69  bl 0x82df3428
	ctx.lr = 0x831FB7C4;
	sub_82DF3428(ctx, base);
	// 831FB7C4: 3BDD0001  addi r30, r29, 1
	ctx.r[30].s64 = ctx.r[29].s64 + 1;
	// 831FB7C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831FB7CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831FB7D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FB7D4: 4BBF8D05  bl 0x82df44d8
	ctx.lr = 0x831FB7D8;
	sub_82DF44D8(ctx, base);
	// 831FB7D8: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831FB7DC: 40810070  ble 0x831fb84c
	if !ctx.cr[0].gt {
	pc = 0x831FB84C; continue 'dispatch;
	}
	// 831FB7E0: 7CDEE850  subf r6, r30, r29
	ctx.r[6].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 831FB7E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831FB7E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FB7EC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 831FB7F0: 4BBF8D79  bl 0x82df4568
	ctx.lr = 0x831FB7F4;
	sub_82DF4568(ctx, base);
	// 831FB7F4: 4BBF79BD  bl 0x82df31b0
	ctx.lr = 0x831FB7F8;
	sub_82DF31B0(ctx, base);
	// 831FB7F8: 4BFB0E51  bl 0x831ac648
	ctx.lr = 0x831FB7FC;
	sub_831AC648(ctx, base);
	// 831FB7FC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FB800: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 831FB804: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 831FB808: 396B0019  addi r11, r11, 0x19
	ctx.r[11].s64 = ctx.r[11].s64 + 25;
	// 831FB80C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FB810: 7C0BFD2E  stfsx f0, r11, r31
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), tmp.u32) };
	// 831FB814: 4BBF7C15  bl 0x82df3428
	ctx.lr = 0x831FB818;
	sub_82DF3428(ctx, base);
	// 831FB818: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FB81C: 3BDD0001  addi r30, r29, 1
	ctx.r[30].s64 = ctx.r[29].s64 + 1;
	// 831FB820: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FB824: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 831FB828: 48000008  b 0x831fb830
	pc = 0x831FB830; continue 'dispatch;
	// 831FB82C: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 831FB830: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 831FB834: 40980010  bge cr6, 0x831fb844
	if !ctx.cr[6].lt {
	pc = 0x831FB844; continue 'dispatch;
	}
	// 831FB838: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FB83C: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 831FB840: 4198FF38  blt cr6, 0x831fb778
	if ctx.cr[6].lt {
	pc = 0x831FB778; continue 'dispatch;
	}
	// 831FB844: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 831FB848: 4800000C  b 0x831fb854
	pc = 0x831FB854; continue 'dispatch;
	// 831FB84C: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 831FB850: 933F0010  stw r25, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[25].u32 ) };
	// 831FB854: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FB858: 4BBF7BD1  bl 0x82df3428
	ctx.lr = 0x831FB85C;
	sub_82DF3428(ctx, base);
	// 831FB85C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FB860: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831FB864: 4BFAC948  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FB868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FB868 size=144
    let mut pc: u32 = 0x831FB868;
    'dispatch: loop {
        match pc {
            0x831FB868 => {
    //   block [0x831FB868..0x831FB8F8)
	// 831FB868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FB86C: 4BFAC901  bl 0x831a816c
	ctx.lr = 0x831FB870;
	sub_831A8130(ctx, base);
	// 831FB870: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FB874: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FB878: 4BC02F21  bl 0x82dfe798
	ctx.lr = 0x831FB87C;
	sub_82DFE798(ctx, base);
	// 831FB87C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831FB880: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 831FB884: 390B2A20  addi r8, r11, 0x2a20
	ctx.r[8].s64 = ctx.r[11].s64 + 10784;
	// 831FB888: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831FB88C: 995F000C  stb r10, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 831FB890: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 831FB894: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 831FB898: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 831FB89C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 831FB8A0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831FB8A4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831FB8A8: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 831FB8AC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831FB8B0: 4080FFF0  bge 0x831fb8a0
	if !ctx.cr[0].lt {
	pc = 0x831FB8A0; continue 'dispatch;
	}
	// 831FB8B4: 397F003C  addi r11, r31, 0x3c
	ctx.r[11].s64 = ctx.r[31].s64 + 60;
	// 831FB8B8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 831FB8BC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831FB8C0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831FB8C4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 831FB8C8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831FB8CC: 4080FFF0  bge 0x831fb8bc
	if !ctx.cr[0].lt {
	pc = 0x831FB8BC; continue 'dispatch;
	}
	// 831FB8D0: 3BBF0078  addi r29, r31, 0x78
	ctx.r[29].s64 = ctx.r[31].s64 + 120;
	// 831FB8D4: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 831FB8D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831FB8DC: 4BBF7815  bl 0x82df30f0
	ctx.lr = 0x831FB8E0;
	sub_82DF30F0(ctx, base);
	// 831FB8E0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831FB8E4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 831FB8E8: 4080FFF0  bge 0x831fb8d8
	if !ctx.cr[0].lt {
	pc = 0x831FB8D8; continue 'dispatch;
	}
	// 831FB8EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FB8F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FB8F4: 4BFAC8C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FB8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FB8F8 size=156
    let mut pc: u32 = 0x831FB8F8;
    'dispatch: loop {
        match pc {
            0x831FB8F8 => {
    //   block [0x831FB8F8..0x831FB994)
	// 831FB8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FB8FC: 4BFAC86D  bl 0x831a8168
	ctx.lr = 0x831FB900;
	sub_831A8130(ctx, base);
	// 831FB900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FB904: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831FB908: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 831FB90C: 3BFC008C  addi r31, r28, 0x8c
	ctx.r[31].s64 = ctx.r[28].s64 + 140;
	// 831FB910: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 831FB914: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FB918: 4BBF7B11  bl 0x82df3428
	ctx.lr = 0x831FB91C;
	sub_82DF3428(ctx, base);
	// 831FB91C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831FB920: 4080FFF0  bge 0x831fb910
	if !ctx.cr[0].lt {
	pc = 0x831FB910; continue 'dispatch;
	}
	// 831FB924: 3BFC0028  addi r31, r28, 0x28
	ctx.r[31].s64 = ctx.r[28].s64 + 40;
	// 831FB928: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 831FB92C: 397F003C  addi r11, r31, 0x3c
	ctx.r[11].s64 = ctx.r[31].s64 + 60;
	// 831FB930: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 831FB934: 3BDEFFF8  addi r30, r30, -8
	ctx.r[30].s64 = ctx.r[30].s64 + -8;
	// 831FB938: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FB93C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831FB940: 419A0008  beq cr6, 0x831fb948
	if ctx.cr[6].eq {
	pc = 0x831FB948; continue 'dispatch;
	}
	// 831FB944: 4B0C4F4D  bl 0x822c0890
	ctx.lr = 0x831FB948;
	sub_822C0890(ctx, base);
	// 831FB948: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831FB94C: 4080FFE8  bge 0x831fb934
	if !ctx.cr[0].lt {
	pc = 0x831FB934; continue 'dispatch;
	}
	// 831FB950: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 831FB954: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 831FB958: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	// 831FB95C: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 831FB960: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FB964: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831FB968: 419A0008  beq cr6, 0x831fb970
	if ctx.cr[6].eq {
	pc = 0x831FB970; continue 'dispatch;
	}
	// 831FB96C: 4B0C4F25  bl 0x822c0890
	ctx.lr = 0x831FB970;
	sub_822C0890(ctx, base);
	// 831FB970: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831FB974: 4080FFE8  bge 0x831fb95c
	if !ctx.cr[0].lt {
	pc = 0x831FB95C; continue 'dispatch;
	}
	// 831FB978: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 831FB97C: 387C0008  addi r3, r28, 8
	ctx.r[3].s64 = ctx.r[28].s64 + 8;
	// 831FB980: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 831FB984: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FB988: 4BBF7AA1  bl 0x82df3428
	ctx.lr = 0x831FB98C;
	sub_82DF3428(ctx, base);
	// 831FB98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FB990: 4BFAC828  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FB998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FB998 size=76
    let mut pc: u32 = 0x831FB998;
    'dispatch: loop {
        match pc {
            0x831FB998 => {
    //   block [0x831FB998..0x831FB9E4)
	// 831FB998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FB99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FB9A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FB9A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FB9A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FB9AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FB9B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FB9B4: 4BFFFF45  bl 0x831fb8f8
	ctx.lr = 0x831FB9B8;
	sub_831FB8F8(ctx, base);
	// 831FB9B8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FB9BC: 4182000C  beq 0x831fb9c8
	if ctx.cr[0].eq {
	pc = 0x831FB9C8; continue 'dispatch;
	}
	// 831FB9C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FB9C4: 4BBF6A15  bl 0x82df23d8
	ctx.lr = 0x831FB9C8;
	sub_82DF23D8(ctx, base);
	// 831FB9C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FB9CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FB9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FB9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FB9D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FB9DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FB9E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FB9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FB9E8 size=48
    let mut pc: u32 = 0x831FB9E8;
    'dispatch: loop {
        match pc {
            0x831FB9E8 => {
    //   block [0x831FB9E8..0x831FBA18)
	// 831FB9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FB9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FB9F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FB9F4: 4BC02E75  bl 0x82dfe868
	ctx.lr = 0x831FB9F8;
	sub_82DFE868(ctx, base);
	// 831FB9F8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 831FB9FC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831FBA00: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831FBA04: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 831FBA08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FBA0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FBA10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FBA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FBA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FBA18 size=444
    let mut pc: u32 = 0x831FBA18;
    'dispatch: loop {
        match pc {
            0x831FBA18 => {
    //   block [0x831FBA18..0x831FBBD4)
	// 831FBA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FBA1C: 4BFAC73D  bl 0x831a8158
	ctx.lr = 0x831FBA20;
	sub_831A8130(ctx, base);
	// 831FBA20: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FBA24: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 831FBA28: 81780010  lwz r11, 0x10(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FBA2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FBA30: 409A000C  bne cr6, 0x831fba3c
	if !ctx.cr[6].eq {
	pc = 0x831FBA3C; continue 'dispatch;
	}
	// 831FBA34: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 831FBA38: 48000010  b 0x831fba48
	pc = 0x831FBA48; continue 'dispatch;
	// 831FBA3C: 81580014  lwz r10, 0x14(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FBA40: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831FBA44: 7D7D1E70  srawi r29, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 831FBA48: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831FBA4C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831FBA50: 4099002C  ble cr6, 0x831fba7c
	if !ctx.cr[6].gt {
	pc = 0x831FBA7C; continue 'dispatch;
	}
	// 831FBA54: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831FBA58: 81780010  lwz r11, 0x10(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FBA5C: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831FBA60: 4BC02D81  bl 0x82dfe7e0
	ctx.lr = 0x831FBA64;
	sub_82DFE7E0(ctx, base);
	// 831FBA64: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FBA68: 41820028  beq 0x831fba90
	if ctx.cr[0].eq {
	pc = 0x831FBA90; continue 'dispatch;
	}
	// 831FBA6C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 831FBA70: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 831FBA74: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 831FBA78: 4198FFE0  blt cr6, 0x831fba58
	if ctx.cr[6].lt {
	pc = 0x831FBA58; continue 'dispatch;
	}
	// 831FBA7C: 81780020  lwz r11, 0x20(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(32 as u32) ) } as u64;
	// 831FBA80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FBA84: 409A0014  bne cr6, 0x831fba98
	if !ctx.cr[6].eq {
	pc = 0x831FBA98; continue 'dispatch;
	}
	// 831FBA88: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 831FBA8C: 48000018  b 0x831fbaa4
	pc = 0x831FBAA4; continue 'dispatch;
	// 831FBA90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831FBA94: 48000138  b 0x831fbbcc
	pc = 0x831FBBCC; continue 'dispatch;
	// 831FBA98: 81580024  lwz r10, 0x24(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(36 as u32) ) } as u64;
	// 831FBA9C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831FBAA0: 7D7D1E70  srawi r29, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 831FBAA4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831FBAA8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831FBAAC: 4099002C  ble cr6, 0x831fbad8
	if !ctx.cr[6].gt {
	pc = 0x831FBAD8; continue 'dispatch;
	}
	// 831FBAB0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831FBAB4: 81780020  lwz r11, 0x20(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(32 as u32) ) } as u64;
	// 831FBAB8: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FBABC: 4BC02D25  bl 0x82dfe7e0
	ctx.lr = 0x831FBAC0;
	sub_82DFE7E0(ctx, base);
	// 831FBAC0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FBAC4: 4182FFCC  beq 0x831fba90
	if ctx.cr[0].eq {
	pc = 0x831FBA90; continue 'dispatch;
	}
	// 831FBAC8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 831FBACC: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 831FBAD0: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 831FBAD4: 4198FFE0  blt cr6, 0x831fbab4
	if ctx.cr[6].lt {
	pc = 0x831FBAB4; continue 'dispatch;
	}
	// 831FBAD8: 81780030  lwz r11, 0x30(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(48 as u32) ) } as u64;
	// 831FBADC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FBAE0: 409A000C  bne cr6, 0x831fbaec
	if !ctx.cr[6].eq {
	pc = 0x831FBAEC; continue 'dispatch;
	}
	// 831FBAE4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 831FBAE8: 48000010  b 0x831fbaf8
	pc = 0x831FBAF8; continue 'dispatch;
	// 831FBAEC: 81580034  lwz r10, 0x34(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(52 as u32) ) } as u64;
	// 831FBAF0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831FBAF4: 7D7D1E70  srawi r29, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 831FBAF8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831FBAFC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831FBB00: 4099002C  ble cr6, 0x831fbb2c
	if !ctx.cr[6].gt {
	pc = 0x831FBB2C; continue 'dispatch;
	}
	// 831FBB04: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831FBB08: 81780030  lwz r11, 0x30(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(48 as u32) ) } as u64;
	// 831FBB0C: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831FBB10: 4BC02CD1  bl 0x82dfe7e0
	ctx.lr = 0x831FBB14;
	sub_82DFE7E0(ctx, base);
	// 831FBB14: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FBB18: 4182FF78  beq 0x831fba90
	if ctx.cr[0].eq {
	pc = 0x831FBA90; continue 'dispatch;
	}
	// 831FBB1C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 831FBB20: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 831FBB24: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 831FBB28: 4198FFE0  blt cr6, 0x831fbb08
	if ctx.cr[6].lt {
	pc = 0x831FBB08; continue 'dispatch;
	}
	// 831FBB2C: 8178004C  lwz r11, 0x4c(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(76 as u32) ) } as u64;
	// 831FBB30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FBB34: 409A000C  bne cr6, 0x831fbb40
	if !ctx.cr[6].eq {
	pc = 0x831FBB40; continue 'dispatch;
	}
	// 831FBB38: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 831FBB3C: 48000010  b 0x831fbb4c
	pc = 0x831FBB4C; continue 'dispatch;
	// 831FBB40: 81580050  lwz r10, 0x50(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FBB44: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831FBB48: 7D792670  srawi r25, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[25].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 831FBB4C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831FBB50: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 831FBB54: 40990074  ble cr6, 0x831fbbc8
	if !ctx.cr[6].gt {
	pc = 0x831FBBC8; continue 'dispatch;
	}
	// 831FBB58: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 831FBB5C: 8178004C  lwz r11, 0x4c(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(76 as u32) ) } as u64;
	// 831FBB60: 7F8BDA14  add r28, r11, r27
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 831FBB64: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FBB68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FBB6C: 409A000C  bne cr6, 0x831fbb78
	if !ctx.cr[6].eq {
	pc = 0x831FBB78; continue 'dispatch;
	}
	// 831FBB70: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 831FBB74: 48000010  b 0x831fbb84
	pc = 0x831FBB84; continue 'dispatch;
	// 831FBB78: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FBB7C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831FBB80: 7D7D1E70  srawi r29, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 831FBB84: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831FBB88: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831FBB8C: 4099002C  ble cr6, 0x831fbbb8
	if !ctx.cr[6].gt {
	pc = 0x831FBBB8; continue 'dispatch;
	}
	// 831FBB90: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831FBB94: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FBB98: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831FBB9C: 4BC02C45  bl 0x82dfe7e0
	ctx.lr = 0x831FBBA0;
	sub_82DFE7E0(ctx, base);
	// 831FBBA0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FBBA4: 4182FEEC  beq 0x831fba90
	if ctx.cr[0].eq {
	pc = 0x831FBA90; continue 'dispatch;
	}
	// 831FBBA8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 831FBBAC: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 831FBBB0: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 831FBBB4: 4198FFE0  blt cr6, 0x831fbb94
	if ctx.cr[6].lt {
	pc = 0x831FBB94; continue 'dispatch;
	}
	// 831FBBB8: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 831FBBBC: 3B7B0010  addi r27, r27, 0x10
	ctx.r[27].s64 = ctx.r[27].s64 + 16;
	// 831FBBC0: 7F1AC800  cmpw cr6, r26, r25
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[25].s32, &mut ctx.xer);
	// 831FBBC4: 4198FF98  blt cr6, 0x831fbb5c
	if ctx.cr[6].lt {
	pc = 0x831FBB5C; continue 'dispatch;
	}
	// 831FBBC8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831FBBCC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831FBBD0: 4BFAC5D8  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FBBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FBBD8 size=144
    let mut pc: u32 = 0x831FBBD8;
    'dispatch: loop {
        match pc {
            0x831FBBD8 => {
    //   block [0x831FBBD8..0x831FBC68)
	// 831FBBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FBBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FBBE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FBBE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FBBE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FBBEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FBBF0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 831FBBF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FBBF8: 4BBF7E11  bl 0x82df3a08
	ctx.lr = 0x831FBBFC;
	sub_82DF3A08(ctx, base);
	// 831FBBFC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FBC00: 4182004C  beq 0x831fbc4c
	if ctx.cr[0].eq {
	pc = 0x831FBC4C; continue 'dispatch;
	}
	// 831FBC04: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831FBC08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FBC0C: 3BCB0C9C  addi r30, r11, 0xc9c
	ctx.r[30].s64 = ctx.r[11].s64 + 3228;
	// 831FBC10: 808B0C9C  lwz r4, 0xc9c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3228 as u32) ) } as u64;
	// 831FBC14: 4BBF7DF5  bl 0x82df3a08
	ctx.lr = 0x831FBC18;
	sub_82DF3A08(ctx, base);
	// 831FBC18: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FBC1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBC20: 4BBF7C91  bl 0x82df38b0
	ctx.lr = 0x831FBC24;
	sub_82DF38B0(ctx, base);
	// 831FBC24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FBC28: 4BBF7801  bl 0x82df3428
	ctx.lr = 0x831FBC2C;
	sub_82DF3428(ctx, base);
	// 831FBC2C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 831FBC30: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FBC34: 4BBF7DD5  bl 0x82df3a08
	ctx.lr = 0x831FBC38;
	sub_82DF3A08(ctx, base);
	// 831FBC38: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 831FBC3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBC40: 4BBF7C71  bl 0x82df38b0
	ctx.lr = 0x831FBC44;
	sub_82DF38B0(ctx, base);
	// 831FBC44: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 831FBC48: 4BBF77E1  bl 0x82df3428
	ctx.lr = 0x831FBC4C;
	sub_82DF3428(ctx, base);
	// 831FBC4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBC50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FBC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FBC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FBC5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FBC60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FBC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FBC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FBC68 size=264
    let mut pc: u32 = 0x831FBC68;
    'dispatch: loop {
        match pc {
            0x831FBC68 => {
    //   block [0x831FBC68..0x831FBD70)
	// 831FBC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FBC6C: 4BFAC501  bl 0x831a816c
	ctx.lr = 0x831FBC70;
	sub_831A8130(ctx, base);
	// 831FBC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FBC74: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831FBC78: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 831FBC7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FBC80: 4BBF7D89  bl 0x82df3a08
	ctx.lr = 0x831FBC84;
	sub_82DF3A08(ctx, base);
	// 831FBC84: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831FBC88: 57AA077B  rlwinm. r10, r29, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831FBC8C: 3BCB0C9C  addi r30, r11, 0xc9c
	ctx.r[30].s64 = ctx.r[11].s64 + 3228;
	// 831FBC90: 41820044  beq 0x831fbcd4
	if ctx.cr[0].eq {
	pc = 0x831FBCD4; continue 'dispatch;
	}
	// 831FBC94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FBC98: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FBC9C: 4BBF7D6D  bl 0x82df3a08
	ctx.lr = 0x831FBCA0;
	sub_82DF3A08(ctx, base);
	// 831FBCA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FBCA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBCA8: 4BBF7C09  bl 0x82df38b0
	ctx.lr = 0x831FBCAC;
	sub_82DF38B0(ctx, base);
	// 831FBCAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FBCB0: 4BBF7779  bl 0x82df3428
	ctx.lr = 0x831FBCB4;
	sub_82DF3428(ctx, base);
	// 831FBCB4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 831FBCB8: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FBCBC: 4BBF7D4D  bl 0x82df3a08
	ctx.lr = 0x831FBCC0;
	sub_82DF3A08(ctx, base);
	// 831FBCC0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 831FBCC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBCC8: 4BBF7BE9  bl 0x82df38b0
	ctx.lr = 0x831FBCCC;
	sub_82DF38B0(ctx, base);
	// 831FBCCC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 831FBCD0: 4BBF7759  bl 0x82df3428
	ctx.lr = 0x831FBCD4;
	sub_82DF3428(ctx, base);
	// 831FBCD4: 57AB07BD  rlwinm. r11, r29, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FBCD8: 41820044  beq 0x831fbd1c
	if ctx.cr[0].eq {
	pc = 0x831FBD1C; continue 'dispatch;
	}
	// 831FBCDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FBCE0: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FBCE4: 4BBF7D25  bl 0x82df3a08
	ctx.lr = 0x831FBCE8;
	sub_82DF3A08(ctx, base);
	// 831FBCE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FBCEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBCF0: 4BBF7BC1  bl 0x82df38b0
	ctx.lr = 0x831FBCF4;
	sub_82DF38B0(ctx, base);
	// 831FBCF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FBCF8: 4BBF7731  bl 0x82df3428
	ctx.lr = 0x831FBCFC;
	sub_82DF3428(ctx, base);
	// 831FBCFC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 831FBD00: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FBD04: 4BBF7D05  bl 0x82df3a08
	ctx.lr = 0x831FBD08;
	sub_82DF3A08(ctx, base);
	// 831FBD08: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 831FBD0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBD10: 4BBF7BA1  bl 0x82df38b0
	ctx.lr = 0x831FBD14;
	sub_82DF38B0(ctx, base);
	// 831FBD14: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 831FBD18: 4BBF7711  bl 0x82df3428
	ctx.lr = 0x831FBD1C;
	sub_82DF3428(ctx, base);
	// 831FBD1C: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FBD20: 41820044  beq 0x831fbd64
	if ctx.cr[0].eq {
	pc = 0x831FBD64; continue 'dispatch;
	}
	// 831FBD24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FBD28: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FBD2C: 4BBF7CDD  bl 0x82df3a08
	ctx.lr = 0x831FBD30;
	sub_82DF3A08(ctx, base);
	// 831FBD30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FBD34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBD38: 4BBF7B79  bl 0x82df38b0
	ctx.lr = 0x831FBD3C;
	sub_82DF38B0(ctx, base);
	// 831FBD3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FBD40: 4BBF76E9  bl 0x82df3428
	ctx.lr = 0x831FBD44;
	sub_82DF3428(ctx, base);
	// 831FBD44: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 831FBD48: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FBD4C: 4BBF7CBD  bl 0x82df3a08
	ctx.lr = 0x831FBD50;
	sub_82DF3A08(ctx, base);
	// 831FBD50: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 831FBD54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBD58: 4BBF7B59  bl 0x82df38b0
	ctx.lr = 0x831FBD5C;
	sub_82DF38B0(ctx, base);
	// 831FBD5C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 831FBD60: 4BBF76C9  bl 0x82df3428
	ctx.lr = 0x831FBD64;
	sub_82DF3428(ctx, base);
	// 831FBD64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBD68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FBD6C: 4BFAC450  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FBD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FBD70 size=72
    let mut pc: u32 = 0x831FBD70;
    'dispatch: loop {
        match pc {
            0x831FBD70 => {
    //   block [0x831FBD70..0x831FBDB8)
	// 831FBD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FBD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FBD78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FBD7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FBD80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FBD84: 4BC02A15  bl 0x82dfe798
	ctx.lr = 0x831FBD88;
	sub_82DFE798(ctx, base);
	// 831FBD88: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831FBD8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FBD90: 394A2A48  addi r10, r10, 0x2a48
	ctx.r[10].s64 = ctx.r[10].s64 + 10824;
	// 831FBD94: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831FBD98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBD9C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831FBDA0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 831FBDA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FBDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FBDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FBDB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FBDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FBDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FBDB8 size=96
    let mut pc: u32 = 0x831FBDB8;
    'dispatch: loop {
        match pc {
            0x831FBDB8 => {
    //   block [0x831FBDB8..0x831FBE18)
	// 831FBDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FBDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FBDC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FBDC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FBDC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FBDCC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831FBDD0: 396B2A48  addi r11, r11, 0x2a48
	ctx.r[11].s64 = ctx.r[11].s64 + 10824;
	// 831FBDD4: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FBDD8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FBDDC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831FBDE0: 419A0010  beq cr6, 0x831fbdf0
	if ctx.cr[6].eq {
	pc = 0x831FBDF0; continue 'dispatch;
	}
	// 831FBDE4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831FBDE8: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 831FBDEC: 4BBF639D  bl 0x82df2188
	ctx.lr = 0x831FBDF0;
	sub_82DF2188(ctx, base);
	// 831FBDF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 831FBDF4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 831FBDF8: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 831FBDFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FBE00: 4BBF7629  bl 0x82df3428
	ctx.lr = 0x831FBE04;
	sub_82DF3428(ctx, base);
	// 831FBE04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FBE08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FBE0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FBE10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FBE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FBE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FBE18 size=76
    let mut pc: u32 = 0x831FBE18;
    'dispatch: loop {
        match pc {
            0x831FBE18 => {
    //   block [0x831FBE18..0x831FBE64)
	// 831FBE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FBE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FBE20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FBE24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FBE28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FBE2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FBE30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FBE34: 4BFFFF85  bl 0x831fbdb8
	ctx.lr = 0x831FBE38;
	sub_831FBDB8(ctx, base);
	// 831FBE38: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FBE3C: 4182000C  beq 0x831fbe48
	if ctx.cr[0].eq {
	pc = 0x831FBE48; continue 'dispatch;
	}
	// 831FBE40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBE44: 4BBF6595  bl 0x82df23d8
	ctx.lr = 0x831FBE48;
	sub_82DF23D8(ctx, base);
	// 831FBE48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBE4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FBE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FBE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FBE58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FBE5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FBE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FBE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FBE68 size=80
    let mut pc: u32 = 0x831FBE68;
    'dispatch: loop {
        match pc {
            0x831FBE68 => {
    //   block [0x831FBE68..0x831FBEB8)
	// 831FBE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FBE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FBE70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FBE74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FBE78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FBE7C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 831FBE80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831FBE84: 396BD0E4  addi r11, r11, -0x2f1c
	ctx.r[11].s64 = ctx.r[11].s64 + -12060;
	// 831FBE88: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 831FBE8C: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 831FBE90: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FBE94: 4BBF725D  bl 0x82df30f0
	ctx.lr = 0x831FBE98;
	sub_82DF30F0(ctx, base);
	// 831FBE98: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 831FBE9C: 4BBF7255  bl 0x82df30f0
	ctx.lr = 0x831FBEA0;
	sub_82DF30F0(ctx, base);
	// 831FBEA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBEA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FBEA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FBEAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FBEB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FBEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FBEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FBEB8 size=168
    let mut pc: u32 = 0x831FBEB8;
    'dispatch: loop {
        match pc {
            0x831FBEB8 => {
    //   block [0x831FBEB8..0x831FBF60)
	// 831FBEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FBEBC: 4BFAC2A5  bl 0x831a8160
	ctx.lr = 0x831FBEC0;
	sub_831A8130(ctx, base);
	// 831FBEC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FBEC4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 831FBEC8: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 831FBECC: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FBED0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FBED4: 409A000C  bne cr6, 0x831fbee0
	if !ctx.cr[6].eq {
	pc = 0x831FBEE0; continue 'dispatch;
	}
	// 831FBED8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 831FBEDC: 48000010  b 0x831fbeec
	pc = 0x831FBEEC; continue 'dispatch;
	// 831FBEE0: 815B0014  lwz r10, 0x14(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FBEE4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831FBEE8: 7D7C1E70  srawi r28, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 831FBEEC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 831FBEF0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 831FBEF4: 419A0060  beq cr6, 0x831fbf54
	if ctx.cr[6].eq {
	pc = 0x831FBF54; continue 'dispatch;
	}
	// 831FBEF8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831FBEFC: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FBF00: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831FBF04: 83CB000C  lwz r30, 0xc(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FBF08: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831FBF0C: 409A0014  bne cr6, 0x831fbf20
	if !ctx.cr[6].eq {
	pc = 0x831FBF20; continue 'dispatch;
	}
	// 831FBF10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831FBF14: 4BC028CD  bl 0x82dfe7e0
	ctx.lr = 0x831FBF18;
	sub_82DFE7E0(ctx, base);
	// 831FBF18: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FBF1C: 41820034  beq 0x831fbf50
	if ctx.cr[0].eq {
	pc = 0x831FBF50; continue 'dispatch;
	}
	// 831FBF20: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FBF24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FBF28: 409A0014  bne cr6, 0x831fbf3c
	if !ctx.cr[6].eq {
	pc = 0x831FBF3C; continue 'dispatch;
	}
	// 831FBF2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831FBF30: 4BC028B1  bl 0x82dfe7e0
	ctx.lr = 0x831FBF34;
	sub_82DFE7E0(ctx, base);
	// 831FBF34: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FBF38: 40820018  bne 0x831fbf50
	if !ctx.cr[0].eq {
	pc = 0x831FBF50; continue 'dispatch;
	}
	// 831FBF3C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 831FBF40: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 831FBF44: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 831FBF48: 4198FFB4  blt cr6, 0x831fbefc
	if ctx.cr[6].lt {
	pc = 0x831FBEFC; continue 'dispatch;
	}
	// 831FBF4C: 48000008  b 0x831fbf54
	pc = 0x831FBF54; continue 'dispatch;
	// 831FBF50: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831FBF54: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 831FBF58: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831FBF5C: 4BFAC254  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FBF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FBF60 size=8
    let mut pc: u32 = 0x831FBF60;
    'dispatch: loop {
        match pc {
            0x831FBF60 => {
    //   block [0x831FBF60..0x831FBF68)
	// 831FBF60: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 831FBF64: 4BBF7C6C  b 0x82df3bd0
	sub_82DF3BD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FBF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FBF68 size=76
    let mut pc: u32 = 0x831FBF68;
    'dispatch: loop {
        match pc {
            0x831FBF68 => {
    //   block [0x831FBF68..0x831FBFB4)
	// 831FBF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FBF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FBF70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FBF74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FBF78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FBF7C: 4BC0281D  bl 0x82dfe798
	ctx.lr = 0x831FBF80;
	sub_82DFE798(ctx, base);
	// 831FBF80: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831FBF84: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 831FBF88: 396B2A50  addi r11, r11, 0x2a50
	ctx.r[11].s64 = ctx.r[11].s64 + 10832;
	// 831FBF8C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 831FBF90: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FBF94: 388A9BC9  addi r4, r10, -0x6437
	ctx.r[4].s64 = ctx.r[10].s64 + -25655;
	// 831FBF98: 4BBF7A71  bl 0x82df3a08
	ctx.lr = 0x831FBF9C;
	sub_82DF3A08(ctx, base);
	// 831FBF9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FBFA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FBFA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FBFA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FBFAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FBFB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FBFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FBFB8 size=156
    let mut pc: u32 = 0x831FBFB8;
    'dispatch: loop {
        match pc {
            0x831FBFB8 => {
    //   block [0x831FBFB8..0x831FC054)
	// 831FBFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FBFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FBFC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FBFC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FBFC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FBFCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FBFD0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831FBFD4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 831FBFD8: 396B2A50  addi r11, r11, 0x2a50
	ctx.r[11].s64 = ctx.r[11].s64 + 10832;
	// 831FBFDC: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 831FBFE0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FBFE4: 388A9BC9  addi r4, r10, -0x6437
	ctx.r[4].s64 = ctx.r[10].s64 + -25655;
	// 831FBFE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FBFEC: 4BBF749D  bl 0x82df3488
	ctx.lr = 0x831FBFF0;
	sub_82DF3488(ctx, base);
	// 831FBFF0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831FBFF4: 4182002C  beq 0x831fc020
	if ctx.cr[0].eq {
	pc = 0x831FC020; continue 'dispatch;
	}
	// 831FBFF8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831FBFFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831FC000: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FC004: 808B271C  lwz r4, 0x271c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10012 as u32) ) } as u64;
	// 831FC008: 4BBF5BF1  bl 0x82df1bf8
	ctx.lr = 0x831FC00C;
	sub_82DF1BF8(ctx, base);
	// 831FC00C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831FC010: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FC014: 4BC547ED  bl 0x82e50800
	ctx.lr = 0x831FC018;
	sub_82E50800(ctx, base);
	// 831FC018: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FC01C: 4BBF5C75  bl 0x82df1c90
	ctx.lr = 0x831FC020;
	sub_82DF1C90(ctx, base);
	// 831FC020: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FC024: 4BBF7405  bl 0x82df3428
	ctx.lr = 0x831FC028;
	sub_82DF3428(ctx, base);
	// 831FC028: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 831FC02C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 831FC030: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 831FC034: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FC038: 4BBF73F1  bl 0x82df3428
	ctx.lr = 0x831FC03C;
	sub_82DF3428(ctx, base);
	// 831FC03C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FC040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FC044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FC048: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FC04C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FC050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FC058 size=76
    let mut pc: u32 = 0x831FC058;
    'dispatch: loop {
        match pc {
            0x831FC058 => {
    //   block [0x831FC058..0x831FC0A4)
	// 831FC058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FC05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FC060: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FC064: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FC068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FC06C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FC070: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FC074: 4BFFFF45  bl 0x831fbfb8
	ctx.lr = 0x831FC078;
	sub_831FBFB8(ctx, base);
	// 831FC078: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FC07C: 4182000C  beq 0x831fc088
	if ctx.cr[0].eq {
	pc = 0x831FC088; continue 'dispatch;
	}
	// 831FC080: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FC084: 4BBF6355  bl 0x82df23d8
	ctx.lr = 0x831FC088;
	sub_82DF23D8(ctx, base);
	// 831FC088: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FC08C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FC090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FC094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FC098: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FC09C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FC0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FC0A8 size=112
    let mut pc: u32 = 0x831FC0A8;
    'dispatch: loop {
        match pc {
            0x831FC0A8 => {
    //   block [0x831FC0A8..0x831FC118)
	// 831FC0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FC0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FC0B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FC0B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FC0B8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 831FC0BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FC0C0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 831FC0C4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 831FC0C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 831FC0CC: 38EA11DC  addi r7, r10, 0x11dc
	ctx.r[7].s64 = ctx.r[10].s64 + 4572;
	// 831FC0D0: 38C918B4  addi r6, r9, 0x18b4
	ctx.r[6].s64 = ctx.r[9].s64 + 6324;
	// 831FC0D4: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 831FC0D8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831FC0DC: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 831FC0E0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 831FC0E4: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 831FC0E8: B0BF0006  sth r5, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 831FC0EC: 88810050  lbz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FC0F0: 480007E1  bl 0x831fc8d0
	ctx.lr = 0x831FC0F4;
	sub_831FC8D0(ctx, base);
	// 831FC0F4: 3C80821A  lis r4, -0x7de6
	ctx.r[4].s64 = -2112225280;
	// 831FC0F8: 38642A5C  addi r3, r4, 0x2a5c
	ctx.r[3].s64 = ctx.r[4].s64 + 10844;
	// 831FC0FC: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 831FC100: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FC104: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FC108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FC10C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FC110: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FC114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FC118 size=64
    let mut pc: u32 = 0x831FC118;
    'dispatch: loop {
        match pc {
            0x831FC118 => {
    //   block [0x831FC118..0x831FC158)
	// 831FC118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FC11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FC120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FC124: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC128: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 831FC12C: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 831FC130: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 831FC134: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FC138: 4BCA45F9  bl 0x82ea0730
	ctx.lr = 0x831FC13C;
	sub_82EA0730(ctx, base);
	// 831FC13C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 831FC140: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 831FC144: 4BFFFF65  bl 0x831fc0a8
	ctx.lr = 0x831FC148;
	sub_831FC0A8(ctx, base);
	// 831FC148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FC14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FC150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FC154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FC158 size=16
    let mut pc: u32 = 0x831FC158;
    'dispatch: loop {
        match pc {
            0x831FC158 => {
    //   block [0x831FC158..0x831FC168)
	// 831FC158: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831FC15C: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC160: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 831FC164: 48000344  b 0x831fc4a8
	sub_831FC4A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FC168 size=112
    let mut pc: u32 = 0x831FC168;
    'dispatch: loop {
        match pc {
            0x831FC168 => {
    //   block [0x831FC168..0x831FC1D8)
	// 831FC168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FC16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FC170: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FC174: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FC178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FC17C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FC180: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 831FC184: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831FC188: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 831FC18C: 480007DD  bl 0x831fc968
	ctx.lr = 0x831FC190;
	sub_831FC968(ctx, base);
	// 831FC190: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FC194: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831FC198: 419A0024  beq cr6, 0x831fc1bc
	if ctx.cr[6].eq {
	pc = 0x831FC1BC; continue 'dispatch;
	}
	// 831FC19C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FC1A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FC1A4: 419A0010  beq cr6, 0x831fc1b4
	if ctx.cr[6].eq {
	pc = 0x831FC1B4; continue 'dispatch;
	}
	// 831FC1A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FC1AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831FC1B0: 4E800421  bctrl
	ctx.lr = 0x831FC1B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FC1B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FC1B8: 48000008  b 0x831fc1c0
	pc = 0x831FC1C0; continue 'dispatch;
	// 831FC1BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831FC1C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FC1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FC1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FC1CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FC1D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FC1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FC1D8 size=104
    let mut pc: u32 = 0x831FC1D8;
    'dispatch: loop {
        match pc {
            0x831FC1D8 => {
    //   block [0x831FC1D8..0x831FC240)
	// 831FC1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FC1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FC1E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FC1E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FC1E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FC1EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FC1F0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 831FC1F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831FC1F8: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 831FC1FC: 4800076D  bl 0x831fc968
	ctx.lr = 0x831FC200;
	sub_831FC968(ctx, base);
	// 831FC200: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FC204: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831FC208: 419A001C  beq cr6, 0x831fc224
	if ctx.cr[6].eq {
	pc = 0x831FC224; continue 'dispatch;
	}
	// 831FC20C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC210: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FC214: 419A0010  beq cr6, 0x831fc224
	if ctx.cr[6].eq {
	pc = 0x831FC224; continue 'dispatch;
	}
	// 831FC218: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FC21C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831FC220: 4E800421  bctrl
	ctx.lr = 0x831FC224;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FC224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FC228: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FC22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FC230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FC234: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FC238: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FC23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FC240 size=160
    let mut pc: u32 = 0x831FC240;
    'dispatch: loop {
        match pc {
            0x831FC240 => {
    //   block [0x831FC240..0x831FC2E0)
	// 831FC240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FC244: 4BFABF25  bl 0x831a8168
	ctx.lr = 0x831FC248;
	sub_831A8130(ctx, base);
	// 831FC248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FC24C: 3BC40008  addi r30, r4, 8
	ctx.r[30].s64 = ctx.r[4].s64 + 8;
	// 831FC250: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831FC254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FC258: 48000179  bl 0x831fc3d0
	ctx.lr = 0x831FC25C;
	sub_831FC3D0(ctx, base);
	// 831FC25C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FC260: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831FC264: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831FC268: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FC26C: 4800021D  bl 0x831fc488
	ctx.lr = 0x831FC270;
	sub_831FC488(ctx, base);
	// 831FC270: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FC274: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FC278: 419A0060  beq cr6, 0x831fc2d8
	if ctx.cr[6].eq {
	pc = 0x831FC2D8; continue 'dispatch;
	}
	// 831FC27C: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 831FC280: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FC284: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FC288: 480001A1  bl 0x831fc428
	ctx.lr = 0x831FC28C;
	sub_831FC428(ctx, base);
	// 831FC28C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831FC290: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FC294: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FC298: 48000171  bl 0x831fc408
	ctx.lr = 0x831FC29C;
	sub_831FC408(ctx, base);
	// 831FC29C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 831FC2A0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831FC2A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831FC2A8: 48000201  bl 0x831fc4a8
	ctx.lr = 0x831FC2AC;
	sub_831FC4A8(ctx, base);
	// 831FC2AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FC2B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FC2B4: 48000195  bl 0x831fc448
	ctx.lr = 0x831FC2B8;
	sub_831FC448(ctx, base);
	// 831FC2B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FC2BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831FC2C0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831FC2C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FC2C8: 480001C1  bl 0x831fc488
	ctx.lr = 0x831FC2CC;
	sub_831FC488(ctx, base);
	// 831FC2CC: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FC2D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FC2D4: 409AFFAC  bne cr6, 0x831fc280
	if !ctx.cr[6].eq {
	pc = 0x831FC280; continue 'dispatch;
	}
	// 831FC2D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FC2DC: 4BFABEDC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FC2E0 size=116
    let mut pc: u32 = 0x831FC2E0;
    'dispatch: loop {
        match pc {
            0x831FC2E0 => {
    //   block [0x831FC2E0..0x831FC354)
	// 831FC2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FC2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FC2E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FC2EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FC2F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FC2F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FC2F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FC2FC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 831FC300: 48000639  bl 0x831fc938
	ctx.lr = 0x831FC304;
	sub_831FC938(ctx, base);
	// 831FC304: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 831FC308: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 831FC30C: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 831FC310: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831FC314: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831FC318: 419A0020  beq cr6, 0x831fc338
	if ctx.cr[6].eq {
	pc = 0x831FC338; continue 'dispatch;
	}
	// 831FC31C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC320: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 831FC324: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 831FC328: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FC32C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FC330: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FC334: 4BCA447D  bl 0x82ea07b0
	ctx.lr = 0x831FC338;
	sub_82EA07B0(ctx, base);
	// 831FC338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FC33C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FC340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FC344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FC348: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FC34C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FC350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FC358 size=120
    let mut pc: u32 = 0x831FC358;
    'dispatch: loop {
        match pc {
            0x831FC358 => {
    //   block [0x831FC358..0x831FC3D0)
	// 831FC358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FC35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FC360: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FC364: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FC368: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC36C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 831FC370: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 831FC374: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 831FC378: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FC37C: 4BCA43B5  bl 0x82ea0730
	ctx.lr = 0x831FC380;
	sub_82EA0730(ctx, base);
	// 831FC380: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 831FC384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FC388: 3D00821A  lis r8, -0x7de6
	ctx.r[8].s64 = -2112225280;
	// 831FC38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831FC390: 38C82A8C  addi r6, r8, 0x2a8c
	ctx.r[6].s64 = ctx.r[8].s64 + 10892;
	// 831FC394: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 831FC398: 98E90000  stb r7, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 831FC39C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831FC3A0: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 831FC3A4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 831FC3A8: B09F0006  sth r4, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[4].u16 ) };
	// 831FC3AC: B0BF0004  sth r5, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u16 ) };
	// 831FC3B0: 88810050  lbz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FC3B4: 4800051D  bl 0x831fc8d0
	ctx.lr = 0x831FC3B8;
	sub_831FC8D0(ctx, base);
	// 831FC3B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FC3BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FC3C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FC3C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FC3C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FC3CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FC3D0 size=20
    let mut pc: u32 = 0x831FC3D0;
    'dispatch: loop {
        match pc {
            0x831FC3D0 => {
    //   block [0x831FC3D0..0x831FC3E4)
	// 831FC3D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831FC3D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831FC3D8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC3DC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831FC3E0: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC3E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FC3E4 size=16
    let mut pc: u32 = 0x831FC3E4;
    'dispatch: loop {
        match pc {
            0x831FC3E4 => {
    //   block [0x831FC3E4..0x831FC3F4)
	// 831FC3E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC3E8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC3EC: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 831FC3F0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC3F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FC3F4 size=20
    let mut pc: u32 = 0x831FC3F4;
    'dispatch: loop {
        match pc {
            0x831FC3F4 => {
    //   block [0x831FC3F4..0x831FC408)
	// 831FC3F4: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 831FC3F8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831FC3FC: 7F035000  cmpw cr6, r3, r10
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831FC400: 4099FFE8  ble cr6, 0x831fc3e8
	if !ctx.cr[6].gt {
		sub_831FC3E4(ctx, base);
		return;
	}
	// 831FC404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FC408 size=28
    let mut pc: u32 = 0x831FC408;
    'dispatch: loop {
        match pc {
            0x831FC408 => {
    //   block [0x831FC408..0x831FC424)
	// 831FC408: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC40C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC410: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 831FC414: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 831FC418: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FC41C: 7C68502E  lwzx r3, r8, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 831FC420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FC428 size=32
    let mut pc: u32 = 0x831FC428;
    'dispatch: loop {
        match pc {
            0x831FC428 => {
    //   block [0x831FC428..0x831FC448)
	// 831FC428: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC42C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC430: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 831FC434: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FC438: 7D0B2214  add r8, r11, r4
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 831FC43C: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FC440: 7C67502E  lwzx r3, r7, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 831FC444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FC448 size=20
    let mut pc: u32 = 0x831FC448;
    'dispatch: loop {
        match pc {
            0x831FC448 => {
    //   block [0x831FC448..0x831FC45C)
	// 831FC448: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831FC44C: 38640001  addi r3, r4, 1
	ctx.r[3].s64 = ctx.r[4].s64 + 1;
	// 831FC450: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC454: 7F034800  cmpw cr6, r3, r9
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831FC458: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC45C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FC45C size=24
    let mut pc: u32 = 0x831FC45C;
    'dispatch: loop {
        match pc {
            0x831FC45C => {
    //   block [0x831FC45C..0x831FC474)
	// 831FC45C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC460: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FC464: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831FC468: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC46C: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 831FC470: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC474(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FC474 size=20
    let mut pc: u32 = 0x831FC474;
    'dispatch: loop {
        match pc {
            0x831FC474 => {
    //   block [0x831FC474..0x831FC488)
	// 831FC474: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 831FC478: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831FC47C: 7F034800  cmpw cr6, r3, r9
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831FC480: 4099FFE8  ble cr6, 0x831fc468
	if !ctx.cr[6].gt {
		sub_831FC45C(ctx, base);
		return;
	}
	// 831FC484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FC488 size=28
    let mut pc: u32 = 0x831FC488;
    'dispatch: loop {
        match pc {
            0x831FC488 => {
    //   block [0x831FC488..0x831FC4A4)
	// 831FC488: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC48C: 7F055800  cmpw cr6, r5, r11
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831FC490: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831FC494: 40990008  ble cr6, 0x831fc49c
	if !ctx.cr[6].gt {
	pc = 0x831FC49C; continue 'dispatch;
	}
	// 831FC498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FC49C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 831FC4A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FC4A8 size=312
    let mut pc: u32 = 0x831FC4A8;
    'dispatch: loop {
        match pc {
            0x831FC4A8 => {
    //   block [0x831FC4A8..0x831FC5E0)
	// 831FC4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FC4AC: 4BFABCB9  bl 0x831a8164
	ctx.lr = 0x831FC4B0;
	sub_831A8130(ctx, base);
	// 831FC4B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FC4B4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 831FC4B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FC4BC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 831FC4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FC4C4: 895C0000  lbz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC4C8: 7D490774  extsb r9, r10
	ctx.r[9].s64 = ctx.r[10].s8 as i64;
	// 831FC4CC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831FC4D0: 419A0028  beq cr6, 0x831fc4f8
	if ctx.cr[6].eq {
	pc = 0x831FC4F8; continue 'dispatch;
	}
	// 831FC4D4: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 831FC4D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831FC4DC: 55682834  slwi r8, r11, 5
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FC4E0: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 831FC4E4: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC4E8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 831FC4EC: 7CE90774  extsb r9, r7
	ctx.r[9].s64 = ctx.r[7].s8 as i64;
	// 831FC4F0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831FC4F4: 409AFFE4  bne cr6, 0x831fc4d8
	if !ctx.cr[6].eq {
	pc = 0x831FC4D8; continue 'dispatch;
	}
	// 831FC4F8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FC4FC: 557D007E  clrlwi r29, r11, 1
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 831FC500: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC504: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FC508: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831FC50C: 40990014  ble cr6, 0x831fc520
	if !ctx.cr[6].gt {
	pc = 0x831FC520; continue 'dispatch;
	}
	// 831FC510: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FC514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FC518: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 831FC51C: 4800047D  bl 0x831fc998
	ctx.lr = 0x831FC520;
	sub_831FC998(ctx, base);
	// 831FC520: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC524: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC528: 7D5EE838  and r30, r10, r29
	ctx.r[30].u64 = ctx.r[10].u64 & ctx.r[29].u64;
	// 831FC52C: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FC530: 7D0B482E  lwzx r8, r11, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 831FC534: 2F08FFFF  cmpwi cr6, r8, -1
	ctx.cr[6].compare_i32(ctx.r[8].s32, -1, &mut ctx.xer);
	// 831FC538: 419A0054  beq cr6, 0x831fc58c
	if ctx.cr[6].eq {
	pc = 0x831FC58C; continue 'dispatch;
	}
	// 831FC53C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC540: 7D69582E  lwzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FC544: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 831FC548: 409A0024  bne cr6, 0x831fc56c
	if !ctx.cr[6].eq {
	pc = 0x831FC56C; continue 'dispatch;
	}
	// 831FC54C: 7D6AF214  add r11, r10, r30
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 831FC550: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831FC554: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FC558: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FC55C: 7C8A482E  lwzx r4, r10, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 831FC560: 4BCADE91  bl 0x82eaa3f0
	ctx.lr = 0x831FC564;
	sub_82EAA3F0(ctx, base);
	// 831FC564: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FC568: 419A0030  beq cr6, 0x831fc598
	if ctx.cr[6].eq {
	pc = 0x831FC598; continue 'dispatch;
	}
	// 831FC56C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC570: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 831FC574: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC578: 7D7E5038  and r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 831FC57C: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FC580: 7D0B482E  lwzx r8, r11, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 831FC584: 2F08FFFF  cmpwi cr6, r8, -1
	ctx.cr[6].compare_i32(ctx.r[8].s32, -1, &mut ctx.xer);
	// 831FC588: 409AFFB4  bne cr6, 0x831fc53c
	if !ctx.cr[6].eq {
	pc = 0x831FC53C; continue 'dispatch;
	}
	// 831FC58C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FC590: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FC594: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831FC598: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC59C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FC5A0: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 831FC5A4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC5A8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC5AC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 831FC5B0: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 831FC5B4: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FC5B8: 7F87492E  stwx r28, r7, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32), ctx.r[28].u32) };
	// 831FC5BC: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC5C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC5C4: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 831FC5C8: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FC5CC: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 831FC5D0: 5483103A  slwi r3, r4, 2
	ctx.r[3].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 831FC5D4: 7F63312E  stwx r27, r3, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[6].u32), ctx.r[27].u32) };
	// 831FC5D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FC5DC: 4BFABBD8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FC5E0 size=208
    let mut pc: u32 = 0x831FC5E0;
    'dispatch: loop {
        match pc {
            0x831FC5E0 => {
    //   block [0x831FC5E0..0x831FC6B0)
	// 831FC5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FC5E4: 4BFABB85  bl 0x831a8168
	ctx.lr = 0x831FC5E8;
	sub_831A8130(ctx, base);
	// 831FC5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FC5EC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 831FC5F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831FC5F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FC5F8: 895C0000  lbz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC5FC: 7D490774  extsb r9, r10
	ctx.r[9].s64 = ctx.r[10].s8 as i64;
	// 831FC600: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831FC604: 419A0028  beq cr6, 0x831fc62c
	if ctx.cr[6].eq {
	pc = 0x831FC62C; continue 'dispatch;
	}
	// 831FC608: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 831FC60C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831FC610: 55682834  slwi r8, r11, 5
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FC614: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 831FC618: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC61C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 831FC620: 7CE90774  extsb r9, r7
	ctx.r[9].s64 = ctx.r[7].s8 as i64;
	// 831FC624: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831FC628: 409AFFE4  bne cr6, 0x831fc60c
	if !ctx.cr[6].eq {
	pc = 0x831FC60C; continue 'dispatch;
	}
	// 831FC62C: 557D007E  clrlwi r29, r11, 1
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 831FC630: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC634: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC638: 7D7FE838  and r31, r11, r29
	ctx.r[31].u64 = ctx.r[11].u64 & ctx.r[29].u64;
	// 831FC63C: 57E9103A  slwi r9, r31, 2
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FC640: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 831FC644: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 831FC648: 419A004C  beq cr6, 0x831fc694
	if ctx.cr[6].eq {
	pc = 0x831FC694; continue 'dispatch;
	}
	// 831FC64C: 7F09E840  cmplw cr6, r9, r29
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[29].u32, &mut ctx.xer);
	// 831FC650: 409A0024  bne cr6, 0x831fc674
	if !ctx.cr[6].eq {
	pc = 0x831FC674; continue 'dispatch;
	}
	// 831FC654: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 831FC658: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831FC65C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FC660: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FC664: 7C89502E  lwzx r4, r9, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 831FC668: 4BCADD89  bl 0x82eaa3f0
	ctx.lr = 0x831FC66C;
	sub_82EAA3F0(ctx, base);
	// 831FC66C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FC670: 419A0034  beq cr6, 0x831fc6a4
	if ctx.cr[6].eq {
	pc = 0x831FC6A4; continue 'dispatch;
	}
	// 831FC674: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC678: 393F0001  addi r9, r31, 1
	ctx.r[9].s64 = ctx.r[31].s64 + 1;
	// 831FC67C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC680: 7D3F5838  and r31, r9, r11
	ctx.r[31].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 831FC684: 57E8103A  slwi r8, r31, 2
	ctx.r[8].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FC688: 7D28502E  lwzx r9, r8, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 831FC68C: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 831FC690: 409AFFBC  bne cr6, 0x831fc64c
	if !ctx.cr[6].eq {
	pc = 0x831FC64C; continue 'dispatch;
	}
	// 831FC694: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC698: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 831FC69C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FC6A0: 4BFABB18  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 831FC6A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FC6A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FC6AC: 4BFABB0C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FC6B0 size=136
    let mut pc: u32 = 0x831FC6B0;
    'dispatch: loop {
        match pc {
            0x831FC6B0 => {
    //   block [0x831FC6B0..0x831FC738)
	// 831FC6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FC6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FC6B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FC6BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FC6C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FC6C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FC6C8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 831FC6CC: 4BFFFF15  bl 0x831fc5e0
	ctx.lr = 0x831FC6D0;
	sub_831FC5E0(ctx, base);
	// 831FC6D0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC6D4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 831FC6D8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831FC6DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831FC6E0: 40990008  ble cr6, 0x831fc6e8
	if !ctx.cr[6].gt {
	pc = 0x831FC6E8; continue 'dispatch;
	}
	// 831FC6E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FC6E8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831FC6EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FC6F0: 419A002C  beq cr6, 0x831fc71c
	if ctx.cr[6].eq {
	pc = 0x831FC71C; continue 'dispatch;
	}
	// 831FC6F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC6F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831FC6FC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC700: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 831FC704: 550B083C  slwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FC708: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831FC70C: 54E6103A  slwi r6, r7, 2
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831FC710: 7CA6482E  lwzx r5, r6, r9
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 831FC714: 90BE0000  stw r5, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 831FC718: 48000008  b 0x831fc720
	pc = 0x831FC720; continue 'dispatch;
	// 831FC71C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831FC720: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FC724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FC728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FC72C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FC730: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FC734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FC738 size=124
    let mut pc: u32 = 0x831FC738;
    'dispatch: loop {
        match pc {
            0x831FC738 => {
    //   block [0x831FC738..0x831FC7B4)
	// 831FC738: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 831FC73C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FC740: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC744: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 831FC748: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FC74C: 38EBFFFF  addi r7, r11, -1
	ctx.r[7].s64 = ctx.r[11].s64 + -1;
	// 831FC750: 90E30004  stw r7, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 831FC754: 7CA9412E  stwx r5, r9, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[5].u32) };
	// 831FC758: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC75C: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC760: 7CC95214  add r6, r9, r10
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 831FC764: 7CCB4838  and r11, r6, r9
	ctx.r[11].u64 = ctx.r[6].u64 & ctx.r[9].u64;
	// 831FC768: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 831FC76C: 7CE4402E  lwzx r7, r4, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 831FC770: 2F07FFFF  cmpwi cr6, r7, -1
	ctx.cr[6].compare_i32(ctx.r[7].s32, -1, &mut ctx.xer);
	// 831FC774: 419A0020  beq cr6, 0x831fc794
	if ctx.cr[6].eq {
	pc = 0x831FC794; continue 'dispatch;
	}
	// 831FC778: 5507003E  slwi r7, r8, 0
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FC77C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 831FC780: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 831FC784: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831FC788: 7C86382E  lwzx r4, r6, r7
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 831FC78C: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 831FC790: 409AFFEC  bne cr6, 0x831fc77c
	if !ctx.cr[6].eq {
	pc = 0x831FC77C; continue 'dispatch;
	}
	// 831FC794: 38EA0001  addi r7, r10, 1
	ctx.r[7].s64 = ctx.r[10].s64 + 1;
	// 831FC798: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 831FC79C: 7CEB4838  and r11, r7, r9
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 831FC7A0: 7CC44838  and r4, r6, r9
	ctx.r[4].u64 = ctx.r[6].u64 & ctx.r[9].u64;
	// 831FC7A4: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FC7A8: 7D08382E  lwzx r8, r8, r7
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 831FC7AC: 2F08FFFF  cmpwi cr6, r8, -1
	ctx.cr[6].compare_i32(ctx.r[8].s32, -1, &mut ctx.xer);
	// 831FC7B0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC7B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FC7B4 size=208
    let mut pc: u32 = 0x831FC7B4;
    'dispatch: loop {
        match pc {
            0x831FC7B4 => {
    //   block [0x831FC7B4..0x831FC884)
	// 831FC7B4: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC7B8: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 831FC7BC: 7CC8382E  lwzx r6, r8, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 831FC7C0: 7CC94838  and r9, r6, r9
	ctx.r[9].u64 = ctx.r[6].u64 & ctx.r[9].u64;
	// 831FC7C4: 4198000C  blt cr6, 0x831fc7d0
	if ctx.cr[6].lt {
	pc = 0x831FC7D0; continue 'dispatch;
	}
	// 831FC7C8: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831FC7CC: 41990094  bgt cr6, 0x831fc860
	if ctx.cr[6].gt {
	pc = 0x831FC860; continue 'dispatch;
	}
	// 831FC7D0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831FC7D4: 40980014  bge cr6, 0x831fc7e8
	if !ctx.cr[6].lt {
	pc = 0x831FC7E8; continue 'dispatch;
	}
	// 831FC7D8: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831FC7DC: 41990084  bgt cr6, 0x831fc860
	if ctx.cr[6].gt {
	pc = 0x831FC860; continue 'dispatch;
	}
	// 831FC7E0: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831FC7E4: 4099007C  ble cr6, 0x831fc860
	if !ctx.cr[6].gt {
	pc = 0x831FC860; continue 'dispatch;
	}
	// 831FC7E8: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831FC7EC: 4099000C  ble cr6, 0x831fc7f8
	if !ctx.cr[6].gt {
	pc = 0x831FC7F8; continue 'dispatch;
	}
	// 831FC7F0: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 831FC7F4: 4198006C  blt cr6, 0x831fc860
	if ctx.cr[6].lt {
	pc = 0x831FC860; continue 'dispatch;
	}
	// 831FC7F8: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FC7FC: 7CC9412E  stwx r6, r9, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[6].u32) };
	// 831FC800: 80C30000  lwz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC804: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC808: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 831FC80C: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 831FC810: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 831FC814: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831FC818: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FC81C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FC820: 7D08302E  lwzx r8, r8, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 831FC824: 7D09312E  stwx r8, r9, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[6].u32), ctx.r[8].u32) };
	// 831FC828: 80C30000  lwz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC82C: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC830: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FC834: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 831FC838: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 831FC83C: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 831FC840: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 831FC844: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FC848: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FC84C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 831FC850: 7D29302E  lwzx r9, r9, r6
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 831FC854: 7D28312E  stwx r9, r8, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32), ctx.r[9].u32) };
	// 831FC858: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC85C: 7CA7412E  stwx r5, r7, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32), ctx.r[5].u32) };
	// 831FC860: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC864: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FC868: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC86C: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 831FC870: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FC874: 7CC7402E  lwzx r6, r7, r8
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 831FC878: 2F06FFFF  cmpwi cr6, r6, -1
	ctx.cr[6].compare_i32(ctx.r[6].s32, -1, &mut ctx.xer);
	// 831FC87C: 409AFF38  bne cr6, 0x831fc7b4
	if !ctx.cr[6].eq {
	pc = 0x831FC7B4; continue 'dispatch;
	}
	// 831FC880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FC888 size=68
    let mut pc: u32 = 0x831FC888;
    'dispatch: loop {
        match pc {
            0x831FC888 => {
    //   block [0x831FC888..0x831FC8CC)
	// 831FC888: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC88C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831FC890: 352B0001  addic. r9, r11, 1
	ctx.xer.ca = (ctx.r[11].u32 > (!(1 as u32)));
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831FC894: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 831FC898: 4081002C  ble 0x831fc8c4
	if !ctx.cr[0].gt {
	pc = 0x831FC8C4; continue 'dispatch;
	}
	// 831FC89C: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 831FC8A0: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 831FC8A4: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC8A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831FC8AC: 7D0B492E  stwx r8, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 831FC8B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831FC8B4: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC8B8: 38C90001  addi r6, r9, 1
	ctx.r[6].s64 = ctx.r[9].s64 + 1;
	// 831FC8BC: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 831FC8C0: 4198FFE4  blt cr6, 0x831fc8a4
	if ctx.cr[6].lt {
	pc = 0x831FC8A4; continue 'dispatch;
	}
	// 831FC8C4: 90E30004  stw r7, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 831FC8C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FC8D0 size=100
    let mut pc: u32 = 0x831FC8D0;
    'dispatch: loop {
        match pc {
            0x831FC8D0 => {
    //   block [0x831FC8D0..0x831FC934)
	// 831FC8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FC8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FC8D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FC8DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FC8E0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC8E4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 831FC8E8: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 831FC8EC: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 831FC8F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FC8F4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FC8F8: 4BCA3E39  bl 0x82ea0730
	ctx.lr = 0x831FC8FC;
	sub_82EA0730(ctx, base);
	// 831FC8FC: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 831FC900: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 831FC904: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 831FC908: 4BCADE51  bl 0x82eaa758
	ctx.lr = 0x831FC90C;
	sub_82EAA758(ctx, base);
	// 831FC90C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831FC910: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 831FC914: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 831FC918: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FC91C: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 831FC920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FC924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FC928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FC92C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FC930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FC938 size=44
    let mut pc: u32 = 0x831FC938;
    'dispatch: loop {
        match pc {
            0x831FC938 => {
    //   block [0x831FC938..0x831FC964)
	// 831FC938: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC93C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 831FC940: 810D0000  lwz r8, 0(r13)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC944: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 831FC948: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FC94C: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC950: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FC954: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831FC958: 7C69402E  lwzx r3, r9, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 831FC95C: 54E5103A  slwi r5, r7, 2
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831FC960: 4BCA3E50  b 0x82ea07b0
	sub_82EA07B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FC968 size=48
    let mut pc: u32 = 0x831FC968;
    'dispatch: loop {
        match pc {
            0x831FC968 => {
    //   block [0x831FC968..0x831FC998)
	// 831FC968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FC96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FC970: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FC974: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 831FC978: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831FC97C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 831FC980: 4BFFFD31  bl 0x831fc6b0
	ctx.lr = 0x831FC984;
	sub_831FC6B0(ctx, base);
	// 831FC984: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FC988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FC98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FC990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FC994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FC998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FC998 size=216
    let mut pc: u32 = 0x831FC998;
    'dispatch: loop {
        match pc {
            0x831FC998 => {
    //   block [0x831FC998..0x831FCA70)
	// 831FC998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FC99C: 4BFAB7BD  bl 0x831a8158
	ctx.lr = 0x831FC9A0;
	sub_831A8130(ctx, base);
	// 831FC9A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FC9A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831FC9A8: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC9AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FC9B0: 57AA083C  slwi r10, r29, 1
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FC9B4: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 831FC9B8: 7D5D5214  add r10, r29, r10
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[10].u64;
	// 831FC9BC: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 831FC9C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FC9C4: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 831FC9C8: 837F0000  lwz r27, 0(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FC9CC: 7C78C82E  lwzx r3, r24, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 831FC9D0: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 831FC9D4: 4BCA3D5D  bl 0x82ea0730
	ctx.lr = 0x831FC9D8;
	sub_82EA0730(ctx, base);
	// 831FC9D8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 831FC9DC: 57A5103A  slwi r5, r29, 2
	ctx.r[5].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831FC9E0: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 831FC9E4: 4BCADD75  bl 0x82eaa758
	ctx.lr = 0x831FC9E8;
	sub_82EAA758(ctx, base);
	// 831FC9E8: 393DFFFF  addi r9, r29, -1
	ctx.r[9].s64 = ctx.r[29].s64 + -1;
	// 831FC9EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 831FC9F0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 831FC9F4: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 831FC9F8: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 831FC9FC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831FCA00: 4099004C  ble cr6, 0x831fca4c
	if !ctx.cr[6].gt {
	pc = 0x831FCA4C; continue 'dispatch;
	}
	// 831FCA04: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FCA08: 7F7ADB78  mr r26, r27
	ctx.r[26].u64 = ctx.r[27].u64;
	// 831FCA0C: 7F8BDA14  add r28, r11, r27
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 831FCA10: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FCA14: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 831FCA18: 419A0020  beq cr6, 0x831fca38
	if ctx.cr[6].eq {
	pc = 0x831FCA38; continue 'dispatch;
	}
	// 831FCA1C: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FCA20: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FCA24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCA28: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 831FCA2C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FCA30: 7CAAD82E  lwzx r5, r10, r27
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 831FCA34: 4BFFFA75  bl 0x831fc4a8
	ctx.lr = 0x831FCA38;
	sub_831FC4A8(ctx, base);
	// 831FCA38: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 831FCA3C: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 831FCA40: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 831FCA44: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 831FCA48: 4198FFC8  blt cr6, 0x831fca10
	if ctx.cr[6].lt {
	pc = 0x831FCA10; continue 'dispatch;
	}
	// 831FCA4C: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FCA50: 7C78C82E  lwzx r3, r24, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 831FCA54: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 831FCA58: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 831FCA5C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831FCA60: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831FCA64: 4BCA3D4D  bl 0x82ea07b0
	ctx.lr = 0x831FCA68;
	sub_82EA07B0(ctx, base);
	// 831FCA68: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831FCA6C: 4BFAB73C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FCA70 size=232
    let mut pc: u32 = 0x831FCA70;
    'dispatch: loop {
        match pc {
            0x831FCA70 => {
    //   block [0x831FCA70..0x831FCB58)
	// 831FCA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FCA74: 4BFAB6ED  bl 0x831a8160
	ctx.lr = 0x831FCA78;
	sub_831A8130(ctx, base);
	// 831FCA78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FCA7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FCA80: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 831FCA84: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 831FCA88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831FCA8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FCA90: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831FCA94: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 831FCA98: 4BCAF141  bl 0x82eabbd8
	ctx.lr = 0x831FCA9C;
	sub_82EABBD8(ctx, base);
	// 831FCA9C: 7C6BD838  and r11, r3, r27
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[27].u64;
	// 831FCAA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FCAA4: 409A00AC  bne cr6, 0x831fcb50
	if !ctx.cr[6].eq {
	pc = 0x831FCB50; continue 'dispatch;
	}
	// 831FCAA8: 7C65DB78  or r5, r3, r27
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[27].u64;
	// 831FCAAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FCAB0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831FCAB4: 4BCAEFF5  bl 0x82eabaa8
	ctx.lr = 0x831FCAB8;
	sub_82EABAA8(ctx, base);
	// 831FCAB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCABC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831FCAC0: 4BCAB3F9  bl 0x82ea7eb8
	ctx.lr = 0x831FCAC4;
	sub_82EA7EB8(ctx, base);
	// 831FCAC4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FCAC8: 40990050  ble cr6, 0x831fcb18
	if !ctx.cr[6].gt {
	pc = 0x831FCB18; continue 'dispatch;
	}
	// 831FCACC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831FCAD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCAD4: 4BCAB3ED  bl 0x82ea7ec0
	ctx.lr = 0x831FCAD8;
	sub_82EA7EC0(ctx, base);
	// 831FCAD8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831FCADC: 4BCB13F5  bl 0x82eaded0
	ctx.lr = 0x831FCAE0;
	sub_82EADED0(ctx, base);
	// 831FCAE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831FCAE4: 419A0014  beq cr6, 0x831fcaf8
	if ctx.cr[6].eq {
	pc = 0x831FCAF8; continue 'dispatch;
	}
	// 831FCAE8: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 831FCAEC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 831FCAF0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831FCAF4: 4BFFFF7D  bl 0x831fca70
	ctx.lr = 0x831FCAF8;
	sub_831FCA70(ctx, base);
	// 831FCAF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831FCAFC: 7F4903A6  mtctr r26
	ctx.ctr.u64 = ctx.r[26].u64;
	// 831FCB00: 4E800421  bctrl
	ctx.lr = 0x831FCB04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FCB04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCB08: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 831FCB0C: 4BCAB3AD  bl 0x82ea7eb8
	ctx.lr = 0x831FCB10;
	sub_82EA7EB8(ctx, base);
	// 831FCB10: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 831FCB14: 4198FFB8  blt cr6, 0x831fcacc
	if ctx.cr[6].lt {
	pc = 0x831FCACC; continue 'dispatch;
	}
	// 831FCB18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCB1C: 4BCAB0FD  bl 0x82ea7c18
	ctx.lr = 0x831FCB20;
	sub_82EA7C18(ctx, base);
	// 831FCB20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831FCB24: 419A002C  beq cr6, 0x831fcb50
	if ctx.cr[6].eq {
	pc = 0x831FCB50; continue 'dispatch;
	}
	// 831FCB28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCB2C: 4BCAB0ED  bl 0x82ea7c18
	ctx.lr = 0x831FCB30;
	sub_82EA7C18(ctx, base);
	// 831FCB30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FCB34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831FCB38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FCB3C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831FCB40: 4BCAF099  bl 0x82eabbd8
	ctx.lr = 0x831FCB44;
	sub_82EABBD8(ctx, base);
	// 831FCB44: 7C6BD838  and r11, r3, r27
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[27].u64;
	// 831FCB48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FCB4C: 419AFF5C  beq cr6, 0x831fcaa8
	if ctx.cr[6].eq {
	pc = 0x831FCAA8; continue 'dispatch;
	}
	// 831FCB50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831FCB54: 4BFAB65C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FCB58 size=12
    let mut pc: u32 = 0x831FCB58;
    'dispatch: loop {
        match pc {
            0x831FCB58 => {
    //   block [0x831FCB58..0x831FCB64)
	// 831FCB58: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FCB5C: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 831FCB60: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCB64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FCB64 size=12
    let mut pc: u32 = 0x831FCB64;
    'dispatch: loop {
        match pc {
            0x831FCB64 => {
    //   block [0x831FCB64..0x831FCB70)
	// 831FCB64: 8963000D  lbz r11, 0xd(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831FCB68: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 831FCB6C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FCB70 size=20
    let mut pc: u32 = 0x831FCB70;
    'dispatch: loop {
        match pc {
            0x831FCB70 => {
    //   block [0x831FCB70..0x831FCB84)
	// 831FCB70: 3960001D  li r11, 0x1d
	ctx.r[11].s64 = 29;
	// 831FCB74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831FCB78: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 831FCB7C: 9943000D  stb r10, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 831FCB80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FCB88 size=344
    let mut pc: u32 = 0x831FCB88;
    'dispatch: loop {
        match pc {
            0x831FCB88 => {
    //   block [0x831FCB88..0x831FCCE0)
	// 831FCB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FCB8C: 4BFAB5CD  bl 0x831a8158
	ctx.lr = 0x831FCB90;
	sub_831A8130(ctx, base);
	// 831FCB90: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FCB94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831FCB98: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 831FCB9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831FCBA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831FCBA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831FCBA8: 4BCAF031  bl 0x82eabbd8
	ctx.lr = 0x831FCBAC;
	sub_82EABBD8(ctx, base);
	// 831FCBAC: 546B07FE  clrlwi r11, r3, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	// 831FCBB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FCBB4: 409A0124  bne cr6, 0x831fccd8
	if !ctx.cr[6].eq {
	pc = 0x831FCCD8; continue 'dispatch;
	}
	// 831FCBB8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 831FCBBC: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831FCBC0: 3D208343  lis r9, -0x7cbd
	ctx.r[9].s64 = -2092761088;
	// 831FCBC4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831FCBC8: 3B0BFB6C  addi r24, r11, -0x494
	ctx.r[24].s64 = ctx.r[11].s64 + -1172;
	// 831FCBCC: 3B2A2AA8  addi r25, r10, 0x2aa8
	ctx.r[25].s64 = ctx.r[10].s64 + 10920;
	// 831FCBD0: 3B69D5E8  addi r27, r9, -0x2a18
	ctx.r[27].s64 = ctx.r[9].s64 + -10776;
	// 831FCBD4: 60650001  ori r5, r3, 1
	ctx.r[5].u64 = ctx.r[3].u64 | 1;
	// 831FCBD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831FCBDC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831FCBE0: 4BCAEEC9  bl 0x82eabaa8
	ctx.lr = 0x831FCBE4;
	sub_82EABAA8(ctx, base);
	// 831FCBE4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 831FCBE8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831FCBEC: 4BCAB2ED  bl 0x82ea7ed8
	ctx.lr = 0x831FCBF0;
	sub_82EA7ED8(ctx, base);
	// 831FCBF0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831FCBF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FCBF8: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 831FCBFC: A3AB0012  lhz r29, 0x12(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 831FCC00: 4BCAB019  bl 0x82ea7c18
	ctx.lr = 0x831FCC04;
	sub_82EA7C18(ctx, base);
	// 831FCC04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831FCC08: 419A0020  beq cr6, 0x831fcc28
	if ctx.cr[6].eq {
	pc = 0x831FCC28; continue 'dispatch;
	}
	// 831FCC0C: 7F5FE92E  stwx r26, r31, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32), ctx.r[26].u32) };
	// 831FCC10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCC14: 4BCAB005  bl 0x82ea7c18
	ctx.lr = 0x831FCC18;
	sub_82EA7C18(ctx, base);
	// 831FCC18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FCC1C: 4BCAAFFD  bl 0x82ea7c18
	ctx.lr = 0x831FCC20;
	sub_82EA7C18(ctx, base);
	// 831FCC20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831FCC24: 409AFFE8  bne cr6, 0x831fcc0c
	if !ctx.cr[6].eq {
	pc = 0x831FCC0C; continue 'dispatch;
	}
	// 831FCC28: 7D7FE8AE  lbzx r11, r31, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 831FCC2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FCC30: 419A0020  beq cr6, 0x831fcc50
	if ctx.cr[6].eq {
	pc = 0x831FCC50; continue 'dispatch;
	}
	// 831FCC34: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 831FCC38: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831FCC3C: 4BCAB29D  bl 0x82ea7ed8
	ctx.lr = 0x831FCC40;
	sub_82EA7ED8(ctx, base);
	// 831FCC40: A1430012  lhz r10, 0x12(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 831FCC44: 7D6AF82E  lwzx r11, r10, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831FCC48: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 831FCC4C: 7D2AF92E  stwx r9, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[9].u32) };
	// 831FCC50: 7F5FE92E  stwx r26, r31, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32), ctx.r[26].u32) };
	// 831FCC54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FCC58: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 831FCC5C: 4BCAB25D  bl 0x82ea7eb8
	ctx.lr = 0x831FCC60;
	sub_82EA7EB8(ctx, base);
	// 831FCC60: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FCC64: 4099003C  ble cr6, 0x831fcca0
	if !ctx.cr[6].gt {
	pc = 0x831FCCA0; continue 'dispatch;
	}
	// 831FCC68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FCC6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FCC70: 4BCAB251  bl 0x82ea7ec0
	ctx.lr = 0x831FCC74;
	sub_82EA7EC0(ctx, base);
	// 831FCC74: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FCC78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FCC7C: 419A0010  beq cr6, 0x831fcc8c
	if ctx.cr[6].eq {
	pc = 0x831FCC8C; continue 'dispatch;
	}
	// 831FCC80: 4BCB1249  bl 0x82eadec8
	ctx.lr = 0x831FCC84;
	sub_82EADEC8(ctx, base);
	// 831FCC84: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831FCC88: 4BFFFF01  bl 0x831fcb88
	ctx.lr = 0x831FCC8C;
	sub_831FCB88(ctx, base);
	// 831FCC8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FCC90: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 831FCC94: 4BCAB225  bl 0x82ea7eb8
	ctx.lr = 0x831FCC98;
	sub_82EA7EB8(ctx, base);
	// 831FCC98: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 831FCC9C: 4198FFCC  blt cr6, 0x831fcc68
	if ctx.cr[6].lt {
	pc = 0x831FCC68; continue 'dispatch;
	}
	// 831FCCA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FCCA4: 4BCAAF75  bl 0x82ea7c18
	ctx.lr = 0x831FCCA8;
	sub_82EA7C18(ctx, base);
	// 831FCCA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831FCCAC: 419A002C  beq cr6, 0x831fccd8
	if ctx.cr[6].eq {
	pc = 0x831FCCD8; continue 'dispatch;
	}
	// 831FCCB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FCCB4: 4BCAAF65  bl 0x82ea7c18
	ctx.lr = 0x831FCCB8;
	sub_82EA7C18(ctx, base);
	// 831FCCB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831FCCBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831FCCC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831FCCC4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831FCCC8: 4BCAEF11  bl 0x82eabbd8
	ctx.lr = 0x831FCCCC;
	sub_82EABBD8(ctx, base);
	// 831FCCCC: 546B07FE  clrlwi r11, r3, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	// 831FCCD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FCCD4: 419AFF00  beq cr6, 0x831fcbd4
	if ctx.cr[6].eq {
	pc = 0x831FCBD4; continue 'dispatch;
	}
	// 831FCCD8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831FCCDC: 4BFAB4CC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FCCE0 size=172
    let mut pc: u32 = 0x831FCCE0;
    'dispatch: loop {
        match pc {
            0x831FCCE0 => {
    //   block [0x831FCCE0..0x831FCD8C)
	// 831FCCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FCCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FCCE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FCCEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FCCF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FCCF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FCCF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FCCFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831FCD00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FCD04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FCD08: 4BCAEED1  bl 0x82eabbd8
	ctx.lr = 0x831FCD0C;
	sub_82EABBD8(ctx, base);
	// 831FCD0C: 546B077A  rlwinm r11, r3, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 831FCD10: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FCD14: 409A0060  bne cr6, 0x831fcd74
	if !ctx.cr[6].eq {
	pc = 0x831FCD74; continue 'dispatch;
	}
	// 831FCD18: 60650004  ori r5, r3, 4
	ctx.r[5].u64 = ctx.r[3].u64 | 4;
	// 831FCD1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FCD20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FCD24: 4BCAED85  bl 0x82eabaa8
	ctx.lr = 0x831FCD28;
	sub_82EABAA8(ctx, base);
	// 831FCD28: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 831FCD2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCD30: 3BCBAB70  addi r30, r11, -0x5490
	ctx.r[30].s64 = ctx.r[11].s64 + -21648;
	// 831FCD34: 4BCAAED5  bl 0x82ea7c08
	ctx.lr = 0x831FCD38;
	sub_82EA7C08(ctx, base);
	// 831FCD38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831FCD3C: 4BCAD6B5  bl 0x82eaa3f0
	ctx.lr = 0x831FCD40;
	sub_82EAA3F0(ctx, base);
	// 831FCD40: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FCD44: 409A0030  bne cr6, 0x831fcd74
	if !ctx.cr[6].eq {
	pc = 0x831FCD74; continue 'dispatch;
	}
	// 831FCD48: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 831FCD4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCD50: 4BCAB171  bl 0x82ea7ec0
	ctx.lr = 0x831FCD54;
	sub_82EA7EC0(ctx, base);
	// 831FCD54: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 831FCD58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FCD5C: 386B6CAC  addi r3, r11, 0x6cac
	ctx.r[3].s64 = ctx.r[11].s64 + 27820;
	// 831FCD60: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 831FCD64: 4BCAB0E5  bl 0x82ea7e48
	ctx.lr = 0x831FCD68;
	sub_82EA7E48(ctx, base);
	// 831FCD68: A1430012  lhz r10, 0x12(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 831FCD6C: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 831FCD70: 7D2AF9AE  stbx r9, r10, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[9].u8) };
	// 831FCD74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FCD78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FCD7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FCD80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FCD84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FCD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FCD90 size=60
    let mut pc: u32 = 0x831FCD90;
    'dispatch: loop {
        match pc {
            0x831FCD90 => {
    //   block [0x831FCD90..0x831FCDCC)
	// 831FCD90: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FCD94: 2B0B0013  cmplwi cr6, r11, 0x13
	ctx.cr[6].compare_u32(ctx.r[11].u32, 19 as u32, &mut ctx.xer);
	// 831FCD98: 409A0020  bne cr6, 0x831fcdb8
	if !ctx.cr[6].eq {
	pc = 0x831FCDB8; continue 'dispatch;
	}
	// 831FCD9C: 8963000D  lbz r11, 0xd(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 831FCDA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831FCDA4: 9943000D  stb r10, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 831FCDA8: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 831FCDAC: A1230010  lhz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FCDB0: 61280400  ori r8, r9, 0x400
	ctx.r[8].u64 = ctx.r[9].u64 | 1024;
	// 831FCDB4: B1030010  sth r8, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u16 ) };
	// 831FCDB8: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FCDBC: 2B0B0018  cmplwi cr6, r11, 0x18
	ctx.cr[6].compare_u32(ctx.r[11].u32, 24 as u32, &mut ctx.xer);
	// 831FCDC0: 419A000C  beq cr6, 0x831fcdcc
	if ctx.cr[6].eq {
		sub_831FCDCC(ctx, base);
		return;
	}
	// 831FCDC4: 2B0B001F  cmplwi cr6, r11, 0x1f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 31 as u32, &mut ctx.xer);
	// 831FCDC8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCDCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FCDCC size=100
    let mut pc: u32 = 0x831FCDCC;
    'dispatch: loop {
        match pc {
            0x831FCDCC => {
    //   block [0x831FCDCC..0x831FCE30)
	// 831FCDCC: A1430010  lhz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FCDD0: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 831FCDD4: 55490738  rlwinm r9, r10, 0, 0x1c, 0x1c
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 831FCDD8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 831FCDDC: 419A001C  beq cr6, 0x831fcdf8
	if ctx.cr[6].eq {
	pc = 0x831FCDF8; continue 'dispatch;
	}
	// 831FCDE0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 831FCDE4: 9943000D  stb r10, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 831FCDE8: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FCDEC: 5528043E  clrlwi r8, r9, 0x10
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 831FCDF0: 55080776  rlwinm r8, r8, 0, 0x1d, 0x1b
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 831FCDF4: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831FCDF8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FCDFC: 554906F6  rlwinm r9, r10, 0, 0x1b, 0x1b
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 831FCE00: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 831FCE04: 419A001C  beq cr6, 0x831fce20
	if ctx.cr[6].eq {
	pc = 0x831FCE20; continue 'dispatch;
	}
	// 831FCE08: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 831FCE0C: 9943000D  stb r10, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 831FCE10: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FCE14: 5528043E  clrlwi r8, r9, 0x10
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 831FCE18: 55080734  rlwinm r8, r8, 0, 0x1c, 0x1a
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 831FCE1C: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831FCE20: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FCE24: 554906B4  rlwinm r9, r10, 0, 0x1a, 0x1a
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 831FCE28: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 831FCE2C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FCE30 size=28
    let mut pc: u32 = 0x831FCE30;
    'dispatch: loop {
        match pc {
            0x831FCE30 => {
    //   block [0x831FCE30..0x831FCE4C)
	// 831FCE30: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 831FCE34: 9943000D  stb r10, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 831FCE38: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FCE3C: 5528043E  clrlwi r8, r9, 0x10
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 831FCE40: 550806F2  rlwinm r8, r8, 0, 0x1b, 0x19
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 831FCE44: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831FCE48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FCE50 size=124
    let mut pc: u32 = 0x831FCE50;
    'dispatch: loop {
        match pc {
            0x831FCE50 => {
    //   block [0x831FCE50..0x831FCECC)
	// 831FCE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FCE54: 4BFAB319  bl 0x831a816c
	ctx.lr = 0x831FCE58;
	sub_831A8130(ctx, base);
	// 831FCE58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FCE5C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831FCE60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FCE64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FCE68: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 831FCE6C: 409A0018  bne cr6, 0x831fce84
	if !ctx.cr[6].eq {
	pc = 0x831FCE84; continue 'dispatch;
	}
	// 831FCE70: 4BFFFD19  bl 0x831fcb88
	ctx.lr = 0x831FCE74;
	sub_831FCB88(ctx, base);
	// 831FCE74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831FCE78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCE7C: 4BFFFE65  bl 0x831fcce0
	ctx.lr = 0x831FCE80;
	sub_831FCCE0(ctx, base);
	// 831FCE80: 4800000C  b 0x831fce8c
	pc = 0x831FCE8C; continue 'dispatch;
	// 831FCE84: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 831FCE88: 4098001C  bge cr6, 0x831fcea4
	if !ctx.cr[6].lt {
	pc = 0x831FCEA4; continue 'dispatch;
	}
	// 831FCE8C: 3D608320  lis r11, -0x7ce0
	ctx.r[11].s64 = -2095054848;
	// 831FCE90: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 831FCE94: 38CBCB58  addi r6, r11, -0x34a8
	ctx.r[6].s64 = ctx.r[11].s64 + -13480;
	// 831FCE98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831FCE9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCEA0: 4BFFFBD1  bl 0x831fca70
	ctx.lr = 0x831FCEA4;
	sub_831FCA70(ctx, base);
	// 831FCEA4: 2F1D0005  cmpwi cr6, r29, 5
	ctx.cr[6].compare_i32(ctx.r[29].s32, 5, &mut ctx.xer);
	// 831FCEA8: 4098001C  bge cr6, 0x831fcec4
	if !ctx.cr[6].lt {
	pc = 0x831FCEC4; continue 'dispatch;
	}
	// 831FCEAC: 3D608320  lis r11, -0x7ce0
	ctx.r[11].s64 = -2095054848;
	// 831FCEB0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 831FCEB4: 38CBCD90  addi r6, r11, -0x3270
	ctx.r[6].s64 = ctx.r[11].s64 + -12912;
	// 831FCEB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831FCEBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCEC0: 4BFFFBB1  bl 0x831fca70
	ctx.lr = 0x831FCEC4;
	sub_831FCA70(ctx, base);
	// 831FCEC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FCEC8: 4BFAB2F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FCED0 size=192
    let mut pc: u32 = 0x831FCED0;
    'dispatch: loop {
        match pc {
            0x831FCED0 => {
    //   block [0x831FCED0..0x831FCF90)
	// 831FCED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FCED4: 4BFAB295  bl 0x831a8168
	ctx.lr = 0x831FCED8;
	sub_831A8130(ctx, base);
	// 831FCED8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FCEDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831FCEE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FCEE4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 831FCEE8: 4BCAEB09  bl 0x82eab9f0
	ctx.lr = 0x831FCEEC;
	sub_82EAB9F0(ctx, base);
	// 831FCEEC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FCEF0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831FCEF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FCEF8: 419A0088  beq cr6, 0x831fcf80
	if ctx.cr[6].eq {
	pc = 0x831FCF80; continue 'dispatch;
	}
	// 831FCEFC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 831FCF00: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FCF04: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 831FCF08: 409A0020  bne cr6, 0x831fcf28
	if !ctx.cr[6].eq {
	pc = 0x831FCF28; continue 'dispatch;
	}
	// 831FCF0C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FCF10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCF14: 4BFFFC75  bl 0x831fcb88
	ctx.lr = 0x831FCF18;
	sub_831FCB88(ctx, base);
	// 831FCF18: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FCF1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCF20: 4BFFFDC1  bl 0x831fcce0
	ctx.lr = 0x831FCF24;
	sub_831FCCE0(ctx, base);
	// 831FCF24: 4800000C  b 0x831fcf30
	pc = 0x831FCF30; continue 'dispatch;
	// 831FCF28: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 831FCF2C: 4098001C  bge cr6, 0x831fcf48
	if !ctx.cr[6].lt {
	pc = 0x831FCF48; continue 'dispatch;
	}
	// 831FCF30: 3D608320  lis r11, -0x7ce0
	ctx.r[11].s64 = -2095054848;
	// 831FCF34: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 831FCF38: 38CBCB58  addi r6, r11, -0x34a8
	ctx.r[6].s64 = ctx.r[11].s64 + -13480;
	// 831FCF3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FCF40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCF44: 4BFFFB2D  bl 0x831fca70
	ctx.lr = 0x831FCF48;
	sub_831FCA70(ctx, base);
	// 831FCF48: 2F1C0005  cmpwi cr6, r28, 5
	ctx.cr[6].compare_i32(ctx.r[28].s32, 5, &mut ctx.xer);
	// 831FCF4C: 4098001C  bge cr6, 0x831fcf68
	if !ctx.cr[6].lt {
	pc = 0x831FCF68; continue 'dispatch;
	}
	// 831FCF50: 3D608320  lis r11, -0x7ce0
	ctx.r[11].s64 = -2095054848;
	// 831FCF54: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 831FCF58: 38CBCD90  addi r6, r11, -0x3270
	ctx.r[6].s64 = ctx.r[11].s64 + -12912;
	// 831FCF5C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FCF60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FCF64: 4BFFFB0D  bl 0x831fca70
	ctx.lr = 0x831FCF68;
	sub_831FCA70(ctx, base);
	// 831FCF68: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 831FCF6C: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FCF70: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 831FCF74: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FCF78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831FCF7C: 409AFF84  bne cr6, 0x831fcf00
	if !ctx.cr[6].eq {
	pc = 0x831FCF00; continue 'dispatch;
	}
	// 831FCF80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FCF84: 4BCAEAED  bl 0x82eaba70
	ctx.lr = 0x831FCF88;
	sub_82EABA70(ctx, base);
	// 831FCF88: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831FCF8C: 4BFAB22C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FCF90 size=20
    let mut pc: u32 = 0x831FCF90;
    'dispatch: loop {
        match pc {
            0x831FCF90 => {
    //   block [0x831FCF90..0x831FCFA4)
	// 831FCF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FCF94: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 831FCF98: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FCF9C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831FCFA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FCFA8 size=4
    let mut pc: u32 = 0x831FCFA8;
    'dispatch: loop {
        match pc {
            0x831FCFA8 => {
    //   block [0x831FCFA8..0x831FCFAC)
	// 831FCFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FCFB0 size=16
    let mut pc: u32 = 0x831FCFB0;
    'dispatch: loop {
        match pc {
            0x831FCFB0 => {
    //   block [0x831FCFB0..0x831FCFC0)
	// 831FCFB0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 831FCFB4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831FCFB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FCFBC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FCFC0 size=20
    let mut pc: u32 = 0x831FCFC0;
    'dispatch: loop {
        match pc {
            0x831FCFC0 => {
    //   block [0x831FCFC0..0x831FCFD4)
	// 831FCFC0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FCFC4: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 831FCFC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FCFCC: 409AFFF4  bne cr6, 0x831fcfc0
	if !ctx.cr[6].eq {
	pc = 0x831FCFC0; continue 'dispatch;
	}
	// 831FCFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FCFD8 size=16
    let mut pc: u32 = 0x831FCFD8;
    'dispatch: loop {
        match pc {
            0x831FCFD8 => {
    //   block [0x831FCFD8..0x831FCFE8)
	// 831FCFD8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FCFDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831FCFE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FCFE4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FCFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FCFE8 size=20
    let mut pc: u32 = 0x831FCFE8;
    'dispatch: loop {
        match pc {
            0x831FCFE8 => {
    //   block [0x831FCFE8..0x831FCFFC)
	// 831FCFE8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FCFEC: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 831FCFF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FCFF4: 409AFFF4  bne cr6, 0x831fcfe8
	if !ctx.cr[6].eq {
	pc = 0x831FCFE8; continue 'dispatch;
	}
	// 831FCFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FD000 size=8
    let mut pc: u32 = 0x831FD000;
    'dispatch: loop {
        match pc {
            0x831FD000 => {
    //   block [0x831FD000..0x831FD008)
	// 831FD000: 38640014  addi r3, r4, 0x14
	ctx.r[3].s64 = ctx.r[4].s64 + 20;
	// 831FD004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FD008 size=8
    let mut pc: u32 = 0x831FD008;
    'dispatch: loop {
        match pc {
            0x831FD008 => {
    //   block [0x831FD008..0x831FD010)
	// 831FD008: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FD010 size=8
    let mut pc: u32 = 0x831FD010;
    'dispatch: loop {
        match pc {
            0x831FD010 => {
    //   block [0x831FD010..0x831FD018)
	// 831FD010: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FD018 size=56
    let mut pc: u32 = 0x831FD018;
    'dispatch: loop {
        match pc {
            0x831FD018 => {
    //   block [0x831FD018..0x831FD050)
	// 831FD018: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD01C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831FD020: 409A002C  bne cr6, 0x831fd04c
	if !ctx.cr[6].eq {
	pc = 0x831FD04C; continue 'dispatch;
	}
	// 831FD024: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD028: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FD02C: 409A001C  bne cr6, 0x831fd048
	if !ctx.cr[6].eq {
	pc = 0x831FD048; continue 'dispatch;
	}
	// 831FD030: 80840010  lwz r4, 0x10(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FD034: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831FD038: 419A0018  beq cr6, 0x831fd050
	if ctx.cr[6].eq {
		sub_831FD050(ctx, base);
		return;
	}
	// 831FD03C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD040: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FD044: 419AFFEC  beq cr6, 0x831fd030
	if ctx.cr[6].eq {
	pc = 0x831FD030; continue 'dispatch;
	}
	// 831FD048: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FD050 size=8
    let mut pc: u32 = 0x831FD050;
    'dispatch: loop {
        match pc {
            0x831FD050 => {
    //   block [0x831FD050..0x831FD058)
	// 831FD050: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831FD054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FD058 size=8
    let mut pc: u32 = 0x831FD058;
    'dispatch: loop {
        match pc {
            0x831FD058 => {
    //   block [0x831FD058..0x831FD060)
	// 831FD058: 80640010  lwz r3, 0x10(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FD05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FD060 size=8
    let mut pc: u32 = 0x831FD060;
    'dispatch: loop {
        match pc {
            0x831FD060 => {
    //   block [0x831FD060..0x831FD068)
	// 831FD060: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FD068 size=236
    let mut pc: u32 = 0x831FD068;
    'dispatch: loop {
        match pc {
            0x831FD068 => {
    //   block [0x831FD068..0x831FD154)
	// 831FD068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FD06C: 4BFAB0F9  bl 0x831a8164
	ctx.lr = 0x831FD070;
	sub_831A8130(ctx, base);
	// 831FD070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FD074: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD078: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 831FD07C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 831FD080: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FD084: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 831FD088: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 831FD08C: 389D0014  addi r4, r29, 0x14
	ctx.r[4].s64 = ctx.r[29].s64 + 20;
	// 831FD090: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831FD094: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FD098: 4BCA3699  bl 0x82ea0730
	ctx.lr = 0x831FD09C;
	sub_82EA0730(ctx, base);
	// 831FD09C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831FD0A0: 419A0024  beq cr6, 0x831fd0c4
	if ctx.cr[6].eq {
	pc = 0x831FD0C4; continue 'dispatch;
	}
	// 831FD0A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FD0A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FD0AC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FD0B0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831FD0B4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831FD0B8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831FD0BC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 831FD0C0: 48000008  b 0x831fd0c8
	pc = 0x831FD0C8; continue 'dispatch;
	// 831FD0C4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831FD0C8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831FD0CC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831FD0D0: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 831FD0D4: 4BCAD675  bl 0x82eaa748
	ctx.lr = 0x831FD0D8;
	sub_82EAA748(ctx, base);
	// 831FD0D8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831FD0DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FD0E0: 419A0040  beq cr6, 0x831fd120
	if ctx.cr[6].eq {
	pc = 0x831FD120; continue 'dispatch;
	}
	// 831FD0E4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FD0E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FD0EC: 419A0020  beq cr6, 0x831fd10c
	if ctx.cr[6].eq {
	pc = 0x831FD10C; continue 'dispatch;
	}
	// 831FD0F0: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 831FD0F4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FD0F8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831FD0FC: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 831FD100: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 831FD104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FD108: 4BFAB0AC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831FD10C: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 831FD110: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 831FD114: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 831FD118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FD11C: 4BFAB098  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831FD120: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FD124: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FD128: 419A001C  beq cr6, 0x831fd144
	if ctx.cr[6].eq {
	pc = 0x831FD144; continue 'dispatch;
	}
	// 831FD12C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831FD130: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FD134: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 831FD138: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 831FD13C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FD140: 4BFAB074  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831FD144: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 831FD148: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 831FD14C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FD150: 4BFAB064  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FD158 size=452
    let mut pc: u32 = 0x831FD158;
    'dispatch: loop {
        match pc {
            0x831FD158 => {
    //   block [0x831FD158..0x831FD31C)
	// 831FD158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FD15C: 4BFAB001  bl 0x831a815c
	ctx.lr = 0x831FD160;
	sub_831A8130(ctx, base);
	// 831FD160: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FD164: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831FD168: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 831FD16C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 831FD170: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD174: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FD178: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 831FD17C: 409A0008  bne cr6, 0x831fd184
	if !ctx.cr[6].eq {
	pc = 0x831FD184; continue 'dispatch;
	}
	// 831FD180: 833F000C  lwz r25, 0xc(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FD184: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FD188: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831FD18C: 419A000C  beq cr6, 0x831fd198
	if ctx.cr[6].eq {
	pc = 0x831FD198; continue 'dispatch;
	}
	// 831FD190: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831FD194: 48000028  b 0x831fd1bc
	pc = 0x831FD1BC; continue 'dispatch;
	// 831FD198: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FD19C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831FD1A0: 419A000C  beq cr6, 0x831fd1ac
	if ctx.cr[6].eq {
	pc = 0x831FD1AC; continue 'dispatch;
	}
	// 831FD1A4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FD1A8: 48000014  b 0x831fd1bc
	pc = 0x831FD1BC; continue 'dispatch;
	// 831FD1AC: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD1B0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831FD1B4: 409A0008  bne cr6, 0x831fd1bc
	if !ctx.cr[6].eq {
	pc = 0x831FD1BC; continue 'dispatch;
	}
	// 831FD1B8: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FD1BC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD1C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FD1C4: 419A0010  beq cr6, 0x831fd1d4
	if ctx.cr[6].eq {
	pc = 0x831FD1D4; continue 'dispatch;
	}
	// 831FD1C8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FD1CC: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 831FD1D0: 48000030  b 0x831fd200
	pc = 0x831FD200; continue 'dispatch;
	// 831FD1D4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FD1D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FD1DC: 419A0010  beq cr6, 0x831fd1ec
	if ctx.cr[6].eq {
	pc = 0x831FD1EC; continue 'dispatch;
	}
	// 831FD1E0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FD1E4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 831FD1E8: 48000018  b 0x831fd200
	pc = 0x831FD200; continue 'dispatch;
	// 831FD1EC: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FD1F0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831FD1F4: 409A000C  bne cr6, 0x831fd200
	if !ctx.cr[6].eq {
	pc = 0x831FD200; continue 'dispatch;
	}
	// 831FD1F8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FD1FC: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831FD200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FD204: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 831FD208: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 831FD20C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 831FD210: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 831FD214: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FD218: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831FD21C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 831FD220: 4BCA9661  bl 0x82ea6880
	ctx.lr = 0x831FD224;
	sub_82EA6880(ctx, base);
	// 831FD224: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831FD228: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FD22C: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FD230: 7FE7412E  stwx r31, r7, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32), ctx.r[31].u32) };
	// 831FD234: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831FD238: 35660001  addic. r11, r6, 1
	ctx.xer.ca = (ctx.r[6].u32 > (!(1 as u32)));
	ctx.r[11].s64 = ctx.r[6].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FD23C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831FD240: 418200A4  beq 0x831fd2e4
	if ctx.cr[0].eq {
	pc = 0x831FD2E4; continue 'dispatch;
	}
	// 831FD244: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD248: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 831FD24C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FD250: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FD254: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831FD258: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 831FD25C: 83EAFFFC  lwz r31, -4(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831FD260: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 831FD264: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD268: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831FD26C: 7F4903A6  mtctr r26
	ctx.ctr.u64 = ctx.r[26].u64;
	// 831FD270: 4E800421  bctrl
	ctx.lr = 0x831FD274;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD274: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD278: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 831FD27C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 831FD280: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FD284: 38AB0014  addi r5, r11, 0x14
	ctx.r[5].s64 = ctx.r[11].s64 + 20;
	// 831FD288: 4BCA3529  bl 0x82ea07b0
	ctx.lr = 0x831FD28C;
	sub_82EA07B0(ctx, base);
	// 831FD28C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831FD290: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831FD294: 419A0048  beq cr6, 0x831fd2dc
	if ctx.cr[6].eq {
	pc = 0x831FD2DC; continue 'dispatch;
	}
	// 831FD298: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 831FD29C: 554900BE  clrlwi r9, r10, 2
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 831FD2A0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831FD2A4: 409A0014  bne cr6, 0x831fd2b8
	if !ctx.cr[6].eq {
	pc = 0x831FD2B8; continue 'dispatch;
	}
	// 831FD2A8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 831FD2AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FD2B0: 4BCA95D1  bl 0x82ea6880
	ctx.lr = 0x831FD2B4;
	sub_82EA6880(ctx, base);
	// 831FD2B4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831FD2B8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FD2BC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FD2C0: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 831FD2C4: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831FD2C8: 39680001  addi r11, r8, 1
	ctx.r[11].s64 = ctx.r[8].s64 + 1;
	// 831FD2CC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831FD2D0: 83DE0008  lwz r30, 8(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD2D4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831FD2D8: 409AFFC0  bne cr6, 0x831fd298
	if !ctx.cr[6].eq {
	pc = 0x831FD298; continue 'dispatch;
	}
	// 831FD2DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FD2E0: 409AFF6C  bne cr6, 0x831fd24c
	if !ctx.cr[6].eq {
	pc = 0x831FD24C; continue 'dispatch;
	}
	// 831FD2E4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 831FD2E8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831FD2EC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831FD2F0: 409A0020  bne cr6, 0x831fd310
	if !ctx.cr[6].eq {
	pc = 0x831FD310; continue 'dispatch;
	}
	// 831FD2F4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD2F8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 831FD2FC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 831FD300: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FD304: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831FD308: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 831FD30C: 4BCA34A5  bl 0x82ea07b0
	ctx.lr = 0x831FD310;
	sub_82EA07B0(ctx, base);
	// 831FD310: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 831FD314: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831FD318: 4BFAAE94  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FD320 size=128
    let mut pc: u32 = 0x831FD320;
    'dispatch: loop {
        match pc {
            0x831FD320 => {
    //   block [0x831FD320..0x831FD3A0)
	// 831FD320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FD324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FD328: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FD32C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FD330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FD334: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FD338: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FD33C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD340: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD344: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FD348: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FD34C: 4E800421  bctrl
	ctx.lr = 0x831FD350;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FD354: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FD358: 40990030  ble cr6, 0x831fd388
	if !ctx.cr[6].gt {
	pc = 0x831FD388; continue 'dispatch;
	}
	// 831FD35C: 7D4BF0AE  lbzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 831FD360: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FD364: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FD368: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 831FD36C: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 831FD370: 390AFFF6  addi r8, r10, -0xa
	ctx.r[8].s64 = ctx.r[10].s64 + -10;
	// 831FD374: 7D070034  cntlzw r7, r8
	ctx.r[7].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 831FD378: 54EADFFE  rlwinm r10, r7, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 831FD37C: 7CCA4A14  add r6, r10, r9
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 831FD380: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 831FD384: 4198FFD8  blt cr6, 0x831fd35c
	if ctx.cr[6].lt {
	pc = 0x831FD35C; continue 'dispatch;
	}
	// 831FD388: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FD38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FD390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FD394: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FD398: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FD39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FD3A0 size=64
    let mut pc: u32 = 0x831FD3A0;
    'dispatch: loop {
        match pc {
            0x831FD3A0 => {
    //   block [0x831FD3A0..0x831FD3E0)
	// 831FD3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FD3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FD3A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FD3AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FD3B0: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD3B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FD3B8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD3BC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FD3C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FD3C4: 4E800421  bctrl
	ctx.lr = 0x831FD3C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD3C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FD3CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FD3D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FD3D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FD3D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FD3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FD3E0 size=64
    let mut pc: u32 = 0x831FD3E0;
    'dispatch: loop {
        match pc {
            0x831FD3E0 => {
    //   block [0x831FD3E0..0x831FD420)
	// 831FD3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FD3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FD3E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FD3EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FD3F0: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD3F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FD3F8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD3FC: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831FD400: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FD404: 4E800421  bctrl
	ctx.lr = 0x831FD408;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD408: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FD40C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FD410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FD414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FD418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FD41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FD420 size=76
    let mut pc: u32 = 0x831FD420;
    'dispatch: loop {
        match pc {
            0x831FD420 => {
    //   block [0x831FD420..0x831FD46C)
	// 831FD420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FD424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FD428: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FD42C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FD430: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FD434: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD438: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD43C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FD440: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FD444: 4E800421  bctrl
	ctx.lr = 0x831FD448;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD448: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FD44C: 409A000C  bne cr6, 0x831fd458
	if !ctx.cr[6].eq {
	pc = 0x831FD458; continue 'dispatch;
	}
	// 831FD450: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FD454: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 831FD458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FD45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FD460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FD464: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FD468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FD470 size=76
    let mut pc: u32 = 0x831FD470;
    'dispatch: loop {
        match pc {
            0x831FD470 => {
    //   block [0x831FD470..0x831FD4BC)
	// 831FD470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FD474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FD478: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FD47C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FD480: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FD484: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD488: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD48C: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831FD490: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FD494: 4E800421  bctrl
	ctx.lr = 0x831FD498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD498: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FD49C: 409A000C  bne cr6, 0x831fd4a8
	if !ctx.cr[6].eq {
	pc = 0x831FD4A8; continue 'dispatch;
	}
	// 831FD4A0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FD4A4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831FD4A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FD4AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FD4B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FD4B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FD4B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FD4C0 size=48
    let mut pc: u32 = 0x831FD4C0;
    'dispatch: loop {
        match pc {
            0x831FD4C0 => {
    //   block [0x831FD4C0..0x831FD4F0)
	// 831FD4C0: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831FD4C4: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 831FD4C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FD4CC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 831FD4D0: 390A32DC  addi r8, r10, 0x32dc
	ctx.r[8].s64 = ctx.r[10].s64 + 13020;
	// 831FD4D4: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831FD4D8: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 831FD4DC: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 831FD4E0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 831FD4E4: A0E40004  lhz r7, 4(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FD4E8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 831FD4EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FD4F0 size=16
    let mut pc: u32 = 0x831FD4F0;
    'dispatch: loop {
        match pc {
            0x831FD4F0 => {
    //   block [0x831FD4F0..0x831FD500)
	// 831FD4F0: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 831FD4F4: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 831FD4F8: B1440006  sth r10, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 831FD4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FD500 size=124
    let mut pc: u32 = 0x831FD500;
    'dispatch: loop {
        match pc {
            0x831FD500 => {
    //   block [0x831FD500..0x831FD57C)
	// 831FD500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FD504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FD508: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FD50C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FD510: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FD514: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831FD518: 394B32DC  addi r10, r11, 0x32dc
	ctx.r[10].s64 = ctx.r[11].s64 + 13020;
	// 831FD51C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD520: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831FD524: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FD528: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 831FD52C: 419A0030  beq cr6, 0x831fd55c
	if ctx.cr[6].eq {
	pc = 0x831FD55C; continue 'dispatch;
	}
	// 831FD530: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 831FD534: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 831FD538: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 831FD53C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 831FD540: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831FD544: 409A0018  bne cr6, 0x831fd55c
	if !ctx.cr[6].eq {
	pc = 0x831FD55C; continue 'dispatch;
	}
	// 831FD548: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD54C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831FD550: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD554: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FD558: 4E800421  bctrl
	ctx.lr = 0x831FD55C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD55C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 831FD560: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 831FD564: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831FD568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FD56C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FD570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FD574: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FD578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FD580 size=100
    let mut pc: u32 = 0x831FD580;
    'dispatch: loop {
        match pc {
            0x831FD580 => {
    //   block [0x831FD580..0x831FD5E4)
	// 831FD580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FD584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FD588: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FD58C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FD590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FD594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FD598: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FD59C: 4BFFFF65  bl 0x831fd500
	ctx.lr = 0x831FD5A0;
	sub_831FD500(ctx, base);
	// 831FD5A0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 831FD5A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FD5A8: 419A0020  beq cr6, 0x831fd5c8
	if ctx.cr[6].eq {
	pc = 0x831FD5C8; continue 'dispatch;
	}
	// 831FD5AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD5B0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 831FD5B4: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 831FD5B8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FD5BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FD5C0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FD5C4: 4BCA31ED  bl 0x82ea07b0
	ctx.lr = 0x831FD5C8;
	sub_82EA07B0(ctx, base);
	// 831FD5C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FD5CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FD5D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FD5D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FD5D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FD5DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FD5E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FD5E8 size=132
    let mut pc: u32 = 0x831FD5E8;
    'dispatch: loop {
        match pc {
            0x831FD5E8 => {
    //   block [0x831FD5E8..0x831FD66C)
	// 831FD5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FD5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FD5F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FD5F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FD5F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FD5FC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831FD600: 394B3310  addi r10, r11, 0x3310
	ctx.r[10].s64 = ctx.r[11].s64 + 13072;
	// 831FD604: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD608: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831FD60C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831FD610: 419A003C  beq cr6, 0x831fd64c
	if ctx.cr[6].eq {
	pc = 0x831FD64C; continue 'dispatch;
	}
	// 831FD614: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FD618: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FD61C: 419A0030  beq cr6, 0x831fd64c
	if ctx.cr[6].eq {
	pc = 0x831FD64C; continue 'dispatch;
	}
	// 831FD620: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 831FD624: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 831FD628: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 831FD62C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 831FD630: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831FD634: 409A0018  bne cr6, 0x831fd64c
	if !ctx.cr[6].eq {
	pc = 0x831FD64C; continue 'dispatch;
	}
	// 831FD638: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD63C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831FD640: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD644: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FD648: 4E800421  bctrl
	ctx.lr = 0x831FD64C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD64C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 831FD650: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 831FD654: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831FD658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FD65C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FD660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FD664: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FD668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FD670 size=104
    let mut pc: u32 = 0x831FD670;
    'dispatch: loop {
        match pc {
            0x831FD670 => {
    //   block [0x831FD670..0x831FD6D8)
	// 831FD670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FD674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FD678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FD67C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FD680: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FD684: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FD688: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FD68C: 419A0030  beq cr6, 0x831fd6bc
	if ctx.cr[6].eq {
	pc = 0x831FD6BC; continue 'dispatch;
	}
	// 831FD690: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 831FD694: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD698: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FD69C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FD6A0: 4E800421  bctrl
	ctx.lr = 0x831FD6A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD6A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FD6A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FD6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FD6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FD6B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FD6B8: 4E800020  blr
	return;
	// 831FD6BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FD6C0: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 831FD6C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FD6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FD6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FD6D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FD6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FD6D8 size=284
    let mut pc: u32 = 0x831FD6D8;
    'dispatch: loop {
        match pc {
            0x831FD6D8 => {
    //   block [0x831FD6D8..0x831FD7F4)
	// 831FD6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FD6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FD6E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FD6E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FD6E8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FD6EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831FD6F0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 831FD6F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD6F8: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FD6FC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FD700: 4E800421  bctrl
	ctx.lr = 0x831FD704;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD704: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD708: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 831FD70C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FD710: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FD714: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FD718: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 831FD71C: 4E800421  bctrl
	ctx.lr = 0x831FD720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD720: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FD724: 419A00B8  beq cr6, 0x831fd7dc
	if ctx.cr[6].eq {
	pc = 0x831FD7DC; continue 'dispatch;
	}
	// 831FD728: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831FD72C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FD730: 40990040  ble cr6, 0x831fd770
	if !ctx.cr[6].gt {
	pc = 0x831FD770; continue 'dispatch;
	}
	// 831FD734: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 831FD738: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD73C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831FD740: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 831FD744: 419A001C  beq cr6, 0x831fd760
	if ctx.cr[6].eq {
	pc = 0x831FD760; continue 'dispatch;
	}
	// 831FD748: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 831FD74C: 419A0014  beq cr6, 0x831fd760
	if ctx.cr[6].eq {
	pc = 0x831FD760; continue 'dispatch;
	}
	// 831FD750: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 831FD754: 419A000C  beq cr6, 0x831fd760
	if ctx.cr[6].eq {
	pc = 0x831FD760; continue 'dispatch;
	}
	// 831FD758: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 831FD75C: 409A0054  bne cr6, 0x831fd7b0
	if !ctx.cr[6].eq {
	pc = 0x831FD7B0; continue 'dispatch;
	}
	// 831FD760: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 831FD764: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831FD768: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 831FD76C: 4198FFCC  blt cr6, 0x831fd738
	if ctx.cr[6].lt {
	pc = 0x831FD738; continue 'dispatch;
	}
	// 831FD770: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD774: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 831FD778: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FD77C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FD780: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FD784: 4E800421  bctrl
	ctx.lr = 0x831FD788;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD788: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD78C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 831FD790: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FD794: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FD798: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FD79C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 831FD7A0: 4E800421  bctrl
	ctx.lr = 0x831FD7A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD7A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FD7A8: 409AFF80  bne cr6, 0x831fd728
	if !ctx.cr[6].eq {
	pc = 0x831FD728; continue 'dispatch;
	}
	// 831FD7AC: 48000030  b 0x831fd7dc
	pc = 0x831FD7DC; continue 'dispatch;
	// 831FD7B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD7B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FD7B8: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831FD7BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FD7C0: 4E800421  bctrl
	ctx.lr = 0x831FD7C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD7C4: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD7C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FD7CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FD7D0: 81090014  lwz r8, 0x14(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FD7D4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 831FD7D8: 4E800421  bctrl
	ctx.lr = 0x831FD7DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD7DC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831FD7E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FD7E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FD7E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FD7EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FD7F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FD7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FD7F8 size=544
    let mut pc: u32 = 0x831FD7F8;
    'dispatch: loop {
        match pc {
            0x831FD7F8 => {
    //   block [0x831FD7F8..0x831FDA18)
	// 831FD7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FD7FC: 4BFAA96D  bl 0x831a8168
	ctx.lr = 0x831FD800;
	sub_831A8130(ctx, base);
	// 831FD800: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FD804: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831FD808: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FD80C: 4BFFFECD  bl 0x831fd6d8
	ctx.lr = 0x831FD810;
	sub_831FD6D8(ctx, base);
	// 831FD810: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831FD814: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831FD818: 9BFE0000  stb r31, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 831FD81C: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 831FD820: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD824: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 831FD828: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FD82C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FD830: 4E800421  bctrl
	ctx.lr = 0x831FD834;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD834: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD838: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 831FD83C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FD840: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831FD844: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FD848: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 831FD84C: 4E800421  bctrl
	ctx.lr = 0x831FD850;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD850: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FD854: 419A01B8  beq cr6, 0x831fda0c
	if ctx.cr[6].eq {
	pc = 0x831FDA0C; continue 'dispatch;
	}
	// 831FD858: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FD85C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831FD860: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 831FD864: 409A000C  bne cr6, 0x831fd870
	if !ctx.cr[6].eq {
	pc = 0x831FD870; continue 'dispatch;
	}
	// 831FD868: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 831FD86C: 48000014  b 0x831fd880
	pc = 0x831FD880; continue 'dispatch;
	// 831FD870: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 831FD874: 409A000C  bne cr6, 0x831fd880
	if !ctx.cr[6].eq {
	pc = 0x831FD880; continue 'dispatch;
	}
	// 831FD878: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 831FD87C: 9BFE0000  stb r31, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 831FD880: 397F0003  addi r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 3;
	// 831FD884: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 831FD888: 40980078  bge cr6, 0x831fd900
	if !ctx.cr[6].lt {
	pc = 0x831FD900; continue 'dispatch;
	}
	// 831FD88C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 831FD890: 7D5F58AE  lbzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FD894: 2B0A0030  cmplwi cr6, r10, 0x30
	ctx.cr[6].compare_u32(ctx.r[10].u32, 48 as u32, &mut ctx.xer);
	// 831FD898: 409A0068  bne cr6, 0x831fd900
	if !ctx.cr[6].eq {
	pc = 0x831FD900; continue 'dispatch;
	}
	// 831FD89C: 39610051  addi r11, r1, 0x51
	ctx.r[11].s64 = ctx.r[1].s64 + 81;
	// 831FD8A0: 7D5F58AE  lbzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FD8A4: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 831FD8A8: 2F0B0078  cmpwi cr6, r11, 0x78
	ctx.cr[6].compare_i32(ctx.r[11].s32, 120, &mut ctx.xer);
	// 831FD8AC: 419A000C  beq cr6, 0x831fd8b8
	if ctx.cr[6].eq {
	pc = 0x831FD8B8; continue 'dispatch;
	}
	// 831FD8B0: 2F0B0058  cmpwi cr6, r11, 0x58
	ctx.cr[6].compare_i32(ctx.r[11].s32, 88, &mut ctx.xer);
	// 831FD8B4: 409A004C  bne cr6, 0x831fd900
	if !ctx.cr[6].eq {
	pc = 0x831FD900; continue 'dispatch;
	}
	// 831FD8B8: 39610052  addi r11, r1, 0x52
	ctx.r[11].s64 = ctx.r[1].s64 + 82;
	// 831FD8BC: 7D5F58AE  lbzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FD8C0: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 831FD8C4: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 831FD8C8: 4198000C  blt cr6, 0x831fd8d4
	if ctx.cr[6].lt {
	pc = 0x831FD8D4; continue 'dispatch;
	}
	// 831FD8CC: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 831FD8D0: 40990024  ble cr6, 0x831fd8f4
	if !ctx.cr[6].gt {
	pc = 0x831FD8F4; continue 'dispatch;
	}
	// 831FD8D4: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 831FD8D8: 4198000C  blt cr6, 0x831fd8e4
	if ctx.cr[6].lt {
	pc = 0x831FD8E4; continue 'dispatch;
	}
	// 831FD8DC: 2F0B0066  cmpwi cr6, r11, 0x66
	ctx.cr[6].compare_i32(ctx.r[11].s32, 102, &mut ctx.xer);
	// 831FD8E0: 40990014  ble cr6, 0x831fd8f4
	if !ctx.cr[6].gt {
	pc = 0x831FD8F4; continue 'dispatch;
	}
	// 831FD8E4: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 831FD8E8: 41980018  blt cr6, 0x831fd900
	if ctx.cr[6].lt {
	pc = 0x831FD900; continue 'dispatch;
	}
	// 831FD8EC: 2F0B0046  cmpwi cr6, r11, 0x46
	ctx.cr[6].compare_i32(ctx.r[11].s32, 70, &mut ctx.xer);
	// 831FD8F0: 41990010  bgt cr6, 0x831fd900
	if ctx.cr[6].gt {
	pc = 0x831FD900; continue 'dispatch;
	}
	// 831FD8F4: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 831FD8F8: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 831FD8FC: 4800004C  b 0x831fd948
	pc = 0x831FD948; continue 'dispatch;
	// 831FD900: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 831FD904: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 831FD908: 4098003C  bge cr6, 0x831fd944
	if !ctx.cr[6].lt {
	pc = 0x831FD944; continue 'dispatch;
	}
	// 831FD90C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 831FD910: 7D5F58AE  lbzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FD914: 2B0A0030  cmplwi cr6, r10, 0x30
	ctx.cr[6].compare_u32(ctx.r[10].u32, 48 as u32, &mut ctx.xer);
	// 831FD918: 409A002C  bne cr6, 0x831fd944
	if !ctx.cr[6].eq {
	pc = 0x831FD944; continue 'dispatch;
	}
	// 831FD91C: 39610051  addi r11, r1, 0x51
	ctx.r[11].s64 = ctx.r[1].s64 + 81;
	// 831FD920: 7D5F58AE  lbzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FD924: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 831FD928: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 831FD92C: 41980018  blt cr6, 0x831fd944
	if ctx.cr[6].lt {
	pc = 0x831FD944; continue 'dispatch;
	}
	// 831FD930: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 831FD934: 41990010  bgt cr6, 0x831fd944
	if ctx.cr[6].gt {
	pc = 0x831FD944; continue 'dispatch;
	}
	// 831FD938: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 831FD93C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 831FD940: 48000008  b 0x831fd948
	pc = 0x831FD948; continue 'dispatch;
	// 831FD944: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 831FD948: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 831FD94C: 40980088  bge cr6, 0x831fd9d4
	if !ctx.cr[6].lt {
	pc = 0x831FD9D4; continue 'dispatch;
	}
	// 831FD950: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 831FD954: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 831FD958: 7D3F5A14  add r9, r31, r11
	ctx.r[9].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 831FD95C: 89690000  lbz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD960: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 831FD964: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831FD968: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 831FD96C: 41980014  blt cr6, 0x831fd980
	if ctx.cr[6].lt {
	pc = 0x831FD980; continue 'dispatch;
	}
	// 831FD970: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 831FD974: 4199000C  bgt cr6, 0x831fd980
	if ctx.cr[6].gt {
	pc = 0x831FD980; continue 'dispatch;
	}
	// 831FD978: 394BFFD0  addi r10, r11, -0x30
	ctx.r[10].s64 = ctx.r[11].s64 + -48;
	// 831FD97C: 48000030  b 0x831fd9ac
	pc = 0x831FD9AC; continue 'dispatch;
	// 831FD980: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 831FD984: 40990014  ble cr6, 0x831fd998
	if !ctx.cr[6].gt {
	pc = 0x831FD998; continue 'dispatch;
	}
	// 831FD988: 2F0B0046  cmpwi cr6, r11, 0x46
	ctx.cr[6].compare_i32(ctx.r[11].s32, 70, &mut ctx.xer);
	// 831FD98C: 4199000C  bgt cr6, 0x831fd998
	if ctx.cr[6].gt {
	pc = 0x831FD998; continue 'dispatch;
	}
	// 831FD990: 394BFFC9  addi r10, r11, -0x37
	ctx.r[10].s64 = ctx.r[11].s64 + -55;
	// 831FD994: 48000018  b 0x831fd9ac
	pc = 0x831FD9AC; continue 'dispatch;
	// 831FD998: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 831FD99C: 40990010  ble cr6, 0x831fd9ac
	if !ctx.cr[6].gt {
	pc = 0x831FD9AC; continue 'dispatch;
	}
	// 831FD9A0: 2F0B0066  cmpwi cr6, r11, 0x66
	ctx.cr[6].compare_i32(ctx.r[11].s32, 102, &mut ctx.xer);
	// 831FD9A4: 41990008  bgt cr6, 0x831fd9ac
	if ctx.cr[6].gt {
	pc = 0x831FD9AC; continue 'dispatch;
	}
	// 831FD9A8: 394BFFA9  addi r10, r11, -0x57
	ctx.r[10].s64 = ctx.r[11].s64 + -87;
	// 831FD9AC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 831FD9B0: 40980024  bge cr6, 0x831fd9d4
	if !ctx.cr[6].lt {
	pc = 0x831FD9D4; continue 'dispatch;
	}
	// 831FD9B4: 790B0020  clrldi r11, r8, 0x20
	ctx.r[11].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 831FD9B8: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 831FD9BC: 7D6BE9D2  mulld r11, r11, r29
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[29].s64;
	// 831FD9C0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 831FD9C4: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831FD9C8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831FD9CC: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 831FD9D0: 4198FF8C  blt cr6, 0x831fd95c
	if ctx.cr[6].lt {
	pc = 0x831FD95C; continue 'dispatch;
	}
	// 831FD9D4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD9D8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831FD9DC: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831FD9E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FD9E4: 4E800421  bctrl
	ctx.lr = 0x831FD9E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FD9E8: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FD9EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FD9F0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831FD9F4: 81090014  lwz r8, 0x14(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FD9F8: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 831FD9FC: 4E800421  bctrl
	ctx.lr = 0x831FDA00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FDA00: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831FDA04: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 831FDA08: 4BFAA7B0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 831FDA0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831FDA10: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 831FDA14: 4BFAA7A4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FDA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FDA18 size=392
    let mut pc: u32 = 0x831FDA18;
    'dispatch: loop {
        match pc {
            0x831FDA18 => {
    //   block [0x831FDA18..0x831FDBA0)
	// 831FDA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FDA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FDA20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FDA24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FDA28: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FDA2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831FDA30: 4BFFFCA9  bl 0x831fd6d8
	ctx.lr = 0x831FDA34;
	sub_831FD6D8(ctx, base);
	// 831FDA34: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDA38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FDA3C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831FDA40: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 831FDA44: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FDA48: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FDA4C: 4E800421  bctrl
	ctx.lr = 0x831FDA50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FDA50: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDA54: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 831FDA58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FDA5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FDA60: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FDA64: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 831FDA68: 4E800421  bctrl
	ctx.lr = 0x831FDA6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FDA6C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FDA70: 419A00BC  beq cr6, 0x831fdb2c
	if ctx.cr[6].eq {
	pc = 0x831FDB2C; continue 'dispatch;
	}
	// 831FDA74: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FDA78: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831FDA7C: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 831FDA80: 4198000C  blt cr6, 0x831fda8c
	if ctx.cr[6].lt {
	pc = 0x831FDA8C; continue 'dispatch;
	}
	// 831FDA84: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 831FDA88: 40990024  ble cr6, 0x831fdaac
	if !ctx.cr[6].gt {
	pc = 0x831FDAAC; continue 'dispatch;
	}
	// 831FDA8C: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 831FDA90: 419A001C  beq cr6, 0x831fdaac
	if ctx.cr[6].eq {
	pc = 0x831FDAAC; continue 'dispatch;
	}
	// 831FDA94: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 831FDA98: 419A0014  beq cr6, 0x831fdaac
	if ctx.cr[6].eq {
	pc = 0x831FDAAC; continue 'dispatch;
	}
	// 831FDA9C: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 831FDAA0: 419A000C  beq cr6, 0x831fdaac
	if ctx.cr[6].eq {
	pc = 0x831FDAAC; continue 'dispatch;
	}
	// 831FDAA4: 2F0B002C  cmpwi cr6, r11, 0x2c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 44, &mut ctx.xer);
	// 831FDAA8: 409A0084  bne cr6, 0x831fdb2c
	if !ctx.cr[6].eq {
	pc = 0x831FDB2C; continue 'dispatch;
	}
	// 831FDAAC: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 831FDAB0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831FDAB4: 40990078  ble cr6, 0x831fdb2c
	if !ctx.cr[6].gt {
	pc = 0x831FDB2C; continue 'dispatch;
	}
	// 831FDAB8: 39410051  addi r10, r1, 0x51
	ctx.r[10].s64 = ctx.r[1].s64 + 81;
	// 831FDABC: 3920002E  li r9, 0x2e
	ctx.r[9].s64 = 46;
	// 831FDAC0: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDAC4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831FDAC8: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 831FDACC: 4198000C  blt cr6, 0x831fdad8
	if ctx.cr[6].lt {
	pc = 0x831FDAD8; continue 'dispatch;
	}
	// 831FDAD0: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 831FDAD4: 40990034  ble cr6, 0x831fdb08
	if !ctx.cr[6].gt {
	pc = 0x831FDB08; continue 'dispatch;
	}
	// 831FDAD8: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 831FDADC: 419A002C  beq cr6, 0x831fdb08
	if ctx.cr[6].eq {
	pc = 0x831FDB08; continue 'dispatch;
	}
	// 831FDAE0: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 831FDAE4: 419A0024  beq cr6, 0x831fdb08
	if ctx.cr[6].eq {
	pc = 0x831FDB08; continue 'dispatch;
	}
	// 831FDAE8: 2F0B0045  cmpwi cr6, r11, 0x45
	ctx.cr[6].compare_i32(ctx.r[11].s32, 69, &mut ctx.xer);
	// 831FDAEC: 419A001C  beq cr6, 0x831fdb08
	if ctx.cr[6].eq {
	pc = 0x831FDB08; continue 'dispatch;
	}
	// 831FDAF0: 2F0B0065  cmpwi cr6, r11, 0x65
	ctx.cr[6].compare_i32(ctx.r[11].s32, 101, &mut ctx.xer);
	// 831FDAF4: 419A0014  beq cr6, 0x831fdb08
	if ctx.cr[6].eq {
	pc = 0x831FDB08; continue 'dispatch;
	}
	// 831FDAF8: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 831FDAFC: 419A001C  beq cr6, 0x831fdb18
	if ctx.cr[6].eq {
	pc = 0x831FDB18; continue 'dispatch;
	}
	// 831FDB00: 2F0B002C  cmpwi cr6, r11, 0x2c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 44, &mut ctx.xer);
	// 831FDB04: 409A0028  bne cr6, 0x831fdb2c
	if !ctx.cr[6].eq {
	pc = 0x831FDB2C; continue 'dispatch;
	}
	// 831FDB08: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 831FDB0C: 419A000C  beq cr6, 0x831fdb18
	if ctx.cr[6].eq {
	pc = 0x831FDB18; continue 'dispatch;
	}
	// 831FDB10: 2F0B002C  cmpwi cr6, r11, 0x2c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 44, &mut ctx.xer);
	// 831FDB14: 409A0008  bne cr6, 0x831fdb1c
	if !ctx.cr[6].eq {
	pc = 0x831FDB1C; continue 'dispatch;
	}
	// 831FDB18: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 831FDB1C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 831FDB20: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831FDB24: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 831FDB28: 4198FF98  blt cr6, 0x831fdac0
	if ctx.cr[6].lt {
	pc = 0x831FDAC0; continue 'dispatch;
	}
	// 831FDB2C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDB30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FDB34: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831FDB38: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FDB3C: 4E800421  bctrl
	ctx.lr = 0x831FDB40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FDB40: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDB44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FDB48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FDB4C: 81090014  lwz r8, 0x14(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FDB50: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 831FDB54: 4E800421  bctrl
	ctx.lr = 0x831FDB58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FDB58: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 831FDB5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831FDB60: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831FDB64: 7CDF39AE  stbx r6, r31, r7
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[7].u32), ctx.r[6].u8) };
	// 831FDB68: 40990018  ble cr6, 0x831fdb80
	if !ctx.cr[6].gt {
	pc = 0x831FDB80; continue 'dispatch;
	}
	// 831FDB6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831FDB70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FDB74: 4BFACB75  bl 0x831aa6e8
	ctx.lr = 0x831FDB78;
	sub_831AA6E8(ctx, base);
	// 831FDB78: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 831FDB7C: 4800000C  b 0x831fdb88
	pc = 0x831FDB88; continue 'dispatch;
	// 831FDB80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 831FDB84: C02B9534  lfs f1, -0x6acc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 831FDB88: 38210170  addi r1, r1, 0x170
	ctx.r[1].s64 = ctx.r[1].s64 + 368;
	// 831FDB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FDB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FDB94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FDB98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FDB9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FDBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FDBA0 size=412
    let mut pc: u32 = 0x831FDBA0;
    'dispatch: loop {
        match pc {
            0x831FDBA0 => {
    //   block [0x831FDBA0..0x831FDD3C)
	// 831FDBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FDBA4: 4BFAA5C9  bl 0x831a816c
	ctx.lr = 0x831FDBA8;
	sub_831A8130(ctx, base);
	// 831FDBA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FDBAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FDBB0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831FDBB4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDBB8: 4BFFFB21  bl 0x831fd6d8
	ctx.lr = 0x831FDBBC;
	sub_831FD6D8(ctx, base);
	// 831FDBBC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDBC0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 831FDBC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDBC8: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FDBCC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FDBD0: 4E800421  bctrl
	ctx.lr = 0x831FDBD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FDBD4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDBD8: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 831FDBDC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FDBE0: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDBE4: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FDBE8: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 831FDBEC: 4E800421  bctrl
	ctx.lr = 0x831FDBF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FDBF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831FDBF4: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 831FDBF8: 41980090  blt cr6, 0x831fdc88
	if ctx.cr[6].lt {
	pc = 0x831FDC88; continue 'dispatch;
	}
	// 831FDBFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 831FDC00: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 831FDC04: 388B9FFC  addi r4, r11, -0x6004
	ctx.r[4].s64 = ctx.r[11].s64 + -24580;
	// 831FDC08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FDC0C: 4BCAC815  bl 0x82eaa420
	ctx.lr = 0x831FDC10;
	sub_82EAA420(ctx, base);
	// 831FDC10: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FDC14: 409A0074  bne cr6, 0x831fdc88
	if !ctx.cr[6].eq {
	pc = 0x831FDC88; continue 'dispatch;
	}
	// 831FDC18: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 831FDC1C: 419A002C  beq cr6, 0x831fdc48
	if ctx.cr[6].eq {
	pc = 0x831FDC48; continue 'dispatch;
	}
	// 831FDC20: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831FDC24: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831FDC28: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 831FDC2C: 419A001C  beq cr6, 0x831fdc48
	if ctx.cr[6].eq {
	pc = 0x831FDC48; continue 'dispatch;
	}
	// 831FDC30: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 831FDC34: 419A0014  beq cr6, 0x831fdc48
	if ctx.cr[6].eq {
	pc = 0x831FDC48; continue 'dispatch;
	}
	// 831FDC38: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 831FDC3C: 419A000C  beq cr6, 0x831fdc48
	if ctx.cr[6].eq {
	pc = 0x831FDC48; continue 'dispatch;
	}
	// 831FDC40: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 831FDC44: 409A0044  bne cr6, 0x831fdc88
	if !ctx.cr[6].eq {
	pc = 0x831FDC88; continue 'dispatch;
	}
	// 831FDC48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831FDC4C: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 831FDC50: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDC54: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDC58: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 831FDC5C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831FDC60: 4E800421  bctrl
	ctx.lr = 0x831FDC64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FDC64: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDC68: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 831FDC6C: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDC70: 80E80014  lwz r7, 0x14(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FDC74: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 831FDC78: 4E800421  bctrl
	ctx.lr = 0x831FDC7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FDC7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FDC80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FDC84: 4BFAA538  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831FDC88: 2F1E0005  cmpwi cr6, r30, 5
	ctx.cr[6].compare_i32(ctx.r[30].s32, 5, &mut ctx.xer);
	// 831FDC8C: 41980090  blt cr6, 0x831fdd1c
	if ctx.cr[6].lt {
	pc = 0x831FDD1C; continue 'dispatch;
	}
	// 831FDC90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 831FDC94: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 831FDC98: 388B9FF4  addi r4, r11, -0x600c
	ctx.r[4].s64 = ctx.r[11].s64 + -24588;
	// 831FDC9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FDCA0: 4BCAC781  bl 0x82eaa420
	ctx.lr = 0x831FDCA4;
	sub_82EAA420(ctx, base);
	// 831FDCA4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FDCA8: 409A0074  bne cr6, 0x831fdd1c
	if !ctx.cr[6].eq {
	pc = 0x831FDD1C; continue 'dispatch;
	}
	// 831FDCAC: 2F1E0005  cmpwi cr6, r30, 5
	ctx.cr[6].compare_i32(ctx.r[30].s32, 5, &mut ctx.xer);
	// 831FDCB0: 419A002C  beq cr6, 0x831fdcdc
	if ctx.cr[6].eq {
	pc = 0x831FDCDC; continue 'dispatch;
	}
	// 831FDCB4: 89610055  lbz r11, 0x55(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 831FDCB8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831FDCBC: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 831FDCC0: 419A001C  beq cr6, 0x831fdcdc
	if ctx.cr[6].eq {
	pc = 0x831FDCDC; continue 'dispatch;
	}
	// 831FDCC4: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 831FDCC8: 419A0014  beq cr6, 0x831fdcdc
	if ctx.cr[6].eq {
	pc = 0x831FDCDC; continue 'dispatch;
	}
	// 831FDCCC: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 831FDCD0: 419A000C  beq cr6, 0x831fdcdc
	if ctx.cr[6].eq {
	pc = 0x831FDCDC; continue 'dispatch;
	}
	// 831FDCD4: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 831FDCD8: 409A0044  bne cr6, 0x831fdd1c
	if !ctx.cr[6].eq {
	pc = 0x831FDD1C; continue 'dispatch;
	}
	// 831FDCDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FDCE0: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 831FDCE4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDCE8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDCEC: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 831FDCF0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831FDCF4: 4E800421  bctrl
	ctx.lr = 0x831FDCF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FDCF8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDCFC: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 831FDD00: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDD04: 80E80014  lwz r7, 0x14(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FDD08: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 831FDD0C: 4E800421  bctrl
	ctx.lr = 0x831FDD10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FDD10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FDD14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FDD18: 4BFAA4A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831FDD1C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDD20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDD24: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831FDD28: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FDD2C: 4E800421  bctrl
	ctx.lr = 0x831FDD30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FDD30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FDD34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FDD38: 4BFAA484  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FDD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FDD40 size=68
    let mut pc: u32 = 0x831FDD40;
    'dispatch: loop {
        match pc {
            0x831FDD40 => {
    //   block [0x831FDD40..0x831FDD84)
	// 831FDD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FDD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FDD48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FDD4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FDD50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FDD54: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831FDD58: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDD5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDD60: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FDD64: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FDD68: 4E800421  bctrl
	ctx.lr = 0x831FDD6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FDD6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FDD70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FDD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FDD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FDD7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FDD80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FDD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FDD88 size=68
    let mut pc: u32 = 0x831FDD88;
    'dispatch: loop {
        match pc {
            0x831FDD88 => {
    //   block [0x831FDD88..0x831FDDCC)
	// 831FDD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FDD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FDD90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FDD94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FDD98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FDD9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FDDA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FDDA4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDDA8: 4BFFFC71  bl 0x831fda18
	ctx.lr = 0x831FDDAC;
	sub_831FDA18(ctx, base);
	// 831FDDAC: D03E0000  stfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831FDDB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FDDB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FDDB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FDDBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FDDC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FDDC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FDDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FDDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FDDD0 size=68
    let mut pc: u32 = 0x831FDDD0;
    'dispatch: loop {
        match pc {
            0x831FDDD0 => {
    //   block [0x831FDDD0..0x831FDE14)
	// 831FDDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FDDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FDDD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FDDDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FDDE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FDDE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FDDE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FDDEC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDDF0: 4BFFFC29  bl 0x831fda18
	ctx.lr = 0x831FDDF4;
	sub_831FDA18(ctx, base);
	// 831FDDF4: D83E0000  stfd f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.f[1].u64 ) };
	// 831FDDF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FDDFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FDE00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FDE04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FDE08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FDE0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FDE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FDE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FDE18 size=124
    let mut pc: u32 = 0x831FDE18;
    'dispatch: loop {
        match pc {
            0x831FDE18 => {
    //   block [0x831FDE18..0x831FDE94)
	// 831FDE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FDE1C: 4BFAA349  bl 0x831a8164
	ctx.lr = 0x831FDE20;
	sub_831A8130(ctx, base);
	// 831FDE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FDE24: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831FDE28: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831FDE2C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 831FDE30: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 831FDE34: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDE38: 4BFFF8A1  bl 0x831fd6d8
	ctx.lr = 0x831FDE3C;
	sub_831FD6D8(ctx, base);
	// 831FDE3C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831FDE40: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831FDE44: 40990030  ble cr6, 0x831fde74
	if !ctx.cr[6].gt {
	pc = 0x831FDE74; continue 'dispatch;
	}
	// 831FDE48: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDE4C: 4BCB1CBD  bl 0x82eafb08
	ctx.lr = 0x831FDE50;
	sub_82EAFB08(ctx, base);
	// 831FDE50: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 831FDE54: 419A002C  beq cr6, 0x831fde80
	if ctx.cr[6].eq {
	pc = 0x831FDE80; continue 'dispatch;
	}
	// 831FDE58: 7F6B0774  extsb r11, r27
	ctx.r[11].s64 = ctx.r[27].s8 as i64;
	// 831FDE5C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831FDE60: 419A0020  beq cr6, 0x831fde80
	if ctx.cr[6].eq {
	pc = 0x831FDE80; continue 'dispatch;
	}
	// 831FDE64: 7C7FE9AE  stbx r3, r31, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32), ctx.r[3].u8) };
	// 831FDE68: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 831FDE6C: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 831FDE70: 4198FFD8  blt cr6, 0x831fde48
	if ctx.cr[6].lt {
	pc = 0x831FDE48; continue 'dispatch;
	}
	// 831FDE74: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831FDE78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FDE7C: 4BFAA338  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831FDE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FDE84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FDE88: 7D7FE9AE  stbx r11, r31, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32), ctx.r[11].u8) };
	// 831FDE8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831FDE90: 4BFAA324  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FDE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FDE98 size=20
    let mut pc: u32 = 0x831FDE98;
    'dispatch: loop {
        match pc {
            0x831FDE98 => {
    //   block [0x831FDE98..0x831FDEAC)
	// 831FDE98: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDE9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDEA0: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 831FDEA4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FDEA8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FDEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FDEB0 size=36
    let mut pc: u32 = 0x831FDEB0;
    'dispatch: loop {
        match pc {
            0x831FDEB0 => {
    //   block [0x831FDEB0..0x831FDED4)
	// 831FDEB0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831FDEB4: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 831FDEB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 831FDEBC: 392B3310  addi r9, r11, 0x3310
	ctx.r[9].s64 = ctx.r[11].s64 + 13072;
	// 831FDEC0: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 831FDEC4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831FDEC8: A1040004  lhz r8, 4(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FDECC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831FDED0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FDED4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FDED4 size=16
    let mut pc: u32 = 0x831FDED4;
    'dispatch: loop {
        match pc {
            0x831FDED4 => {
    //   block [0x831FDED4..0x831FDEE4)
	// 831FDED4: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 831FDED8: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 831FDEDC: B1440006  sth r10, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 831FDEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FDEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FDEE8 size=92
    let mut pc: u32 = 0x831FDEE8;
    'dispatch: loop {
        match pc {
            0x831FDEE8 => {
    //   block [0x831FDEE8..0x831FDF44)
	// 831FDEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FDEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FDEF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FDEF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FDEF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FDEFC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831FDF00: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 831FDF04: 392B3310  addi r9, r11, 0x3310
	ctx.r[9].s64 = ctx.r[11].s64 + 13072;
	// 831FDF08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 831FDF0C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831FDF10: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 831FDF14: 806A70EC  lwz r3, 0x70ec(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28908 as u32) ) } as u64;
	// 831FDF18: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDF1C: 80C7000C  lwz r6, 0xc(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 831FDF20: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 831FDF24: 4E800421  bctrl
	ctx.lr = 0x831FDF28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FDF28: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 831FDF2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FDF30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FDF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FDF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FDF3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FDF40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FDF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FDF48 size=108
    let mut pc: u32 = 0x831FDF48;
    'dispatch: loop {
        match pc {
            0x831FDF48 => {
    //   block [0x831FDF48..0x831FDFB4)
	// 831FDF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FDF4C: 4BFAA221  bl 0x831a816c
	ctx.lr = 0x831FDF50;
	sub_831A8130(ctx, base);
	// 831FDF50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FDF54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FDF58: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FDF5C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831FDF60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 831FDF64: 390B3310  addi r8, r11, 0x3310
	ctx.r[8].s64 = ctx.r[11].s64 + 13072;
	// 831FDF68: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 831FDF6C: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 831FDF70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FDF74: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 831FDF78: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831FDF7C: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 831FDF80: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 831FDF84: 7C67502E  lwzx r3, r7, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 831FDF88: 4BCA27A9  bl 0x82ea0730
	ctx.lr = 0x831FDF8C;
	sub_82EA0730(ctx, base);
	// 831FDF8C: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 831FDF90: B0A30004  sth r5, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u16 ) };
	// 831FDF94: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 831FDF98: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831FDF9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831FDFA0: 4BCAA8C9  bl 0x82ea8868
	ctx.lr = 0x831FDFA4;
	sub_82EA8868(ctx, base);
	// 831FDFA4: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 831FDFA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FDFAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FDFB0: 4BFAA20C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FDFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FDFB8 size=104
    let mut pc: u32 = 0x831FDFB8;
    'dispatch: loop {
        match pc {
            0x831FDFB8 => {
    //   block [0x831FDFB8..0x831FE020)
	// 831FDFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FDFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FDFC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FDFC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FDFC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FDFCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FDFD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FDFD4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FDFD8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FDFDC: 4BFFF81D  bl 0x831fd7f8
	ctx.lr = 0x831FDFE0;
	sub_831FD7F8(ctx, base);
	// 831FDFE0: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FDFE4: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 831FDFE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FDFEC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831FDFF0: 419A0014  beq cr6, 0x831fe004
	if ctx.cr[6].eq {
	pc = 0x831FE004; continue 'dispatch;
	}
	// 831FDFF4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 831FDFF8: 7D4B00D0  neg r10, r11
	ctx.r[10].s64 = -ctx.r[11].s64;
	// 831FDFFC: B15E0000  sth r10, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 831FE000: 48000008  b 0x831fe008
	pc = 0x831FE008; continue 'dispatch;
	// 831FE004: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 831FE008: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FE00C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FE010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FE014: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FE018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FE01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FE020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FE020 size=104
    let mut pc: u32 = 0x831FE020;
    'dispatch: loop {
        match pc {
            0x831FE020 => {
    //   block [0x831FE020..0x831FE088)
	// 831FE020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FE024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FE028: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FE02C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FE030: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FE034: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FE038: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FE03C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FE040: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FE044: 4BFFF7B5  bl 0x831fd7f8
	ctx.lr = 0x831FE048;
	sub_831FD7F8(ctx, base);
	// 831FE048: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FE04C: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 831FE050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FE054: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831FE058: 419A0014  beq cr6, 0x831fe06c
	if ctx.cr[6].eq {
	pc = 0x831FE06C; continue 'dispatch;
	}
	// 831FE05C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 831FE060: 7D4B00D0  neg r10, r11
	ctx.r[10].s64 = -ctx.r[11].s64;
	// 831FE064: B15E0000  sth r10, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 831FE068: 48000008  b 0x831fe070
	pc = 0x831FE070; continue 'dispatch;
	// 831FE06C: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 831FE070: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FE074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FE078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FE07C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FE080: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FE084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FE088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FE088 size=92
    let mut pc: u32 = 0x831FE088;
    'dispatch: loop {
        match pc {
            0x831FE088 => {
    //   block [0x831FE088..0x831FE0E4)
	// 831FE088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FE08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FE090: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FE094: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FE098: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FE09C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FE0A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FE0A4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FE0A8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FE0AC: 4BFFF74D  bl 0x831fd7f8
	ctx.lr = 0x831FE0B0;
	sub_831FD7F8(ctx, base);
	// 831FE0B0: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FE0B4: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FE0B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FE0BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831FE0C0: 419A0008  beq cr6, 0x831fe0c8
	if ctx.cr[6].eq {
	pc = 0x831FE0C8; continue 'dispatch;
	}
	// 831FE0C4: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 831FE0C8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FE0CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FE0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FE0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FE0D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FE0DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FE0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FE0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FE0E8 size=92
    let mut pc: u32 = 0x831FE0E8;
    'dispatch: loop {
        match pc {
            0x831FE0E8 => {
    //   block [0x831FE0E8..0x831FE144)
	// 831FE0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FE0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FE0F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FE0F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FE0F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FE0FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FE100: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FE104: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FE108: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FE10C: 4BFFF6ED  bl 0x831fd7f8
	ctx.lr = 0x831FE110;
	sub_831FD7F8(ctx, base);
	// 831FE110: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FE114: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FE118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FE11C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831FE120: 419A0008  beq cr6, 0x831fe128
	if ctx.cr[6].eq {
	pc = 0x831FE128; continue 'dispatch;
	}
	// 831FE124: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 831FE128: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FE12C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FE130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FE134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FE138: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FE13C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FE140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FE148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FE148 size=92
    let mut pc: u32 = 0x831FE148;
    'dispatch: loop {
        match pc {
            0x831FE148 => {
    //   block [0x831FE148..0x831FE1A4)
	// 831FE148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FE14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FE150: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FE154: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FE158: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FE15C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FE160: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FE164: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FE168: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FE16C: 4BFFF68D  bl 0x831fd7f8
	ctx.lr = 0x831FE170;
	sub_831FD7F8(ctx, base);
	// 831FE170: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FE174: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831FE178: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FE17C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831FE180: 419A0008  beq cr6, 0x831fe188
	if ctx.cr[6].eq {
	pc = 0x831FE188; continue 'dispatch;
	}
	// 831FE184: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 831FE188: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 831FE18C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FE190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FE194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FE198: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FE19C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FE1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FE1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FE1A8 size=92
    let mut pc: u32 = 0x831FE1A8;
    'dispatch: loop {
        match pc {
            0x831FE1A8 => {
    //   block [0x831FE1A8..0x831FE204)
	// 831FE1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FE1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FE1B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FE1B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FE1B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FE1BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FE1C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FE1C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831FE1C8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FE1CC: 4BFFF62D  bl 0x831fd7f8
	ctx.lr = 0x831FE1D0;
	sub_831FD7F8(ctx, base);
	// 831FE1D0: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FE1D4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831FE1D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FE1DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831FE1E0: 419A0008  beq cr6, 0x831fe1e8
	if ctx.cr[6].eq {
	pc = 0x831FE1E8; continue 'dispatch;
	}
	// 831FE1E4: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 831FE1E8: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 831FE1EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FE1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FE1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FE1F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FE1FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FE200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FE208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FE208 size=536
    let mut pc: u32 = 0x831FE208;
    'dispatch: loop {
        match pc {
            0x831FE208 => {
    //   block [0x831FE208..0x831FE420)
	// 831FE208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FE20C: 4BFA9F59  bl 0x831a8164
	ctx.lr = 0x831FE210;
	sub_831A8130(ctx, base);
	// 831FE210: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FE214: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 831FE218: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 831FE21C: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 831FE220: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831FE224: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 831FE228: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 831FE22C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 831FE230: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831FE234: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 831FE238: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FE23C: 4BCA85BD  bl 0x82ea67f8
	ctx.lr = 0x831FE240;
	sub_82EA67F8(ctx, base);
	// 831FE240: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FE244: 4BFFF495  bl 0x831fd6d8
	ctx.lr = 0x831FE248;
	sub_831FD6D8(ctx, base);
	// 831FE248: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FE24C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831FE250: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FE254: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FE258: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831FE25C: 4E800421  bctrl
	ctx.lr = 0x831FE260;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FE260: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FE264: 4BCB18A5  bl 0x82eafb08
	ctx.lr = 0x831FE268;
	sub_82EAFB08(ctx, base);
	// 831FE268: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 831FE26C: 419A0088  beq cr6, 0x831fe2f4
	if ctx.cr[6].eq {
	pc = 0x831FE2F4; continue 'dispatch;
	}
	// 831FE270: 2F030020  cmpwi cr6, r3, 0x20
	ctx.cr[6].compare_i32(ctx.r[3].s32, 32, &mut ctx.xer);
	// 831FE274: 419A0080  beq cr6, 0x831fe2f4
	if ctx.cr[6].eq {
	pc = 0x831FE2F4; continue 'dispatch;
	}
	// 831FE278: 2F030009  cmpwi cr6, r3, 9
	ctx.cr[6].compare_i32(ctx.r[3].s32, 9, &mut ctx.xer);
	// 831FE27C: 419A0078  beq cr6, 0x831fe2f4
	if ctx.cr[6].eq {
	pc = 0x831FE2F4; continue 'dispatch;
	}
	// 831FE280: 2F03000D  cmpwi cr6, r3, 0xd
	ctx.cr[6].compare_i32(ctx.r[3].s32, 13, &mut ctx.xer);
	// 831FE284: 419A0070  beq cr6, 0x831fe2f4
	if ctx.cr[6].eq {
	pc = 0x831FE2F4; continue 'dispatch;
	}
	// 831FE288: 2F03000A  cmpwi cr6, r3, 0xa
	ctx.cr[6].compare_i32(ctx.r[3].s32, 10, &mut ctx.xer);
	// 831FE28C: 419A0068  beq cr6, 0x831fe2f4
	if ctx.cr[6].eq {
	pc = 0x831FE2F4; continue 'dispatch;
	}
	// 831FE290: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 831FE294: 7C7F0774  extsb r31, r3
	ctx.r[31].s64 = ctx.r[3].s8 as i64;
	// 831FE298: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831FE29C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 831FE2A0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831FE2A4: 409A0010  bne cr6, 0x831fe2b4
	if !ctx.cr[6].eq {
	pc = 0x831FE2B4; continue 'dispatch;
	}
	// 831FE2A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831FE2AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FE2B0: 4BCA85D1  bl 0x82ea6880
	ctx.lr = 0x831FE2B4;
	sub_82EA6880(ctx, base);
	// 831FE2B4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831FE2B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831FE2BC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FE2C0: 7FEA59AE  stbx r31, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[31].u8) };
	// 831FE2C4: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FE2C8: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831FE2CC: 39090001  addi r8, r9, 1
	ctx.r[8].s64 = ctx.r[9].s64 + 1;
	// 831FE2D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 831FE2D4: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FE2D8: 80C7001C  lwz r6, 0x1c(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 831FE2DC: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 831FE2E0: 4E800421  bctrl
	ctx.lr = 0x831FE2E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FE2E4: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FE2E8: 4BCB1821  bl 0x82eafb08
	ctx.lr = 0x831FE2EC;
	sub_82EAFB08(ctx, base);
	// 831FE2EC: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 831FE2F0: 409AFF80  bne cr6, 0x831fe270
	if !ctx.cr[6].eq {
	pc = 0x831FE270; continue 'dispatch;
	}
	// 831FE2F4: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FE2F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FE2FC: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831FE300: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FE304: 4E800421  bctrl
	ctx.lr = 0x831FE308;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FE308: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 831FE30C: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831FE310: 552700BE  clrlwi r7, r9, 2
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x3FFFFFFFu64;
	// 831FE314: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 831FE318: 409A0010  bne cr6, 0x831fe328
	if !ctx.cr[6].eq {
	pc = 0x831FE328; continue 'dispatch;
	}
	// 831FE31C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831FE320: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FE324: 4BCA855D  bl 0x82ea6880
	ctx.lr = 0x831FE328;
	sub_82EA6880(ctx, base);
	// 831FE328: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831FE32C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FE330: 7F6A59AE  stbx r27, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[27].u8) };
	// 831FE334: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FE338: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831FE33C: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831FE340: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831FE344: 39090001  addi r8, r9, 1
	ctx.r[8].s64 = ctx.r[9].s64 + 1;
	// 831FE348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 831FE34C: 419A0060  beq cr6, 0x831fe3ac
	if ctx.cr[6].eq {
	pc = 0x831FE3AC; continue 'dispatch;
	}
	// 831FE350: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FE354: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FE358: 419A0054  beq cr6, 0x831fe3ac
	if ctx.cr[6].eq {
	pc = 0x831FE3AC; continue 'dispatch;
	}
	// 831FE35C: 4BCAC2E5  bl 0x82eaa640
	ctx.lr = 0x831FE360;
	sub_82EAA640(ctx, base);
	// 831FE360: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FE364: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 831FE368: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 831FE36C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 831FE370: 40980024  bge cr6, 0x831fe394
	if !ctx.cr[6].lt {
	pc = 0x831FE394; continue 'dispatch;
	}
	// 831FE374: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FE378: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831FE37C: 41980008  blt cr6, 0x831fe384
	if ctx.cr[6].lt {
	pc = 0x831FE384; continue 'dispatch;
	}
	// 831FE380: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 831FE384: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831FE388: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 831FE38C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831FE390: 4BCA8469  bl 0x82ea67f8
	ctx.lr = 0x831FE394;
	sub_82EA67F8(ctx, base);
	// 831FE394: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 831FE398: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831FE39C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831FE3A0: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FE3A4: 4BCAC3A5  bl 0x82eaa748
	ctx.lr = 0x831FE3A8;
	sub_82EAA748(ctx, base);
	// 831FE3A8: 48000040  b 0x831fe3e8
	pc = 0x831FE3E8; continue 'dispatch;
	// 831FE3AC: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FE3B0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 831FE3B4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831FE3B8: 40980020  bge cr6, 0x831fe3d8
	if !ctx.cr[6].lt {
	pc = 0x831FE3D8; continue 'dispatch;
	}
	// 831FE3BC: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 831FE3C0: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 831FE3C4: 41990008  bgt cr6, 0x831fe3cc
	if ctx.cr[6].gt {
	pc = 0x831FE3CC; continue 'dispatch;
	}
	// 831FE3C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831FE3CC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831FE3D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831FE3D4: 4BCA8425  bl 0x82ea67f8
	ctx.lr = 0x831FE3D8;
	sub_82EA67F8(ctx, base);
	// 831FE3D8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FE3DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 831FE3E0: 915C0004  stw r10, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 831FE3E4: 9B6B0000  stb r27, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 831FE3E8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 831FE3EC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831FE3F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831FE3F4: 409A0020  bne cr6, 0x831fe414
	if !ctx.cr[6].eq {
	pc = 0x831FE414; continue 'dispatch;
	}
	// 831FE3F8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FE3FC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 831FE400: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 831FE404: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831FE408: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 831FE40C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 831FE410: 4BCA23A1  bl 0x82ea07b0
	ctx.lr = 0x831FE414;
	sub_82EA07B0(ctx, base);
	// 831FE414: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831FE418: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831FE41C: 4BFA9D98  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FE420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FE420 size=100
    let mut pc: u32 = 0x831FE420;
    'dispatch: loop {
        match pc {
            0x831FE420 => {
    //   block [0x831FE420..0x831FE484)
	// 831FE420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FE424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FE428: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FE42C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FE430: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FE434: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FE438: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FE43C: 4BFFF1AD  bl 0x831fd5e8
	ctx.lr = 0x831FE440;
	sub_831FD5E8(ctx, base);
	// 831FE440: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 831FE444: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FE448: 419A0020  beq cr6, 0x831fe468
	if ctx.cr[6].eq {
	pc = 0x831FE468; continue 'dispatch;
	}
	// 831FE44C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FE450: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 831FE454: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 831FE458: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FE45C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831FE460: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FE464: 4BCA234D  bl 0x82ea07b0
	ctx.lr = 0x831FE468;
	sub_82EA07B0(ctx, base);
	// 831FE468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FE46C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FE470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FE474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FE478: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FE47C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FE480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FE488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831FE488 size=96
    let mut pc: u32 = 0x831FE488;
    'dispatch: loop {
        match pc {
            0x831FE488 => {
    //   block [0x831FE488..0x831FE4E8)
	// 831FE488: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 831FE48C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831FE490: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 831FE494: C1AB0000  lfs f13, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831FE498: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 831FE49C: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 831FE4A0: 55070050  rlwinm r7, r8, 0, 1, 8
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 831FE4A4: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 831FE4A8: 419A0040  beq cr6, 0x831fe4e8
	if ctx.cr[6].eq {
		sub_831FE4E8(ctx, base);
		return;
	}
	// 831FE4AC: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831FE4B0: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 831FE4B4: 8101FFF4  lwz r8, -0xc(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 831FE4B8: 55070050  rlwinm r7, r8, 0, 1, 8
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 831FE4BC: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 831FE4C0: 419A0028  beq cr6, 0x831fe4e8
	if ctx.cr[6].eq {
		sub_831FE4E8(ctx, base);
		return;
	}
	// 831FE4C4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831FE4C8: 41990020  bgt cr6, 0x831fe4e8
	if ctx.cr[6].gt {
		sub_831FE4E8(ctx, base);
		return;
	}
	// 831FE4CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831FE4D0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831FE4D4: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 831FE4D8: 4198FFBC  blt cr6, 0x831fe494
	if ctx.cr[6].lt {
	pc = 0x831FE494; continue 'dispatch;
	}
	// 831FE4DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831FE4E0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 831FE4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FE4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FE4E8 size=12
    let mut pc: u32 = 0x831FE4E8;
    'dispatch: loop {
        match pc {
            0x831FE4E8 => {
    //   block [0x831FE4E8..0x831FE4F4)
	// 831FE4E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FE4EC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 831FE4F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FE4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831FE4F8 size=12
    let mut pc: u32 = 0x831FE4F8;
    'dispatch: loop {
        match pc {
            0x831FE4F8 => {
    //   block [0x831FE4F8..0x831FE504)
	// 831FE4F8: D025004C  stfs f1, 0x4c(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 831FE4FC: D045005C  stfs f2, 0x5c(r5)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 831FE500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FE508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FE508 size=128
    let mut pc: u32 = 0x831FE508;
    'dispatch: loop {
        match pc {
            0x831FE508 => {
    //   block [0x831FE508..0x831FE588)
	// 831FE508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FE50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FE510: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FE514: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FE518: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FE588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FE588 size=448
    let mut pc: u32 = 0x831FE588;
    'dispatch: loop {
        match pc {
            0x831FE588 => {
    //   block [0x831FE588..0x831FE748)
	// 831FE588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FE58C: 4BFA9BDD  bl 0x831a8168
	ctx.lr = 0x831FE590;
	sub_831A8130(ctx, base);
	// 831FE590: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FE594: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831FE598: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 831FE59C: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
	// 831FE5A0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 831FE5A4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 831FE5A8: C01E000C  lfs f0, 0xc(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831FE5AC: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FE748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FE748 size=452
    let mut pc: u32 = 0x831FE748;
    'dispatch: loop {
        match pc {
            0x831FE748 => {
    //   block [0x831FE748..0x831FE90C)
	// 831FE748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FE74C: 4BFA9A1D  bl 0x831a8168
	ctx.lr = 0x831FE750;
	sub_831A8130(ctx, base);
	// 831FE750: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FE754: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831FE758: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 831FE75C: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
	// 831FE760: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 831FE764: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 831FE768: C01E000C  lfs f0, 0xc(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831FE76C: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FE910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FE910 size=432
    let mut pc: u32 = 0x831FE910;
    'dispatch: loop {
        match pc {
            0x831FE910 => {
    //   block [0x831FE910..0x831FEAC0)
	// 831FE910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FE914: 4BFA9855  bl 0x831a8168
	ctx.lr = 0x831FE918;
	sub_831A8130(ctx, base);
	// 831FE918: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FE91C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831FE920: D0210054  stfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 831FE924: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 831FE928: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
	// 831FE92C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 831FE930: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 831FE934: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FEAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FEAC0 size=512
    let mut pc: u32 = 0x831FEAC0;
    'dispatch: loop {
        match pc {
            0x831FEAC0 => {
    //   block [0x831FEAC0..0x831FECC0)
	// 831FEAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FEAC4: 4BFA96A1  bl 0x831a8164
	ctx.lr = 0x831FEAC8;
	sub_831A8130(ctx, base);
	// 831FEAC8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FEACC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831FEAD0: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
	// 831FEAD4: 3BDF0040  addi r30, r31, 0x40
	ctx.r[30].s64 = ctx.r[31].s64 + 64;
	// 831FEAD8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 831FEADC: 397E0030  addi r11, r30, 0x30
	ctx.r[11].s64 = ctx.r[30].s64 + 48;
	// 831FEAE0: C01F004C  lfs f0, 0x4c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831FEAE4: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 831FEAE8: EDA10028  fsubs f13, f1, f0
	ctx.f[13].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 831FEAEC: C19F005C  lfs f12, 0x5c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FECC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831FECC0 size=500
    let mut pc: u32 = 0x831FECC0;
    'dispatch: loop {
        match pc {
            0x831FECC0 => {
    //   block [0x831FECC0..0x831FEEB4)
	// 831FECC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FECC4: 4BFA94A1  bl 0x831a8164
	ctx.lr = 0x831FECC8;
	sub_831A8130(ctx, base);
	// 831FECC8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FECCC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FECD0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 831FECD4: 3BFE0040  addi r31, r30, 0x40
	ctx.r[31].s64 = ctx.r[30].s64 + 64;
	// 831FECD8: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 831FECDC: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 831FECE0: C01E004C  lfs f0, 0x4c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831FECE4: 393F0020  addi r9, r31, 0x20
	ctx.r[9].s64 = ctx.r[31].s64 + 32;
	// 831FECE8: EDA10028  fsubs f13, f1, f0
	ctx.f[13].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 831FECEC: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 831FECF0: C19E005C  lfs f12, 0x5c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831FECF4: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
	// 831FECF8: 3C608201  lis r3, -0x7dff
	ctx.r[3].s64 = -2113863680;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FEEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FEEB8 size=172
    let mut pc: u32 = 0x831FEEB8;
    'dispatch: loop {
        match pc {
            0x831FEEB8 => {
    //   block [0x831FEEB8..0x831FEF64)
	// 831FEEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FEEBC: 4BFA92A9  bl 0x831a8164
	ctx.lr = 0x831FEEC0;
	sub_831A8130(ctx, base);
	// 831FEEC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FEEC4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 831FEEC8: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
	// 831FEECC: 397F0090  addi r11, r31, 0x90
	ctx.r[11].s64 = ctx.r[31].s64 + 144;
	// 831FEED0: 3BDF0040  addi r30, r31, 0x40
	ctx.r[30].s64 = ctx.r[31].s64 + 64;
	// 831FEED4: 3BA00030  li r29, 0x30
	ctx.r[29].s64 = 48;
	// 831FEED8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 831FEEDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FEF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FEF68 size=212
    let mut pc: u32 = 0x831FEF68;
    'dispatch: loop {
        match pc {
            0x831FEF68 => {
    //   block [0x831FEF68..0x831FF03C)
	// 831FEF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FEF6C: 4BFA9201  bl 0x831a816c
	ctx.lr = 0x831FEF70;
	sub_831A8130(ctx, base);
	// 831FEF70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FEF74: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831FEF78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831FEF7C: 397F0090  addi r11, r31, 0x90
	ctx.r[11].s64 = ctx.r[31].s64 + 144;
	// 831FEF80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831FEF84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831FEF88: 3BBF0040  addi r29, r31, 0x40
	ctx.r[29].s64 = ctx.r[31].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FF040 size=132
    let mut pc: u32 = 0x831FF040;
    'dispatch: loop {
        match pc {
            0x831FF040 => {
    //   block [0x831FF040..0x831FF0C4)
	// 831FF040: 39440090  addi r10, r4, 0x90
	ctx.r[10].s64 = ctx.r[4].s64 + 144;
	// 831FF044: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 831FF048: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 831FF04C: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 831FF050: 39640040  addi r11, r4, 0x40
	ctx.r[11].s64 = ctx.r[4].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FF0C8 size=16
    let mut pc: u32 = 0x831FF0C8;
    'dispatch: loop {
        match pc {
            0x831FF0C8 => {
    //   block [0x831FF0C8..0x831FF0D8)
	// 831FF0C8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831FF0CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 831FF0D0: 38650030  addi r3, r5, 0x30
	ctx.r[3].s64 = ctx.r[5].s64 + 48;
	// 831FF0D4: 4BFFFDE4  b 0x831feeb8
	sub_831FEEB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FF0D8 size=504
    let mut pc: u32 = 0x831FF0D8;
    'dispatch: loop {
        match pc {
            0x831FF0D8 => {
    //   block [0x831FF0D8..0x831FF2D0)
	// 831FF0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FF0DC: 4BFA9089  bl 0x831a8164
	ctx.lr = 0x831FF0E0;
	sub_831A8130(ctx, base);
	// 831FF0E0: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 831FF0E4: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FF2D0 size=136
    let mut pc: u32 = 0x831FF2D0;
    'dispatch: loop {
        match pc {
            0x831FF2D0 => {
    //   block [0x831FF2D0..0x831FF358)
	// 831FF2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FF2D4: 4BFA8E8D  bl 0x831a8160
	ctx.lr = 0x831FF2D8;
	sub_831A8130(ctx, base);
	// 831FF2D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FF2DC: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF2E0: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 831FF2E4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 831FF2E8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 831FF2EC: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 831FF2F0: 7C9BF1D6  mullw r4, r27, r30
	ctx.r[4].s64 = (ctx.r[27].s32 as i64) * (ctx.r[30].s32 as i64);
	// 831FF2F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FF2F8: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 831FF2FC: 4BCA1435  bl 0x82ea0730
	ctx.lr = 0x831FF300;
	sub_82EA0730(ctx, base);
	// 831FF300: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FF304: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF308: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 831FF30C: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 831FF310: 4BFA9201  bl 0x831a8510
	ctx.lr = 0x831FF314;
	sub_831A8510(ctx, base);
	// 831FF314: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 831FF318: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831FF31C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831FF320: 409A001C  bne cr6, 0x831ff33c
	if !ctx.cr[6].eq {
	pc = 0x831FF33C; continue 'dispatch;
	}
	// 831FF324: 556B04BE  clrlwi r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 831FF328: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF32C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 831FF330: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 831FF334: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 831FF338: 4BCA1479  bl 0x82ea07b0
	ctx.lr = 0x831FF33C;
	sub_82EA07B0(ctx, base);
	// 831FF33C: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 831FF340: 935F0000  stw r26, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 831FF344: 556A0462  rlwinm r10, r11, 0, 0x11, 0x11
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831FF348: 7D49DB78  or r9, r10, r27
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[27].u64;
	// 831FF34C: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 831FF350: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831FF354: 4BFA8E5C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FF358 size=152
    let mut pc: u32 = 0x831FF358;
    'dispatch: loop {
        match pc {
            0x831FF358 => {
    //   block [0x831FF358..0x831FF3F0)
	// 831FF358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FF35C: 4BFA8E05  bl 0x831a8160
	ctx.lr = 0x831FF360;
	sub_831A8130(ctx, base);
	// 831FF360: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FF364: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FF368: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FF36C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FF370: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FF374: 557A083C  slwi r26, r11, 1
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 831FF378: 409A0008  bne cr6, 0x831ff380
	if !ctx.cr[6].eq {
	pc = 0x831FF380; continue 'dispatch;
	}
	// 831FF37C: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 831FF380: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF384: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 831FF388: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 831FF38C: 7C9AF1D6  mullw r4, r26, r30
	ctx.r[4].s64 = (ctx.r[26].s32 as i64) * (ctx.r[30].s32 as i64);
	// 831FF390: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 831FF394: 4BCA139D  bl 0x82ea0730
	ctx.lr = 0x831FF398;
	sub_82EA0730(ctx, base);
	// 831FF398: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FF39C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF3A0: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 831FF3A4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 831FF3A8: 4BFA9169  bl 0x831a8510
	ctx.lr = 0x831FF3AC;
	sub_831A8510(ctx, base);
	// 831FF3AC: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 831FF3B0: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831FF3B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831FF3B8: 409A001C  bne cr6, 0x831ff3d4
	if !ctx.cr[6].eq {
	pc = 0x831FF3D4; continue 'dispatch;
	}
	// 831FF3BC: 556B04BE  clrlwi r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 831FF3C0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF3C4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 831FF3C8: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 831FF3CC: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 831FF3D0: 4BCA13E1  bl 0x82ea07b0
	ctx.lr = 0x831FF3D4;
	sub_82EA07B0(ctx, base);
	// 831FF3D4: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 831FF3D8: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 831FF3DC: 556A0462  rlwinm r10, r11, 0, 0x11, 0x11
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831FF3E0: 7D49D378  or r9, r10, r26
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[26].u64;
	// 831FF3E4: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 831FF3E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831FF3EC: 4BFA8DC4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FF3F0 size=4
    let mut pc: u32 = 0x831FF3F0;
    'dispatch: loop {
        match pc {
            0x831FF3F0 => {
    //   block [0x831FF3F0..0x831FF3F4)
	// 831FF3F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FF3F8 size=4
    let mut pc: u32 = 0x831FF3F8;
    'dispatch: loop {
        match pc {
            0x831FF3F8 => {
    //   block [0x831FF3F8..0x831FF3FC)
	// 831FF3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FF400 size=12
    let mut pc: u32 = 0x831FF400;
    'dispatch: loop {
        match pc {
            0x831FF400 => {
    //   block [0x831FF400..0x831FF40C)
	// 831FF400: 3960FFD1  li r11, -0x2f
	ctx.r[11].s64 = -47;
	// 831FF404: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FF408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FF410 size=12
    let mut pc: u32 = 0x831FF410;
    'dispatch: loop {
        match pc {
            0x831FF410 => {
    //   block [0x831FF410..0x831FF41C)
	// 831FF410: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF414: 2F0BFFD1  cmpwi cr6, r11, -0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, -47, &mut ctx.xer);
	// 831FF418: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF41C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FF41C size=20
    let mut pc: u32 = 0x831FF41C;
    'dispatch: loop {
        match pc {
            0x831FF41C => {
    //   block [0x831FF41C..0x831FF430)
	// 831FF41C: 3960FFF1  li r11, -0xf
	ctx.r[11].s64 = -15;
	// 831FF420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831FF424: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FF428: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 831FF42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FF430 size=28
    let mut pc: u32 = 0x831FF430;
    'dispatch: loop {
        match pc {
            0x831FF430 => {
    //   block [0x831FF430..0x831FF44C)
	// 831FF430: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF434: 3940FFD1  li r10, -0x2f
	ctx.r[10].s64 = -47;
	// 831FF438: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831FF43C: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 831FF440: 5507DFFE  rlwinm r7, r8, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 831FF444: 68E30001  xori r3, r7, 1
	ctx.r[3].u64 = ctx.r[7].u64 ^ 1;
	// 831FF448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FF450 size=360
    let mut pc: u32 = 0x831FF450;
    'dispatch: loop {
        match pc {
            0x831FF450 => {
    //   block [0x831FF450..0x831FF550)
	// 831FF450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FF454: 4BFA8D19  bl 0x831a816c
	ctx.lr = 0x831FF458;
	sub_831A8130(ctx, base);
	// 831FF458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FF45C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FF460: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831FF464: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF468: 396B003F  addi r11, r11, 0x3f
	ctx.r[11].s64 = ctx.r[11].s64 + 63;
	// 831FF46C: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 831FF470: 41990108  bgt cr6, 0x831ff578
	if ctx.cr[6].gt {
	pc = 0x831FF578; continue 'dispatch;
	}
	// 831FF474: 3D808320  lis r12, -0x7ce0
	ctx.r[12].s64 = -2095054848;
	// 831FF478: 398CF48C  addi r12, r12, -0xb74
	ctx.r[12].s64 = ctx.r[12].s64 + -2932;
	// 831FF47C: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 831FF480: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 831FF484: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831FF488: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x831FF550; continue 'dispatch;
		},
		1 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		2 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		3 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		4 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		5 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		6 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		7 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		8 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		9 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		10 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		11 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		12 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		13 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		14 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		15 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		16 => {
	pc = 0x831FF5B0; continue 'dispatch;
		},
		17 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		18 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		19 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		20 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		21 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		22 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		23 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		24 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		25 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		26 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		27 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		28 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		29 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		30 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		31 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		32 => {
	pc = 0x831FF550; continue 'dispatch;
		},
		33 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		34 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		35 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		36 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		37 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		38 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		39 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		40 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		41 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		42 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		43 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		44 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		45 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		46 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		47 => {
	pc = 0x831FF578; continue 'dispatch;
		},
		48 => {
	pc = 0x831FF550; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 831FF48C: 831FF550  lwz r24, -0xab0(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2736 as u32) ) } as u64;
	// 831FF490: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF494: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF498: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF49C: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4A0: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4A4: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4A8: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4AC: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4B0: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4B4: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4B8: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4BC: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4C0: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4C4: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4C8: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4CC: 831FF5B0  lwz r24, -0xa50(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2640 as u32) ) } as u64;
	// 831FF4D0: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4D4: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4D8: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4DC: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4E0: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4E4: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4E8: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4EC: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4F0: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4F4: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4F8: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF4FC: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF500: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF504: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF508: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF50C: 831FF550  lwz r24, -0xab0(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2736 as u32) ) } as u64;
	// 831FF510: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF514: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF518: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF51C: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF520: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF524: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF528: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF52C: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF530: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF534: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF538: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF53C: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF540: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF544: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF548: 831FF578  lwz r24, -0xa88(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2696 as u32) ) } as u64;
	// 831FF54C: 831FF550  lwz r24, -0xab0(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2736 as u32) ) } as u64;
            }
            0x831FF550 => {
    //   block [0x831FF550..0x831FF578)
	// 831FF550: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831FF554: 3BCBD7C8  addi r30, r11, -0x2838
	ctx.r[30].s64 = ctx.r[11].s64 + -10296;
	// 831FF558: 806BD7C8  lwz r3, -0x2838(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10296 as u32) ) } as u64;
	// 831FF55C: 4BCA2B55  bl 0x82ea20b0
	ctx.lr = 0x831FF560;
	sub_82EA20B0(ctx, base);
	// 831FF560: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 831FF564: 3960FFE1  li r11, -0x1f
	ctx.r[11].s64 = -31;
	// 831FF568: 419A0008  beq cr6, 0x831ff570
	if ctx.cr[6].eq {
	pc = 0x831FF570; continue 'dispatch;
	}
	// 831FF56C: 3960FFC1  li r11, -0x3f
	ctx.r[11].s64 = -63;
	// 831FF570: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FF574: 48000014  b 0x831ff588
	pc = 0x831FF588; continue 'dispatch;
            }
            0x831FF578 => {
    //   block [0x831FF578..0x831FF5B0)
	// 831FF578: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831FF57C: 3BCBD7C8  addi r30, r11, -0x2838
	ctx.r[30].s64 = ctx.r[11].s64 + -10296;
	// 831FF580: 806BD7C8  lwz r3, -0x2838(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10296 as u32) ) } as u64;
	// 831FF584: 4BCA2B2D  bl 0x82ea20b0
	ctx.lr = 0x831FF588;
	sub_82EA20B0(ctx, base);
	// 831FF588: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FF58C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 831FF590: A13F0006  lhz r9, 6(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 831FF594: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 831FF598: 55280C3C  rlwinm r8, r9, 1, 0x10, 0x1e
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x7FFFFFFFu64;
	// 831FF59C: B0FF0004  sth r7, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 831FF5A0: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 831FF5A4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF5A8: F9430020  std r10, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 831FF5AC: 480433B1  bl 0x8324295c
	ctx.lr = 0x831FF5B0;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	pc = 0x831FF5B0; continue 'dispatch;
            }
            0x831FF5B0 => {
    //   block [0x831FF5B0..0x831FF5B8)
	// 831FF5B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FF5B4: 4BFA8C08  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FF5B8 size=148
    let mut pc: u32 = 0x831FF5B8;
    'dispatch: loop {
        match pc {
            0x831FF5B8 => {
    //   block [0x831FF5B8..0x831FF64C)
	// 831FF5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FF5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FF5C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FF5C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FF5C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FF5CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FF5D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF5D4: 2F0BFFD1  cmpwi cr6, r11, -0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, -47, &mut ctx.xer);
	// 831FF5D8: 419A005C  beq cr6, 0x831ff634
	if ctx.cr[6].eq {
	pc = 0x831FF634; continue 'dispatch;
	}
	// 831FF5DC: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831FF5E0: 83CBD7C8  lwz r30, -0x2838(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10296 as u32) ) } as u64;
	// 831FF5E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FF5E8: 4BCA2AC9  bl 0x82ea20b0
	ctx.lr = 0x831FF5EC;
	sub_82EA20B0(ctx, base);
	// 831FF5EC: 4BCAACB5  bl 0x82eaa2a0
	ctx.lr = 0x831FF5F0;
	sub_82EAA2A0(ctx, base);
	// 831FF5F0: A13F0006  lhz r9, 6(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 831FF5F4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FF5F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 831FF5FC: 5526083E  rotlwi r6, r9, 1
	ctx.r[6].u64 = ((ctx.r[9].u32).rotate_left(1)) as u64;
	// 831FF600: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF604: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 831FF608: 510607DE  rlwimi r6, r8, 0, 0x1f, 0xf
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFFF0001) | (ctx.r[6].u64 & 0x000000000000FFFE);
	// 831FF60C: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FF610: B0BF0004  sth r5, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u16 ) };
	// 831FF614: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 831FF618: 2F07FFF1  cmpwi cr6, r7, -0xf
	ctx.cr[6].compare_i32(ctx.r[7].s32, -15, &mut ctx.xer);
	// 831FF61C: 409A0008  bne cr6, 0x831ff624
	if !ctx.cr[6].eq {
	pc = 0x831FF624; continue 'dispatch;
	}
	// 831FF620: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FF624: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831FF628: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FF62C: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 831FF630: 4804332D  bl 0x8324295c
	ctx.lr = 0x831FF634;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 831FF634: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FF638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FF63C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FF640: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FF644: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FF648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FF650 size=128
    let mut pc: u32 = 0x831FF650;
    'dispatch: loop {
        match pc {
            0x831FF650 => {
    //   block [0x831FF650..0x831FF6D0)
	// 831FF650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FF654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FF658: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FF65C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FF660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FF664: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831FF668: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF66C: 2F0BFFD1  cmpwi cr6, r11, -0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, -47, &mut ctx.xer);
	// 831FF670: 419A0038  beq cr6, 0x831ff6a8
	if ctx.cr[6].eq {
	pc = 0x831FF6A8; continue 'dispatch;
	}
	// 831FF674: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831FF678: 83EBD7C8  lwz r31, -0x2838(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10296 as u32) ) } as u64;
	// 831FF67C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FF680: 4BCA2A31  bl 0x82ea20b0
	ctx.lr = 0x831FF684;
	sub_82EA20B0(ctx, base);
	// 831FF684: 4BCAAC1D  bl 0x82eaa2a0
	ctx.lr = 0x831FF688;
	sub_82EAA2A0(ctx, base);
	// 831FF688: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF68C: 5469003E  slwi r9, r3, 0
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FF690: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831FF694: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 831FF698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FF69C: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 831FF6A0: 409A0010  bne cr6, 0x831ff6b0
	if !ctx.cr[6].eq {
	pc = 0x831FF6B0; continue 'dispatch;
	}
	// 831FF6A4: 480432B9  bl 0x8324295c
	ctx.lr = 0x831FF6A8;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 831FF6A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831FF6AC: 4800000C  b 0x831ff6b8
	pc = 0x831FF6B8; continue 'dispatch;
	// 831FF6B0: 480432AD  bl 0x8324295c
	ctx.lr = 0x831FF6B4;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 831FF6B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831FF6B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FF6BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FF6C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FF6C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FF6C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FF6CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FF6D0 size=136
    let mut pc: u32 = 0x831FF6D0;
    'dispatch: loop {
        match pc {
            0x831FF6D0 => {
    //   block [0x831FF6D0..0x831FF758)
	// 831FF6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FF6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FF6D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FF6DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FF6E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FF6E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FF6E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF6EC: 2F0BFFD1  cmpwi cr6, r11, -0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, -47, &mut ctx.xer);
	// 831FF6F0: 419A0050  beq cr6, 0x831ff740
	if ctx.cr[6].eq {
	pc = 0x831FF740; continue 'dispatch;
	}
	// 831FF6F4: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831FF6F8: 83CBD7C8  lwz r30, -0x2838(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10296 as u32) ) } as u64;
	// 831FF6FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FF700: 4BCA29B1  bl 0x82ea20b0
	ctx.lr = 0x831FF704;
	sub_82EA20B0(ctx, base);
	// 831FF704: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FF708: A13F0006  lhz r9, 6(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 831FF70C: 38EBFFFF  addi r7, r11, -1
	ctx.r[7].s64 = ctx.r[11].s64 + -1;
	// 831FF710: 5528F87E  srwi r8, r9, 1
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FF714: 7CE60734  extsh r6, r7
	ctx.r[6].s64 = ctx.r[7].s16 as i64;
	// 831FF718: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 831FF71C: B0DF0004  sth r6, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 831FF720: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831FF724: 409A000C  bne cr6, 0x831ff730
	if !ctx.cr[6].eq {
	pc = 0x831FF730; continue 'dispatch;
	}
	// 831FF728: 3960FFF1  li r11, -0xf
	ctx.r[11].s64 = -15;
	// 831FF72C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FF730: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831FF734: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FF738: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 831FF73C: 48043221  bl 0x8324295c
	ctx.lr = 0x831FF740;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 831FF740: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FF744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FF748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FF74C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FF750: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FF754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FF758 size=136
    let mut pc: u32 = 0x831FF758;
    'dispatch: loop {
        match pc {
            0x831FF758 => {
    //   block [0x831FF758..0x831FF7E0)
	// 831FF758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FF75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FF760: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831FF764: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FF768: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FF76C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FF770: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF774: 2F0BFFD1  cmpwi cr6, r11, -0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, -47, &mut ctx.xer);
	// 831FF778: 419A0050  beq cr6, 0x831ff7c8
	if ctx.cr[6].eq {
	pc = 0x831FF7C8; continue 'dispatch;
	}
	// 831FF77C: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831FF780: 83CBD7C8  lwz r30, -0x2838(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10296 as u32) ) } as u64;
	// 831FF784: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FF788: 4BCA2929  bl 0x82ea20b0
	ctx.lr = 0x831FF78C;
	sub_82EA20B0(ctx, base);
	// 831FF78C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FF790: A13F0006  lhz r9, 6(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 831FF794: 38EBFFFF  addi r7, r11, -1
	ctx.r[7].s64 = ctx.r[11].s64 + -1;
	// 831FF798: 5528F87E  srwi r8, r9, 1
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FF79C: 7CE60734  extsh r6, r7
	ctx.r[6].s64 = ctx.r[7].s16 as i64;
	// 831FF7A0: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 831FF7A4: B0DF0004  sth r6, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 831FF7A8: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831FF7AC: 409A000C  bne cr6, 0x831ff7b8
	if !ctx.cr[6].eq {
	pc = 0x831FF7B8; continue 'dispatch;
	}
	// 831FF7B0: 3960FFF1  li r11, -0xf
	ctx.r[11].s64 = -15;
	// 831FF7B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FF7B8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831FF7BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FF7C0: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 831FF7C4: 48043199  bl 0x8324295c
	ctx.lr = 0x831FF7C8;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 831FF7C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FF7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FF7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FF7D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831FF7D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FF7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FF7E0 size=228
    let mut pc: u32 = 0x831FF7E0;
    'dispatch: loop {
        match pc {
            0x831FF7E0 => {
    //   block [0x831FF7E0..0x831FF8C4)
	// 831FF7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FF7E4: 4BFA8989  bl 0x831a816c
	ctx.lr = 0x831FF7E8;
	sub_831A8130(ctx, base);
	// 831FF7E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FF7EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF7F0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 831FF7F4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FF7F8: 83E30060  lwz r31, 0x60(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 831FF7FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831FF800: 419A001C  beq cr6, 0x831ff81c
	if ctx.cr[6].eq {
	pc = 0x831FF81C; continue 'dispatch;
	}
	// 831FF804: 81630064  lwz r11, 0x64(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 831FF808: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831FF80C: 91630064  stw r11, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 831FF810: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FF814: 91430060  stw r10, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 831FF818: 48000014  b 0x831ff82c
	pc = 0x831FF82C; continue 'dispatch;
	// 831FF81C: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 831FF820: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 831FF824: 4BCA0E3D  bl 0x82ea0660
	ctx.lr = 0x831FF828;
	sub_82EA0660(ctx, base);
	// 831FF828: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FF82C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831FF830: 419A0080  beq cr6, 0x831ff8b0
	if ctx.cr[6].eq {
	pc = 0x831FF8B0; continue 'dispatch;
	}
	// 831FF834: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831FF838: 392003E8  li r9, 0x3e8
	ctx.r[9].s64 = 1000;
	// 831FF83C: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 831FF840: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 831FF844: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 831FF848: 3BBF0028  addi r29, r31, 0x28
	ctx.r[29].s64 = ctx.r[31].s64 + 40;
	// 831FF84C: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 831FF850: 3BCB70B0  addi r30, r11, 0x70b0
	ctx.r[30].s64 = ctx.r[11].s64 + 28848;
	// 831FF854: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FF858: 4BCA2859  bl 0x82ea20b0
	ctx.lr = 0x831FF85C;
	sub_82EA20B0(ctx, base);
	// 831FF85C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 831FF860: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 831FF864: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FF868: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831FF86C: 419A0008  beq cr6, 0x831ff874
	if ctx.cr[6].eq {
	pc = 0x831FF874; continue 'dispatch;
	}
	// 831FF870: 93EB002C  stw r31, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[31].u32 ) };
	// 831FF874: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831FF878: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 831FF87C: 93FE0030  stw r31, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[31].u32 ) };
	// 831FF880: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831FF884: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 831FF888: 480430D5  bl 0x8324295c
	ctx.lr = 0x831FF88C;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 831FF88C: 388003E8  li r4, 0x3e8
	ctx.r[4].s64 = 1000;
	// 831FF890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FF894: 480435B9  bl 0x83242e4c
	ctx.lr = 0x831FF898;
	// extern call 0x83242E4C → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 831FF898: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831FF89C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 831FF8A0: F95F0020  std r10, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 831FF8A4: 93EBD7C8  stw r31, -0x2838(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-10296 as u32), ctx.r[31].u32 ) };
	// 831FF8A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FF8AC: 4BFA8910  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831FF8B0: 3D408343  lis r10, -0x7cbd
	ctx.r[10].s64 = -2092761088;
	// 831FF8B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FF8B8: 916AD7C8  stw r11, -0x2838(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10296 as u32), ctx.r[11].u32 ) };
	// 831FF8BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831FF8C0: 4BFA88FC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FF8C8 size=68
    let mut pc: u32 = 0x831FF8C8;
    'dispatch: loop {
        match pc {
            0x831FF8C8 => {
    //   block [0x831FF8C8..0x831FF90C)
	// 831FF8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FF8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831FF8D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831FF8D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FF8D8: 3FE08343  lis r31, -0x7cbd
	ctx.r[31].s64 = -2092761088;
	// 831FF8DC: 807FD7C8  lwz r3, -0x2838(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-10296 as u32) ) } as u64;
	// 831FF8E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831FF8E4: 419A0014  beq cr6, 0x831ff8f8
	if ctx.cr[6].eq {
	pc = 0x831FF8F8; continue 'dispatch;
	}
	// 831FF8E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831FF8EC: 4BCC82C5  bl 0x82ec7bb0
	ctx.lr = 0x831FF8F0;
	sub_82EC7BB0(ctx, base);
	// 831FF8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FF8F4: 917FD7C8  stw r11, -0x2838(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-10296 as u32), ctx.r[11].u32 ) };
	// 831FF8F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831FF8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831FF900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831FF904: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831FF908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FF910 size=92
    let mut pc: u32 = 0x831FF910;
    'dispatch: loop {
        match pc {
            0x831FF910 => {
    //   block [0x831FF910..0x831FF96C)
	// 831FF910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FF914: 4BFA8859  bl 0x831a816c
	ctx.lr = 0x831FF918;
	sub_831A8130(ctx, base);
	// 831FF918: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FF91C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FF920: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831FF924: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 831FF928: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831FF92C: 48000045  bl 0x831ff970
	ctx.lr = 0x831FF930;
	sub_831FF970(ctx, base);
	// 831FF930: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831FF934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FF938: 4BCA1149  bl 0x82ea0a80
	ctx.lr = 0x831FF93C;
	sub_82EA0A80(ctx, base);
	// 831FF93C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 831FF940: 397F0090  addi r11, r31, 0x90
	ctx.r[11].s64 = ctx.r[31].s64 + 144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FF970 size=76
    let mut pc: u32 = 0x831FF970;
    'dispatch: loop {
        match pc {
            0x831FF970 => {
    //   block [0x831FF970..0x831FF9BC)
	// 831FF970: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FF9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FF9C0 size=292
    let mut pc: u32 = 0x831FF9C0;
    'dispatch: loop {
        match pc {
            0x831FF9C0 => {
    //   block [0x831FF9C0..0x831FFAE4)
	// 831FF9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FF9C4: 4BFA87A5  bl 0x831a8168
	ctx.lr = 0x831FF9C8;
	sub_831A8130(ctx, base);
	// 831FF9C8: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FFAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FFAE8 size=20
    let mut pc: u32 = 0x831FFAE8;
    'dispatch: loop {
        match pc {
            0x831FFAE8 => {
    //   block [0x831FFAE8..0x831FFAFC)
	// 831FFAE8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 831FFAEC: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831FFAF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FFAF4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831FFAF8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FFAFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FFAFC size=36
    let mut pc: u32 = 0x831FFAFC;
    'dispatch: loop {
        match pc {
            0x831FFAFC => {
    //   block [0x831FFAFC..0x831FFB20)
	// 831FFAFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831FFB00: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 831FFB04: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFB08: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FFB0C: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFB10: 7D27512E  stwx r9, r7, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 831FFB14: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831FFB18: 4082FFEC  bne 0x831ffb04
	if !ctx.cr[0].eq {
	pc = 0x831FFB04; continue 'dispatch;
	}
	// 831FFB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FFB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FFB20 size=224
    let mut pc: u32 = 0x831FFB20;
    'dispatch: loop {
        match pc {
            0x831FFB20 => {
    //   block [0x831FFB20..0x831FFC00)
	// 831FFB20: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 831FFB24: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831FFB28: 40990010  ble cr6, 0x831ffb38
	if !ctx.cr[6].gt {
	pc = 0x831FFB38; continue 'dispatch;
	}
	// 831FFB2C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 831FFB30: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 831FFB34: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 831FFB38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFB3C: 5488103A  slwi r8, r4, 2
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FFB40: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 831FFB44: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFB48: 7CC8582E  lwzx r6, r8, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FFB4C: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831FFB50: 41980048  blt cr6, 0x831ffb98
	if ctx.cr[6].lt {
	pc = 0x831FFB98; continue 'dispatch;
	}
	// 831FFB54: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 831FFB58: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FFB5C: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 831FFB60: 7D07582E  lwzx r8, r7, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FFB64: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831FFB68: 4098FFF0  bge cr6, 0x831ffb58
	if !ctx.cr[6].lt {
	pc = 0x831FFB58; continue 'dispatch;
	}
	// 831FFB6C: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831FFB70: 41980028  blt cr6, 0x831ffb98
	if ctx.cr[6].lt {
	pc = 0x831FFB98; continue 'dispatch;
	}
	// 831FFB74: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FFB78: 7D28582E  lwzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FFB7C: 7C88592E  stwx r4, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[4].u32) };
	// 831FFB80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFB84: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FFB88: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFB8C: 7CE8582E  lwzx r7, r8, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FFB90: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 831FFB94: 4098FFE0  bge cr6, 0x831ffb74
	if !ctx.cr[6].lt {
	pc = 0x831FFB74; continue 'dispatch;
	}
	// 831FFB98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFB9C: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FFBA0: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 831FFBA4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFBA8: 7CC8582E  lwzx r6, r8, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FFBAC: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831FFBB0: 41980048  blt cr6, 0x831ffbf8
	if ctx.cr[6].lt {
	pc = 0x831FFBF8; continue 'dispatch;
	}
	// 831FFBB4: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 831FFBB8: 5505103A  slwi r5, r8, 2
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831FFBBC: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 831FFBC0: 7D05582E  lwzx r8, r5, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FFBC4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831FFBC8: 4098FFF0  bge cr6, 0x831ffbb8
	if !ctx.cr[6].lt {
	pc = 0x831FFBB8; continue 'dispatch;
	}
	// 831FFBCC: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831FFBD0: 41980028  blt cr6, 0x831ffbf8
	if ctx.cr[6].lt {
	pc = 0x831FFBF8; continue 'dispatch;
	}
	// 831FFBD4: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FFBD8: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FFBDC: 7CE8592E  stwx r7, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[7].u32) };
	// 831FFBE0: 80C30000  lwz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFBE4: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FFBE8: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFBEC: 7CA8582E  lwzx r5, r8, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FFBF0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 831FFBF4: 4098FFE0  bge cr6, 0x831ffbd4
	if !ctx.cr[6].lt {
	pc = 0x831FFBD4; continue 'dispatch;
	}
	// 831FFBF8: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831FFBFC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FFC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FFC00 size=60
    let mut pc: u32 = 0x831FFC00;
    'dispatch: loop {
        match pc {
            0x831FFC00 => {
    //   block [0x831FFC00..0x831FFC3C)
	// 831FFC00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFC04: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831FFC08: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831FFC0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFC10: 7CC8582E  lwzx r6, r8, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FFC14: 40980028  bge cr6, 0x831ffc3c
	if !ctx.cr[6].lt {
		sub_831FFC3C(ctx, base);
		return;
	}
	// 831FFC18: 5546103A  slwi r6, r10, 2
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831FFC1C: 7CE8582E  lwzx r7, r8, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FFC20: 7D46582E  lwzx r10, r6, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FFC24: 7CAA3A14  add r5, r10, r7
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 831FFC28: 7CA8592E  stwx r5, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[5].u32) };
	// 831FFC2C: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFC30: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFC34: 7D23312E  stwx r9, r3, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[6].u32), ctx.r[9].u32) };
	// 831FFC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FFC3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FFC3C size=32
    let mut pc: u32 = 0x831FFC3C;
    'dispatch: loop {
        match pc {
            0x831FFC3C => {
    //   block [0x831FFC3C..0x831FFC5C)
	// 831FFC3C: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FFC40: 7CE9582E  lwzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FFC44: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 831FFC48: 7CE9592E  stwx r7, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[7].u32) };
	// 831FFC4C: 80C30000  lwz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFC50: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFC54: 7D45412E  stwx r10, r5, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 831FFC58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FFC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FFC60 size=32
    let mut pc: u32 = 0x831FFC60;
    'dispatch: loop {
        match pc {
            0x831FFC60 => {
    //   block [0x831FFC60..0x831FFC80)
	// 831FFC60: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFC64: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FFC68: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831FFC6C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFC70: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831FFC74: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831FFC78: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 831FFC7C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FFC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831FFC80 size=88
    let mut pc: u32 = 0x831FFC80;
    'dispatch: loop {
        match pc {
            0x831FFC80 => {
    //   block [0x831FFC80..0x831FFCD8)
	// 831FFC80: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFC84: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831FFC88: 41980040  blt cr6, 0x831ffcc8
	if ctx.cr[6].lt {
	pc = 0x831FFCC8; continue 'dispatch;
	}
	// 831FFC8C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FFC90: 7CE9502E  lwzx r7, r9, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 831FFC94: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 831FFC98: 41980030  blt cr6, 0x831ffcc8
	if ctx.cr[6].lt {
	pc = 0x831FFCC8; continue 'dispatch;
	}
	// 831FFC9C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFCA0: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FFCA4: 7CC7502E  lwzx r6, r7, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 831FFCA8: 54C4003E  slwi r4, r6, 0
	ctx.r[4].u32 = ctx.r[6].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 831FFCAC: 5489103A  slwi r9, r4, 2
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FFCB0: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 831FFCB4: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFCB8: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFCBC: 7CE9502E  lwzx r7, r9, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 831FFCC0: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 831FFCC4: 4098FFD8  bge cr6, 0x831ffc9c
	if !ctx.cr[6].lt {
	pc = 0x831FFC9C; continue 'dispatch;
	}
	// 831FFCC8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831FFCCC: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 831FFCD0: 409AFFB0  bne cr6, 0x831ffc80
	if !ctx.cr[6].eq {
	pc = 0x831FFC80; continue 'dispatch;
	}
	// 831FFCD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FFCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FFCD8 size=192
    let mut pc: u32 = 0x831FFCD8;
    'dispatch: loop {
        match pc {
            0x831FFCD8 => {
    //   block [0x831FFCD8..0x831FFD98)
	// 831FFCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FFCDC: 4BFA8485  bl 0x831a8160
	ctx.lr = 0x831FFCE0;
	sub_831A8130(ctx, base);
	// 831FFCE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FFCE4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831FFCE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831FFCEC: 4BFFFF75  bl 0x831ffc60
	ctx.lr = 0x831FFCF0;
	sub_831FFC60(ctx, base);
	// 831FFCF0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FFCF4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 831FFCF8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831FFCFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FFD00: 40990090  ble cr6, 0x831ffd90
	if !ctx.cr[6].gt {
	pc = 0x831FFD90; continue 'dispatch;
	}
	// 831FFD04: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831FFD08: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFD0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFD10: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FFD14: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831FFD18: 40980058  bge cr6, 0x831ffd70
	if !ctx.cr[6].lt {
	pc = 0x831FFD70; continue 'dispatch;
	}
	// 831FFD1C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831FFD20: 7F8A00D0  neg r28, r10
	ctx.r[28].s64 = -ctx.r[10].s64;
	// 831FFD24: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FFD28: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 831FFD2C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831FFD30: 409A0010  bne cr6, 0x831ffd40
	if !ctx.cr[6].eq {
	pc = 0x831FFD40; continue 'dispatch;
	}
	// 831FFD34: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 831FFD38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831FFD3C: 4BCA6B45  bl 0x82ea6880
	ctx.lr = 0x831FFD40;
	sub_82EA6880(ctx, base);
	// 831FFD40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FFD44: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFD48: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831FFD4C: 7F89512E  stwx r28, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 831FFD50: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FFD54: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 831FFD58: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 831FFD5C: 80FD0000  lwz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFD60: 80C70000  lwz r6, 0(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFD64: 7F66F12E  stwx r27, r6, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u32) };
	// 831FFD68: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 831FFD6C: 48000010  b 0x831ffd7c
	pc = 0x831FFD7C; continue 'dispatch;
	// 831FFD70: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FFD74: 7D2A582E  lwzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831FFD78: 7D3E592E  stwx r9, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 831FFD7C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FFD80: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 831FFD84: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 831FFD88: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831FFD8C: 4198FF7C  blt cr6, 0x831ffd08
	if ctx.cr[6].lt {
	pc = 0x831FFD08; continue 'dispatch;
	}
	// 831FFD90: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831FFD94: 4BFA841C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FFD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FFD98 size=372
    let mut pc: u32 = 0x831FFD98;
    'dispatch: loop {
        match pc {
            0x831FFD98 => {
    //   block [0x831FFD98..0x831FFF0C)
	// 831FFD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FFD9C: 4BFA83C5  bl 0x831a8160
	ctx.lr = 0x831FFDA0;
	sub_831A8130(ctx, base);
	// 831FFDA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FFDA4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831FFDA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831FFDAC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831FFDB0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831FFDB4: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFDB8: 83FD0004  lwz r31, 4(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FFDBC: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 831FFDC0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFDC4: 40990034  ble cr6, 0x831ffdf8
	if !ctx.cr[6].gt {
	pc = 0x831FFDF8; continue 'dispatch;
	}
	// 831FFDC8: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 831FFDCC: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFDD0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831FFDD4: 4099000C  ble cr6, 0x831ffde0
	if !ctx.cr[6].gt {
	pc = 0x831FFDE0; continue 'dispatch;
	}
	// 831FFDD8: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 831FFDDC: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 831FFDE0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FFDE4: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 831FFDE8: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 831FFDEC: 4198FFE0  blt cr6, 0x831ffdcc
	if ctx.cr[6].lt {
	pc = 0x831FFDCC; continue 'dispatch;
	}
	// 831FFDF0: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 831FFDF4: 409A0010  bne cr6, 0x831ffe04
	if !ctx.cr[6].eq {
	pc = 0x831FFE04; continue 'dispatch;
	}
	// 831FFDF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831FFDFC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831FFE00: 4BFA83B0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 831FFE04: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFE08: 3B600014  li r27, 0x14
	ctx.r[27].s64 = 20;
	// 831FFE0C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 831FFE10: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 831FFE14: 7C7BE02E  lwzx r3, r27, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 831FFE18: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 831FFE1C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 831FFE20: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 831FFE24: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 831FFE28: 41990010  bgt cr6, 0x831ffe38
	if ctx.cr[6].gt {
	pc = 0x831FFE38; continue 'dispatch;
	}
	// 831FFE2C: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 831FFE30: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 831FFE34: 48000018  b 0x831ffe4c
	pc = 0x831FFE4C; continue 'dispatch;
	// 831FFE38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFE3C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FFE40: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FFE44: 4E800421  bctrl
	ctx.lr = 0x831FFE48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FFE48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 831FFE4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FFE50: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831FFE54: 4099001C  ble cr6, 0x831ffe70
	if !ctx.cr[6].gt {
	pc = 0x831FFE70; continue 'dispatch;
	}
	// 831FFE58: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 831FFE5C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831FFE60: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FFE64: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831FFE68: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 831FFE6C: 4198FFF0  blt cr6, 0x831ffe5c
	if ctx.cr[6].lt {
	pc = 0x831FFE5C; continue 'dispatch;
	}
	// 831FFE70: 574A103A  slwi r10, r26, 2
	ctx.r[10].u32 = ctx.r[26].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831FFE74: 93440000  stw r26, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 831FFE78: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831FFE7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FFE80: 7D2A212E  stwx r9, r10, r4
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32), ctx.r[9].u32) };
	// 831FFE84: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFE88: 7CEA402E  lwzx r7, r10, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 831FFE8C: 80C80000  lwz r6, 0(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFE90: 7CCA412E  stwx r6, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[6].u32) };
	// 831FFE94: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFE98: 90E50000  stw r7, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 831FFE9C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FFEA0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831FFEA4: 40990038  ble cr6, 0x831ffedc
	if !ctx.cr[6].gt {
	pc = 0x831FFEDC; continue 'dispatch;
	}
	// 831FFEA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831FFEAC: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFEB0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FFEB4: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFEB8: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 831FFEBC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831FFEC0: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFEC4: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831FFEC8: 7CC7202E  lwzx r6, r7, r4
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 831FFECC: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 831FFED0: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FFED4: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 831FFED8: 4198FFD4  blt cr6, 0x831ffeac
	if ctx.cr[6].lt {
	pc = 0x831FFEAC; continue 'dispatch;
	}
	// 831FFEDC: 7C7BE02E  lwzx r3, r27, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 831FFEE0: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 831FFEE4: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 831FFEE8: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831FFEEC: 409A0014  bne cr6, 0x831fff00
	if !ctx.cr[6].eq {
	pc = 0x831FFF00; continue 'dispatch;
	}
	// 831FFEF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFEF4: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831FFEF8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FFEFC: 4E800421  bctrl
	ctx.lr = 0x831FFF00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FFF00: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 831FFF04: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831FFF08: 4BFA82A8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831FFF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831FFF10 size=416
    let mut pc: u32 = 0x831FFF10;
    'dispatch: loop {
        match pc {
            0x831FFF10 => {
    //   block [0x831FFF10..0x832000B0)
	// 831FFF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831FFF14: 4BFA824D  bl 0x831a8160
	ctx.lr = 0x831FFF18;
	sub_831A8130(ctx, base);
	// 831FFF18: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831FFF1C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FFF20: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831FFF24: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831FFF28: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 831FFF2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FFF30: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831FFF34: 4099003C  ble cr6, 0x831fff70
	if !ctx.cr[6].gt {
	pc = 0x831FFF70; continue 'dispatch;
	}
	// 831FFF38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831FFF3C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFF40: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831FFF44: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFF48: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFF4C: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 831FFF50: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831FFF54: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFF58: 54E6103A  slwi r6, r7, 2
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831FFF5C: 7CA6402E  lwzx r5, r6, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 831FFF60: 90A90000  stw r5, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 831FFF64: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FFF68: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 831FFF6C: 4198FFD0  blt cr6, 0x831fff3c
	if ctx.cr[6].lt {
	pc = 0x831FFF3C; continue 'dispatch;
	}
	// 831FFF70: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFF74: 3B600014  li r27, 0x14
	ctx.r[27].s64 = 20;
	// 831FFF78: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 831FFF7C: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 831FFF80: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 831FFF84: 83E30020  lwz r31, 0x20(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 831FFF88: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 831FFF8C: 7D7F2214  add r11, r31, r4
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 831FFF90: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831FFF94: 4199000C  bgt cr6, 0x831fffa0
	if ctx.cr[6].gt {
	pc = 0x831FFFA0; continue 'dispatch;
	}
	// 831FFF98: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 831FFF9C: 48000018  b 0x831fffb4
	pc = 0x831FFFB4; continue 'dispatch;
	// 831FFFA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFFA4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831FFFA8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831FFFAC: 4E800421  bctrl
	ctx.lr = 0x831FFFB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831FFFB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831FFFB4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 831FFFB8: 40990024  ble cr6, 0x831fffdc
	if !ctx.cr[6].gt {
	pc = 0x831FFFDC; continue 'dispatch;
	}
	// 831FFFBC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 831FFFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831FFFC4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 831FFFC8: 419A0014  beq cr6, 0x831fffdc
	if ctx.cr[6].eq {
	pc = 0x831FFFDC; continue 'dispatch;
	}
	// 831FFFCC: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 831FFFD0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831FFFD4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831FFFD8: 4200FFF8  bdnz 0x831fffd0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831FFFD0; continue 'dispatch;
	}
	// 831FFFDC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831FFFE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831FFFE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831FFFE8: 4099003C  ble cr6, 0x83200024
	if !ctx.cr[6].gt {
	pc = 0x83200024; continue 'dispatch;
	}
	// 831FFFEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831FFFF0: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFFF4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831FFFF8: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831FFFFC: 7CE9582E  lwzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83200000: 7D0B402E  lwzx r8, r11, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 83200004: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83200008: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8320000C: 7CE9F82E  lwzx r7, r9, r31
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83200010: 7CC83A14  add r6, r8, r7
	ctx.r[6].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 83200014: 7CC9F92E  stwx r6, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[6].u32) };
	// 83200018: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8320001C: 7F0A2800  cmpw cr6, r10, r5
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[5].s32, &mut ctx.xer);
	// 83200020: 4198FFD0  blt cr6, 0x831ffff0
	if ctx.cr[6].lt {
	pc = 0x831FFFF0; continue 'dispatch;
	}
	// 83200024: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83200028: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8320002C: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83200030: 40980024  bge cr6, 0x83200054
	if !ctx.cr[6].lt {
	pc = 0x83200054; continue 'dispatch;
	}
	// 83200034: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83200038: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8320003C: 41980008  blt cr6, 0x83200044
	if ctx.cr[6].lt {
	pc = 0x83200044; continue 'dispatch;
	}
	// 83200040: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83200044: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83200048: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8320004C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83200050: 4BCA67A9  bl 0x82ea67f8
	ctx.lr = 0x83200054;
	sub_82EA67F8(ctx, base);
	// 83200054: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 83200058: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8320005C: 40990024  ble cr6, 0x83200080
	if !ctx.cr[6].gt {
	pc = 0x83200080; continue 'dispatch;
	}
	// 83200060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83200064: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 83200068: 7D2BF82E  lwzx r9, r11, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8320006C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83200070: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83200074: 7D2B412E  stwx r9, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 83200078: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8320007C: 4082FFEC  bne 0x83200068
	if !ctx.cr[0].eq {
	pc = 0x83200068; continue 'dispatch;
	}
	// 83200080: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83200084: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83200088: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8320008C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83200090: 409A0018  bne cr6, 0x832000a8
	if !ctx.cr[6].eq {
	pc = 0x832000A8; continue 'dispatch;
	}
	// 83200094: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83200098: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8320009C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 832000A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832000A4: 4E800421  bctrl
	ctx.lr = 0x832000A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832000A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832000AC: 4BFA8104  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832000B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832000B0 size=332
    let mut pc: u32 = 0x832000B0;
    'dispatch: loop {
        match pc {
            0x832000B0 => {
    //   block [0x832000B0..0x832001FC)
	// 832000B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832000B4: 4BFA80AD  bl 0x831a8160
	ctx.lr = 0x832000B8;
	sub_831A8130(ctx, base);
	// 832000B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832000BC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 832000C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 832000C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832000C8: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 832000CC: 83FD0004  lwz r31, 4(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 832000D0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 832000D4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 832000D8: 40980024  bge cr6, 0x832000fc
	if !ctx.cr[6].lt {
	pc = 0x832000FC; continue 'dispatch;
	}
	// 832000DC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832000E0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832000E4: 41980008  blt cr6, 0x832000ec
	if ctx.cr[6].lt {
	pc = 0x832000EC; continue 'dispatch;
	}
	// 832000E8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 832000EC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 832000F0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 832000F4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 832000F8: 4BCA6701  bl 0x82ea67f8
	ctx.lr = 0x832000FC;
	sub_82EA67F8(ctx, base);
	// 832000FC: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 83200100: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83200104: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83200108: 409900EC  ble cr6, 0x832001f4
	if !ctx.cr[6].gt {
	pc = 0x832001F4; continue 'dispatch;
	}
	// 8320010C: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 83200110: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 83200114: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 83200118: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8320011C: 7C7AD82E  lwzx r3, r26, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 83200120: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83200124: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83200128: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8320012C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83200130: 41990010  bgt cr6, 0x83200140
	if ctx.cr[6].gt {
	pc = 0x83200140; continue 'dispatch;
	}
	// 83200134: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 83200138: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8320013C: 48000018  b 0x83200154
	pc = 0x83200154; continue 'dispatch;
	// 83200140: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83200144: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83200148: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8320014C: 4E800421  bctrl
	ctx.lr = 0x83200150;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83200150: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83200154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83200158: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8320015C: 40990028  ble cr6, 0x83200184
	if !ctx.cr[6].gt {
	pc = 0x83200184; continue 'dispatch;
	}
	// 83200160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83200164: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 83200168: 7D2B212E  stwx r9, r11, r4
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[9].u32) };
	// 8320016C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83200170: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83200174: 7D08582E  lwzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83200178: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8320017C: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 83200180: 4082FFE8  bne 0x83200168
	if !ctx.cr[0].eq {
	pc = 0x83200168; continue 'dispatch;
	}
	// 83200184: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83200188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8320018C: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83200190: 80FC0000  lwz r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83200194: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83200198: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320019C: 40990034  ble cr6, 0x832001d0
	if !ctx.cr[6].gt {
	pc = 0x832001D0; continue 'dispatch;
	}
	// 832001A0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832001A4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 832001A8: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832001AC: 7D28202E  lwzx r9, r8, r4
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 832001B0: 5526103A  slwi r6, r9, 2
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 832001B4: 38A90001  addi r5, r9, 1
	ctx.r[5].s64 = ctx.r[9].s64 + 1;
	// 832001B8: 7CA8212E  stwx r5, r8, r4
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32), ctx.r[5].u32) };
	// 832001BC: 7D66392E  stwx r11, r6, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[7].u32), ctx.r[11].u32) };
	// 832001C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832001C4: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 832001C8: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 832001CC: 4198FFD4  blt cr6, 0x832001a0
	if ctx.cr[6].lt {
	pc = 0x832001A0; continue 'dispatch;
	}
	// 832001D0: 7C7AD82E  lwzx r3, r26, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 832001D4: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 832001D8: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 832001DC: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832001E0: 409A0014  bne cr6, 0x832001f4
	if !ctx.cr[6].eq {
	pc = 0x832001F4; continue 'dispatch;
	}
	// 832001E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832001E8: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 832001EC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832001F0: 4E800421  bctrl
	ctx.lr = 0x832001F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832001F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832001F8: 4BFA7FB8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83200200 size=152
    let mut pc: u32 = 0x83200200;
    'dispatch: loop {
        match pc {
            0x83200200 => {
    //   block [0x83200200..0x83200298)
	// 83200200: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83200204: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83200208: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 8320020C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 83200210: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83200214: C00A08A8  lfs f0, 0x8a8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83200218: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 8320021C: C1A989AC  lfs f13, -0x7654(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83200220: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 83200224: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83200298 size=76
    let mut pc: u32 = 0x83200298;
    'dispatch: loop {
        match pc {
            0x83200298 => {
    //   block [0x83200298..0x832002E4)
	// 83200298: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 8320029C: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 832002A0: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 832002A4: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 832002A8: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 832002AC: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 832002B0: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832002E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832002E4 size=76
    let mut pc: u32 = 0x832002E4;
    'dispatch: loop {
        match pc {
            0x832002E4 => {
    //   block [0x832002E4..0x83200330)
	// 832002E4: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 832002E8: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 832002EC: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 832002F0: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 832002F4: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 832002F8: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 832002FC: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83200330 size=76
    let mut pc: u32 = 0x83200330;
    'dispatch: loop {
        match pc {
            0x83200330 => {
    //   block [0x83200330..0x8320037C)
	// 83200330: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 83200334: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 83200338: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 8320033C: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 83200340: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83200344: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 83200348: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8320037C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8320037C size=76
    let mut pc: u32 = 0x8320037C;
    'dispatch: loop {
        match pc {
            0x8320037C => {
    //   block [0x8320037C..0x832003C8)
	// 8320037C: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 83200380: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 83200384: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 83200388: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 8320038C: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83200390: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 83200394: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832003C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832003C8 size=76
    let mut pc: u32 = 0x832003C8;
    'dispatch: loop {
        match pc {
            0x832003C8 => {
    //   block [0x832003C8..0x83200414)
	// 832003C8: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 832003CC: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 832003D0: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 832003D4: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 832003D8: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 832003DC: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 832003E0: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200414(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83200414 size=76
    let mut pc: u32 = 0x83200414;
    'dispatch: loop {
        match pc {
            0x83200414 => {
    //   block [0x83200414..0x83200460)
	// 83200414: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 83200418: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8320041C: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 83200420: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 83200424: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83200428: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 8320042C: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83200460 size=76
    let mut pc: u32 = 0x83200460;
    'dispatch: loop {
        match pc {
            0x83200460 => {
    //   block [0x83200460..0x832004AC)
	// 83200460: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 83200464: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 83200468: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 8320046C: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 83200470: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83200474: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 83200478: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832004AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832004AC size=88
    let mut pc: u32 = 0x832004AC;
    'dispatch: loop {
        match pc {
            0x832004AC => {
    //   block [0x832004AC..0x83200504)
	// 832004AC: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 832004B0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832004B4: 4082FD9C  bne 0x83200250
	if !ctx.cr[0].eq {
		sub_83200200(ctx, base);
		return;
	}
	// 832004B8: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 832004BC: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 832004C0: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 832004C4: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 832004C8: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 832004CC: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 832004D0: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200504(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83200504 size=24
    let mut pc: u32 = 0x83200504;
    'dispatch: loop {
        match pc {
            0x83200504 => {
    //   block [0x83200504..0x8320051C)
	// 83200504: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 83200508: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8320050C: 4082FFB0  bne 0x832004bc
	if !ctx.cr[0].eq {
		sub_832004AC(ctx, base);
		return;
	}
	// 83200510: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 83200514: EC2052F8  fmsubs f1, f0, f11, f10
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[11].f64 - ctx.f[10].f64) as f32) as f64);
	// 83200518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83200520 size=148
    let mut pc: u32 = 0x83200520;
    'dispatch: loop {
        match pc {
            0x83200520 => {
    //   block [0x83200520..0x832005B4)
	// 83200520: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83200524: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83200528: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8320052C: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 83200530: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83200534: C00A08A8  lfs f0, 0x8a8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83200538: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8320053C: C1A989AC  lfs f13, -0x7654(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83200540: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83200544: C148E47C  lfs f10, -0x1b84(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-7044 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 83200548: 390A7060  addi r8, r10, 0x7060
	ctx.r[8].s64 = ctx.r[10].s64 + 28768;
	// 8320054C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83200550: FC005090  fmr f0, f10
	ctx.f[0].f64 = ctx.f[10].f64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832005B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832005B4 size=84
    let mut pc: u32 = 0x832005B4;
    'dispatch: loop {
        match pc {
            0x832005B4 => {
    //   block [0x832005B4..0x83200608)
	// 832005B4: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 832005B8: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 832005BC: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 832005C0: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 832005C4: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 832005C8: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 832005CC: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83200608 size=84
    let mut pc: u32 = 0x83200608;
    'dispatch: loop {
        match pc {
            0x83200608 => {
    //   block [0x83200608..0x8320065C)
	// 83200608: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 8320060C: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 83200610: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 83200614: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 83200618: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 8320061C: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 83200620: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8320065C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8320065C size=84
    let mut pc: u32 = 0x8320065C;
    'dispatch: loop {
        match pc {
            0x8320065C => {
    //   block [0x8320065C..0x832006B0)
	// 8320065C: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 83200660: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 83200664: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 83200668: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 8320066C: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83200670: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 83200674: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832006B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832006B0 size=84
    let mut pc: u32 = 0x832006B0;
    'dispatch: loop {
        match pc {
            0x832006B0 => {
    //   block [0x832006B0..0x83200704)
	// 832006B0: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 832006B4: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 832006B8: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 832006BC: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 832006C0: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 832006C4: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 832006C8: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200704(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83200704 size=84
    let mut pc: u32 = 0x83200704;
    'dispatch: loop {
        match pc {
            0x83200704 => {
    //   block [0x83200704..0x83200758)
	// 83200704: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 83200708: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8320070C: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 83200710: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 83200714: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83200718: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 8320071C: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83200758 size=84
    let mut pc: u32 = 0x83200758;
    'dispatch: loop {
        match pc {
            0x83200758 => {
    //   block [0x83200758..0x832007AC)
	// 83200758: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 8320075C: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 83200760: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 83200764: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 83200768: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 8320076C: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 83200770: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832007AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832007AC size=84
    let mut pc: u32 = 0x832007AC;
    'dispatch: loop {
        match pc {
            0x832007AC => {
    //   block [0x832007AC..0x83200800)
	// 832007AC: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 832007B0: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 832007B4: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 832007B8: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 832007BC: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 832007C0: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 832007C4: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83200800 size=96
    let mut pc: u32 = 0x83200800;
    'dispatch: loop {
        match pc {
            0x83200800 => {
    //   block [0x83200800..0x83200860)
	// 83200800: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 83200804: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83200808: 4082FD5C  bne 0x83200564
	if !ctx.cr[0].eq {
		sub_83200520(ctx, base);
		return;
	}
	// 8320080C: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 83200810: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 83200814: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 83200818: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 8320081C: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83200820: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 83200824: D181FFE0  stfs f12, -0x20(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83200860 size=24
    let mut pc: u32 = 0x83200860;
    'dispatch: loop {
        match pc {
            0x83200860 => {
    //   block [0x83200860..0x83200878)
	// 83200860: FC006090  fmr f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[12].f64;
	// 83200864: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83200868: 4082FFA8  bne 0x83200810
	if !ctx.cr[0].eq {
		sub_83200800(ctx, base);
		return;
	}
	// 8320086C: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 83200870: EC2052F8  fmsubs f1, f0, f11, f10
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[11].f64 - ctx.f[10].f64) as f32) as f64);
	// 83200874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83200878 size=80
    let mut pc: u32 = 0x83200878;
    'dispatch: loop {
        match pc {
            0x83200878 => {
    //   block [0x83200878..0x832008C8)
	// 83200878: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832008C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832008C8 size=4
    let mut pc: u32 = 0x832008C8;
    'dispatch: loop {
        match pc {
            0x832008C8 => {
    //   block [0x832008C8..0x832008CC)
	// 832008C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832008D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832008D0 size=256
    let mut pc: u32 = 0x832008D0;
    'dispatch: loop {
        match pc {
            0x832008D0 => {
    //   block [0x832008D0..0x832009D0)
	// 832008D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832008D4: 4BFA7891  bl 0x831a8164
	ctx.lr = 0x832008D8;
	sub_831A8130(ctx, base);
	// 832008D8: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 832008DC: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 832008E0: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 832008E4: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 832008E8: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 832008EC: 9421FB70  stwu r1, -0x490(r1)
	ea = ctx.r[1].u32.wrapping_add(-1168 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832008F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832008F4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 832008F8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 832008FC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83200900: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83200904: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83200908: 40990034  ble cr6, 0x8320093c
	if !ctx.cr[6].gt {
	pc = 0x8320093C; continue 'dispatch;
	}
	// 8320090C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83200910: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83200914: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83200918: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8320091C: 4BCA9AD5  bl 0x82eaa3f0
	ctx.lr = 0x83200920;
	sub_82EAA3F0(ctx, base);
	// 83200920: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83200924: 419A00A4  beq cr6, 0x832009c8
	if ctx.cr[6].eq {
	pc = 0x832009C8; continue 'dispatch;
	}
	// 83200928: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8320092C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83200930: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 83200934: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83200938: 4198FFD8  blt cr6, 0x83200910
	if ctx.cr[6].lt {
	pc = 0x83200910; continue 'dispatch;
	}
	// 8320093C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83200940: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83200944: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 83200948: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8320094C: 90BF000C  stw r5, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 83200950: 419A0078  beq cr6, 0x832009c8
	if ctx.cr[6].eq {
	pc = 0x832009C8; continue 'dispatch;
	}
	// 83200954: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 83200958: 895B0000  lbz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320095C: 392104B8  addi r9, r1, 0x4b8
	ctx.r[9].s64 = ctx.r[1].s64 + 1208;
	// 83200960: 2B0A0023  cmplwi cr6, r10, 0x23
	ctx.cr[6].compare_u32(ctx.r[10].u32, 35 as u32, &mut ctx.xer);
	// 83200964: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83200968: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8320096C: 409A0034  bne cr6, 0x832009a0
	if !ctx.cr[6].eq {
	pc = 0x832009A0; continue 'dispatch;
	}
	// 83200970: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83200974: 388B3340  addi r4, r11, 0x3340
	ctx.r[4].s64 = ctx.r[11].s64 + 13120;
	// 83200978: 4BCA9A31  bl 0x82eaa3a8
	ctx.lr = 0x8320097C;
	sub_82EAA3A8(ctx, base);
	// 8320097C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83200980: 4BCA9CC1  bl 0x82eaa640
	ctx.lr = 0x83200984;
	sub_82EAA640(ctx, base);
	// 83200984: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83200988: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8320098C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83200990: 4BCA5251  bl 0x82ea5be0
	ctx.lr = 0x83200994;
	sub_82EA5BE0(ctx, base);
	// 83200994: 38BB0001  addi r5, r27, 1
	ctx.r[5].s64 = ctx.r[27].s64 + 1;
	// 83200998: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8320099C: 48000008  b 0x832009a4
	pc = 0x832009A4; continue 'dispatch;
	// 832009A0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 832009A4: 38800400  li r4, 0x400
	ctx.r[4].s64 = 1024;
	// 832009A8: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832009AC: 4BCA99AD  bl 0x82eaa358
	ctx.lr = 0x832009B0;
	sub_82EAA358(ctx, base);
	// 832009B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 832009B4: 4BCA9C8D  bl 0x82eaa640
	ctx.lr = 0x832009B8;
	sub_82EAA640(ctx, base);
	// 832009B8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 832009BC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 832009C0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832009C4: 4BCA521D  bl 0x82ea5be0
	ctx.lr = 0x832009C8;
	sub_82EA5BE0(ctx, base);
	// 832009C8: 38210490  addi r1, r1, 0x490
	ctx.r[1].s64 = ctx.r[1].s64 + 1168;
	// 832009CC: 4BFA77E8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832009D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832009D0 size=108
    let mut pc: u32 = 0x832009D0;
    'dispatch: loop {
        match pc {
            0x832009D0 => {
    //   block [0x832009D0..0x83200A3C)
	// 832009D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832009D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832009D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832009DC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 832009E0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 832009E4: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 832009E8: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 832009EC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 832009F0: 4BC9FD41  bl 0x82ea0730
	ctx.lr = 0x832009F4;
	sub_82EA0730(ctx, base);
	// 832009F4: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 832009F8: 3900001C  li r8, 0x1c
	ctx.r[8].s64 = 28;
	// 832009FC: 38E9334C  addi r7, r9, 0x334c
	ctx.r[7].s64 = ctx.r[9].s64 + 13132;
	// 83200A00: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83200A04: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 83200A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83200A0C: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83200A10: 3CA08000  lis r5, -0x8000
	ctx.r[5].s64 = -2147483648;
	// 83200A14: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 83200A18: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83200A1C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83200A20: 90A30018  stw r5, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[5].u32 ) };
	// 83200A24: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83200A28: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83200A2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83200A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83200A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83200A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83200A40 size=152
    let mut pc: u32 = 0x83200A40;
    'dispatch: loop {
        match pc {
            0x83200A40 => {
    //   block [0x83200A40..0x83200AD8)
	// 83200A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83200A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83200A48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83200A4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83200A50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83200A54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83200A58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83200A5C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83200A60: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83200A64: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83200A68: 409A0020  bne cr6, 0x83200a88
	if !ctx.cr[6].eq {
	pc = 0x83200A88; continue 'dispatch;
	}
	// 83200A6C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 83200A70: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 83200A74: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 83200A78: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83200A7C: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83200A80: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83200A84: 4BC9FD2D  bl 0x82ea07b0
	ctx.lr = 0x83200A88;
	sub_82EA07B0(ctx, base);
	// 83200A88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83200A8C: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 83200A90: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 83200A94: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83200A98: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83200A9C: 419A0020  beq cr6, 0x83200abc
	if ctx.cr[6].eq {
	pc = 0x83200ABC; continue 'dispatch;
	}
	// 83200AA0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 83200AA4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 83200AA8: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 83200AAC: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83200AB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83200AB4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83200AB8: 4BC9FCF9  bl 0x82ea07b0
	ctx.lr = 0x83200ABC;
	sub_82EA07B0(ctx, base);
	// 83200ABC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83200AC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83200AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83200AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83200ACC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83200AD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83200AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83200AD8 size=92
    let mut pc: u32 = 0x83200AD8;
    'dispatch: loop {
        match pc {
            0x83200AD8 => {
    //   block [0x83200AD8..0x83200B34)
	// 83200AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83200ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83200AE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83200AE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83200AE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83200AEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83200AF0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83200AF4: 4BCA84FD  bl 0x82ea8ff0
	ctx.lr = 0x83200AF8;
	sub_82EA8FF0(ctx, base);
	// 83200AF8: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 83200AFC: 387E0030  addi r3, r30, 0x30
	ctx.r[3].s64 = ctx.r[30].s64 + 48;
	// 83200B00: 4BCA84F1  bl 0x82ea8ff0
	ctx.lr = 0x83200B04;
	sub_82EA8FF0(ctx, base);
	// 83200B04: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 83200B08: 387E0060  addi r3, r30, 0x60
	ctx.r[3].s64 = ctx.r[30].s64 + 96;
	// 83200B0C: 4BCA84E5  bl 0x82ea8ff0
	ctx.lr = 0x83200B10;
	sub_82EA8FF0(ctx, base);
	// 83200B10: 389F0090  addi r4, r31, 0x90
	ctx.r[4].s64 = ctx.r[31].s64 + 144;
	// 83200B14: 387E0090  addi r3, r30, 0x90
	ctx.r[3].s64 = ctx.r[30].s64 + 144;
	// 83200B18: 4BCA84D9  bl 0x82ea8ff0
	ctx.lr = 0x83200B1C;
	sub_82EA8FF0(ctx, base);
	// 83200B1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83200B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83200B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83200B28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83200B2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83200B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83200B38 size=268
    let mut pc: u32 = 0x83200B38;
    'dispatch: loop {
        match pc {
            0x83200B38 => {
    //   block [0x83200B38..0x83200C44)
	// 83200B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83200B3C: 4BFA761D  bl 0x831a8158
	ctx.lr = 0x83200B40;
	sub_831A8130(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83200C48 size=268
    let mut pc: u32 = 0x83200C48;
    'dispatch: loop {
        match pc {
            0x83200C48 => {
    //   block [0x83200C48..0x83200D54)
	// 83200C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83200C4C: 4BFA750D  bl 0x831a8158
	ctx.lr = 0x83200C50;
	sub_831A8130(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83200D58 size=176
    let mut pc: u32 = 0x83200D58;
    'dispatch: loop {
        match pc {
            0x83200D58 => {
    //   block [0x83200D58..0x83200E08)
	// 83200D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83200D5C: 4BFA73FD  bl 0x831a8158
	ctx.lr = 0x83200D60;
	sub_831A8130(ctx, base);
	// 83200D60: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83200D64: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83200D68: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 83200D6C: 3BE30020  addi r31, r3, 0x20
	ctx.r[31].s64 = ctx.r[3].s64 + 32;
	// 83200D70: 3B200002  li r25, 2
	ctx.r[25].s64 = 2;
	// 83200D74: 3B5B0030  addi r26, r27, 0x30
	ctx.r[26].s64 = ctx.r[27].s64 + 48;
	// 83200D78: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 83200D7C: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 83200D80: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83200D84: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83200D88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83200D8C: 4BCA876D  bl 0x82ea94f8
	ctx.lr = 0x83200D90;
	sub_82EA94F8(ctx, base);
	// 83200D90: 3BBFFFE0  addi r29, r31, -0x20
	ctx.r[29].s64 = ctx.r[31].s64 + -32;
	// 83200D94: 38BE0060  addi r5, r30, 0x60
	ctx.r[5].s64 = ctx.r[30].s64 + 96;
	// 83200D98: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83200D9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83200DA0: 4BCA8759  bl 0x82ea94f8
	ctx.lr = 0x83200DA4;
	sub_82EA94F8(ctx, base);
	// 83200DA4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83200E08 size=224
    let mut pc: u32 = 0x83200E08;
    'dispatch: loop {
        match pc {
            0x83200E08 => {
    //   block [0x83200E08..0x83200EE8)
	// 83200E08: 38C50010  addi r6, r5, 0x10
	ctx.r[6].s64 = ctx.r[5].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83200EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83200EE8 size=1016
    let mut pc: u32 = 0x83200EE8;
    'dispatch: loop {
        match pc {
            0x83200EE8 => {
    //   block [0x83200EE8..0x832012E0)
	// 83200EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83200EEC: 4BFA7245  bl 0x831a8130
	ctx.lr = 0x83200EF0;
	sub_831A8130(ctx, base);
	// 83200EF0: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83200EF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83200EF8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83200EFC: 4BD141ED  bl 0x82f150e8
	ctx.lr = 0x83200F00;
	sub_82F150E8(ctx, base);
	// 83200F00: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 83200F04: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 83200F08: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 83200F0C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 83200F10: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832012E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832012E0 size=4
    let mut pc: u32 = 0x832012E0;
    'dispatch: loop {
        match pc {
            0x832012E0 => {
    //   block [0x832012E0..0x832012E4)
	// 832012E0: 4BFFFC08  b 0x83200ee8
	sub_83200EE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832012E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832012E8 size=72
    let mut pc: u32 = 0x832012E8;
    'dispatch: loop {
        match pc {
            0x832012E8 => {
    //   block [0x832012E8..0x83201330)
	// 832012E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832012EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832012F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832012F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832012F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832012FC: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83201300: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83201304: 419A0008  beq cr6, 0x8320130c
	if ctx.cr[6].eq {
	pc = 0x8320130C; continue 'dispatch;
	}
	// 83201308: 4BCD3DE1  bl 0x82ed50e8
	ctx.lr = 0x8320130C;
	sub_82ED50E8(ctx, base);
	// 8320130C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83201310: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83201314: 419A0008  beq cr6, 0x8320131c
	if ctx.cr[6].eq {
	pc = 0x8320131C; continue 'dispatch;
	}
	// 83201318: 4BCD3DD1  bl 0x82ed50e8
	ctx.lr = 0x8320131C;
	sub_82ED50E8(ctx, base);
	// 8320131C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83201320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83201324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83201328: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8320132C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83201330 size=16
    let mut pc: u32 = 0x83201330;
    'dispatch: loop {
        match pc {
            0x83201330 => {
    //   block [0x83201330..0x83201340)
	// 83201330: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83201334: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83201338: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8320133C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83201340 size=8
    let mut pc: u32 = 0x83201340;
    'dispatch: loop {
        match pc {
            0x83201340 => {
    //   block [0x83201340..0x83201348)
	// 83201340: 4BCD0EB8  b 0x82ed21f8
	sub_82ED21F8(ctx, base);
	return;
	// 83201344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83201348 size=136
    let mut pc: u32 = 0x83201348;
    'dispatch: loop {
        match pc {
            0x83201348 => {
    //   block [0x83201348..0x832013D0)
	// 83201348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320134C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83201350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83201354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83201358: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320135C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83201360: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83201364: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83201368: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8320136C: 419A0028  beq cr6, 0x83201394
	if ctx.cr[6].eq {
	pc = 0x83201394; continue 'dispatch;
	}
	// 83201370: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83201374: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83201378: 419A000C  beq cr6, 0x83201384
	if ctx.cr[6].eq {
	pc = 0x83201384; continue 'dispatch;
	}
	// 8320137C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83201380: 4BCD10D1  bl 0x82ed2450
	ctx.lr = 0x83201384;
	sub_82ED2450(ctx, base);
	// 83201384: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83201388: 4BCD3D81  bl 0x82ed5108
	ctx.lr = 0x8320138C;
	sub_82ED5108(ctx, base);
	// 8320138C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83201390: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83201394: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 83201398: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8320139C: 4BCD3D4D  bl 0x82ed50e8
	ctx.lr = 0x832013A0;
	sub_82ED50E8(ctx, base);
	// 832013A0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832013A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832013A8: 419A0010  beq cr6, 0x832013b8
	if ctx.cr[6].eq {
	pc = 0x832013B8; continue 'dispatch;
	}
	// 832013AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832013B0: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 832013B4: 4BCCD5C5  bl 0x82ece978
	ctx.lr = 0x832013B8;
	sub_82ECE978(ctx, base);
	// 832013B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832013BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832013C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832013C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832013C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832013CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832013D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832013D0 size=136
    let mut pc: u32 = 0x832013D0;
    'dispatch: loop {
        match pc {
            0x832013D0 => {
    //   block [0x832013D0..0x83201458)
	// 832013D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832013D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832013D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832013DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832013E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832013E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832013E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832013EC: 80BF001C  lwz r5, 0x1c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 832013F0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 832013F4: 419A0028  beq cr6, 0x8320141c
	if ctx.cr[6].eq {
	pc = 0x8320141C; continue 'dispatch;
	}
	// 832013F8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832013FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83201400: 419A000C  beq cr6, 0x8320140c
	if ctx.cr[6].eq {
	pc = 0x8320140C; continue 'dispatch;
	}
	// 83201404: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83201408: 4BCD1049  bl 0x82ed2450
	ctx.lr = 0x8320140C;
	sub_82ED2450(ctx, base);
	// 8320140C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83201410: 4BCD3CF9  bl 0x82ed5108
	ctx.lr = 0x83201414;
	sub_82ED5108(ctx, base);
	// 83201414: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83201418: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8320141C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 83201420: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83201424: 4BCD3CC5  bl 0x82ed50e8
	ctx.lr = 0x83201428;
	sub_82ED50E8(ctx, base);
	// 83201428: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8320142C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83201430: 419A0010  beq cr6, 0x83201440
	if ctx.cr[6].eq {
	pc = 0x83201440; continue 'dispatch;
	}
	// 83201434: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83201438: 80BF001C  lwz r5, 0x1c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8320143C: 4BCCD53D  bl 0x82ece978
	ctx.lr = 0x83201440;
	sub_82ECE978(ctx, base);
	// 83201440: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83201444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83201448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8320144C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83201450: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83201454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83201458 size=140
    let mut pc: u32 = 0x83201458;
    'dispatch: loop {
        match pc {
            0x83201458 => {
    //   block [0x83201458..0x832014E4)
	// 83201458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320145C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83201460: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83201464: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83201468: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8320146C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83201470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83201474: 392A0CF4  addi r9, r10, 0xcf4
	ctx.r[9].s64 = ctx.r[10].s64 + 3316;
	// 83201478: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8320147C: 90DF0010  stw r6, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 83201480: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83201484: 909F0018  stw r4, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 83201488: 90BF001C  stw r5, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 8320148C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83201490: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83201494: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 83201498: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8320149C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 832014A0: 409A000C  bne cr6, 0x832014ac
	if !ctx.cr[6].eq {
	pc = 0x832014AC; continue 'dispatch;
	}
	// 832014A4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832014A8: 419A0024  beq cr6, 0x832014cc
	if ctx.cr[6].eq {
	pc = 0x832014CC; continue 'dispatch;
	}
	// 832014AC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832014B0: 419A000C  beq cr6, 0x832014bc
	if ctx.cr[6].eq {
	pc = 0x832014BC; continue 'dispatch;
	}
	// 832014B4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 832014B8: 4BCD3C31  bl 0x82ed50e8
	ctx.lr = 0x832014BC;
	sub_82ED50E8(ctx, base);
	// 832014BC: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 832014C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832014C4: 419A0008  beq cr6, 0x832014cc
	if ctx.cr[6].eq {
	pc = 0x832014CC; continue 'dispatch;
	}
	// 832014C8: 4BCD3C21  bl 0x82ed50e8
	ctx.lr = 0x832014CC;
	sub_82ED50E8(ctx, base);
	// 832014CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832014D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832014D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832014D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832014DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832014E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832014E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832014E8 size=116
    let mut pc: u32 = 0x832014E8;
    'dispatch: loop {
        match pc {
            0x832014E8 => {
    //   block [0x832014E8..0x8320155C)
	// 832014E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832014EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832014F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832014F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832014F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832014FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83201500: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83201504: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83201508: 394B0CF4  addi r10, r11, 0xcf4
	ctx.r[10].s64 = ctx.r[11].s64 + 3316;
	// 8320150C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83201510: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83201514: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83201518: 419A000C  beq cr6, 0x83201524
	if ctx.cr[6].eq {
	pc = 0x83201524; continue 'dispatch;
	}
	// 8320151C: 4BCD3BED  bl 0x82ed5108
	ctx.lr = 0x83201520;
	sub_82ED5108(ctx, base);
	// 83201520: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 83201524: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83201528: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8320152C: 419A000C  beq cr6, 0x83201538
	if ctx.cr[6].eq {
	pc = 0x83201538; continue 'dispatch;
	}
	// 83201530: 4BCD3BD9  bl 0x82ed5108
	ctx.lr = 0x83201534;
	sub_82ED5108(ctx, base);
	// 83201534: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 83201538: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8320153C: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 83201540: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83201544: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83201548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8320154C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83201550: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83201554: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83201558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83201560 size=176
    let mut pc: u32 = 0x83201560;
    'dispatch: loop {
        match pc {
            0x83201560 => {
    //   block [0x83201560..0x83201610)
	// 83201560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83201564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83201568: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8320156C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83201570: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83201574: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83201578: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8320157C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83201580: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83201584: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 83201588: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8320158C: 409A0010  bne cr6, 0x8320159c
	if !ctx.cr[6].eq {
	pc = 0x8320159C; continue 'dispatch;
	}
	// 83201590: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83201594: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83201598: 4BCA52E9  bl 0x82ea6880
	ctx.lr = 0x8320159C;
	sub_82EA6880(ctx, base);
	// 8320159C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832015A0: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 832015A4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832015A8: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832015AC: 7D48492E  stwx r10, r8, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 832015B0: 80FF0008  lwz r7, 8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832015B4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832015B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832015BC: 54E600BE  clrlwi r6, r7, 2
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x3FFFFFFFu64;
	// 832015C0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832015C4: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 832015C8: 409A0010  bne cr6, 0x832015d8
	if !ctx.cr[6].eq {
	pc = 0x832015D8; continue 'dispatch;
	}
	// 832015CC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 832015D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832015D4: 4BCA52AD  bl 0x82ea6880
	ctx.lr = 0x832015D8;
	sub_82EA6880(ctx, base);
	// 832015D8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832015DC: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 832015E0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832015E4: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832015E8: 7D48492E  stwx r10, r8, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 832015EC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832015F0: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 832015F4: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 832015F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832015FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83201600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83201604: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83201608: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8320160C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83201610 size=16
    let mut pc: u32 = 0x83201610;
    'dispatch: loop {
        match pc {
            0x83201610 => {
    //   block [0x83201610..0x83201620)
	// 83201610: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83201614: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83201618: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8320161C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83201620 size=8
    let mut pc: u32 = 0x83201620;
    'dispatch: loop {
        match pc {
            0x83201620 => {
    //   block [0x83201620..0x83201628)
	// 83201620: 4BCD0BD8  b 0x82ed21f8
	sub_82ED21F8(ctx, base);
	return;
	// 83201624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83201628 size=136
    let mut pc: u32 = 0x83201628;
    'dispatch: loop {
        match pc {
            0x83201628 => {
    //   block [0x83201628..0x832016B0)
	// 83201628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320162C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83201630: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83201634: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83201638: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320163C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83201640: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83201644: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83201648: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8320164C: 419A0028  beq cr6, 0x83201674
	if ctx.cr[6].eq {
	pc = 0x83201674; continue 'dispatch;
	}
	// 83201650: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83201654: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83201658: 419A000C  beq cr6, 0x83201664
	if ctx.cr[6].eq {
	pc = 0x83201664; continue 'dispatch;
	}
	// 8320165C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83201660: 4BCD0DF1  bl 0x82ed2450
	ctx.lr = 0x83201664;
	sub_82ED2450(ctx, base);
	// 83201664: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83201668: 4BCD3AA1  bl 0x82ed5108
	ctx.lr = 0x8320166C;
	sub_82ED5108(ctx, base);
	// 8320166C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83201670: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83201674: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 83201678: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8320167C: 4BCD3A6D  bl 0x82ed50e8
	ctx.lr = 0x83201680;
	sub_82ED50E8(ctx, base);
	// 83201680: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83201684: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83201688: 419A0010  beq cr6, 0x83201698
	if ctx.cr[6].eq {
	pc = 0x83201698; continue 'dispatch;
	}
	// 8320168C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83201690: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83201694: 4BCCD2E5  bl 0x82ece978
	ctx.lr = 0x83201698;
	sub_82ECE978(ctx, base);
	// 83201698: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8320169C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832016A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832016A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832016A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832016AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832016B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832016B0 size=104
    let mut pc: u32 = 0x832016B0;
    'dispatch: loop {
        match pc {
            0x832016B0 => {
    //   block [0x832016B0..0x83201718)
	// 832016B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832016B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832016B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832016BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832016C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832016C4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 832016C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832016CC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832016D0: 390A0E5C  addi r8, r10, 0xe5c
	ctx.r[8].s64 = ctx.r[10].s64 + 3676;
	// 832016D4: 90BF0010  stw r5, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 832016D8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832016DC: 909F0018  stw r4, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 832016E0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832016E4: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 832016E8: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 832016EC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832016F0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 832016F4: 419A000C  beq cr6, 0x83201700
	if ctx.cr[6].eq {
	pc = 0x83201700; continue 'dispatch;
	}
	// 832016F8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 832016FC: 4BCD39ED  bl 0x82ed50e8
	ctx.lr = 0x83201700;
	sub_82ED50E8(ctx, base);
	// 83201700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83201704: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83201708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8320170C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83201710: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83201714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83201718 size=88
    let mut pc: u32 = 0x83201718;
    'dispatch: loop {
        match pc {
            0x83201718 => {
    //   block [0x83201718..0x83201770)
	// 83201718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320171C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83201720: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83201724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83201728: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8320172C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83201730: 394B0E5C  addi r10, r11, 0xe5c
	ctx.r[10].s64 = ctx.r[11].s64 + 3676;
	// 83201734: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83201738: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8320173C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83201740: 419A0010  beq cr6, 0x83201750
	if ctx.cr[6].eq {
	pc = 0x83201750; continue 'dispatch;
	}
	// 83201744: 4BCD39C5  bl 0x82ed5108
	ctx.lr = 0x83201748;
	sub_82ED5108(ctx, base);
	// 83201748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8320174C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83201750: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83201754: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 83201758: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8320175C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83201760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83201764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83201768: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8320176C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83201770 size=116
    let mut pc: u32 = 0x83201770;
    'dispatch: loop {
        match pc {
            0x83201770 => {
    //   block [0x83201770..0x832017E4)
	// 83201770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83201774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83201778: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8320177C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83201780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83201784: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83201788: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8320178C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83201790: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83201794: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 83201798: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8320179C: 409A0010  bne cr6, 0x832017ac
	if !ctx.cr[6].eq {
	pc = 0x832017AC; continue 'dispatch;
	}
	// 832017A0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 832017A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832017A8: 4BCA50D9  bl 0x82ea6880
	ctx.lr = 0x832017AC;
	sub_82EA6880(ctx, base);
	// 832017AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832017B0: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 832017B4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832017B8: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832017BC: 7D48492E  stwx r10, r8, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 832017C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832017C4: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 832017C8: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 832017CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832017D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832017D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832017D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832017DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832017E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832017E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832017E8 size=816
    let mut pc: u32 = 0x832017E8;
    'dispatch: loop {
        match pc {
            0x832017E8 => {
    //   block [0x832017E8..0x83201864)
	// 832017E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832017EC: 4BFA696D  bl 0x831a8158
	ctx.lr = 0x832017F0;
	sub_831A8130(ctx, base);
	// 832017F0: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 832017F4: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 832017F8: A1440006  lhz r10, 6(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 832017FC: 3F408200  lis r26, -0x7e00
	ctx.r[26].s64 = -2113929216;
	// 83201800: 556B383E  rotlwi r11, r11, 7
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(7)) as u64;
	// 83201804: 554A383E  rotlwi r10, r10, 7
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(7)) as u64;
	// 83201808: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8320180C: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 83201810: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 83201814: C01A08A4  lfs f0, 0x8a4(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83201818: 3BEB0020  addi r31, r11, 0x20
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	// 8320181C: 3BCA0010  addi r30, r10, 0x10
	ctx.r[30].s64 = ctx.r[10].s64 + 16;
	// 83201820: 3BAA0020  addi r29, r10, 0x20
	ctx.r[29].s64 = ctx.r[10].s64 + 32;
	pc = 0x83201864; continue 'dispatch;
            }
            0x83201864 => {
    //   block [0x83201864..0x832018F0)
	// 83201864: 890B0003  lbz r8, 3(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83201868: 7D1A0774  extsb r26, r8
	ctx.r[26].s64 = ctx.r[8].s8 as i64;
	// 8320186C: 2B1A0018  cmplwi cr6, r26, 0x18
	ctx.cr[6].compare_u32(ctx.r[26].u32, 24 as u32, &mut ctx.xer);
	// 83201870: 4199FFF4  bgt cr6, 0x83201864
	if ctx.cr[6].gt {
	pc = 0x83201864; continue 'dispatch;
	}
	// 83201874: 3D808320  lis r12, -0x7ce0
	ctx.r[12].s64 = -2095054848;
	// 83201878: 398C188C  addi r12, r12, 0x188c
	ctx.r[12].s64 = ctx.r[12].s64 + 6284;
	// 8320187C: 5740103A  slwi r0, r26, 2
	ctx.r[0].u32 = ctx.r[26].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83201880: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83201884: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83201888: 4E800420  bctr
	match ctx.r[26].u64 {
		0 => {
	pc = 0x83201B14; continue 'dispatch;
		},
		1 => {
	pc = 0x83201B14; continue 'dispatch;
		},
		2 => {
	pc = 0x83201914; continue 'dispatch;
		},
		3 => {
	pc = 0x832018F0; continue 'dispatch;
		},
		4 => {
	pc = 0x83201864; continue 'dispatch;
		},
		5 => {
	pc = 0x83201A4C; continue 'dispatch;
		},
		6 => {
	pc = 0x83201AB0; continue 'dispatch;
		},
		7 => {
	pc = 0x832018F0; continue 'dispatch;
		},
		8 => {
	pc = 0x832018F0; continue 'dispatch;
		},
		9 => {
	pc = 0x832018F0; continue 'dispatch;
		},
		10 => {
	pc = 0x83201864; continue 'dispatch;
		},
		11 => {
	pc = 0x832018F0; continue 'dispatch;
		},
		12 => {
	pc = 0x832018F0; continue 'dispatch;
		},
		13 => {
	pc = 0x832018F0; continue 'dispatch;
		},
		14 => {
	pc = 0x832018F0; continue 'dispatch;
		},
		15 => {
	pc = 0x83201920; continue 'dispatch;
		},
		16 => {
	pc = 0x83201920; continue 'dispatch;
		},
		17 => {
	pc = 0x83201920; continue 'dispatch;
		},
		18 => {
	pc = 0x83201920; continue 'dispatch;
		},
		19 => {
	pc = 0x8320198C; continue 'dispatch;
		},
		20 => {
	pc = 0x8320198C; continue 'dispatch;
		},
		21 => {
	pc = 0x83201B14; continue 'dispatch;
		},
		22 => {
	pc = 0x83201B14; continue 'dispatch;
		},
		23 => {
	pc = 0x832018F0; continue 'dispatch;
		},
		24 => {
	pc = 0x832018F0; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8320188C: 83201B14  lwz r25, 0x1b14(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6932u32 ) } as u64;
	// 83201890: 83201B14  lwz r25, 0x1b14(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6932u32 ) } as u64;
	// 83201894: 83201914  lwz r25, 0x1914(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6420u32 ) } as u64;
	// 83201898: 832018F0  lwz r25, 0x18f0(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6384u32 ) } as u64;
	// 8320189C: 83201864  lwz r25, 0x1864(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6244u32 ) } as u64;
	// 832018A0: 83201A4C  lwz r25, 0x1a4c(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6732u32 ) } as u64;
	// 832018A4: 83201AB0  lwz r25, 0x1ab0(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6832u32 ) } as u64;
	// 832018A8: 832018F0  lwz r25, 0x18f0(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6384u32 ) } as u64;
	// 832018AC: 832018F0  lwz r25, 0x18f0(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6384u32 ) } as u64;
	// 832018B0: 832018F0  lwz r25, 0x18f0(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6384u32 ) } as u64;
	// 832018B4: 83201864  lwz r25, 0x1864(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6244u32 ) } as u64;
	// 832018B8: 832018F0  lwz r25, 0x18f0(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6384u32 ) } as u64;
	// 832018BC: 832018F0  lwz r25, 0x18f0(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6384u32 ) } as u64;
	// 832018C0: 832018F0  lwz r25, 0x18f0(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6384u32 ) } as u64;
	// 832018C4: 832018F0  lwz r25, 0x18f0(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6384u32 ) } as u64;
	// 832018C8: 83201920  lwz r25, 0x1920(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6432u32 ) } as u64;
	// 832018CC: 83201920  lwz r25, 0x1920(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6432u32 ) } as u64;
	// 832018D0: 83201920  lwz r25, 0x1920(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6432u32 ) } as u64;
	// 832018D4: 83201920  lwz r25, 0x1920(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6432u32 ) } as u64;
	// 832018D8: 8320198C  lwz r25, 0x198c(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6540u32 ) } as u64;
	// 832018DC: 8320198C  lwz r25, 0x198c(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6540u32 ) } as u64;
	// 832018E0: 83201B14  lwz r25, 0x1b14(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6932u32 ) } as u64;
	// 832018E4: 83201B14  lwz r25, 0x1b14(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6932u32 ) } as u64;
	// 832018E8: 832018F0  lwz r25, 0x18f0(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6384u32 ) } as u64;
	// 832018EC: 832018F0  lwz r25, 0x18f0(0)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, 6384u32 ) } as u64;
            }
            0x832018F0 => {
    //   block [0x832018F0..0x83201914)
	// 832018F0: 7F5A0774  extsb r26, r26
	ctx.r[26].s64 = ctx.r[26].s8 as i64;
	// 832018F4: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 832018F8: 7D08C8AE  lbzx r8, r8, r25
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 832018FC: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 83201900: 890B0003  lbz r8, 3(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83201904: 7D180774  extsb r24, r8
	ctx.r[24].s64 = ctx.r[8].s8 as i64;
	// 83201908: 7F18D000  cmpw cr6, r24, r26
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[26].s32, &mut ctx.xer);
	// 8320190C: 419AFFE8  beq cr6, 0x832018f4
	if ctx.cr[6].eq {
	pc = 0x832018F4; continue 'dispatch;
	}
	// 83201910: 4BFFFF54  b 0x83201864
	pc = 0x83201864; continue 'dispatch;
            }
            0x83201914 => {
    //   block [0x83201914..0x83201920)
	// 83201914: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83201918: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8320191C: 4BFFFF48  b 0x83201864
	pc = 0x83201864; continue 'dispatch;
            }
            0x83201920 => {
    //   block [0x83201920..0x8320198C)
	// 83201920: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 83201924: 419901F0  bgt cr6, 0x83201b14
	if ctx.cr[6].gt {
	pc = 0x83201B14; continue 'dispatch;
	}
	pc = 0x8320198C; continue 'dispatch;
            }
            0x8320198C => {
    //   block [0x8320198C..0x83201A4C)
	// 8320198C: 7F093000  cmpw cr6, r9, r6
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[6].s32, &mut ctx.xer);
	// 83201990: 41990184  bgt cr6, 0x83201b14
	if ctx.cr[6].gt {
	pc = 0x83201B14; continue 'dispatch;
	}
	pc = 0x83201A4C; continue 'dispatch;
            }
            0x83201A4C => {
    //   block [0x83201A4C..0x83201AB0)
	// 83201A4C: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 83201A50: 419900C4  bgt cr6, 0x83201b14
	if ctx.cr[6].gt {
	pc = 0x83201B14; continue 'dispatch;
	}
	pc = 0x83201AB0; continue 'dispatch;
            }
            0x83201AB0 => {
    //   block [0x83201AB0..0x83201B14)
	// 83201AB0: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 83201AB4: 41990060  bgt cr6, 0x83201b14
	if ctx.cr[6].gt {
	pc = 0x83201B14; continue 'dispatch;
	}
	pc = 0x83201B14; continue 'dispatch;
            }
            0x83201B14 => {
    //   block [0x83201B14..0x83201B18)
	// 83201B14: 4BFA6694  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83201B18 size=644
    let mut pc: u32 = 0x83201B18;
    'dispatch: loop {
        match pc {
            0x83201B18 => {
    //   block [0x83201B18..0x83201D9C)
	// 83201B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83201B1C: 4BFA6645  bl 0x831a8160
	ctx.lr = 0x83201B20;
	sub_831A8130(ctx, base);
	// 83201B20: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 83201B24: 4BFA6F4D  bl 0x831a8a70
	ctx.lr = 0x83201B28;
	sub_831A8A40(ctx, base);
	// 83201B28: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83201B2C: C0030090  lfs f0, 0x90(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83201B30: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83201B34: D0040004  stfs f0, 4(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 83201B38: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 83201B3C: C1A3009C  lfs f13, 0x9c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83201B40: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 83201B44: D1A4005C  stfs f13, 0x5c(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 83201B48: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83201B4C: C18300A0  lfs f12, 0xa0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83201B50: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 83201B54: D18400C0  stfs f12, 0xc0(r4)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 83201B58: 3BE10054  addi r31, r1, 0x54
	ctx.r[31].s64 = ctx.r[1].s64 + 84;
	// 83201B5C: C3EB08A8  lfs f31, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 83201B60: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 83201B64: C163009C  lfs f11, 0x9c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 83201B68: ED5F5824  fdivs f10, f31, f11
	ctx.f[10].f64 = ((ctx.f[31].f64 / ctx.f[11].f64) as f32) as f64;
	// 83201B6C: D1440060  stfs f10, 0x60(r4)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 83201B70: FD205090  fmr f9, f10
	ctx.f[9].f64 = ctx.f[10].f64;
	// 83201B74: C10300A0  lfs f8, 0xa0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 83201B78: ECFF4024  fdivs f7, f31, f8
	ctx.f[7].f64 = ((ctx.f[31].f64 / ctx.f[8].f64) as f32) as f64;
	// 83201B7C: FC403890  fmr f2, f7
	ctx.f[2].f64 = ctx.f[7].f64;
	// 83201B80: D0E400C4  stfs f7, 0xc4(r4)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 83201B84: C0C30094  lfs f6, 0x94(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 83201B88: ECA60272  fmuls f5, f6, f9
	ctx.f[5].f64 = (((ctx.f[6].f64 * ctx.f[9].f64) as f32) as f64);
	// 83201B8C: D0A40050  stfs f5, 0x50(r4)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 83201B90: FC802890  fmr f4, f5
	ctx.f[4].f64 = ctx.f[5].f64;
	// 83201B94: C0630098  lfs f3, 0x98(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 83201B98: EC2300B2  fmuls f1, f3, f2
	ctx.f[1].f64 = (((ctx.f[3].f64 * ctx.f[2].f64) as f32) as f64);
	// 83201B9C: FDA00890  fmr f13, f1
	ctx.f[13].f64 = ctx.f[1].f64;
	// 83201BA0: D02400B4  stfs f1, 0xb4(r4)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 83201BA4: EC1F2024  fdivs f0, f31, f4
	ctx.f[0].f64 = ((ctx.f[31].f64 / ctx.f[4].f64) as f32) as f64;
	// 83201BA8: D0040054  stfs f0, 0x54(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 83201BAC: 39230030  addi r9, r3, 0x30
	ctx.r[9].s64 = ctx.r[3].s64 + 48;
	// 83201BB0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83201BB4: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 83201BB8: 39630060  addi r11, r3, 0x60
	ctx.r[11].s64 = ctx.r[3].s64 + 96;
	// 83201BBC: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 83201BC0: 3B600050  li r27, 0x50
	ctx.r[27].s64 = 80;
	// 83201BC4: 39040004  addi r8, r4, 4
	ctx.r[8].s64 = ctx.r[4].s64 + 4;
	// 83201BC8: 3943009C  addi r10, r3, 0x9c
	ctx.r[10].s64 = ctx.r[3].s64 + 156;
	// 83201BCC: ED9F6824  fdivs f12, f31, f13
	ctx.f[12].f64 = ((ctx.f[31].f64 / ctx.f[13].f64) as f32) as f64;
	// 83201BD0: D18400B8  stfs f12, 0xb8(r4)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(184 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83201DA0 size=72
    let mut pc: u32 = 0x83201DA0;
    'dispatch: loop {
        match pc {
            0x83201DA0 => {
    //   block [0x83201DA0..0x83201DE8)
	// 83201DA0: C0040094  lfs f0, 0x94(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83201DA4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83201DA8: C1A40084  lfs f13, 0x84(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83201DAC: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 83201DB0: C1840090  lfs f12, 0x90(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(144 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83201DB4: C1640080  lfs f11, 0x80(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(128 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 83201DB8: ED4C02F2  fmuls f10, f12, f11
	ctx.f[10].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 83201DBC: D006001C  stfs f0, 0x1c(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 83201DC0: C12B08A8  lfs f9, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 83201DC4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83201DC8: D1460020  stfs f10, 0x20(r6)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 83201DCC: ED000032  fmuls f8, f0, f0
	ctx.f[8].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 83201DD0: ECEA42BA  fmadds f7, f10, f10, f8
	ctx.f[7].f64 = (((ctx.f[10].f64 * ctx.f[10].f64 + ctx.f[8].f64) as f32) as f64);
	// 83201DD4: FF074800  fcmpu cr6, f7, f9
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[9].f64);
	// 83201DD8: 40980010  bge cr6, 0x83201de8
	if !ctx.cr[6].lt {
		sub_83201DE8(ctx, base);
		return;
	}
	// 83201DDC: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83201DE0: D0040098  stfs f0, 0x98(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 83201DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83201DE8 size=232
    let mut pc: u32 = 0x83201DE8;
    'dispatch: loop {
        match pc {
            0x83201DE8 => {
    //   block [0x83201DE8..0x83201ED0)
	// 83201DE8: C1A50044  lfs f13, 0x44(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(68 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83201DEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83201DF0: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 83201DF4: C1850000  lfs f12, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83201DF8: ED8C02B2  fmuls f12, f12, f10
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[10].f64) as f32) as f64);
	// 83201DFC: C16B08A4  lfs f11, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 83201E00: FCC05890  fmr f6, f11
	ctx.f[6].f64 = ctx.f[11].f64;
	// 83201E04: FD005890  fmr f8, f11
	ctx.f[8].f64 = ctx.f[11].f64;
	// 83201E08: ECE00032  fmuls f7, f0, f0
	ctx.f[7].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 83201E0C: EDAC3B3A  fmadds f13, f12, f12, f7
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[12].f64 + ctx.f[7].f64) as f32) as f64);
	// 83201E10: FF0D4800  fcmpu cr6, f13, f9
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[9].f64);
	// 83201E14: 41990044  bgt cr6, 0x83201e58
	if ctx.cr[6].gt {
	pc = 0x83201E58; continue 'dispatch;
	}
	// 83201E18: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 83201E1C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83201E20: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83201E24: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 83201E28: 40980030  bge cr6, 0x83201e58
	if !ctx.cr[6].lt {
	pc = 0x83201E58; continue 'dispatch;
	}
	// 83201E2C: C1050040  lfs f8, 0x40(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(64 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 83201E30: FD600090  fmr f11, f0
	ctx.f[11].f64 = ctx.f[0].f64;
	// 83201E34: EC080032  fmuls f0, f8, f0
	ctx.f[0].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 83201E38: C0EB0000  lfs f7, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 83201E3C: FCC06090  fmr f6, f12
	ctx.f[6].f64 = ctx.f[12].f64;
	// 83201E40: ED8702B2  fmuls f12, f7, f10
	ctx.f[12].f64 = (((ctx.f[7].f64 * ctx.f[10].f64) as f32) as f64);
	// 83201E44: FD006890  fmr f8, f13
	ctx.f[8].f64 = ctx.f[13].f64;
	// 83201E48: ECA00032  fmuls f5, f0, f0
	ctx.f[5].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 83201E4C: EDAC2B3A  fmadds f13, f12, f12, f5
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[12].f64 + ctx.f[5].f64) as f32) as f64);
	// 83201E50: FF0D4800  fcmpu cr6, f13, f9
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[9].f64);
	// 83201E54: 4099FFC8  ble cr6, 0x83201e1c
	if !ctx.cr[6].gt {
	pc = 0x83201E1C; continue 'dispatch;
	}
	// 83201E58: EDA0682C  fsqrts f13, f13
	ctx.f[13].f64 = ((ctx.f[13].f64).sqrt() as f32) as f64;
	// 83201E5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83201E60: C0EBC3C8  lfs f7, -0x3c38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15416 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 83201E64: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83201E68: ED00402C  fsqrts f8, f8
	ctx.f[8].f64 = ((ctx.f[8].f64).sqrt() as f32) as f64;
	// 83201E6C: ED4D4828  fsubs f10, f13, f9
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[9].f64) as f32) as f64);
	// 83201E70: ED084828  fsubs f8, f8, f9
	ctx.f[8].f64 = (((ctx.f[8].f64 - ctx.f[9].f64) as f32) as f64);
	// 83201E74: EDAA4028  fsubs f13, f10, f8
	ctx.f[13].f64 = (((ctx.f[10].f64 - ctx.f[8].f64) as f32) as f64);
	// 83201E78: FCA06A10  fabs f5, f13
	ctx.f[5].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 83201E7C: FF053800  fcmpu cr6, f5, f7
	ctx.cr[6].compare_f64(ctx.f[5].f64, ctx.f[7].f64);
	// 83201E80: 41980008  blt cr6, 0x83201e88
	if ctx.cr[6].lt {
	pc = 0x83201E88; continue 'dispatch;
	}
	// 83201E84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83201E88: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83201E8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83201E90: 419A0008  beq cr6, 0x83201e98
	if ctx.cr[6].eq {
	pc = 0x83201E98; continue 'dispatch;
	}
	// 83201E94: FDA03890  fmr f13, f7
	ctx.f[13].f64 = ctx.f[7].f64;
	// 83201E98: ECA96824  fdivs f5, f9, f13
	ctx.f[5].f64 = ((ctx.f[9].f64 / ctx.f[13].f64) as f32) as f64;
	// 83201E9C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83201EA0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83201EA4: C1AB3C80  lfs f13, 0x3c80(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15488 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83201EA8: C0EA0A94  lfs f7, 0xa94(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2708 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 83201EAC: EC850232  fmuls f4, f5, f8
	ctx.f[4].f64 = (((ctx.f[5].f64 * ctx.f[8].f64) as f32) as f64);
	// 83201EB0: ED4502B2  fmuls f10, f5, f10
	ctx.f[10].f64 = (((ctx.f[5].f64 * ctx.f[10].f64) as f32) as f64);
	// 83201EB4: FD002050  fneg f8, f4
	ctx.f[8].u64 = ctx.f[4].u64 ^ 0x8000_0000_0000_0000u64;
	// 83201EB8: EC680032  fmuls f3, f8, f0
	ctx.f[3].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 83201EBC: EC0A1AFA  fmadds f0, f10, f11, f3
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[11].f64 + ctx.f[3].f64) as f32) as f64);
	// 83201EC0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83201EC4: 4098000C  bge cr6, 0x83201ed0
	if !ctx.cr[6].lt {
		sub_83201ED0(ctx, base);
		return;
	}
	// 83201EC8: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 83201ECC: 48000010  b 0x83201edc
	sub_83201ED0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83201ED0 size=44
    let mut pc: u32 = 0x83201ED0;
    'dispatch: loop {
        match pc {
            0x83201ED0 => {
    //   block [0x83201ED0..0x83201EFC)
	// 83201ED0: FF003800  fcmpu cr6, f0, f7
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[7].f64);
	// 83201ED4: 40990008  ble cr6, 0x83201edc
	if !ctx.cr[6].gt {
	pc = 0x83201EDC; continue 'dispatch;
	}
	// 83201ED8: FC003890  fmr f0, f7
	ctx.f[0].f64 = ctx.f[7].f64;
	// 83201EDC: ED6A01B2  fmuls f11, f10, f6
	ctx.f[11].f64 = (((ctx.f[10].f64 * ctx.f[6].f64) as f32) as f64);
	// 83201EE0: ED885B3A  fmadds f12, f8, f12, f11
	ctx.f[12].f64 = (((ctx.f[8].f64 * ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64);
	// 83201EE4: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 83201EE8: 41980018  blt cr6, 0x83201f00
	if ctx.cr[6].lt {
		sub_83201EFC(ctx, base);
		return;
	}
	// 83201EEC: FF0C3800  fcmpu cr6, f12, f7
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[7].f64);
	// 83201EF0: 4099000C  ble cr6, 0x83201efc
	if !ctx.cr[6].gt {
		sub_83201EFC(ctx, base);
		return;
	}
	// 83201EF4: FDA03890  fmr f13, f7
	ctx.f[13].f64 = ctx.f[7].f64;
	// 83201EF8: 48000008  b 0x83201f00
	sub_83201EFC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201EFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83201EFC size=120
    let mut pc: u32 = 0x83201EFC;
    'dispatch: loop {
        match pc {
            0x83201EFC => {
    //   block [0x83201EFC..0x83201F74)
	// 83201EFC: FDA06090  fmr f13, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ctx.f[12].f64;
	// 83201F00: C1840088  lfs f12, 0x88(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(136 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83201F04: ED6C0372  fmuls f11, f12, f13
	ctx.f[11].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 83201F08: C1440080  lfs f10, 0x80(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(128 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 83201F0C: D1640080  stfs f11, 0x80(r4)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 83201F10: C104008C  lfs f8, 0x8c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(140 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 83201F14: C0E4007C  lfs f7, 0x7c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(124 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 83201F18: ECC80032  fmuls f6, f8, f0
	ctx.f[6].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 83201F1C: C0840074  lfs f4, 0x74(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(116 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 83201F20: C0A40084  lfs f5, 0x84(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 83201F24: EC440232  fmuls f2, f4, f8
	ctx.f[2].f64 = (((ctx.f[4].f64 * ctx.f[8].f64) as f32) as f64);
	// 83201F28: C0640068  lfs f3, 0x68(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(104 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 83201F2C: EC2300F2  fmuls f1, f3, f3
	ctx.f[1].f64 = (((ctx.f[3].f64 * ctx.f[3].f64) as f32) as f64);
	// 83201F30: C0040060  lfs f0, 0x60(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83201F34: D0C40084  stfs f6, 0x84(r4)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 83201F38: EDAA5828  fsubs f13, f10, f11
	ctx.f[13].f64 = (((ctx.f[10].f64 - ctx.f[11].f64) as f32) as f64);
	// 83201F3C: D1A40098  stfs f13, 0x98(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 83201F40: ED053028  fsubs f8, f5, f6
	ctx.f[8].f64 = (((ctx.f[5].f64 - ctx.f[6].f64) as f32) as f64);
	// 83201F44: ECC0083A  fmadds f6, f0, f0, f1
	ctx.f[6].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64);
	// 83201F48: FD606890  fmr f11, f13
	ctx.f[11].f64 = ctx.f[13].f64;
	// 83201F4C: ECA20232  fmuls f5, f2, f8
	ctx.f[5].f64 = (((ctx.f[2].f64 * ctx.f[8].f64) as f32) as f64);
	// 83201F50: EC693024  fdivs f3, f9, f6
	ctx.f[3].f64 = ((ctx.f[9].f64 / ctx.f[6].f64) as f32) as f64;
	// 83201F54: ED4702F2  fmuls f10, f7, f11
	ctx.f[10].f64 = (((ctx.f[7].f64 * ctx.f[11].f64) as f32) as f64);
	// 83201F58: ECEA0332  fmuls f7, f10, f12
	ctx.f[7].f64 = (((ctx.f[10].f64 * ctx.f[12].f64) as f32) as f64);
	// 83201F5C: EC8701F2  fmuls f4, f7, f7
	ctx.f[4].f64 = (((ctx.f[7].f64 * ctx.f[7].f64) as f32) as f64);
	// 83201F60: EC45217A  fmadds f2, f5, f5, f4
	ctx.f[2].f64 = (((ctx.f[5].f64 * ctx.f[5].f64 + ctx.f[4].f64) as f32) as f64);
	// 83201F64: EC2300B2  fmuls f1, f3, f2
	ctx.f[1].f64 = (((ctx.f[3].f64 * ctx.f[2].f64) as f32) as f64);
	// 83201F68: EC00082C  fsqrts f0, f1
	ctx.f[0].f64 = ((ctx.f[1].f64).sqrt() as f32) as f64;
	// 83201F6C: D0060008  stfs f0, 8(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 83201F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83201F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83201F78 size=3064
    let mut pc: u32 = 0x83201F78;
    'dispatch: loop {
        match pc {
            0x83201F78 => {
    //   block [0x83201F78..0x83202B70)
	// 83201F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83201F7C: 4BFA61B9  bl 0x831a8134
	ctx.lr = 0x83201F80;
	sub_831A8130(ctx, base);
	// 83201F80: DBA1FF58  stfd f29, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[29].u64 ) };
	// 83201F84: DBC1FF60  stfd f30, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[30].u64 ) };
	// 83201F88: DBE1FF68  stfd f31, -0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.f[31].u64 ) };
	// 83201F8C: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83201F90: 3FE08343  lis r31, -0x7cbd
	ctx.r[31].s64 = -2092761088;
	// 83201F94: 90610274  stw r3, 0x274(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(628 as u32), ctx.r[3].u32 ) };
	// 83201F98: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83201F9C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83201FA0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83201FA4: 9381027C  stw r28, 0x27c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(636 as u32), ctx.r[28].u32 ) };
	// 83201FA8: 93C10284  stw r30, 0x284(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(644 as u32), ctx.r[30].u32 ) };
	// 83201FAC: 897FD7D4  lbz r11, -0x282c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(-10284 as u32) ) } as u64;
	// 83201FB0: 93A1028C  stw r29, 0x28c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(652 as u32), ctx.r[29].u32 ) };
	// 83201FB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83201FB8: 409A001C  bne cr6, 0x83201fd4
	if !ctx.cr[6].eq {
	pc = 0x83201FD4; continue 'dispatch;
	}
	// 83201FBC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83201FC0: 4BD14C39  bl 0x82f16bf8
	ctx.lr = 0x83201FC4;
	sub_82F16BF8(ctx, base);
	// 83201FC4: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 83201FC8: 987FD7D4  stb r3, -0x282c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-10284 as u32), ctx.r[3].u8 ) };
	// 83201FCC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83201FD0: 419A0B8C  beq cr6, 0x83202b5c
	if ctx.cr[6].eq {
	pc = 0x83202B5C; continue 'dispatch;
	}
	// 83201FD4: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 83201FD8: 1001038C  vspltisw v0, 1
	for i in 0..4 {
		ctx.v[0].u32[i] = 1;
	}
	// 83201FDC: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 83201FE0: 10C0038C  vspltisw v6, 0
	for i in 0..4 {
		ctx.v[6].u32[i] = 0;
	}
	// 83201FE4: 397E0150  addi r11, r30, 0x150
	ctx.r[11].s64 = ctx.r[30].s64 + 336;
	// 83201FE8: 3B5E00D0  addi r26, r30, 0xd0
	ctx.r[26].s64 = ctx.r[30].s64 + 208;
	// 83201FEC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 83201FF0: 10A0030A  vcfux v5, v0, 0
	// vcfux/vcuxwfp128: ctx.v[5].f32[i] = ( ctx.v[0].u32[i] as f32 ) * (2.0f32).powi(0);
	for i in 0..4 { ctx.v[5].f32[i] = (ctx.v[0].u32[i] as f32) * (2.0f32).powi(0); }
	// 83201FF4: 395D0018  addi r10, r29, 0x18
	ctx.r[10].s64 = ctx.r[29].s64 + 24;
	// 83201FF8: C02708A8  lfs f1, 0x8a8(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83201FFC: 3AEB0010  addi r23, r11, 0x10
	ctx.r[23].s64 = ctx.r[11].s64 + 16;
	// 83202000: C0A6C3C8  lfs f5, -0x3c38(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-15416 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 83202004: 3ACB0020  addi r22, r11, 0x20
	ctx.r[22].s64 = ctx.r[11].s64 + 32;
	// 83202008: 3AAB0030  addi r21, r11, 0x30
	ctx.r[21].s64 = ctx.r[11].s64 + 48;
	// 8320200C: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 83202010: 3A7A0040  addi r19, r26, 0x40
	ctx.r[19].s64 = ctx.r[26].s64 + 64;
	// 83202014: 3A5A0001  addi r18, r26, 1
	ctx.r[18].s64 = ctx.r[26].s64 + 1;
	// 83202018: 387C0054  addi r3, r28, 0x54
	ctx.r[3].s64 = ctx.r[28].s64 + 84;
	// 8320201C: 393E0020  addi r9, r30, 0x20
	ctx.r[9].s64 = ctx.r[30].s64 + 32;
	// 83202020: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	// 83202024: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 83202028: 3B000010  li r24, 0x10
	ctx.r[24].s64 = 16;
	// 8320202C: 3B200020  li r25, 0x20
	ctx.r[25].s64 = 32;
	// 83202030: 3B600030  li r27, 0x30
	ctx.r[27].s64 = 48;
	// 83202034: 3A88FFA0  addi r20, r8, -0x60
	ctx.r[20].s64 = ctx.r[8].s64 + -96;
	// 83202038: 38E0FFE0  li r7, -0x20
	ctx.r[7].s64 = -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83202B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83202B70 size=40
    let mut pc: u32 = 0x83202B70;
    'dispatch: loop {
        match pc {
            0x83202B70 => {
    //   block [0x83202B70..0x83202B98)
	// 83202B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83202B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83202B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83202B7C: 480018CD  bl 0x83204448
	ctx.lr = 0x83202B80;
	sub_83204448(ctx, base);
	// 83202B80: 480018D1  bl 0x83204450
	ctx.lr = 0x83202B84;
	sub_83204450(ctx, base);
	// 83202B84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83202B88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83202B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83202B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83202B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83202B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83202B98 size=172
    let mut pc: u32 = 0x83202B98;
    'dispatch: loop {
        match pc {
            0x83202B98 => {
    //   block [0x83202B98..0x83202C44)
	// 83202B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83202B9C: 4BFA55C9  bl 0x831a8164
	ctx.lr = 0x83202BA0;
	sub_831A8130(ctx, base);
	// 83202BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83202BA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83202BA8: 409A0018  bne cr6, 0x83202bc0
	if !ctx.cr[6].eq {
	pc = 0x83202BC0; continue 'dispatch;
	}
	// 83202BAC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83202BB0: 386B34A4  addi r3, r11, 0x34a4
	ctx.r[3].s64 = ctx.r[11].s64 + 13476;
	// 83202BB4: 48002915  bl 0x832054c8
	ctx.lr = 0x83202BB8;
	sub_832054C8(ctx, base);
	// 83202BB8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83202BBC: 48000080  b 0x83202c3c
	pc = 0x83202C3C; continue 'dispatch;
	// 83202BC0: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 83202BC4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83202BC8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83202BCC: 4198006C  blt cr6, 0x83202c38
	if ctx.cr[6].lt {
	pc = 0x83202C38; continue 'dispatch;
	}
	// 83202BD0: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83202BD4: 41990064  bgt cr6, 0x83202c38
	if ctx.cr[6].gt {
	pc = 0x83202C38; continue 'dispatch;
	}
	// 83202BD8: 83C30008  lwz r30, 8(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83202BDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83202BE0: 480019E1  bl 0x832045c0
	ctx.lr = 0x83202BE4;
	sub_832045C0(ctx, base);
	// 83202BE4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83202BE8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83202BEC: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83202BF0: 40810040  ble 0x83202c30
	if !ctx.cr[0].gt {
	pc = 0x83202C30; continue 'dispatch;
	}
	// 83202BF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83202BF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83202BFC: 480019D5  bl 0x832045d0
	ctx.lr = 0x83202C00;
	sub_832045D0(ctx, base);
	// 83202C00: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83202C04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83202C08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83202C0C: 480019D5  bl 0x832045e0
	ctx.lr = 0x83202C10;
	sub_832045E0(ctx, base);
	// 83202C10: 7D63D9D6  mullw r11, r3, r27
	ctx.r[11].s64 = (ctx.r[3].s32 as i64) * (ctx.r[27].s32 as i64);
	// 83202C14: 1D6B0012  mulli r11, r11, 0x12
	ctx.r[11].s64 = ctx.r[11].s64 * 18;
	// 83202C18: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 83202C1C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83202C20: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83202C24: 7F1FE000  cmpw cr6, r31, r28
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83202C28: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83202C2C: 4198FFC8  blt cr6, 0x83202bf4
	if ctx.cr[6].lt {
	pc = 0x83202BF4; continue 'dispatch;
	}
	// 83202C30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83202C34: 48000008  b 0x83202c3c
	pc = 0x83202C3C; continue 'dispatch;
	// 83202C38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83202C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83202C40: 4BFA5574  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83202C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83202C48 size=356
    let mut pc: u32 = 0x83202C48;
    'dispatch: loop {
        match pc {
            0x83202C48 => {
    //   block [0x83202C48..0x83202DAC)
	// 83202C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83202C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83202C50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83202C54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83202C58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83202C5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83202C60: 817F01B0  lwz r11, 0x1b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(432 as u32) ) } as u64;
	// 83202C64: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83202C68: 419A0010  beq cr6, 0x83202c78
	if ctx.cr[6].eq {
	pc = 0x83202C78; continue 'dispatch;
	}
	// 83202C6C: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83202C70: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 83202C74: 409A0120  bne cr6, 0x83202d94
	if !ctx.cr[6].eq {
	pc = 0x83202D94; continue 'dispatch;
	}
	// 83202C78: 817F0274  lwz r11, 0x274(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(628 as u32) ) } as u64;
	// 83202C7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83202C80: 419A0044  beq cr6, 0x83202cc4
	if ctx.cr[6].eq {
	pc = 0x83202CC4; continue 'dispatch;
	}
	// 83202C84: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83202C88: 48001971  bl 0x832045f8
	ctx.lr = 0x83202C8C;
	sub_832045F8(ctx, base);
	// 83202C8C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83202C90: 809F01C0  lwz r4, 0x1c0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(448 as u32) ) } as u64;
	// 83202C94: 807F0278  lwz r3, 0x278(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 83202C98: 815F0274  lwz r10, 0x274(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(628 as u32) ) } as u64;
	// 83202C9C: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 83202CA0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83202CA4: 4E800421  bctrl
	ctx.lr = 0x83202CA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83202CA8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83202CAC: 907F01C0  stw r3, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[3].u32 ) };
	// 83202CB0: 40800020  bge 0x83202cd0
	if !ctx.cr[0].lt {
	pc = 0x83202CD0; continue 'dispatch;
	}
	// 83202CB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83202CB8: 997F01B4  stb r11, 0x1b4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(436 as u32), ctx.r[11].u8 ) };
	// 83202CBC: 997F01B5  stb r11, 0x1b5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(437 as u32), ctx.r[11].u8 ) };
	// 83202CC0: 48000010  b 0x83202cd0
	pc = 0x83202CD0; continue 'dispatch;
	// 83202CC4: 817F01C0  lwz r11, 0x1c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(448 as u32) ) } as u64;
	// 83202CC8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83202CCC: 917F01C0  stw r11, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[11].u32 ) };
	// 83202CD0: 897F01B4  lbz r11, 0x1b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(436 as u32) ) } as u64;
	// 83202CD4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83202CD8: 409A001C  bne cr6, 0x83202cf4
	if !ctx.cr[6].eq {
	pc = 0x83202CF4; continue 'dispatch;
	}
	// 83202CDC: 817F01C0  lwz r11, 0x1c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(448 as u32) ) } as u64;
	// 83202CE0: 815F01CC  lwz r10, 0x1cc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(460 as u32) ) } as u64;
	// 83202CE4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83202CE8: 4198000C  blt cr6, 0x83202cf4
	if ctx.cr[6].lt {
	pc = 0x83202CF4; continue 'dispatch;
	}
	// 83202CEC: 817F01C8  lwz r11, 0x1c8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(456 as u32) ) } as u64;
	// 83202CF0: 917F01C0  stw r11, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[11].u32 ) };
	// 83202CF4: 897F01B5  lbz r11, 0x1b5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(437 as u32) ) } as u64;
	// 83202CF8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83202CFC: 409A0098  bne cr6, 0x83202d94
	if !ctx.cr[6].eq {
	pc = 0x83202D94; continue 'dispatch;
	}
	// 83202D00: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83202D04: 480018F5  bl 0x832045f8
	ctx.lr = 0x83202D08;
	sub_832045F8(ctx, base);
	// 83202D08: 817F01C0  lwz r11, 0x1c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(448 as u32) ) } as u64;
	// 83202D0C: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83202D10: 40980084  bge cr6, 0x83202d94
	if !ctx.cr[6].lt {
	pc = 0x83202D94; continue 'dispatch;
	}
	// 83202D14: 4BEC4DCD  bl 0x830c7ae0
	ctx.lr = 0x83202D18;
	sub_830C7AE0(ctx, base);
	// 83202D18: 83DF01C0  lwz r30, 0x1c0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(448 as u32) ) } as u64;
	// 83202D1C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83202D20: 48001919  bl 0x83204638
	ctx.lr = 0x83202D24;
	sub_83204638(ctx, base);
	// 83202D24: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 83202D28: 4198001C  blt cr6, 0x83202d44
	if ctx.cr[6].lt {
	pc = 0x83202D44; continue 'dispatch;
	}
	// 83202D2C: 409A0064  bne cr6, 0x83202d90
	if !ctx.cr[6].eq {
	pc = 0x83202D90; continue 'dispatch;
	}
	// 83202D30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83202D34: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83202D38: 4BF2F991  bl 0x831326c8
	ctx.lr = 0x83202D3C;
	sub_831326C8(ctx, base);
	// 83202D3C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 83202D40: 48000040  b 0x83202d80
	pc = 0x83202D80; continue 'dispatch;
	// 83202D44: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83202D48: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83202D4C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83202D50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83202D54: 480018AD  bl 0x83204600
	ctx.lr = 0x83202D58;
	sub_83204600(ctx, base);
	// 83202D58: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83202D5C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83202D60: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 83202D64: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 83202D68: 4BF2F961  bl 0x831326c8
	ctx.lr = 0x83202D6C;
	sub_831326C8(ctx, base);
	// 83202D6C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83202D70: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83202D74: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83202D78: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 83202D7C: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 83202D80: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83202D84: 4BF2FA1D  bl 0x831327a0
	ctx.lr = 0x83202D88;
	sub_831327A0(ctx, base);
	// 83202D88: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83202D8C: 4BF30205  bl 0x83132f90
	ctx.lr = 0x83202D90;
	sub_83132F90(ctx, base);
	// 83202D90: 4BEC4D51  bl 0x830c7ae0
	ctx.lr = 0x83202D94;
	sub_830C7AE0(ctx, base);
	// 83202D94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83202D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83202D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83202DA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83202DA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83202DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83202DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83202DB0 size=224
    let mut pc: u32 = 0x83202DB0;
    'dispatch: loop {
        match pc {
            0x83202DB0 => {
    //   block [0x83202DB0..0x83202E90)
	// 83202DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83202DB4: 4BFA53B9  bl 0x831a816c
	ctx.lr = 0x83202DB8;
	sub_831A8130(ctx, base);
	// 83202DB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83202DBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83202DC0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83202DC4: 4BF3039D  bl 0x83133160
	ctx.lr = 0x83202DC8;
	sub_83133160(ctx, base);
	// 83202DC8: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83202DCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83202DD0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83202DD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83202DD8: 4E800421  bctrl
	ctx.lr = 0x83202DDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83202DDC: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83202DE0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83202DE4: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83202DE8: 40810034  ble 0x83202e1c
	if !ctx.cr[0].gt {
	pc = 0x83202E1C; continue 'dispatch;
	}
	// 83202DEC: 3BDF0098  addi r30, r31, 0x98
	ctx.r[30].s64 = ctx.r[31].s64 + 152;
	// 83202DF0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83202DF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83202DF8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83202DFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83202E00: 4E800421  bctrl
	ctx.lr = 0x83202E04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83202E04: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83202E08: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83202E0C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83202E10: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83202E14: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83202E18: 4198FFD8  blt cr6, 0x83202df0
	if ctx.cr[6].lt {
	pc = 0x83202DF0; continue 'dispatch;
	}
	// 83202E1C: 817F01E0  lwz r11, 0x1e0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(480 as u32) ) } as u64;
	// 83202E20: 80DF01DC  lwz r6, 0x1dc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(476 as u32) ) } as u64;
	// 83202E24: 79675FE4  rldicr r7, r11, 0xb, 0x3f
	ctx.r[7].u64 = (ctx.r[11].u64).rotate_left(11) & 0xFFFFFFFFFFFFFFFF;
	// 83202E28: 80BF01D8  lwz r5, 0x1d8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(472 as u32) ) } as u64;
	// 83202E2C: 809F01D4  lwz r4, 0x1d4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(468 as u32) ) } as u64;
	// 83202E30: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83202E34: 4BF300F5  bl 0x83132f28
	ctx.lr = 0x83202E38;
	sub_83132F28(ctx, base);
	// 83202E38: 817F011C  lwz r11, 0x11c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 83202E3C: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 83202E40: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83202E44: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83202E48: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 83202E4C: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 83202E50: 4BF30019  bl 0x83132e68
	ctx.lr = 0x83202E54;
	sub_83132E68(ctx, base);
	// 83202E54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83202E58: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83202E5C: 4BF2F86D  bl 0x831326c8
	ctx.lr = 0x83202E60;
	sub_831326C8(ctx, base);
	// 83202E60: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83202E64: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83202E68: 4BF2F939  bl 0x831327a0
	ctx.lr = 0x83202E6C;
	sub_831327A0(ctx, base);
	// 83202E6C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83202E70: 4BF30121  bl 0x83132f90
	ctx.lr = 0x83202E74;
	sub_83132F90(ctx, base);
	// 83202E74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83202E78: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83202E7C: 4800173D  bl 0x832045b8
	ctx.lr = 0x83202E80;
	sub_832045B8(ctx, base);
	// 83202E80: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83202E84: 480016DD  bl 0x83204560
	ctx.lr = 0x83202E88;
	sub_83204560(ctx, base);
	// 83202E88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83202E8C: 4BFA5330  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83202E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83202E90 size=260
    let mut pc: u32 = 0x83202E90;
    'dispatch: loop {
        match pc {
            0x83202E90 => {
    //   block [0x83202E90..0x83202F94)
	// 83202E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83202E94: 4BFA52CD  bl 0x831a8160
	ctx.lr = 0x83202E98;
	sub_831A8130(ctx, base);
	// 83202E98: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83202E9C: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 83202EA0: 3BC30010  addi r30, r3, 0x10
	ctx.r[30].s64 = ctx.r[3].s64 + 16;
	// 83202EA4: 7B5A0040  clrldi r26, r26, 1
	ctx.r[26].u64 = ctx.r[26].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 83202EA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83202EAC: 409A0014  bne cr6, 0x83202ec0
	if !ctx.cr[6].eq {
	pc = 0x83202EC0; continue 'dispatch;
	}
	// 83202EB0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83202EB4: 386B346C  addi r3, r11, 0x346c
	ctx.r[3].s64 = ctx.r[11].s64 + 13420;
	// 83202EB8: 48002611  bl 0x832054c8
	ctx.lr = 0x83202EBC;
	sub_832054C8(ctx, base);
	// 83202EBC: 480000D0  b 0x83202f8c
	pc = 0x83202F8C; continue 'dispatch;
	// 83202EC0: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 83202EC4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83202EC8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83202ECC: 4198001C  blt cr6, 0x83202ee8
	if ctx.cr[6].lt {
	pc = 0x83202EE8; continue 'dispatch;
	}
	// 83202ED0: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83202ED4: 41990014  bgt cr6, 0x83202ee8
	if ctx.cr[6].gt {
	pc = 0x83202EE8; continue 'dispatch;
	}
	// 83202ED8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83202EDC: 480016E5  bl 0x832045c0
	ctx.lr = 0x83202EE0;
	sub_832045C0(ctx, base);
	// 83202EE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83202EE4: 48000008  b 0x83202eec
	pc = 0x83202EEC; continue 'dispatch;
	// 83202EE8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83202EEC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83202EF0: 4099009C  ble cr6, 0x83202f8c
	if !ctx.cr[6].gt {
	pc = 0x83202F8C; continue 'dispatch;
	}
	// 83202EF4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83202EF8: 4BF2A8C1  bl 0x8312d7b8
	ctx.lr = 0x83202EFC;
	sub_8312D7B8(ctx, base);
	// 83202EFC: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83202F00: 4081008C  ble 0x83202f8c
	if !ctx.cr[0].gt {
	pc = 0x83202F8C; continue 'dispatch;
	}
	// 83202F04: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 83202F08: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 83202F0C: 40990028  ble cr6, 0x83202f34
	if !ctx.cr[6].gt {
	pc = 0x83202F34; continue 'dispatch;
	}
	// 83202F10: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 83202F14: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83202F18: 4BF2A8A1  bl 0x8312d7b8
	ctx.lr = 0x83202F1C;
	sub_8312D7B8(ctx, base);
	// 83202F1C: 7F1B1800  cmpw cr6, r27, r3
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83202F20: 409A006C  bne cr6, 0x83202f8c
	if !ctx.cr[6].eq {
	pc = 0x83202F8C; continue 'dispatch;
	}
	// 83202F24: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83202F28: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83202F2C: 7F1CF800  cmpw cr6, r28, r31
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[31].s32, &mut ctx.xer);
	// 83202F30: 4198FFE4  blt cr6, 0x83202f14
	if ctx.cr[6].lt {
	pc = 0x83202F14; continue 'dispatch;
	}
	// 83202F34: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83202F38: 40990054  ble cr6, 0x83202f8c
	if !ctx.cr[6].gt {
	pc = 0x83202F8C; continue 'dispatch;
	}
	// 83202F3C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 83202F40: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 83202F44: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83202F48: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83202F4C: 4BF2BDDD  bl 0x8312ed28
	ctx.lr = 0x83202F50;
	sub_8312ED28(ctx, base);
	// 83202F50: 7F3A1800  cmpd cr6, r26, r3
	ctx.cr[6].compare_i64(ctx.r[26].s64, ctx.r[3].s64, &mut ctx.xer);
	// 83202F54: 41980008  blt cr6, 0x83202f5c
	if ctx.cr[6].lt {
	pc = 0x83202F5C; continue 'dispatch;
	}
	// 83202F58: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83202F5C: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83202F60: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83202F64: 4082FFE0  bne 0x83202f44
	if !ctx.cr[0].eq {
	pc = 0x83202F44; continue 'dispatch;
	}
	// 83202F68: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83202F6C: 40990020  ble cr6, 0x83202f8c
	if !ctx.cr[6].gt {
	pc = 0x83202F8C; continue 'dispatch;
	}
	// 83202F70: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83202F74: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83202F78: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83202F7C: 4BF2BE05  bl 0x8312ed80
	ctx.lr = 0x83202F80;
	sub_8312ED80(ctx, base);
	// 83202F80: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83202F84: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83202F88: 4082FFE8  bne 0x83202f70
	if !ctx.cr[0].eq {
	pc = 0x83202F70; continue 'dispatch;
	}
	// 83202F8C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83202F90: 4BFA5220  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83202F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83202F98 size=200
    let mut pc: u32 = 0x83202F98;
    'dispatch: loop {
        match pc {
            0x83202F98 => {
    //   block [0x83202F98..0x83203060)
	// 83202F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83202F9C: 4BFA51CD  bl 0x831a8168
	ctx.lr = 0x83202FA0;
	sub_831A8130(ctx, base);
	// 83202FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83202FA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83202FA8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83202FAC: 409A0014  bne cr6, 0x83202fc0
	if !ctx.cr[6].eq {
	pc = 0x83202FC0; continue 'dispatch;
	}
	// 83202FB0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83202FB4: 386B3528  addi r3, r11, 0x3528
	ctx.r[3].s64 = ctx.r[11].s64 + 13608;
	// 83202FB8: 48002511  bl 0x832054c8
	ctx.lr = 0x83202FBC;
	sub_832054C8(ctx, base);
	// 83202FBC: 4800009C  b 0x83203058
	pc = 0x83203058; continue 'dispatch;
	// 83202FC0: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83202FC4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83202FC8: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 83202FCC: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83202FD0: 40810028  ble 0x83202ff8
	if !ctx.cr[0].gt {
	pc = 0x83202FF8; continue 'dispatch;
	}
	// 83202FD4: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 83202FD8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83202FDC: 4BF2A69D  bl 0x8312d678
	ctx.lr = 0x83202FE0;
	sub_8312D678(ctx, base);
	// 83202FE0: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83202FE4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83202FE8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83202FEC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83202FF0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83202FF4: 4198FFE4  blt cr6, 0x83202fd8
	if ctx.cr[6].lt {
	pc = 0x83202FD8; continue 'dispatch;
	}
	// 83202FF8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83202FFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203000: 41820008  beq 0x83203008
	if ctx.cr[0].eq {
	pc = 0x83203008; continue 'dispatch;
	}
	// 83203004: 4BF3000D  bl 0x83133010
	ctx.lr = 0x83203008;
	sub_83133010(ctx, base);
	// 83203008: 4BEC4AD9  bl 0x830c7ae0
	ctx.lr = 0x8320300C;
	sub_830C7AE0(ctx, base);
	// 8320300C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203010: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203014: 41820008  beq 0x8320301c
	if ctx.cr[0].eq {
	pc = 0x8320301C; continue 'dispatch;
	}
	// 83203018: 48001591  bl 0x832045a8
	ctx.lr = 0x8320301C;
	sub_832045A8(ctx, base);
	// 8320301C: 817F01E4  lwz r11, 0x1e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(484 as u32) ) } as u64;
	// 83203020: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83203024: 409A0024  bne cr6, 0x83203048
	if !ctx.cr[6].eq {
	pc = 0x83203048; continue 'dispatch;
	}
	// 83203028: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8320302C: 939F0094  stw r28, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[28].u32 ) };
	// 83203030: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203034: 41820014  beq 0x83203048
	if ctx.cr[0].eq {
	pc = 0x83203048; continue 'dispatch;
	}
	// 83203038: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320303C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83203040: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83203044: 4E800421  bctrl
	ctx.lr = 0x83203048;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83203048: 939F0094  stw r28, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[28].u32 ) };
	// 8320304C: 9B9F0001  stb r28, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[28].u8 ) };
	// 83203050: 939F01B0  stw r28, 0x1b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(432 as u32), ctx.r[28].u32 ) };
	// 83203054: 4BEC4A8D  bl 0x830c7ae0
	ctx.lr = 0x83203058;
	sub_830C7AE0(ctx, base);
	// 83203058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8320305C: 4BFA515C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203060 size=88
    let mut pc: u32 = 0x83203060;
    'dispatch: loop {
        match pc {
            0x83203060 => {
    //   block [0x83203060..0x832030B8)
	// 83203060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83203064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83203068: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8320306C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203070: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203074: 480013D5  bl 0x83204448
	ctx.lr = 0x83203078;
	sub_83204448(ctx, base);
	// 83203078: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8320307C: 409A0018  bne cr6, 0x83203094
	if !ctx.cr[6].eq {
	pc = 0x83203094; continue 'dispatch;
	}
	// 83203080: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83203084: 386B3398  addi r3, r11, 0x3398
	ctx.r[3].s64 = ctx.r[11].s64 + 13208;
	// 83203088: 48002441  bl 0x832054c8
	ctx.lr = 0x8320308C;
	sub_832054C8(ctx, base);
	// 8320308C: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 83203090: 4800000C  b 0x8320309c
	pc = 0x8320309C; continue 'dispatch;
	// 83203094: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83203098: 7D7F0774  extsb r31, r11
	ctx.r[31].s64 = ctx.r[11].s8 as i64;
	// 8320309C: 480013B5  bl 0x83204450
	ctx.lr = 0x832030A0;
	sub_83204450(ctx, base);
	// 832030A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832030A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832030A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832030AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832030B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832030B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832030B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832030B8 size=124
    let mut pc: u32 = 0x832030B8;
    'dispatch: loop {
        match pc {
            0x832030B8 => {
    //   block [0x832030B8..0x83203134)
	// 832030B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832030BC: 4BFA50B1  bl 0x831a816c
	ctx.lr = 0x832030C0;
	sub_831A8130(ctx, base);
	// 832030C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832030C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832030C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832030CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 832030D0: 48001379  bl 0x83204448
	ctx.lr = 0x832030D4;
	sub_83204448(ctx, base);
	// 832030D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832030D8: 409A0014  bne cr6, 0x832030ec
	if !ctx.cr[6].eq {
	pc = 0x832030EC; continue 'dispatch;
	}
	// 832030DC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 832030E0: 386B33C0  addi r3, r11, 0x33c0
	ctx.r[3].s64 = ctx.r[11].s64 + 13248;
	// 832030E4: 480023E5  bl 0x832054c8
	ctx.lr = 0x832030E8;
	sub_832054C8(ctx, base);
	// 832030E8: 48000040  b 0x83203128
	pc = 0x83203128; continue 'dispatch;
	// 832030EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832030F0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 832030F4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 832030F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 832030FC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83203100: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83203104: 4BF2A605  bl 0x8312d708
	ctx.lr = 0x83203108;
	sub_8312D708(ctx, base);
	// 83203108: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8320310C: 419A000C  beq cr6, 0x83203118
	if ctx.cr[6].eq {
	pc = 0x83203118; continue 'dispatch;
	}
	// 83203110: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83203114: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83203118: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8320311C: 419A000C  beq cr6, 0x83203128
	if ctx.cr[6].eq {
	pc = 0x83203128; continue 'dispatch;
	}
	// 83203120: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83203124: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83203128: 48001329  bl 0x83204450
	ctx.lr = 0x8320312C;
	sub_83204450(ctx, base);
	// 8320312C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83203130: 4BFA508C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203138 size=124
    let mut pc: u32 = 0x83203138;
    'dispatch: loop {
        match pc {
            0x83203138 => {
    //   block [0x83203138..0x832031B4)
	// 83203138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320313C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83203140: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83203144: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83203148: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320314C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203150: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83203154: 480012F5  bl 0x83204448
	ctx.lr = 0x83203158;
	sub_83204448(ctx, base);
	// 83203158: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8320315C: 409A0018  bne cr6, 0x83203174
	if !ctx.cr[6].eq {
	pc = 0x83203174; continue 'dispatch;
	}
	// 83203160: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83203164: 386B341C  addi r3, r11, 0x341c
	ctx.r[3].s64 = ctx.r[11].s64 + 13340;
	// 83203168: 48002361  bl 0x832054c8
	ctx.lr = 0x8320316C;
	sub_832054C8(ctx, base);
	// 8320316C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83203170: 48000024  b 0x83203194
	pc = 0x83203194; continue 'dispatch;
	// 83203174: 2F1E0020  cmpwi cr6, r30, 0x20
	ctx.cr[6].compare_i32(ctx.r[30].s32, 32, &mut ctx.xer);
	// 83203178: 40990010  ble cr6, 0x83203188
	if !ctx.cr[6].gt {
	pc = 0x83203188; continue 'dispatch;
	}
	// 8320317C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83203180: 386B33E8  addi r3, r11, 0x33e8
	ctx.r[3].s64 = ctx.r[11].s64 + 13288;
	// 83203184: 4BFFFFE4  b 0x83203168
	pc = 0x83203168; continue 'dispatch;
	// 83203188: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 8320318C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83203190: 7FEBF82E  lwzx r31, r11, r31
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83203194: 480012BD  bl 0x83204450
	ctx.lr = 0x83203198;
	sub_83204450(ctx, base);
	// 83203198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320319C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832031A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832031A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832031A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832031AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832031B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832031B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832031B8 size=88
    let mut pc: u32 = 0x832031B8;
    'dispatch: loop {
        match pc {
            0x832031B8 => {
    //   block [0x832031B8..0x83203210)
	// 832031B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832031BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832031C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832031C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832031C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832031CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832031D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832031D4: 48001275  bl 0x83204448
	ctx.lr = 0x832031D8;
	sub_83204448(ctx, base);
	// 832031D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832031DC: 409A0014  bne cr6, 0x832031f0
	if !ctx.cr[6].eq {
	pc = 0x832031F0; continue 'dispatch;
	}
	// 832031E0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 832031E4: 386B3444  addi r3, r11, 0x3444
	ctx.r[3].s64 = ctx.r[11].s64 + 13380;
	// 832031E8: 480022E1  bl 0x832054c8
	ctx.lr = 0x832031EC;
	sub_832054C8(ctx, base);
	// 832031EC: 48000008  b 0x832031f4
	pc = 0x832031F4; continue 'dispatch;
	// 832031F0: 9BDF01B5  stb r30, 0x1b5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(437 as u32), ctx.r[30].u8 ) };
	// 832031F4: 4800125D  bl 0x83204450
	ctx.lr = 0x832031F8;
	sub_83204450(ctx, base);
	// 832031F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832031FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83203200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83203204: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83203208: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8320320C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83203210 size=1544
    let mut pc: u32 = 0x83203210;
    'dispatch: loop {
        match pc {
            0x83203210 => {
    //   block [0x83203210..0x83203818)
	// 83203210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83203214: 4BFA4F45  bl 0x831a8158
	ctx.lr = 0x83203218;
	sub_831A8130(ctx, base);
	// 83203218: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320321C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203220: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83203224: 3B5F0010  addi r26, r31, 0x10
	ctx.r[26].s64 = ctx.r[31].s64 + 16;
	// 83203228: 837F0008  lwz r27, 8(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8320322C: 93210068  stw r25, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[25].u32 ) };
	// 83203230: 93210060  stw r25, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 83203234: 4BFFFC5D  bl 0x83202e90
	ctx.lr = 0x83203238;
	sub_83202E90(ctx, base);
	// 83203238: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8320323C: 3B000004  li r24, 4
	ctx.r[24].s64 = 4;
	// 83203240: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83203244: 409A04DC  bne cr6, 0x83203720
	if !ctx.cr[6].eq {
	pc = 0x83203720; continue 'dispatch;
	}
	// 83203248: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8320324C: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 83203250: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203254: 40810034  ble 0x83203288
	if !ctx.cr[0].gt {
	pc = 0x83203288; continue 'dispatch;
	}
	// 83203258: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 8320325C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203260: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203264: 4182000C  beq 0x83203270
	if ctx.cr[0].eq {
	pc = 0x83203270; continue 'dispatch;
	}
	// 83203268: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8320326C: 4BF2AA0D  bl 0x8312dc78
	ctx.lr = 0x83203270;
	sub_8312DC78(ctx, base);
	// 83203270: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83203274: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83203278: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8320327C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83203280: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83203284: 4198FFD8  blt cr6, 0x8320325c
	if ctx.cr[6].lt {
	pc = 0x8320325C; continue 'dispatch;
	}
	// 83203288: 817F01B0  lwz r11, 0x1b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(432 as u32) ) } as u64;
	// 8320328C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203290: 409A035C  bne cr6, 0x832035ec
	if !ctx.cr[6].eq {
	pc = 0x832035EC; continue 'dispatch;
	}
	// 83203294: 817F01E4  lwz r11, 0x1e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(484 as u32) ) } as u64;
	// 83203298: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8320329C: 418200E0  beq 0x8320337c
	if ctx.cr[0].eq {
	pc = 0x8320337C; continue 'dispatch;
	}
	// 832032A0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 832032A4: 419A00D8  beq cr6, 0x8320337c
	if ctx.cr[6].eq {
	pc = 0x8320337C; continue 'dispatch;
	}
	// 832032A8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832032AC: 480012A5  bl 0x83204550
	ctx.lr = 0x832032B0;
	sub_83204550(ctx, base);
	// 832032B0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 832032B4: 40820074  bne 0x83203328
	if !ctx.cr[0].eq {
	pc = 0x83203328; continue 'dispatch;
	}
	// 832032B8: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 832032BC: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 832032C0: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832032C4: 40810034  ble 0x832032f8
	if !ctx.cr[0].gt {
	pc = 0x832032F8; continue 'dispatch;
	}
	// 832032C8: 3BDF0098  addi r30, r31, 0x98
	ctx.r[30].s64 = ctx.r[31].s64 + 152;
	// 832032CC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832032D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832032D4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 832032D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832032DC: 4E800421  bctrl
	ctx.lr = 0x832032E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832032E0: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 832032E4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 832032E8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 832032EC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 832032F0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832032F4: 4198FFD8  blt cr6, 0x832032cc
	if ctx.cr[6].lt {
	pc = 0x832032CC; continue 'dispatch;
	}
	// 832032F8: 897F01B4  lbz r11, 0x1b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(436 as u32) ) } as u64;
	// 832032FC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83203300: 419A0010  beq cr6, 0x83203310
	if ctx.cr[6].eq {
	pc = 0x83203310; continue 'dispatch;
	}
	// 83203304: 897F01B5  lbz r11, 0x1b5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(437 as u32) ) } as u64;
	// 83203308: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8320330C: 409A0010  bne cr6, 0x8320331c
	if !ctx.cr[6].eq {
	pc = 0x8320331C; continue 'dispatch;
	}
	// 83203310: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83203314: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83203318: 4BC0EC09  bl 0x82e11f20
	ctx.lr = 0x8320331C;
	sub_82E11F20(ctx, base);
	// 8320331C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83203320: 48001241  bl 0x83204560
	ctx.lr = 0x83203324;
	sub_83204560(ctx, base);
	// 83203324: 480003FC  b 0x83203720
	pc = 0x83203720; continue 'dispatch;
	// 83203328: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8320332C: 409A02B4  bne cr6, 0x832035e0
	if !ctx.cr[6].eq {
	pc = 0x832035E0; continue 'dispatch;
	}
	// 83203330: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203334: 4800128D  bl 0x832045c0
	ctx.lr = 0x83203338;
	sub_832045C0(ctx, base);
	// 83203338: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8320333C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83203340: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83203344: 41990250  bgt cr6, 0x83203594
	if ctx.cr[6].gt {
	pc = 0x83203594; continue 'dispatch;
	}
	// 83203348: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8320334C: 40990024  ble cr6, 0x83203370
	if !ctx.cr[6].gt {
	pc = 0x83203370; continue 'dispatch;
	}
	// 83203350: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 83203354: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83203358: 809E0088  lwz r4, 0x88(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 8320335C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203360: 4BF2B1C1  bl 0x8312e520
	ctx.lr = 0x83203364;
	sub_8312E520(ctx, base);
	// 83203364: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83203368: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8320336C: 4082FFEC  bne 0x83203358
	if !ctx.cr[0].eq {
	pc = 0x83203358; continue 'dispatch;
	}
	// 83203370: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83203374: 917F01B0  stw r11, 0x1b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(432 as u32), ctx.r[11].u32 ) };
	// 83203378: 480003A8  b 0x83203720
	pc = 0x83203720; continue 'dispatch;
	// 8320337C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83203380: 4BF2F309  bl 0x83132688
	ctx.lr = 0x83203384;
	sub_83132688(ctx, base);
	// 83203384: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83203388: 4BEC4759  bl 0x830c7ae0
	ctx.lr = 0x8320338C;
	sub_830C7AE0(ctx, base);
	// 8320338C: 897F01B6  lbz r11, 0x1b6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(438 as u32) ) } as u64;
	// 83203390: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83203394: 409A0020  bne cr6, 0x832033b4
	if !ctx.cr[6].eq {
	pc = 0x832033B4; continue 'dispatch;
	}
	// 83203398: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 8320339C: 419A0018  beq cr6, 0x832033b4
	if ctx.cr[6].eq {
	pc = 0x832033B4; continue 'dispatch;
	}
	// 832033A0: 9B3F01B6  stb r25, 0x1b6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(438 as u32), ctx.r[25].u8 ) };
	// 832033A4: 4BEC473D  bl 0x830c7ae0
	ctx.lr = 0x832033A8;
	sub_830C7AE0(ctx, base);
	// 832033A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832033AC: 4BFFFA05  bl 0x83202db0
	ctx.lr = 0x832033B0;
	sub_83202DB0(ctx, base);
	// 832033B0: 48000008  b 0x832033b8
	pc = 0x832033B8; continue 'dispatch;
	// 832033B4: 4BEC472D  bl 0x830c7ae0
	ctx.lr = 0x832033B8;
	sub_830C7AE0(ctx, base);
	// 832033B8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832033BC: 48001195  bl 0x83204550
	ctx.lr = 0x832033C0;
	sub_83204550(ctx, base);
	// 832033C0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 832033C4: 409A021C  bne cr6, 0x832035e0
	if !ctx.cr[6].eq {
	pc = 0x832035E0; continue 'dispatch;
	}
	// 832033C8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832033CC: 480011F5  bl 0x832045c0
	ctx.lr = 0x832033D0;
	sub_832045C0(ctx, base);
	// 832033D0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832033D4: 48001265  bl 0x83204638
	ctx.lr = 0x832033D8;
	sub_83204638(ctx, base);
	// 832033D8: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 832033DC: 4198007C  blt cr6, 0x83203458
	if ctx.cr[6].lt {
	pc = 0x83203458; continue 'dispatch;
	}
	// 832033E0: 409AFF90  bne cr6, 0x83203370
	if !ctx.cr[6].eq {
	pc = 0x83203370; continue 'dispatch;
	}
	// 832033E4: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 832033E8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832033EC: 4BF2F3B5  bl 0x831327a0
	ctx.lr = 0x832033F0;
	sub_831327A0(ctx, base);
	// 832033F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832033F4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832033F8: 480011C1  bl 0x832045b8
	ctx.lr = 0x832033FC;
	sub_832045B8(ctx, base);
	// 832033FC: 897F01B4  lbz r11, 0x1b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(436 as u32) ) } as u64;
	// 83203400: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83203404: 419A0010  beq cr6, 0x83203414
	if ctx.cr[6].eq {
	pc = 0x83203414; continue 'dispatch;
	}
	// 83203408: 897F01B5  lbz r11, 0x1b5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(437 as u32) ) } as u64;
	// 8320340C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83203410: 409A0010  bne cr6, 0x83203420
	if !ctx.cr[6].eq {
	pc = 0x83203420; continue 'dispatch;
	}
	// 83203414: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83203418: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8320341C: 4BC0EB05  bl 0x82e11f20
	ctx.lr = 0x83203420;
	sub_82E11F20(ctx, base);
	// 83203420: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203424: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203428: 4182001C  beq 0x83203444
	if ctx.cr[0].eq {
	pc = 0x83203444; continue 'dispatch;
	}
	// 8320342C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83203430: 4BF2A9A9  bl 0x8312ddd8
	ctx.lr = 0x83203434;
	sub_8312DDD8(ctx, base);
	// 83203434: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83203438: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320343C: 4BF2B0E5  bl 0x8312e520
	ctx.lr = 0x83203440;
	sub_8312E520(ctx, base);
	// 83203440: 4BFFFF30  b 0x83203370
	pc = 0x83203370; continue 'dispatch;
	// 83203444: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83203448: 386B3578  addi r3, r11, 0x3578
	ctx.r[3].s64 = ctx.r[11].s64 + 13688;
	// 8320344C: 4800207D  bl 0x832054c8
	ctx.lr = 0x83203450;
	sub_832054C8(ctx, base);
	// 83203450: 9B1F0001  stb r24, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[24].u8 ) };
	// 83203454: 4BFFFF1C  b 0x83203370
	pc = 0x83203370; continue 'dispatch;
	// 83203458: 83DF01C4  lwz r30, 0x1c4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(452 as u32) ) } as u64;
	// 8320345C: 2C1E0000  cmpwi r30, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83203460: 93DF01C0  stw r30, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[30].u32 ) };
	// 83203464: 41800170  blt 0x832035d4
	if ctx.cr[0].lt {
	pc = 0x832035D4; continue 'dispatch;
	}
	// 83203468: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8320346C: 4800118D  bl 0x832045f8
	ctx.lr = 0x83203470;
	sub_832045F8(ctx, base);
	// 83203470: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83203474: 40980160  bge cr6, 0x832035d4
	if !ctx.cr[6].lt {
	pc = 0x832035D4; continue 'dispatch;
	}
	// 83203478: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8320347C: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 83203480: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83203484: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83203488: 48001179  bl 0x83204600
	ctx.lr = 0x8320348C;
	sub_83204600(ctx, base);
	// 8320348C: 817F01BC  lwz r11, 0x1bc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(444 as u32) ) } as u64;
	// 83203490: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 83203494: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83203498: 40990050  ble cr6, 0x832034e8
	if !ctx.cr[6].gt {
	pc = 0x832034E8; continue 'dispatch;
	}
	// 8320349C: 3BBE0001  addi r29, r30, 1
	ctx.r[29].s64 = ctx.r[30].s64 + 1;
	// 832034A0: 817F01CC  lwz r11, 0x1cc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(460 as u32) ) } as u64;
	// 832034A4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832034A8: 41990030  bgt cr6, 0x832034d8
	if ctx.cr[6].gt {
	pc = 0x832034D8; continue 'dispatch;
	}
	// 832034AC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 832034B0: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 832034B4: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 832034B8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832034BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832034C0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 832034C4: 4800113D  bl 0x83204600
	ctx.lr = 0x832034C8;
	sub_83204600(ctx, base);
	// 832034C8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832034CC: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832034D0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 832034D4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 832034D8: 817F01BC  lwz r11, 0x1bc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(444 as u32) ) } as u64;
	// 832034DC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 832034E0: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832034E4: 4198FFBC  blt cr6, 0x832034a0
	if ctx.cr[6].lt {
	pc = 0x832034A0; continue 'dispatch;
	}
	// 832034E8: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 832034EC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832034F0: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 832034F4: 93DF01C0  stw r30, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[30].u32 ) };
	// 832034F8: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 832034FC: 4BF2F1CD  bl 0x831326c8
	ctx.lr = 0x83203500;
	sub_831326C8(ctx, base);
	// 83203500: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83203504: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83203508: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8320350C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83203510: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 83203514: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 83203518: 4BF2F289  bl 0x831327a0
	ctx.lr = 0x8320351C;
	sub_831327A0(ctx, base);
	// 8320351C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83203520: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83203524: 48001095  bl 0x832045b8
	ctx.lr = 0x83203528;
	sub_832045B8(ctx, base);
	// 83203528: 897F01B4  lbz r11, 0x1b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(436 as u32) ) } as u64;
	// 8320352C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83203530: 419A0010  beq cr6, 0x83203540
	if ctx.cr[6].eq {
	pc = 0x83203540; continue 'dispatch;
	}
	// 83203534: 897F01B5  lbz r11, 0x1b5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(437 as u32) ) } as u64;
	// 83203538: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8320353C: 409A0010  bne cr6, 0x8320354c
	if !ctx.cr[6].eq {
	pc = 0x8320354C; continue 'dispatch;
	}
	// 83203540: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83203544: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83203548: 4BC0E9D9  bl 0x82e11f20
	ctx.lr = 0x8320354C;
	sub_82E11F20(ctx, base);
	// 8320354C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203550: 48001071  bl 0x832045c0
	ctx.lr = 0x83203554;
	sub_832045C0(ctx, base);
	// 83203554: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83203558: 40810024  ble 0x8320357c
	if !ctx.cr[0].gt {
	pc = 0x8320357C; continue 'dispatch;
	}
	// 8320355C: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 83203560: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83203564: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83203568: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320356C: 4BF2A86D  bl 0x8312ddd8
	ctx.lr = 0x83203570;
	sub_8312DDD8(ctx, base);
	// 83203570: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83203574: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83203578: 4082FFEC  bne 0x83203564
	if !ctx.cr[0].eq {
	pc = 0x83203564; continue 'dispatch;
	}
	// 8320357C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203580: 48001041  bl 0x832045c0
	ctx.lr = 0x83203584;
	sub_832045C0(ctx, base);
	// 83203584: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83203588: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8320358C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83203590: 40990018  ble cr6, 0x832035a8
	if !ctx.cr[6].gt {
	pc = 0x832035A8; continue 'dispatch;
	}
	// 83203594: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83203598: 386B34E0  addi r3, r11, 0x34e0
	ctx.r[3].s64 = ctx.r[11].s64 + 13536;
	// 8320359C: 48001F2D  bl 0x832054c8
	ctx.lr = 0x832035A0;
	sub_832054C8(ctx, base);
	// 832035A0: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 832035A4: 48000034  b 0x832035d8
	pc = 0x832035D8; continue 'dispatch;
	// 832035A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832035AC: 4099FDC4  ble cr6, 0x83203370
	if !ctx.cr[6].gt {
	pc = 0x83203370; continue 'dispatch;
	}
	// 832035B0: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 832035B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 832035B8: 809E0088  lwz r4, 0x88(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 832035BC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832035C0: 4BF2AF61  bl 0x8312e520
	ctx.lr = 0x832035C4;
	sub_8312E520(ctx, base);
	// 832035C4: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 832035C8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 832035CC: 4082FFEC  bne 0x832035b8
	if !ctx.cr[0].eq {
	pc = 0x832035B8; continue 'dispatch;
	}
	// 832035D0: 4BFFFDA0  b 0x83203370
	pc = 0x83203370; continue 'dispatch;
	// 832035D4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832035D8: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 832035DC: 48000234  b 0x83203810
	pc = 0x83203810; continue 'dispatch;
	// 832035E0: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 832035E4: 409A013C  bne cr6, 0x83203720
	if !ctx.cr[6].eq {
	pc = 0x83203720; continue 'dispatch;
	}
	// 832035E8: 4BFFFFB8  b 0x832035a0
	pc = 0x832035A0; continue 'dispatch;
	// 832035EC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832035F0: 48001049  bl 0x83204638
	ctx.lr = 0x832035F4;
	sub_83204638(ctx, base);
	// 832035F4: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 832035F8: 41980014  blt cr6, 0x8320360c
	if ctx.cr[6].lt {
	pc = 0x8320360C; continue 'dispatch;
	}
	// 832035FC: 409A0124  bne cr6, 0x83203720
	if !ctx.cr[6].eq {
	pc = 0x83203720; continue 'dispatch;
	}
	// 83203600: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83203604: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83203608: 48000118  b 0x83203720
	pc = 0x83203720; continue 'dispatch;
	// 8320360C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203610: 48000FB1  bl 0x832045c0
	ctx.lr = 0x83203614;
	sub_832045C0(ctx, base);
	// 83203614: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 83203618: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8320361C: 40810034  ble 0x83203650
	if !ctx.cr[0].gt {
	pc = 0x83203650; continue 'dispatch;
	}
	// 83203620: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 83203624: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203628: 4BF2A089  bl 0x8312d6b0
	ctx.lr = 0x8320362C;
	sub_8312D6B0(ctx, base);
	// 8320362C: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 83203630: 409A0008  bne cr6, 0x83203638
	if !ctx.cr[6].eq {
	pc = 0x83203638; continue 'dispatch;
	}
	// 83203634: 9B1F0001  stb r24, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[24].u8 ) };
	// 83203638: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8320363C: 409A0014  bne cr6, 0x83203650
	if !ctx.cr[6].eq {
	pc = 0x83203650; continue 'dispatch;
	}
	// 83203640: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83203644: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83203648: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8320364C: 4198FFD8  blt cr6, 0x83203624
	if ctx.cr[6].lt {
	pc = 0x83203624; continue 'dispatch;
	}
	// 83203650: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83203654: 409A00CC  bne cr6, 0x83203720
	if !ctx.cr[6].eq {
	pc = 0x83203720; continue 'dispatch;
	}
	// 83203658: 4BEC4489  bl 0x830c7ae0
	ctx.lr = 0x8320365C;
	sub_830C7AE0(ctx, base);
	// 8320365C: 817F01AC  lwz r11, 0x1ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(428 as u32) ) } as u64;
	// 83203660: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203664: 409A0044  bne cr6, 0x832036a8
	if !ctx.cr[6].eq {
	pc = 0x832036A8; continue 'dispatch;
	}
	// 83203668: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8320366C: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 83203670: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203674: 40810034  ble 0x832036a8
	if !ctx.cr[0].gt {
	pc = 0x832036A8; continue 'dispatch;
	}
	// 83203678: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 8320367C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203680: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203684: 4182000C  beq 0x83203690
	if ctx.cr[0].eq {
	pc = 0x83203690; continue 'dispatch;
	}
	// 83203688: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8320368C: 4BF2A5ED  bl 0x8312dc78
	ctx.lr = 0x83203690;
	sub_8312DC78(ctx, base);
	// 83203690: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83203694: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83203698: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8320369C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 832036A0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832036A4: 4198FFD8  blt cr6, 0x8320367c
	if ctx.cr[6].lt {
	pc = 0x8320367C; continue 'dispatch;
	}
	// 832036A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832036AC: C1BF01E8  lfs f13, 0x1e8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832036B0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 832036B4: 933F01B0  stw r25, 0x1b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(432 as u32), ctx.r[25].u32 ) };
	// 832036B8: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832036BC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 832036C0: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 832036C4: 419A0044  beq cr6, 0x83203708
	if ctx.cr[6].eq {
	pc = 0x83203708; continue 'dispatch;
	}
	// 832036C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832036CC: 4BFFF4CD  bl 0x83202b98
	ctx.lr = 0x832036D0;
	sub_83202B98(ctx, base);
	// 832036D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832036D4: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 832036D8: C01F01E8  lfs f0, 0x1e8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832036DC: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 832036E0: 80BF011C  lwz r5, 0x11c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 832036E4: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 832036E8: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 832036EC: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 832036F0: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 832036F4: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 832036F8: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 832036FC: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 83203700: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83203704: 48000010  b 0x83203714
	pc = 0x83203714; continue 'dispatch;
	// 83203708: 80BF011C  lwz r5, 0x11c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 8320370C: 7CAB0E70  srawi r11, r5, 1
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[5].s32 >> 1) as i64;
	// 83203710: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 83203714: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83203718: 4BF2F751  bl 0x83132e68
	ctx.lr = 0x8320371C;
	sub_83132E68(ctx, base);
	// 8320371C: 4BEC43C5  bl 0x830c7ae0
	ctx.lr = 0x83203720;
	sub_830C7AE0(ctx, base);
	// 83203720: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83203724: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 83203728: 409A00C4  bne cr6, 0x832037ec
	if !ctx.cr[6].eq {
	pc = 0x832037EC; continue 'dispatch;
	}
	// 8320372C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83203730: 48000F09  bl 0x83204638
	ctx.lr = 0x83203734;
	sub_83204638(ctx, base);
	// 83203734: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 83203738: 41980038  blt cr6, 0x83203770
	if ctx.cr[6].lt {
	pc = 0x83203770; continue 'dispatch;
	}
	// 8320373C: 419A000C  beq cr6, 0x83203748
	if ctx.cr[6].eq {
	pc = 0x83203748; continue 'dispatch;
	}
	// 83203740: 9B1F0001  stb r24, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[24].u8 ) };
	// 83203744: 480000A8  b 0x832037ec
	pc = 0x832037EC; continue 'dispatch;
	// 83203748: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8320374C: 4BF2EF3D  bl 0x83132688
	ctx.lr = 0x83203750;
	sub_83132688(ctx, base);
	// 83203750: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83203754: 419A0098  beq cr6, 0x832037ec
	if ctx.cr[6].eq {
	pc = 0x832037EC; continue 'dispatch;
	}
	// 83203758: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320375C: 4BF29F55  bl 0x8312d6b0
	ctx.lr = 0x83203760;
	sub_8312D6B0(ctx, base);
	// 83203760: 2F030005  cmpwi cr6, r3, 5
	ctx.cr[6].compare_i32(ctx.r[3].s32, 5, &mut ctx.xer);
	// 83203764: 419A0080  beq cr6, 0x832037e4
	if ctx.cr[6].eq {
	pc = 0x832037E4; continue 'dispatch;
	}
	// 83203768: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8320376C: 48000074  b 0x832037e0
	pc = 0x832037E0; continue 'dispatch;
	// 83203770: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83203774: 48000DDD  bl 0x83204550
	ctx.lr = 0x83203778;
	sub_83204550(ctx, base);
	// 83203778: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8320377C: 409A0070  bne cr6, 0x832037ec
	if !ctx.cr[6].eq {
	pc = 0x832037EC; continue 'dispatch;
	}
	// 83203780: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203784: 48000DCD  bl 0x83204550
	ctx.lr = 0x83203788;
	sub_83204550(ctx, base);
	// 83203788: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8320378C: 409A0060  bne cr6, 0x832037ec
	if !ctx.cr[6].eq {
	pc = 0x832037EC; continue 'dispatch;
	}
	// 83203790: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83203794: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 83203798: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8320379C: 40810038  ble 0x832037d4
	if !ctx.cr[0].gt {
	pc = 0x832037D4; continue 'dispatch;
	}
	// 832037A0: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 832037A4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 832037A8: 4BF29F09  bl 0x8312d6b0
	ctx.lr = 0x832037AC;
	sub_8312D6B0(ctx, base);
	// 832037AC: 2F030005  cmpwi cr6, r3, 5
	ctx.cr[6].compare_i32(ctx.r[3].s32, 5, &mut ctx.xer);
	// 832037B0: 419A000C  beq cr6, 0x832037bc
	if ctx.cr[6].eq {
	pc = 0x832037BC; continue 'dispatch;
	}
	// 832037B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832037B8: 409A001C  bne cr6, 0x832037d4
	if !ctx.cr[6].eq {
	pc = 0x832037D4; continue 'dispatch;
	}
	// 832037BC: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 832037C0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 832037C4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 832037C8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 832037CC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832037D0: 4198FFD4  blt cr6, 0x832037a4
	if ctx.cr[6].lt {
	pc = 0x832037A4; continue 'dispatch;
	}
	// 832037D4: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 832037D8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 832037DC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832037E0: 409A000C  bne cr6, 0x832037ec
	if !ctx.cr[6].eq {
	pc = 0x832037EC; continue 'dispatch;
	}
	// 832037E4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832037E8: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 832037EC: 4BEC42F5  bl 0x830c7ae0
	ctx.lr = 0x832037F0;
	sub_830C7AE0(ctx, base);
	// 832037F0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832037F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 832037F8: 41820014  beq 0x8320380c
	if ctx.cr[0].eq {
	pc = 0x8320380C; continue 'dispatch;
	}
	// 832037FC: 4BF2EE8D  bl 0x83132688
	ctx.lr = 0x83203800;
	sub_83132688(ctx, base);
	// 83203800: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 83203804: 409A0008  bne cr6, 0x8320380c
	if !ctx.cr[6].eq {
	pc = 0x8320380C; continue 'dispatch;
	}
	// 83203808: 9B1F0001  stb r24, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[24].u8 ) };
	// 8320380C: 4BEC42D5  bl 0x830c7ae0
	ctx.lr = 0x83203810;
	sub_830C7AE0(ctx, base);
	// 83203810: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83203814: 4BFA4994  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203818 size=332
    let mut pc: u32 = 0x83203818;
    'dispatch: loop {
        match pc {
            0x83203818 => {
    //   block [0x83203818..0x83203964)
	// 83203818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320381C: 4BFA494D  bl 0x831a8168
	ctx.lr = 0x83203820;
	sub_831A8130(ctx, base);
	// 83203820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203824: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203828: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8320382C: 409A0014  bne cr6, 0x83203840
	if !ctx.cr[6].eq {
	pc = 0x83203840; continue 'dispatch;
	}
	// 83203830: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83203834: 386B3618  addi r3, r11, 0x3618
	ctx.r[3].s64 = ctx.r[11].s64 + 13848;
	// 83203838: 48001C91  bl 0x832054c8
	ctx.lr = 0x8320383C;
	sub_832054C8(ctx, base);
	// 8320383C: 48000120  b 0x8320395c
	pc = 0x8320395C; continue 'dispatch;
	// 83203840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203844: 4BFFF755  bl 0x83202f98
	ctx.lr = 0x83203848;
	sub_83202F98(ctx, base);
	// 83203848: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8320384C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83203850: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203854: 41820020  beq 0x83203874
	if ctx.cr[0].eq {
	pc = 0x83203874; continue 'dispatch;
	}
	// 83203858: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8320385C: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 83203860: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83203864: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83203868: 4BF2EF09  bl 0x83132770
	ctx.lr = 0x8320386C;
	sub_83132770(ctx, base);
	// 8320386C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83203870: 4BF2F9C1  bl 0x83133230
	ctx.lr = 0x83203874;
	sub_83133230(ctx, base);
	// 83203874: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83203878: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 8320387C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83203880: 419A0040  beq cr6, 0x832038c0
	if ctx.cr[6].eq {
	pc = 0x832038C0; continue 'dispatch;
	}
	// 83203884: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83203888: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 8320388C: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203890: 40810030  ble 0x832038c0
	if !ctx.cr[0].gt {
	pc = 0x832038C0; continue 'dispatch;
	}
	// 83203894: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203898: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8320389C: 4182000C  beq 0x832038a8
	if ctx.cr[0].eq {
	pc = 0x832038A8; continue 'dispatch;
	}
	// 832038A0: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832038A4: 4BF2A8D5  bl 0x8312e178
	ctx.lr = 0x832038A8;
	sub_8312E178(ctx, base);
	// 832038A8: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 832038AC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 832038B0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 832038B4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 832038B8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832038BC: 4198FFD8  blt cr6, 0x83203894
	if ctx.cr[6].lt {
	pc = 0x83203894; continue 'dispatch;
	}
	// 832038C0: 4BEC4221  bl 0x830c7ae0
	ctx.lr = 0x832038C4;
	sub_830C7AE0(ctx, base);
	// 832038C4: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 832038C8: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 832038CC: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832038D0: 40810040  ble 0x83203910
	if !ctx.cr[0].gt {
	pc = 0x83203910; continue 'dispatch;
	}
	// 832038D4: 3BDF0098  addi r30, r31, 0x98
	ctx.r[30].s64 = ctx.r[31].s64 + 152;
	// 832038D8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832038DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 832038E0: 41820018  beq 0x832038f8
	if ctx.cr[0].eq {
	pc = 0x832038F8; continue 'dispatch;
	}
	// 832038E4: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832038E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832038EC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 832038F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832038F4: 4E800421  bctrl
	ctx.lr = 0x832038F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832038F8: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 832038FC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83203900: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83203904: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83203908: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8320390C: 4198FFCC  blt cr6, 0x832038d8
	if ctx.cr[6].lt {
	pc = 0x832038D8; continue 'dispatch;
	}
	// 83203910: 807F0090  lwz r3, 0x90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 83203914: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203918: 41820018  beq 0x83203930
	if ctx.cr[0].eq {
	pc = 0x83203930; continue 'dispatch;
	}
	// 8320391C: 939F0090  stw r28, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[28].u32 ) };
	// 83203920: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203924: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83203928: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8320392C: 4E800421  bctrl
	ctx.lr = 0x83203930;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83203930: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203934: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203938: 4182000C  beq 0x83203944
	if ctx.cr[0].eq {
	pc = 0x83203944; continue 'dispatch;
	}
	// 8320393C: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 83203940: 48000BC9  bl 0x83204508
	ctx.lr = 0x83203944;
	sub_83204508(ctx, base);
	// 83203944: 38A002A0  li r5, 0x2a0
	ctx.r[5].s64 = 672;
	// 83203948: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8320394C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203950: 4BFA4891  bl 0x831a81e0
	ctx.lr = 0x83203954;
	sub_831A81E0(ctx, base);
	// 83203954: 9B9F0000  stb r28, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 83203958: 4BEC4189  bl 0x830c7ae0
	ctx.lr = 0x8320395C;
	sub_830C7AE0(ctx, base);
	// 8320395C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83203960: 4BFA4858  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203968 size=56
    let mut pc: u32 = 0x83203968;
    'dispatch: loop {
        match pc {
            0x83203968 => {
    //   block [0x83203968..0x832039A0)
	// 83203968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320396C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83203970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83203974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203978: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8320397C: 48000ACD  bl 0x83204448
	ctx.lr = 0x83203980;
	sub_83204448(ctx, base);
	// 83203980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203984: 4BFFF615  bl 0x83202f98
	ctx.lr = 0x83203988;
	sub_83202F98(ctx, base);
	// 83203988: 48000AC9  bl 0x83204450
	ctx.lr = 0x8320398C;
	sub_83204450(ctx, base);
	// 8320398C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83203990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83203994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83203998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8320399C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832039A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832039A0 size=148
    let mut pc: u32 = 0x832039A0;
    'dispatch: loop {
        match pc {
            0x832039A0 => {
    //   block [0x832039A0..0x83203A34)
	// 832039A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832039A4: 4BFA47C5  bl 0x831a8168
	ctx.lr = 0x832039A8;
	sub_831A8130(ctx, base);
	// 832039A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832039AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832039B0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 832039B4: 48000A95  bl 0x83204448
	ctx.lr = 0x832039B8;
	sub_83204448(ctx, base);
	// 832039B8: 4BEC4129  bl 0x830c7ae0
	ctx.lr = 0x832039BC;
	sub_830C7AE0(ctx, base);
	// 832039BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832039C0: 409A0014  bne cr6, 0x832039d4
	if !ctx.cr[6].eq {
	pc = 0x832039D4; continue 'dispatch;
	}
	// 832039C4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 832039C8: 386B3550  addi r3, r11, 0x3550
	ctx.r[3].s64 = ctx.r[11].s64 + 13648;
	// 832039CC: 48001AFD  bl 0x832054c8
	ctx.lr = 0x832039D0;
	sub_832054C8(ctx, base);
	// 832039D0: 48000054  b 0x83203a24
	pc = 0x83203A24; continue 'dispatch;
	// 832039D4: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 832039D8: 939F01AC  stw r28, 0x1ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(428 as u32), ctx.r[28].u32 ) };
	// 832039DC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 832039E0: 419A0044  beq cr6, 0x83203a24
	if ctx.cr[6].eq {
	pc = 0x83203A24; continue 'dispatch;
	}
	// 832039E4: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 832039E8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 832039EC: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832039F0: 40810034  ble 0x83203a24
	if !ctx.cr[0].gt {
	pc = 0x83203A24; continue 'dispatch;
	}
	// 832039F4: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 832039F8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832039FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203A00: 4182000C  beq 0x83203a0c
	if ctx.cr[0].eq {
	pc = 0x83203A0C; continue 'dispatch;
	}
	// 83203A04: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83203A08: 4BF2A271  bl 0x8312dc78
	ctx.lr = 0x83203A0C;
	sub_8312DC78(ctx, base);
	// 83203A0C: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83203A10: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83203A14: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83203A18: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83203A1C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83203A20: 4198FFD8  blt cr6, 0x832039f8
	if ctx.cr[6].lt {
	pc = 0x832039F8; continue 'dispatch;
	}
	// 83203A24: 4BEC40BD  bl 0x830c7ae0
	ctx.lr = 0x83203A28;
	sub_830C7AE0(ctx, base);
	// 83203A28: 48000A29  bl 0x83204450
	ctx.lr = 0x83203A2C;
	sub_83204450(ctx, base);
	// 83203A2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83203A30: 4BFA4788  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203A38 size=128
    let mut pc: u32 = 0x83203A38;
    'dispatch: loop {
        match pc {
            0x83203A38 => {
    //   block [0x83203A38..0x83203AB8)
	// 83203A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83203A3C: 4BFA4731  bl 0x831a816c
	ctx.lr = 0x83203A40;
	sub_831A8130(ctx, base);
	// 83203A40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203A44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203A48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83203A4C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83203A50: 480009F9  bl 0x83204448
	ctx.lr = 0x83203A54;
	sub_83204448(ctx, base);
	// 83203A54: 4BEC408D  bl 0x830c7ae0
	ctx.lr = 0x83203A58;
	sub_830C7AE0(ctx, base);
	// 83203A58: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83203A5C: 409A0014  bne cr6, 0x83203a70
	if !ctx.cr[6].eq {
	pc = 0x83203A70; continue 'dispatch;
	}
	// 83203A60: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83203A64: 386B35F0  addi r3, r11, 0x35f0
	ctx.r[3].s64 = ctx.r[11].s64 + 13808;
	// 83203A68: 48001A61  bl 0x832054c8
	ctx.lr = 0x83203A6C;
	sub_832054C8(ctx, base);
	// 83203A6C: 4800003C  b 0x83203aa8
	pc = 0x83203AA8; continue 'dispatch;
	// 83203A70: 2F1E0020  cmpwi cr6, r30, 0x20
	ctx.cr[6].compare_i32(ctx.r[30].s32, 32, &mut ctx.xer);
	// 83203A74: 40990010  ble cr6, 0x83203a84
	if !ctx.cr[6].gt {
	pc = 0x83203A84; continue 'dispatch;
	}
	// 83203A78: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83203A7C: 386B35BC  addi r3, r11, 0x35bc
	ctx.r[3].s64 = ctx.r[11].s64 + 13756;
	// 83203A80: 4BFFFFE8  b 0x83203a68
	pc = 0x83203A68; continue 'dispatch;
	// 83203A84: 397E007C  addi r11, r30, 0x7c
	ctx.r[11].s64 = ctx.r[30].s64 + 124;
	// 83203A88: 395E0004  addi r10, r30, 4
	ctx.r[10].s64 = ctx.r[30].s64 + 4;
	// 83203A8C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83203A90: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83203A94: 7FABF92E  stwx r29, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 83203A98: 817F01EC  lwz r11, 0x1ec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(492 as u32) ) } as u64;
	// 83203A9C: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83203AA0: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83203AA4: 4BF29EA5  bl 0x8312d948
	ctx.lr = 0x83203AA8;
	sub_8312D948(ctx, base);
	// 83203AA8: 4BEC4039  bl 0x830c7ae0
	ctx.lr = 0x83203AAC;
	sub_830C7AE0(ctx, base);
	// 83203AAC: 480009A5  bl 0x83204450
	ctx.lr = 0x83203AB0;
	sub_83204450(ctx, base);
	// 83203AB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83203AB4: 4BFA4708  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203AB8 size=196
    let mut pc: u32 = 0x83203AB8;
    'dispatch: loop {
        match pc {
            0x83203AB8 => {
    //   block [0x83203AB8..0x83203B7C)
	// 83203AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83203ABC: 4BFA46B1  bl 0x831a816c
	ctx.lr = 0x83203AC0;
	sub_831A8130(ctx, base);
	// 83203AC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203AC4: 3D408343  lis r10, -0x7cbd
	ctx.r[10].s64 = -2092761088;
	// 83203AC8: 816AD8D8  lwz r11, -0x2728(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-10024 as u32) ) } as u64;
	// 83203ACC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203AD0: 916AD8D8  stw r11, -0x2728(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10024 as u32), ctx.r[11].u32 ) };
	// 83203AD4: 408200A0  bne 0x83203b74
	if !ctx.cr[0].eq {
	pc = 0x83203B74; continue 'dispatch;
	}
	// 83203AD8: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 83203ADC: 3BCBE720  addi r30, r11, -0x18e0
	ctx.r[30].s64 = ctx.r[11].s64 + -6368;
	// 83203AE0: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 83203AE4: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203AE8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83203AEC: 409A000C  bne cr6, 0x83203af8
	if !ctx.cr[6].eq {
	pc = 0x83203AF8; continue 'dispatch;
	}
	// 83203AF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203AF4: 4BFFFD25  bl 0x83203818
	ctx.lr = 0x83203AF8;
	sub_83203818(ctx, base);
	// 83203AF8: 3BFF02A0  addi r31, r31, 0x2a0
	ctx.r[31].s64 = ctx.r[31].s64 + 672;
	// 83203AFC: 397E0A80  addi r11, r30, 0xa80
	ctx.r[11].s64 = ctx.r[30].s64 + 2688;
	// 83203B00: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83203B04: 4198FFE0  blt cr6, 0x83203ae4
	if ctx.cr[6].lt {
	pc = 0x83203AE4; continue 'dispatch;
	}
	// 83203B08: 38A00A80  li r5, 0xa80
	ctx.r[5].s64 = 2688;
	// 83203B0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83203B10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83203B14: 4BFA46CD  bl 0x831a81e0
	ctx.lr = 0x83203B18;
	sub_831A81E0(ctx, base);
	// 83203B18: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83203B1C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83203B20: 4BF22F59  bl 0x83126a78
	ctx.lr = 0x83203B24;
	sub_83126A78(ctx, base);
	// 83203B24: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 83203B28: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 83203B2C: 808BD8DC  lwz r4, -0x2724(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10020 as u32) ) } as u64;
	// 83203B30: 4BF22F49  bl 0x83126a78
	ctx.lr = 0x83203B34;
	sub_83126A78(ctx, base);
	// 83203B34: 48000985  bl 0x832044b8
	ctx.lr = 0x83203B38;
	sub_832044B8(ctx, base);
	// 83203B38: 4BF27539  bl 0x8312b070
	ctx.lr = 0x83203B3C;
	sub_8312B070(ctx, base);
	// 83203B3C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83203B40: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 83203B44: 3BAB3668  addi r29, r11, 0x3668
	ctx.r[29].s64 = ctx.r[11].s64 + 13928;
	// 83203B48: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203B4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83203B50: 419A0014  beq cr6, 0x83203b64
	if ctx.cr[6].eq {
	pc = 0x83203B64; continue 'dispatch;
	}
	// 83203B54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83203B58: 4BF22891  bl 0x831263e8
	ctx.lr = 0x83203B5C;
	sub_831263E8(ctx, base);
	// 83203B5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203B60: 4BFFFCB9  bl 0x83203818
	ctx.lr = 0x83203B64;
	sub_83203818(ctx, base);
	// 83203B64: 3BFF02A0  addi r31, r31, 0x2a0
	ctx.r[31].s64 = ctx.r[31].s64 + 672;
	// 83203B68: 397E0A80  addi r11, r30, 0xa80
	ctx.r[11].s64 = ctx.r[30].s64 + 2688;
	// 83203B6C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83203B70: 4198FFD8  blt cr6, 0x83203b48
	if ctx.cr[6].lt {
	pc = 0x83203B48; continue 'dispatch;
	}
	// 83203B74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83203B78: 4BFA4644  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83203B80 size=760
    let mut pc: u32 = 0x83203B80;
    'dispatch: loop {
        match pc {
            0x83203B80 => {
    //   block [0x83203B80..0x83203E78)
	// 83203B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83203B84: 4BFA45C5  bl 0x831a8148
	ctx.lr = 0x83203B88;
	sub_831A8130(ctx, base);
	// 83203B88: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203B8C: 3964003F  addi r11, r4, 0x3f
	ctx.r[11].s64 = ctx.r[4].s64 + 63;
	// 83203B90: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 83203B94: 55780032  rlwinm r24, r11, 0, 0, 0x19
	ctx.r[24].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83203B98: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83203B9C: 7D782050  subf r11, r24, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[24].s64;
	// 83203BA0: 7EAB2A14  add r21, r11, r5
	ctx.r[21].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 83203BA4: 409A0018  bne cr6, 0x83203bbc
	if !ctx.cr[6].eq {
	pc = 0x83203BBC; continue 'dispatch;
	}
	// 83203BA8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83203BAC: 386B3694  addi r3, r11, 0x3694
	ctx.r[3].s64 = ctx.r[11].s64 + 13972;
	// 83203BB0: 48001919  bl 0x832054c8
	ctx.lr = 0x83203BB4;
	sub_832054C8(ctx, base);
	// 83203BB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83203BB8: 480002B8  b 0x83203e70
	pc = 0x83203E70; continue 'dispatch;
	// 83203BBC: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 83203BC0: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 83203BC4: 394BE720  addi r10, r11, -0x18e0
	ctx.r[10].s64 = ctx.r[11].s64 + -6368;
	// 83203BC8: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 83203BCC: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 83203BD0: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203BD4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83203BD8: 419A0018  beq cr6, 0x83203bf0
	if ctx.cr[6].eq {
	pc = 0x83203BF0; continue 'dispatch;
	}
	// 83203BDC: 392902A0  addi r9, r9, 0x2a0
	ctx.r[9].s64 = ctx.r[9].s64 + 672;
	// 83203BE0: 390A0A80  addi r8, r10, 0xa80
	ctx.r[8].s64 = ctx.r[10].s64 + 2688;
	// 83203BE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83203BE8: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83203BEC: 4198FFE4  blt cr6, 0x83203bd0
	if ctx.cr[6].lt {
	pc = 0x83203BD0; continue 'dispatch;
	}
	// 83203BF0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83203BF4: 419AFFC0  beq cr6, 0x83203bb4
	if ctx.cr[6].eq {
	pc = 0x83203BB4; continue 'dispatch;
	}
	// 83203BF8: 1D6B02A0  mulli r11, r11, 0x2a0
	ctx.r[11].s64 = ctx.r[11].s64 * 672;
	// 83203BFC: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83203C00: 38A002A0  li r5, 0x2a0
	ctx.r[5].s64 = 672;
	// 83203C04: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83203C08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203C0C: 4BFA45D5  bl 0x831a81e0
	ctx.lr = 0x83203C10;
	sub_831A81E0(ctx, base);
	// 83203C10: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203C14: 3A802000  li r20, 0x2000
	ctx.r[20].s64 = 8192;
	// 83203C18: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 83203C1C: 997F0003  stb r11, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 83203C20: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203C24: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 83203C28: 81760008  lwz r11, 8(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203C2C: 929F01A4  stw r20, 0x1a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(420 as u32), ctx.r[20].u32 ) };
	// 83203C30: 915F01A8  stw r10, 0x1a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(424 as u32), ctx.r[10].u32 ) };
	// 83203C34: 917F0270  stw r11, 0x270(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(624 as u32), ctx.r[11].u32 ) };
	// 83203C38: 81760008  lwz r11, 8(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203C3C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83203C40: 409A0010  bne cr6, 0x83203c50
	if !ctx.cr[6].eq {
	pc = 0x83203C50; continue 'dispatch;
	}
	// 83203C44: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203C48: 1F2B4100  mulli r25, r11, 0x4100
	ctx.r[25].s64 = ctx.r[11].s64 * 16640;
	// 83203C4C: 4800001C  b 0x83203c68
	pc = 0x83203C68; continue 'dispatch;
	// 83203C50: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203C54: 81560000  lwz r10, 0(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203C58: 1D6B40C0  mulli r11, r11, 0x40c0
	ctx.r[11].s64 = ctx.r[11].s64 * 16576;
	// 83203C5C: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 83203C60: 556B0032  rlwinm r11, r11, 0, 0, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83203C64: 7F2B51D6  mullw r25, r11, r10
	ctx.r[25].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 83203C68: 3978003F  addi r11, r24, 0x3f
	ctx.r[11].s64 = ctx.r[24].s64 + 63;
	// 83203C6C: 815F0270  lwz r10, 0x270(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 83203C70: 557C0032  rlwinm r28, r11, 0, 0, 0x19
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83203C74: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83203C78: 7D7CC850  subf r11, r28, r25
	ctx.r[11].s64 = ctx.r[25].s64 - ctx.r[28].s64;
	// 83203C7C: 7F4BC214  add r26, r11, r24
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 83203C80: 409A000C  bne cr6, 0x83203c8c
	if !ctx.cr[6].eq {
	pc = 0x83203C8C; continue 'dispatch;
	}
	// 83203C84: 3BA04100  li r29, 0x4100
	ctx.r[29].s64 = 16640;
	// 83203C88: 48000018  b 0x83203ca0
	pc = 0x83203CA0; continue 'dispatch;
	// 83203C8C: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83203C90: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83203C94: 1D6B40C0  mulli r11, r11, 0x40c0
	ctx.r[11].s64 = ctx.r[11].s64 * 16576;
	// 83203C98: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 83203C9C: 557D0032  rlwinm r29, r11, 0, 0, 0x19
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83203CA0: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83203CA4: 7EFBBB78  mr r27, r23
	ctx.r[27].u64 = ctx.r[23].u64;
	// 83203CA8: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203CAC: 40810068  ble 0x83203d14
	if !ctx.cr[0].gt {
	pc = 0x83203D14; continue 'dispatch;
	}
	// 83203CB0: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 83203CB4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83203CB8: 7F5DD051  subf. r26, r29, r26
	ctx.r[26].s64 = ctx.r[26].s64 - ctx.r[29].s64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83203CBC: 7F9DE214  add r28, r29, r28
	ctx.r[28].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 83203CC0: 41800150  blt 0x83203e10
	if ctx.cr[0].lt {
	pc = 0x83203E10; continue 'dispatch;
	}
	// 83203CC4: 817F0270  lwz r11, 0x270(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 83203CC8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83203CCC: 409A0010  bne cr6, 0x83203cdc
	if !ctx.cr[6].eq {
	pc = 0x83203CDC; continue 'dispatch;
	}
	// 83203CD0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83203CD4: 4BF2A7F5  bl 0x8312e4c8
	ctx.lr = 0x83203CD8;
	sub_8312E4C8(ctx, base);
	// 83203CD8: 48000018  b 0x83203cf0
	pc = 0x83203CF0; continue 'dispatch;
	// 83203CDC: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83203CE0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83203CE4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83203CE8: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 83203CEC: 4BF2A79D  bl 0x8312e488
	ctx.lr = 0x83203CF0;
	sub_8312E488(ctx, base);
	// 83203CF0: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83203CF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203CF8: 41820118  beq 0x83203e10
	if ctx.cr[0].eq {
	pc = 0x83203E10; continue 'dispatch;
	}
	// 83203CFC: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83203D00: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83203D04: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83203D08: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83203D0C: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83203D10: 4198FFA4  blt cr6, 0x83203cb4
	if ctx.cr[6].lt {
	pc = 0x83203CB4; continue 'dispatch;
	}
	// 83203D14: 7FB9A851  subf. r29, r25, r21
	ctx.r[29].s64 = ctx.r[21].s64 - ctx.r[25].s64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83203D18: 7FD9C214  add r30, r25, r24
	ctx.r[30].u64 = ctx.r[25].u64 + ctx.r[24].u64;
	// 83203D1C: 418000F4  blt 0x83203e10
	if ctx.cr[0].lt {
	pc = 0x83203E10; continue 'dispatch;
	}
	// 83203D20: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83203D24: 7EFBBB78  mr r27, r23
	ctx.r[27].u64 = ctx.r[23].u64;
	// 83203D28: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203D2C: 40810054  ble 0x83203d80
	if !ctx.cr[0].gt {
	pc = 0x83203D80; continue 'dispatch;
	}
	// 83203D30: 3B9F0124  addi r28, r31, 0x124
	ctx.r[28].s64 = ctx.r[31].s64 + 292;
	// 83203D34: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83203D38: 809F01A4  lwz r4, 0x1a4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(420 as u32) ) } as u64;
	// 83203D3C: 80BF01A8  lwz r5, 0x1a8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(424 as u32) ) } as u64;
	// 83203D40: 7D44E850  subf r10, r4, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[4].s64;
	// 83203D44: 7D642A14  add r11, r4, r5
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 83203D48: 7FA55051  subf. r29, r5, r10
	ctx.r[29].s64 = ctx.r[10].s64 - ctx.r[5].s64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83203D4C: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83203D50: 418000C0  blt 0x83203e10
	if ctx.cr[0].lt {
	pc = 0x83203E10; continue 'dispatch;
	}
	// 83203D54: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203D58: 4BF30A79  bl 0x831347d0
	ctx.lr = 0x83203D5C;
	sub_831347D0(ctx, base);
	// 83203D5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203D60: 907CFF74  stw r3, -0x8c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-140 as u32), ctx.r[3].u32 ) };
	// 83203D64: 418200AC  beq 0x83203e10
	if ctx.cr[0].eq {
	pc = 0x83203E10; continue 'dispatch;
	}
	// 83203D68: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83203D6C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83203D70: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 83203D74: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83203D78: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83203D7C: 4198FFB8  blt cr6, 0x83203d34
	if ctx.cr[6].lt {
	pc = 0x83203D34; continue 'dispatch;
	}
	// 83203D80: 397DE000  addi r11, r29, -0x2000
	ctx.r[11].s64 = ctx.r[29].s64 + -8192;
	// 83203D84: 929F0120  stw r20, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[20].u32 ) };
	// 83203D88: 93DF0118  stw r30, 0x118(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[30].u32 ) };
	// 83203D8C: 394BFF00  addi r10, r11, -0x100
	ctx.r[10].s64 = ctx.r[11].s64 + -256;
	// 83203D90: 7D4A5E70  srawi r10, r10, 0xb
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 11) as i64;
	// 83203D94: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83203D98: 554A5828  slwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83203D9C: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203DA0: 915F011C  stw r10, 0x11c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[10].u32 ) };
	// 83203DA4: 4180006C  blt 0x83203e10
	if ctx.cr[0].lt {
	pc = 0x83203E10; continue 'dispatch;
	}
	// 83203DA8: 5544003E  slwi r4, r10, 0
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83203DAC: 2F0B0100  cmpwi cr6, r11, 0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, 256, &mut ctx.xer);
	// 83203DB0: 7D64F214  add r11, r4, r30
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[30].u64;
	// 83203DB4: 396B2000  addi r11, r11, 0x2000
	ctx.r[11].s64 = ctx.r[11].s64 + 8192;
	// 83203DB8: 917F01D0  stw r11, 0x1d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(464 as u32), ctx.r[11].u32 ) };
	// 83203DBC: 41980054  blt cr6, 0x83203e10
	if ctx.cr[6].lt {
	pc = 0x83203E10; continue 'dispatch;
	}
	// 83203DC0: 38A02000  li r5, 0x2000
	ctx.r[5].s64 = 8192;
	// 83203DC4: 92FF0094  stw r23, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[23].u32 ) };
	// 83203DC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83203DCC: 4BF30A05  bl 0x831347d0
	ctx.lr = 0x83203DD0;
	sub_831347D0(ctx, base);
	// 83203DD0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203DD4: 907F0090  stw r3, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[3].u32 ) };
	// 83203DD8: 41820038  beq 0x83203e10
	if ctx.cr[0].eq {
	pc = 0x83203E10; continue 'dispatch;
	}
	// 83203DDC: 38BF0098  addi r5, r31, 0x98
	ctx.r[5].s64 = ctx.r[31].s64 + 152;
	// 83203DE0: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83203DE4: 80760000  lwz r3, 0(r22)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203DE8: 48000EF1  bl 0x83204cd8
	ctx.lr = 0x83203DEC;
	sub_83204CD8(ctx, base);
	// 83203DEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203DF0: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83203DF4: 4182001C  beq 0x83203e10
	if ctx.cr[0].eq {
	pc = 0x83203E10; continue 'dispatch;
	}
	// 83203DF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83203DFC: 807F0090  lwz r3, 0x90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 83203E00: 4BF2F0D9  bl 0x83132ed8
	ctx.lr = 0x83203E04;
	sub_83132ED8(ctx, base);
	// 83203E04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203E08: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83203E0C: 40820010  bne 0x83203e1c
	if !ctx.cr[0].eq {
	pc = 0x83203E1C; continue 'dispatch;
	}
	// 83203E10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203E14: 4BFFFA05  bl 0x83203818
	ctx.lr = 0x83203E18;
	sub_83203818(ctx, base);
	// 83203E18: 4BFFFD9C  b 0x83203bb4
	pc = 0x83203BB4; continue 'dispatch;
	// 83203E1C: 3D608320  lis r11, -0x7ce0
	ctx.r[11].s64 = -2095054848;
	// 83203E20: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83203E24: 388B2C48  addi r4, r11, 0x2c48
	ctx.r[4].s64 = ctx.r[11].s64 + 11336;
	// 83203E28: 4BF2E949  bl 0x83132770
	ctx.lr = 0x83203E2C;
	sub_83132770(ctx, base);
	// 83203E2C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83203E30: 92FF01AC  stw r23, 0x1ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(428 as u32), ctx.r[23].u32 ) };
	// 83203E34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203E38: 9AFF01B4  stb r23, 0x1b4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(436 as u32), ctx.r[23].u8 ) };
	// 83203E3C: 92FF01B8  stw r23, 0x1b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(440 as u32), ctx.r[23].u32 ) };
	// 83203E40: 92FF01C4  stw r23, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[23].u32 ) };
	// 83203E44: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83203E48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83203E4C: D01F01E8  stfs f0, 0x1e8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(488 as u32), tmp.u32 ) };
	// 83203E50: 92FF01C0  stw r23, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[23].u32 ) };
	// 83203E54: 92FF01C8  stw r23, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[23].u32 ) };
	// 83203E58: 92FF0274  stw r23, 0x274(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(628 as u32), ctx.r[23].u32 ) };
	// 83203E5C: 92FF0278  stw r23, 0x278(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(632 as u32), ctx.r[23].u32 ) };
	// 83203E60: 997F01B5  stb r11, 0x1b5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(437 as u32), ctx.r[11].u8 ) };
	// 83203E64: 917F01BC  stw r11, 0x1bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(444 as u32), ctx.r[11].u32 ) };
	// 83203E68: 917F01CC  stw r11, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[11].u32 ) };
	// 83203E6C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83203E70: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83203E74: 4BFA4324  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203E78 size=56
    let mut pc: u32 = 0x83203E78;
    'dispatch: loop {
        match pc {
            0x83203E78 => {
    //   block [0x83203E78..0x83203EB0)
	// 83203E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83203E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83203E80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83203E84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203E88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203E8C: 480005BD  bl 0x83204448
	ctx.lr = 0x83203E90;
	sub_83204448(ctx, base);
	// 83203E90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203E94: 4BFFF985  bl 0x83203818
	ctx.lr = 0x83203E98;
	sub_83203818(ctx, base);
	// 83203E98: 480005B9  bl 0x83204450
	ctx.lr = 0x83203E9C;
	sub_83204450(ctx, base);
	// 83203E9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83203EA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83203EA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83203EA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83203EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203EB0 size=140
    let mut pc: u32 = 0x83203EB0;
    'dispatch: loop {
        match pc {
            0x83203EB0 => {
    //   block [0x83203EB0..0x83203F3C)
	// 83203EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83203EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83203EB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83203EBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83203EC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203EC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203EC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83203ECC: 4800057D  bl 0x83204448
	ctx.lr = 0x83203ED0;
	sub_83204448(ctx, base);
	// 83203ED0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83203ED4: 419A0040  beq cr6, 0x83203f14
	if ctx.cr[6].eq {
	pc = 0x83203F14; continue 'dispatch;
	}
	// 83203ED8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83203EDC: 419A0038  beq cr6, 0x83203f14
	if ctx.cr[6].eq {
	pc = 0x83203F14; continue 'dispatch;
	}
	// 83203EE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203EE4: 4BFFF0B5  bl 0x83202f98
	ctx.lr = 0x83203EE8;
	sub_83202F98(ctx, base);
	// 83203EE8: 4BEC3BF9  bl 0x830c7ae0
	ctx.lr = 0x83203EEC;
	sub_830C7AE0(ctx, base);
	// 83203EEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83203EF0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203EF4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83203EF8: 4BE4F419  bl 0x83053310
	ctx.lr = 0x83203EFC;
	sub_83053310(ctx, base);
	// 83203EFC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83203F00: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83203F04: 917F01E4  stw r11, 0x1e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(484 as u32), ctx.r[11].u32 ) };
	// 83203F08: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 83203F0C: 4BEC3BD5  bl 0x830c7ae0
	ctx.lr = 0x83203F10;
	sub_830C7AE0(ctx, base);
	// 83203F10: 48000010  b 0x83203f20
	pc = 0x83203F20; continue 'dispatch;
	// 83203F14: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83203F18: 386B3640  addi r3, r11, 0x3640
	ctx.r[3].s64 = ctx.r[11].s64 + 13888;
	// 83203F1C: 480015AD  bl 0x832054c8
	ctx.lr = 0x83203F20;
	sub_832054C8(ctx, base);
	// 83203F20: 48000531  bl 0x83204450
	ctx.lr = 0x83203F24;
	sub_83204450(ctx, base);
	// 83203F24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83203F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83203F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83203F30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83203F34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83203F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


